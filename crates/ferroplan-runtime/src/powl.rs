#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub before: String,
    pub after: String,
}
pub fn acyclic(e: &[Order]) -> bool {
    !e.iter().any(|x| x.before == x.after)
}
