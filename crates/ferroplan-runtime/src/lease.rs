#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lease {
    pub owner: String,
    pub epoch: u64,
}
impl Lease {
    pub fn valid_for(&self, o: &str, e: u64) -> bool {
        self.owner == o && self.epoch == e
    }
}
