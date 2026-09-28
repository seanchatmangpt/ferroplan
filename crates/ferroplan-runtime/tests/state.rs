use ferroplan_runtime::state::*; #[test] fn recovery_returns_to_planning(){assert!(allowed(State::Recovering,State::Planning));}
