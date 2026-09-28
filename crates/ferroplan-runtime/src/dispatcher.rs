use crate::{provider::Provider,outcome::Outcome}; pub fn dispatch<P:Provider>(p:&P,s:&str)->Outcome<Vec<String>>{p.plan(s)}
