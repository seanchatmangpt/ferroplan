use ferroplan_runtime::receipt::Receipt;
#[test]
fn receipt_requires_binding() {
    assert!(!Receipt {
        subject_sha: "".into(),
        epoch: 1,
        provider: "p".into(),
        edge: "e".into(),
        outcome: "ok".into()
    }
    .exact());
}
