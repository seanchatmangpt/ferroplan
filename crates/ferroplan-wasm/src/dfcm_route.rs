//! Target-independent DfCM repair router and counterfactual probe kernel.
//!
//! Both session surfaces (`wasi_abi::op_session_repair`/`op_session_probe`
//! for beam4pm and `WasmSession::repair`/`probe_json` for the browser) are
//! compile-time exclusive. Before this module each surface carried its own
//! copy of the router and the probe loop, so a revert on one surface (for
//! example swapping follow-before-rethink for a full replan) was invisible
//! to the other surface's tests. The logic now lives here once, compiled on
//! every target and tested natively; the surfaces only own their budget
//! checks, their handle/registry plumbing, and their error envelopes.
//!
//! Every function here manufactures candidates (plans, probe results). None
//! of them advances a cursor past a step or actuates anything.

use ferroplan::api::{Plan, Solution};
use ferroplan::Session;
use serde_json::{json, Value};

/// Maximum number of candidates in one probe request (both surfaces).
pub const MAX_PROBE_CANDIDATES: usize = 32;
/// Maximum number of observations per probe candidate (both surfaces).
pub const MAX_PROBE_OBSERVATIONS: usize = 1_024;
/// Maximum bytes of one observed fact or restriction filter.
pub const MAX_PROBE_FACT_BYTES: usize = 4_096;

/// One counterfactual candidate: an optional goal retarget, a bounded
/// observation set, and an optional action-surface restriction.
#[derive(serde::Deserialize, Clone, Debug)]
pub struct ProbeCandidate {
    pub id: String,
    pub goal: Option<String>,
    #[serde(default)]
    pub sight: Vec<(String, bool)>,
    pub restrict_contains: Option<String>,
}

