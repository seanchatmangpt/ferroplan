use ferroplan_runtime::ocel::Event;
#[test]
fn event_binds_objects() {
    assert!(Event {
        id: "1".into(),
        activity: "recover".into(),
        objects: vec!["edge:e".into()]
    }
    .bound());
}
