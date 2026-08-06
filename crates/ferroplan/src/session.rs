//! FIELD LOG // ground once, replan forever. Deploy asset for callers that re-solve
//! the same theater every tick — villager swarms, sim loops, live agent runtimes.
//!
//! [`crate::solve`] burns a fresh parse and a fresh grounding on every call — dead
//! weight when the map hasn't moved. A [`Session`] parses once, compiles `:derived`
//! axioms once, grounds once, then keeps the *live world state* pinned in memory:
//! shove facts and fluents through [`Session::set_fact`] / [`Session::set_fluent`]
//! as the ground truth shifts, then call [`Session::replan`] to solve from wherever
//! things now stand. No re-grounding tax. Just the search.
//!
//! ```no_run
//! use ferroplan::{Options, Session};
//! # let (domain_src, problem_src) = (String::new(), String::new());
//! let mut s = Session::new(&domain_src, &problem_src, &Options::default())?;
//! let first = s.replan();                       // plan from the problem's :init
//! s.set_fact("(at v1 field)", true)?;           // the world moved
//! s.set_fluent("(grain)", 3.0)?;
//! let next = s.replan();                        // replan from the current state
//! # Ok::<(), String>(())
//! ```
//!
//! **Territory covered.** Classical, numeric, ADL — and since 0.12, TEMPORAL
//! (durative-action) theaters: the snap compilation grounds once, every think runs
//! the bounded decision-epoch ladder off the current state. Since 0.14 the world can
//! be caught IN FLIGHT — [`Session::apply_start`] triggers a durative action now,
//! every think and validity replay carries its pending end as a live happening (a
//! plan holds valid THROUGH any running interval), and [`Session::elapse`] fires due
//! ends on schedule — the old mirror-the-end-effects trick is retired. The world can
//! also run a SCHEDULE (0.14 Phase 3): [`Session::set_timed_fact`] plants
//! clock-relative events (market closes in five) that thinks route around and
//! `elapse` detonates on cue. Absolute-clock TILs get rejected on sight at
//! construction — session time only knows relative-to-NOW. PDDL3 preference problems
//! bounce too (the metric optimizer needs the whole problem per solve). The goal
//! itself can be retargeted mid-run (0.13): [`Session::set_goal`] drops in any ground
//! conjunction over the interned fact space, no regrounding required. A session can
//! fork (0.13): [`Session::fork`] splits a second mind off the SAME grounded world —
//! one grounding, N cheap state views. And a mind can be leashed (0.14):
//! [`Session::restrict_ops`] boxes it into its own actions — the primitive that keeps
//! a many-minds theater from crossing wires.
//!
//! **Why a frozen fact stays frozen.** Grounding enumerates operator parameters
//! against *static* predicates read straight from `:init` — flip a static fact after
//! the ground is poured and you need operators that were never cast. Rather than
//! hand back a plan that lies, [`Session::set_fact`] only lets through facts some
//! operator can actually add or strike (the world's *dynamic* layer) and slams the
//! door on the rest. Fluent values are read live every time, so any grounded fluent
//! is fair game.

use crate::api::{stats, steps_of, timed_steps, Mode, Options, Plan, Search, Solution};
use crate::ground::ground_task;
use crate::hash::FxHashMap;
use crate::packed::PackedTask;
use crate::search;
use crate::types::{Expr, Formula, NExpr, NumPre, Term};
use std::sync::Arc;

/// Strip a fact display down to its predicate head (`(AT V1 HUT)` -> `AT`),
/// cutting through a complementary-mirror wrapper (`(NOT (RUNNING-BUILD W1))`
/// -> `RUNNING-BUILD`) so the `RUNNING-*` fence can't be slipped by disguise.
fn atom_head(display: &str) -> &str {
    let mut s = display.trim();
    loop {
        s = s.trim_start_matches('(').trim_start();
        let head = s
            .split(|c: char| c.is_whitespace() || c == '(' || c == ')')
            .next()
            .unwrap_or("");
        if head.eq_ignore_ascii_case("NOT") && !head.is_empty() {
            s = &s[head.len()..];
            continue;
        }
        return head;
    }
}

/// A grounded mind, live and replannable. See the module dispatch above.
pub struct Session {
    task: PackedTask,
    threads: usize,
    weight_g: f64,
    weight_h: f64,
    max_evaluated: Option<usize>,
    ehc_first: bool,
    /// Display name (uppercase, e.g. `(AT V1 FIELD)`) -> fact id. Shared
    /// across every fork (0.13 Phase 2) — frozen the instant it's poured,
    /// same as every `Arc`'d field below: N minds, one copy, no drift.
    fact_ids: Arc<FxHashMap<String, u32>>,
    /// Per fact id: does any operator touch it — add it, strike it? Statics
    /// are cast into the grounding and stay cast. No exceptions.
    dynamic: Arc<[bool]>,
    /// Display name (uppercase, e.g. `(GRAIN)`) -> fluent id.
    fluent_ids: Arc<FxHashMap<String, u32>>,
    /// Temporal session state (0.12 Phase 1): the snap compilation, held so
    /// every think can REBUILD `build_kind`'s duration table off the
    /// CURRENT fluent readings — a `set_fluent` touching a duration-linked
    /// fluent has to bleed into parameter-dependent durations, not sit
    /// frozen at construction time. `None` means classical, no clock.
    temporal: Option<Arc<crate::temporal::TemporalCompiled>>,
    /// The demand tier, locked in ONCE at construction — a session's
    /// behavior stays fixed even if the process environment shifts under it
    /// mid-run.
    tier: crate::features::DemandMode,
    /// Compiler-minted `RUNNING-*` token predicates (temporal only). Between
    /// thinks the world is AT REST — no interval left running — so
    /// `set_fact` fences these tokens exactly like it fences statics.
    running_preds: Vec<String>,
    /// Op display (`WALK V1 HUT FIELD`) -> op id, for suffix replay
    /// ([`Session::plan_still_valid`]).
    op_ids: Arc<FxHashMap<String, usize>>,
    /// Complementary-mirror pairing (0.13 Phase 1), running BOTH ways: when
    /// grounding minted a `(NOT (p ...))` shadow fact (negative
    /// preconditions/goals), [`Session::set_fact`] on either side drags the
    /// other along — a stale mirror rots every applicability and goal test
    /// downstream, silently.
    mirror: Arc<FxHashMap<u32, u32>>,
    /// Per-mind op mask (0.14 Phase 1): `forbidden[oi]` ops never fire for
    /// this mind's thinks and never pass its replays. Empty means no leash.
    /// The scoping primitive for many-minds theaters — a mind plans ITS OWN
    /// moves only ([`Session::restrict_ops`]); a rival's actions land on it
    /// as world drift, never as plan steps to claim.
    forbidden: Vec<bool>,
    /// Pending scheduled events (0.14 Phase 3, temporal only): `(dt, fact,
    /// value)` — the fact flips in `dt` time units from NOW. Sorted by
    /// (dt, fact, value) for deterministic detonation order; fed into every
    /// think as think-relative TIL events; burned down by [`Session::elapse`].
    timed: Vec<(f64, u32, bool)>,
    /// (fact, value) -> the agenda-fired setter op that lands it (its mirror
    /// stays in sync inside the op's own effects). Poured once at
    /// construction (empty for classical sessions), shared by every fork.
    til_setters: Arc<FxHashMap<(u32, bool), usize>>,
    /// IN-FLIGHT intervals (0.14 Phase 5): `(remaining, end_op)` for every
    /// durative action the world is running right now
    /// ([`Session::apply_start`]). They ride into thinks and replays as
    /// root-agenda happenings — a mind rethinks WHILE the kiln still fires —
    /// and [`Session::elapse`] detonates the due ends, retiring the old
    /// mirror-the-end-effects trick for good.
    running: Vec<(f64, usize)>,
}

/// Wire a freshly grounded TEMPORAL task for scheduled-event detonation
/// (0.14 Phase 3): per dynamic fact `f`, mint `TILSET-f` (lands `f`, strikes
/// its mirror) and `TILCLR-f` (strikes `f`, lands its mirror). Every setter
/// sits behind a fresh, never-true `TIL-NEVER` fence — the relaxation and the
/// search's start block never see them (no heuristic pollution, no spurious
/// achievers — deliberately absent from `add_by_fact`), and only a
/// pre-seeded agenda triggers them (`Kind::Til` fires unconditionally —
/// exogenous events don't wait for permission). Returns (fact, value) -> op id.
fn append_til_setters(
    task: &mut PackedTask,
    dynamic: &[bool],
    mirror: &FxHashMap<u32, u32>,
) -> FxHashMap<(u32, bool), usize> {
    use crate::packed::CsrBuilder;
    let never = task.n_facts as u32;
    task.n_facts += 1;
    task.words = task.n_facts.div_ceil(64);
    let mut init_bits = task.init_bits.clone();
    init_bits.resize(task.words, 0);
    task.init_bits = init_bits;
    let mut fact_names = task.fact_names.to_vec();
    fact_names.push("TIL-NEVER".into());

    let rebuild = |extra: &[(Vec<u32>, Vec<u32>, Vec<u32>)]| {
        let (mut pre, mut add, mut del) = (CsrBuilder::new(), CsrBuilder::new(), CsrBuilder::new());
        for oi in 0..task.n_ops {
            pre.push_row(task.pre_pos.slice(oi).iter().copied());
            add.push_row(task.add.slice(oi).iter().copied());
            del.push_row(task.del.slice(oi).iter().copied());
        }
        for (p, a, d) in extra {
            pre.push_row(p.iter().copied());
            add.push_row(a.iter().copied());
            del.push_row(d.iter().copied());
        }
        (pre.finish(), add.finish(), del.finish())
    };
    let mut map = FxHashMap::default();
    let mut rows: Vec<(Vec<u32>, Vec<u32>, Vec<u32>)> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for f in 0..dynamic.len() as u32 {
        if !dynamic[f as usize] {
            continue;
        }
        let m = mirror.get(&f).copied();
        for value in [true, false] {
            let (mut a, mut d) = if value {
                (vec![f], m.map(|m| vec![m]).unwrap_or_default())
            } else {
                (m.map(|m| vec![m]).unwrap_or_default(), vec![f])
            };
            a.sort_unstable();
            d.sort_unstable();
            map.insert((f, value), task.n_ops + rows.len());
            names.push(format!("{}-{f}", if value { "TILSET" } else { "TILCLR" }));
            rows.push((vec![never], a, d));
        }
    }
    let (pre, add, del) = rebuild(&rows);
    task.pre_pos = pre;
    task.add = add;
    task.del = del;
    fn extend_empty<T: Clone>(csr: &mut crate::packed::Csr<T>, n: usize) {
        let mut b = crate::packed::CsrBuilder::new();
        for i in 0..csr.off.len() - 1 {
            b.push_row(csr.slice(i).iter().cloned());
        }
        for _ in 0..n {
            b.push_row(std::iter::empty());
        }
        *csr = b.finish();
    }
    extend_empty(&mut task.pre_num, rows.len());
    extend_empty(&mut task.num_eff, rows.len());
    extend_empty(&mut task.cond, rows.len());
    // add_by_fact grows a row for TIL-NEVER but the setters are NOT
    // registered as achievers — the whole point of the fence.
    {
        let mut b = CsrBuilder::new();
        for i in 0..task.add_by_fact.off.len() - 1 {
            b.push_row(task.add_by_fact.slice(i).iter().copied());
        }
        b.push_row(std::iter::empty());
        task.add_by_fact = b.finish();
    }
    let mut op_display = task.op_display.to_vec();
    op_display.extend(names);
    let mut monitored = task.monitored.to_vec();
    monitored.resize(task.n_ops + rows.len(), false);
    task.op_display = op_display.into();
    task.monitored = monitored.into();
    task.fact_names = fact_names.into();
    task.n_ops += rows.len();
    map
}

