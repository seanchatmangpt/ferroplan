#[derive(Clone,Debug,PartialEq,Eq)] pub struct Plan{pub provider:String,pub steps:Vec<String>,pub cost:u64} impl Plan{pub fn valid(&self)->bool{!self.provider.is_empty()&&!self.steps.is_empty()}}
