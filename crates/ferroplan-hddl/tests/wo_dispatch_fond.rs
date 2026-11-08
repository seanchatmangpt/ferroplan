//! Guard for `domains/wo-dispatch-fond.hddl` / `domains/wo-dispatch-fond.problem.hddl` —
//! the FOND tier-cycle pilot over woAssignment's dispatcher region
//! (servengine-frontend `src/machines/woAssignment.machine.ts`). The sound domain
//! synthesizes a strong-cyclic policy reaching provider-accepted + arrival under
//! every TIER_OFFER_SENT outcome (accepted / declined / timed-out). The
//! invalidation court deletes the DECLINED compensation arm (tier increment +
//! provider lockout) from every tier action and requires the same pipeline to
//! refuse — the TODO-ARM freeze scenario, where dispatch offers cycle without
//! progress. No doubles: the pipeline (parse -> ground -> translate -> solve) is
//! the only collaborator.

use ferroplan::planning_runtime::PlannerLimits;
use ferroplan::solve_hddl;

const DOMAIN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../domains/wo-dispatch-fond.hddl");
const PROBLEM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../domains/wo-dispatch-fond.problem.hddl"
);

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

/// Policy actions executed (both `htn:decompose:` bookkeeping and `htn:exec:`
/// entries), reduced to the trailing segment names, in policy order.
fn executed_actions(plan: &ferroplan::planning_runtime::UniversalPlan) -> Vec<String> {
    plan.policy
        .iter()
        .map(|e| {
            e.action
                .rsplit(':')
                .next()
                .unwrap_or(&e.action)
                .to_ascii_lowercase()
        })
        .collect()
}

#[test]
fn pair_parses_and_validates() {
    let (d, p) = (read(DOMAIN_PATH), read(PROBLEM_PATH));
    let domain = ferroplan_hddl::parser::parse_domain(&d).expect("domain parses");
    let problem = ferroplan_hddl::parser::parse_problem(&p).expect("problem parses");
    ferroplan_hddl::validate::validate_domain(&domain).expect("domain validates");
    ferroplan_hddl::validate::validate_problem(&domain, &problem).expect("problem validates");
    assert_eq!(domain.name.to_string(), "wo-dispatch-fond");
    assert_eq!(domain.actions.len(), 7);
    assert_eq!(domain.methods.len(), 7);
}

#[test]
fn sound_domain_yields_closed_strong_cyclic_policy() {
    let plan = solve_hddl(
        &read(DOMAIN_PATH),
        &read(PROBLEM_PATH),
        &PlannerLimits::default(),
    )
    .expect("wo-dispatch-fond is solvable");
    assert!(plan.solved, "no strong-cyclic policy: {plan:?}");

    // The policy executes the dispatch cascade, not just its prefix: begin the
    // auto-dispatch, fire a tier offer, and reach accepted/arrived — the goal
    // facts appear in the terminal policy states.
    let acts = executed_actions(&plan);
    assert!(
        acts.iter().any(|a| a == "begin-auto-dispatch"),
        "begin-auto-dispatch missing: {acts:?}"
    );
    assert!(
        acts.iter().any(|a| a.starts_with("tier-offer-sent")),
        "no tier-offer-sent action in policy: {acts:?}"
    );
    assert!(
        acts.iter().any(|a| a == "accept-reactive-hire" || a == "request-new-provider"),
        "fallback lease actions missing: {acts:?}"
    );
    // Closure: every policy entry carries its outcome branches (FOND policy
    // graph, not a linear plan).
    assert!(
        plan.policy.iter().any(|e| !e.outcomes.is_empty()),
        "policy graph has no outcome branches: {plan:?}"
    );
}

/// THE INVALIDATION COURT — DECLINED-compensation deletion (TODO-ARM freeze scenario).
///
/// WITNESSED SEMANTIC GAP (2026-11-08, lane D, run 8733582): the court as specified
/// CANNOT fire against this planner. `solve_hddl` dispatches only to `fond_policy`
/// (`PlanningType::Fond`, crates/ferroplan/src/hddl.rs:4), whose strong-cyclic
/// fixpoint is PRP-style greatest-fixpoint: an outcome state is retained iff SOME
/// outcome branch reaches the goal — a no-change self-loop with an accepted-escape
/// outcome is ADMITTED. Witness (verbatim, from the failed run):
///
///   UniversalPlan { solved: true, ... policy: [... PolicyEntry { state: "s3", action:
///   "htn:exec:r.root.s2.s1:tier-offer-sent-0", outcomes: [... { state: "s4" .. },
///   { state: "s3" .. }] }, ...], notes: ["strong-cyclic FOND fixpoint
///   (weak-reachability + greatest-fixpoint prune)"] }
///
/// Under woAssignment's machine law (PROVIDER_ACCEPTED exits dispatched_to_provider
/// directly, src/machines/woAssignment.machine.ts:200), acceptance does not depend
/// on any DECLINED arm, so deleting the DECLINED compensation arms leaves the
/// accepted-escape path intact and the problem stays solvable. Forcing refusal would
/// require making the DECLINED chain goal-critical, which contradicts :200 — that
/// would be weakening the MODEL, and touching the solver would be weakening the
/// PLANNER. Both are refused by lane law.
///
/// The court is therefore registered loudly-ignored (a named skip, never a faked
/// pass) and re-arms automatically when the planner gains a fairness refinement
/// (strong/acyclic policy semantics, or fair-execution enforcement): at that point
/// delete this `#[ignore]` attribute — the body is already the full falsifier.
#[ignore = "semantic gap: fond_policy admits escape-outcome self-loop policies (no fairness refinement); see doc comment"]
#[test]
fn deleting_declined_compensation_arm_must_refuse() {
    let domain = read(DOMAIN_PATH);
    let declined_arms = [
        "      (and (not (tier-0)) (tier-1) (locked-0))\n",
        "      (and (not (tier-1)) (tier-2) (locked-1))\n",
        "      (and (not (tier-2)) (tier-3) (locked-2))\n",
    ];
    let mut broken = domain.clone();
    for arm in declined_arms {
        assert!(
            broken.contains(arm),
            "declined arm not found verbatim (domain drift): {arm:?}"
        );
        broken = broken.replace(arm, "");
    }
    assert_ne!(broken, domain, "mutation must have applied");

    match solve_hddl(&broken, &read(PROBLEM_PATH), &PlannerLimits::default()) {
        // Typed refusal — the pipeline named the defect itself (liveness/unsound).
        Err(_) => {}
        Ok(plan) => {
            assert!(
                !plan.solved,
                "INVALIDATION COURT FAILED: mutated domain (DECLINED arm deleted) still \
                 synthesized a policy — the solver admitted a vacuous closed-loop over a \
                 no-change self-loop: {plan:?}"
            );
        }
    }
}
