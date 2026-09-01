//! Solve off-thread and animate only bounded, independently validated candidate plans.
//! The solver runs dark, off the main thread, while the timeline waits on a
//! keystroke. Mobiles ghost between snapshots — the shadow of one node bleeding
//! into the next — tweened frame by frame across the recorded trace.
//!
//! Controls: **S** wake the solver · **Space** run/hold · **←/→** step the tape · **R** wipe.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use futures_lite::future;

use ferroplan::{
    Mode, Options, OutcomeClass, ProductionLimits, StateSnapshot, Step, ValidationStatus,
};

use crate::scene::{FanOffset, MobileObj, NodeObj, Scene};

pub(crate) struct SolveResult {
    steps: Vec<Step>,
    snapshots: Vec<StateSnapshot>,
    status: String,
    temporal: bool,
    makespan: f32,
}

#[derive(Resource, Default)]
pub struct Plan {
    pub steps: Vec<Step>,
    pub snapshots: Vec<StateSnapshot>,
    /// The needle. Classic runs count it in steps, `0..=steps.len()`; temporal
    /// runs count it in seconds of plan-time, `0..=makespan`, synced to the
    /// Gantt axis so both views read the same clock.
    pub t: f32,
    pub playing: bool,
    pub status: String,
    /// Marks a temporal run — overlapping durative actions. The graph stops
    /// tweening between snapshots and the timescale readout takes over.
    pub temporal: bool,
    pub makespan: f32,
}

impl Plan {
    /// Where the needle runs out — makespan for a temporal job, step count otherwise.
    pub fn span(&self) -> f32 {
        if self.temporal {
            self.makespan.max(1e-3)
        } else {
            (self.steps.len().max(1)) as f32
        }
    }

    /// The needle's read, `0..=1` — feeds the fill bar and the playhead.
    pub fn frac(&self) -> f32 {
        (self.t / self.span()).clamp(0.0, 1.0)
    }

    /// Where an action ignites, as a fraction of the run — the coordinate the
    /// transport notches and Gantt bars key off of.
    pub fn start_frac(&self, step: &Step, idx: usize) -> f32 {
        let value = if self.temporal {
            step.time.unwrap_or(0.0) as f32
        } else {
            idx as f32
        };
        (value / self.span()).clamp(0.0, 1.0)
    }
}

#[derive(Resource, Default)]
pub struct SolveJob(Option<Task<SolveResult>>);

pub fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    scene: Res<Scene>,
    editor: Res<crate::blocks::Editor>,
    mut plan: ResMut<Plan>,
    mut job: ResMut<SolveJob>,
) {
    if editor.focus.is_some() {
        return;
    }
    if keys.just_pressed(KeyCode::KeyS)
        && job.0.is_none()
        && !scene.domain_src.is_empty()
        && !scene.problem_src.is_empty()
    {
        let limits = ProductionLimits::default();
        if scene.domain_src.len() > limits.max_domain_bytes
            || scene.problem_src.len() > limits.max_problem_bytes
        {
            plan.status = "refused: model exceeds the bounded GUI planning profile".into();
        } else {
            let domain = scene.domain_src.clone();
            let problem = scene.problem_src.clone();
            job.0 = Some(
                AsyncComputeTaskPool::get().spawn(async move { solve_blocking(domain, problem) }),
            );
            plan.status = "solving bounded candidate…".into();
        }
    }
    let span = plan.span();
    if keys.just_pressed(KeyCode::Space) && !plan.steps.is_empty() {
        if plan.t >= span {
            plan.t = 0.0;
        }
        plan.playing = !plan.playing;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        plan.t = next_mark(&plan, plan.t).min(span);
        plan.playing = false;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        plan.t = prev_mark(&plan, plan.t).max(0.0);
        plan.playing = false;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        plan.t = 0.0;
        plan.playing = false;
    }
}

