//! Bounded native implementations for every planning family admitted by
//! [`crate::planning_types::PlanningType`].
//!
//! These planners operate over one explicit, serializable state-transition
//! model.  PDDL/PPDDL front-ends may project into this model; RDF, A2A, and MCP
//! front-ends may manufacture it directly.  The module performs no actuation.

use crate::planning_types::PlanningType;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use std::fmt;
use std::time::Instant;

const PROBABILITY_SCALE: u64 = 1_000_000;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    #[serde(default)]
    pub facts: BTreeSet<String>,
    #[serde(default)]
    pub fluents: BTreeMap<String, i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    #[serde(default)]
    pub facts: BTreeSet<String>,
    #[serde(default)]
    pub numeric_min: BTreeMap<String, i64>,
    #[serde(default)]
    pub numeric_max: BTreeMap<String, i64>,
}

impl Goal {
    fn holds(&self, state: &State) -> bool {
        self.facts.is_subset(&state.facts)
            && self
                .numeric_min
                .iter()
                .all(|(name, value)| state.fluents.get(name).is_some_and(|v| v >= value))
            && self
                .numeric_max
                .iter()
                .all(|(name, value)| state.fluents.get(name).is_some_and(|v| v <= value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    pub action: String,
    pub from: String,
    pub to: String,
    #[serde(default = "one")]
    pub cost: u64,
    #[serde(default = "one")]
    pub duration: u64,
    #[serde(default)]
    pub reward: i64,
    /// Probability in millionths. Deterministic edges use 1_000_000.
    #[serde(default = "probability_one")]
    pub probability_ppm: u32,
    #[serde(default)]
    pub observation: Option<String>,
    #[serde(default)]
    pub requires: BTreeSet<String>,
}

fn one() -> u64 {
    1
}
fn probability_one() -> u32 {
    PROBABILITY_SCALE as u32
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(default)]
    pub primitive_action: Option<String>,
    #[serde(default)]
    pub requires: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Method {
    pub id: String,
    pub task: String,
    #[serde(default)]
    pub subtasks: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub before: String,
    pub after: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueState {
    pub id: String,
    pub current_wip: u64,
    pub max_wip: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    #[serde(default)]
    pub capabilities: BTreeSet<String>,
    #[serde(default = "one")]
    pub capacity: u64,
    #[serde(default)]
    pub current_wip: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    #[serde(default)]
    pub capabilities: BTreeSet<String>,
    #[serde(default)]
    pub authority_bound: bool,
    #[serde(default)]
    pub verifier_bound: bool,
    #[serde(default)]
    pub receipt_bound: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RdfTriple {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProblem {
    #[serde(default)]
    pub states: Vec<State>,
    #[serde(default)]
    pub initial_states: Vec<String>,
    #[serde(default)]
    pub goal: Goal,
    #[serde(default)]
    pub unsafe_states: BTreeSet<String>,
    #[serde(default)]
    pub soft_goal_facts: BTreeMap<String, u64>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub root_tasks: Vec<String>,
    #[serde(default)]
    pub methods: Vec<Method>,
    #[serde(default)]
    pub workflow_edges: Vec<WorkflowEdge>,
    #[serde(default)]
    pub queues: Vec<QueueState>,
    #[serde(default)]
    pub agents: Vec<Agent>,
    #[serde(default)]
    pub tools: Vec<Tool>,
    #[serde(default)]
    pub rdf: Vec<RdfTriple>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerLimits {
    #[serde(default = "default_depth")]
    pub max_depth: usize,
    #[serde(default = "default_states")]
    pub max_states: usize,
    #[serde(default = "default_iterations")]
    pub max_iterations: usize,
    /// Wall-clock budget in milliseconds for one `solve_planning_type` call.
    /// `0` means unbounded (matches every pre-existing caller/test's current
    /// behavior if it explicitly constructs `PlannerLimits { .. }` with this
    /// left at its bare-integer default); `default()` sets a real bound so a
    /// caller using `PlannerLimits::default()` is protected without opting
    /// in. Plain `u64` milliseconds rather than `Option<Duration>` (unlike
    /// `ferroplan_hddl::{GroundingLimits, TranslateLimits}::max_wall`)
    /// specifically to match this struct's own existing convention — every
    /// other field here is a bare, always-serializable integer, and
    /// `PlannerLimits` round-trips through `serde_json` (see
    /// `UniversalPlanningRequest`), where `Option<Duration>` is an
    /// unnecessary complication a plain `u64` sidesteps entirely.
    #[serde(default = "default_wall_ms")]
    pub max_wall_ms: u64,
}

fn default_depth() -> usize {
    128
}
fn default_states() -> usize {
    100_000
}
fn default_iterations() -> usize {
    512
}
fn default_wall_ms() -> u64 {
    10_000
}

impl Default for PlannerLimits {
    fn default() -> Self {
        Self {
            max_depth: default_depth(),
            max_states: default_states(),
            max_iterations: default_iterations(),
            max_wall_ms: default_wall_ms(),
        }
    }
}

/// Checked once per outer-loop iteration by the fixpoint solvers whose
/// round cost scales with the problem's state/action count (`fond_policy`,
/// bounded by `max_iterations`; `fond_policy_strong_cyclic`, bounded by its
/// `states + 1` round failsafe) — see `PlannerError::Timeout`'s doc comment
/// for why a wall-clock bound is needed in addition to a round count
/// (a single round's cost scales with the problem's state/action count,
/// which neither round bound alone bounds).
fn check_wall_deadline(start: Instant, limits: &PlannerLimits) -> Result<(), PlannerError> {
    if limits.max_wall_ms > 0 {
        let elapsed = start.elapsed();
        if elapsed.as_millis() as u64 > limits.max_wall_ms {
            return Err(PlannerError::Timeout {
                elapsed_ms: elapsed.as_millis(),
                limit_ms: u128::from(limits.max_wall_ms),
            });
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversalPlanningRequest {
    pub planning_type: PlanningType,
    pub problem: PlanningProblem,
    #[serde(default)]
    pub limits: PlannerLimits,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanStep {
    pub action: String,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub start: u64,
    #[serde(default)]
    pub duration: u64,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub tool: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyOutcome {
    pub state: String,
    pub probability_ppm: u32,
    #[serde(default)]
    pub observation: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEntry {
    pub state: String,
    pub action: String,
    #[serde(default)]
    pub outcomes: Vec<PolicyOutcome>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversalPlan {
    pub planning_type: Option<PlanningType>,
    pub solved: bool,
    #[serde(default)]
    pub steps: Vec<PlanStep>,
    #[serde(default)]
    pub policy: Vec<PolicyEntry>,
    #[serde(default)]
    pub decomposition: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlannerError {
    EmptyInitialState,
    UnknownState {
        state: String,
    },
    InvalidProbabilityMass {
        state: String,
        action: String,
        mass: u64,
    },
    ResourceBound {
        resource: String,
        limit: usize,
    },
    NoPlan,
    HierarchyCycle {
        task: String,
    },
    UnknownTask {
        task: String,
    },
    NoMethod {
        task: String,
    },
    WorkflowCycle,
    WipBoundExceeded {
        queue: String,
        current: u64,
        max: u64,
    },
    CapabilityUncovered {
        item: String,
        missing: BTreeSet<String>,
    },
    AuthorityUnbound {
        tool: String,
    },
    VerifierUnbound {
        tool: String,
    },
    ReceiptUnbound {
        tool: String,
    },
    InvalidRdfProjection {
        reason: String,
    },
    /// `PlannerLimits::max_wall_ms` elapsed before the solver finished. A
    /// wall-clock companion to `ResourceBound`: `max_iterations`/`max_states`
    /// bound the *count* of loop rounds/states a solver visits, but a single
    /// round's cost scales with the problem size (number of states/actions),
    /// so a large-but-within-count-limits problem can still run for an
    /// unbounded amount of real time — see `fond_policy`'s and
    /// `fond_policy_strong_cyclic`'s call sites of `check_wall_deadline` for
    /// the two fixpoint loops this actually protects today.
    Timeout {
        elapsed_ms: u128,
        limit_ms: u128,
    },
}

impl fmt::Display for PlannerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for PlannerError {}

/// Execute the bounded native planner corresponding to the request type.
pub fn solve_planning_type(
    request: &UniversalPlanningRequest,
) -> Result<UniversalPlan, PlannerError> {
    validate_problem(&request.problem)?;
    let mut plan = match request.planning_type {
        PlanningType::Classical => shortest_path(&request.problem, Metric::Steps, &request.limits),
        PlanningType::CostOptimal => shortest_path(&request.problem, Metric::Cost, &request.limits),
        PlanningType::Numeric => shortest_path(&request.problem, Metric::Cost, &request.limits),
        PlanningType::Temporal => {
            shortest_path(&request.problem, Metric::Duration, &request.limits)
        }
        PlanningType::Preferences => preference_plan(&request.problem, &request.limits),
        PlanningType::Probabilistic => probabilistic_policy(&request.problem, &request.limits),
        // Prefer the acyclic strong-plan fixpoint (bounded steps, no
        // fairness assumption needed); fall back to the strong-cyclic
        // fixpoint only when that fails, so this changes no behavior for
        // any domain `fond_policy` already solves -- it only adds coverage
        // for domains that structurally require a retry loop.
        PlanningType::Fond => match fond_policy(&request.problem, &request.limits) {
            Ok(plan) => Ok(plan),
            Err(PlannerError::NoPlan) => {
                fond_policy_strong_cyclic(&request.problem, &request.limits)
            }
            Err(other) => Err(other),
        },
        PlanningType::Conformant => conformant_plan(&request.problem, &request.limits),
        PlanningType::Contingent => contingent_policy(&request.problem, &request.limits),
        PlanningType::Hierarchical => hierarchical_plan(&request.problem, &request.limits),
        PlanningType::PartialOrder | PlanningType::Workflow => {
            workflow_plan(&request.problem, &request.limits)
        }
        PlanningType::FlowConstrained => flow_plan(&request.problem, &request.limits),
        PlanningType::ResolutionAdaptive => resolution_plan(&request.problem, &request.limits),
        PlanningType::MultiAgent => multi_agent_plan(&request.problem, &request.limits),
        PlanningType::RdfDerived => rdf_plan(&request.problem, &request.limits),
        PlanningType::A2aDelegated => delegated_plan(&request.problem, &request.limits),
        PlanningType::McpBound => mcp_plan(&request.problem, &request.limits),
    }?;
    plan.planning_type = Some(request.planning_type);
    Ok(plan)
}

fn validate_problem(problem: &PlanningProblem) -> Result<(), PlannerError> {
    if problem.initial_states.is_empty()
        && !matches!(problem.tasks.as_slice(), [_, ..])
        && problem.rdf.is_empty()
    {
        return Err(PlannerError::EmptyInitialState);
    }
    let states = problem
        .states
        .iter()
        .map(|state| state.id.as_str())
        .collect::<BTreeSet<_>>();
    for initial in &problem.initial_states {
        if !states.contains(initial.as_str()) {
            return Err(PlannerError::UnknownState {
                state: initial.clone(),
            });
        }
    }
    for transition in &problem.transitions {
        for state in [&transition.from, &transition.to] {
            if !states.contains(state.as_str()) {
                return Err(PlannerError::UnknownState {
                    state: state.clone(),
                });
            }
        }
    }
    let mut masses = BTreeMap::<(&str, &str), u64>::new();
    for edge in &problem.transitions {
        *masses.entry((&edge.from, &edge.action)).or_default() += u64::from(edge.probability_ppm);
    }
    for ((state, action), mass) in masses {
        if mass != PROBABILITY_SCALE {
            return Err(PlannerError::InvalidProbabilityMass {
                state: state.to_owned(),
                action: action.to_owned(),
                mass,
            });
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Metric {
    Steps,
    Cost,
    Duration,
}

fn edge_weight(edge: &Transition, metric: Metric) -> u64 {
    match metric {
        Metric::Steps => 1,
        Metric::Cost => edge.cost,
        Metric::Duration => edge.duration,
    }
}

fn state_index(problem: &PlanningProblem) -> BTreeMap<&str, &State> {
    problem
        .states
        .iter()
        .map(|state| (state.id.as_str(), state))
        .collect()
}

fn grouped_edges(problem: &PlanningProblem) -> BTreeMap<&str, Vec<&Transition>> {
    let mut grouped = BTreeMap::<&str, Vec<&Transition>>::new();
    for edge in &problem.transitions {
        grouped.entry(edge.from.as_str()).or_default().push(edge);
    }
    grouped
}

fn action_groups(problem: &PlanningProblem) -> BTreeMap<(&str, &str), Vec<&Transition>> {
    let mut grouped = BTreeMap::<(&str, &str), Vec<&Transition>>::new();
    for edge in &problem.transitions {
        grouped
            .entry((edge.from.as_str(), edge.action.as_str()))
            .or_default()
            .push(edge);
    }
    grouped
}

fn reconstruct(goal: &str, parent: &BTreeMap<String, (String, Transition)>) -> Vec<PlanStep> {
    let mut cursor = goal.to_owned();
    let mut reverse = Vec::new();
    while let Some((previous, edge)) = parent.get(&cursor) {
        reverse.push(PlanStep {
            action: edge.action.clone(),
            from: Some(edge.from.clone()),
            to: Some(edge.to.clone()),
            duration: edge.duration,
            ..PlanStep::default()
        });
        cursor = previous.clone();
    }
    reverse.reverse();
    let mut start = 0;
    for step in &mut reverse {
        step.start = start;
        start += step.duration;
    }
    reverse
}

fn shortest_path(
    problem: &PlanningProblem,
    metric: Metric,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let states = state_index(problem);
    let outgoing = grouped_edges(problem);
    let mut heap = BinaryHeap::new();
    let mut distance = BTreeMap::<String, u64>::new();
    let mut parent = BTreeMap::<String, (String, Transition)>::new();
    for initial in &problem.initial_states {
        distance.insert(initial.clone(), 0);
        heap.push(Reverse((0_u64, initial.clone())));
    }
    let mut visited = 0;
    while let Some(Reverse((cost, state_id))) = heap.pop() {
        if distance.get(&state_id).is_some_and(|best| cost != *best) {
            continue;
        }
        visited += 1;
        if visited > limits.max_states {
            return Err(PlannerError::ResourceBound {
                resource: "states".to_owned(),
                limit: limits.max_states,
            });
        }
        let state = states[state_id.as_str()];
        if problem.goal.holds(state) {
            return Ok(UniversalPlan {
                solved: true,
                steps: reconstruct(&state_id, &parent),
                ..UniversalPlan::default()
            });
        }
        for edge in outgoing.get(state_id.as_str()).into_iter().flatten() {
            if problem.unsafe_states.contains(&edge.to) {
                continue;
            }
            let next = cost.saturating_add(edge_weight(edge, metric));
            if distance.get(&edge.to).map_or(true, |known| next < *known) {
                distance.insert(edge.to.clone(), next);
                parent.insert(edge.to.clone(), (state_id.clone(), (*edge).clone()));
                heap.push(Reverse((next, edge.to.clone())));
            }
        }
    }
    Err(PlannerError::NoPlan)
}

fn preference_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let states = state_index(problem);
    let outgoing = grouped_edges(problem);
    let mut heap = BinaryHeap::new();
    let mut distance = BTreeMap::<String, u64>::new();
    let mut parent = BTreeMap::<String, (String, Transition)>::new();
    let mut best_goal: Option<(u64, String)> = None;
    for initial in &problem.initial_states {
        distance.insert(initial.clone(), 0);
        heap.push(Reverse((0_u64, initial.clone())));
    }
    let mut visited = 0;
    while let Some(Reverse((cost, state_id))) = heap.pop() {
        if distance.get(&state_id).is_some_and(|best| cost != *best) {
            continue;
        }
        visited += 1;
        if visited > limits.max_states {
            return Err(PlannerError::ResourceBound {
                resource: "states".to_owned(),
                limit: limits.max_states,
            });
        }
        let state = states[state_id.as_str()];
        if problem.goal.holds(state) {
            let penalty = problem
                .soft_goal_facts
                .iter()
                .filter(|(fact, _)| !state.facts.contains(*fact))
                .map(|(_, penalty)| *penalty)
                .sum::<u64>();
            let score = cost.saturating_add(penalty);
            if best_goal.as_ref().map_or(true, |(best, _)| score < *best) {
                best_goal = Some((score, state_id.clone()));
            }
        }
        for edge in outgoing.get(state_id.as_str()).into_iter().flatten() {
            if problem.unsafe_states.contains(&edge.to) {
                continue;
            }
            let next = cost.saturating_add(edge.cost);
            if distance.get(&edge.to).map_or(true, |known| next < *known) {
                distance.insert(edge.to.clone(), next);
                parent.insert(edge.to.clone(), (state_id.clone(), (*edge).clone()));
                heap.push(Reverse((next, edge.to.clone())));
            }
        }
    }
    let (_, goal) = best_goal.ok_or(PlannerError::NoPlan)?;
    Ok(UniversalPlan {
        solved: true,
        steps: reconstruct(&goal, &parent),
        ..UniversalPlan::default()
    })
}

fn probabilistic_policy(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let _states = state_index(problem);
    let groups = action_groups(problem);
    let mut values = problem
        .states
        .iter()
        .map(|state| {
            let value = if problem.goal.holds(state) {
                PROBABILITY_SCALE
            } else {
                0
            };
            (state.id.clone(), value as i128)
        })
        .collect::<BTreeMap<_, _>>();
    let mut choices = BTreeMap::<String, String>::new();
    for _ in 0..limits.max_iterations {
        let mut changed = false;
        let mut next_values = values.clone();
        for state in &problem.states {
            if problem.goal.holds(state) || problem.unsafe_states.contains(&state.id) {
                continue;
            }
            let mut best: Option<(i128, String)> = None;
            for ((from, action), edges) in &groups {
                if *from != state.id {
                    continue;
                }
                let score = edges
                    .iter()
                    .map(|edge| {
                        i128::from(edge.probability_ppm)
                            * (values[&edge.to] + i128::from(edge.reward))
                    })
                    .sum::<i128>()
                    / i128::from(PROBABILITY_SCALE);
                if best.as_ref().map_or(true, |(value, _)| score > *value) {
                    best = Some((score, (*action).to_owned()));
                }
            }
            if let Some((value, action)) = best {
                if next_values[&state.id] != value {
                    changed = true;
                }
                next_values.insert(state.id.clone(), value);
                choices.insert(state.id.clone(), action);
            }
        }
        values = next_values;
        if !changed {
            break;
        }
    }
    let initial_has_value = problem
        .initial_states
        .iter()
        .any(|state| values.get(state).is_some_and(|value| *value > 0));
    if !initial_has_value {
        return Err(PlannerError::NoPlan);
    }
    Ok(policy_from_choices(
        problem,
        choices,
        "bounded value iteration",
    ))
}

fn policy_from_choices(
    problem: &PlanningProblem,
    choices: BTreeMap<String, String>,
    note: &str,
) -> UniversalPlan {
    let groups = action_groups(problem);
    let policy = choices
        .into_iter()
        .map(|(state, action)| PolicyEntry {
            outcomes: groups
                .get(&(state.as_str(), action.as_str()))
                .into_iter()
                .flatten()
                .map(|edge| PolicyOutcome {
                    state: edge.to.clone(),
                    probability_ppm: edge.probability_ppm,
                    observation: edge.observation.clone(),
                })
                .collect(),
            state,
            action,
        })
        .collect();
    UniversalPlan {
        solved: true,
        policy,
        notes: vec![note.to_owned()],
        ..UniversalPlan::default()
    }
}

fn fond_policy(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let start = Instant::now();
    let states = state_index(problem);
    let groups = action_groups(problem);
    let mut winning = problem
        .states
        .iter()
        .filter(|state| problem.goal.holds(state))
        .map(|state| state.id.clone())
        .collect::<BTreeSet<_>>();
    let mut choices = BTreeMap::<String, String>::new();
    for _ in 0..limits.max_iterations {
        check_wall_deadline(start, limits)?;
        let mut changed = false;
        for state in &problem.states {
            if winning.contains(&state.id) || problem.unsafe_states.contains(&state.id) {
                continue;
            }
            for ((from, action), outcomes) in &groups {
                if *from == state.id
                    && !outcomes.is_empty()
                    && outcomes.iter().all(|edge| winning.contains(&edge.to))
                {
                    winning.insert(state.id.clone());
                    choices.insert(state.id.clone(), (*action).to_owned());
                    changed = true;
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }
    if !problem
        .initial_states
        .iter()
        .all(|state| winning.contains(state))
    {
        return Err(PlannerError::NoPlan);
    }
    // Goal states need no policy entry; all non-goal winning states do.
    for state in &winning {
        if !problem.goal.holds(states[state.as_str()]) && !choices.contains_key(state) {
            return Err(PlannerError::NoPlan);
        }
    }
    Ok(policy_from_choices(
        problem,
        choices,
        "strong FOND fixed point",
    ))
}

/// Strong-cyclic FOND fixpoint solver (Cimatti, Pistore, Roveri, Traverso,
/// *"Weak, Strong, and Strong Cyclic Planning via Symbolic Model Checking,"*
/// AIJ 2003 — the standard reference algorithm). Unlike [`fond_policy`] (a
/// least fixpoint grown from the goal outward, which can only express
/// acyclic strong plans — a cyclic state's own successor set always
/// contains a not-yet-`winning` member, namely itself, at the moment it
/// would need to be admitted), this runs the standard two-phase
/// construction:
///
/// - **Phase 1 (weak/OR backward reachability):** compute `weak`, the set of
///   states from which the goal is reachable under *some* lucky run — seed
///   at the goal states, then repeatedly admit any state with at least one
///   outgoing edge (any action, any single outcome) landing in `weak`. This
///   deliberately ignores that action's other outcomes.
/// - **Phase 2 (greatest-fixpoint prune, AND-semantics, restricted to
///   `weak`):** start optimistically at `surviving := weak`, then repeatedly
///   remove any non-goal state with **no** action all of whose outcomes
///   still land in `surviving`. Because `surviving` starts as the *whole*
///   weakly-reachable set rather than empty, a self-loop outcome survives as
///   long as its state does — this is exactly what admits retry loops that
///   `fond_policy`'s least-fixpoint-from-empty construction cannot express.
///
/// Soundness/completeness of this construction for **strong-cyclic** (not
/// strong) solutions relies on the standard fairness assumption: every
/// non-deterministic outcome that is reachable infinitely often along an
/// infinite execution eventually occurs. Unlike `fond_policy`, this solver
/// does not additionally guarantee a bounded number of steps to the goal —
/// only that the goal is reached with probability 1 in the limit under
/// fairness. Every acyclic strong solution `fond_policy` finds is also a
/// strong-cyclic solution (`weak` always contains `fond_policy`'s `winning`,
/// since OR-reachability is weaker than AND-reachability, and Phase 2 never
/// prunes a state that had an all-outcomes-covered witness), so this
/// function is a strict superset solver relative to `fond_policy`.
fn fond_policy_strong_cyclic(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let start = Instant::now();
    let states = state_index(problem);
    let edges_by_from = grouped_edges(problem);
    let groups = action_groups(problem);

    // Both fixpoint loops below are bounded by their own mathematical
    // termination argument, NOT by `PlannerLimits::max_iterations`: every
    // round that changes anything admits (Phase 1) or prunes (Phase 2) at
    // least one state, so each loop converges within
    // `problem.states.len()` changing rounds plus one confirming round.
    // `max_iterations` used to gate these loops; truncating a
    // greatest-fixpoint prune mid-convergence left witnesses validated
    // early in a round pointing at states pruned later in the same round —
    // a returned policy with a dead sink (the defect this fixes), so it no
    // longer gates either loop. The `states + 1` failsafe is unreachable by
    // the argument above; if it ever fires that is an internal invariant
    // violation and must surface as `Err(Timeout)` rather than a silently
    // truncated (possibly stale) policy — `limit_ms` echoes the configured
    // wall budget (`0` when unbounded) because the `Timeout` payload is
    // wall-shaped, but the bound that fired is the round failsafe.
    // `check_wall_deadline` still runs every round, so wall-clock deadline
    // semantics (`Timeout` propagation) are unchanged.
    let failsafe_rounds = problem.states.len() + 1;

    // Phase 1: weak/OR backward reachability -> `weak`.
    let mut weak = problem
        .states
        .iter()
        .filter(|state| problem.goal.holds(state))
        .map(|state| state.id.clone())
        .collect::<BTreeSet<_>>();
    let mut rounds = 0_usize;
    loop {
        check_wall_deadline(start, limits)?;
        rounds += 1;
        if rounds > failsafe_rounds {
            return Err(PlannerError::Timeout {
                elapsed_ms: start.elapsed().as_millis(),
                limit_ms: u128::from(limits.max_wall_ms),
            });
        }
        let mut changed = false;
        for state in &problem.states {
            if weak.contains(&state.id) || problem.unsafe_states.contains(&state.id) {
                continue;
            }
            if edges_by_from
                .get(state.id.as_str())
                .into_iter()
                .flatten()
                .any(|edge| weak.contains(&edge.to))
            {
                weak.insert(state.id.clone());
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Phase 2: greatest-fixpoint prune weak -> surviving, extracting the
    // policy as states are confirmed to have an all-outcomes-covered action.
    let mut surviving = weak.clone();
    let mut choices = BTreeMap::<String, String>::new();
    let mut rounds = 0_usize;
    loop {
        check_wall_deadline(start, limits)?;
        rounds += 1;
        if rounds > failsafe_rounds {
            return Err(PlannerError::Timeout {
                elapsed_ms: start.elapsed().as_millis(),
                limit_ms: u128::from(limits.max_wall_ms),
            });
        }
        let mut changed = false;
        for state in &problem.states {
            if !surviving.contains(&state.id) || problem.goal.holds(state) {
                continue;
            }
            let witness = groups.iter().find(|((from, _action), outcomes)| {
                *from == state.id
                    && !outcomes.is_empty()
                    && outcomes.iter().all(|edge| surviving.contains(&edge.to))
            });
            match witness {
                Some(((_, action), _)) => {
                    choices.insert(state.id.clone(), (*action).to_owned());
                }
                None => {
                    surviving.remove(&state.id);
                    choices.remove(&state.id);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    if !problem
        .initial_states
        .iter()
        .all(|state| surviving.contains(state))
    {
        return Err(PlannerError::NoPlan);
    }
    // Outcome closure, computed on the FINAL (surviving, choices) pair:
    // every surviving non-goal state must have a choice, and every outcome
    // of that choice must land in a goal state or in another surviving
    // state that itself has a choice. Phase 2 validates each state's
    // witness against `surviving` *as of its own round*, so a witness
    // validated early in a round can be invalidated by a prune later in
    // the same round; when `max_iterations` used to truncate the loop
    // before the next revalidation round, that stale witness survived into
    // the returned policy as a choice leading to a pruned, choice-less,
    // non-goal state (a dead sink). This check is the soundness witness
    // that no stale witness can masquerade as a plan.
    for state in &surviving {
        if problem.goal.holds(states[state.as_str()]) {
            continue;
        }
        let Some(action) = choices.get(state) else {
            // Goal states need no policy entry; all other surviving states do.
            return Err(PlannerError::NoPlan);
        };
        let closed = groups
            .get(&(state.as_str(), action.as_str()))
            .into_iter()
            .flatten()
            .all(|edge| {
                problem.goal.holds(states[edge.to.as_str()])
                    || (surviving.contains(&edge.to) && choices.contains_key(&edge.to))
            });
        if !closed {
            return Err(PlannerError::NoPlan);
        }
    }

    Ok(policy_from_choices(
        problem,
        choices,
        "strong-cyclic FOND fixpoint (weak-reachability + greatest-fixpoint prune)",
    ))
}

fn conformant_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let states = state_index(problem);
    let groups = action_groups(problem);
    let start = problem
        .initial_states
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut queue = VecDeque::from([(start.clone(), Vec::<String>::new())]);
    let mut seen = BTreeSet::from([start]);
    while let Some((belief, actions)) = queue.pop_front() {
        if belief
            .iter()
            .all(|id| problem.goal.holds(states[id.as_str()]))
        {
            return Ok(UniversalPlan {
                solved: true,
                steps: actions
                    .into_iter()
                    .map(|action| PlanStep {
                        action,
                        ..PlanStep::default()
                    })
                    .collect(),
                notes: vec!["belief-state breadth-first search".to_owned()],
                ..UniversalPlan::default()
            });
        }
        if actions.len() >= limits.max_depth || seen.len() > limits.max_states {
            continue;
        }
        let candidates = groups
            .keys()
            .filter(|(state, _)| belief.contains(*state))
            .map(|(_, action)| (*action).to_owned())
            .collect::<BTreeSet<_>>();
        for action in candidates {
            let mut successor = BTreeSet::new();
            let mut applicable = true;
            for state in &belief {
                if let Some(outcomes) = groups.get(&(state.as_str(), action.as_str())) {
                    successor.extend(outcomes.iter().map(|edge| edge.to.clone()));
                } else {
                    applicable = false;
                    break;
                }
            }
            if applicable
                && successor
                    .iter()
                    .all(|state| !problem.unsafe_states.contains(state))
                && seen.insert(successor.clone())
            {
                let mut next_actions = actions.clone();
                next_actions.push(action);
                queue.push_back((successor, next_actions));
            }
        }
    }
    Err(PlannerError::NoPlan)
}

fn contingent_policy(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let states = state_index(problem);
    let groups = action_groups(problem);
    let start = problem
        .initial_states
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut memo = BTreeMap::<BTreeSet<String>, Option<Vec<PolicyEntry>>>::new();
    fn solve_belief(
        belief: &BTreeSet<String>,
        depth: usize,
        problem: &PlanningProblem,
        states: &BTreeMap<&str, &State>,
        groups: &BTreeMap<(&str, &str), Vec<&Transition>>,
        limits: &PlannerLimits,
        memo: &mut BTreeMap<BTreeSet<String>, Option<Vec<PolicyEntry>>>,
    ) -> Option<Vec<PolicyEntry>> {
        if belief
            .iter()
            .all(|state| problem.goal.holds(states[state.as_str()]))
        {
            return Some(Vec::new());
        }
        if depth >= limits.max_depth || memo.len() >= limits.max_states {
            return None;
        }
        if let Some(cached) = memo.get(belief) {
            return cached.clone();
        }
        memo.insert(belief.clone(), None);
        let actions = groups
            .keys()
            .filter(|(state, _)| belief.contains(*state))
            .map(|(_, action)| (*action).to_owned())
            .collect::<BTreeSet<_>>();
        for action in actions {
            let mut branches = BTreeMap::<String, BTreeSet<String>>::new();
            let mut applicable = true;
            for state in belief {
                let Some(outcomes) = groups.get(&(state.as_str(), action.as_str())) else {
                    applicable = false;
                    break;
                };
                for edge in outcomes {
                    let observation = edge.observation.clone().unwrap_or_else(|| edge.to.clone());
                    branches
                        .entry(observation)
                        .or_default()
                        .insert(edge.to.clone());
                }
            }
            if !applicable
                || branches
                    .values()
                    .flatten()
                    .any(|state| problem.unsafe_states.contains(state))
            {
                continue;
            }
            let mut combined = Vec::new();
            let mut all_solved = true;
            for branch in branches.values() {
                if let Some(mut child) =
                    solve_belief(branch, depth + 1, problem, states, groups, limits, memo)
                {
                    combined.append(&mut child);
                } else {
                    all_solved = false;
                    break;
                }
            }
            if all_solved {
                combined.push(PolicyEntry {
                    state: belief.iter().cloned().collect::<Vec<_>>().join("|"),
                    action,
                    outcomes: branches
                        .into_iter()
                        .map(|(observation, states)| PolicyOutcome {
                            state: states.into_iter().collect::<Vec<_>>().join("|"),
                            probability_ppm: 0,
                            observation: Some(observation),
                        })
                        .collect(),
                });
                memo.insert(belief.clone(), Some(combined.clone()));
                return Some(combined);
            }
        }
        None
    }
    let policy = solve_belief(&start, 0, problem, &states, &groups, limits, &mut memo)
        .ok_or(PlannerError::NoPlan)?;
    Ok(UniversalPlan {
        solved: true,
        policy,
        notes: vec!["AND-OR contingent belief policy".to_owned()],
        ..UniversalPlan::default()
    })
}

fn task_map(problem: &PlanningProblem) -> BTreeMap<&str, &Task> {
    problem
        .tasks
        .iter()
        .map(|task| (task.id.as_str(), task))
        .collect()
}

fn hierarchical_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let tasks = task_map(problem);
    let mut methods = BTreeMap::<&str, Vec<&Method>>::new();
    for method in &problem.methods {
        methods
            .entry(method.task.as_str())
            .or_default()
            .push(method);
    }
    fn expand(
        task_id: &str,
        tasks: &BTreeMap<&str, &Task>,
        methods: &BTreeMap<&str, Vec<&Method>>,
        stack: &mut BTreeSet<String>,
        output: &mut Vec<String>,
        depth: usize,
        limits: &PlannerLimits,
    ) -> Result<(), PlannerError> {
        if depth > limits.max_depth || output.len() > limits.max_states {
            return Err(PlannerError::ResourceBound {
                resource: "hierarchy".to_owned(),
                limit: limits.max_states,
            });
        }
        let task = tasks
            .get(task_id)
            .ok_or_else(|| PlannerError::UnknownTask {
                task: task_id.to_owned(),
            })?;
        if let Some(action) = &task.primitive_action {
            output.push(action.clone());
            return Ok(());
        }
        if !stack.insert(task_id.to_owned()) {
            return Err(PlannerError::HierarchyCycle {
                task: task_id.to_owned(),
            });
        }
        let method = methods
            .get(task_id)
            .and_then(|candidates| candidates.first())
            .ok_or_else(|| PlannerError::NoMethod {
                task: task_id.to_owned(),
            })?;
        for subtask in &method.subtasks {
            expand(subtask, tasks, methods, stack, output, depth + 1, limits)?;
        }
        stack.remove(task_id);
        Ok(())
    }
    let mut actions = Vec::new();
    let mut stack = BTreeSet::new();
    for root in &problem.root_tasks {
        expand(root, &tasks, &methods, &mut stack, &mut actions, 0, limits)?;
    }
    Ok(UniversalPlan {
        solved: true,
        decomposition: actions.clone(),
        steps: actions
            .into_iter()
            .map(|action| PlanStep {
                action,
                ..PlanStep::default()
            })
            .collect(),
        ..UniversalPlan::default()
    })
}

fn workflow_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let tasks = task_map(problem);
    let mut indegree = problem
        .tasks
        .iter()
        .map(|task| (task.id.clone(), 0_usize))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = BTreeMap::<String, Vec<String>>::new();
    for edge in &problem.workflow_edges {
        if !tasks.contains_key(edge.before.as_str()) {
            return Err(PlannerError::UnknownTask {
                task: edge.before.clone(),
            });
        }
        if !tasks.contains_key(edge.after.as_str()) {
            return Err(PlannerError::UnknownTask {
                task: edge.after.clone(),
            });
        }
        *indegree.entry(edge.after.clone()).or_default() += 1;
        outgoing
            .entry(edge.before.clone())
            .or_default()
            .push(edge.after.clone());
    }
    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(task, _)| task.clone())
        .collect::<BTreeSet<_>>();
    let mut order = Vec::new();
    while let Some(task) = ready.pop_first() {
        order.push(task.clone());
        if order.len() > limits.max_states {
            return Err(PlannerError::ResourceBound {
                resource: "workflow".to_owned(),
                limit: limits.max_states,
            });
        }
        for child in outgoing.get(&task).into_iter().flatten() {
            let degree = indegree.get_mut(child).expect("workflow child exists");
            *degree -= 1;
            if *degree == 0 {
                ready.insert(child.clone());
            }
        }
    }
    if order.len() != problem.tasks.len() {
        return Err(PlannerError::WorkflowCycle);
    }
    let steps = order
        .iter()
        .filter_map(|task| tasks[task.as_str()].primitive_action.clone())
        .map(|action| PlanStep {
            action,
            ..PlanStep::default()
        })
        .collect();
    Ok(UniversalPlan {
        solved: true,
        steps,
        decomposition: order,
        notes: vec!["stable topological order".to_owned()],
        ..UniversalPlan::default()
    })
}

fn flow_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    for queue in &problem.queues {
        if queue.current_wip >= queue.max_wip {
            return Err(PlannerError::WipBoundExceeded {
                queue: queue.id.clone(),
                current: queue.current_wip,
                max: queue.max_wip,
            });
        }
    }
    let mut plan = shortest_path(problem, Metric::Cost, limits)?;
    plan.notes
        .push("Little-law admission bounds satisfied".to_owned());
    Ok(plan)
}

fn resolution_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let mut plan = hierarchical_plan(problem, limits)?;
    plan.notes
        .push("expanded until every leaf was primitive".to_owned());
    Ok(plan)
}

fn assign_agents(
    steps: &mut [PlanStep],
    requirements: &BTreeMap<String, BTreeSet<String>>,
    agents: &[Agent],
) -> Result<(), PlannerError> {
    let mut loads = agents
        .iter()
        .map(|agent| (agent.id.clone(), agent.current_wip))
        .collect::<BTreeMap<_, _>>();
    for step in steps {
        let required = requirements.get(&step.action).cloned().unwrap_or_default();
        let candidate = agents
            .iter()
            .filter(|agent| {
                required.is_subset(&agent.capabilities)
                    && loads.get(&agent.id).copied().unwrap_or_default() < agent.capacity
            })
            .min_by_key(|agent| (loads.get(&agent.id).copied().unwrap_or_default(), &agent.id));
        let Some(agent) = candidate else {
            return Err(PlannerError::CapabilityUncovered {
                item: step.action.clone(),
                missing: required,
            });
        };
        *loads.entry(agent.id.clone()).or_default() += 1;
        step.agent = Some(agent.id.clone());
    }
    Ok(())
}

fn action_requirements(problem: &PlanningProblem) -> BTreeMap<String, BTreeSet<String>> {
    let mut requirements = BTreeMap::new();
    for transition in &problem.transitions {
        requirements
            .entry(transition.action.clone())
            .or_insert_with(|| transition.requires.clone());
    }
    for task in &problem.tasks {
        if let Some(action) = &task.primitive_action {
            requirements
                .entry(action.clone())
                .or_insert_with(|| task.requires.clone());
        }
    }
    requirements
}

fn multi_agent_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let mut plan = shortest_path(problem, Metric::Cost, limits)?;
    assign_agents(
        &mut plan.steps,
        &action_requirements(problem),
        &problem.agents,
    )?;
    plan.notes
        .push("capacity-aware multi-agent assignment".to_owned());
    Ok(plan)
}

fn delegated_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let mut plan = if problem.root_tasks.is_empty() {
        shortest_path(problem, Metric::Cost, limits)?
    } else {
        hierarchical_plan(problem, limits)?
    };
    assign_agents(
        &mut plan.steps,
        &action_requirements(problem),
        &problem.agents,
    )?;
    plan.notes.push("A2A capability delegation".to_owned());
    Ok(plan)
}

fn mcp_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let mut plan = if problem.root_tasks.is_empty() {
        shortest_path(problem, Metric::Cost, limits)?
    } else {
        hierarchical_plan(problem, limits)?
    };
    let requirements = action_requirements(problem);
    for step in &mut plan.steps {
        let required = requirements.get(&step.action).cloned().unwrap_or_default();
        let Some(tool) = problem
            .tools
            .iter()
            .filter(|tool| required.is_subset(&tool.capabilities))
            .min_by_key(|tool| &tool.id)
        else {
            return Err(PlannerError::CapabilityUncovered {
                item: step.action.clone(),
                missing: required,
            });
        };
        if !tool.authority_bound {
            return Err(PlannerError::AuthorityUnbound {
                tool: tool.id.clone(),
            });
        }
        if !tool.verifier_bound {
            return Err(PlannerError::VerifierUnbound {
                tool: tool.id.clone(),
            });
        }
        if !tool.receipt_bound {
            return Err(PlannerError::ReceiptUnbound {
                tool: tool.id.clone(),
            });
        }
        step.tool = Some(tool.id.clone());
    }
    plan.notes
        .push("MCP primitive capability binding".to_owned());
    Ok(plan)
}

fn rdf_plan(
    problem: &PlanningProblem,
    limits: &PlannerLimits,
) -> Result<UniversalPlan, PlannerError> {
    let mut projected = problem.clone();
    if projected.rdf.is_empty() {
        return Err(PlannerError::InvalidRdfProjection {
            reason: "empty graph".to_owned(),
        });
    }
    let mut records = BTreeMap::<String, BTreeMap<String, Vec<String>>>::new();
    for triple in &projected.rdf {
        records
            .entry(triple.subject.clone())
            .or_default()
            .entry(triple.predicate.clone())
            .or_default()
            .push(triple.object.clone());
    }
    let mut state_ids = BTreeSet::new();
    for (subject, predicates) in &records {
        if predicates.contains_key("state") {
            state_ids.insert(subject.clone());
        }
        if predicates.contains_key("initial") {
            projected.initial_states.push(subject.clone());
            state_ids.insert(subject.clone());
        }
        if predicates.contains_key("goal") {
            projected.goal.facts.insert(format!("goal:{subject}"));
            state_ids.insert(subject.clone());
        }
    }
    for state in &state_ids {
        let mut facts = BTreeSet::new();
        if records[state].contains_key("goal") {
            facts.insert(format!("goal:{state}"));
        }
        projected.states.push(State {
            id: state.clone(),
            facts,
            fluents: BTreeMap::new(),
        });
    }
    for (subject, predicates) in &records {
        let Some(from) = predicates.get("from").and_then(|v| v.first()) else {
            continue;
        };
        let Some(to) = predicates.get("to").and_then(|v| v.first()) else {
            continue;
        };
        let action = predicates
            .get("action")
            .and_then(|v| v.first())
            .cloned()
            .unwrap_or_else(|| subject.clone());
        projected.transitions.push(Transition {
            action,
            from: from.clone(),
            to: to.clone(),
            cost: 1,
            duration: 1,
            reward: 0,
            probability_ppm: probability_one(),
            observation: None,
            requires: BTreeSet::new(),
        });
    }
    projected.rdf.clear();
    // The source problem may already carry projections; retain only one state per ID.
    let mut unique = BTreeMap::new();
    for state in projected.states {
        unique.insert(state.id.clone(), state);
    }
    projected.states = unique.into_values().collect();
    projected.initial_states.sort();
    projected.initial_states.dedup();
    validate_problem(&projected)?;
    let mut plan = shortest_path(&projected, Metric::Cost, limits)?;
    plan.notes
        .push("RDF graph projected into bounded state space".to_owned());
    Ok(plan)
}

#[cfg(test)]
mod tests {
    //! Unit tests for the two private FOND fixpoint solvers. These call
    //! `fond_policy` / `fond_policy_strong_cyclic` directly (both are
    //! module-private, so only reachable from an in-module test, unlike
    //! `crates/ferroplan/tests/planning_runtime.rs`'s integration tests,
    //! which can only exercise them indirectly through the public
    //! `solve_planning_type`).
    use super::*;

    fn edge(action: &str, from: &str, to: &str, probability_ppm: u32) -> Transition {
        Transition {
            action: action.to_owned(),
            from: from.to_owned(),
            to: to.to_owned(),
            cost: 1,
            duration: 1,
            reward: 0,
            probability_ppm,
            observation: None,
            requires: BTreeSet::new(),
        }
    }

    fn state(id: &str, facts: &[&str]) -> State {
        State {
            id: id.to_owned(),
            facts: facts.iter().map(|fact| (*fact).to_owned()).collect(),
            fluents: BTreeMap::new(),
        }
    }

    /// Deterministic (backdated `start`, not a real slow operation) check
    /// that `check_wall_deadline` fires once elapsed exceeds `max_wall_ms`.
    /// Mirrors the same-named test in `ferroplan_hddl::grounder`/`::translate`.
    #[test]
    fn check_wall_deadline_times_out_once_elapsed_exceeds_max_wall_ms() {
        let limits = PlannerLimits {
            max_wall_ms: 10,
            ..PlannerLimits::default()
        };
        let backdated_start = Instant::now() - std::time::Duration::from_millis(50);
        let err = check_wall_deadline(backdated_start, &limits).unwrap_err();
        match err {
            PlannerError::Timeout {
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
    fn check_wall_deadline_never_fires_when_max_wall_ms_is_zero() {
        let limits = PlannerLimits {
            max_wall_ms: 0,
            ..PlannerLimits::default()
        };
        let ancient_start = Instant::now() - std::time::Duration::from_secs(3600);
        assert!(check_wall_deadline(ancient_start, &limits).is_ok());
    }

    // `fond_policy`'s own loop calls `check_wall_deadline` with a real
    // `Instant::now()` at entry (not a backdated one, unlike the direct
    // unit tests above) — an end-to-end timing-based assertion on that
    // would be flaky (this fixture's fixpoint gives up after its first
    // non-converging round regardless, in well under a millisecond, so a
    // tiny `max_wall_ms` would almost never actually race it). The
    // deterministic tests above already prove the exact mechanism
    // `fond_policy`/`fond_policy_strong_cyclic` call; this comment records
    // that omission as a deliberate scope boundary, not an oversight —
    // see `~/.claude/rules/testing-chicago-style.md` on preferring a real,
    // deterministic check over a timing-dependent one when both are
    // available.

    /// The committed retry-loop fixture: a single non-deterministic action
    /// `flip` from `s0` either reaches the goal (`g`) or loops back onto
    /// `s0` itself. No acyclic strong policy exists (the `s0 -> s0` outcome
    /// can never be `winning` ahead of `s0` itself), but a strong-cyclic
    /// policy trivially does (`{s0: "flip"}`).
    fn retry_loop_problem() -> PlanningProblem {
        PlanningProblem {
            states: vec![state("s0", &[]), state("g", &["done"])],
            initial_states: vec!["s0".to_owned()],
            goal: Goal {
                facts: BTreeSet::from(["done".to_owned()]),
                ..Goal::default()
            },
            transitions: vec![
                edge("flip", "s0", "g", 500_000),
                edge("flip", "s0", "s0", 500_000),
            ],
            ..PlanningProblem::default()
        }
    }

    /// Direct, function-level pin: `fond_policy` alone -- called on its own,
    /// not through `solve_planning_type`'s fallback -- still correctly
    /// returns `NoPlan` on a domain that structurally requires a
    /// strong-cyclic retry policy. This is the acyclic-only fixpoint's own
    /// contract and must not regress just because a fallback now sits in
    /// front of it at the dispatch layer.
    #[test]
    fn fond_policy_alone_still_returns_no_plan_on_retry_loop() {
        let problem = retry_loop_problem();
        let limits = PlannerLimits::default();
        assert_eq!(
            fond_policy(&problem, &limits),
            Err(PlannerError::NoPlan),
            "fond_policy's acyclic-only fixpoint must still reject a domain \
             that requires a retry loop when called directly"
        );
    }

    /// The direct positive counterpart: `fond_policy_strong_cyclic` alone
    /// solves the same fixture `fond_policy` cannot, with the exact policy
    /// traced in the design (`{s0: "flip"}`, both outcomes preserved).
    #[test]
    fn fond_policy_strong_cyclic_solves_the_retry_loop_domain() {
        let problem = retry_loop_problem();
        let limits = PlannerLimits::default();
        let plan = fond_policy_strong_cyclic(&problem, &limits)
            .expect("strong-cyclic fixpoint solves the retry-loop domain");
        assert!(plan.solved);
        assert_eq!(plan.policy.len(), 1);
        let entry = &plan.policy[0];
        assert_eq!(entry.state, "s0");
        assert_eq!(entry.action, "flip");
        let mut outcomes = entry
            .outcomes
            .iter()
            .map(|outcome| outcome.state.clone())
            .collect::<Vec<_>>();
        outcomes.sort();
        assert_eq!(outcomes, vec!["g".to_owned(), "s0".to_owned()]);
    }

    /// A slightly larger strong-cyclic fixture with two independent retry
    /// points chained together: `s0` must retry `flip1` until it advances to
    /// `a`, then `a` must independently retry `flip2` until it advances to
    /// the goal `g`. Neither retry point is expressible by `fond_policy`'s
    /// acyclic fixpoint (each has a same-state self-loop outcome), so this
    /// exercises Phase 1/Phase 2 propagating admission across two hops, not
    /// just a single self-loop.
    fn two_retry_points_problem() -> PlanningProblem {
        PlanningProblem {
            states: vec![state("s0", &[]), state("a", &[]), state("g", &["done"])],
            initial_states: vec!["s0".to_owned()],
            goal: Goal {
                facts: BTreeSet::from(["done".to_owned()]),
                ..Goal::default()
            },
            transitions: vec![
                edge("flip1", "s0", "a", 500_000),
                edge("flip1", "s0", "s0", 500_000),
                edge("flip2", "a", "g", 500_000),
                edge("flip2", "a", "a", 500_000),
            ],
            ..PlanningProblem::default()
        }
    }

    #[test]
    fn fond_policy_strong_cyclic_solves_two_independent_retry_points() {
        let problem = two_retry_points_problem();
        let limits = PlannerLimits::default();
        // Confirm the acyclic-only solver still cannot express this domain
        // either, for the same structural reason as the single-loop case.
        assert_eq!(
            fond_policy(&problem, &limits),
            Err(PlannerError::NoPlan),
            "fond_policy must still reject the chained two-retry-point domain"
        );
        let plan = fond_policy_strong_cyclic(&problem, &limits)
            .expect("strong-cyclic fixpoint solves the chained two-retry-point domain");
        assert!(plan.solved);
        assert_eq!(plan.policy.len(), 2);
        let by_state = plan
            .policy
            .iter()
            .map(|entry| (entry.state.as_str(), entry))
            .collect::<BTreeMap<_, _>>();
        let s0_entry = by_state.get("s0").expect("s0 has a policy entry");
        assert_eq!(s0_entry.action, "flip1");
        let mut s0_outcomes = s0_entry
            .outcomes
            .iter()
            .map(|outcome| outcome.state.clone())
            .collect::<Vec<_>>();
        s0_outcomes.sort();
        assert_eq!(s0_outcomes, vec!["a".to_owned(), "s0".to_owned()]);
        let a_entry = by_state.get("a").expect("a has a policy entry");
        assert_eq!(a_entry.action, "flip2");
        let mut a_outcomes = a_entry
            .outcomes
            .iter()
            .map(|outcome| outcome.state.clone())
            .collect::<Vec<_>>();
        a_outcomes.sort();
        assert_eq!(a_outcomes, vec!["a".to_owned(), "g".to_owned()]);
    }

    /// End-to-end proof (within the same module, over the private
    /// functions) that the fallback wiring in `solve_planning_type` does not
    /// disturb a domain `fond_policy` already solves outright: the strong
    /// (acyclic) FOND fixture used by the integration test suite must still
    /// resolve via `fond_policy` alone, never reaching the strong-cyclic
    /// fallback.
    #[test]
    fn fond_policy_still_solves_acyclic_strong_domains_directly() {
        let problem = PlanningProblem {
            states: vec![
                state("s0", &[]),
                state("g1", &["done"]),
                state("g2", &["done"]),
            ],
            initial_states: vec!["s0".to_owned()],
            goal: Goal {
                facts: BTreeSet::from(["done".to_owned()]),
                ..Goal::default()
            },
            transitions: vec![
                edge("commit", "s0", "g1", 500_000),
                edge("commit", "s0", "g2", 500_000),
            ],
            ..PlanningProblem::default()
        };
        let plan = fond_policy(&problem, &PlannerLimits::default())
            .expect("fond_policy solves the acyclic strong domain on its own");
        assert!(plan.solved);
        assert_eq!(plan.notes, vec!["strong FOND fixed point".to_owned()]);
    }

    /// The wave-1 audit's truncation dead-sink reproducer (4 states): `a`
    /// leaves `s0` for `x`; `b` from `x` either reaches the goal `g` or the
    /// dead-end `z` (non-goal, choice-less, never weakly goal-reachable).
    /// No strong-cyclic policy exists: every execution of `b` can fall into
    /// `z`, from which the goal is unreachable.
    ///
    /// Phase 2's first prune round validates `s0` against `x` and then
    /// prunes `x` later in that same round; only a further revalidation
    /// round prunes `s0`. A loop truncated between those two rounds
    /// returned the bogus policy `{s0: a}` — its sole step leads into the
    /// pruned, choice-less, non-goal dead sink, a shape the old post-loop
    /// checks (initials surviving + has-some-choice) could not see because
    /// they never examined outcome closure. Post-fix the loop runs to its
    /// fixpoint (`max_iterations` no longer gates it) and the outcome-
    /// closure post-check rejects that shape unconditionally: round 2
    /// revalidates `s0`, finds `x` pruned, prunes `s0`, and the solver
    /// must return `Err(NoPlan)`.
    fn dead_sink_reproducer_problem() -> PlanningProblem {
        PlanningProblem {
            states: vec![
                state("s0", &[]),
                state("x", &[]),
                state("z", &[]),
                state("g", &["done"]),
            ],
            initial_states: vec!["s0".to_owned()],
            goal: Goal {
                facts: BTreeSet::from(["done".to_owned()]),
                ..Goal::default()
            },
            transitions: vec![
                edge("a", "s0", "x", 1_000_000),
                edge("b", "x", "g", 500_000),
                edge("b", "x", "z", 500_000),
            ],
            ..PlanningProblem::default()
        }
    }

    #[test]
    fn fond_policy_strong_cyclic_rejects_the_truncation_dead_sink_reproducer() {
        let problem = dead_sink_reproducer_problem();
        // Pre-fix, `max_iterations: 1` truncated the fixpoints mid-flight
        // (Phase 1 admitted only `x` in its single round); post-fix the
        // cap no longer gates either loop — the chain of rounds must run
        // to its fixpoint, where `s0` is admitted, validated in round 1,
        // and pruned in round 2 — so only `Err(NoPlan)` is admissible.
        let limits = PlannerLimits {
            max_iterations: 1,
            ..PlannerLimits::default()
        };
        assert_eq!(
            fond_policy_strong_cyclic(&problem, &limits),
            Err(PlannerError::NoPlan),
            "s0 is validated in round 1 and must be pruned in round 2 once x \
             dies; returning any policy here means a stale witness pointing \
             into the pruned dead sink survived"
        );
    }

    /// A chain of `CHAIN_LEN` non-goal states `c0 -> c1 -> ... -> g`, one
    /// deterministic action per hop, laid out in the state vector in chain
    /// order so Phase 1's backward admission propagates exactly one hop per
    /// round: full convergence needs `CHAIN_LEN` changing rounds plus one
    /// confirming round — far more than the `max_iterations: 1` passed
    /// here. Pre-fix, that cap truncated Phase 1 after its first round
    /// (only `cCHAIN_LEN-1` admitted), so the initial state could never
    /// enter `surviving` and a solvable domain was rejected with a bogus
    /// `NoPlan`. Post-fix, `max_iterations` no longer gates either fixpoint
    /// — the loop runs `while changed` under the `states + 1` round
    /// failsafe (never reached: each changing round admits at least one
    /// state) — so the chain must still be solved completely.
    #[test]
    fn fond_policy_strong_cyclic_converges_past_max_iterations_on_a_chain() {
        const CHAIN_LEN: usize = 12;
        let ids = (0..CHAIN_LEN)
            .map(|index| format!("c{index}"))
            .collect::<Vec<_>>();
        let problem = PlanningProblem {
            states: ids
                .iter()
                .map(|id| state(id, &[]))
                .chain(std::iter::once(state("g", &["done"])))
                .collect(),
            initial_states: vec!["c0".to_owned()],
            goal: Goal {
                facts: BTreeSet::from(["done".to_owned()]),
                ..Goal::default()
            },
            transitions: (0..CHAIN_LEN)
                .map(|index| {
                    let to = if index + 1 < CHAIN_LEN {
                        format!("c{}", index + 1)
                    } else {
                        "g".to_owned()
                    };
                    edge(&format!("step_{index}"), &ids[index], &to, 1_000_000)
                })
                .collect(),
            ..PlanningProblem::default()
        };
        let limits = PlannerLimits {
            max_iterations: 1,
            ..PlannerLimits::default()
        };
        let plan = fond_policy_strong_cyclic(&problem, &limits).expect(
            "the chain fixpoint needs one round per hop, so solving it under \
             max_iterations: 1 proves the cap no longer gates the loop",
        );
        assert!(plan.solved);
        assert_eq!(plan.policy.len(), CHAIN_LEN);
        let by_state = plan
            .policy
            .iter()
            .map(|entry| (entry.state.as_str(), entry))
            .collect::<BTreeMap<_, _>>();
        for (index, id) in ids.iter().enumerate() {
            let entry = by_state
                .get(id.as_str())
                .expect("every chain state gets a policy entry");
            assert_eq!(
                entry.action,
                format!("step_{index}"),
                "state {id} must choose its own chain action"
            );
            let expected_to = if index + 1 < CHAIN_LEN {
                format!("c{}", index + 1)
            } else {
                "g".to_owned()
            };
            assert_eq!(entry.outcomes.len(), 1);
            assert_eq!(entry.outcomes[0].state, expected_to);
        }
    }
}
