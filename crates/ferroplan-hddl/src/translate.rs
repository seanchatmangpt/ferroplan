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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateError {
    /// `(not <non-atomic goal description>)` — e.g. `(not (and ...))`.
    /// `ferroplan::planning_runtime::Goal` has no negative-fact field, so a
    /// negative *atomic* literal is instead compiled here into a synthetic
    /// marker fact (see `negation_marker`) rather than needing one, but that
    /// compilation only knows how to negate a single ground atom — the same
    /// restriction the grounder already applies to `not` inside
    /// preconditions and `when`-conditions (see `grounder::flatten_goal`).
    UnsupportedNegativeGoal,
    /// `:goal` used `or`/`imply` at some point in the formula. Action
    /// *preconditions* fully support `or`/`imply` (see
    /// `grounder::GroundGoal`/`evaluate_ground_goal`), but the problem
    /// `:goal` still only supports positive/negative literals — extending
    /// past that means giving `ferroplan::planning_runtime::Goal` real
    /// disjunctive-goal semantics, which (like the negative-goal marker
    /// above) would change behavior for every planning family that shares
    /// that type, not just HDDL. Out of scope for this pass; refused loudly
    /// here rather than silently mis-translated.
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
}

impl Default for TranslateLimits {
    fn default() -> Self {
        Self {
            max_task_network_depth: 64,
        }
    }
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

/// Splits a `:goal` description into positive ground atoms (`pos`) and
/// atoms whose absence the goal requires (`neg`). `Not` is only supported
/// directly around a single atom — `(not (and ...))` and similar remain
/// out of scope, the same restriction `grounder::flatten_goal` already
/// applies to preconditions and `when`-conditions.
fn flatten_goal(
    goal: &GoalDesc,
    pos: &mut BTreeSet<String>,
    neg: &mut BTreeSet<String>,
) -> Result<(), TranslateError> {
    match goal {
        GoalDesc::Empty => Ok(()),
        GoalDesc::Atom(a) => {
            pos.insert(ground_atom_key(a)?);
            Ok(())
        }
        GoalDesc::Not(inner) => match inner.as_ref() {
            GoalDesc::Atom(a) => {
                neg.insert(ground_atom_key(a)?);
                Ok(())
            }
            _ => Err(TranslateError::UnsupportedNegativeGoal),
        },
        GoalDesc::And(parts) => {
            for p in parts {
                flatten_goal(p, pos, neg)?;
            }
            Ok(())
        }
        GoalDesc::Or(_) => Err(TranslateError::UnsupportedGoalConnective("or".to_owned())),
        GoalDesc::Imply(_, _) => Err(TranslateError::UnsupportedGoalConnective(
            "imply".to_owned(),
        )),
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

pub fn translate(
    ir: &GroundedIR,
    limits: &TranslateLimits,
) -> Result<PlanningProblem, TranslateError> {
    let mut goal_pos = BTreeSet::new();
    let mut goal_neg = BTreeSet::new();
    flatten_goal(&ir.goal, &mut goal_pos, &mut goal_neg)?;

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

    // Reachability-filtering for negative goal literals: the real BFS above
    // is already finished (every state's `facts` is its exact, final ground
    // fact set, computed purely from real action effects — untouched by
    // what follows). For each `(not P)` goal literal, add the synthetic
    // `negation_marker(P)` fact to exactly the states where `P` is genuinely
    // absent, so the shared, unmodified `Goal::holds` positive-subset test
    // downstream can express "P does not hold" as "this marker fact does".
    if !goal_neg.is_empty() {
        for state in &mut states {
            for neg_fact in &goal_neg {
                if !state.facts.contains(neg_fact) {
                    state.facts.insert(negation_marker(neg_fact));
                }
            }
        }
    }
    let mut goal_facts = goal_pos;
    for neg_fact in &goal_neg {
        goal_facts.insert(negation_marker(neg_fact));
    }
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

    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../fixtures/c/problem.hddl");
    const FIXTURE_E_DOMAIN: &str = include_str!("../fixtures/e/domain.hddl");
    const FIXTURE_E_PROBLEM: &str = include_str!("../fixtures/e/problem.hddl");

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

    /// `(not <non-atomic goal description>)` remains out of scope — the same
    /// restriction `grounder::flatten_goal` already applies to preconditions
    /// and `when`-conditions. This is the one shape `TranslateError::
    /// UnsupportedNegativeGoal` still names.
    #[test]
    fn rejects_not_of_a_non_atomic_goal_description() {
        use crate::ast::AtomicFormula;
        let ir = GroundedIR {
            goal: GoalDesc::Not(Box::new(GoalDesc::And(vec![GoalDesc::Atom(
                AtomicFormula {
                    predicate: "at".to_owned(),
                    args: vec![Term::Const("l1".to_owned())],
                },
            )]))),
            ..GroundedIR::default()
        };
        let err = translate(&ir, &TranslateLimits::default()).unwrap_err();
        assert_eq!(err, TranslateError::UnsupportedNegativeGoal);
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
}
