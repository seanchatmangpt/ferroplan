#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Health {
    Healthy,
    Degraded,
    Open,
}
pub fn eligible(h: Health) -> bool {
    h != Health::Open
}
