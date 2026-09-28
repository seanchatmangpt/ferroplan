use crate::{coordinator::Coordinator,fond}; impl Coordinator{pub fn next_edge(&self)->Option<&str>{fond::reselect(&self.graph,&self.excluded)}}