/// A typed probe-admission refusal: `(code, message)`.
pub type Refusal = (&'static str, String);

/// Admission for a whole probe request. `goal_limit` is the surface's text
/// field limit; `surface` names the surface in the limit message.
pub fn admit_candidates(
    candidates: &[ProbeCandidate],
    goal_limit: usize,
    surface: &str,
) -> Result<(), Refusal> {
    if candidates.is_empty() || candidates.len() > MAX_PROBE_CANDIDATES {
        return Err((
            "FP_LIMIT_CANDIDATES",
            format!("candidates must contain between 1 and {MAX_PROBE_CANDIDATES} entries"),
        ));
    }
    if let Some(id) = crate::probe_guard::malformed_id(candidates.iter().map(|c| c.id.as_str())) {
        return Err((
            "FP_LIMIT_CANDIDATE",
            format!(
                "candidate id `{id}` must be 1..={} bytes",
                crate::probe_guard::MAX_PROBE_ID_BYTES
            ),
        ));
    }
    if let Some(id) = crate::probe_guard::duplicate_id(candidates.iter().map(|c| c.id.as_str())) {
        return Err((
            "FP_DUPLICATE_CANDIDATE",
            format!("candidate id `{id}` is delivered more than once"),
        ));
    }
    if candidates.iter().any(|candidate| {
        candidate
            .goal
            .as_ref()
            .is_some_and(|goal| goal.len() > goal_limit)
            || candidate.sight.len() > MAX_PROBE_OBSERVATIONS
            || candidate
                .sight
                .iter()
                .any(|(fact, _)| fact.len() > MAX_PROBE_FACT_BYTES)
            || candidate
                .restrict_contains
                .as_ref()
                .is_some_and(|filter| filter.len() > MAX_PROBE_FACT_BYTES)
    }) {
        return Err((
            "FP_LIMIT_CANDIDATE",
            format!(
                "candidate id, goal, observation, or restriction exceeds the {surface} probe limit"
            ),
        ));
    }
    Ok(())
}

/// Evaluate one candidate on a cheap fork of `parent`. The parent is never
/// mutated. `goal_met_before_search` is read before the bounded search runs
/// and `evals`/`mem_mb` are passed through to that search unchanged.
pub fn probe_candidate(
    parent: &Session,
    candidate: &ProbeCandidate,
    evals: usize,
    mem_mb: usize,
) -> Value {
    if let Some(fact) = crate::probe_guard::contradictory_fact(&candidate.sight) {
        return json!({
            "id": candidate.id,
            "outcome": "refused",
            "stage": "observe",
            "error": format!("contradictory observation of `{fact}`"),
        });
    }
    let mut mind = parent.fork();
    if let Some(goal) = &candidate.goal {
        if let Err(error) = mind.set_goal(goal) {
            return json!({
                "id": candidate.id,
                "outcome": "refused",
                "stage": "set_goal",
                "error": error,
            });
        }
    }
    if let Some(filter) = &candidate.restrict_contains {
        let filter = filter.clone();
        mind.restrict_ops(move |display| display.contains(&filter));
    }
    let surprises = if candidate.sight.is_empty() {
        Vec::new()
    } else {
        let refs = candidate
            .sight
            .iter()
            .map(|(fact, value)| (fact.as_str(), *value))
            .collect::<Vec<_>>();
        match mind.observe(&refs) {
            Ok(news) => news,
            Err(error) => {
                return json!({
                    "id": candidate.id,
                    "outcome": "refused",
                    "stage": "observe",
                    "error": error,
                })
            }
        }
    };
    let goal_met_before_search = mind.goal_met();
    let solution = mind.replan_budgeted(evals, Some(mem_mb));
    json!({
        "id": candidate.id,
        "outcome": if solution.solved { "solved" } else { "unsolved" },
        "surprises": surprises,
        "goal_met_before_search": goal_met_before_search,
        "world_bytes": mind.world_bytes(),
        "mind_bytes": mind.mind_bytes(),
        "solution": solution,
    })
}

/// Probe every (already admitted) candidate against `parent`.
pub fn probe_all(
    parent: &Session,
    candidates: &[ProbeCandidate],
    evals: usize,
    mem_mb: usize,
) -> Vec<Value> {
    candidates
        .iter()
        .map(|candidate| probe_candidate(parent, candidate, evals, mem_mb))
        .collect()
}

fn stash(plan: &mut Option<Plan>, cursor: &mut usize, sol: &Solution) -> Vec<ferroplan::api::Step> {
    *plan = if sol.solved { sol.plan.clone() } else { None };
    *cursor = 0;
    plan.as_ref()
        .map(|plan| plan.steps.clone())
        .unwrap_or_default()
}

/// DfCM repair router. Preserve the cheapest reversible option:
/// goal-met -> valid suffix reuse (zero search) -> follow-biased repair of
/// the broken tail -> full bounded replan when no plan is stashed. The
/// caller has already admitted `evals`/`mem_mb` against its budget.
pub fn repair(
    inner: &Session,
    plan: &mut Option<Plan>,
    cursor: &mut usize,
    evals: usize,
    mem_mb: usize,
) -> Value {
    if inner.goal_met() {
        return json!({
            "decision": "goal_met",
            "trigger": "goal_met",
            "plan_valid": Value::Null,
            "previous_suffix": [],
            "suffix": [],
            "solution": Value::Null,
        });
    }
    match plan.clone() {
        Some(prior) => {
            let from = (*cursor).min(prior.steps.len());
            let previous_suffix = prior.steps[from..].to_vec();
            if inner.plan_still_valid(&prior, *cursor) {
                return json!({
                    "decision": "reuse_suffix",
                    "trigger": "none",
                    "plan_valid": true,
                    "previous_suffix": previous_suffix,
                    "suffix": previous_suffix,
                    "solution": Value::Null,
                });
            }
            let sol = inner.replan_following(&prior, *cursor, evals, Some(mem_mb));
            let decision = if sol.solved {
                "replanned_following"
            } else {
                "replan_unsolved"
            };
            let suffix = stash(plan, cursor, &sol);
            json!({
                "decision": decision,
                "trigger": "invalid_plan",
                "plan_valid": false,
                "previous_suffix": previous_suffix,
                "suffix": suffix,
                "solution": sol,
            })
        }
        None => {
            let sol = inner.replan_budgeted(evals, Some(mem_mb));
            let decision = if sol.solved {
                "replanned_full"
            } else {
                "replan_unsolved"
            };
            let suffix = stash(plan, cursor, &sol);
            json!({
                "decision": decision,
                "trigger": "no_plan",
                "plan_valid": Value::Null,
                "previous_suffix": [],
                "suffix": suffix,
                "solution": sol,
            })
        }
    }
}

/// Deterministic benchmark/regression fixture shared by the WASI tests, the
/// browser parity tests, and this module's own tests: a corridor `r0..rn`
/// with a detour `ri -> xi -> yi -> r(i+2)` beside every main-line hop and a
/// `seal` action that no plan needs. Not part of any runtime surface.
#[doc(hidden)]
pub mod fixture {
    pub const CORRIDOR_DOMAIN: &str = r#"(define (domain corridor)
  (:requirements :strips :typing)
  (:types room)
  (:predicates (at ?r - room) (link ?a - room ?b - room) (clear ?r - room))
  (:action go
    :parameters (?a - room ?b - room)
    :precondition (and (at ?a) (link ?a ?b) (clear ?b))
    :effect (and (at ?b) (not (at ?a))))
  (:action seal
    :parameters (?r - room)
    :precondition (clear ?r)
    :effect (not (clear ?r))))"#;

    pub fn corridor_problem(n: usize) -> String {
        let mut objects = Vec::new();
        let mut init = vec!["(at r0)".to_string()];
        for i in 0..=n {
            objects.push(format!("r{i}"));
            init.push(format!("(clear r{i})"));
        }
        for i in 0..n {
            init.push(format!("(link r{i} r{})", i + 1));
            if i + 2 <= n {
                objects.push(format!("x{i}"));
                objects.push(format!("y{i}"));
                init.push(format!("(clear x{i})"));
                init.push(format!("(clear y{i})"));
                init.push(format!("(link r{i} x{i})"));
                init.push(format!("(link x{i} y{i})"));
                init.push(format!("(link y{i} r{})", i + 2));
            }
        }
        format!(
            "(define (problem corridor{n}) (:domain corridor) (:objects {} - room) (:init {}) (:goal (at r{n})))",
            objects.join(" "),
            init.join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::fixture::{corridor_problem, CORRIDOR_DOMAIN};
    use super::*;

    fn corridor(n: usize) -> Session {
        let opts = ferroplan::Options {
            threads: 1,
            max_evaluated: Some(ferroplan::ProductionLimits::default().max_evaluated),
            ..Default::default()
        };
        Session::new(CORRIDOR_DOMAIN, &corridor_problem(n), &opts).expect("corridor grounds")
    }

    fn evaluated(v: &Value) -> u64 {
        v["statistics"]["evaluated_states"]
            .as_u64()
            .unwrap_or_else(|| panic!("{v}"))
    }

    /// The deterministic court for follow-before-rethink. A revert that
    /// swaps `replan_following` for `replan_budgeted` inside `repair` makes
    /// the follow count equal the full count (26 == 26 at n=24) and drops
    /// the "followed" note, so both the strict inequality and the note
    /// witness fail.
    #[test]
    fn repair_follow_is_strictly_cheaper_than_a_full_replan_and_keeps_the_prefix() {
        let n = 24;
        let k = n / 2;
        let mut s = corridor(n);
        let mut plan = None;
        let mut cursor = 0;

        let first = repair(&s, &mut plan, &mut cursor, 10_000, 64);
        assert_eq!(first["decision"], json!("replanned_full"), "{first}");
        let prior = first["suffix"].as_array().unwrap().clone();
        assert_eq!(prior.len(), n, "{first}");

        let reused = repair(&s, &mut plan, &mut cursor, 10_000, 64);
        assert_eq!(reused["decision"], json!("reuse_suffix"), "{reused}");
        assert_eq!(reused["solution"], Value::Null, "{reused}");

        s.observe(&[(format!("(clear r{k})").as_str(), false)])
            .expect("drift observable");

        let full = serde_json::to_value(s.replan_budgeted(10_000, Some(64))).unwrap();
        assert_eq!(full["solved"], json!(true), "{full}");

        let repaired = repair(&s, &mut plan, &mut cursor, 10_000, 64);
        assert_eq!(
            repaired["decision"],
            json!("replanned_following"),
            "{repaired}"
        );
        let sol = &repaired["solution"];
        assert!(
            evaluated(sol) < evaluated(&full),
            "follow {} must be strictly below full {}",
            evaluated(sol),
            evaluated(&full)
        );
        let notes = sol["notes"].as_array().cloned().unwrap_or_default();
        let witness = format!("followed {} still-applicable step(s)", k - 1);
        assert!(
            notes
                .iter()
                .any(|n| n.as_str().is_some_and(|n| n.contains(&witness))),
            "missing kept-prefix witness `{witness}`: {repaired}"
        );
        let suffix = repaired["suffix"].as_array().unwrap();
        for i in 0..(k - 1) {
            assert_eq!(suffix[i], prior[i], "prefix step {i}: {repaired}");
        }
        assert!(plan.is_some() && cursor == 0);
        assert!(s.plan_still_valid(plan.as_ref().unwrap(), cursor));
    }

    #[test]
    fn repair_unsolved_follow_clears_the_stash_without_actuating() {
        let n = 8;
        let mut s = corridor(n);
        let mut plan = None;
        let mut cursor = 0;
        repair(&s, &mut plan, &mut cursor, 10_000, 64);
        // Seal the goal room itself: nothing can reach it any more.
        s.observe(&[(format!("(clear r{n})").as_str(), false)])
            .expect("drift observable");
        let out = repair(&s, &mut plan, &mut cursor, 10_000, 64);
        assert_eq!(out["decision"], json!("replan_unsolved"), "{out}");
        assert_eq!(out["suffix"], json!([]), "{out}");
        assert!(
            plan.is_none(),
            "an unsolved repair must not keep a stale plan"
        );
        assert!(!s.goal_met());
    }

    /// Budget pass-through court: the probe's search must honour `evals`.
    /// Ignoring it (searching with `usize::MAX`) solves this corridor.
    #[test]
    fn probe_search_honours_the_evaluation_budget() {
        let n = 24;
        let s = corridor(n);
        let candidate = ProbeCandidate {
            id: "tight".into(),
            goal: None,
            sight: vec![(format!("(clear r{})", n / 2), false)],
            restrict_contains: None,
        };
        let tight = probe_candidate(&s, &candidate, 1, 64);
        assert_eq!(tight["outcome"], json!("unsolved"), "{tight}");

        let roomy = probe_candidate(&s, &candidate, 10_000, 64);
        assert_eq!(roomy["outcome"], json!("solved"), "{roomy}");
        // The outcome is the witness: `evaluated_states` is not comparable
        // across the two runs because an exhausted budget also counts the
        // enforced-hill-climbing attempt that preceded the capped search.
        assert!(!s.goal_met(), "probing must not mutate the parent");
    }

    #[test]
    fn probe_reports_goal_met_before_search_from_the_fork() {
        let n = 4;
        let s = corridor(n);
        let at_goal = ProbeCandidate {
            id: "teleported".into(),
            goal: None,
            sight: vec![("(at r0)".into(), false), (format!("(at r{n})"), true)],
            restrict_contains: None,
        };
        let away = ProbeCandidate {
            id: "home".into(),
            goal: None,
            sight: vec![],
            restrict_contains: None,
        };
        let results = probe_all(&s, &[at_goal, away], 10_000, 64);
        assert_eq!(
            results[0]["goal_met_before_search"],
            json!(true),
            "{}",
            results[0]
        );
        assert_eq!(
            results[1]["goal_met_before_search"],
            json!(false),
            "{}",
            results[1]
        );
        assert!(!s.goal_met(), "the parent stays where it was");
    }

    #[test]
    fn admission_refuses_each_malformed_request_shape() {
        let c = |id: &str| ProbeCandidate {
            id: id.into(),
            goal: None,
            sight: vec![],
            restrict_contains: None,
        };
        assert_eq!(
            admit_candidates(&[], 16, "x").unwrap_err().0,
            "FP_LIMIT_CANDIDATES"
        );
        let many = (0..=MAX_PROBE_CANDIDATES)
            .map(|i| c(&format!("c{i}")))
            .collect::<Vec<_>>();
        assert_eq!(
            admit_candidates(&many, 16, "x").unwrap_err().0,
            "FP_LIMIT_CANDIDATES"
        );
        assert_eq!(
            admit_candidates(&[c("")], 16, "x").unwrap_err().0,
            "FP_LIMIT_CANDIDATE"
        );
        assert_eq!(
            admit_candidates(&[c("a"), c("a")], 16, "x").unwrap_err().0,
            "FP_DUPLICATE_CANDIDATE"
        );
        let mut long_goal = c("g");
        long_goal.goal = Some("x".repeat(17));
        let refusal = admit_candidates(&[long_goal], 16, "demo").unwrap_err();
        assert_eq!(refusal.0, "FP_LIMIT_CANDIDATE");
        assert!(refusal.1.contains("demo probe limit"), "{refusal:?}");
        assert!(admit_candidates(&[c("a"), c("b")], 16, "x").is_ok());
    }
}
