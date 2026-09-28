use ferroplan_runtime::tla_evidence::Counterexample; #[test] fn empty_counterexample_not_evidence(){assert!(!Counterexample{invariant:"I".into(),trace:vec![]}.actionable());}
