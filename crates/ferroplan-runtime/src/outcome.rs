#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome<T> {
    Success(T),
    Retryable(String),
    Permanent(String),
    Refused(String),
    Unknown(String),
}
impl<T> Outcome<T> {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }
}
