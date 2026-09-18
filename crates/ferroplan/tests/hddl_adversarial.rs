//! Adversarial-robustness integration tests for the HDDL front-end
//! (ticket `fond-htn-14-test-adversarial`).
//!
//! Contract under test: malformed HDDL — whether hand-authored in-repo
//! fixtures (every fixture in `tests/fixtures/hddl-adversarial/` carries a
//! one-line provenance comment; none are derived from any corpus) or an
//! external negative corpus of flawed models — is always answered with a
//! typed error variant or a clean reject, NEVER a panic and NEVER a hang.
//!
//! Coverage relationship to prior hardening (extend, don't duplicate):
//! - `dfd907b` covered short/malformed *headers* and nested `:probabilistic`
//!   blocks at unit level (`parser.rs`/`probabilistic.rs` `#[cfg(test)]`).
//! - `c7c2c4d` added `TranslateLimits::max_states` + one regression test.
//! - `9a41178` added typed refusals for constraints/numeric fluents and
//!   nested probabilistic blocks.
//!
//! This file adds the *breadth* corpus of malformed shapes (unbalanced
//! parens, keyword typos, ordering pathologies, oneof-placement violations,
//! recursion bombs, deep nesting, zero-length/binary files, unicode ids) as
//! a `ferroplan`-side integration suite, plus an `#[ignore]`d runner for the
//! external Sleath–Bercher flawed-domain corpus (koala-repo data — run from
//! `/tmp` only, never vendored).
//!
//! Every adversarial call below goes through [`no_panic`], so a panic is
//! converted into a test failure naming the case — a panic can never pass.

use ferroplan::hddl::solve_hddl;
use ferroplan::planning_runtime::PlannerLimits;
use ferroplan_hddl::grounder::{ground, GroundingLimits};
use ferroplan_hddl::parser::{parse_domain, parse_problem, ParseError, DEFAULT_MAX_PARSE_DEPTH};
use ferroplan_hddl::translate::{translate, TranslateLimits};
use ferroplan_hddl::validate::{validate_domain, validate_problem, ValidationError};

/// Read a fixture at compile time (must stay valid UTF-8 — the non-UTF-8
/// binary case uses [`include_bytes`] and is rejected at the loader
/// boundary instead).
macro_rules! fx {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/hddl-adversarial/",
            $name
        ))
    };
}

/// Run `f` and fail the test with a named message if it panics. This is the
/// per-case wrap the ticket requires: a PANIC is a failure, never an
/// acceptance.
fn no_panic<T>(case: &str, f: impl FnOnce() -> T) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(v) => v,
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| (*s).to_owned())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<opaque panic payload>".to_owned());
            panic!("adversarial case '{case}' PANICKED (must return a typed error instead): {msg}")
        }
    }
}

fn default_planner_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 10_000,
        ..PlannerLimits::default()
    }
}

/// A minimal well-formed problem for a domain-only fixture, so pipeline
/// cases (ground/translate/solve) can run against it. Hand-authored here.
fn problem_for(domain: &str, root_task: &str) -> String {
    format!(
        "(define (problem {domain}-adv-p1)\n\
         \x20 (:domain {domain})\n\
         \x20 (:objects x1)\n\
         \x20 (:init)\n\
         \x20 (:goal (at x1))\n\
         \x20 (:htn :ordered-subtasks ({root_task} x1)))\n"
    )
}

// ---------------------------------------------------------------------------
// Unbalanced parentheses
// ---------------------------------------------------------------------------