/// The waypoints the step keys snap to — every action's ignition (and, on a
/// temporal run, its burnout), plus zero and the far edge. Classic runs fall
/// back to plain integer boundaries.
fn marks(plan: &Plan) -> Vec<f32> {
    if !plan.temporal {
        return (0..=plan.steps.len()).map(|index| index as f32).collect();
    }
    let mut marks = vec![0.0_f32, plan.span()];
    for step in &plan.steps {
        if let Some(time) = step.time {
            marks.push(time as f32);
            if let Some(duration) = step.duration {
                marks.push((time + duration) as f32);
            }
        }
    }
    marks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    marks.dedup();
    marks
}

fn next_mark(plan: &Plan, t: f32) -> f32 {
    marks(plan)
        .into_iter()
        .find(|mark| *mark > t + 1e-4)
        .unwrap_or_else(|| plan.span())
}

fn prev_mark(plan: &Plan, t: f32) -> f32 {
    marks(plan)
        .into_iter()
        .rev()
        .find(|mark| *mark < t - 1e-4)
        .unwrap_or(0.0)
}

fn solve_blocking(domain: String, problem: String) -> SolveResult {
    let limits = ProductionLimits::default();
    let options = Options {
        threads: 1,
        max_evaluated: Some(limits.max_evaluated),
        ..Options::default()
    };
    let envelope = ferroplan::solve_production(
        &domain,
        &problem,
        &options,
        &limits,
        Some("bevy-interactive-solve"),
    );
    match (envelope.outcome, envelope.validation, envelope.payload) {
        (OutcomeClass::Solved, ValidationStatus::Valid, Some(solution)) => {
            let mut result = result_from_solution(&domain, &problem, solution);
            result
                .status
                .push_str(" · candidate-only · independently validated");
            result
        }
        (OutcomeClass::Solved, ValidationStatus::NotApplicable, Some(solution)) => {
            let mut result = result_from_solution(&domain, &problem, solution);
            result
                .status
                .push_str(" · candidate-only · empty-plan goal closure");
            result
        }
        (OutcomeClass::NoPlan, _, _) => empty_result("no plan found within the declared model"),
        (OutcomeClass::LimitExceeded, _, _) => empty_result(&format!(
            "bounded solve saturated: {}",
            envelope
                .error
                .as_ref()
                .map(|error| error.message.as_str())
                .unwrap_or("resource limit exceeded")
        )),
        (OutcomeClass::Refused | OutcomeClass::Failed | OutcomeClass::Solved, _, _) => {
            empty_result(&format!(
                "refused: {}",
                envelope
                    .error
                    .as_ref()
                    .map(|error| error.message.as_str())
                    .unwrap_or("candidate failed production validation")
            ))
        }
    }
}

fn empty_result(status: &str) -> SolveResult {
    SolveResult {
        steps: vec![],
        snapshots: vec![],
        status: status.into(),
        temporal: false,
        makespan: 0.0,
    }
}

/// Assemble the animator's [`SolveResult`] — steps plus replayed snapshots — from
/// a [`ferroplan::Solution`] already pulled off the wire. Two callers converge
/// here: the native `S`-key job, and the web Solver page's handoff (`webhandoff`),
/// which arrives pre-solved and asks only to be replayed, not recomputed.
pub(crate) fn result_from_solution(
    domain: &str,
    problem: &str,
    solution: ferroplan::Solution,
) -> SolveResult {
    match solution.plan {
        Some(plan) => {
            let pairs: Vec<(String, Vec<String>)> = plan
                .steps
                .iter()
                .map(|step| (step.action.clone(), step.args.clone()))
                .collect();
            let temporal = solution.mode == Mode::Temporal;
            let snapshots = if temporal {
                Vec::new()
            } else {
                match ferroplan::trace(domain, problem, &pairs) {
                    Ok(snapshots) => snapshots,
                    Err(error) => {
                        return empty_result(&format!(
                            "refused: candidate trace replay failed ({error})"
                        ))
                    }
                }
            };
            let makespan = plan.makespan.unwrap_or(0.0) as f32;
            let mut status = format!("candidate: {} steps", plan.steps.len());
            if let Some(metric) = plan.metric {
                status.push_str(&format!(", metric {metric}"));
            }
            if temporal {
                status.push_str(&format!(" (temporal makespan {makespan:.2})"));
            }
            SolveResult {
                steps: plan.steps,
                snapshots,
                status,
                temporal,
                makespan,
            }
        }
        None => empty_result("no candidate plan"),
    }
}

