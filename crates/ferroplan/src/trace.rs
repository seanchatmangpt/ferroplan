//! FIELD DISPATCH — TRACE UNIT. Replay the plan, frame by frame. Camera
//! rolling before the first move, and after every one after that — this is
//! the footage a UI paints across the wire. Same corridor as
//! [`crate::verify`]'s walk-through, except this one keeps every frame
//! instead of burning them. Sequential ops only — classic, numeric, PDDL3.
//! Temporal plans run overlapping timelines; this camera can't hold two
//! frames at once. Don't point it there.

use serde::{Deserialize, Serialize};

use crate::ground::ground_task;
use crate::packed::State;
use crate::parser::{parse_domain, parse_problem};

/// One still. Everything true, everything measured, at a single tick —
/// facts as bare strings (`(AT TRUCK1 LOC2)`), fluents with their number
/// attached (`(FUEL TRUCK1) = 30`).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StateSnapshot {
    pub facts: Vec<String>,
    pub fluents: Vec<(String, f64)>,
}

/// Run the tape. `plan` — a chain of `(action, args)` pulled from a
/// [`crate::Solution`] — gets replayed over a task ground fresh for this
/// job. Hand back the opening frame plus one for every move made:
/// `plan.len() + 1` snapshots, no fewer. The tape stops cold if grounding
/// fails, if a move doesn't match any grounded op, or if the world won't
/// allow it when its turn comes.
pub fn trace(
    domain_src: &str,
    problem_src: &str,
    plan: &[(String, Vec<String>)],
) -> Result<Vec<StateSnapshot>, String> {
    let domain = parse_domain(domain_src).map_err(|e| format!("domain: {e}"))?;
    let problem = parse_problem(problem_src).map_err(|e| format!("problem: {e}"))?;
    // Compile `:derived` axioms away, like the solve that produced the plan —
    // replaying against the raw problem would miss the derived init facts.
    let (domain, problem) = crate::derived::compile(&domain, &problem)?;
    let task = ground_task(&domain, &problem, 1)
        .ok_or_else(|| "grounding failed (empty type)".to_string())?;

    let snap = |s: &State| -> StateSnapshot {
        let facts = (0..task.n_facts)
            .filter(|&i| (s.bits[i / 64] >> (i % 64)) & 1 == 1)
            .map(|i| task.fact_names[i].clone())
            .collect();
        let fluents = (0..task.fluent_names.len())
            .filter(|&i| s.fdef[i])
            .map(|i| (task.fluent_names[i].clone(), s.fv[i]))
            .collect();
        StateSnapshot { facts, fluents }
    };

    let mut s = task.initial();
    let mut out = vec![snap(&s)];
    for (name, args) in plan {
        let want: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
        let oi = (0..task.n_ops)
            .find(|&oi| {
                let mut it = task.op_display[oi].split_whitespace();
                it.next() == Some(name.as_str()) && it.eq(want.iter().copied())
            })
            .ok_or_else(|| {
                format!(
                    "plan action `{} {}` is not a grounded op",
                    name,
                    args.join(" ")
                )
            })?;
        if !task.op_applicable(oi, &s) {
            return Err(format!(
                "plan action `{} {}` is not applicable in the reached state",
                name,
                args.join(" ")
            ));
        }
        s = task.apply(oi, &s);
        out.push(snap(&s));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOM: &str = "
    (define (domain logi) (:requirements :typing)
      (:types location truck)
      (:predicates (at ?t - truck ?l - location) (road ?a ?b - location))
      (:action drive :parameters (?t - truck ?from ?to - location)
        :precondition (and (at ?t ?from) (road ?from ?to))
        :effect (and (not (at ?t ?from)) (at ?t ?to))))";
    const PRB: &str = "
    (define (problem p) (:domain logi)
      (:objects a b - location  t1 - truck)
      (:init (at t1 a) (road a b))
      (:goal (at t1 b)))";

    #[test]
    fn trace_captures_each_step() {
        let plan = vec![(
            "DRIVE".to_string(),
            vec!["T1".into(), "A".into(), "B".into()],
        )];
        let snaps = trace(DOM, PRB, &plan).expect("trace");
        assert_eq!(snaps.len(), 2, "initial + after the one action");
        assert!(snaps[0].facts.iter().any(|f| f == "(AT T1 A)"));
        assert!(!snaps[0].facts.iter().any(|f| f == "(AT T1 B)"));
        assert!(snaps[1].facts.iter().any(|f| f == "(AT T1 B)"));
        assert!(!snaps[1].facts.iter().any(|f| f == "(AT T1 A)"));
    }

    #[test]
    fn trace_rejects_inapplicable() {
        // driving from B (truck is at A) is not applicable
        let plan = vec![(
            "DRIVE".to_string(),
            vec!["T1".into(), "B".into(), "A".into()],
        )];
        assert!(trace(DOM, PRB, &plan).is_err());
    }
}
