//! Front door into the machine. No text, no smoke.
//!
//! [`solve`] takes the raw plans, runs the grid, hands back a typed
//! [`Solution`] — steps in order, the numbers that matter, the PDDL3 metric
//! if there was one. Every shape in here is `serde`-cut: it survives the
//! wire in either direction.

use std::collections::HashSet;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ground::{ground, Outcome};
use crate::packed::PackedTask;
use crate::parser;
use crate::pddl3;
use crate::resolve::{self, Solved};
use crate::search;

/// The doctrine the engine runs under.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Reads the wire. PDDL3 metric if the job carries preferences, plain
    /// FF otherwise.
    #[default]
    Auto,
    /// Straight delete-relaxation FF, best-first, no detours.
    Ff,
    /// SGPlan cut: carve the task into pieces, solve, stitch.
    Partition,
    /// Soft goals on the table, anytime branch-and-bound tightening the
    /// metric until the clock or the budget calls it.
    Pddl3,
    /// PDDL2.1 durative actions, decision-epoch temporal search — time
    /// itself is part of the state.
    Temporal,
    /// A line of classical configurations run in sequence against one
    /// shared eval budget (ferroplan-roadmap.md Phase 6). Classical-search
    /// only — temporal and preference/metric jobs get routed to their own
    /// machinery, same fallback as `auto`.
    Portfolio,
}

/// The search doctrine inside a mode.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum Search {
    /// Let the machine pick: hill-climb hard, drop to weighted best-first
    /// the moment it stalls. The FF/Metric-FF default — fast on almost
    /// everything.
    #[default]
    Auto,
    /// Enforced hill-climbing riding helpful actions, falling back to
    /// best-first the instant no improving state turns up. Never goes
    /// incomplete.
    Ehc,
    /// Weighted best-first, no shortcuts, the whole task laid bare.
    /// Complete — ignores helpful actions entirely.
    BestFirst,
    /// EHC first, best-first when it chokes. Same shape as `auto`.
    EhcThenBestFirst,
}

fn default_weight_g() -> f64 {
    1.0
}
fn default_weight_h() -> f64 {
    5.0
}
fn default_true() -> bool {
    true
}

/// Every dial the solver answers to, in one place. Set it from code, ship it
/// over the wire (`serde` round-trips it clean), or leave it alone — the CLI
/// pulls its own flags from the same panel. Anything you don't set falls
/// back to these defaults.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Options {
    /// The doctrine (`auto` reads the problem's shape and routes itself).
    #[serde(default)]
    pub mode: Mode,
    /// The search doctrine riding inside that mode.
    #[serde(default)]
    pub search: Search,
    /// Chain expansion to helpful actions only. EHC obeys it; plain
    /// best-first doesn't feel it yet.
    #[serde(default = "default_true")]
    pub helpful_actions: bool,
    /// Best-first weight on `g` — how much the path already walked counts.
    #[serde(default = "default_weight_g")]
    pub weight_g: f64,
    /// Best-first weight on `h` — how much the heuristic's guess counts.
    /// House default: `1·g + 5·h`.
    #[serde(default = "default_weight_h")]
    pub weight_h: f64,
    /// Worker threads. `0` hands the choice to the machine
    /// (`min(cores, 6)`, or `FFDP_THREADS` if it's set).
    #[serde(default)]
    pub threads: usize,
    /// Hard ceiling on states evaluated. `None` leaves the engine's own
    /// limit standing.
    #[serde(default)]
    pub max_evaluated: Option<usize>,
    /// PDDL3 fork in the road: push for the optimal metric (`true`), or
    /// take the first plan that clears the hard goals and walk (`false`).
    #[serde(default = "default_true")]
    pub optimize: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            mode: Mode::Auto,
            search: Search::Auto,
            helpful_actions: true,
            weight_g: default_weight_g(),
            weight_h: default_weight_h(),
            threads: 0,
            max_evaluated: None,
            optimize: true,
        }
    }
}

impl Options {
    fn search_cfg(&self) -> crate::search::SearchCfg {
        crate::search::SearchCfg::from_weights(self.weight_g, self.weight_h, self.max_evaluated)
    }
}

