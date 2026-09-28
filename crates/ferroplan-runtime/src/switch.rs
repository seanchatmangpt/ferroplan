use crate::plan::Plan; pub fn next<'a>(ps:&'a[Plan],failed:&str)->Option<&'a Plan>{ps.iter().find(|p|p.provider!=failed&&p.valid())}
