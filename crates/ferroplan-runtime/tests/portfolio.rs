use ferroplan_runtime::{plan::Plan, portfolio};
#[test]
fn cheaper_plan_first() {
    let p = portfolio::ranked(vec![
        Plan {
            provider: "a".into(),
            steps: vec!["x".into()],
            cost: 2,
        },
        Plan {
            provider: "b".into(),
            steps: vec!["x".into()],
            cost: 1,
        },
    ]);
    assert_eq!(p[0].provider, "b");
}