/// One grounded move — a single step out on the wire.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step {
    pub index: usize,
    pub action: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Temporal mode only: the clock-time this move fires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    /// Temporal mode only: how long it burns. Instantaneous moves carry
    /// nothing here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

/// A plan that made it out alive.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plan {
    pub steps: Vec<Step>,
    pub length: usize,
    /// The PDDL3 metric's final tally, if one was in play.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<f64>,
    /// Temporal mode only: total wall-time the plan burns end to end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub makespan: Option<f64>,
}

/// The receipts from grounding and search — what it cost to get here.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Statistics {
    pub grounded_facts: usize,
    pub grounded_actions: usize,
    pub evaluated_states: usize,
    pub threads: usize,
}

/// What came back from the run — solved or not, plan or silence, plus the
/// numbers and any field notes.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Solution {
    pub solved: bool,
    pub mode: Mode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    pub statistics: Statistics,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

/// One contract cut from a [`Decomposition`]: a sub-goal small enough for the
/// temporal search to swallow whole, the sub-plan that closes it, and where
/// that sub-plan lands once it's welded back into the global timeline.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub index: usize,
    /// The sub-goal this contract pays off, rendered plain for the reader
    /// (e.g. `(order o1), (order o2)` or `coin >= 15`).
    pub goal: String,
    /// The sub-plan, clock zeroed to its own start.
    pub steps: Vec<Step>,
    /// How long this piece runs.
    pub makespan: f64,
    /// Where this piece sits once welded into the whole-goal timeline.
    pub offset: f64,
}

/// The paper trail of cutting a temporal goal into solvable contracts: the
/// pieces in order, the stitched whole-goal plan, and a flag for when the
/// goal wouldn't split — un-cuttable, or the cut didn't hold up — and had to
/// go through as one monolithic solve instead (then there's exactly one
/// contract: the whole goal, standing alone).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Decomposition {
    pub solved: bool,
    pub contracts: Vec<Contract>,
    /// The stitched plan, checked and cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    /// True when the cut failed to take — `contracts` then holds the one,
    /// unsplit whole.
    pub monolithic: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

/// A quick pass, nothing more: check the PDDL **syntax**, sketch the shape,
/// touch nothing else — no grounding, no solving. Guesses domain vs. problem
/// on sight. `ok` drops to `false` with `error` set the moment the parse
/// breaks. Wire-ready for editor tooling, an authoring loop wanting a fast
/// verdict, or the MCP `parse` tool. Need the full typed tree instead? Go to
/// [`crate::parser::parse_domain`] / `parse_problem`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParseReport {
    pub ok: bool,
    /// `"domain"` or `"problem"` — a best guess, called even when the parse
    /// wrecks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<String>,
    /// The wreck report — with a line number — when `ok` reads false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<ProblemSummary>,
}

/// A domain, X-rayed: signatures laid bare as `name argtypes…`.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DomainSummary {
    pub types: Vec<String>,
    pub predicates: Vec<String>,
    pub functions: Vec<String>,
    pub actions: Vec<String>,
    pub durative_actions: Vec<String>,
    pub derived: usize,
}

/// A problem, X-rayed.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProblemSummary {
    /// The domain this problem calls home.
    pub domain: String,
    pub objects: usize,
    pub init_facts: usize,
    pub init_fluents: usize,
    pub timed_initial_literals: usize,
    pub has_goal: bool,
    pub has_metric: bool,
}

