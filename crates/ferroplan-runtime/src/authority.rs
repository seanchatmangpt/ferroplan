#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)] pub enum Authority{Observe,Select,Construct,Do} pub fn permits(g:Authority,n:Authority)->bool{g>=n}