impl Session {
    /// Stand up a mind: parse, compile `:derived` axioms, ground once
    /// (temporal domains snap-compile first). Fails loud on a bad parse, a
    /// bad ground, or cargo it won't carry — constraints, preferences,
    /// timed-initial-literals (see the dispatch above for why).
    pub fn new(domain_src: &str, problem_src: &str, opts: &Options) -> Result<Session, String> {
        let domain = crate::parser::parse_domain(domain_src).map_err(|e| format!("domain: {e}"))?;
        let problem =
            crate::parser::parse_problem(problem_src).map_err(|e| format!("problem: {e}"))?;
        let (domain, problem) = crate::derived::compile(&domain, &problem)?;
        if !domain.constraints.is_empty() || !problem.constraints.is_empty() {
            return Err("Session does not support PDDL3 trajectory constraints yet \
                 (ferroplan::solve enforces the hard untimed ones); use \
                 ferroplan::solve per instance"
                .into());
        }
        if crate::pddl3::has_preferences(&problem) {
            return Err("Session does not support PDDL3 preference problems yet; \
                 use ferroplan::solve per instance"
                .into());
        }
        let threads = if opts.threads == 0 {
            crate::par::num_threads()
        } else {
            opts.threads
        };
        // Temporal domains (0.12 Phase 1): snap-compile + stratified-ground
        // ONCE; every think then solves from the CURRENT at-rest state. TILs
        // are rejected — they pin the ABSOLUTE clock, and session thinks are
        // clock-relative (recorded follow-up if the game needs scheduled
        // exogenous events).
        let mut temporal_c = if crate::temporal::is_temporal(&domain) {
            if !problem.til.is_empty() {
                return Err("Session does not support timed initial literals \
                     (a TIL pins the absolute clock; session thinks are \
                     clock-relative); use ferroplan::solve per instance"
                    .into());
            }
            Some(crate::temporal::compile(&domain, &problem))
        } else {
            None
        };
        // Force a task even when the base goal is trivially true/false — a session's
        // world moves, so the base-init verdict says nothing about later replans.
        let task = match &temporal_c {
            Some(c) => match crate::ground::ground_fixpoint(&c.domain, &c.problem, threads) {
                crate::ground::Outcome::Task(t) => t,
                _ => return Err("grounding failed (empty type)".to_string()),
            },
            None => ground_task(&domain, &problem, threads)
                .ok_or_else(|| "grounding failed (empty type)".to_string())?,
        };

        let fact_ids: FxHashMap<String, u32> = task
            .fact_names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i as u32))
            .collect();
        let mut dynamic = vec![false; task.n_facts];
        for oi in 0..task.n_ops {
            for &f in task.add.slice(oi).iter().chain(task.del.slice(oi)) {
                dynamic[f as usize] = true;
            }
            for ce in task.cond_effs(oi) {
                for &f in ce.add.iter().chain(ce.del.iter()) {
                    dynamic[f as usize] = true;
                }
            }
        }
        let fluent_ids = task
            .fluent_names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i as u32))
            .collect();

        let running_preds: Vec<String> = temporal_c
            .as_ref()
            .map(|c| c.snaps.iter().map(|s| s.running_pred.clone()).collect())
            .unwrap_or_default();
        // Mirror pairing: a fact displayed `(NOT (P ...))` complements the
        // fact displayed `(P ...)` (when the latter was interned at all).
        let mut mirror = FxHashMap::default();
        for (i, name) in task.fact_names.iter().enumerate() {
            if let Some(inner) = name.strip_prefix("(NOT ").and_then(|r| r.strip_suffix(')')) {
                if let Some(&base) = fact_ids.get(inner) {
                    mirror.insert(i as u32, base);
                    mirror.insert(base, i as u32);
                }
            }
        }
        // Scheduled-event setters (0.14 Phase 3, temporal only): per dynamic
        // fact, agenda-fired TIL ops behind the never-true fence. Appended
        // AFTER the maps above so `fact_ids` never resolves the fence fact.
        let mut task = task;
        let til_setters = match temporal_c.as_mut() {
            Some(c) => {
                let map = append_til_setters(&mut task, &dynamic, &mirror);
                for &op in map.values() {
                    c.til_ops.push((0.0, task.op_display[op].clone()));
                }
                map
            }
            None => FxHashMap::default(),
        };
        let op_ids = task
            .op_display
            .iter()
            .enumerate()
            .map(|(i, d)| (d.clone(), i))
            .collect();
        Ok(Session {
            task,
            threads,
            weight_g: opts.weight_g,
            weight_h: opts.weight_h,
            max_evaluated: opts.max_evaluated,
            ehc_first: opts.search != Search::BestFirst,
            fact_ids: Arc::new(fact_ids),
            dynamic: dynamic.into(),
            fluent_ids: Arc::new(fluent_ids),
            temporal: temporal_c.map(Arc::new),
            tier: crate::features::demand_mode(),
            running_preds,
            op_ids: Arc::new(op_ids),
            mirror: Arc::new(mirror),
            forbidden: Vec::new(),
            timed: Vec::new(),
            til_setters: Arc::new(til_setters),
            running: Vec::new(),
        })
    }

    /// Pull the trigger on a durative action, NOW (0.14 Phase 5): the
    /// start's effects land immediately — including the compiler's
    /// `RUNNING-*` brand — and the interval's end drops into the session's
    /// in-flight set, due once the action's duration burns out (resolved
    /// against CURRENT fluent readings, same as every think). From here the
    /// mind can rethink mid-burn: pending ends ride into every think and
    /// [`Session::plan_still_valid`] replays them as scheduled happenings —
    /// a returned plan holds valid THROUGH every running end (the search
    /// fires them, the goal must hold once nothing's still pending;
    /// conservative, sound). [`Session::elapse`] detonates ends as their
    /// moment arrives, so the old mirror-the-end-effects trick is dead:
    /// trigger with `apply_start`, advance with `elapse`, and the world
    /// never lies.
    ///
    /// `name` is the plan-step form, e.g. `"(fire urn)"`. Bounces on:
    /// classical sessions (no clock to run), unknown actions, and starts
    /// whose preconditions don't hold in the room right now.
    pub fn apply_start(&mut self, name: &str) -> Result<(), String> {
        let c = match &self.temporal {
            Some(c) => Arc::clone(c),
            None => {
                return Err("apply_start needs a TEMPORAL session (classical \
                     actions are instantaneous — apply them with set_fact)"
                    .into())
            }
        };
        let inner = name
            .trim()
            .strip_prefix('(')
            .and_then(|r| r.strip_suffix(')'))
            .ok_or_else(|| format!("expected `(action args...)`, got `{name}`"))?
            .to_ascii_uppercase();
        let mut words = inner.split_whitespace();
        let head = words.next().unwrap_or("");
        let rest: Vec<&str> = words.collect();
        let disp = if rest.is_empty() {
            format!("{head}-START")
        } else {
            format!("{head}-START {}", rest.join(" "))
        };
        let &start_op = self
            .op_ids
            .get(&disp)
            .ok_or_else(|| format!("unknown durative action `{name}` (no grounded `{disp}`)"))?;
        let (kind, dur_exprs, _inv) = crate::temporal::build_kind(&self.task, &c);
        let (dur, end_op) = match kind[start_op] {
            crate::temporal::Kind::Start { dur, end_op, dexp } => {
                let d = if dexp == u32::MAX {
                    dur
                } else {
                    match dur_exprs[dexp as usize].eval(&self.task.fv0, &self.task.fdef0) {
                        Some(v) if v.is_finite() && v > 0.0 => v,
                        _ => {
                            return Err(format!(
                                "duration of `{name}` does not resolve to a positive \
                                 value in the current state"
                            ))
                        }
                    }
                };
                (d, end_op)
            }
            _ => return Err(format!("`{name}` is not a durative action start")),
        };
        let start_state = self.task.initial();
        if !self.task.op_applicable(start_op, &start_state) {
            return Err(format!(
                "`{name}` is not applicable in the current state — its start \
                 preconditions do not hold"
            ));
        }
        let ns = self.task.apply(start_op, &start_state);
        self.task.init_bits = ns.bits;
        self.task.fv0 = ns.fv;
        self.task.fdef0 = ns.fdef;
        self.running.push((dur, end_op));
        self.running.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        Ok(())
    }

    /// Put this mind on a leash: only the ops `keep` accepts stay live
    /// (0.14 Phase 1). Every op whose display (e.g. `TRADE V3 V4 ITEM3
    /// ITEM4`) gets rejected turns FORBIDDEN — never picked by a think, and
    /// any plan step riding one fails [`Session::plan_still_valid`] / the
    /// [`Session::replan_following`] prefix replay. This is the discipline
    /// that keeps a mind in a many-minds theater confined to its OWN moves:
    /// leash it to what it can actually take, and a rival's trades reach it
    /// as `set_fact` drift — never as plan steps up for grabs.
    ///
    /// The leash is part of the mind's identity: [`Session::fork`] carries
    /// it over (re-leash the fork to change it), determinism holds firm
    /// (the mask is just another input, t1 ≡ t8), and completeness stays
    /// honest — a goal out of reach within the allowed ops comes back
    /// `solved: false`, never an error. Call again and the mask gets fully
    /// REPLACED; `restrict_ops(|_| true)` cuts the leash. World state
    /// (`set_fact` / `set_fluent`) is never touched by the mask.
    pub fn restrict_ops(&mut self, mut keep: impl FnMut(&str) -> bool) {
        let mask: Vec<bool> = self.task.op_display.iter().map(|d| !keep(d)).collect();
        self.forbidden = if mask.iter().any(|&f| f) {
            mask
        } else {
            Vec::new()
        };
    }

    /// Split a second mind off this one (0.13 Phase 2): an independent
    /// `Session` riding the SAME grounded world. N minds, ONE grounding —
    /// the fork shares the grounded payload (operator columns, names,
    /// achiever indexes, the temporal compilation) behind `Arc` and copies
    /// only the cheap per-mind cargo: current facts and fluents, goal,
    /// fluent relevance.
    ///
    /// The fork inherits this session's CURRENT state and goal (not the
    /// problem's `:init`), then drifts on its own: its [`Session::set_fact`]
    /// / [`Session::set_goal`] / [`Session::replan`] never bleed into a
    /// sibling — no shared tie-breaks, no cross-mind bleed-through. Each
    /// fork keeps the parent's options (threads, weights, budget) and
    /// demand tier.
    pub fn fork(&self) -> Session {
        Session {
            task: self.task.clone(),
            threads: self.threads,
            weight_g: self.weight_g,
            weight_h: self.weight_h,
            max_evaluated: self.max_evaluated,
            ehc_first: self.ehc_first,
            fact_ids: Arc::clone(&self.fact_ids),
            dynamic: Arc::clone(&self.dynamic),
            fluent_ids: Arc::clone(&self.fluent_ids),
            temporal: self.temporal.clone(),
            tier: self.tier,
            running_preds: self.running_preds.clone(),
            op_ids: Arc::clone(&self.op_ids),
            mirror: Arc::clone(&self.mirror),
            forbidden: self.forbidden.clone(),
            timed: self.timed.clone(),
            til_setters: Arc::clone(&self.til_setters),
            running: self.running.clone(),
        }
    }

    /// The temporal think (0.12 Phase 1): rebuild the duration table against
    /// the CURRENT fluent values (so `set_fluent` on an op-unmodified fluent
    /// flows into parameter-dependent durations), then run the bounded
    /// decision-epoch ladder from the current at-rest state. The eval budget
    /// spans the WHOLE pass ladder; the memory target plumbs to the
    /// deterministic temporal node cap. No escalation beyond the ladder, no
    /// decomposer — a think is a bounded call, not a campaign.
    fn replan_temporal(
        &self,
        c: &crate::temporal::TemporalCompiled,
        budget_evals: Option<usize>,
        memory_mb: Option<usize>,
    ) -> Solution {
        let start = self.task.initial();
        // A running interval's end could still UNMEET the goal — only an
        // at-rest world takes the trivial exit; in-flight worlds run the
        // search, which verifies the goal holds through every pending end.
        if self.task.goal_met(&start) && self.running.is_empty() {
            return Solution {
                solved: true,
                mode: Mode::Temporal,
                plan: Some(Plan {
                    steps: Vec::new(),
                    length: 0,
                    metric: None,
                    makespan: Some(0.0),
                }),
                statistics: stats(&self.task, 0, self.threads),
                notes: vec!["goal already satisfied; the empty plan solves it".into()],
            };
        }
        let (kind, dur_exprs, inv) = crate::temporal::build_kind(&self.task, c);
        let total = budget_evals.or(self.max_evaluated).unwrap_or(usize::MAX);
        let mut remaining = total;
        let node_bytes = memory_mb
            .map(|mb| mb.saturating_mul(1 << 20))
            .unwrap_or(crate::search::NODE_CAP_TARGET_BYTES);
        // Pending scheduled events AND running-interval ends ride in as the
        // think's root agenda (times are relative to NOW, this think's clock
        // zero). Ends are real ops with real preconditions; the search fires
        // them at their moments and a goal only counts once no action end is
        // pending — a returned plan is valid THROUGH the running intervals.
        let mut til_events: Vec<(f64, usize)> = self
            .timed
            .iter()
            .map(|&(t, f, v)| (t, self.til_setters[&(f, v)]))
            .collect();
        til_events.extend(self.running.iter().copied());
        til_events.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        let tp = crate::temporal::solve_from_seeded(
            &self.task,
            &kind,
            &dur_exprs,
            &inv,
            &start,
            &self.task.goal_pos,
            &self.task.goal_num,
            &self.forbidden,
            &til_events,
            self.threads,
            self.tier,
            &mut remaining,
            node_bytes,
            true, // pending events seed h: waits through repaired outages
        );
        let evaluated = total.saturating_sub(remaining);
        match tp {
            Some(tp) => {
                let steps = timed_steps(&tp);
                Solution {
                    solved: true,
                    mode: Mode::Temporal,
                    plan: Some(Plan {
                        length: steps.len(),
                        steps,
                        metric: None,
                        makespan: Some(tp.makespan),
                    }),
                    statistics: stats(&self.task, evaluated, self.threads),
                    notes: Vec::new(),
                }
            }
            None => Solution {
                solved: false,
                mode: Mode::Temporal,
                plan: None,
                statistics: stats(&self.task, evaluated, self.threads),
                notes: Vec::new(),
            },
        }
    }

    /// Follow before you rethink (0.12 Phase 2): does the plan's remaining
    /// suffix (steps `from_step..`) still play out from the CURRENT world
    /// state and land the goal? A `true` costs one replay — no search burned,
    /// no think budget spent — so a mind whose world drifted IRRELEVANTLY
    /// keeps following its plan for free; only a broken suffix earns a real
    /// rethink. Exact, no heuristics: classical suffixes replay step by step
    /// (applicability, effects), temporal suffixes replay their timed
    /// happenings in epoch order (the internal validator's own machinery),
    /// and both close on the goal test.
    pub fn plan_still_valid(&self, plan: &Plan, from_step: usize) -> bool {
        let steps = match plan.steps.get(from_step..) {
            Some(s) => s,
            None => return false,
        };
        let display = |s: &crate::api::Step| {
            if s.args.is_empty() {
                s.action.clone()
            } else {
                format!("{} {}", s.action, s.args.join(" "))
            }
        };
        if self.temporal.is_some() {
            let mut tp = crate::temporal::TimedPlan {
                steps: steps
                    .iter()
                    .map(|s| crate::temporal::TimedStep {
                        time: s.time.unwrap_or(0.0),
                        action: display(s),
                        duration: s.duration,
                    })
                    .collect(),
                makespan: 0.0,
            };
            // The suffix replays WITH the scheduled events it would live
            // through (0.14 Phase 3): pending events inside the plan's span
            // join the happening list. Events strictly after the plan
            // completes are the game's future, not the plan's problem.
            let horizon = steps
                .iter()
                .map(|s| s.time.unwrap_or(0.0) + s.duration.unwrap_or(0.0))
                .fold(0.0, f64::max)
                .max(self.running.iter().map(|&(t, _)| t).fold(0.0, f64::max));
            for &(t, f, v) in &self.timed {
                if t <= horizon {
                    tp.steps.push(crate::temporal::TimedStep {
                        time: t,
                        action: self.task.op_display[self.til_setters[&(f, v)]].clone(),
                        duration: None,
                    });
                }
            }
            // Running-interval ends are REAL happenings the suffix must
            // survive (their preconditions checked in replay — drift that
            // breaks an interval breaks every plan living through it).
            for &(t, end_op) in &self.running {
                tp.steps.push(crate::temporal::TimedStep {
                    time: t,
                    action: self.task.op_display[end_op].clone(),
                    duration: None,
                });
            }
            let mut exempt: Vec<usize> = self.til_setters.values().copied().collect();
            exempt.sort_unstable();
            return match crate::temporal::treplay_with_exempt(
                &self.task,
                &self.task.initial(),
                &tp,
                &exempt,
            ) {
                Some(end) => self.task.goal_met(&end),
                None => false,
            };
        }
        let mut state = self.task.initial();
        for s in steps {
            let oi = match self.op_ids.get(&display(s)) {
                Some(&oi) => oi,
                None => return false,
            };
            // A step this mind may not take invalidates the plan for THIS
            // mind, however applicable the world finds it.
            if self.forbidden.get(oi).copied().unwrap_or(false)
                || !self.task.op_applicable(oi, &state)
            {
                return false;
            }
            state = self.task.apply(oi, &state);
        }
        self.task.goal_met(&state)
    }

    /// Flip a world fact true or false right now, e.g.
    /// `set_fact("(at v1 field)", true)`. Case-insensitive. Bounces if the
    /// fact was never grounded, or is static (baked in at grounding — see
    /// the dispatch above). If grounding minted the complementary
    /// `(NOT (p ...))` shadow fact, it moves too — set either side, both
    /// flip together, no drift.
    pub fn set_fact(&mut self, name: &str, value: bool) -> Result<(), String> {
        let id = self.dynamic_fact_id(name)?;
        self.write_fact_bit(id, value);
        if let Some(&m) = self.mirror.get(&id) {
            self.write_fact_bit(m, !value);
        }
        Ok(())
    }

    /// The mind's eyes, nothing more (0.15 Phase 4): report ground truth for
    /// the facts this mind can currently SEE. Belief (this session's state)
    /// snaps to match; anything outside `sight` keeps its old believed value
    /// — that's the fog line. Returns the sighted displays whose observed
    /// value DIFFERED from belief, uppercased — raw intel for a rethink
    /// decision: feed it to [`Session::plan_still_valid`] /
    /// [`Session::goal_met`] and see if the world moved under the plan
    /// (surprise-driven rethinks, not clock-watching paranoia). Same
    /// writability fences as [`Session::set_fact`]; the whole batch checks
    /// out clean before any belief moves — one bad sighting, nothing changes.
    pub fn observe(&mut self, sight: &[(&str, bool)]) -> Result<Vec<String>, String> {
        let mut ids = Vec::with_capacity(sight.len());
        for (name, v) in sight {
            ids.push((self.dynamic_fact_id(name)?, *v));
        }
        let mut surprises = Vec::new();
        for ((name, _), (id, v)) in sight.iter().zip(ids) {
            let (w, b) = (id as usize / 64, id as usize % 64);
            let believed = self.task.init_bits[w] >> b & 1 == 1;
            if believed != v {
                surprises.push(name.to_ascii_uppercase());
                self.write_fact_bit(id, v);
                if let Some(&m) = self.mirror.get(&id) {
                    self.write_fact_bit(m, !v);
                }
            }
        }
        Ok(surprises)
    }

    /// Run a fact display through the checkpoint shared by
    /// [`Session::set_fact`] and [`Session::set_timed_fact`]: the fact must
    /// be grounded, DYNAMIC (statics are locked at grounding), and not a
    /// compiler-internal `RUNNING-*` token (the at-rest fence — `atom_head`
    /// sees through `(NOT ...)` so a mirror can't slip past disguised).
    fn dynamic_fact_id(&self, name: &str) -> Result<u32, String> {
        let key = name.to_ascii_uppercase();
        let &id = self
            .fact_ids
            .get(&key)
            .ok_or_else(|| format!("unknown fact `{key}` (not in the grounded task)"))?;
        if !self.dynamic[id as usize] {
            return Err(format!(
                "fact `{key}` is static — grounding baked it in; changing it could \
                 require operators that were never enumerated"
            ));
        }
        if !self.running_preds.is_empty() {
            let head = atom_head(&key);
            if self.running_preds.iter().any(|p| p == head) {
                return Err(format!(
                    "fact `{key}` is a compiler-internal running-interval token; \
                     a session's world is at rest between thinks — mirror the \
                     action's end effects instead"
                ));
            }
        }
        Ok(id)
    }

    /// Plant a timer on the world (0.14 Phase 3, temporal sessions): in `dt`
    /// time units from now, the fact flips to `value` — the
    /// market-opens-at-nine shape, clock-RELATIVE (every think in a session
    /// runs clock-relative, which is exactly why absolute-clock TILs get
    /// bounced at construction). Pending timers ride into every think as
    /// think-relative events the plan has to reckon with: beat the closing
    /// window or fail honest, and [`Session::plan_still_valid`] replays a
    /// suffix WITH the events it would actually live through. As the
    /// game-clock ticks forward, call [`Session::elapse`] to burn down the
    /// delays and detonate due events into the state.
    ///
    /// Checkpoints: same writability rules as [`Session::set_fact`]; `dt`
    /// must be positive and finite (for "now," use `set_fact` instead);
    /// classical sessions reject scheduling outright — no clock, no timers.
    /// The DYNAMIC-fact checkpoint bites twice as hard here — a truly static
    /// fact is compiled INTO the grounded ops (stripped from runtime
    /// preconditions), so flipping it by timer changes nothing soundly.
    /// Model an exogenous-changeable fact (market-open, power) with SOME
    /// domain action touching it, and only then does it become schedulable.
    ///
    /// Waiting works. The agenda carries pending events, so a think can idle
    /// straight THROUGH an outage and strike the moment an enabler returns.
    /// The honest limit: a goal whose only enabler arrives via a timer never
    /// grounds at all — the fact space can't express it — and fails loud at
    /// construction, not silently downstream.
    pub fn set_timed_fact(&mut self, dt: f64, name: &str, value: bool) -> Result<(), String> {
        if self.temporal.is_none() {
            return Err("scheduled events need a TEMPORAL session (a classical \
                 session has no clock); flip the fact when it happens with \
                 set_fact instead"
                .into());
        }
        if !(dt.is_finite() && dt > 0.0) {
            return Err(format!(
                "event delay must be a positive, finite time offset (got {dt}); \
                 for `now`, use set_fact"
            ));
        }
        let id = self.dynamic_fact_id(name)?;
        self.timed.push((dt, id, value));
        self.timed.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
                .then(a.2.cmp(&b.2))
        });
        Ok(())
    }

    /// The clock moved forward by `dt`. Burn down every pending schedule
    /// entry, DETONATE those whose moment has passed, in time order —
    /// scheduled events ([`Session::set_timed_fact`], mirrors stay synced)
    /// and running-interval ENDS ([`Session::apply_start`]'s at-end
    /// effects). Returns the intervals whose ends could NOT fire — an end
    /// whose preconditions no longer hold means drift snapped the interval
    /// mid-flight; its effects get dropped and the game decides what that
    /// means. World facts the game itself changes still route through
    /// [`Session::set_fact`] — `elapse` only marches the schedule forward.
    pub fn elapse(&mut self, dt: f64) -> Result<Vec<String>, String> {
        if !(dt.is_finite() && dt >= 0.0) {
            return Err(format!("elapse needs a non-negative, finite dt (got {dt})"));
        }
        // Merge both schedules into one due list, fired in time order.
        enum Due {
            Event(u32, bool),
            End(usize),
        }
        let mut due: Vec<(f64, Due)> = Vec::new();
        let timed = std::mem::take(&mut self.timed);
        for (t, id, value) in timed {
            if t <= dt {
                due.push((t, Due::Event(id, value)));
            } else {
                self.timed.push((t - dt, id, value));
            }
        }
        let running = std::mem::take(&mut self.running);
        for (t, end_op) in running {
            if t <= dt {
                due.push((t, Due::End(end_op)));
            } else {
                self.running.push((t - dt, end_op));
            }
        }
        due.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut broken = Vec::new();
        for (_, d) in due {
            match d {
                Due::Event(id, value) => {
                    self.write_fact_bit(id, value);
                    let m = self.mirror.get(&id).copied();
                    if let Some(m) = m {
                        self.write_fact_bit(m, !value);
                    }
                }
                Due::End(end_op) => {
                    let state = self.task.initial();
                    if self.task.op_applicable(end_op, &state) {
                        let ns = self.task.apply(end_op, &state);
                        self.task.init_bits = ns.bits;
                        self.task.fv0 = ns.fv;
                        self.task.fdef0 = ns.fdef;
                    } else {
                        broken.push(self.task.op_display[end_op].clone());
                    }
                }
            }
        }
        Ok(broken)
    }

    fn write_fact_bit(&mut self, id: u32, value: bool) {
        let (w, b) = (id as usize / 64, id as usize % 64);
        if value {
            self.task.init_bits[w] |= 1 << b;
        } else {
            self.task.init_bits[w] &= !(1 << b);
        }
    }

    /// Set a fluent's live value, e.g. `set_fluent("(grain)", 3.0)`.
    /// Case-insensitive. Bounces if the fluent was never grounded.
    pub fn set_fluent(&mut self, name: &str, value: f64) -> Result<(), String> {
        let key = name.to_ascii_uppercase();
        let &id = self
            .fluent_ids
            .get(&key)
            .ok_or_else(|| format!("unknown fluent `{key}` (not in the grounded task)"))?;
        self.task.fv0[id as usize] = value;
        self.task.fdef0[id as usize] = true;
        Ok(())
    }

    /// Rewrite the mission (0.13 Phase 1): swap the goal for a GROUND
    /// conjunction over the already-interned fact space — no regrounding,
    /// no re-parse of the world. Same world, new want: a runner who wanted
    /// iron and now wants bread swaps orders and keeps moving.
    ///
    /// Accepted grammar (same `(:goal ...)` body syntax): nested `(and ...)`
    /// of ground atoms `(pred obj ...)`, negated atoms `(not (pred obj
    /// ...))` where grounding minted the complementary `(NOT ...)` shadow
    /// fact, and numeric comparisons `(>= (fluent ...) expr)`. An empty
    /// `(and)` is the goal that's always already won.
    ///
    /// Bounces — before touching the live goal — on: atoms/fluents the
    /// grounded world never carried (statics and unreachable atoms get
    /// compiled away — a mind can't want what its world can't express),
    /// negations with no grounded mirror, compiler-reserved `RUNNING-*`
    /// tokens (a temporal world stands at rest between thinks), non-ground
    /// terms, and ADL connectives (`or`/`exists`/`forall`/object `=`) — those
    /// compile at grounding time, so stand up a fresh `Session` for an ADL
    /// goal.
    ///
    /// A numeric goal may reach for a fluent the original goal never
    /// touched: the visited-key relevance closure re-runs with the new
    /// fluents folded in. Relevance only ever GROWS inside a session — state
    /// keys sharpen, never blur — so replay soundness and t1 ≡ t8
    /// determinism hold firm across every retarget.
    pub fn set_goal(&mut self, goal: &str) -> Result<(), String> {
        let f = crate::parser::parse_goal(goal)?;
        let mut pos: Vec<u32> = Vec::new();
        let mut num: Vec<NumPre> = Vec::new();
        self.goal_conj(&f, &mut pos, &mut num)?;
        pos.sort_unstable();
        pos.dedup();

        // Fluents newly read by this goal join the visited-key relevance
        // closure (see `PackedTask::state_key`): a fluent outside the key
        // cannot distinguish states, which is only sound while no
        // precondition or GOAL reads it. Mirror of ground.rs's closure —
        // any fluent read by an effect that writes a relevant fluent is
        // itself relevant.
        let mut rel = std::mem::take(&mut self.task.relevant_fluent);
        let mut scratch = Vec::new();
        let mut grew = false;
        for np in &num {
            np.lhs.collect_fluents(&mut scratch);
            np.rhs.collect_fluents(&mut scratch);
        }
        for &fl in &scratch {
            if !rel[fl as usize] {
                rel[fl as usize] = true;
                grew = true;
            }
        }
        if grew {
            loop {
                let mut changed = false;
                for oi in 0..self.task.n_ops {
                    let neffs = self
                        .task
                        .num_eff
                        .slice(oi)
                        .iter()
                        .chain(self.task.cond_effs(oi).flat_map(|c| c.num.iter()));
                    for ne in neffs {
                        if !rel[ne.target as usize] {
                            continue;
                        }
                        scratch.clear();
                        ne.value.collect_fluents(&mut scratch);
                        for &fl in &scratch {
                            if !rel[fl as usize] {
                                rel[fl as usize] = true;
                                changed = true;
                            }
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
            self.task.rel_fluents = (0..rel.len() as u32).filter(|&i| rel[i as usize]).collect();
        }
        self.task.relevant_fluent = rel;

        self.task.goal_pos = pos;
        self.task.goal_num = num;
        Ok(())
    }

    /// Collapse a goal formula into the packed conjunction, checking every
    /// literal against the interned fact space. See [`Session::set_goal`].
    fn goal_conj(
        &self,
        f: &Formula,
        pos: &mut Vec<u32>,
        num: &mut Vec<NumPre>,
    ) -> Result<(), String> {
        match f {
            Formula::And(fs) => {
                for g in fs {
                    self.goal_conj(g, pos, num)?;
                }
                Ok(())
            }
            Formula::True => Ok(()),
            Formula::Atom(p, args) => {
                pos.push(self.goal_atom(p, args, false)?);
                Ok(())
            }
            Formula::Not(inner) => match &**inner {
                Formula::Atom(p, args) => {
                    pos.push(self.goal_atom(p, args, true)?);
                    Ok(())
                }
                _ => Err("set_goal supports `(not ...)` only directly around a \
                     ground atom"
                    .into()),
            },
            Formula::Comp(op, l, r) => {
                num.push(NumPre {
                    op: *op,
                    lhs: self.goal_nexpr(l)?,
                    rhs: self.goal_nexpr(r)?,
                });
                Ok(())
            }
            Formula::Or(_) | Formula::Exists(..) | Formula::Forall(..) | Formula::Eq(..) => Err(
                "set_goal supports ground conjunctions (atoms, negated atoms \
                 with grounded mirrors, numeric comparisons); ADL goal \
                 connectives compile at grounding time — re-create the Session \
                 with this goal instead"
                    .into(),
            ),
            Formula::Pref(..) => {
                Err("set_goal does not support PDDL3 preferences (soft goals)".into())
            }
            Formula::False => Err("goal `false` is unsatisfiable by construction".into()),
        }
    }

    /// Trace one ground goal literal down to its fact id (the mirror fact
    /// when the literal is negated).
    fn goal_atom(&self, p: &str, args: &[Term], negated: bool) -> Result<u32, String> {
        let mut disp = String::from("(");
        disp.push_str(&p.to_ascii_uppercase());
        for a in args {
            match a {
                Term::Const(c) => {
                    disp.push(' ');
                    disp.push_str(&c.to_ascii_uppercase());
                }
                Term::Var(v) => {
                    return Err(format!(
                        "goal must be ground — variable `{v}` in goal atom `({p} ...)`"
                    ))
                }
            }
        }
        disp.push(')');
        if self
            .running_preds
            .iter()
            .any(|rp| rp == &p.to_ascii_uppercase())
        {
            return Err(format!(
                "goal atom `{disp}` is a compiler-internal running-interval \
                 token; a session's world is at rest between thinks"
            ));
        }
        if negated {
            let mirror_disp = format!("(NOT {disp})");
            return self.fact_ids.get(&mirror_disp).copied().ok_or_else(|| {
                format!(
                    "negative goal literal `(not {disp})` has no grounded mirror \
                     fact — grounding creates `(NOT ...)` mirrors only for atoms \
                     that occur negatively in the domain or the original goal; \
                     re-create the Session with this goal instead"
                )
            });
        }
        self.fact_ids.get(&disp).copied().ok_or_else(|| {
            format!(
                "goal atom `{disp}` is not in the grounded fact space (statics \
                 and unreachable atoms are compiled away; a session cannot want \
                 what its world cannot express)"
            )
        })
    }

    /// Cast a goal's numeric expression down onto the interned fluent space.
    fn goal_nexpr(&self, e: &Expr) -> Result<NExpr, String> {
        Ok(match e {
            Expr::Num(n) => NExpr::Num(*n),
            Expr::Fluent(name, args) => {
                let mut disp = String::from("(");
                disp.push_str(&name.to_ascii_uppercase());
                for a in args {
                    match a {
                        Term::Const(c) => {
                            disp.push(' ');
                            disp.push_str(&c.to_ascii_uppercase());
                        }
                        Term::Var(v) => {
                            return Err(format!(
                                "goal must be ground — variable `{v}` in fluent `({name} ...)`"
                            ))
                        }
                    }
                }
                disp.push(')');
                let &id = self
                    .fluent_ids
                    .get(&disp)
                    .ok_or_else(|| format!("unknown fluent `{disp}` (not in the grounded task)"))?;
                NExpr::Fluent(id)
            }
            Expr::Add(a, b) => {
                NExpr::Add(Box::new(self.goal_nexpr(a)?), Box::new(self.goal_nexpr(b)?))
            }
            Expr::Sub(a, b) => {
                NExpr::Sub(Box::new(self.goal_nexpr(a)?), Box::new(self.goal_nexpr(b)?))
            }
            Expr::Mul(a, b) => {
                NExpr::Mul(Box::new(self.goal_nexpr(a)?), Box::new(self.goal_nexpr(b)?))
            }
            Expr::Div(a, b) => {
                NExpr::Div(Box::new(self.goal_nexpr(a)?), Box::new(self.goal_nexpr(b)?))
            }
            Expr::Neg(a) => NExpr::Neg(Box::new(self.goal_nexpr(a)?)),
        })
    }

    /// The weight of the SHARED grounded payload, estimated in bytes —
    /// operator columns, names, achiever indexes, the monitor block. Exists
    /// once per world no matter how many forks live off it
    /// ([`Session::fork`]); flat array/string bytes only counted (nested
    /// conditional-effect allocations go unwalked) — read this as a floor,
    /// not a full audit.
    pub fn world_bytes(&self) -> usize {
        use std::mem::size_of_val;
        let t = &self.task;
        let strings = |v: &[String]| {
            v.iter()
                .map(|s| s.len() + size_of::<String>())
                .sum::<usize>()
        };
        strings(&t.op_display)
            + strings(&t.fact_names)
            + strings(&t.fluent_names)
            + size_of_val(&t.pre_pos.flat[..])
            + size_of_val(&t.add.flat[..])
            + size_of_val(&t.del.flat[..])
            + size_of_val(&t.pre_num.flat[..])
            + size_of_val(&t.num_eff.flat[..])
            + size_of_val(&t.cond.flat[..])
            + size_of_val(&t.shared_cond[..])
            + size_of_val(&t.monitored[..])
            + size_of_val(&t.add_by_fact.flat[..])
            + size_of_val(&t.neff_by_fluent.flat[..])
    }

    /// The weight of this mind's PRIVATE state, estimated — current facts
    /// and fluents, goal, fluent relevance. This is the toll one more
    /// [`Session::fork`] pays; same flat-bytes caveat as
    /// [`Session::world_bytes`].
    pub fn mind_bytes(&self) -> usize {
        use std::mem::size_of_val;
        let t = &self.task;
        size_of_val(&t.init_bits[..])
            + size_of_val(&t.fv0[..])
            + size_of_val(&t.fdef0[..])
            + size_of_val(&t.goal_pos[..])
            + size_of_val(&t.goal_num[..])
            + size_of_val(&t.relevant_fluent[..])
            + size_of_val(&t.rel_fluents[..])
    }

    /// Does the CURRENT world satisfy the mission, right now? A pure state
    /// check — no search, no plan, no think budget spent (0.14 Phase 1: the
    /// tick loop's "am I done" probe; a zero-budget think answers a
    /// different question — "could I still find a plan" — and a near-done
    /// mind must never confuse the two).
    pub fn goal_met(&self) -> bool {
        self.task.goal_met(&self.task.initial())
    }

    /// Read a fact off the current state (`None` if it was never grounded).
    pub fn fact(&self, name: &str) -> Option<bool> {
        let &id = self.fact_ids.get(&name.to_ascii_uppercase())?;
        Some((self.task.init_bits[id as usize / 64] >> (id as usize % 64)) & 1 == 1)
    }

    /// Read a fluent's live value (`None` if never grounded, or undefined).
    pub fn fluent(&self, name: &str) -> Option<f64> {
        let &id = self.fluent_ids.get(&name.to_ascii_uppercase())?;
        self.task.fdef0[id as usize].then(|| self.task.fv0[id as usize])
    }

    /// [`Self::replan`] on a leash — an explicit THINK BUDGET (0.11 Phase 4,
    /// the game-embedding surface): `max_evaluated` caps states evaluated
    /// (the deterministic unit, never the wall clock) and `memory_mb` caps
    /// the search's retained memory through the deterministic per-node byte
    /// model (see `search::node_cap_for_bytes`). A budget burned dry
    /// returns `solved: false`, honest, no fudging; identical inputs land
    /// identical results at any thread count.
    pub fn replan_budgeted(&self, max_evaluated: usize, memory_mb: Option<usize>) -> Solution {
        self.replan_inner(Some(max_evaluated), memory_mb)
    }

    /// Solve from the CURRENT world state toward the mission, paying only for
    /// the search — no re-parse, no re-ground. Same structured [`Solution`]
    /// as [`crate::solve`]; `solved: false` when the goal can't be reached
    /// from here.
    pub fn replan(&self) -> Solution {
        self.replan_inner(None, None)
    }

    /// Follow, don't dither (0.13 Phase 4): a bounded rethink BIASED toward
    /// the broken plan's own shape. When drift snaps a plan mid-flight, an
    /// unconstrained rethink can thrash sideways into a structurally
    /// different plan — visible dithering even when both plans work fine.
    /// This variant replays the still-applicable PREFIX of the old plan's
    /// remaining suffix (steps `from_step..`, up to the first step that no
    /// longer applies — pure replay, zero search), then searches only for a
    /// new TAIL from where the prefix runs out: the new plan shares the
    /// prefix by construction, so churn stays confined to the piece drift
    /// actually broke.
    ///
    /// The bias can cost budget, never completeness, never honesty: if the
    /// goal is met anywhere along the prefix, the plan gets cut right there
    /// — no search spent. If NO tail exists past the prefix's end (the
    /// prefix may have wandered somewhere the new goal can't be reached
    /// from), the rethink falls back to an unbiased
    /// [`Session::replan_budgeted`] on the same budget, and the returned
    /// statistics count BOTH searches. Both legs are the same ordinary
    /// deterministic bounded think (t1 ≡ t8).
    ///
    /// The bias only rides classical sessions — a temporal plan's prefix
    /// ends mid-interval, not the at-rest state a session may be standing
    /// in, so temporal sessions fall straight to the plain bounded think.
    pub fn replan_following(
        &self,
        prior: &Plan,
        from_step: usize,
        max_evaluated: usize,
        memory_mb: Option<usize>,
    ) -> Solution {
        if self.temporal.is_some() {
            return self.replan_following_temporal(prior, from_step, max_evaluated, memory_mb);
        }
        let display = |s: &crate::api::Step| {
            if s.args.is_empty() {
                s.action.clone()
            } else {
                format!("{} {}", s.action, s.args.join(" "))
            }
        };
        let mut state = self.task.initial();
        let mut prefix: Vec<crate::api::Step> = Vec::new();
        for s in prior.steps.get(from_step..).unwrap_or(&[]) {
            if self.task.goal_met(&state) {
                break;
            }
            let oi = match self.op_ids.get(&display(s)) {
                Some(&oi)
                    if !self.forbidden.get(oi).copied().unwrap_or(false)
                        && self.task.op_applicable(oi, &state) =>
                {
                    oi
                }
                _ => break,
            };
            state = self.task.apply(oi, &state);
            prefix.push(s.clone());
        }
        let renumber = |mut steps: Vec<crate::api::Step>| {
            for (i, s) in steps.iter_mut().enumerate() {
                s.index = i;
            }
            steps
        };
        if self.task.goal_met(&state) {
            let steps = renumber(prefix);
            return Solution {
                solved: true,
                mode: Mode::Ff,
                plan: Some(Plan {
                    length: steps.len(),
                    steps,
                    metric: None,
                    makespan: None,
                }),
                statistics: stats(&self.task, 0, self.threads),
                notes: vec!["prior plan's own steps reach the goal (pure replay)".into()],
            };
        }
        // Search for a tail from the prefix end. The shared payload (Phase 2)
        // makes this seeded task view an Arc bump plus small state vectors.
        let mut seeded = self.task.clone();
        seeded.init_bits = state.bits;
        seeded.fv0 = state.fv;
        seeded.fdef0 = state.fdef;
        let mut cfg = crate::search::SearchCfg::from_weights(
            self.weight_g,
            self.weight_h,
            Some(max_evaluated),
        );
        cfg.node_bytes_target = memory_mb.map(|mb| mb.saturating_mul(1 << 20));
        let o = search::plan_avoiding(&seeded, self.threads, cfg, self.ehc_first, &self.forbidden);
        match o.ops {
            Some(ops) => {
                let followed = prefix.len();
                let mut steps = prefix;
                steps.extend(steps_of(&seeded, &ops, None));
                let steps = renumber(steps);
                Solution {
                    solved: true,
                    mode: Mode::Ff,
                    plan: Some(Plan {
                        length: steps.len(),
                        steps,
                        metric: None,
                        makespan: None,
                    }),
                    statistics: stats(&self.task, o.evaluated, self.threads),
                    notes: vec![format!(
                        "followed {followed} still-applicable step(s) of the prior \
                         plan; searched only the tail"
                    )],
                }
            }
            None => {
                // No tail from the prefix end — unbiased rethink, honest totals.
                let mut sol = self.replan_inner(Some(max_evaluated), memory_mb);
                sol.statistics.evaluated_states += o.evaluated;
                sol.notes.push(
                    "prefix-follow found no tail within budget; fell back to an \
                     unbiased rethink"
                        .into(),
                );
                sol
            }
        }
    }

    /// The temporal arm of [`Session::replan_following`] (0.14 ext Phase 9),
    /// UNLOCKED by in-flight intervals: replay the prior timed plan's
    /// happenings — starts, ends, this session's scheduled events, and its
    /// REAL running ends, walked on the ε grid, events and ends firing
    /// before same-instant starts — until the first one that no longer
    /// applies. The replay's stopping point simulates an IN-FLIGHT state:
    /// intervals whose starts replayed but whose ends lie past the break
    /// carry straight into the tail think as its root agenda, same as real
    /// running intervals. The returned plan is the surviving prefix
    /// (original times) plus the tail (times shifted to the break moment)
    /// — churn confined to what drift actually broke. No tail found means
    /// an unbiased fallback from the REAL state, combined eval counts; the
    /// bias never costs completeness.
    fn replan_following_temporal(
        &self,
        prior: &Plan,
        from_step: usize,
        max_evaluated: usize,
        memory_mb: Option<usize>,
    ) -> Solution {
        let c = self.temporal.as_ref().expect("temporal arm").clone();
        let eps = crate::temporal::EPS;
        let steps: &[crate::api::Step] = prior.steps.get(from_step..).unwrap_or(&[]);
        enum Hap {
            /// suffix-step index, start (or instantaneous) op
            Start(usize, usize),
            /// end op — trailing a suffix step's interval, or a REAL running one
            End(usize),
            /// scheduled-event setter — detonates no questions asked
            Til(usize),
        }
        let display = |s: &crate::api::Step| {
            if s.args.is_empty() {
                s.action.clone()
            } else {
                format!("{} {}", s.action, s.args.join(" "))
            }
        };
        let mut haps: Vec<(f64, i8, Hap)> = Vec::new();
        for (si, s) in steps.iter().enumerate() {
            let t = s.time.unwrap_or(0.0);
            let d = display(s);
            match s.duration {
                Some(dur) => {
                    let mut it = d.splitn(2, ' ');
                    let head = it.next().unwrap_or("");
                    let rest = it.next();
                    let with = |suffix: &str| match rest {
                        Some(r) => format!("{head}{suffix} {r}"),
                        None => format!("{head}{suffix}"),
                    };
                    let (so, eo) = match (
                        self.op_ids.get(&with("-START")),
                        self.op_ids.get(&with("-END")),
                    ) {
                        (Some(&so), Some(&eo)) => (so, eo),
                        // Unmappable step: nothing to follow — plain rethink.
                        _ => return self.replan_inner(Some(max_evaluated), memory_mb),
                    };
                    haps.push((t, 1, Hap::Start(si, so)));
                    haps.push((t + dur, 0, Hap::End(eo)));
                }
                None => match self.op_ids.get(&d) {
                    Some(&o) => haps.push((t, 1, Hap::Start(si, o))),
                    None => return self.replan_inner(Some(max_evaluated), memory_mb),
                },
            }
        }
        for &(t, f, v) in &self.timed {
            haps.push((t, 0, Hap::Til(self.til_setters[&(f, v)])));
        }
        for &(t, eo) in &self.running {
            haps.push((t, 0, Hap::End(eo)));
        }
        haps.sort_by_key(|&(t, class, _)| ((t / eps).round() as i64, class));

        // Replay UP TO THE FOLLOWED PLAN'S SPAN (its last plan happening):
        // the mind acts again at the break (or at the plan's end) — anything
        // scheduled beyond that moment is the tail's future and CARRIES
        // instead of applying. Track which suffix starts applied and which
        // plan intervals are open (end op + absolute end time).
        let horizon = haps
            .iter()
            .filter(|(_, _, h)| matches!(h, Hap::Start(..)) || matches!(h, Hap::End(_)))
            .map(|&(t, _, _)| t)
            .fold(0.0, f64::max);
        let mut state = self.task.initial();
        let mut applied: Vec<bool> = vec![false; steps.len()];
        let mut open: Vec<(f64, usize)> = Vec::new();
        let mut break_t: Option<f64> = None;
        for &(t, _, ref h) in &haps {
            if break_t.is_some() || (t / eps).round() > (horizon / eps).round() {
                break;
            }
            match *h {
                Hap::Til(op) => state = self.task.apply(op, &state),
                Hap::Start(si, op) => {
                    if !self.forbidden.get(op).copied().unwrap_or(false)
                        && self.task.op_applicable(op, &state)
                    {
                        state = self.task.apply(op, &state);
                        applied[si] = true;
                        if let Some(d) = steps[si].duration {
                            let et = steps[si].time.unwrap_or(0.0) + d;
                            let eo = haps.iter().find_map(|&(ht, _, ref eh)| match *eh {
                                Hap::End(eo) if ((ht - et) / eps).abs() < 0.5 => Some(eo),
                                _ => None,
                            });
                            if let Some(eo) = eo {
                                open.push((et, eo));
                            }
                        }
                    } else {
                        break_t = Some(t);
                    }
                }
                Hap::End(op) => {
                    if self.task.op_applicable(op, &state) {
                        state = self.task.apply(op, &state);
                        // Match by op AND time: a REAL running interval may
                        // share its end op with a plan interval of the same
                        // action — the wrong removal would drop a pending
                        // end from the tail's agenda.
                        if let Some(pos) = open
                            .iter()
                            .position(|&(et, eo)| eo == op && ((et - t) / eps).abs() < 0.5)
                        {
                            open.remove(pos);
                        }
                    } else {
                        break_t = Some(t);
                    }
                }
            }
        }
        let bt = break_t.unwrap_or(horizon);

        // Tail root agenda, re-based to the break moment: open plan
        // intervals, this session's REAL running ends still in the future,
        // and scheduled events not yet fired.
        let mut carried: Vec<(f64, usize)> = open
            .iter()
            .map(|&(et, eo)| ((et - bt).max(0.0), eo))
            .collect();
        for &(t, eo) in &self.running {
            if (t / eps).round() > (bt / eps).round() {
                carried.push((t - bt, eo));
            }
        }
        for &(t, f, v) in &self.timed {
            if (t / eps).round() > (bt / eps).round() {
                carried.push((t - bt, self.til_setters[&(f, v)]));
            }
        }
        carried.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });

        let (kind, dur_exprs, inv) = crate::temporal::build_kind(&self.task, &c);
        let total = max_evaluated;
        let mut remaining = total;
        let node_bytes = memory_mb
            .map(|mb| mb.saturating_mul(1 << 20))
            .unwrap_or(crate::search::NODE_CAP_TARGET_BYTES);
        let tp = crate::temporal::solve_from_seeded(
            &self.task,
            &kind,
            &dur_exprs,
            &inv,
            &state,
            &self.task.goal_pos,
            &self.task.goal_num,
            &self.forbidden,
            &carried,
            self.threads,
            self.tier,
            &mut remaining,
            node_bytes,
            true,
        );
        let evaluated = total.saturating_sub(remaining);
        match tp {
            Some(tp) => {
                let followed = applied.iter().filter(|&&a| a).count();
                let mut out: Vec<crate::api::Step> = steps
                    .iter()
                    .enumerate()
                    .filter(|&(si, _)| applied[si])
                    .map(|(_, s)| s.clone())
                    .collect();
                let mut makespan: f64 = out
                    .iter()
                    .map(|s| s.time.unwrap_or(0.0) + s.duration.unwrap_or(0.0))
                    .fold(0.0, f64::max);
                for mut s in timed_steps(&tp) {
                    s.time = Some(s.time.unwrap_or(0.0) + bt);
                    makespan = makespan.max(s.time.unwrap_or(0.0) + s.duration.unwrap_or(0.0));
                    out.push(s);
                }
                for (i, s) in out.iter_mut().enumerate() {
                    s.index = i;
                }
                Solution {
                    solved: true,
                    mode: Mode::Temporal,
                    plan: Some(Plan {
                        length: out.len(),
                        steps: out,
                        metric: None,
                        makespan: Some(makespan),
                    }),
                    statistics: stats(&self.task, evaluated, self.threads),
                    notes: vec![format!(
                        "followed {followed} still-valid timed step(s) of the prior \
                         plan through the break at t={bt:.3}; searched only the tail"
                    )],
                }
            }
            None => {
                let mut sol = self.replan_inner(Some(max_evaluated), memory_mb);
                sol.statistics.evaluated_states += evaluated;
                sol.notes.push(
                    "timed prefix-follow found no tail within budget; fell back \
                     to an unbiased rethink"
                        .into(),
                );
                sol
            }
        }
    }

    fn replan_inner(&self, budget_evals: Option<usize>, memory_mb: Option<usize>) -> Solution {
        if let Some(c) = &self.temporal {
            return self.replan_temporal(c, budget_evals, memory_mb);
        }
        if self.task.goal_met(&self.task.initial()) {
            return Solution {
                solved: true,
                mode: Mode::Ff,
                plan: Some(Plan {
                    steps: Vec::new(),
                    length: 0,
                    metric: None,
                    makespan: None,
                }),
                statistics: stats(&self.task, 0, self.threads),
                notes: vec!["goal already satisfied; the empty plan solves it".into()],
            };
        }
        let mut cfg = crate::search::SearchCfg::from_weights(
            self.weight_g,
            self.weight_h,
            budget_evals.or(self.max_evaluated),
        );
        cfg.node_bytes_target = memory_mb.map(|mb| mb.saturating_mul(1 << 20));
        let o = search::plan_avoiding(
            &self.task,
            self.threads,
            cfg,
            self.ehc_first,
            &self.forbidden,
        );
        let mut notes = Vec::new();
        if o.ehc_fell_back && o.ops.is_some() {
            notes.push("EHC found no improving state; used weighted best-first".into());
        }
        match o.ops {
            Some(ops) => {
                let steps = steps_of(&self.task, &ops, None);
                Solution {
                    solved: true,
                    mode: Mode::Ff,
                    plan: Some(Plan {
                        length: steps.len(),
                        steps,
                        metric: None,
                        makespan: None,
                    }),
                    statistics: stats(&self.task, o.evaluated, self.threads),
                    notes,
                }
            }
            None => Solution {
                solved: false,
                mode: Mode::Ff,
                plan: None,
                statistics: stats(&self.task, o.evaluated, self.threads),
                notes,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOM: &str = "
    (define (domain farm) (:requirements :strips :typing :numeric-fluents)
      (:types agent place)
      (:predicates (at ?a - agent ?p - place) (road ?x ?y - place) (fertile ?p - place))
      (:functions (grain))
      (:action walk :parameters (?a - agent ?from ?to - place)
        :precondition (and (at ?a ?from) (road ?from ?to))
        :effect (and (not (at ?a ?from)) (at ?a ?to)))
      (:action harvest :parameters (?a - agent ?p - place)
        :precondition (and (at ?a ?p) (fertile ?p))
        :effect (increase (grain) 1)))";
    const PRB: &str = "
    (define (problem p) (:domain farm)
      (:objects v1 - agent hut field - place)
      (:init (at v1 hut) (road hut field) (road field hut) (fertile field) (= (grain) 0))
      (:goal (>= (grain) 2)))";

    #[test]
    fn observe_updates_sight_only_and_reports_surprises() {
        // The fog contract (0.15 Phase 4): sighted facts snap to the
        // observed truth and only genuine belief changes count as
        // surprises; unsighted facts keep their believed values; a bad
        // batch errors WITHOUT moving belief.
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        // Re-observing what's already believed: no surprises.
        let quiet = s
            .observe(&[("(at v1 hut)", true), ("(at v1 field)", false)])
            .expect("observe");
        assert!(quiet.is_empty(), "matching observations are not news");
        // The world moved: v1 is actually at the field now.
        let news = s
            .observe(&[("(at v1 hut)", false), ("(at v1 field)", true)])
            .expect("observe");
        assert_eq!(news.len(), 2, "both facts differed from belief");
        assert_eq!(s.fact("(at v1 field)"), Some(true));
        assert_eq!(s.fact("(at v1 hut)"), Some(false));
        // A bad batch (unknown fact) must not half-apply.
        let err = s.observe(&[("(at v1 hut)", true), ("(at v1 nowhere)", true)]);
        assert!(err.is_err(), "unknown fact errors");
        assert_eq!(
            s.fact("(at v1 hut)"),
            Some(false),
            "belief untouched by the failed batch"
        );
    }

    #[test]
    fn budgeted_think_is_bounded_and_deterministic() {
        // A tiny budget returns an honest unsolved verdict without blowing
        // the cap; identical budgets give identical solutions at any thread
        // count (the eval budget is the deterministic unit, never wall
        // clock); an adequate budget solves.
        let s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let tiny = s.replan_budgeted(1, Some(1));
        assert!(!tiny.solved, "1-eval think cannot solve the farm");
        let t1 = {
            let o = Options {
                threads: 1,
                ..Options::default()
            };
            let s = Session::new(DOM, PRB, &o).expect("session");
            s.replan_budgeted(10_000, Some(64))
        };
        let t8 = {
            let o = Options {
                threads: 8,
                ..Options::default()
            };
            let s = Session::new(DOM, PRB, &o).expect("session");
            s.replan_budgeted(10_000, Some(64))
        };
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            steps(&t1),
            steps(&t8),
            "budgeted think differs across threads"
        );
    }

    #[test]
    fn replan_solves_and_tracks_world_state() {
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let first = s.replan();
        assert!(first.solved, "base problem solves");
        // walk + harvest x2 = 3 steps
        assert_eq!(first.plan.as_ref().unwrap().steps.len(), 3);

        // The world moved: the villager is already at the field with 1 grain.
        s.set_fact("(at v1 hut)", false).unwrap();
        s.set_fact("(at v1 field)", true).unwrap();
        s.set_fluent("(grain)", 1.0).unwrap();
        assert_eq!(s.fact("(at v1 field)"), Some(true));
        assert_eq!(s.fluent("(grain)"), Some(1.0));

        let next = s.replan();
        assert!(next.solved);
        // one harvest remains
        assert_eq!(next.plan.as_ref().unwrap().steps.len(), 1);
        assert_eq!(next.plan.unwrap().steps[0].action, "HARVEST");
    }

    #[test]
    fn goal_already_met_returns_empty_plan() {
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        s.set_fluent("(grain)", 5.0).unwrap();
        let sol = s.replan();
        assert!(sol.solved);
        assert_eq!(sol.plan.unwrap().steps.len(), 0);
    }

    #[test]
    fn static_and_unknown_facts_are_rejected() {
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        // `road` is static: no operator adds or deletes it.
        assert!(s.set_fact("(road hut field)", false).is_err());
        assert!(s.set_fact("(at v1 nowhere)", true).is_err());
        assert!(s.set_fluent("(gold)", 1.0).is_err());
    }

    const TDOM: &str = "
    (define (domain workshop) (:requirements :strips :typing :durative-actions)
      (:types worker)
      (:predicates (idle ?w - worker) (built ?w - worker))
      (:durative-action build
        :parameters (?w - worker)
        :duration (= ?duration 5)
        :condition (at start (idle ?w))
        :effect (and (at start (not (idle ?w))) (at end (built ?w)))))";
    const TPRB: &str = "
    (define (problem shift) (:domain workshop)
      (:objects w1 w2 - worker)
      (:init (idle w1) (idle w2))
      (:goal (and (built w1) (built w2))))";

    #[test]
    fn follow_before_you_rethink_scripted_drift() {
        // The Phase 2 contract, scripted: irrelevant drift costs ZERO search
        // (the suffix replay says keep following); breaking drift is detected
        // exactly; think count over the script is exactly 2.
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let mut thinks = 0;

        thinks += 1;
        let think = s.replan();
        assert!(think.solved);
        let plan = think.plan.unwrap();
        assert_eq!(plan.length, 3, "walk + harvest x2");

        // Tick 1: the agent FOLLOWS step 0 (walk); the game mirrors it.
        s.set_fact("(at v1 hut)", false).unwrap();
        s.set_fact("(at v1 field)", true).unwrap();
        assert!(
            s.plan_still_valid(&plan, 1),
            "suffix after the walk must still execute"
        );

        // Tick 2: IRRELEVANT drift — a bird delivers grain. The suffix still
        // reaches the goal (grain 1 + 2 harvests >= 2): keep following, free.
        s.set_fluent("(grain)", 1.0).unwrap();
        assert!(
            s.plan_still_valid(&plan, 1),
            "helpful drift must not force a rethink"
        );

        // Tick 3: BREAKING drift — the villager is blown back home; the
        // remaining harvests are inapplicable. Exactly now we think again.
        s.set_fact("(at v1 field)", false).unwrap();
        s.set_fact("(at v1 hut)", true).unwrap();
        assert!(
            !s.plan_still_valid(&plan, 1),
            "breaking drift must be caught"
        );
        thinks += 1;
        let rethink = s.replan();
        assert!(rethink.solved);
        assert!(s.plan_still_valid(&rethink.plan.unwrap(), 0));

        assert_eq!(thinks, 2, "the whole script cost exactly two thinks");
    }

    #[test]
    fn temporal_suffix_replay_detects_breaks() {
        let mut s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        let think = s.replan_budgeted(50_000, Some(128));
        let plan = think.plan.unwrap();
        assert!(s.plan_still_valid(&plan, 0), "fresh plan replays");

        // w1's build completed out-of-band: the FULL plan (which re-starts
        // w1's build) breaks on (idle w1); the w2-only suffix still runs but
        // no longer reaches the goal alone... it DOES — (built w1) is now
        // true in the world. Both verdicts must be exact.
        s.set_fact("(built w1)", true).unwrap();
        s.set_fact("(idle w1)", false).unwrap();
        assert!(
            !s.plan_still_valid(&plan, 0),
            "re-starting w1's build must break on (idle w1)"
        );
        let w2_only = plan
            .steps
            .iter()
            .position(|st| st.args == vec!["W2".to_string()])
            .unwrap();
        assert!(
            s.plan_still_valid(&plan, w2_only.max(1)),
            "the w2-only suffix still reaches the goal"
        );
    }

    #[test]
    fn temporal_sessions_think_concurrently_and_replan() {
        // Two workers build in PARALLEL (genuine concurrency: both intervals
        // overlap); the world drifts (w1's build completes out-of-band) and
        // the rethink plans only w2's remaining work.
        let s = Session::new(TDOM, TPRB, &Options::default()).expect("temporal session");
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved, "workshop must solve");
        let plan = think.plan.as_ref().unwrap();
        assert_eq!(plan.length, 2, "one build per worker");
        assert!(plan.makespan.unwrap() < 5.1, "concurrent, not sequential");
        let (a, b) = (&plan.steps[0], &plan.steps[1]);
        assert!(a.time.is_some() && a.duration.is_some(), "timed steps");
        // interval overlap: each starts before the other ends
        let (ta, da) = (a.time.unwrap(), a.duration.unwrap());
        let (tb, db) = (b.time.unwrap(), b.duration.unwrap());
        assert!(ta < tb + db && tb < ta + da, "intervals must overlap");

        let mut s = s;
        s.set_fact("(built w1)", true).unwrap();
        s.set_fact("(idle w1)", false).unwrap();
        let rethink = s.replan_budgeted(50_000, Some(128));
        assert!(rethink.solved);
        assert_eq!(
            rethink.plan.as_ref().unwrap().length,
            1,
            "only w2's build remains"
        );
    }

    #[test]
    fn temporal_think_budget_is_bounded_and_deterministic() {
        let s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        let tiny = s.replan_budgeted(1, Some(1));
        assert!(!tiny.solved, "a 1-eval temporal think cannot solve");
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let s = Session::new(TDOM, TPRB, &o).expect("session");
            s.replan_budgeted(50_000, Some(128))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let key = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| {
                    (
                        st.action.clone(),
                        st.args.clone(),
                        (st.time.unwrap() * 1000.0).round() as i64,
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(key(&t1), key(&t8), "temporal think differs across threads");
    }

    #[test]
    fn set_goal_retargets_without_regrounding() {
        // One world, changing desires: the same session serves a numeric
        // goal, then a positional one, then an empty one — no regrounding,
        // and plan_still_valid always answers against the CURRENT goal.
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let grain_plan = s.replan();
        assert!(grain_plan.solved);
        let grain_plan = grain_plan.plan.unwrap();
        assert_eq!(grain_plan.length, 3, "walk + harvest x2");

        // Retarget: forget grain, just be at the field.
        s.set_goal("(at v1 field)").unwrap();
        let at_field = s.replan();
        assert!(at_field.solved);
        assert_eq!(at_field.plan.as_ref().unwrap().length, 1, "one walk");
        // The OLD plan still reaches the new goal (its walk passes through) —
        // replay must answer against the CURRENT goal, not the birth goal.
        assert!(s.plan_still_valid(&grain_plan, 0));
        // A goal the old plan does NOT reach: back at the hut.
        s.set_goal("(and (at v1 hut) (>= (grain) 1))").unwrap();
        assert!(
            !s.plan_still_valid(&grain_plan, 0),
            "the old plan ends at the field with grain 2 but never returns home"
        );
        let home = s.replan();
        assert!(home.solved);
        // walk + harvest + walk back (order may vary): 3 steps min? walk,
        // harvest, walk = 3.
        assert_eq!(home.plan.as_ref().unwrap().length, 3);

        // Empty conjunction: the always-met goal.
        s.set_goal("(and)").unwrap();
        let idle = s.replan();
        assert!(idle.solved);
        assert_eq!(idle.plan.unwrap().length, 0);
    }

    #[test]
    fn set_goal_rejects_unknown_and_adl() {
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let before = s.replan();
        // Unknown atom: never in the grounded world.
        let err = s.set_goal("(at v1 nowhere)").unwrap_err();
        assert!(err.contains("not in the grounded fact space"), "{err}");
        // Unknown fluent.
        let err = s.set_goal("(>= (gold) 1)").unwrap_err();
        assert!(err.contains("unknown fluent"), "{err}");
        // ADL connective.
        let err = s.set_goal("(or (at v1 hut) (at v1 field))").unwrap_err();
        assert!(err.contains("ADL"), "{err}");
        // Non-ground.
        let err = s.set_goal("(at ?a field)").unwrap_err();
        assert!(err.contains("ground"), "{err}");
        // Negation without a grounded mirror (farm has no negative pre/goals).
        let err = s.set_goal("(not (at v1 hut))").unwrap_err();
        assert!(err.contains("no grounded mirror"), "{err}");
        // Every rejection left the ORIGINAL goal untouched.
        let after = s.replan();
        assert_eq!(
            before.plan.unwrap().length,
            after.plan.unwrap().length,
            "a failed set_goal must not corrupt the current goal"
        );
    }

    const NEG_DOM: &str = "
    (define (domain lamp) (:requirements :strips :negative-preconditions)
      (:predicates (on) (broken))
      (:action switch-on :precondition (and (not (on)) (not (broken))) :effect (on))
      (:action switch-off :precondition (on) :effect (not (on))))";
    const NEG_PRB: &str = "
    (define (problem p) (:domain lamp)
      (:init) (:goal (on)))";

    #[test]
    fn set_goal_negative_literals_via_mirrors_and_set_fact_sync() {
        // `(not (on))` occurs negatively in a precondition, so grounding
        // created the mirror fact — a negated goal literal is expressible,
        // and set_fact keeps base and mirror in sync (a stale mirror would
        // corrupt applicability, not just goals).
        let mut s = Session::new(NEG_DOM, NEG_PRB, &Options::default()).expect("session");
        let on = s.replan();
        assert!(on.solved);
        assert_eq!(on.plan.unwrap().length, 1, "switch-on");

        // The lamp got switched on out-of-band; retarget to (not (on)).
        s.set_fact("(on)", true).unwrap();
        s.set_goal("(not (on))").unwrap();
        let off = s.replan();
        assert!(off.solved);
        assert_eq!(off.plan.unwrap().length, 1, "switch-off");

        // Mirror sync end-to-end: turn it back off out-of-band; switch-on
        // (which REQUIRES the mirror `(NOT (ON))` true) must be applicable
        // again — it would not be if set_fact left the mirror stale.
        s.set_fact("(on)", false).unwrap();
        s.set_goal("(on)").unwrap();
        let relit = s.replan();
        assert!(
            relit.solved,
            "stale mirror would make switch-on inapplicable"
        );
        assert_eq!(relit.plan.unwrap().length, 1);
    }

    #[test]
    fn set_goal_numeric_relevance_grows_the_state_key() {
        // A domain with a write-only accumulator: irrelevant at construction
        // (it is in no precondition/goal), so it is OUT of the visited key.
        // Retargeting the goal onto it must pull it INTO the key — otherwise
        // states differing only in the accumulator dedup and search is wrong.
        let dom = "
        (define (domain walkers) (:requirements :strips :typing :numeric-fluents)
          (:types agent place)
          (:predicates (at ?a - agent ?p - place) (road ?x ?y - place))
          (:functions (steps))
          (:action walk :parameters (?a - agent ?from ?to - place)
            :precondition (and (at ?a ?from) (road ?from ?to))
            :effect (and (not (at ?a ?from)) (at ?a ?to) (increase (steps) 1))))";
        let prb = "
        (define (problem p) (:domain walkers)
          (:objects v1 - agent a b - place)
          (:init (at v1 a) (road a b) (road b a) (= (steps) 0))
          (:goal (at v1 b)))";
        let mut s = Session::new(dom, prb, &Options::default()).expect("session");
        assert!(s.replan().solved);
        // Retarget onto the accumulator: pace until three steps are walked.
        s.set_goal("(>= (steps) 3)").unwrap();
        let paced = s.replan();
        assert!(
            paced.solved,
            "goal over a formerly-irrelevant fluent must solve (key must grow)"
        );
        assert_eq!(paced.plan.unwrap().length, 3, "a-b, b-a, a-b");
    }

    #[test]
    fn set_goal_temporal_retarget() {
        // Temporal sessions retarget too: build only w2 instead of both.
        let mut s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        let both = s.replan_budgeted(50_000, Some(128));
        assert!(both.solved);
        assert_eq!(both.plan.as_ref().unwrap().length, 2);

        s.set_goal("(built w2)").unwrap();
        let solo = s.replan_budgeted(50_000, Some(128));
        assert!(solo.solved);
        let plan = solo.plan.as_ref().unwrap();
        assert_eq!(plan.length, 1, "only w2's build serves the new goal");
        assert_eq!(plan.steps[0].args, vec!["W2".to_string()]);
        // The two-build plan still meets (built w2) — replay agrees.
        assert!(s.plan_still_valid(both.plan.as_ref().unwrap(), 0));
        // RUNNING tokens are fenced in goals exactly as in set_fact.
        let err = s.set_goal("(RUNNING-BUILD w1)").unwrap_err();
        assert!(err.contains("running-interval"), "{err}");
    }

    #[test]
    fn set_goal_retarget_is_deterministic_across_threads() {
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let mut s = Session::new(DOM, PRB, &o).expect("session");
            s.set_goal("(and (at v1 hut) (>= (grain) 2))").unwrap();
            s.replan_budgeted(10_000, Some(64))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            steps(&t1),
            steps(&t8),
            "retargeted think differs across threads"
        );
    }

    #[test]
    fn temporal_sessions_fence_tils_and_running_tokens() {
        let til_prb = "(define (problem p) (:domain workshop)
          (:objects w1 - worker)
          (:init (idle w1) (at 3 (idle w1)))
          (:goal (built w1)))";
        let err = match Session::new(TDOM, til_prb, &Options::default()) {
            Err(e) => e,
            Ok(_) => panic!("TIL problem must be rejected"),
        };
        assert!(
            err.contains("timed initial"),
            "TILs must be rejected: {err}"
        );

        let mut s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        let err = s.set_fact("(RUNNING-BUILD w1)", true).unwrap_err();
        assert!(
            err.contains("running-interval"),
            "RUNNING-* must be fenced: {err}"
        );
    }

    #[test]
    fn fork_shares_the_world_and_inherits_current_state() {
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        // Move the world BEFORE forking: the fork starts from here, not :init.
        s.set_fact("(at v1 hut)", false).unwrap();
        s.set_fact("(at v1 field)", true).unwrap();
        s.set_fluent("(grain)", 1.0).unwrap();
        let f = s.fork();
        assert_eq!(f.fact("(at v1 field)"), Some(true));
        assert_eq!(f.fluent("(grain)"), Some(1.0));
        // The grounded payload is SHARED, not copied — the whole point.
        assert!(Arc::ptr_eq(&s.task.fact_names, &f.task.fact_names));
        assert!(Arc::ptr_eq(&s.task.op_display, &f.task.op_display));
        assert!(Arc::ptr_eq(&s.task.add.flat, &f.task.add.flat));
        assert!(Arc::ptr_eq(&s.fact_ids, &f.fact_ids));
        let sol = f.replan();
        assert!(sol.solved);
        assert_eq!(sol.plan.unwrap().length, 1, "one harvest from here");
    }

    #[test]
    fn forks_diverge_without_touching_siblings() {
        let s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let mut hungry = s.fork();
        let mut idle = s.fork();
        // One mind's world-writes and retarget...
        hungry.set_fluent("(grain)", 1.0).unwrap();
        hungry.set_goal("(>= (grain) 5)").unwrap();
        // ...are invisible to its sibling and its parent.
        assert_eq!(idle.fluent("(grain)"), Some(0.0));
        assert_eq!(s.fluent("(grain)"), Some(0.0));
        let h = hungry.replan();
        assert!(h.solved);
        assert_eq!(h.plan.unwrap().length, 5, "walk + 4 more harvests");
        let i = idle.replan();
        assert!(i.solved);
        assert_eq!(
            i.plan.unwrap().length,
            3,
            "sibling still walks + harvests 2"
        );
        assert_eq!(s.replan().plan.unwrap().length, 3, "parent unmoved");
        // A fork's relevance growth (set_goal onto a new fluent elsewhere)
        // stays its own: idle's key/goal are untouched by hungry's retarget.
        idle.set_fact("(at v1 hut)", false).unwrap();
        idle.set_fact("(at v1 field)", true).unwrap();
        assert_eq!(hungry.fact("(at v1 field)"), Some(false));
    }

    #[test]
    fn fork_temporal_population_thinks_independently() {
        let s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        let mut a = s.fork();
        let mut b = s.fork();
        a.set_goal("(built w1)").unwrap();
        b.set_goal("(built w2)").unwrap();
        let pa = a.replan_budgeted(50_000, Some(128));
        let pb = b.replan_budgeted(50_000, Some(128));
        assert!(pa.solved && pb.solved);
        assert_eq!(
            pa.plan.as_ref().unwrap().steps[0].args,
            vec!["W1".to_string()]
        );
        assert_eq!(
            pb.plan.as_ref().unwrap().steps[0].args,
            vec!["W2".to_string()]
        );
        // Parent still wants both and still gets both.
        assert_eq!(s.replan_budgeted(50_000, Some(128)).plan.unwrap().length, 2);
    }

    #[test]
    fn replan_following_preserves_the_surviving_prefix() {
        // Drift that invalidates the plan WITHOUT breaking any step's
        // applicability (the goal just isn't met at the end anymore): the
        // whole suffix replays as the prefix, search adds only the tail.
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let prior = s.replan();
        assert!(prior.solved);
        let prior = prior.plan.unwrap(); // walk, harvest, harvest
        s.set_fluent("(grain)", -1.0).unwrap();
        assert!(!s.plan_still_valid(&prior, 0), "goal shortfall breaks it");
        let sol = s.replan_following(&prior, 0, 10_000, Some(64));
        assert!(sol.solved);
        let steps = &sol.plan.as_ref().unwrap().steps;
        assert_eq!(steps.len(), 4, "old 3 steps + one more harvest");
        for (i, old) in prior.steps.iter().enumerate() {
            assert_eq!(steps[i].action, old.action, "prefix must be verbatim");
            assert_eq!(steps[i].args, old.args);
        }
        assert!(
            sol.notes.iter().any(|n| n.contains("followed 3")),
            "{:?}",
            sol.notes
        );
    }

    #[test]
    fn replan_following_cuts_at_goal_mid_prefix() {
        // The prior plan OVERSHOOTS after drift (grain already 1): the goal
        // is met two steps in, the plan is cut there — pure replay, zero
        // search spent.
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        let prior = s.replan().plan.unwrap();
        s.set_fluent("(grain)", 1.0).unwrap();
        let sol = s.replan_following(&prior, 0, 10_000, Some(64));
        assert!(sol.solved);
        assert_eq!(sol.plan.as_ref().unwrap().length, 2, "walk + one harvest");
        assert_eq!(sol.statistics.evaluated_states, 0, "replay, not search");
    }

    #[test]
    fn replan_following_falls_back_when_the_prefix_strands() {
        // A prior "plan" whose first step is a WRONG PICK (trades the seed
        // for junk nobody wants): the prefix replays into a dead end, the
        // seeded search finds no tail, and the fallback unbiased rethink
        // still solves — the bias may cost budget, never completeness.
        let dom = include_str!("../../../benchmarks/bench/bazaar-chain-domain.pddl");
        let prb = include_str!("../../../benchmarks/bench/bazaar-chain.pddl");
        let mut s = Session::new(dom, prb, &Options::default()).expect("session");
        s.set_goal("(has a0 item1)").unwrap();
        let bad = Plan {
            steps: vec![crate::api::Step {
                index: 0,
                action: "TRADE".into(),
                args: ["A0", "V1", "ITEM0", "JUNK0"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                time: None,
                duration: None,
            }],
            length: 1,
            metric: None,
            makespan: None,
        };
        let sol = s.replan_following(&bad, 0, 50_000, Some(128));
        assert!(sol.solved, "fallback must recover: {:?}", sol.notes);
        assert_eq!(
            sol.plan.as_ref().unwrap().length,
            1,
            "trade item0 for item1"
        );
        assert!(
            sol.notes.iter().any(|n| n.contains("fell back")),
            "{:?}",
            sol.notes
        );
    }

    #[test]
    fn restrict_ops_scopes_a_mind_to_its_own_actions() {
        // In the wants-gated bazaar the solver freely plans vendor-vendor
        // pre-trades (rival moves). An actor-scoped mind must plan around
        // them: every step's actor is the mind itself, even when that means
        // a longer chain — and a rival step in a plan fails validation.
        let dom = include_str!("../../../benchmarks/bench/bazaar-chain-domain.pddl");
        let prb = include_str!("../../../benchmarks/bench/bazaar-chain.pddl");
        let mut s = Session::new(dom, prb, &Options::default()).expect("session");
        s.set_goal("(has a0 item4)").unwrap();
        let free = s.replan_budgeted(50_000, Some(128));
        assert!(free.solved);
        s.restrict_ops(|d| d.starts_with("TRADE A0 "));
        let scoped = s.replan_budgeted(50_000, Some(128));
        assert!(scoped.solved, "the own-actor chain exists");
        for st in &scoped.plan.as_ref().unwrap().steps {
            assert_eq!(st.args[0], "A0", "actor-scoped plan used a rival move");
        }
        // A plan with a rival step is invalid FOR THIS MIND even though the
        // world could execute it.
        let rival = Plan {
            steps: vec![crate::api::Step {
                index: 0,
                action: "TRADE".into(),
                args: ["V2", "V3", "ITEM2", "ITEM3"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                time: None,
                duration: None,
            }],
            length: 1,
            metric: None,
            makespan: None,
        };
        assert!(!s.plan_still_valid(&rival, 0));
        // Fork inherits the restriction; clearing it restores rival moves.
        let f = s.fork();
        assert!(f
            .replan_budgeted(50_000, Some(128))
            .plan
            .unwrap()
            .steps
            .iter()
            .all(|st| st.args[0] == "A0"));
        s.restrict_ops(|_| true);
        assert!(s.forbidden.is_empty(), "keep-everything clears the mask");
    }

    #[test]
    fn goal_met_is_a_pure_state_test() {
        let mut s = Session::new(DOM, PRB, &Options::default()).expect("session");
        assert!(!s.goal_met(), "grain starts at 0, goal needs 2");
        // A think can still SOLVE from here — that is a different question.
        assert!(s.replan().solved);
        assert!(!s.goal_met(), "a solvable goal is not a met goal");
        s.set_fluent("(grain)", 2.0).unwrap();
        assert!(s.goal_met());
        s.set_goal("(>= (grain) 5)").unwrap();
        assert!(!s.goal_met(), "goal_met answers for the CURRENT goal");
    }

    #[test]
    fn restricted_thinks_stay_deterministic_across_threads() {
        let dom = include_str!("../../../benchmarks/bench/bazaar-chain-domain.pddl");
        let prb = include_str!("../../../benchmarks/bench/bazaar-chain.pddl");
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let mut s = Session::new(dom, prb, &o).expect("session");
            s.set_goal("(has a0 item6)").unwrap();
            s.restrict_ops(|d| d.starts_with("TRADE A0 "));
            s.replan_budgeted(50_000, Some(128))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            steps(&t1),
            steps(&t8),
            "restricted think differs across threads"
        );
    }

    // `power` is exogenous-CHANGEABLE: the `grid` action touches it, making
    // it dynamic — the scheduling contract. A truly STATIC fact is compiled
    // INTO the ops (stripped from runtime preconditions), so flipping one by
    // event could not soundly affect behavior; `set_timed_fact` refuses.
    const SEQ_DOM: &str = "
    (define (domain seqshop) (:requirements :strips :typing :durative-actions)
      (:types w)
      (:predicates (idle ?x - w) (staged ?x - w) (built ?x - w) (power))
      (:durative-action stage1 :parameters (?x - w) :duration (= ?duration 5)
        :condition (at start (idle ?x))
        :effect (and (at start (not (idle ?x))) (at end (staged ?x))))
      (:durative-action stage2 :parameters (?x - w) :duration (= ?duration 5)
        :condition (and (at start (staged ?x)) (at start (power)))
        :effect (at end (built ?x)))
      (:durative-action grid :parameters () :duration (= ?duration 1)
        :condition (at start (power))
        :effect (and (at start (not (power))) (at end (power)))))";
    const SEQ_PRB: &str = "
    (define (problem p) (:domain seqshop)
      (:objects w1 - w)
      (:init (idle w1) (power))
      (:goal (built w1)))";

    #[test]
    fn timed_events_close_windows_plans_beat_or_fail_honestly() {
        // stage2 needs (staged w1) — earliest t=5 via stage1 — AND (power).
        // Power dying at t=20 leaves a window: the plan must fit inside it.
        let mut s = Session::new(SEQ_DOM, SEQ_PRB, &Options::default()).expect("session");
        s.set_timed_fact(20.0, "(power)", false).unwrap();
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved, "the window is beatable");
        let plan = think.plan.unwrap();
        let s2 = plan.steps.iter().find(|st| st.action == "STAGE2").unwrap();
        assert!(
            s2.time.unwrap() < 20.0,
            "stage2 must start inside the window"
        );
        assert!(s.plan_still_valid(&plan, 0), "the winning plan replays");
        // A LATE hand-built plan walks into the dead window: the replay
        // experiences the scheduled event and rejects it.
        let late = Plan {
            steps: vec![
                crate::api::Step {
                    index: 0,
                    action: "STAGE1".into(),
                    args: vec!["W1".into()],
                    time: Some(0.0),
                    duration: Some(5.0),
                },
                crate::api::Step {
                    index: 1,
                    action: "STAGE2".into(),
                    args: vec!["W1".into()],
                    time: Some(25.0),
                    duration: Some(5.0),
                },
            ],
            length: 2,
            metric: None,
            makespan: Some(30.0),
        };
        assert!(
            !s.plan_still_valid(&late, 0),
            "the event kills the late plan"
        );
        // Power dying at t=3 closes the window before staged can exist.
        s.set_timed_fact(3.0, "(power)", false).unwrap();
        assert!(
            !s.replan_budgeted(50_000, Some(128)).solved,
            "unbeatable window is an honest unsolved"
        );
    }

    #[test]
    fn timed_events_after_the_plan_are_the_games_future() {
        let mut s = Session::new(SEQ_DOM, SEQ_PRB, &Options::default()).expect("session");
        s.set_timed_fact(1000.0, "(power)", false).unwrap();
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved);
        assert!(s.plan_still_valid(&think.plan.unwrap(), 0));
    }

    #[test]
    fn elapse_decays_and_fires_events_in_order() {
        let mut s = Session::new(SEQ_DOM, SEQ_PRB, &Options::default()).expect("session");
        s.set_timed_fact(3.0, "(power)", false).unwrap();
        s.set_timed_fact(6.0, "(idle w1)", false).unwrap();
        s.elapse(2.0).unwrap();
        assert_eq!(s.fact("(power)"), Some(true), "not yet due");
        s.elapse(1.5).unwrap();
        assert_eq!(s.fact("(power)"), Some(false), "fired");
        assert_eq!(s.fact("(idle w1)"), Some(true), "still pending");
        s.elapse(10.0).unwrap();
        assert_eq!(s.fact("(idle w1)"), Some(false));
    }

    #[test]
    fn timed_event_fences_hold() {
        // Classical sessions have no clock.
        let mut c = Session::new(DOM, PRB, &Options::default()).expect("session");
        let err = c.set_timed_fact(5.0, "(at v1 field)", true).unwrap_err();
        assert!(err.contains("TEMPORAL"), "{err}");
        // Temporal: same writability fences as set_fact, plus dt sanity.
        let mut s = Session::new(SEQ_DOM, SEQ_PRB, &Options::default()).expect("session");
        assert!(s.set_timed_fact(0.0, "(power)", false).is_err());
        assert!(s.set_timed_fact(-1.0, "(power)", false).is_err());
        assert!(s.set_timed_fact(5.0, "(no-such)", false).is_err());
        // The recorded limit: a goal whose enabler exists ONLY via events
        // never even grounds (the fact space cannot express it) — an honest
        // construction error, not a silent unsolvable.
        let unpowered = "
        (define (problem p) (:domain seqshop)
          (:objects w1 - w)
          (:init (idle w1))
          (:goal (built w1)))";
        assert!(
            Session::new(SEQ_DOM, unpowered, &Options::default()).is_err(),
            "event-only enablers cannot ground — the recorded 0.14 limit"
        );
    }

    #[test]
    fn thinks_wait_through_enabling_events() {
        // Power dies at t=1 and RETURNS at t=10: stage2's window is
        // [10, ...] — the agenda carries both events, so the search WAITS
        // through the outage and starts stage2 after the enabler fires.
        let mut s = Session::new(SEQ_DOM, SEQ_PRB, &Options::default()).expect("session");
        s.set_timed_fact(1.0, "(power)", false).unwrap();
        s.set_timed_fact(10.0, "(power)", true).unwrap();
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved, "waiting through the outage must work");
        let plan = think.plan.as_ref().unwrap();
        let s2 = plan.steps.iter().find(|st| st.action == "STAGE2").unwrap();
        assert!(
            s2.time.unwrap() >= 10.0,
            "stage2 must wait for power's return, got t={}",
            s2.time.unwrap()
        );
        assert!(s.plan_still_valid(plan, 0));
    }

    #[test]
    fn timed_thinks_stay_deterministic_across_threads() {
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let mut s = Session::new(SEQ_DOM, SEQ_PRB, &o).expect("session");
            s.set_timed_fact(20.0, "(power)", false).unwrap();
            s.replan_budgeted(50_000, Some(128))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone(), st.time))
                .collect::<Vec<_>>()
        };
        assert_eq!(steps(&t1), steps(&t8), "timed think differs across threads");
    }

    #[test]
    fn apply_start_lets_a_mind_think_mid_interval() {
        // The world starts w1's build; the think happens WHILE it runs: the
        // plan covers only w2 (w1's interval is already in flight) and is
        // valid THROUGH w1's pending end.
        let mut s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        s.apply_start("(build w1)").unwrap();
        assert_eq!(s.fact("(idle w1)"), Some(false), "start effects applied");
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved);
        let plan = think.plan.as_ref().unwrap();
        assert!(
            plan.steps
                .iter()
                .all(|st| st.args != vec!["W1".to_string()]),
            "w1 is already building — the plan must not restart it: {:?}",
            plan.steps
        );
        assert!(s.plan_still_valid(plan, 0));
        // Double-start is not applicable (idle w1 is gone).
        assert!(s.apply_start("(build w1)").is_err());
        // Classical sessions have no durations.
        let mut c = Session::new(DOM, PRB, &Options::default()).expect("session");
        assert!(c.apply_start("(walk v1 hut field)").is_err());
    }

    #[test]
    fn elapse_fires_interval_ends_retiring_the_mirror_idiom() {
        let mut s = Session::new(TDOM, TPRB, &Options::default()).expect("session");
        s.apply_start("(build w1)").unwrap();
        let broken = s.elapse(2.0).unwrap();
        assert!(broken.is_empty());
        assert_eq!(s.fact("(built w1)"), Some(false), "3 units still to go");
        let broken = s.elapse(3.0).unwrap();
        assert!(broken.is_empty());
        assert_eq!(
            s.fact("(built w1)"),
            Some(true),
            "the end fired its own effects — no manual mirroring"
        );
    }

    #[test]
    fn a_think_can_be_just_waiting_for_a_running_end() {
        // `grid` drops (power) at start and restores it at end (duration 1).
        // With the goal (power) and grid IN FLIGHT, the honest plan is to
        // wait: zero steps, makespan = the pending end's moment.
        let mut s = Session::new(SEQ_DOM, SEQ_PRB, &Options::default()).expect("session");
        s.set_goal("(power)").unwrap();
        s.apply_start("(grid)").unwrap();
        assert_eq!(s.fact("(power)"), Some(false), "mid-cycle: power is down");
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved, "waiting for the running end must solve");
        let plan = think.plan.unwrap();
        assert_eq!(plan.length, 0, "no new starts — the wait IS the plan");
        assert!(
            (plan.makespan.unwrap() - 1.0).abs() < 0.01,
            "makespan is the pending end's moment, got {:?}",
            plan.makespan
        );
    }

    #[test]
    fn drift_that_breaks_a_running_interval_is_honest() {
        // `hold` requires (power) OVER ALL. Kill power mid-flight: the plan
        // fails validation, and the elapse that reaches the end reports the
        // interval broken instead of applying its effects.
        let oa_dom = "
        (define (domain oa) (:requirements :strips :typing :durative-actions)
          (:types w)
          (:predicates (idle ?x - w) (staged ?x - w) (power))
          (:durative-action hold :parameters (?x - w) :duration (= ?duration 5)
            :condition (and (at start (idle ?x)) (over all (power)))
            :effect (and (at start (not (idle ?x))) (at end (staged ?x))))
          (:durative-action grid :parameters () :duration (= ?duration 1)
            :condition (at start (power))
            :effect (and (at start (not (power))) (at end (power)))))";
        let oa_prb = "
        (define (problem p) (:domain oa)
          (:objects w1 - w)
          (:init (idle w1) (power))
          (:goal (staged w1)))";
        let mut s = Session::new(oa_dom, oa_prb, &Options::default()).expect("session");
        s.apply_start("(hold w1)").unwrap();
        let think = s.replan_budgeted(50_000, Some(128));
        assert!(think.solved);
        let plan = think.plan.unwrap();
        assert!(s.plan_still_valid(&plan, 0));
        s.set_fact("(power)", false).unwrap();
        assert!(
            !s.plan_still_valid(&plan, 0),
            "a broken running interval breaks every plan living through it"
        );
        let broken = s.elapse(5.0).unwrap();
        assert_eq!(broken.len(), 1, "the failed end is reported: {broken:?}");
        assert!(broken[0].contains("HOLD"), "{broken:?}");
        assert_eq!(
            s.fact("(staged w1)"),
            Some(false),
            "a broken interval's end effects are dropped"
        );
    }

    #[test]
    fn in_flight_thinks_stay_deterministic_across_threads() {
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let mut s = Session::new(TDOM, TPRB, &o).expect("session");
            s.apply_start("(build w1)").unwrap();
            s.replan_budgeted(50_000, Some(128))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone(), st.time))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            steps(&t1),
            steps(&t8),
            "in-flight think differs across threads"
        );
    }

    // Two machines, two jobs: FAST does a job in 2, SLOW in 8. Drift can
    // retire a machine between thinks — the reroute fixture for temporal
    // follow-biased rethinks.
    const SHOP_DOM: &str = "
    (define (domain shop) (:requirements :strips :typing :durative-actions)
      (:types job machine)
      (:predicates (todo ?j - job) (done ?j - job) (up ?m - machine) (fast ?m - machine)
                   (slow ?m - machine))
      (:durative-action run-fast :parameters (?j - job ?m - machine)
        :duration (= ?duration 2)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (fast ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action run-slow :parameters (?j - job ?m - machine)
        :duration (= ?duration 8)
        :condition (and (at start (todo ?j)) (at start (up ?m)) (at start (slow ?m))
                        (over all (up ?m)))
        :effect (and (at start (not (todo ?j))) (at end (done ?j))))
      (:durative-action maintain :parameters (?m - machine)
        :duration (= ?duration 1)
        :condition (at start (up ?m))
        :effect (and (at start (not (up ?m))) (at end (up ?m)))))";
    const SHOP_PRB: &str = "
    (define (problem p) (:domain shop)
      (:objects j1 j2 - job f s - machine)
      (:init (todo j1) (todo j2) (up f) (up s) (fast f) (slow s))
      (:goal (and (done j1) (done j2))))";

    #[test]
    fn temporal_following_keeps_the_prefix_and_reroutes_the_tail() {
        let mut sess = Session::new(SHOP_DOM, SHOP_PRB, &Options::default()).expect("session");
        let prior = sess.replan_budgeted(50_000, Some(128));
        assert!(prior.solved);
        let prior = prior.plan.unwrap();
        // Both jobs go to the FAST machine (2 < 8): j1 at t=0, j2 at t~2.
        assert!(prior.steps.iter().all(|s| s.action == "RUN-FAST"));
        // Drift: the fast machine dies before j2's run starts.
        sess.set_fact("(up f)", false).unwrap();
        assert!(!sess.plan_still_valid(&prior, 0));
        let followed = sess.replan_following(&prior, 0, 50_000, Some(128));
        assert!(followed.solved, "{:?}", followed.notes);
        let plan = followed.plan.as_ref().unwrap();
        // j1's fast run breaks too (over-all up f) — nothing survives the
        // replay, so the whole plan reroutes to the slow machine.
        assert!(
            plan.steps.iter().all(|s| s.action == "RUN-SLOW"),
            "{:?}",
            plan.steps
        );
        assert!(sess.plan_still_valid(plan, 0));
    }

    #[test]
    fn temporal_following_carries_the_in_flight_interval() {
        // j1's SLOW run is already IN FLIGHT (real interval) when drift
        // breaks the plan's fast-machine tail: the followed rethink must
        // keep the running interval (not restart j1) and reroute only j2.
        let mut sess = Session::new(SHOP_DOM, SHOP_PRB, &Options::default()).expect("session");
        sess.apply_start("(run-slow j1 s)").unwrap();
        let prior = sess.replan_budgeted(50_000, Some(128));
        assert!(prior.solved);
        let prior = prior.plan.unwrap();
        assert_eq!(prior.length, 1, "only j2 needs planning: {:?}", prior.steps);
        assert_eq!(prior.steps[0].action, "RUN-FAST");
        sess.set_fact("(up f)", false).unwrap();
        assert!(!sess.plan_still_valid(&prior, 0));
        let followed = sess.replan_following(&prior, 0, 50_000, Some(128));
        assert!(followed.solved, "{:?}", followed.notes);
        let plan = followed.plan.as_ref().unwrap();
        assert_eq!(
            plan.steps
                .iter()
                .filter(|s| s.args.first().map(String::as_str) == Some("J1"))
                .count(),
            0,
            "the in-flight j1 interval must not be restarted: {:?}",
            plan.steps
        );
        assert!(
            plan.steps
                .iter()
                .any(|s| s.action == "RUN-SLOW"
                    && s.args.first().map(String::as_str) == Some("J2")),
            "{:?}",
            plan.steps
        );
        assert!(sess.plan_still_valid(plan, 0));
    }

    #[test]
    fn temporal_following_is_deterministic_across_threads() {
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let mut sess = Session::new(SHOP_DOM, SHOP_PRB, &o).expect("session");
            let prior = sess.replan_budgeted(50_000, Some(128)).plan.unwrap();
            sess.set_fact("(up f)", false).unwrap();
            sess.replan_following(&prior, 0, 50_000, Some(128))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone(), st.time))
                .collect::<Vec<_>>()
        };
        assert_eq!(steps(&t1), steps(&t8));
    }

    #[test]
    fn fork_keeps_thread_determinism() {
        let run = |threads: usize| {
            let o = Options {
                threads,
                ..Options::default()
            };
            let s = Session::new(DOM, PRB, &o).expect("session");
            let mut f = s.fork();
            f.set_fluent("(grain)", 1.0).unwrap();
            f.set_goal("(>= (grain) 4)").unwrap();
            f.replan_budgeted(10_000, Some(64))
        };
        let (t1, t8) = (run(1), run(8));
        assert!(t1.solved && t8.solved);
        let steps = |sol: &Solution| {
            sol.plan
                .as_ref()
                .unwrap()
                .steps
                .iter()
                .map(|st| (st.action.clone(), st.args.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            steps(&t1),
            steps(&t8),
            "forked think differs across threads"
        );
    }
}
