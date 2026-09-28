use crate::epoch::Epoch;
#[derive(Clone, Debug)]
pub struct EpochArtifact {
    pub epoch: Epoch,
    pub digest: u64,
}
pub fn changed(a: &EpochArtifact, b: &EpochArtifact) -> bool {
    a.epoch != b.epoch && a.digest != b.digest
}
