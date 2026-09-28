use crate::{outcome::Outcome, provider::Provider};
pub fn dispatch<P: Provider>(p: &P, s: &str) -> Outcome<Vec<String>> {
    p.plan(s)
}
