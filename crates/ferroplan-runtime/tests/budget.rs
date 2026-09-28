use ferroplan_runtime::budget::Budget; #[test] fn attempts_bounded(){let mut b=Budget::new(1);assert!(b.take());assert!(!b.take());}
