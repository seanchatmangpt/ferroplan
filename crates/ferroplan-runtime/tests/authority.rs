use ferroplan_runtime::authority::*; #[test] fn do_is_fenced(){assert!(!permits(Authority::Observe,Authority::Do));}
