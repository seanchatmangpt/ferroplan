//! Ground IR -> a flat FOND planning-problem IR: a breadth-first enumeration
//! of the reachable *composite* (ground-fact-set, task-network-frontier)
//! state space. Only ground actions reachable via an actual decomposition of
//! the problem's root `:htn` task network are ever offered as executable
//! moves — this is what makes the search HTN-decomposition-aware rather than
//! a flat "try every precondition-satisfying ground action" BFS: a ground
//! action whose precondition happens to hold but which no decomposition of
//! the remaining task network could ever reach is never turned into a
//! transition, even if the underlying `evaluate_ground_goal` check would
//! otherwise pass.
//!
//! The types here (`State`/`Goal`/`Transition`/`Task`/`Method`/
//! `PlanningProblem`) deliberately mirror `ferroplan::planning_runtime`'s
//! shapes field-for-field rather than importing them — see the crate-level
//! dependency-direction note in `Cargo.toml`/`lib.rs`. The adapter that maps
//! one onto the other lives in the `ferroplan` crate. Task-network
//! decomposition progress (`Frontier`/`CompositeState`, below) is internal
//! bookkeeping to this module only: it never leaks into `PlanningProblem`'s
//! shape. Method-choice/execution-order OR-branchpoints are instead encoded
//! as ordinary `Transition` records (`"htn:decompose:<addr>:<method>"` /
//! `"htn:exec:<addr>:<action>"`), and task-network completion is encoded as
//! a synthetic `"htn:done"` fact folded into `State.facts` and required by
//! `Goal.facts` — both facts `fond_policy`'s existing, unmodified
//! all-outcomes-winning fixpoint already knows how to consume.

use crate::ast::{GoalDesc, Term};
use crate::grounder::{
    atom_key, evaluate_ground_goal, GroundEffectBranch, GroundMethod, GroundSubtask, GroundedIR,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
#[cfg(test)]
use std::ops::Not;

const PROBABILITY_SCALE: u32 = 1_000_000;

/// An error produced by `translate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateError {
    /// Formerly returned for `(not <non-atomic goal description>)` (e.g.
    /// `(not (and ...))`) in `:goal`. That restriction is lifted: `to_dnf`
    /// now pushes negation down to individual literals via De Morgan's laws,
    /// so a negated compound goal description compiles into a multi-clause
    /// DNF, each clause consumed via the `goal_reached_marker` mechanism
    /// (see `to_dnf`'s doc comment). This variant is kept only so
    /// `TranslateError`'s public API and exhaustive `Display` match stay
    /// stable across the change — it is never constructed by current code.
    UnsupportedNegativeGoal,
    /// `:goal` used a quantifier (`forall`/`exists`) that reached `to_dnf`
    /// un-expanded — in practice unreachable, since `GroundedIR::goal` is
    /// only ever populated after `grounder::expand_goal_quantifiers` has
    /// already expanded every quantifier away; kept as a defensive refusal
    /// for a `GroundedIR` constructed directly (as several tests in this
    /// module do) with an un-expanded quantifier, rather than silently
    /// mis-evaluating one. Formerly also returned for a plain `or`/`imply`
    /// at the top of `:goal` — that restriction is lifted: `to_dnf` now
    /// fully expands `or`/`imply`/negated-compounds into a DNF `Vec<Clause>`
    /// consumed via `goal_reached_marker`, the same connectives action
    /// *preconditions* already supported via `grounder::GroundGoal`/
    /// `evaluate_ground_goal`.
    UnsupportedGoalConnective(String),
    UnboundVariable(String),
    /// A pending task-network address exceeded
    /// `TranslateLimits::max_task_network_depth` while decomposing — refused
    /// loudly rather than silently truncating the search (truncation could
    /// report `NoPlan` for a domain that has a real, only slightly-deeper
    /// solution). Mirrors `GroundError::LimitExceeded`'s "loud refusal, not
    /// silent truncation" discipline.
    TaskNetworkDepthExceeded {
        addr: String,
        limit: usize,
    },
    /// `TranslateLimits::max_wall` elapsed before the BFS finished. Loud
    /// wall-clock refusal, same discipline as `TaskNetworkDepthExceeded` and
    /// `grounder::GroundError::Timeout` — protects against a domain whose
    /// reachable composite-state space is enormous in *breadth* (many
    /// distinct fact combinations at a shallow, within-limit task-network
    /// depth) rather than in *depth*, which `max_task_network_depth` alone
    /// does not bound.
    Timeout {
        elapsed_ms: u128,
        limit_ms: u128,
    },
    /// `TranslateLimits::max_states` was reached before the BFS converged.
    /// Independent of, and checked at the same point as, `Timeout` (once per
    /// state popped off the queue in `translate`'s BFS) — a real, hardware-
    /// independent memory ceiling rather than a wall-clock proxy for one. A
    /// machine faster than whatever `max_wall` was tuned against can still
    /// intern an unbounded number of `State`/`Transition` structs (each
    /// cloning a `BTreeSet<String>` of ground-fact strings) before a
    /// wall-clock-only deadline fires; this variant refuses loudly at a
    /// fixed state count regardless of clock speed, the same discipline
    /// `grounder::GroundingLimits::max_ground_actions`/`max_ground_methods`
    /// already apply to grounding.
    MemoryLimitExceeded {
        states: usize,
        limit: usize,
    },
}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedNegativeGoal => {
                write!(f, "'not' of a non-atomic goal description is out of scope")
            }
            Self::UnsupportedGoalConnective(head) => write!(
                f,
                "'{head}' in ':goal' is out of scope (supported only in action preconditions)"
            ),
            Self::UnboundVariable(v) => write!(f, "unbound variable '?{v}' in goal"),
            Self::TaskNetworkDepthExceeded { addr, limit } => write!(
                f,
                "task-network address '{addr}' exceeded max_task_network_depth ({limit})"
            ),
            Self::Timeout {
                elapsed_ms,
                limit_ms,
            } => write!(
                f,
                "translate wall-clock limit exceeded: {elapsed_ms}ms elapsed, limit {limit_ms}ms"
            ),
            Self::MemoryLimitExceeded { states, limit } => write!(
                f,
                "translate memory limit exceeded: {states} states interned, limit {limit}"
            ),
        }
    }
}
impl std::error::Error for TranslateError {}

/// Bounds decomposition depth so a buggy/genuinely-non-terminating recursive
/// method set (e.g. a ground method whose subtask re-invokes the identical
/// ground task name) fails loudly instead of running the BFS forever. Mirrors
/// `GroundingLimits`'s existing `max_ground_actions`/`max_ground_methods`
/// style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateLimits {
    pub max_task_network_depth: usize,
    /// Wall-clock budget for the whole BFS in `translate`, checked once per
    /// state popped off the queue (see `check_wall_deadline`). `None` means
    /// unbounded; `default()` sets a real bound (mirrors
    /// `grounder::GroundingLimits::max_wall`) so a caller using
    /// `TranslateLimits::default()` is protected without opting in.
    pub max_wall: Option<std::time::Duration>,
    /// Hard, count-based ceiling on the number of composite states interned
    /// by `translate`'s BFS, checked at the same point as `max_wall` (once
    /// per state popped off the queue) — independent of the wall clock.
    /// Each interned `State` (plus its outgoing `Transition`s) clones a
    /// `BTreeSet<String>` of ground-fact strings, so uncapped state growth
    /// is uncapped memory growth; this bounds it directly by count rather
    /// than relying on a wall-clock timeout as a memory proxy, mirroring
    /// `grounder::GroundingLimits::max_ground_actions`/`max_ground_methods`.
    /// `None` means unbounded; `default()` sets a real bound.
    pub max_states: Option<usize>,
}

impl Default for TranslateLimits {
    fn default() -> Self {
        Self {
            max_task_network_depth: 64,
            max_wall: Some(std::time::Duration::from_secs(10)),
            max_states: Some(200_000),
        }
    }
}

