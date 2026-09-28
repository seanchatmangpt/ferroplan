use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub fn key(subject: &str, epoch: u64, provider: &str) -> u64 {
    let mut h = DefaultHasher::new();
    (subject, epoch, provider).hash(&mut h);
    h.finish()
}