/// Take a raw PDDL string, guess domain or problem, hand back a structured
/// [`ParseReport`] — syntax checked, shape sketched, nothing grounded or
/// solved.
pub fn parse(src: &str) -> ParseReport {
    // Same content-routing heuristic the visualizer uses: whichever of `(problem` /
    // `(domain` appears first wins.
    let up = src.to_ascii_uppercase();
    let is_problem = match (up.find("(PROBLEM"), up.find("(DOMAIN")) {
        (Some(p), Some(d)) => p < d,
        (Some(_), None) => true,
        _ => false,
    };
    if is_problem {
        match parser::parse_problem(src) {
            Ok(p) => ParseReport {
                ok: true,
                kind: Some("problem".into()),
                name: Some(p.name.to_lowercase()),
                requirements: Vec::new(), // problem-file requirements are over-read
                error: None,
                domain: None,
                problem: Some(ProblemSummary {
                    domain: p.domain_name.to_lowercase(),
                    objects: p.objects.len(),
                    init_facts: p.init_atoms.len(),
                    init_fluents: p.init_fluents.len(),
                    timed_initial_literals: p.til.len(),
                    has_goal: !matches!(p.goal, crate::types::Formula::True),
                    has_metric: p.metric.is_some(),
                }),
            },
            Err(e) => parse_err("problem", e),
        }
    } else {
        match parser::parse_domain(src) {
            Ok(d) => ParseReport {
                ok: true,
                kind: Some("domain".into()),
                name: Some(d.name.to_lowercase()),
                requirements: d
                    .requirements
                    .iter()
                    .map(|r| format!(":{}", r.trim_start_matches(':').to_lowercase()))
                    .collect(),
                error: None,
                domain: Some(DomainSummary {
                    types: d.types.iter().map(|t| t.to_lowercase()).collect(),
                    predicates: d.predicates.iter().map(|(n, a)| render_sig(n, a)).collect(),
                    functions: d.functions.iter().map(|(n, a)| render_sig(n, a)).collect(),
                    actions: d.actions.iter().map(|a| a.name.to_lowercase()).collect(),
                    durative_actions: d
                        .durative_actions
                        .iter()
                        .map(|a| a.name.to_lowercase())
                        .collect(),
                    derived: d.derived.len(),
                }),
                problem: None,
            },
            Err(e) => parse_err("domain", e),
        }
    }
}

/// `name argtype1 argtype2 …` — just `name` when the predicate/function
/// carries no arguments at all.
fn render_sig(name: &str, arg_types: &[String]) -> String {
    if arg_types.is_empty() {
        name.to_lowercase()
    } else {
        let args = arg_types
            .iter()
            .map(|t| t.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");
        format!("{} {}", name.to_lowercase(), args)
    }
}

fn parse_err(kind: &str, e: crate::types::ParseError) -> ParseReport {
    ParseReport {
        ok: false,
        kind: Some(kind.to_string()),
        name: None,
        requirements: Vec::new(),
        error: Some(e.to_string()),
        domain: None,
        problem: None,
    }
}

/// Re-exported so callers can name the PDDL3 metric type when they need to.
pub type Metric = f64;

/// Everything that can stop a [`Solution`] from ever forming.
#[derive(thiserror::Error, Debug)]
pub enum SolveError {
    #[error("domain parse error: {0}")]
    DomainParse(crate::types::ParseError),
    #[error("problem parse error: {0}")]
    ProblemParse(crate::types::ParseError),
    #[error("{kind} {pred} uses an unknown or empty type {ty}")]
    EmptyType {
        kind: String,
        pred: String,
        ty: String,
    },
    #[error("derived predicate error: {0}")]
    Derived(String),
    #[error("unsupported feature: {0}")]
    Unsupported(String),
}

enum Grounded {
    Task(Box<PackedTask>),
    /// The goal was already standing — an empty plan closes it, no shots fired.
    Trivial,
    /// goal provably false / references an undefined fluent
    Unsolvable,
}

fn do_ground(
    domain: &crate::types::Domain,
    problem: &crate::types::Problem,
    threads: usize,
) -> Result<Grounded, SolveError> {
    match ground(domain, problem, threads) {
        Outcome::Task(t) => Ok(Grounded::Task(Box::new(t))),
        Outcome::GoalTrue => Ok(Grounded::Trivial),
        Outcome::GoalFalse | Outcome::GoalUndefinedFluent => Ok(Grounded::Unsolvable),
        Outcome::EmptyType { kind, pred, ty } => Err(SolveError::EmptyType {
            kind: kind.to_string(),
            pred,
            ty,
        }),
    }
}

pub(crate) fn steps_of(
    task: &PackedTask,
    ops: &[usize],
    synthetic: Option<&HashSet<String>>,
) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut idx = 0;
    for &oi in ops {
        let disp = &task.op_display[oi];
        let mut it = disp.split_whitespace();
        let action = it.next().unwrap_or("").to_string();
        // strip the artificial goal-closer + PDDL3 bookkeeping actions
        if action == "REACH-GOAL" || synthetic.is_some_and(|s| s.contains(&action)) {
            continue;
        }
        steps.push(Step {
            index: idx,
            action,
            args: it.map(|s| s.to_string()).collect(),
            time: None,
            duration: None,
        });
        idx += 1;
    }
    steps
}

