#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capability(pub &'static str);

pub fn compatible(available: &[Capability], required: &str) -> bool {
    available.iter().any(|capability| capability.0 == required)
}
