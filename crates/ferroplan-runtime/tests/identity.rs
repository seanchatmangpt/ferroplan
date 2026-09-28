use ferroplan_runtime::exact_subject::ExactSubject; #[test] fn exact_subject_requires_sha(){assert!(!ExactSubject{repo:"r".into(),sha:"".into()}.admitted());}
