#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)] pub struct Epoch(pub u64); impl Epoch{pub fn next(self)->Self{Self(self.0+1)}}
