#[derive(Clone, Debug)]
pub struct EvidenceSet {
    pub subject: String,
    pub sources: Vec<String>,
}
impl EvidenceSet {
    pub fn exact(&self) -> bool {
        !self.subject.is_empty() && self.sources.iter().all(|s| !s.is_empty())
    }
}
