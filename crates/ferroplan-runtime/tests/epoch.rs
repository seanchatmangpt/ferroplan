use ferroplan_runtime::epoch::Epoch;
#[test]
fn epochs_monotonic() {
    assert!(Epoch(1).next() > Epoch(1));
}
