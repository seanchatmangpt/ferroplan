use std::time::{Duration, Instant};
pub struct Deadline(pub Instant);
impl Deadline {
    pub fn after(d: Duration) -> Self {
        Self(Instant::now() + d)
    }
    pub fn expired(&self) -> bool {
        Instant::now() >= self.0
    }
}