/// Cut the synthetic `TRAJ-END` step out of a converted step list (the 0.8
/// END construction) — only when the constraint gate actually compiled.
/// Indices get re-cut so they stay unbroken across the real actions left
/// standing.
fn strip_end_steps(steps: Vec<Step>, constrained: bool) -> Vec<Step> {
    if !constrained {
        return steps;
    }
    steps
        .into_iter()
        .filter(|s| s.action != crate::constraints::END_ACTION)
        .enumerate()
        .map(|(i, mut s)| {
            s.index = i;
            s
        })
        .collect()
}

/// Recast a temporal plan's timed steps into API [`Step`]s — action head,
/// args, time, duration. Shared machinery between the temporal solve and the
/// decomposer.
pub(crate) fn timed_steps(tp: &crate::temporal::TimedPlan) -> Vec<Step> {
    tp.steps
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut it = s.action.split_whitespace();
            Step {
                index: i,
                action: it.next().unwrap_or("").to_string(),
                args: it.map(|x| x.to_string()).collect(),
                time: Some(s.time),
                duration: s.duration,
            }
        })
        .collect()
}

pub(crate) fn stats(task: &PackedTask, evaluated: usize, threads: usize) -> Statistics {
    Statistics {
        grounded_facts: task.n_reach_facts,
        grounded_actions: task.n_reach_actions,
        evaluated_states: evaluated,
        threads,
    }
}

fn trivial(mode: Mode, threads: usize) -> Solution {
    Solution {
        solved: true,
        mode,
        plan: Some(Plan {
            steps: Vec::new(),
            length: 0,
            metric: None,
            makespan: None,
        }),
        statistics: Statistics {
            threads,
            ..Default::default()
        },
        notes: vec!["goal already satisfied; the empty plan solves it".into()],
    }
}

fn unsolved(mode: Mode, stats: Statistics, notes: Vec<String>) -> Solution {
    Solution {
        solved: false,
        mode,
        plan: None,
        statistics: stats,
        notes,
    }
}

/// Ground the world, run the search, hand back a structured [`Solution`].
///
/// **Temporal domains, v0.3.0+:** a failed default-tier search doesn't die
/// here — it climbs to the `Full` demand tier, then the goal decomposer,
/// before it finally calls it quits (see [`crate::temporal::solve`]). A job
/// that used to fail fast can now burn a lot more clock before it hands back
/// `solved: false`. Set `FF_NO_ESCALATE` (or
/// [`crate::features::set_escalate_override`]`(false)` in-process) to pull
/// the old single-pass pre-0.3.0 behavior back.
pub fn solve(domain_src: &str, problem_src: &str, opts: &Options) -> Result<Solution, SolveError> {
    let domain = parser::parse_domain(domain_src).map_err(SolveError::DomainParse)?;
    let problem = parser::parse_problem(problem_src).map_err(SolveError::ProblemParse)?;
    // Compile `:derived` axioms away (static rules -> init facts) before routing.
    let (domain, problem) =
        crate::derived::compile(&domain, &problem).map_err(SolveError::Derived)?;
    // 0.7: hard untimed trajectory constraints compile into monitor automata;
    // everything else gets a NAMED rejection (see constraints::gate).
    // `constrained` records that the gate compiled — the flag that tells
    // reporting to strip the synthetic TRAJ-END step (0.8 END construction);
    // it is never set on the constraint-free byte-identical path.
    let (domain, problem, constrained) = match crate::constraints::gate(&domain, &problem) {
        Ok(Some((d, p))) => (d, p, true),
        Ok(None) => (domain, problem, false),
        Err(reason) => return Err(SolveError::Unsupported(reason)),
    };
    let threads = if opts.threads == 0 {
        crate::par::num_threads()
    } else {
        opts.threads
    };

    let mode = match opts.mode {
        Mode::Auto => {
            if crate::temporal::is_temporal(&domain) {
                Mode::Temporal
            } else if pddl3::has_preferences(&problem) {
                Mode::Pddl3
            } else {
                Mode::Ff
            }
        }
        m => m,
    };

    // Portfolio is a classical-search feature: problems the portfolio's
    // members cannot represent keep their own machinery, exactly like auto.
    let mode = if mode == Mode::Portfolio
        && (crate::temporal::is_temporal(&domain) || pddl3::has_preferences(&problem))
    {
        if crate::temporal::is_temporal(&domain) {
            Mode::Temporal
        } else {
            Mode::Pddl3
        }
    } else {
        mode
    };

    match mode {
        Mode::Temporal => solve_temporal(&domain, &problem, threads),
        Mode::Pddl3 => solve_pddl3(&domain, &problem, opts, threads, constrained),
        _ => solve_classic(
            &domain,
            &problem,
            opts,
            threads,
            mode,
            Vec::new(),
            constrained,
        ),
    }
}

