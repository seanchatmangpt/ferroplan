#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub id: String,
    pub provider: String,
    pub enabled: bool,
}
impl Edge {
    pub fn exclude(&mut self) {
        self.enabled = false
    }
}