/// Drop a [`SolveResult`] straight onto the timeline, as if the solve had just
/// landed — called from `poll_solve` on native completion, and from the web
/// handoff when the Solver page already did the work. `autoplay` fires the tape
/// immediately: the signature of a user who clicked "Animate this plan."
pub(crate) fn load_result(plan: &mut Plan, result: SolveResult, autoplay: bool) {
    plan.steps = result.steps;
    plan.snapshots = result.snapshots;
    plan.status = result.status;
    plan.temporal = result.temporal;
    plan.makespan = result.makespan;
    plan.t = 0.0;
    plan.playing = autoplay && !plan.steps.is_empty();
}

pub fn poll_solve(mut job: ResMut<SolveJob>, mut plan: ResMut<Plan>) {
    if let Some(task) = job.0.as_mut() {
        if let Some(result) = block_on(future::poll_once(task)) {
            job.0 = None;
            load_result(&mut plan, result, false);
        }
    }
}

/// Idle speed for a classic run — unit-duration steps ticking by, per second.
const PLAY_RATE: f32 = 1.5;
/// A temporal run burns its whole makespan in about this many real seconds —
/// long horizons stay watchable, ratios between durations stay honest.
const TEMPORAL_SECONDS: f32 = 7.0;

pub fn advance(time: Res<Time>, mut plan: ResMut<Plan>) {
    if !plan.playing || plan.steps.is_empty() {
        return;
    }
    let span = plan.span();
    if plan.temporal {
        plan.t = (plan.t + time.delta_secs() * span / TEMPORAL_SECONDS).min(span);
    } else {
        let index = (plan.t.floor() as usize).min(plan.steps.len() - 1);
        let duration = plan.steps[index].duration.unwrap_or(1.0).max(0.05) as f32;
        plan.t = (plan.t + time.delta_secs() * PLAY_RATE / duration).min(span);
    }
    if plan.t >= span {
        plan.playing = false;
    }
}

/// Drag every mobile to its coordinate at time `t`, tweened between the node it
/// held in snapshot k and where it's headed in k+1.
pub fn animate(
    plan: Res<Plan>,
    scene: Res<Scene>,
    nodes: Query<(&NodeObj, &Transform)>,
    mut mobiles: Query<(&MobileObj, &FanOffset, &mut Transform), Without<NodeObj>>,
) {
    if plan.snapshots.is_empty() {
        return;
    }
    let count = plan.snapshots.len();
    let index = (plan.t.floor() as usize).min(count - 1);
    let next = (index + 1).min(count - 1);
    let fraction = if next == index {
        0.0
    } else {
        ease_in_out_cubic((plan.t - index as f32).clamp(0.0, 1.0))
    };
    let from = scene.graph.positions_at(&plan.snapshots[index].facts);
    let to = scene.graph.positions_at(&plan.snapshots[next].facts);
    let node_positions: HashMap<&str, Vec2> = nodes
        .iter()
        .map(|(node, transform)| (node.0.as_str(), transform.translation.truncate()))
        .collect();

    for (mobile, offset, mut transform) in &mut mobiles {
        let current = transform.translation.truncate() - offset.0;
        let from_position = node_pos(&from, &mobile.0, &node_positions).unwrap_or(current);
        let to_position = node_pos(&to, &mobile.0, &node_positions).unwrap_or(current);
        let target = from_position.lerp(to_position, fraction) + offset.0;
        transform.translation.x = target.x;
        transform.translation.y = target.y;
    }
}

/// The motion curve: a cold launch, a hard glide, then a dead stop — ease-in-out
/// cubic over `t` in `0..=1`.
fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

fn node_pos(
    map: &HashMap<String, Option<String>>,
    object: &str,
    positions: &HashMap<&str, Vec2>,
) -> Option<Vec2> {
    map.get(object)
        .and_then(|node| node.as_deref())
        .and_then(|node| positions.get(node).copied())
}