/// Decompose a temporal goal into solvable contracts, solve and stitch them, and
/// return the inspectable [`Decomposition`]. This always runs the partition-and-
/// resolve decomposer (independent of the `FF_TDECOMP` flag): a goal too big for the
/// one-shot temporal search is split into ordered sub-contracts — each solved whole
/// and verified — then stitched into one validated plan. A goal that can't be split
/// (or whose split doesn't validate) falls back to a single monolithic contract.
pub fn decompose(
    domain_src: &str,
    problem_src: &str,
    opts: &Options,
) -> Result<Decomposition, SolveError> {
    let domain = parser::parse_domain(domain_src).map_err(SolveError::DomainParse)?;
    let problem = parser::parse_problem(problem_src).map_err(SolveError::ProblemParse)?;
    let (domain, problem) =
        crate::derived::compile(&domain, &problem).map_err(SolveError::Derived)?;
    // 0.7 gate: decompose targets temporal goals, where trajectory
    // constraints stay rejected (Phase 3) — the gate names that. A CLASSICAL
    // constrained input still passes through (falling back to one contract),
    // so `constrained` drives the TRAJ-END step strip below.
    let (domain, problem, constrained) = match crate::constraints::gate(&domain, &problem) {
        Ok(Some((d, p))) => (d, p, true),
        Ok(None) => (domain, problem, false),
        Err(reason) => return Err(SolveError::Unsupported(reason)),
    };
    let threads = if opts.threads == 0 {
        crate::par::num_threads()
    } else {
        opts.threads
    };

    let mut notes = Vec::new();
    if !crate::temporal::is_temporal(&domain) {
        notes.push(
            "decomposition targets temporal (durative-action) goals; this domain has none".into(),
        );
    }

    match crate::tresolve::decompose(&domain, &problem, threads) {
        Some(d) => {
            let contracts = d
                .contracts
                .iter()
                .enumerate()
                .map(|(i, cr)| Contract {
                    index: i,
                    goal: cr.goal.clone(),
                    steps: strip_end_steps(timed_steps(&cr.plan), constrained),
                    makespan: cr.plan.makespan,
                    offset: cr.offset,
                })
                .collect();
            let steps = strip_end_steps(timed_steps(&d.plan), constrained);
            if d.monolithic {
                notes.push(
                    "goal could not be split into independent contracts; solved monolithically"
                        .into(),
                );
            }
            Ok(Decomposition {
                solved: true,
                contracts,
                plan: Some(Plan {
                    length: steps.len(),
                    steps,
                    metric: None,
                    makespan: Some(d.plan.makespan),
                }),
                monolithic: d.monolithic,
                notes,
            })
        }
        None => {
            notes.push("no plan found (decomposed or monolithic)".into());
            Ok(Decomposition {
                solved: false,
                contracts: Vec::new(),
                plan: None,
                monolithic: false,
                notes,
            })
        }
    }
}

