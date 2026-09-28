#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Restart {
    Permanent,
    Transient,
    Temporary,
}
pub fn should_restart(r: Restart, abnormal: bool) -> bool {
    matches!(r, Restart::Permanent) || (matches!(r, Restart::Transient) && abnormal)
}