/// Checked once per state popped off `translate`'s BFS queue. See
/// `grounder::check_wall_deadline`, which this mirrors (that function is
/// private to the `grounder` module, so this crate carries its own copy
/// rather than exporting one — cheap enough, and each module's `Limits`
/// type/`Error::Timeout` variant differ, so there's no shared trait to hang
/// a single implementation off without adding one just for this).
fn check_wall_deadline(
    start: std::time::Instant,
    limits: &TranslateLimits,
) -> Result<(), TranslateError> {
    if let Some(max_wall) = limits.max_wall {
        let elapsed = start.elapsed();
        if elapsed > max_wall {
            return Err(TranslateError::Timeout {
                elapsed_ms: elapsed.as_millis(),
                limit_ms: max_wall.as_millis(),
            });
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    pub id: String,
    pub facts: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Goal {
    pub facts: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub action: String,
    pub from: String,
    pub to: String,
    pub probability_ppm: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Task {
    pub id: String,
    pub primitive_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Method {
    pub id: String,
    pub task: String,
    pub subtasks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlanningProblem {
    pub states: Vec<State>,
    pub initial_states: Vec<String>,
    pub goal: Goal,
    pub transitions: Vec<Transition>,
    pub tasks: Vec<Task>,
    pub root_tasks: Vec<String>,
    pub methods: Vec<Method>,
}

fn ground_atom_key(a: &crate::ast::AtomicFormula) -> Result<String, TranslateError> {
    let args = a
        .args
        .iter()
        .map(|t| match t {
            Term::Const(c) => Ok(c.clone()),
            Term::Var(v) => Err(TranslateError::UnboundVariable(v.clone())),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(atom_key(&a.predicate, &args))
}

/// The synthetic fact key standing in for "the ground atom `fact` is
/// currently absent". `fond_policy`/`Goal::holds` in `ferroplan`'s
/// `planning_runtime` only support goal membership as a positive-fact
/// subset test (`self.facts.is_subset(&state.facts)`) — there is no
/// negative-fact field to extend, and extending that shared `Goal` type
/// would change behavior for every planning family that consumes it
/// (Classical, CostOptimal, Fond, ...), not just HDDL. So a `(not P)` goal
/// literal is compiled entirely within this crate instead: once the real
/// BFS over composite states below has produced its final, exact per-state
/// fact sets (used unmodified for actions' real precondition/effect
/// semantics), a `not:`-prefixed marker fact is inserted into exactly those
/// states where `fact` is genuinely absent — reachability-filtering computed
/// from the real per-state result, not a new `Goal` shape. The `:` cannot
/// appear in a real `atom_key` (predicate/object names are plain HDDL
/// identifiers), so no collision with a real fact is possible.
fn negation_marker(fact: &str) -> String {
    format!("not:{fact}")
}

/// One conjunctive clause of a `:goal` description's disjunctive-normal-form
/// expansion: `.0` is the set of ground atoms that must hold, `.1` is the set
/// of ground atoms that must NOT hold. A goal holds overall iff at least one
/// clause in the DNF vector holds.
type Clause = (BTreeSet<String>, BTreeSet<String>);

/// Expands `goal` into disjunctive normal form: a `Vec<Clause>` such that the
/// original goal holds in a fact set iff at least one clause's `pos` is a
/// subset of the facts and none of its `neg` atoms are present. Negation is
/// pushed down to individual literals via De Morgan's laws (`negate` tracks
/// whether the current subtree is under an odd number of enclosing `Not`s),
/// so `(not (and A B))` becomes the two-clause DNF `(not A) OR (not B)` and
/// `(not (or A B))` becomes the single-clause DNF `(not A) AND (not B)` —
/// this is what lets `:goal` support `or`/`imply` and negation of compound
/// goal descriptions, not just a single ground literal, reusing exactly the
/// same synthetic-marker-fact mechanism (`negation_marker`,
/// `goal_reached_marker`) that already handles a bare `(not P)` literal.
///
/// `And` combines its parts' DNFs via cartesian product (conjunction
/// distributes over each part's disjuncts); `Or` combines them by
/// concatenation. A domain whose `:goal` is deeply nested `and`/`or` can
/// therefore produce a DNF exponential in the nesting depth — acceptable
/// here since a `:goal` description is authored by hand and stays small in
/// every fixture and real domain this crate has seen; `translate`'s existing
/// `TranslateLimits` (wall-clock/state-count) still bound the BFS this
/// feeds, independent of DNF size.
fn to_dnf(goal: &GoalDesc, negate: bool) -> Result<Vec<Clause>, TranslateError> {
    match goal {
        GoalDesc::Empty => Ok(vec![(BTreeSet::new(), BTreeSet::new())]),
        GoalDesc::Atom(a) => {
            let key = ground_atom_key(a)?;
            let mut pos = BTreeSet::new();
            let mut neg = BTreeSet::new();
            if negate {
                neg.insert(key);
            } else {
                pos.insert(key);
            }
            Ok(vec![(pos, neg)])
        }
        GoalDesc::Not(inner) => to_dnf(inner, !negate),
        GoalDesc::And(parts) => {
            if negate {
                // De Morgan: not(A and B) == (not A) or (not B) -- OR, so
                // concatenate each part's (negated) DNF.
                let mut out = Vec::new();
                for p in parts {
                    out.extend(to_dnf(p, true)?);
                }
                Ok(out)
            } else {
                cartesian_and(parts, negate)
            }
        }
        GoalDesc::Or(parts) => {
            if negate {
                // De Morgan: not(A or B) == (not A) and (not B) -- AND, so
                // cartesian-combine each part's (negated) DNF.
                cartesian_and(parts, negate)
            } else {
                let mut out = Vec::new();
                for p in parts {
                    out.extend(to_dnf(p, false)?);
                }
                Ok(out)
            }
        }
        GoalDesc::Imply(antecedent, consequent) => {
            // (imply A B) == (or (not A) B); negated == (and A (not B)).
            let desugared = GoalDesc::Or(vec![
                GoalDesc::Not(antecedent.clone()),
                consequent.as_ref().clone(),
            ]);
            to_dnf(&desugared, negate)
        }
        // `:goal` quantifiers are expanded away by the grounder
        // (`grounder::expand_goal_quantifiers`) before `GroundedIR::goal` is
        // ever populated, so a `Forall`/`Exists` node can never reach this
        // function in practice. Refused rather than silently ignored, on
        // the off chance `GroundedIR` is constructed directly (as several
        // tests in this module do) with an un-expanded quantifier.
        GoalDesc::Forall(_, _) | GoalDesc::Exists(_, _) => Err(
            TranslateError::UnsupportedGoalConnective("forall/exists".to_owned()),
        ),
    }
}

/// Conjunction of `parts`' DNFs (each recursively expanded under `negate`)
/// via cartesian product: every combination of one clause from each part's
/// DNF, with `pos`/`neg` sets unioned. Used both for a plain (non-negated)
/// `And` and for a negated `Or` (De Morgan reduces both to the same
/// "AND across parts" shape).
fn cartesian_and(parts: &[GoalDesc], negate: bool) -> Result<Vec<Clause>, TranslateError> {
    let mut acc: Vec<Clause> = vec![(BTreeSet::new(), BTreeSet::new())];
    for p in parts {
        let part_clauses = to_dnf(p, negate)?;
        let mut next = Vec::with_capacity(acc.len() * part_clauses.len().max(1));
        for (apos, aneg) in &acc {
            for (ppos, pneg) in &part_clauses {
                let mut pos = apos.clone();
                pos.extend(ppos.iter().cloned());
                let mut neg = aneg.clone();
                neg.extend(pneg.iter().cloned());
                next.push((pos, neg));
            }
        }
        acc = next;
    }
    Ok(acc)
}

/// The synthetic fact key marking "this state satisfies at least one
/// disjunctive-goal clause" — inserted post-BFS (see `translate`) into every
/// reachable state whose real, final fact set matches one of the goal's DNF
/// clauses. Mirrors `negation_marker`'s "no real `atom_key` can collide"
/// argument: `:` never appears in a real ground-fact string.
fn goal_reached_marker() -> &'static str {
    "goal:reached"
}

/// This action's real outcome probabilities, in ppm (parts-per-million out
/// of `PROBABILITY_SCALE`), one per `outcomes` entry in order, always
/// summing to exactly `PROBABILITY_SCALE`.
///
/// When every outcome carries a `probability_weight` (see
/// `grounder::GroundEffectBranch` — set from a `(:probabilistic ...)` effect
/// block, see `probabilistic::preprocess`) that parses to a nonnegative
/// `f64` with a positive total, the split is proportional to those real
/// weights via the largest-remainder method (assign each outcome
/// `floor(weight_i / total * PROBABILITY_SCALE)`, then hand out the leftover
/// ppm one at a time to the outcomes with the largest fractional remainder
/// so the exact-sum invariant holds even though flooring alone would
/// undercount). Otherwise (a plain `oneof`, or a mix of weighted/unweighted
/// outcomes, or unparseable/all-zero weights) every outcome gets an equal
/// share — the original, weight-oblivious behavior, preserved byte-for-byte
/// for every domain that doesn't use `:probabilistic`.
fn outcome_ppms(outcomes: &[GroundEffectBranch]) -> Vec<u32> {
    let n = outcomes.len();
    if n == 0 {
        return Vec::new();
    }

    let declared_weights: Option<Vec<f64>> = outcomes
        .iter()
        .map(|o| {
            o.probability_weight
                .as_deref()
                .and_then(|w| w.parse::<f64>().ok())
                .filter(|w| w.is_finite() && *w >= 0.0)
        })
        .collect();

    if let Some(weights) = declared_weights {
        let total: f64 = weights.iter().sum();
        if total > 0.0 {
            let scale = f64::from(PROBABILITY_SCALE);
            let raw: Vec<f64> = weights.iter().map(|w| w / total * scale).collect();
            let mut ppms: Vec<u32> = raw.iter().map(|r| r.floor() as u32).collect();
            let assigned: u32 = ppms.iter().sum();
            let mut leftover = PROBABILITY_SCALE - assigned;
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_by(|&a, &b| {
                let fa = raw[a] - raw[a].floor();
                let fb = raw[b] - raw[b].floor();
                fb.partial_cmp(&fa).unwrap_or(std::cmp::Ordering::Equal)
            });
            for &i in &order {
                if leftover == 0 {
                    break;
                }
                ppms[i] += 1;
                leftover -= 1;
            }
            return ppms;
        }
    }

    let branch_count = n as u32;
    let base = PROBABILITY_SCALE / branch_count;
    let remainder = PROBABILITY_SCALE % branch_count;
    (0..n)
        .map(|i| base + u32::from((i as u32) < remainder))
        .collect()
}

fn intern_state(
    facts: &BTreeSet<String>,
    state_ids: &mut BTreeMap<BTreeSet<String>, String>,
    states: &mut Vec<State>,
) -> String {
    if let Some(id) = state_ids.get(facts) {
        return id.clone();
    }
    let id = format!("s{}", state_ids.len());
    state_ids.insert(facts.clone(), id.clone());
    states.push(State {
        id: id.clone(),
        facts: facts.clone(),
    });
    id
}

// --- HTN-decomposition-aware search state -----------------------------

/// A structural position in the (partially decomposed) task-network tree.
/// Root subtasks address as `"r.<htn-subtask-id>"`; a subtask spawned by
/// decomposing node `addr` via a method addresses as `"<addr>.<subtask-id>"`.
/// Addresses only ever grow in length through decomposition, so no address
/// can equal an ancestor's or a sibling subtree's address — collision-free by
/// construction, independent of BFS visitation order.
type Addr = String;

fn child_addr(parent: &str, subtask_id: &str) -> Addr {
    format!("{parent}.{subtask_id}")
}

/// Number of decomposition steps `addr` is away from the root — root
/// subtasks (`"r.<id>"`) are depth 1, their children depth 2, and so on.
fn depth_of(addr: &str) -> usize {
    addr.matches('.').count()
}

/// The still-to-execute part of the task network at one composite search
/// state: which task-network nodes remain, and the (restricted) partial
/// order still constraining them. `Ord`-derivable so it can be used as a
/// map/set key directly.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Frontier {
    /// addr -> ground task name (matches either a `GroundAction::name` or a
    /// `GroundMethod::task_name`).
    pending: BTreeMap<Addr, String>,
    /// (before_addr, after_addr) pairs, restricted at all times to pairs
    /// where BOTH ends are keys of `pending` (a dangling end is always
    /// dropped immediately when that end is consumed — see `refine`/
    /// `advance`).
    order: BTreeSet<(Addr, Addr)>,
}

/// One BFS work-item: raw ground facts + task-network progress. This pair,
/// not `facts` alone, is the real search-state identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CompositeState {
    facts: BTreeSet<String>,
    frontier: Frontier,
}

/// Task-network nodes with no unexecuted predecessor (unordered subtasks —
/// i.e. no `<` edge touching them — are trivially enabled, matching HDDL's
/// default no-order semantics).
fn enabled(frontier: &Frontier) -> BTreeSet<&str> {
    let has_incoming: BTreeSet<&str> = frontier.order.iter().map(|(_, a)| a.as_str()).collect();
    frontier
        .pending
        .keys()
        .map(String::as_str)
        .filter(|a| !has_incoming.contains(a))
        .collect()
}

/// The standard HTN task-network-refinement rule: decompose `addr` (whose
/// task name matched `m.task_name`) into `m`'s subtasks. An external
/// predecessor of the decomposed node becomes a predecessor of every
/// *first* child (no incoming internal-order edge); an external successor
/// becomes a successor of every *last* child (no outgoing internal-order
/// edge); if the method's subtask list is empty, predecessor and successor
/// bridge directly to each other (the node vanishes with nothing to inherit
/// the constraint).
fn refine(frontier: &Frontier, addr: &str, m: &GroundMethod) -> Frontier {
    let children: Vec<Addr> = m
        .subtasks
        .iter()
        .map(|st| child_addr(addr, &st.id))
        .collect();
    let internal: BTreeSet<(Addr, Addr)> = m
        .order
        .iter()
        .map(|(b, a)| (child_addr(addr, b), child_addr(addr, a)))
        .collect();
    let has_incoming: BTreeSet<&str> = internal.iter().map(|(_, a)| a.as_str()).collect();
    let has_outgoing: BTreeSet<&str> = internal.iter().map(|(b, _)| b.as_str()).collect();
    let firsts: Vec<&Addr> = children
        .iter()
        .filter(|c| !has_incoming.contains(c.as_str()))
        .collect();
    let lasts: Vec<&Addr> = children
        .iter()
        .filter(|c| !has_outgoing.contains(c.as_str()))
        .collect();

    let preds: Vec<&Addr> = frontier
        .order
        .iter()
        .filter(|(_, a)| a == addr)
        .map(|(b, _)| b)
        .collect();
    let succs: Vec<&Addr> = frontier
        .order
        .iter()
        .filter(|(b, _)| b == addr)
        .map(|(_, a)| a)
        .collect();

    let mut new_order: BTreeSet<(Addr, Addr)> = frontier
        .order
        .iter()
        .filter(|(b, a)| b != addr && a != addr)
        .cloned()
        .collect();
    new_order.extend(internal);
    for p in &preds {
        for f in &firsts {
            new_order.insert(((*p).clone(), (*f).clone()));
        }
    }
    for s in &succs {
        for l in &lasts {
            new_order.insert(((*l).clone(), (*s).clone()));
        }
    }
    if children.is_empty() {
        for p in &preds {
            for s in &succs {
                new_order.insert(((*p).clone(), (*s).clone()));
            }
        }
    }

    let mut new_pending = frontier.pending.clone();
    new_pending.remove(addr);
    for st in &m.subtasks {
        new_pending.insert(child_addr(addr, &st.id), st.task_name.clone());
    }
    Frontier {
        pending: new_pending,
        order: new_order,
    }
}

/// Execution advancement: executing a primitive task discharges it
/// outright. Unlike decomposition, nothing needs to "inherit" the
/// constraint — the task is genuinely done, so its order edges are simply
/// dropped rather than bridged.
fn advance(frontier: &Frontier, addr: &str) -> Frontier {
    let mut pending = frontier.pending.clone();
    pending.remove(addr);
    let order = frontier
        .order
        .iter()
        .filter(|(b, a)| b != addr && a != addr)
        .cloned()
        .collect();
    Frontier { pending, order }
}

/// Length-prefixes `s` onto `out` (a "netstring"-style encoding): unlike a
/// naive delimiter-joined format, this is unconditionally injective
/// regardless of `s`'s content — a real `atom_key` output can itself
/// contain `,`/`(`/`)` (e.g. `"drive(l1,l2)"`), so a naive delimiter join
/// would risk two distinct frontiers serializing identically.
fn encode_field(s: &str, out: &mut String) {
    out.push_str(&s.len().to_string());
    out.push(':');
    out.push_str(s);
}

/// An injective serialization of `f`, suitable for use as a synthetic marker
/// fact folded into a composite state's fact set (see `translate`). `:`
/// never appears in a real `atom_key` output (this crate already relies on
/// that fact for `not:`-prefixed goal markers — see `negation_marker`) — so
/// this whole string can never collide with a real ground fact.
fn frontier_marker(f: &Frontier) -> String {
    let mut out = String::from("htn-frontier:");
    for (addr, name) in &f.pending {
        // BTreeMap: iterated in sorted key order, so this is deterministic.
        encode_field(addr, &mut out);
        encode_field(name, &mut out);
    }
    out.push(';');
    for (before, after) in &f.order {
        // BTreeSet: iterated in sorted order, so this is deterministic.
        encode_field(before, &mut out);
        encode_field(after, &mut out);
    }
    out
}

fn htn_done_marker() -> &'static str {
    "htn:done"
}

/// The augmented fact set actually stored on `State`/used for BFS-visited
/// dedup: the composite state's real facts, plus a marker fact encoding its
/// task-network frontier, plus (when the frontier is empty) the `htn:done`
/// marker `Goal` requires. Because `frontier_marker` is injective in
/// `(pending, order)` and `htn:done`'s presence is a deterministic function
/// of `pending.is_empty()`, this whole set is injective in `(facts,
/// frontier)` — so deduping on it (via `intern_state`'s underlying map) is
/// exactly deduping on the real composite-state identity.
fn augmented_facts(cs: &CompositeState) -> BTreeSet<String> {
    let mut facts = cs.facts.clone();
    facts.insert(frontier_marker(&cs.frontier));
    if cs.frontier.pending.is_empty() {
        facts.insert(htn_done_marker().to_owned());
    }
    facts
}

/// Translate a `GroundedIR` into a flat FOND `PlanningProblem`: a real
/// breadth-first enumeration of the reachable composite (ground-fact-set,
/// task-network-frontier) state space, restricted to states reachable via an
/// actual decomposition of the root `:htn` network (see the module docs for
/// why this is HTN-decomposition-aware rather than a flat "try every
/// applicable ground action" BFS).
///
/// Each non-deterministic action's `oneof` outcomes are split into
/// probability-weighted transitions (`Transition::probability_ppm`, parts per
/// million so they sum to `1_000_000` per source action) — proportionally to
/// `GroundEffectBranch::probability_weight` when declared, evenly otherwise.
/// Method-choice and execution-order branch points are encoded as ordinary
/// `Transition`s (`"htn:decompose:..."` / `"htn:exec:..."`); task-network
/// completion is folded into `State.facts` as a synthetic `"htn:done"` fact
/// and required by `Goal.facts` — this is what lets an existing, unmodified
/// all-outcomes-winning FOND policy solver consume the result directly.
///
/// # Errors
///
/// `:goal` fully supports `and`/`or`/`imply`/`not` (including negation of a
/// compound description, expanded via De Morgan's laws) via `to_dnf`'s
/// disjunctive-normal-form expansion — see that function's doc comment.
/// Returns `TranslateError::UnsupportedGoalConnective` only for a stray,
/// un-expanded `forall`/`exists` reaching `to_dnf` (in practice unreachable
/// via the normal parse -> ground -> translate pipeline, since
/// `grounder::expand_goal_quantifiers` already expands every quantifier away
/// before `GroundedIR::goal` is populated), `TranslateError::UnboundVariable`
/// for a variable used without a binding, and
/// `TranslateError::TaskNetworkDepthExceeded` if decomposition exceeds
/// `limits.max_task_network_depth`.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::grounder::{ground, GroundingLimits};
/// use ferroplan_hddl::parser::{parse_domain, parse_problem};
/// use ferroplan_hddl::translate::{translate, TranslateLimits};
///
/// let domain = parse_domain(r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#).unwrap();
/// let problem = parse_problem(r#"
///     (define (problem doors-p1)
///       (:domain doors)
///       (:objects d1)
///       (:init)
///       (:goal (open d1))
///       (:htn :ordered-subtasks (open-door d1)))
/// "#).unwrap();
///
/// let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
/// let problem = translate(&ir, &TranslateLimits::default())
///     .expect("grounded IR translates cleanly");
/// assert!(!problem.states.is_empty());
/// assert!(!problem.transitions.is_empty());
/// ```
pub fn translate(
    ir: &GroundedIR,
    limits: &TranslateLimits,
) -> Result<PlanningProblem, TranslateError> {
    let start = std::time::Instant::now();
    let goal_clauses = to_dnf(&ir.goal, false)?;

    let actions_by_name: BTreeMap<&str, Vec<&crate::grounder::GroundAction>> =
        ir.actions.iter().fold(BTreeMap::new(), |mut m, a| {
            m.entry(a.name.as_str()).or_default().push(a);
            m
        });
    let methods_by_task: BTreeMap<&str, Vec<&GroundMethod>> =
        ir.methods.iter().fold(BTreeMap::new(), |mut m, gm| {
            m.entry(gm.task_name.as_str()).or_default().push(gm);
            m
        });

    let initial_frontier = {
        let pending: BTreeMap<Addr, String> = ir
            .root_subtasks
            .iter()
            .map(|st: &GroundSubtask| (child_addr("r", &st.id), st.task_name.clone()))
            .collect();
        let order: BTreeSet<(Addr, Addr)> = ir
            .root_order
            .iter()
            .map(|(b, a)| (child_addr("r", b), child_addr("r", a)))
            .collect();
        Frontier { pending, order }
    };

    let mut state_ids: BTreeMap<BTreeSet<String>, String> = BTreeMap::new();
    let mut states: Vec<State> = Vec::new();
    let mut transitions: Vec<Transition> = Vec::new();

    let initial_cs = CompositeState {
        facts: ir.initial_facts.clone(),
        frontier: initial_frontier,
    };
    let initial_aug = augmented_facts(&initial_cs);
    let initial_id = intern_state(&initial_aug, &mut state_ids, &mut states);
    let mut visited: BTreeSet<BTreeSet<String>> = BTreeSet::from([initial_aug]);
    let mut queue: VecDeque<(CompositeState, String)> =
        VecDeque::from([(initial_cs, initial_id.clone())]);

    while let Some((cs, from_id)) = queue.pop_front() {
        if let Err(e) = check_wall_deadline(start, limits) {
            // Diagnostic-only: a wall-clock timeout otherwise discards all
            // BFS progress with no visibility into how far it actually got.
            // Real counts here (not a description of them) are what let a
            // caller distinguish "close, just needs a bigger budget" from
            // "still exploding combinatorially" post-fix.
            eprintln!(
                "translate: wall-clock limit hit -- {} states interned, {} transitions built, {} states still queued",
                states.len(),
                transitions.len(),
                queue.len() + 1,
            );
            return Err(e);
        }
        if let Some(limit) = limits.max_states {
            if states.len() >= limit {
                eprintln!(
                    "translate: memory limit hit -- {} states interned, {} transitions built, {} states still queued",
                    states.len(),
                    transitions.len(),
                    queue.len() + 1,
                );
                return Err(TranslateError::MemoryLimitExceeded {
                    states: states.len(),
                    limit,
                });
            }
        }
        let addrs: Vec<String> = enabled(&cs.frontier)
            .into_iter()
            .map(str::to_owned)
            .collect();
        for addr in addrs {
            let task_name = cs.frontier.pending[&addr].clone();

            // (a) decomposition moves: one deterministic transition per
            // applicable ground method (an OR-branchpoint over method
            // choice, exactly like `fond_policy` already treats distinct
            // ground-action names as alternatives from the same state).
            if let Some(methods) = methods_by_task.get(task_name.as_str()) {
                for m in methods {
                    // Method-precondition gating: a method whose ground
                    // `:precondition` doesn't hold in the current fact set is
                    // never offered as a decomposition branch — mirrors the
                    // `evaluate_ground_goal` check the execution-move loop
                    // below already applies to `GroundAction::precondition`.
                    // Without this, every method matching `task_name` was
                    // offered regardless of world state, which is exactly the
                    // unconstrained method-choice branching that made the
                    // real IPC2020 blocksworld fixture (fixtures/f) blow up
                    // combinatorially instead of solving.
                    if !evaluate_ground_goal(&m.precondition, &cs.facts) {
                        continue;
                    }
                    for st in &m.subtasks {
                        let child = child_addr(&addr, &st.id);
                        let depth = depth_of(&child);
                        if depth > limits.max_task_network_depth {
                            return Err(TranslateError::TaskNetworkDepthExceeded {
                                addr: child,
                                limit: limits.max_task_network_depth,
                            });
                        }
                    }
                    let new_frontier = refine(&cs.frontier, &addr, m);
                    let new_cs = CompositeState {
                        facts: cs.facts.clone(),
                        frontier: new_frontier,
                    };
                    let aug = augmented_facts(&new_cs);
                    let to_id = intern_state(&aug, &mut state_ids, &mut states);
                    transitions.push(Transition {
                        action: format!("htn:decompose:{addr}:{}", m.name),
                        from: from_id.clone(),
                        to: to_id.clone(),
                        probability_ppm: PROBABILITY_SCALE,
                    });
                    if visited.insert(aug) {
                        queue.push_back((new_cs, to_id));
                    }
                }
            }

            // (b) execution moves: every applicable ground action matching
            // this pending primitive task name, each outcome becoming a
            // real transition with real probability mass.
            if let Some(actions) = actions_by_name.get(task_name.as_str()) {
                for action in actions {
                    if !evaluate_ground_goal(&action.precondition, &cs.facts) {
                        continue;
                    }
                    if action.outcomes.is_empty() {
                        continue;
                    }
                    let ppms = outcome_ppms(&action.outcomes);
                    // Which task executes is never itself uncertain — only
                    // its *effect* is — so every oneof/weighted outcome of
                    // this one execution shares the same frontier
                    // component, computed once, and differs only in facts.
                    let next_frontier = advance(&cs.frontier, &addr);
                    for (i, branch) in action.outcomes.iter().enumerate() {
                        let mut next_facts = cs.facts.clone();
                        for d in &branch.del {
                            next_facts.remove(d);
                        }
                        for a in &branch.add {
                            next_facts.insert(a.clone());
                        }
                        for cond in &branch.conditional {
                            let holds = cond.pos_cond.is_subset(&cs.facts)
                                && cond.neg_cond.iter().all(|f| !cs.facts.contains(f));
                            if holds {
                                for d in &cond.del {
                                    next_facts.remove(d);
                                }
                                for a in &cond.add {
                                    next_facts.insert(a.clone());
                                }
                            }
                        }
                        let new_cs = CompositeState {
                            facts: next_facts,
                            frontier: next_frontier.clone(),
                        };
                        let aug = augmented_facts(&new_cs);
                        let to_id = intern_state(&aug, &mut state_ids, &mut states);
                        transitions.push(Transition {
                            action: format!("htn:exec:{addr}:{}", action.name),
                            from: from_id.clone(),
                            to: to_id.clone(),
                            probability_ppm: ppms[i],
                        });
                        if visited.insert(aug) {
                            queue.push_back((new_cs, to_id));
                        }
                    }
                }
            }
            // A task_name matching NEITHER a method NOR an action (e.g. an
            // object-typing mismatch left it with no ground method, or a
            // ground action whose precondition never holds along this
            // path) yields zero successors here -- a designed dead-end, not
            // an error: `fond_policy`'s win-propagation naturally never
            // marks a state with no outgoing edges as winning.
        }
    }

    // Reachability-filtering for the goal's DNF clauses: the real BFS above
    // is already finished (every state's `facts` is its exact, final ground
    // fact set, computed purely from real action effects — untouched by
    // what follows).
    //
    // The single-clause case (the overwhelming majority of real `:goal`s: a
    // plain conjunction, optionally with negative literals, and no `or`/
    // `imply`/negated-compound) is compiled exactly as before: each `(not
    // P)` literal gets the synthetic `negation_marker(P)` fact inserted into
    // exactly the states where `P` is genuinely absent, and `Goal.facts` is
    // the clause's positive atoms plus those markers — byte-identical output
    // to the pre-DNF implementation, so every existing single-clause
    // caller/test keeps its exact current fact-string shape.
    //
    // A real multi-clause DNF (an actual `or`/`imply` disjunction, or a
    // negated compound that expanded to more than one clause) has no single
    // fixed set of "the" positive/negative literals to publish — different
    // reachable goal states can satisfy entirely different clauses. So
    // instead of picking one clause arbitrarily, every reachable state is
    // checked against every clause post-BFS and the single synthetic
    // `goal_reached_marker()` fact is inserted into exactly the states that
    // satisfy at least one — `Goal.facts` then just requires that one
    // marker (plus `htn:done`), letting the existing, unmodified
    // `Goal::holds` positive-subset test consume a real disjunction without
    // needing its own OR-aware evaluation logic.
    let goal_facts = if let [(pos, neg)] = goal_clauses.as_slice() {
        if !neg.is_empty() {
            for state in &mut states {
                for neg_fact in neg {
                    if !state.facts.contains(neg_fact) {
                        state.facts.insert(negation_marker(neg_fact));
                    }
                }
            }
        }
        let mut goal_facts = pos.clone();
        for neg_fact in neg {
            goal_facts.insert(negation_marker(neg_fact));
        }
        goal_facts
    } else {
        for state in &mut states {
            let satisfied = goal_clauses.iter().any(|(pos, neg)| {
                pos.is_subset(&state.facts) && neg.iter().all(|f| !state.facts.contains(f))
            });
            if satisfied {
                state.facts.insert(goal_reached_marker().to_owned());
            }
        }
        BTreeSet::from([goal_reached_marker().to_owned()])
    };
    let mut goal_facts = goal_facts;
    // A real HTN solution requires the *entire* task network reduced to
    // executed primitives, with `:goal` (if present) as an *additional*
    // requirement on the final facts — not just `:goal` holding on a
    // half-decomposed/executed state. `htn:done` is inserted into a
    // composite state's augmented facts at construction time (see
    // `augmented_facts`) the moment `frontier.pending` is empty, so this is
    // a no-op for a domain with an empty/absent `:htn` (every reachable
    // state already carries it from the start), preserving today's
    // behavior exactly for that case.
    goal_facts.insert(htn_done_marker().to_owned());
    let goal = Goal { facts: goal_facts };

    let mut tasks = Vec::new();
    let mut seen_tasks = BTreeSet::new();
    for action in &ir.actions {
        if seen_tasks.insert(action.name.clone()) {
            tasks.push(Task {
                id: action.name.clone(),
                primitive_action: Some(action.name.clone()),
            });
        }
    }
    for m in &ir.methods {
        if seen_tasks.insert(m.task_name.clone()) {
            tasks.push(Task {
                id: m.task_name.clone(),
                primitive_action: None,
            });
        }
    }
    let methods = ir
        .methods
        .iter()
        .map(|m| Method {
            id: m.name.clone(),
            task: m.task_name.clone(),
            subtasks: m.subtasks.iter().map(|s| s.task_name.clone()).collect(),
        })
        .collect();
    let mut root_tasks = Vec::new();
    let mut seen_root_tasks = BTreeSet::new();
    for st in &ir.root_subtasks {
        if seen_root_tasks.insert(st.task_name.clone()) {
            root_tasks.push(st.task_name.clone());
        }
    }

    Ok(PlanningProblem {
        states,
        initial_states: vec![initial_id],
        goal,
        transitions,
        tasks,
        root_tasks,
        methods,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grounder::{ground, GroundingLimits};
    use crate::parser::{parse_domain, parse_problem};

    const FIXTURE_A_DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../fixtures/a/problem.hddl");
    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../fixtures/c/problem.hddl");
    const FIXTURE_E_DOMAIN: &str = include_str!("../fixtures/e/domain.hddl");
    const FIXTURE_E_PROBLEM: &str = include_str!("../fixtures/e/problem.hddl");

    /// Deterministic (backdated `start`, not a real slow operation) check
    /// that `check_wall_deadline` fires once elapsed exceeds `max_wall`.
    /// Mirrors `grounder::tests::check_wall_deadline_times_out_once_elapsed_exceeds_max_wall`.
    #[test]
    fn check_wall_deadline_times_out_once_elapsed_exceeds_max_wall() {
        let limits = TranslateLimits {
            max_wall: Some(std::time::Duration::from_millis(10)),
            ..TranslateLimits::default()
        };
        let backdated_start = std::time::Instant::now() - std::time::Duration::from_millis(50);
        let err = check_wall_deadline(backdated_start, &limits).unwrap_err();
        match err {
            TranslateError::Timeout {
                elapsed_ms,
                limit_ms,
            } => {
                assert!(
                    elapsed_ms >= 50,
                    "expected >=50ms elapsed, got {elapsed_ms}"
                );
                assert_eq!(limit_ms, 10);
            }
            other => panic!("expected Timeout, got {other:?}"),
        }
    }

    #[test]
    fn check_wall_deadline_never_fires_when_max_wall_is_none() {
        let limits = TranslateLimits {
            max_wall: None,
            ..TranslateLimits::default()
        };
        let ancient_start = std::time::Instant::now() - std::time::Duration::from_secs(3600);
        assert!(check_wall_deadline(ancient_start, &limits).is_ok());
    }

    /// End-to-end confirmation that `translate()` itself surfaces the
    /// timeout: fixture A's BFS pops at least the initial state off the
    /// queue before finding no more work, and `check_wall_deadline` is
    /// called on every pop, so a zero `max_wall` reliably refuses on the
    /// very first iteration regardless of machine speed.
    #[test]
    fn translate_refuses_with_timeout_when_max_wall_is_zero() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let limits = TranslateLimits {
            max_wall: Some(std::time::Duration::from_nanos(0)),
            ..TranslateLimits::default()
        };
        let err = translate(&ir, &limits).unwrap_err();
        assert!(
            matches!(err, TranslateError::Timeout { .. }),
            "expected Timeout, got {err:?}"
        );
    }

    const FIXTURE_F_DOMAIN: &str = include_str!("../fixtures/f/domain.hddl");
    const FIXTURE_F_PROBLEM: &str = include_str!("../fixtures/f/problem.hddl");

    /// Real combinatorial-blowup regression: fixture F (the IPC2020
    /// blocksworld fixture named in the investigation that motivated
    /// `max_states` — its BFS interned >112,000 states before a 20s
    /// wall-clock budget cut it off, still growing, no plateau) must be
    /// refused with a typed `MemoryLimitExceeded` error — not an OOM crash,
    /// not a silent truncation — once `max_states` is reached, and it must
    /// trigger well before the 10s default wall-clock budget would even be
    /// checked meaningfully (a tiny `max_states` here makes the ceiling the
    /// thing that actually fires, independent of the wall clock: `max_wall`
    /// is left generous so a slow CI machine can't make this test flaky by
    /// racing the two limits against each other).
    #[test]
    fn translate_refuses_with_memory_limit_exceeded_on_blocksworld_blowup() {
        let domain = parse_domain(FIXTURE_F_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_F_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let start = std::time::Instant::now();
        let limits = TranslateLimits {
            max_wall: Some(std::time::Duration::from_secs(60)),
            max_states: Some(500),
            ..TranslateLimits::default()
        };
        let err = translate(&ir, &limits).unwrap_err();
        let elapsed = start.elapsed();
        match err {
            TranslateError::MemoryLimitExceeded { states, limit } => {
                assert!(
                    states >= 500,
                    "expected at least 500 states interned before refusal, got {states}"
                );
                assert_eq!(limit, 500);
            }
            other => panic!("expected MemoryLimitExceeded, got {other:?}"),
        }
        assert!(
            elapsed < std::time::Duration::from_secs(10),
            "memory ceiling should trigger well before the 10s default wall-clock budget, took {elapsed:?}"
        );
    }

    /// End-to-end parse -> ground -> translate over fixture E's
    /// `(:probabilistic 3 (and (heads)) 1 (and (tails)))` effect. `toss-coin`
    /// only becomes executable after the root `flip-coin` task decomposes
    /// via method `m-flip` — it is never directly reachable from the
    /// initial composite state, proving the search really does gate
    /// execution on a real decomposition rather than treating every
    /// precondition-satisfying ground action as available immediately. The
    /// real resulting transition probabilities must land on the declared
    /// 3:1 split (750_000/250_000 ppm), not an even 500_000/500_000 split.
    #[test]
    fn probabilistic_weights_translate_into_proportional_transition_probabilities() {
        let domain = parse_domain(FIXTURE_E_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_E_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let plan = translate(&ir, &TranslateLimits::default()).expect("fixture E translates");

        let initial = plan
            .states
            .iter()
            .find(|s| s.id == plan.initial_states[0])
            .unwrap();
        let decompose = plan
            .transitions
            .iter()
            .find(|t| t.from == initial.id && t.action.ends_with(":m-flip"))
            .expect("initial state decomposes flip-coin via m-flip");
        let decomposed = plan
            .states
            .iter()
            .find(|s| s.id == decompose.to)
            .expect("decomposed state exists");

        let toss_edges = plan
            .transitions
            .iter()
            .filter(|t| t.from == decomposed.id && t.action.ends_with(":toss-coin"))
            .collect::<Vec<_>>();
        assert_eq!(toss_edges.len(), 2);
        let mass: u32 = toss_edges.iter().map(|t| t.probability_ppm).sum();
        assert_eq!(mass, 1_000_000);

        let mut heads_ppm = None;
        let mut tails_ppm = None;
        for edge in &toss_edges {
            let to_state = plan.states.iter().find(|s| s.id == edge.to).unwrap();
            if to_state.facts.contains("heads") {
                heads_ppm = Some(edge.probability_ppm);
            }
            if to_state.facts.contains("tails") {
                tails_ppm = Some(edge.probability_ppm);
            }
        }
        // Weights are 3:1 -> 750_000/250_000, NOT an even 500_000/500_000
        // split -- this is the assertion that would fail if `translate`
        // still ignored `probability_weight` and split evenly.
        assert_eq!(heads_ppm, Some(750_000));
        assert_eq!(tails_ppm, Some(250_000));
    }

    /// Fixture C's oneof action ("cross-bridge") only becomes executable
    /// after the root "reach" task decomposes via method "m-direct" — real
    /// HTN-decomposition-gating, the exact property the old flat BFS did
    /// not have. Both real (non-cyclic) oneof outcomes are reached, and
    /// task-network completion (`"htn:done"`) is folded onto both landing
    /// states since m-direct's single subtask is discharged either way.
    ///
    /// Critically: fixture C's domain *also* declares a second method,
    /// "m-two-step", whose subtask network additionally includes a "walk"
    /// step after crossing the bridge. Under the OLD, decomposition-blind
    /// BFS, landing on the "blocked" outcome (`at(l3)`) would make "walk"
    /// look directly executable (its precondition holds), letting a flat
    /// search wander from an m-direct decomposition straight into a "walk"
    /// step that no decomposition the search actually committed to ever
    /// sanctioned. This test asserts that shortcut is gone: from the
    /// "blocked" outcome reached via "m-direct", the task network is
    /// already exhausted (m-direct had only one subtask), so there must be
    /// NO further transition of any kind — "walk" included.
    #[test]
    fn translates_oneof_fixture_into_a_real_state_graph() {
        let domain = parse_domain(FIXTURE_C_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_C_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let plan = translate(&ir, &TranslateLimits::default()).unwrap();

        assert_eq!(plan.initial_states.len(), 1);
        assert!(plan.goal.facts.contains("at(l2)"));
        assert!(plan.goal.facts.contains("htn:done"));

        let initial = plan
            .states
            .iter()
            .find(|s| s.id == plan.initial_states[0])
            .unwrap();
        assert!(initial.facts.contains("at(l1)"));

        // "cross-bridge" is never directly executable from the initial
        // state -- only decomposition moves are offered there.
        assert!(plan
            .transitions
            .iter()
            .all(|t| t.from != initial.id || t.action.starts_with("htn:decompose:")));

        let decompose = plan
            .transitions
            .iter()
            .find(|t| t.from == initial.id && t.action.ends_with(":m-direct(l1,l2,l3)"))
            .expect("initial state decomposes reach(l1,l2) via m-direct(l1,l2,l3)");
        let decomposed = plan
            .states
            .iter()
            .find(|s| s.id == decompose.to)
            .expect("decomposed state exists");

        // The oneof action must appear with exactly two outcomes from the
        // decomposed state, splitting the full 1_000_000 probability mass.
        let cross_bridge_edges = plan
            .transitions
            .iter()
            .filter(|t| t.from == decomposed.id && t.action.ends_with(":cross-bridge(l1,l2,l3)"))
            .collect::<Vec<_>>();
        assert_eq!(cross_bridge_edges.len(), 2);
        let mass: u32 = cross_bridge_edges.iter().map(|t| t.probability_ppm).sum();
        assert_eq!(mass, 1_000_000);

        let mut landed_on_goal = false;
        let mut landed_on_fallback = false;
        for edge in &cross_bridge_edges {
            let to_state = plan.states.iter().find(|s| s.id == edge.to).unwrap();
            // Either outcome discharges m-direct's one and only subtask, so
            // the task network is done regardless of which outcome occurred.
            assert!(to_state.facts.contains("htn:done"));
            if to_state.facts.contains("at(l2)") {
                landed_on_goal = true;
            }
            if to_state.facts.contains("at(l3)") {
                landed_on_fallback = true;
                // The task network is already exhausted here -- "walk" is
                // NOT part of the decomposition this search committed to
                // (m-direct), so there must be no further transitions at
                // all from this state, "walk" included. This is the real
                // falsifier: the old, decomposition-blind BFS would have
                // offered "walk(l3,l2)" here since its precondition holds.
                let outgoing = plan
                    .transitions
                    .iter()
                    .filter(|t| t.from == to_state.id)
                    .count();
                assert_eq!(
                    outgoing, 0,
                    "no transition (esp. not 'walk') may leave a state whose task \
                     network is already exhausted"
                );
            }
        }
        assert!(landed_on_goal && landed_on_fallback);
    }

    /// `(not (at l1))` at the top of `:goal` (no `and` wrapper) must be
    /// accepted and compiled: no state exists yet, so this only exercises
    /// `flatten_goal` + the empty-BFS goal-fact synthesis, not reachability
    /// filtering across real states (see the fixture-driven tests below for
    /// that).
    #[test]
    fn accepts_bare_negative_goal_atom() {
        use crate::ast::AtomicFormula;
        let ir = GroundedIR {
            goal: GoalDesc::Not(Box::new(GoalDesc::Atom(AtomicFormula {
                predicate: "at".to_owned(),
                args: vec![Term::Const("l1".to_owned())],
            }))),
            initial_facts: BTreeSet::new(),
            ..GroundedIR::default()
        };
        let plan = translate(&ir, &TranslateLimits::default())
            .expect("bare negative goal atom translates");
        // The lone (empty) reachable state has no "at(l1)" fact, so the
        // negation marker must be present on it and required by the goal.
        assert!(plan.goal.facts.contains("not:at(l1)"));
        assert!(!plan.goal.facts.contains("at(l1)"));
        assert_eq!(plan.states.len(), 1);
        assert!(plan.states[0].facts.contains("not:at(l1)"));
    }

    /// `(not (and A B))` at the top of `:goal` now compiles via De Morgan
    /// expansion (`to_dnf`) into the two-clause DNF `(not A) OR (not B)`,
    /// each clause getting the same `goal_reached_marker` treatment a real
    /// `or` gets — this is the "negative goals beyond a single ground
    /// literal" capability `TranslateError::UnsupportedNegativeGoal` used to
    /// refuse outright (see its doc comment: the restriction is gone, the
    /// variant is kept only for the error-type's public API/exhaustive
    /// `Display` match). No state exists yet (empty `GroundedIR`), so this
    /// only exercises `to_dnf` + the empty-BFS goal-fact synthesis, not
    /// reachability filtering across real states (see the fixture-driven
    /// disjunctive-goal test below for that).
    #[test]
    fn accepts_negated_compound_goal_via_de_morgan_expansion() {
        use crate::ast::AtomicFormula;
        let atom = |pred: &str| {
            GoalDesc::Atom(AtomicFormula {
                predicate: pred.to_owned(),
                args: vec![Term::Const("l1".to_owned())],
            })
        };
        let ir = GroundedIR {
            goal: GoalDesc::Not(Box::new(GoalDesc::And(vec![atom("at"), atom("holding")]))),
            initial_facts: BTreeSet::new(),
            ..GroundedIR::default()
        };
        let plan = translate(&ir, &TranslateLimits::default())
            .expect("negated compound goal now translates via De Morgan expansion");
        // Neither "at(l1)" nor "holding(l1)" holds in the lone empty
        // reachable state, so at least one De Morgan disjunct is satisfied
        // and the synthetic marker must be present.
        assert!(plan.goal.facts.contains("goal:reached"));
        assert_eq!(plan.states.len(), 1);
        assert!(plan.states[0].facts.contains("goal:reached"));
    }

    /// A real `(or A B)` `:goal` (not merely supported inside action
    /// preconditions — see the module docs' history) reaches the goal when
    /// EITHER disjunct's real, reachable fact set holds: fixture A's
    /// transport domain with `(or (at l2) (at l3))`, where only `l2` is
    /// actually connected and reachable, must still solve via the `at(l2)`
    /// disjunct — proving `or` in `:goal` is evaluated against real BFS
    /// states via `goal_reached_marker`, not silently refused with
    /// `UnsupportedGoalConnective` the way it used to be.
    #[test]
    fn disjunctive_goal_is_satisfied_by_either_reachable_disjunct() {
        const DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
        const PROBLEM: &str = "(define (problem transport-a-p1-or)
  (:domain transport-a)
  (:objects l1 l2 l3 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (or (at l2) (at l3))))";

        let domain = parse_domain(DOMAIN).unwrap();
        let problem = parse_problem(PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let plan = translate(&ir, &TranslateLimits::default()).expect("disjunctive goal translates");

        assert!(plan.goal.facts.contains("goal:reached"));
        // A real, fully-decomposed reachable state with at(l2) (the whole
        // task network discharged, not merely mid-delivery after "drive"
        // but before "dropoff") must carry the marker and satisfy the goal.
        let at_l2_done = plan
            .states
            .iter()
            .find(|s| s.facts.contains("at(l2)") && s.facts.contains("htn:done"))
            .expect("a real, task-network-complete state with at(l2) must be reachable");
        assert!(at_l2_done.facts.contains("goal:reached"));
        assert!(plan.goal.facts.is_subset(&at_l2_done.facts));
        // ...and l3 (the never-connected disjunct) must never be reachable
        // at all -- proving the goal was satisfied by the real l2 branch,
        // not by some vacuous always-true marker.
        assert!(!plan.states.iter().any(|s| s.facts.contains("at(l3)")));
    }

    /// End-to-end (parse -> ground -> translate) over fixture A (a simple
    /// pickup/drive/dropoff transport domain) with a compound goal `(and (at
    /// l2) (not (has-package)))`: reaching l2 without having dropped the
    /// package off first must NOT count as a goal state, proving the
    /// negation marker really does reachability-filter on the BFS's real,
    /// distinct per-state fact sets rather than trivially holding everywhere.
    #[test]
    fn negative_goal_literal_filters_real_reachable_states() {
        const DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
        // Fixture A's domain (pickup/drive/dropoff) with a compound goal
        // requiring `(at l2)` AND the ABSENCE of `has-package` — i.e. the
        // package must have been dropped off, not merely carried past l2.
        const PROBLEM: &str = "(define (problem transport-a-p1-neg)
  (:domain transport-a)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (m1 (deliver l1 l2))))
  (:init (at l1) (connected l1 l2))
  (:goal (and (at l2) (not (has-package)))))";

        let domain = parse_domain(DOMAIN).unwrap();
        let problem = parse_problem(PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let plan =
            translate(&ir, &TranslateLimits::default()).expect("negative goal literal translates");

        // Real reachable state with (at l2) AND (has-package) still set
        // (pickup -> drive, before dropoff) must exist and must NOT satisfy
        // the compound goal.
        let at_l2_with_package = plan
            .states
            .iter()
            .find(|s| s.facts.contains("at(l2)") && s.facts.contains("has-package"))
            .expect("a real state with at(l2) and has-package must be reachable");
        assert!(
            !plan.goal.facts.is_subset(&at_l2_with_package.facts),
            "a state that still holds the negated fact must not satisfy the goal"
        );
        assert!(at_l2_with_package.facts.contains("not:has-package").not());

        // Real reachable state with (at l2) and NOT (has-package) (after
        // dropoff) must exist and must satisfy the compound goal.
        let at_l2_dropped_off = plan
            .states
            .iter()
            .find(|s| s.facts.contains("at(l2)") && !s.facts.contains("has-package"))
            .expect("a real state with at(l2) and no has-package must be reachable");
        assert!(at_l2_dropped_off.facts.contains("not:has-package"));
        assert!(plan.goal.facts.is_subset(&at_l2_dropped_off.facts));
    }

    /// The falsifier this whole fix targets. Domain declares two unrelated
    /// primitive tasks both usable from `at(l1)`: `drive` (part of the root
    /// task network's actual decomposition) and `special-action` (never
    /// mentioned by any method or the root network at all). The old,
    /// decomposition-blind BFS enumerated `ir.actions` directly against
    /// precondition satisfaction alone, so it would offer `special-action`
    /// as a "shortcut" straight from the initial state and let it reach the
    /// goal fact `cheated` — a plan that does not correspond to any valid
    /// decomposition of the root task network. The fixed, decomposition-
    /// aware BFS must never offer `special-action` at all: it is never a
    /// pending task name at any reachable task-network address.
    #[test]
    fn refuses_a_shortcut_action_unreachable_via_any_decomposition() {
        const DOMAIN: &str = "(define (domain shortcut-g)
  (:types loc)
  (:predicates (at ?l - loc) (cheated))
  (:task run :parameters ())
  (:action drive
    :parameters (?a - loc ?b - loc)
    :precondition (at ?a)
    :effect (and (not (at ?a)) (at ?b)))
  (:action special-action
    :parameters (?a - loc)
    :precondition (at ?a)
    :effect (and (cheated)))
  (:method m-run
    :parameters ()
    :task (run)
    :ordered-subtasks (and (t1 (drive l1 l2)))))";
        const PROBLEM: &str = "(define (problem shortcut-g-p1)
  (:domain shortcut-g)
  (:objects l1 l2 - loc)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (at l1))
  (:goal (and (cheated))))";

        let domain = parse_domain(DOMAIN).unwrap();
        let problem = parse_problem(PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        // `special-action(l1)` is a real, precondition-satisfiable ground
        // action -- the bug this fixes is specifically that such an action
        // must NOT be reachable in the translated graph unless some
        // decomposition of the root task network actually names it.
        assert!(ir.actions.iter().any(|a| a.name == "special-action(l1)"));

        let plan =
            translate(&ir, &TranslateLimits::default()).expect("shortcut fixture translates");

        assert!(
            plan.transitions
                .iter()
                .all(|t| !t.action.contains("special-action")),
            "special-action must never appear in any transition: it is not reachable \
             via any decomposition of the root task network, even though its \
             precondition is satisfied from the initial state"
        );
        assert!(
            !plan.states.iter().any(|s| s.facts.contains("cheated")),
            "the goal fact 'cheated' must never become reachable through the \
             excluded shortcut action"
        );
    }

    /// Method-precondition gating. Two methods (`use-a`, `use-b`) both
    /// decompose the same task (`run`), with mutually-exclusive
    /// preconditions (`(ready)` vs `(not (ready))`) over the same initial
    /// fact. Before the fix, `translate` offered every ground method whose
    /// `task_name` matched a pending task regardless of world state — both
    /// `use-a` and `use-b` would be offered as decomposition branches from
    /// the initial state, and `mark-b`/`done-b` would become reachable even
    /// though `(ready)` holds initially and `use-b`'s precondition
    /// `(not (ready))` does not. Grounding itself must NOT filter on the
    /// precondition (methods are grounded once, statically, before any
    /// fact-set exists to check against) — both ground methods must still
    /// appear in `ir.methods`; only `translate`'s per-state BFS, which does
    /// have a real fact set to evaluate against, may exclude one.
    #[test]
    fn method_precondition_gates_which_decomposition_is_offered() {
        const DOMAIN: &str = "(define (domain method-precond-g)
  (:predicates (ready) (done-a) (done-b))
  (:task run :parameters ())
  (:method use-a
    :parameters ()
    :task (run)
    :precondition (ready)
    :ordered-subtasks (mark-a))
  (:method use-b
    :parameters ()
    :task (run)
    :precondition (not (ready))
    :ordered-subtasks (mark-b))
  (:action mark-a
    :effect (done-a))
  (:action mark-b
    :effect (done-b)))";
        const PROBLEM: &str = "(define (problem method-precond-g-p1)
  (:domain method-precond-g)
  (:objects)
  (:htn
    :parameters ()
    :ordered-subtasks (and (g1 (run))))
  (:init (ready))
  (:goal (and (done-a))))";

        let domain = parse_domain(DOMAIN).unwrap();
        let problem = parse_problem(PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();

        // Both ground methods must exist post-grounding: grounding is
        // static and has no fact set to evaluate a precondition against, so
        // it must never itself prune on world state.
        assert!(
            ir.methods.iter().any(|m| m.name == "use-a"),
            "use-a must still be a real ground method after grounding"
        );
        assert!(
            ir.methods.iter().any(|m| m.name == "use-b"),
            "use-b must still be a real ground method after grounding \
             (grounding is world-state-blind by construction)"
        );

        let plan = translate(&ir, &TranslateLimits::default())
            .expect("method-precondition fixture translates");

        // `use-b`'s precondition `(not (ready))` never holds from the
        // initial state onward (nothing in this domain ever retracts
        // `ready`), so `use-b` must never be offered as a decomposition
        // transition, and `mark-b`/`done-b` must never become reachable.
        assert!(
            plan.transitions
                .iter()
                .all(|t| !t.action.contains(":use-b")),
            "use-b's precondition never holds, so it must never appear as a \
             decomposition transition: {:?}",
            plan.transitions
                .iter()
                .map(|t| &t.action)
                .collect::<Vec<_>>()
        );
        assert!(
            !plan.states.iter().any(|s| s.facts.contains("done-b")),
            "done-b must never become reachable: only use-b's excluded \
             subtask (mark-b) could ever produce it"
        );

        // `use-a`'s precondition `(ready)` holds from the initial state, so
        // it must be offered, and the real goal (`done-a`) must be reached.
        assert!(
            plan.transitions.iter().any(|t| t.action.contains(":use-a")),
            "use-a's precondition holds, so it must appear as a decomposition \
             transition"
        );
        assert!(
            plan.states
                .iter()
                .any(|s| plan.goal.facts.is_subset(&s.facts)),
            "a real goal-satisfying state (done-a reached via use-a/mark-a) \
             must be reachable"
        );
    }
}
