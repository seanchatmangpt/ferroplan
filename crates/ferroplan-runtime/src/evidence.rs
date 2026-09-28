use crate::observation::Observation;
pub fn admitted(xs: &[Observation]) -> bool {
    !xs.is_empty() && xs.iter().all(|x| !x.source.is_empty())
}