fn solve_temporal(
    domain: &crate::types::Domain,
    problem: &crate::types::Problem,
    threads: usize,
) -> Result<Solution, SolveError> {
    // FF_TDECOMP routes through the partition-and-resolve decomposer (Phase B), the
    // same gate as the text path (run_planner); the default is `temporal::solve` —
    // the monolithic search plus its on-failure escalation ladder.
    let result = if crate::features::tdecomp() {
        crate::tresolve::solve(domain, problem, threads)
    } else {
        crate::temporal::solve(domain, problem, threads)
    };
    match result {
        Some(tp) => {
            let steps = timed_steps(&tp);
            Ok(Solution {
                solved: true,
                mode: Mode::Temporal,
                plan: Some(Plan {
                    length: steps.len(),
                    steps,
                    metric: None,
                    makespan: Some(tp.makespan),
                }),
                statistics: Statistics {
                    threads,
                    ..Default::default()
                },
                notes: Vec::new(),
            })
        }
        None => Ok(unsolved(
            Mode::Temporal,
            Statistics {
                threads,
                ..Default::default()
            },
            Vec::new(),
        )),
    }
}

fn solve_classic(
    domain: &crate::types::Domain,
    problem: &crate::types::Problem,
    opts: &Options,
    threads: usize,
    mode: Mode,
    extra_notes: Vec<String>,
    // The constraint gate compiled: strip the synthetic TRAJ-END step
    // from the reported plan (0.8 END construction).
    strip_end: bool,
) -> Result<Solution, SolveError> {
    let mut notes = extra_notes;
    let task = match do_ground(domain, problem, threads)? {
        Grounded::Task(t) => t,
        Grounded::Trivial => return Ok(trivial(mode, threads)),
        Grounded::Unsolvable => {
            return Ok(unsolved(
                mode,
                Statistics {
                    threads,
                    ..Default::default()
                },
                notes,
            ))
        }
    };

    let (ops, evaluated) = if mode == Mode::Portfolio {
        let o = crate::portfolio::solve(&task, threads, opts.search_cfg());
        if let Some(w) = o.winner {
            notes.push(format!("portfolio: solved by member `{w}`"));
        }
        (o.ops, o.evaluated)
    } else if mode == Mode::Partition {
        let groups = crate::invariants::synthesize(domain, &task);
        match resolve::solve(&task, threads, opts.search_cfg(), &groups) {
            Solved::Plan(ops, _) => (Some(ops), 0),
            Solved::Unsolvable => (None, 0),
        }
    } else {
        let ehc_first = opts.search != Search::BestFirst;
        let o = search::plan(&task, threads, opts.search_cfg(), ehc_first);
        if o.ehc_fell_back && o.ops.is_some() {
            notes.push("EHC found no improving state; used weighted best-first".into());
        }
        (o.ops, o.evaluated)
    };

    match ops {
        Some(mut ops) => {
            // IPC6 `:action-costs`: report the metric's real value and run the
            // anytime cost-improvement sweep (0.9 Phase 2). The first plan
            // above is untouched machinery — only this polish pass is new.
            let mut metric = None;
            let mut sweep_evals = 0;
            if let Some(cf) =
                crate::costs::metric_fluent(problem).and_then(|disp| task.fluent_id(&disp))
            {
                match crate::costs::plan_cost(&task, cf, &ops) {
                    Some(c0) if opts.optimize => {
                        let r = crate::costs::improve(
                            &task,
                            cf,
                            ops,
                            c0,
                            threads,
                            opts.search_cfg(),
                            evaluated,
                        );
                        ops = r.ops;
                        metric = Some(r.cost);
                        sweep_evals = r.evaluated;
                        if r.improved {
                            notes.push(format!(
                                "anytime cost sweep improved plan cost {} -> {}",
                                c0, r.cost
                            ));
                        }
                        if r.proven {
                            notes.push("plan cost proven optimal".into());
                        }
                    }
                    Some(c0) => {
                        metric = Some(c0);
                        notes.push("cost metric reported, not optimized (--satisfice)".into());
                    }
                    None => notes
                        .push("metric fluent undefined at plan end; metric not reported".into()),
                }
            } else if problem.metric.is_none() && opts.optimize {
                // Metric-FREE problem: plan LENGTH is the quality measure.
                // Iterated-weight anytime (0.9 Phase 3 remainder) — bounded
                // re-searches at decreasing w_h keep the shortest plan;
                // FF_LEN_SWEEP_EVALS=0 restores first-found byte-identically.
                let len0 = ops.len();
                let (better, evals, improved) =
                    crate::costs::improve_length(&task, ops, threads, opts.search_cfg(), evaluated);
                ops = better;
                sweep_evals = evals;
                if improved {
                    notes.push(format!(
                        "iterated-weight sweep shortened plan {} -> {} steps",
                        len0,
                        ops.len()
                    ));
                }
            }
            if strip_end {
                crate::constraints::strip_end(&task, &mut ops);
            }
            let steps = steps_of(&task, &ops, None);
            Ok(Solution {
                solved: true,
                mode,
                plan: Some(Plan {
                    length: steps.len(),
                    steps,
                    metric,
                    makespan: None,
                }),
                statistics: stats(&task, evaluated + sweep_evals, threads),
                notes,
            })
        }
        None => Ok(unsolved(mode, stats(&task, evaluated, threads), notes)),
    }
}

