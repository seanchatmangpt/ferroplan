use crate::{graph::Graph,health::{eligible,Health}}; pub fn reconcile(g:&mut Graph,s:&[(String,Health)]){for(id,h)in s{if !eligible(*h){g.exclude(id)}}}
