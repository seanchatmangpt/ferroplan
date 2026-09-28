#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Bound,
    Planning,
    Executing,
    Recovering,
    Succeeded,
    Failed,
}
pub fn allowed(a: State, b: State) -> bool {
    use State::*;
    matches!(
        (a, b),
        (Bound, Planning)
            | (Planning, Executing)
            | (Executing, Recovering)
            | (Recovering, Planning)
            | (Executing, Succeeded)
            | (_, Failed)
    )
}
