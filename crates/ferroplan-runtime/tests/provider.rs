use ferroplan_runtime::{
    capability::{compatible, Capability},
    health::{eligible, Health},
};
#[test]
fn provider_requires_capability_and_health() {
    assert!(compatible(&[Capability("fond")], "fond"));
    assert!(!eligible(Health::Open));
}
