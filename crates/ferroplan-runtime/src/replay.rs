use crate::receipt::Receipt;
pub fn same_decision(a: &Receipt, b: &Receipt) -> bool {
    a.subject_sha == b.subject_sha
        && a.epoch == b.epoch
        && a.provider == b.provider
        && a.edge == b.edge
}