#[test]
fn unbalanced_parens_are_typed_syntax_errors() {
    for (case, src) in [
        (
            "unclosed-domain",
            fx!("unbalanced-parens-unclosed-domain.hddl"),
        ),
        (
            "extra-close-domain",
            fx!("unbalanced-parens-extra-close-domain.hddl"),
        ),
        (
            "unclosed-problem",
            fx!("unbalanced-parens-unclosed-problem.hddl"),
        ),
    ] {
        let err = no_panic(case, || {
            parse_domain(src).err().or_else(|| parse_problem(src).err())
        });
        let err =
            err.unwrap_or_else(|| panic!("adversarial case '{case}': expected a parse error"));
        assert!(
            matches!(err, ParseError::Syntax(_)),
            "case '{case}': expected ParseError::Syntax, got {err:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Empty / zero-length / binary inputs
// ---------------------------------------------------------------------------

#[test]
fn empty_define_form_is_typed_syntax_error() {
    let err = no_panic("empty-define-form", || {
        parse_domain(fx!("empty-define-form.hddl"))
    });
    match err {
        Err(ParseError::Syntax(_)) => {}
        other => panic!("expected ParseError::Syntax for '(define)', got {other:?}"),
    }
}

#[test]
fn zero_length_file_is_typed_syntax_error() {
    let err = no_panic("zero-length", || parse_domain(fx!("zero-length.hddl")));
    match err {
        Err(ParseError::Syntax(_)) => {}
        other => panic!("expected ParseError::Syntax for a 0-byte file, got {other:?}"),
    }
}

#[test]
fn utf8_binary_garbage_is_typed_syntax_error() {
    let err = no_panic("binary-garbage-utf8", || {
        parse_domain(fx!("binary-garbage-utf8.hddl"))
    });
    match err {
        Err(ParseError::Syntax(_)) => {}
        other => panic!("expected ParseError::Syntax for UTF-8 binary garbage, got {other:?}"),
    }
}

#[test]
fn non_utf8_binary_garbage_is_rejected_at_loader_boundary() {
    // include_str! cannot even express this file (invalid UTF-8); a real
    // loader must reject it with `from_utf8` BEFORE handing text to the
    // parser. Assert the boundary contract, never a panic. (`#[allow]`
    // needed because the compiler const-folds the include_bytes! literal
    // and can prove the result is Err — that provability IS the point of
    // the assertion.)
    #[allow(invalid_from_utf8)]
    let res = no_panic("binary-garbage-nonutf8", || {
        std::str::from_utf8(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/hddl-adversarial/binary-garbage-nonutf8.bin"
        )))
    });
    assert!(
        res.is_err(),
        "non-UTF-8 bytes must be rejected by the loader boundary (std::str::from_utf8)"
    );
}

// ---------------------------------------------------------------------------
// Empty but structurally legal domain (tolerance contract)
// ---------------------------------------------------------------------------

#[test]
fn empty_sectionless_domain_is_accepted_without_panic() {
    // A `(define (domain empty))` with zero sections is structurally legal
    // HDDL; the parser's contract is tolerance here (no sections are
    // mandatory), so assert clean acceptance + clean validation, no panic.
    let domain = no_panic("empty-domain", || {
        parse_domain(fx!("empty-domain-no-sections.hddl"))
    })
    .expect("sectionless domain must parse cleanly");
    no_panic("empty-domain-validate", || validate_domain(&domain))
        .expect("sectionless domain must validate cleanly");
}

// ---------------------------------------------------------------------------
// Keyword typos
// ---------------------------------------------------------------------------

#[test]
fn keyword_typos_are_typed_unknown_section_errors() {
    let err = no_panic("keyword-typo-domain", || {
        parse_domain(fx!("keyword-typo-domain-section.hddl"))
    });
    match &err {
        Err(ParseError::Syntax(msg)) => {
            assert!(
                msg.contains("unknown domain section"),
                "diagnostic should name the unknown section, got: {msg}"
            );
        }
        other => panic!("expected ParseError::Syntax for ':predicatse', got {other:?}"),
    }

    let err = no_panic("keyword-typo-problem", || {
        parse_problem(fx!("keyword-typo-problem-file.hddl"))
    });
    match &err {
        Err(ParseError::Syntax(msg)) => {
            assert!(
                msg.contains("unknown problem section"),
                "diagnostic should name the unknown section, got: {msg}"
            );
        }
        other => panic!("expected ParseError::Syntax for ':initt', got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// :method without :task
// ---------------------------------------------------------------------------

#[test]
fn method_without_task_is_typed_syntax_error() {
    let err = no_panic("method-missing-task", || {
        parse_domain(fx!("method-missing-task.hddl"))
    });
    match &err {
        Err(ParseError::Syntax(msg)) => assert!(
            msg.contains("':task'"),
            "diagnostic should name the missing ':task', got: {msg}"
        ),
        other => panic!("expected ParseError::Syntax for a :method without :task, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Ordering pathologies
// ---------------------------------------------------------------------------

#[test]
fn ordering_edge_to_unknown_subtask_id_is_documented_tolerance() {
    // Superseded semantics (ticket History records the audit-era tolerance):
    // validation now cross-checks ':ordering' edges against ':subtasks' ids
    // (fix/hddl-validation) — an edge naming a nonexistent id is a typed
    // UndefinedOrderRef rejection at validation/ground time, not silently
    // carried dead data. Parsing still accepts; validation must refuse.
    let src = fx!("ordering-unknown-subtask-id.hddl");
    let domain = no_panic("order-unknown-parse", || parse_domain(src))
        .expect("ordering-with-unknown-id must parse (tolerance moved to validation)");
    let validation = no_panic("order-unknown-validate", || validate_domain(&domain));
    assert!(
        matches!(&validation, Err(ValidationError::UndefinedOrderRef { .. })),
        "unknown ordering id must be a typed UndefinedOrderRef rejection, got {validation:?}"
    );
}

#[test]
fn cyclic_ordering_is_rejected_cleanly_never_hangs() {
    // s1 < s2 < s1: nothing is ready, the BFS frontier dead-ends with a
    // finite state space. Clean pipeline outcome (planner-level refusal),
    // never a panic, never a hang — the whole call is under solve_hddl's
    // wall-clock budget.
    let domain_src = fx!("ordering-cyclic.hddl");
    let problem_src = problem_for("order-cyclic", "travel");
    let result = no_panic("order-cyclic-solve", || {
        solve_hddl(domain_src, &problem_src, &default_planner_limits())
    });
    match &result {
        Err(_) => {} // clean rejection — any typed HddlError variant is fine
        Ok(_) => panic!(
            "cyclic ordering must not yield a plan (dead-end frontier expected to refuse cleanly)"
        ),
    }
}

// ---------------------------------------------------------------------------
// Arity pathologies
// ---------------------------------------------------------------------------

#[test]
fn method_task_wrong_arity_is_typed_arity_mismatch() {
    let domain = no_panic("arity-method-parse", || {
        parse_domain(fx!("method-task-wrong-arity.hddl"))
    })
    .expect("arity-flawed domain must still parse (validation catches it)");
    let err = no_panic("arity-method-validate", || validate_domain(&domain))
        .expect_err("wrong-arity method :task must be rejected");
    assert_eq!(
        err,
        ValidationError::ArityMismatch {
            task: "travel".to_owned(),
            expected: 2,
            found: 1,
        },
        "expected the specific ArityMismatch variant"
    );
}

#[test]
fn htn_root_task_wrong_arity_is_clean_reject_never_hangs() {
    // The root ':htn' network calls 0-parameter action 'move' with one
    // argument. validate_problem checks subtask *names* but not root arity
    // (audit outcome, recorded in ticket History), so this flows through
    // grounding into a dead-end translation and a clean planner refusal.
    let domain_src = "\
(define (domain arity-root)
  (:predicates (at ?x))
  (:action move
    :parameters ()
    :precondition ()
    :effect (at ?x)))
";
    let problem_src = fx!("htn-root-task-wrong-arity.hddl");
    let result = no_panic("arity-root-solve", || {
        solve_hddl(domain_src, problem_src, &default_planner_limits())
    });
    assert!(
        result.is_err(),
        "wrong-arity root task must not produce a plan"
    );
}

// ---------------------------------------------------------------------------
// oneof placement
// ---------------------------------------------------------------------------

#[test]
fn empty_oneof_is_cleanly_handled_never_panics() {
    // Superseded semantics (ticket History records the audit-era tolerance):
    // since fix/oneof-koala-semantics, '(oneof)' with zero branches is a
    // typed MalformedOneof rejection at parse time — never a panic, never
    // a silently-unexecutable action.
    let parsed = no_panic("oneof-empty-parse", || {
        parse_domain(fx!("oneof-empty.hddl"))
    });
    match &parsed {
        Err(ParseError::MalformedOneof(_)) => {}
        other => panic!("'(oneof)' must be a typed MalformedOneof rejection, got {other:?}"),
    }
}

#[test]
fn nested_oneof_is_typed_malformed_oneof() {
    let err = no_panic("oneof-nested", || parse_domain(fx!("oneof-nested.hddl")));
    match err {
        Err(e @ ParseError::MalformedOneof(_)) => {
            assert!(
                e.to_string().to_lowercase().contains("oneof"),
                "diagnostic should name the construct, got: {e}"
            );
        }
        other => panic!("expected ParseError::MalformedOneof for nested oneof, got {other:?}"),
    }
}

#[test]
fn oneof_in_precondition_is_typed_rejection() {
    // Since fix/oneof-koala-semantics, oneof in a goal-description position
    // is a dedicated MalformedOneof arm in parse_goal — no longer a generic
    // Syntax failure.
    let err = no_panic("oneof-precond", || {
        parse_domain(fx!("oneof-in-precondition.hddl"))
    });
    match &err {
        Err(ParseError::MalformedOneof(_)) => {}
        other => panic!("expected MalformedOneof for oneof-in-precondition, got {other:?}"),
    }
}

#[test]
fn when_inside_oneof_branch_is_cleanly_handled_never_panics() {
    // Superseded semantics (ticket History records the audit-era tolerance):
    // the reference dialect parses '(when ...)' inside a oneof branch and
    // then silently DROPS the conditional effect downstream; since
    // fix/oneof-koala-semantics ferroplan refuses loudly instead — a typed
    // MalformedOneof rejection at parse, never a panic, never a silently
    // weakened domain.
    let parsed = no_panic("when-oneof-parse", || {
        parse_domain(fx!("when-inside-oneof-branch.hddl"))
    });
    match &parsed {
        Err(ParseError::MalformedOneof(_)) => {}
        other => {
            panic!("when-inside-oneof must be a typed MalformedOneof rejection, got {other:?}")
        }
    }
}

// ---------------------------------------------------------------------------
// Declaration/reference mismatches (validator family)
// ---------------------------------------------------------------------------

#[test]
fn undeclared_predicate_in_effect_is_typed_validation_error() {
    let domain = no_panic("undeclared-pred-parse", || {
        parse_domain(fx!("undeclared-predicate-in-effect.hddl"))
    })
    .expect("undeclared-predicate domain must parse");
    let err = no_panic("undeclared-pred-validate", || validate_domain(&domain))
        .expect_err("undeclared predicate in :effect must be rejected");
    assert_eq!(
        err,
        ValidationError::UndefinedPredicate("teleport".to_owned()),
    );
}

#[test]
fn undeclared_type_in_params_is_typed_validation_error() {
    let domain = no_panic("undeclared-type-parse", || {
        parse_domain(fx!("undeclared-type-in-params.hddl"))
    })
    .expect("undeclared-type domain must parse");
    let err = no_panic("undeclared-type-validate", || validate_domain(&domain))
        .expect_err("undeclared parameter type must be rejected");
    assert_eq!(err, ValidationError::UndefinedType("vehicle".to_owned()));
}

#[test]
fn duplicate_predicate_is_typed_validation_error() {
    let domain = no_panic("duplicate-pred-parse", || {
        parse_domain(fx!("duplicate-predicate.hddl"))
    })
    .expect("duplicate-predicate domain must parse");
    let err = no_panic("duplicate-pred-validate", || validate_domain(&domain))
        .expect_err("duplicate predicate declaration must be rejected");
    assert_eq!(
        err,
        ValidationError::DuplicateDefinition {
            kind: ferroplan_hddl::validate::DuplicateKind::Predicate,
            name: "at".to_owned(),
        }
    );
}

#[test]
fn method_subtask_naming_unknown_task_is_typed_validation_error() {
    let domain = no_panic("unknown-subtask-parse", || {
        parse_domain(fx!("subtask-unknown-task.hddl"))
    })
    .expect("unknown-subtask domain must parse");
    let err = no_panic("unknown-subtask-validate", || validate_domain(&domain))
        .expect_err("subtask naming an undeclared task/action must be rejected");
    assert_eq!(err, ValidationError::UnknownTaskOrAction("warp".to_owned()));
}

#[test]
fn problem_goal_referencing_undeclared_object_never_panics() {
    // Superseded semantics (ticket History records the audit-era tolerance):
    // since fix/hddl-validation, validate_problem checks goal atoms against
    // declared predicates/objects — an undeclared object in ':goal' is a
    // typed rejection before grounding. Never a panic.
    let domain_src = "\
(define (domain ghost-domain)
  (:predicates (at ?x))
  (:action move
    :parameters (?x)
    :precondition ()
    :effect (at ?x)))
";
    let domain = parse_domain(domain_src).expect("companion domain parses");
    let problem = no_panic("ghost-problem-parse", || {
        parse_problem(fx!("problem-goal-undeclared-object.hddl"))
    })
    .expect("goal-references-unknown-object problem must parse");
    let validation = no_panic("ghost-validate", || validate_problem(&domain, &problem));
    assert!(
        validation.is_err(),
        "goal referencing an undeclared object must be a typed validation rejection, got Ok"
    );
}

// ---------------------------------------------------------------------------
// Domain-name mismatch (documented tolerance — audited gap)
// ---------------------------------------------------------------------------

#[test]
fn domain_name_mismatch_is_documented_tolerance_never_panics() {
    // Audit outcome (ticket History): NEITHER parse_problem NOR
    // validate_problem cross-checks problem.domain_name against the paired
    // Domain::name — the names are only matched by the caller. Assert the
    // current tolerance explicitly (clean acceptance, no panic) so a future
    // mismatch check must update this test consciously.
    let domain_src = "\
(define (domain mismatch)
  (:predicates (at ?x))
  (:action move
    :parameters (?x)
    :precondition ()
    :effect (at ?x)))
";
    let domain = parse_domain(domain_src).expect("domain parses");
    let problem = no_panic("mismatch-problem-parse", || {
        parse_problem(fx!("domain-name-mismatch-problem.hddl"))
    })
    .expect("mismatched problem must parse");
    assert_eq!(problem.domain_name, "other");
    assert_eq!(domain.name, "mismatch");
    no_panic("mismatch-validate", || validate_problem(&domain, &problem))
        .expect("per audit, name mismatch is not validated (documented tolerance)");
}

// ---------------------------------------------------------------------------
// Recursion / deep-nesting DoS shapes
// ---------------------------------------------------------------------------

#[test]
fn infinite_decomposition_recursion_hits_translate_limits_never_hangs() {
    // Task 'loop' has exactly one method decomposing into 'loop' itself.
    // Must be refused by TranslateLimits (depth or states — the API walls),
    // well inside solve_hddl's wall-clock budget; never a hang.
    let domain_src = fx!("infinite-recursion-method.hddl");
    // Root network calls the 0-parameter task 'loop' with NO arguments —
    // the recursive method must actually match, or nothing decomposes.
    let problem_src = "\
(define (problem infinite-recursion-adv-p1)
  (:domain infinite-recursion)
  (:objects x1)
  (:init)
  (:goal (at x1))
  (:htn :ordered-subtasks (loop)))
";
    let started = std::time::Instant::now();
    let domain = parse_domain(domain_src).expect("recursive domain parses");
    let problem = parse_problem(problem_src).expect("companion problem parses");
    let ir = no_panic("recursion-ground", || {
        ground(&domain, &problem, &GroundingLimits::default())
    })
    .expect("grounding a recursive method set must terminate");
    // AUDIT OUTCOME (ticket History — ticket premise falsified, in a good
    // way): translate does NOT need a limit knob here. Frontier
    // canonicalization collapses the infinite decomposition (loop -> loop)
    // into a single composite state with a self-loop decompose transition,
    // so the BFS reaches its fixed point immediately and returns Ok. The
    // recursion is absorbed by canonicalization, not refused by a wall.
    // Assert the terminating behavior; the clean refusal happens at the
    // solver (the self-loop never reaches the goal).
    let translated = no_panic("recursion-translate", || {
        translate(&ir, &TranslateLimits::default())
    })
    .expect("recursive decomposition must terminate (canonicalized), not hang");
    assert!(
        !translated.states.is_empty(),
        "translated problem must still be well-formed"
    );

    // Full pipeline: the collapsed self-loop can never reach the goal, so
    // solve_hddl refuses cleanly (typed Err) — no panic, no hang, the whole
    // call bounded by PlannerLimits::max_wall_ms.
    let result = no_panic("recursion-solve", || {
        solve_hddl(domain_src, problem_src, &default_planner_limits())
    });
    assert!(
        result.is_err(),
        "full pipeline must refuse the recursive domain (unreachable goal)"
    );
    let elapsed = started.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(30),
        "recursion case took {elapsed:?} — suspiciously close to hanging"
    );
}

/// Formerly the `#[ignore]`d KNOWN DEFECT (TODO parse-depth-budget): deep
/// `(and (and ...))` nesting used to ABORT the process with a stack
/// overflow — the recursive reader had no depth bound. Measured when the
/// defect was filed: depth 250 parsed, depth 500 SIGABRTed inside
/// `parse_domain`, depth 1000 killed the libtest harness outright (a stack
/// overflow is not a panic: `catch_unwind` cannot intercept it).
///
/// Fixed by the parser depth budget (ticket
/// `fond-htn-22-parse-depth-budget`): `read_one` refuses any form nested
/// deeper than [`DEFAULT_MAX_PARSE_DEPTH`] (256) with a typed
/// `ParseError::NestingTooDeep` carrying the offending `(`'s 1-based
/// line/column, and the budget bounds every downstream recursive pass
/// (goal/effect descent included). This test now runs un-ignored and
/// asserts the ticket contract: the 1000-deep input is answered with the
/// typed error at the parser boundary AND through the full `solve_hddl`
/// pipeline, inside the wall-clock budget — never process death.
#[test]
fn deep_nesting_1000_and_returns_typed_nesting_error_never_aborts() {
    let domain_src = {
        let mut goal = "(at x1)".to_owned();
        for _ in 0..1000 {
            goal = format!("(and {goal})");
        }
        format!(
            "(define (domain deep-nest)\n\
             \x20 (:predicates (at ?x))\n\
             \x20 (:action move\n\
             \x20   :parameters (?x)\n\
             \x20   :precondition {goal}\n\
             \x20   :effect (at ?x)))\n"
        )
    };
    let problem_src = problem_for("deep-nest", "move");

    // Parser boundary: typed NestingTooDeep with the default budget and a
    // real 1-based position payload pointing at the offending '('.
    let err = no_panic("deep-nest-parse", || parse_domain(&domain_src)).expect_err(
        "1000-deep nesting must be refused with a typed error, never parsed or aborted",
    );
    match &err {
        ParseError::NestingTooDeep {
            line,
            column,
            budget,
        } => {
            assert_eq!(*budget, DEFAULT_MAX_PARSE_DEPTH);
            assert!(
                *line >= 1 && *column >= 1,
                "position payload must be a real 1-based position, got {line}:{column}"
            );
        }
        other => panic!("expected NestingTooDeep, got {other:?}"),
    }

    // Full pipeline: solve_hddl surfaces the same typed refusal as a
    // `HddlError::Parse` whose diagnostic names the depth violation — the
    // process survives to run these assertions.
    let started = std::time::Instant::now();
    let result = no_panic("deep-nest-pipeline", || {
        solve_hddl(&domain_src, &problem_src, &default_planner_limits())
    });
    let elapsed = started.elapsed();
    let msg = result
        .expect_err("pipeline must refuse the 1000-deep domain with a typed error, not abort")
        .to_string();
    assert!(
        msg.to_lowercase().contains("nesting too deep"),
        "pipeline diagnostic should carry the depth refusal, got: {msg}"
    );
    assert!(
        elapsed < std::time::Duration::from_secs(30),
        "deep-nesting case took {elapsed:?} — suspiciously close to hanging"
    );
}

/// The ticket falsifier: a 10 000-deep `(and ...)` input returns the typed
/// error, exit 0, no signal. Reaching the final assertion at all proves the
/// process was never killed by a signal — before the depth budget,
/// 500-deep already aborted the process on this same 2 MiB libtest stack.
#[test]
fn ten_thousand_deep_and_returns_typed_error_exit_zero_no_signal() {
    let domain_src = format!(
        "(define (domain deep-nest)\n\
         \x20 (:predicates (at ?x))\n\
         \x20 (:action move\n\
         \x20   :parameters (?x)\n\
         \x20   :precondition {}(at x1){}\n\
         \x20   :effect (at ?x)))\n",
        "(and ".repeat(10_000),
        ")".repeat(10_000),
    );
    let problem_src = problem_for("deep-nest", "move");

    let err = no_panic("deep-nest-10k-parse", || parse_domain(&domain_src))
        .expect_err("10 000-deep nesting must be refused with a typed error, never aborted");
    assert!(
        matches!(
            &err,
            ParseError::NestingTooDeep { budget, .. } if *budget == DEFAULT_MAX_PARSE_DEPTH
        ),
        "expected NestingTooDeep at the default budget, got {err:?}"
    );

    let result = no_panic("deep-nest-10k-pipeline", || {
        solve_hddl(&domain_src, &problem_src, &default_planner_limits())
    });
    assert!(
        result.is_err(),
        "pipeline must refuse the 10 000-deep domain with a typed error, not abort"
    );
}

// ---------------------------------------------------------------------------
// Unicode identifiers (positive control)
// ---------------------------------------------------------------------------

#[test]
fn unicode_identifiers_flow_through_the_whole_pipeline() {
    let domain_src = fx!("unicode-identifiers.hddl");
    let problem_src = fx!("unicode-identifiers-problem.hddl");
    let domain = no_panic("unicode-parse-domain", || parse_domain(domain_src))
        .expect("unicode identifiers must parse");
    let problem = no_panic("unicode-parse-problem", || parse_problem(problem_src))
        .expect("unicode problem must parse");
    no_panic("unicode-validate", || validate_problem(&domain, &problem))
        .expect("unicode model must validate");
    let ir = no_panic("unicode-ground", || {
        ground(&domain, &problem, &GroundingLimits::default())
    })
    .expect("unicode model must ground");
    no_panic("unicode-translate", || {
        translate(&ir, &TranslateLimits::default())
    })
    .expect("unicode model must translate");
}

// ---------------------------------------------------------------------------
// Trailing extra paren (documented tolerance — audited read_top gap)
// ---------------------------------------------------------------------------

#[test]
fn trailing_extra_paren_is_documented_tolerance() {
    // read_one stops after the first complete top-level form; read_top
    // discards any trailing tokens, so one extra ')' after a balanced
    // 'define' is currently accepted silently. (An extra ')' *inside* the
    // form IS caught — see unbalanced_parens_are_typed_syntax_errors.)
    // Assert the tolerance explicitly so a stricter reader must update
    // this test consciously.
    let parsed = no_panic("trailing-paren", || {
        parse_domain(fx!("trailing-extra-paren-domain.hddl"))
    });
    assert!(
        parsed.is_ok(),
        "per audit, a trailing extra paren is silently discarded by read_top — got {parsed:?}"
    );
}

// ---------------------------------------------------------------------------
// External negative corpus: 26 Sleath–Bercher flawed domains
// (koala HDDL-Parser repo data — run from /tmp ONLY, never vendored)
// ---------------------------------------------------------------------------

/// Files the external corpus classifies as flawed-model cases that
/// ferroplan's front-end (parse + validate + ground) currently answers
/// with clean acceptance. Each entry is a SCOPE GAP, recorded in ticket
/// History: the detector either does not exist yet (e.g. no abstract-task
/// decomposition-coverage check, no subtask-ordering checks, no
/// duplicate-action/parameter checks, no predicate-arity check, no
/// complementary-effects/preconditions FOND check) or lives past the
/// front-end (solve-time). This allowlist may only SHRINK: any file that
/// starts being rejected must be removed here consciously.
const CORPUS_ACCEPTED_SCOPE_GAPS: &[(&str, &str)] = &[
    (
        "abstract-task-without-decomposition-domain.hddl",
        "no abstract-task decomposition-coverage check",
    ),
    (
        "abstract-task-without-refinement-domain.hddl",
        "no abstract-task refinement-coverage check",
    ),
    (
        "complementary-effects-domain.hddl",
        "no FOND complementary-effects check (solve-time semantic flaw)",
    ),
    (
        "complementary-preconditions-domain.hddl",
        "no FOND complementary-preconditions check",
    ),
    (
        "possible-complementary-effects-domain.hddl",
        "no FOND possible-complementary-effects check",
    ),
    (
        "cyclic-ordering-for-subtasks-domain.hddl",
        "no subtask-ordering graph checks (same gap as the in-repo ordering fixtures)",
    ),
    (
        "duplicate-action-domain.hddl",
        "validate_domain checks duplicate tasks/predicates/types but not duplicate actions",
    ),
    (
        "duplicate-decomposition-method-domain.hddl",
        "no duplicate-method-name check (both methods are offered as decompositions)",
    ),
    (
        "duplicate-parameters-domain.hddl",
        "no duplicate-parameter-name check within one schema",
    ),
    (
        "inconsistent-num-parameters-predicate-domain.hddl",
        "no predicate-arity check (names only, not argument counts)",
    ),
    (
        "inconsistent-num-parameters-task-domain.hddl",
        "no subtask-arity check (method :task arity IS checked; subtask calls are not)",
    ),
    (
        "inconsistent-type-parameters-predicate-domain.hddl",
        "no parameter-type-consistency check across predicate uses",
    ),
    (
        "inconsistent-type-parameters-task-domain.hddl",
        "no parameter-type-consistency check across task uses",
    ),
];

/// Run the ferroplan HDDL front-end (parse -> validate -> ground, never
/// panic) over the external corpus of 26 flawed-domain models. Each file
/// must be answered with a diagnostic (parse error, validation error, or
/// grounding error) — never a panic — or appear in
/// [`CORPUS_ACCEPTED_SCOPE_GAPS`]. Outcomes are printed per file; the
/// summary is recorded in ticket History.
///
/// `#[ignore]`d because the corpus lives at absolute paths outside the repo
/// (`/tmp/fond-review/HDDL-Parser/tests/flawed_domains/`) and must not gate
/// CI on machine state. Run explicitly with:
/// `cargo test -p ferroplan --test hddl_adversarial -- --ignored corpus`
#[test]
#[ignore = "external negative corpus at absolute path outside the repo (/tmp/fond-review/HDDL-Parser/tests/flawed_domains); run explicitly with --ignored"]
fn external_corpus_flawed_domains_reject_with_diagnostics_never_panic() {
    let dir = "/tmp/fond-review/HDDL-Parser/tests/flawed_domains";
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("SKIP-WITH-NOTE: external corpus dir {dir} not readable: {e}");
            return;
        }
    };

    let mut files: Vec<std::path::PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "hddl").unwrap_or(false))
        .collect();
    files.sort();

    assert!(
        !files.is_empty(),
        "corpus dir {dir} exists but contains no .hddl files — investigate"
    );

    let mut rejected_parse = 0usize;
    let mut rejected_validate = 0usize;
    let mut rejected_ground = 0usize;
    let mut accepted: Vec<String> = Vec::new();

    for path in &files {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => panic!("corpus file {name} unreadable: {e}"),
        };
        let src = match std::str::from_utf8(&bytes) {
            Ok(s) => s.to_owned(),
            Err(_) => {
                // Rejected at the loader boundary — counts as a diagnostic.
                rejected_parse += 1;
                eprintln!("{name}: REJECT(non-utf8 at loader boundary)");
                continue;
            }
        };
        let parse_result = no_panic(&name, || parse_domain(&src));
        match parse_result {
            Err(e) => {
                rejected_parse += 1;
                eprintln!("{name}: REJECT(parse): {e}");
            }
            Ok(domain) => match no_panic(&name, || validate_domain(&domain)) {
                Err(e) => {
                    rejected_validate += 1;
                    eprintln!("{name}: REJECT(validate): {e}");
                }
                Ok(()) => {
                    // No problem file exists in the corpus (domains only);
                    // ground against a minimal companion problem so the
                    // grounder's own diagnostics (TypeCycle, UnboundVariable)
                    // get their chance before a file is called accepted.
                    let problem_src = format!(
                        "(define (problem {name}-probe)\n\
                         \x20 (:domain {})\n\
                         \x20 (:objects)\n\
                         \x20 (:init)\n\
                         \x20 (:goal (and))\n\
                         \x20 (:htn :ordered-subtasks (and)))\n",
                        domain.name
                    );
                    let ground_attempt = no_panic(&name, || {
                        parse_problem(&problem_src)
                            .map_err(|e| e.to_string())
                            .and_then(|p| {
                                ground(&domain, &p, &GroundingLimits::default())
                                    .map_err(|e| e.to_string())
                            })
                    });
                    match ground_attempt {
                        Err(e) => {
                            rejected_ground += 1;
                            eprintln!("{name}: REJECT(ground): {e}");
                        }
                        Ok(_) => {
                            accepted.push(name.clone());
                            eprintln!("{name}: ACCEPT (parse+validate+ground clean)");
                        }
                    }
                }
            },
        }
    }

    eprintln!(
        "corpus summary: {} files, {} rejected at parse, {} rejected at validate, {} rejected at ground, {} accepted clean",
        files.len(),
        rejected_parse,
        rejected_validate,
        rejected_ground,
        accepted.len()
    );

    // The core safety contract is enforced by `no_panic` above: a panic on
    // ANY corpus file fails this test before reaching these asserts.
    //
    // The rejection contract (ticket): every flawed file must be answered
    // with a diagnostic, OR sit in the explicit scope-gap allowlist below —
    // the allowlist is the ledger and may only shrink.
    let expected_accepted: Vec<&str> = CORPUS_ACCEPTED_SCOPE_GAPS
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let mut actual_accepted = accepted.clone();
    let mut expected_sorted = expected_accepted.clone();
    actual_accepted.sort();
    expected_sorted.sort();
    assert_eq!(
        actual_accepted, expected_sorted,
        "corpus acceptance set drifted from CORPUS_ACCEPTED_SCOPE_GAPS — shrink or extend the allowlist consciously (rejected files removed from it; newly-accepted files added with a gap reason)"
    );
}
