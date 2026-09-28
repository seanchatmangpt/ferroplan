#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactSubject {
    pub repo: String,
    pub sha: String,
}
impl ExactSubject {
    pub fn admitted(&self) -> bool {
        self.sha.len() >= 7 && !self.repo.is_empty()
    }
}