fn solve_pddl3(
    domain: &crate::types::Domain,
    problem: &crate::types::Problem,
    opts: &Options,
    threads: usize,
    // The constraint gate compiled: strip the synthetic TRAJ-END step
    // from the reported plan (0.8 END construction).
    strip_end: bool,
) -> Result<Solution, SolveError> {
    // caller opted out of metric optimization -> satisficing plan (hard goals).
    if !opts.optimize {
        let note = "PDDL3 metric not optimized (optimize = false); satisficing plan".to_string();
        return solve_classic(
            domain,
            problem,
            opts,
            threads,
            Mode::Pddl3,
            vec![note],
            strip_end,
        );
    }

    let mut c = pddl3::compile(domain, problem);
    if strip_end {
        // TRAJ-END is a real action to the P3 machinery (it plans before the
        // freeze) but a synthetic step to every reporting surface.
        c.synthetic
            .insert(crate::constraints::END_ACTION.to_string());
    }

    // metric outside the supported class -> satisficing plan over the hard goals
    if let Some(reason) = c.unsupported.clone() {
        let note = format!(
            "PDDL3 metric not optimized ({}); returning a satisficing plan",
            reason
        );
        return solve_classic(
            domain,
            problem,
            opts,
            threads,
            Mode::Pddl3,
            vec![note],
            strip_end,
        );
    }

    let task = match do_ground(&c.domain, &c.problem, threads)? {
        Grounded::Task(t) => t,
        Grounded::Trivial => return Ok(trivial(Mode::Pddl3, threads)),
        Grounded::Unsolvable => {
            return Ok(unsolved(
                Mode::Pddl3,
                Statistics {
                    threads,
                    ..Default::default()
                },
                Vec::new(),
            ))
        }
    };

    let cf = task
        .fluent_id(pddl3::COST_DISP)
        .expect("compile() always injects the total-cost fluent");
    let forgos: Vec<(usize, f64)> = c
        .forgos
        .iter()
        .filter_map(|(name, w)| {
            task.op_display
                .iter()
                .position(|d| d == name)
                .map(|oi| (oi, *w))
        })
        .collect();

    // Mutex groups feed the resource-aware guidance (renewable counter resources).
    let groups = crate::invariants::synthesize(&c.domain, &task);
    match pddl3::metric_optimize(&task, cf, &forgos, &groups, c.folded_metric, threads) {
        Some(r) => {
            let mut notes = Vec::new();
            if c.warn_other {
                notes.push(
                    "metric has terms beyond is-violated/total-cost; optimized the supported part"
                        .into(),
                );
            }
            if c.maximized {
                notes.push(
                    "maximize metric normalized to minimize; reported metric is the original \
                     (maximized) value"
                        .into(),
                );
            }
            if !r.proven {
                notes.push("search bound hit; metric is best-found, not proven optimal".into());
            }
            let steps = steps_of(&task, &r.ops, Some(&c.synthetic));
            Ok(Solution {
                solved: true,
                mode: Mode::Pddl3,
                plan: Some(Plan {
                    length: steps.len(),
                    steps,
                    metric: Some(c.display_metric(r.cost)),
                    makespan: None,
                }),
                statistics: stats(&task, 0, threads),
                notes,
            })
        }
        None => Ok(unsolved(Mode::Pddl3, stats(&task, 0, threads), Vec::new())),
    }
}
