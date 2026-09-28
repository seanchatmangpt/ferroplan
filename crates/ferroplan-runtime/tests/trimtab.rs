use ferroplan_runtime::trimtab::ModelRole;
#[test]
fn model_context_only() {
    let r = ModelRole::default();
    assert!(r.context && !r.select && !r.construct && !r.do_act);
}
