#[derive(Clone,Debug,PartialEq,Eq)] pub struct Capability(pub &'static str); pub fn compatible(a:&[Capability],required:&str)->bool{a.iter().any(|c|c.0==required)}
