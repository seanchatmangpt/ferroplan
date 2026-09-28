use crate::outcome::Outcome; pub trait Provider{fn id(&self)->&str; fn plan(&self,subject:&str)->Outcome<Vec<String>>;}
