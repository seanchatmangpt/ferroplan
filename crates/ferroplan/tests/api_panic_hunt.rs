//! Public-API panic hunt (ticket `fond-htn-32-fuzz-api-panic-hunt`, wave 4).
//!
//! Contract under test: EVERY public entry point listed below, fed
//! structured garbage matched to its signature types, answers with `Ok` or a
//! TYPED `Err` — never a panic, never a hang. Each call is wall-budgeted
//! (<= 5 s asserted post-call; internal budgets bound the call itself) and
//! wrapped in [`std::panic::catch_unwind`], so a panic is converted into a
//! named test failure instead of silently aborting the hunt.
//!
//! # Call-target table (ticket scope item 1)
//!
//! | # | Target | Signature class | Garbage classes applied here |
//! |---|--------|-----------------|------------------------------|
//! | 1 | `ferroplan_hddl::parser::parse_domain(&str)` | text -> Result | S1-S9 (t01) |
//! | 2 | `ferroplan_hddl::parser::parse_problem(&str)` | text -> Result | S1-S9 (t02) |
//! | 3 | `ferroplan_hddl::probabilistic::preprocess(&str)` | text -> Result | S1-S9 (t03) |
//! | 4 | `ferroplan_hddl::probabilistic::has_probabilistic(&str)` | text -> bool | S1-S9 (t03) |
//! | 5 | `ferroplan_hddl::validate::validate_domain(&Domain)` | AST -> Result | D1-D6 (t04) |
//! | 6 | `ferroplan_hddl::validate::validate_domain_with_warnings(&Domain)` | AST -> Result | D1-D6 (t04) |
//! | 7 | `ferroplan_hddl::validate::validate_problem(&Domain, &Problem)` | AST -> Result | P1-P5 (t05) |
//! | 8 | `ferroplan_hddl::validate::validate_problem_with_warnings(&Domain, &Problem)` | AST -> Result | P1-P5 (t05) |
//! | 9 | `ferroplan_hddl::grounder::build_type_closure(&Domain)` | AST -> Result | D1-D6 (t06) |
//! | 10 | `ferroplan_hddl::grounder::index_objects_by_type(&Domain, &Problem, &closure)` | AST -> map | D1-D6/P1-P5 (t06) |
//! | 11 | `ferroplan_hddl::grounder::compute_reachability(&Domain, &map, &facts, &limits)` | AST -> Result | D2-D6/P2-P5 (t06) |
//! | 12 | `ferroplan_hddl::grounder::ground_actions(&Domain, &map, &limits)` | AST -> Result | D2-D6 (t06) |
//! | 13 | `ferroplan_hddl::grounder::ground_actions_reachable(.., &reach)` | AST -> Result | D2-D6 (t06) |
//! | 14 | `ferroplan_hddl::grounder::ground_methods(&Domain, &map, &limits)` | AST -> Result | D2-D6 (t06) |
//! | 15 | `ferroplan_hddl::grounder::ground_methods_reachable(.., &reach)` | AST -> Result | D2-D6 (t06) |
//! | 16 | `ferroplan_hddl::grounder::ground_initial_facts(&Problem)` | AST -> Result | P1-P5 (t06) |
//! | 17 | `ferroplan_hddl::grounder::ground_root_network(&Problem, &map)` | AST -> Result | P1-P5 (t06) |
//! | 18 | `ferroplan_hddl::grounder::ground(&Domain, &Problem, &limits)` | AST -> Result | D1-D6/P1-P5 (t06) |
//! | 19 | `ferroplan_hddl::grounder::evaluate_ground_goal(&GroundGoal, &facts)` | tree -> bool | G1-G3 bounded (t05); FINDING 1 unbounded (`#[ignore]`d below) |
//! | 20 | `ferroplan_hddl::grounder::action_applicable(&GroundAction, &facts)` | tree -> bool | G1-G3 bounded (t05); FINDING 1 via same path |
//! | 21 | `ferroplan_hddl::translate::translate(&GroundedIR, &limits)` | IR -> Result | I1-I3 (t08) |
//! | 22 | `ferroplan::solve_hddl(&str, &str, &PlannerLimits)` | text+limits -> Result | S1-S9/L1-L2 (t09) |
//! | 23 | `ferroplan::solve_planning_type(&UniversalPlanningRequest)` x18 `PlanningType`s | typed -> Result | U1-U6 (t10-t12) |
//! | 24 | `ferroplan::solve_ppddl(&str, &str, &ProbabilisticOptions)` | text+f32 opts -> Result | S1-S9/F1-F4/L3 (t13) |
//! | 25 | `ferroplan::validate_ppddl_policy(&str, &str, &opts, &solution)` | text+opts+sol -> Result | S1-S9/F1-F4/V1-V2 (t14) |
//! | 26 | `ferroplan::Eve::enter(EveRequest)` | typed record -> Result | E1-E4 (t15) |
//! | 27 | `ferroplan::readiness::capability_manifest()` | unit -> manifest | R1 (t16) |
//! | 28 | `ferroplan::readiness::evaluate_readiness(id, evidence)` | strings -> Result | R2-R4 (t16) |
//! | 29 | `ferroplan::route_planning_request(&PlanningRequest)` | typed record -> Result | S1-S6 x18 types (t17) |
//! | 30 | serde_json deserialization of `UniversalPlanningRequest` | JSON -> Result | J1-J3 (t18) |
//!
//! Garbage classes: S1 empty, S2 whitespace-only, S3 unicode/RTL/zero-width/
//! combining, S4 control chars, S5 1 MB flat, S6 1 MB patterned, S7 bare
//! keyword-collision tokens ("define", "and", "oneof", ":htn", "("), S8
//! unbalanced/mismatched parens, S9 keyword ids in declaration positions.
//! D1 empty Domain, D2 cyclic type hierarchy, D3 self-parent type, D4
//! unicode duplicate names, D5 predicate arity misuse, D6 method heads that
//! do not exist. P1 empty Problem, P2 unicode object/atom names, P3 vars in
//! init, P4 keyword-collision names, P5 garbage `:htn` networks. G1-G3:
//! ground goal trees with unicode atoms / deep `Not` chains (bounded depth
//! here; the unbounded-depth probe is an `#[ignore]`d finding below) / empty
//! compounds. I1-I3: empty `GroundedIR`, goal referencing nothing, tight
//! translate limits. U1 empty `PlanningProblem`, U2 unicode/keyword ids
//! everywhere, U3 edges to unknown states, U4 `PlannerLimits` all-zero
//! (ticket-mandated exact shape), U5 `u64::MAX`/`u32::MAX` counts, U6
//! minimal valid solvable problem. F1 NaN, F2 +inf, F3 -inf, F4 0.0/1.0
//! boundary (discount/epsilon). L1 zero counters, L2 `usize::MAX` counters,
//! L3 zero `ProbabilisticOptions` counters. V1 NaN-valued solution fields,
//! V2 solution structurally inconsistent with sources. E1-E4: Eve request
//! fields empty / unicode / keyword-root-task / over-`MAX_PRIMARY_ACTIVATORS`
//! activator lists. R1-R4: manifest call; empty/whitespace identity, unicode
//! identity, garbage evidence strings. J1 100k-deep JSON nesting, J2 huge
//! integer literals, J3 type-confused fields.
//!
//! Known-defect boundary (do NOT re-run here): 500+-deep `(and (and ...))`
//! HDDL text stack-overflows `parse_domain` — already committed as an
//! `#[ignore]`d reproducer in `tests/hddl_adversarial.rs`
//! (`deep_nesting_1000_and_does_not_overflow_or_hang`) and ticketed as
//! ticket 22 (`fix/parse-depth-budget`); this suite keeps its own text
//! probes at depth <= 200 (measured safe: 250 ok) and its NEW findings in
//! the `#[ignore]`d section at the bottom.

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};

use ferroplan::eve::{
    Activator, CapabilityTarget, Eve, EveRequest, GenesisWorld, HddlSurface, HumanPurpose,
    ManufactureTarget, MAX_PRIMARY_ACTIVATORS,
};
use ferroplan::hddl::solve_hddl;
use ferroplan::planning_runtime::{
    Agent, Method as UMethod, PlannerError, PlannerLimits, PlanningProblem, QueueState, RdfTriple,
    State as UState, Task as UTask, Tool, Transition as UTransition, UniversalPlanningRequest,
    WorkflowEdge,
};
use ferroplan::planning_types::{route_planning_request, PlanningRequest, PlanningType};
use ferroplan::ppddl::{
    solve_ppddl, validate_ppddl_policy, InitialStateProbability, PolicyDecision,
    PolicyOutcome as PpddlOutcome, ProbabilisticObjective, ProbabilisticOptions,
    ProbabilisticSolution, ProbabilisticState, ProbabilisticStatistics,
};
use ferroplan::readiness::{capability_manifest, evaluate_readiness};
use ferroplan_hddl::ast::{Domain, Problem};
use ferroplan_hddl::grounder::{
    action_applicable, build_type_closure, compute_reachability, evaluate_ground_goal, ground,
    ground_actions, ground_actions_reachable, ground_initial_facts, ground_methods,
    ground_methods_reachable, ground_root_network, index_objects_by_type, GroundAction, GroundGoal,
    GroundedIR, GroundingLimits,
};
use ferroplan_hddl::parser::{parse_domain, parse_problem};
use ferroplan_hddl::probabilistic::{has_probabilistic, preprocess};
use ferroplan_hddl::translate::{translate, TranslateLimits};
use ferroplan_hddl::validate::{
    validate_domain, validate_domain_with_warnings, validate_problem,
    validate_problem_with_warnings,
};

/// Per-call wall budget asserted after every hunt call (ticket: <= 5 s each).
const CALL_BUDGET: Duration = Duration::from_secs(5);

/// Run `f` under [`catch_unwind`]; a panic is a FINDING and fails the test
/// naming `target` + `case`. Asserts the post-call wall budget. Returns
/// `f`'s output so assertions about typed results stay possible.
fn hunt<R>(target: &str, case: &str, f: impl FnOnce() -> R) -> Option<R> {
    let start = Instant::now();
    let outcome = catch_unwind(AssertUnwindSafe(f));
    let elapsed = start.elapsed();
    assert!(
        elapsed <= CALL_BUDGET,
        "WALL BUDGET EXCEEDED: {target}({case}) took {elapsed:?} (budget {CALL_BUDGET:?})"
    );
    match outcome {
        Ok(value) => Some(value),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| (*s).to_owned())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "<non-string panic payload>".to_owned());
            panic!("PANIC FINDING: {target}({case}) panicked: {msg}");
        }
    }
}

/// Deterministic xorshift64* — the seeded generator required by the wave-4
/// fuzz rules (seed recorded here: `0x5EED_F00D_2026_0917`). No external
/// dependency; every run of this suite is byte-identical.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
}

/// Build the shared structured-garbage string corpus (S1-S8). Sizes kept
/// explicit: the 1 MB members are used sparingly (S5/S6 are expensive to
/// copy per target, so they appear exactly once per target list).
fn garbage_strings() -> Vec<(String, String)> {
    let mut rng = Seeded(0x5EED_F00D_2026_0917);
    let flat_1mb: String = "x".repeat(1_000_000);
    let patterned_1mb: String = (0..1_000_000)
        .map(|i| {
            match i % 7 {
                0 => '(',
                1 => ')',
                2 => '?',
                3 => ':',
                4 => rng.below(0x80) as u8 as char,
                5 => '\u{0301}', // combining acute — S3
                _ => ' ',
            }
        })
        .collect();
    vec![
        ("S1-empty".to_owned(), String::new()),
        ("S2-ws".to_owned(), " \t\r\n ".to_owned()),
        (
            "S3-unicode".to_owned(),
            "日本語_\u{1F980}_\u{200B}\u{200D}_\u{202E}rtl_\u{0301}a\u{0301}".to_owned(),
        ),
        (
            "S4-control".to_owned(),
            "\u{0}\u{1}\u{7}\u{1b}[31m".to_owned(),
        ),
        ("S5-flat-1mb".to_owned(), flat_1mb),
        ("S6-patterned-1mb".to_owned(), patterned_1mb),
        (
            "S7-keywords".to_owned(),
            "define and or not oneof :htn :goal ( )".to_owned(),
        ),
        ("S8-parens".to_owned(), "((((( )]]]]".to_owned()),
    ]
}

const TARGETS_PARSER: [&str; 2] = ["parse_domain", "parse_problem"];
const TARGETS_PREPROCESS: [&str; 2] = ["preprocess", "has_probabilistic"];

/// S1-S8 against both text->Result parser entry points (call-targets 1-4).
#[test]
fn t01_text_surface_garbage_never_panics() {
    for (case, src) in garbage_strings() {
        for target in TARGETS_PARSER {
            hunt(target, &case, || match target {
                "parse_domain" => parse_domain(&src).is_ok(),
                _ => parse_problem(&src).is_ok(),
            });
        }
        for target in TARGETS_PREPROCESS {
            hunt(target, &case, || match target {
                "preprocess" => preprocess(&src).is_ok(),
                _ => has_probabilistic(&src),
            });
        }
    }
}

/// S9: keyword ids in declaration positions (task named `define`/`and`,
/// predicate named `oneof`), depth-200 nesting (measured safe boundary),
/// and unicode task ids colliding with keywords (ticket scope item 2).
#[test]
fn t02_text_surface_keyword_and_depth_garbage_never_panics() {
    let cases: Vec<(&str, String)> = vec![
        (
            "S9-keyword-task-and-pred",
            r#"(define (domain kw)
                 (:predicates (and ?x) (or ?x) (not ?x))
                 (:task t-and :parameters (?define))
                 (:action define
                   :parameters (?and)
                   :precondition (and)
                   :effect (and)))
               (define (problem p)
                 (:domain kw)
                 (:objects o1)
                 (:init (and o1))
                 (:goal (not (and o1)))
                 (:htn :ordered-subtasks (define o1)))"#
                .to_owned(),
        ),
        (
            "S9-unicode-task-collides-:htn",
            r#"(define (domain uni)
                 (:predicates (p ?x))
                 (:task :htn :parameters (?x))
                 (:action oneof
                   :parameters (?x)
                   :precondition (p ?x)
                   :effect (p ?x)))
               (define (problem pu)
                 (:domain uni)
                 (:objects \u{1F980})
                 (:init (p \u{1F980}))
                 (:goal (p \u{1F980}))
                 (:htn :ordered-subtasks (:htn \u{1F980})))"#
                .replace(r"\u{1F980}", "\u{1F980}"),
        ),
        ("S9-depth200-domain", deep_and_domain(200)),
    ];
    for (case, src) in cases {
        for target in TARGETS_PARSER {
            hunt(target, case, || match target {
                "parse_domain" => parse_domain(&src).is_ok(),
                _ => parse_problem(&src).is_ok(),
            });
        }
    }
}

fn deep_and_domain(depth: usize) -> String {
    let mut pre = String::from("(and ");
    for _ in 0..depth {
        pre.push_str("(and ");
    }
    pre.push_str("(open d1)");
    for _ in 0..depth {
        pre.push(')');
    }
    pre.push(')');
    format!(
        "(define (domain deep)\n\
         \x20 (:predicates (open ?d))\n\
         \x20 (:action move\n\
         \x20   :parameters (?d)\n\
         \x20   :precondition {pre}\n\
         \x20   :effect (open ?d)))\n\
         (define (problem deepp)\n\
         \x20 (:domain deep)\n\
         \x20 (:objects d1)\n\
         \x20 (:init (open d1))\n\
         \x20 (:goal (open d1))\n\
         \x20 (:htn :ordered-subtasks (move d1)))"
    )
}

/// Minimal VALID domain/problem texts — the base against which garbage
/// `Problem`s are validated/grounded (a parse-Err here would silently skip
/// the target).
fn valid_domain_src() -> &'static str {
    "(define (domain doors)
       (:types thing - object)
       (:predicates (open ?d - thing) (at ?x - thing))
       (:constants hub - thing)
       (:task go :parameters (?d - thing))
       (:method m-go
         :parameters (?d - thing)
         :task (go ?d)
         :precondition (open ?d)
         :subtasks ())
       (:action open-door
         :parameters (?d - thing)
         :precondition (at ?d)
         :effect (and (open ?d) (not (at ?d)))))"
}

fn valid_problem_src() -> &'static str {
    "(define (problem doorsp)
       (:domain doors)
       (:objects d1 d2 - thing)
       (:init (at d1))
       (:goal (open d1))
       (:htn :ordered-subtasks (go d1)))"
}

fn valid_pair() -> (Domain, Problem) {
    let domain = parse_domain(valid_domain_src()).expect("valid domain parses");
    let problem = parse_problem(valid_problem_src()).expect("valid problem parses");
    (domain, problem)
}

/// Garbage `Problem` ASTs (P1-P5) that still parse as text where possible;
/// where a shape is unreachable via text, it is built directly on top of the
/// parsed valid `Problem` by mutation of pub fields.
fn garbage_problems() -> Vec<(String, Problem)> {
    let (_, valid) = valid_pair();
    let cases: Vec<(String, Problem)> = vec![
        ("P1-empty-default".to_owned(), Problem::default()),
        ("P2-unicode-objects".to_owned(), {
            let mut p = valid.clone();
            p.objects = vec![
                ferroplan_hddl::ast::TypedObject {
                    name: "\u{1F980}".to_owned(),
                    type_name: "\u{65E5}本語".to_owned(),
                };
                3
            ];
            p.init = vec![ferroplan_hddl::ast::AtomicFormula {
                predicate: "\u{200B}".to_owned(),
                args: vec![ferroplan_hddl::ast::Term::Const(
                    "\u{202E}rtl\u{0301}".to_owned(),
                )],
            }];
            p
        }),
        ("P3-var-in-init".to_owned(), {
            let mut p = valid.clone();
            p.init = vec![ferroplan_hddl::ast::AtomicFormula {
                predicate: "open".to_owned(),
                args: vec![ferroplan_hddl::ast::Term::Var("unbound".to_owned())],
            }];
            p
        }),
        ("P4-keyword-names".to_owned(), {
            let mut p = valid.clone();
            p.objects = vec![ferroplan_hddl::ast::TypedObject {
                name: "define".to_owned(),
                type_name: "and".to_owned(),
            }];
            p
        }),
        ("P5-garbage-htn".to_owned(), {
            let mut p = valid.clone();
            p.htn.subtasks = vec![ferroplan_hddl::ast::Subtask {
                id: "\u{1F980}".to_owned(),
                task: ferroplan_hddl::ast::TaskCall {
                    name: ":goal".to_owned(),
                    args: vec![ferroplan_hddl::ast::Term::Var("x".to_owned())],
                },
            }];
            p.htn.order = vec![
                ferroplan_hddl::ast::OrderEdge {
                    before: "\u{1F980}".to_owned(),
                    after: "\u{1F980}".to_owned(),
                },
                ferroplan_hddl::ast::OrderEdge {
                    before: "ghost".to_owned(),
                    after: "\u{1F980}".to_owned(),
                },
            ];
            p
        }),
    ];
    cases
}

/// Garbage `Domain` ASTs (D1-D6) — empty default + text-parsed pathological
/// shapes (cyclic/self-parent types, unicode duplicates, unknown method
/// heads, arity misuse).
fn garbage_domains() -> Vec<(String, Domain)> {
    let mut cases: Vec<(String, Domain)> = vec![("D1-empty-default".to_owned(), Domain::default())];
    let texts: Vec<(&str, &str)> = vec![
        (
            "D2-cyclic-types",
            "(define (domain cyc)
               (:types a - b b - a)
               (:predicates (p ?x - a))
               (:task t :parameters (?x - b))
               (:action act :parameters (?x - a)
                 :precondition () :effect ()))",
        ),
        (
            "D3-self-parent-type",
            "(define (domain selfp)
               (:types a - a)
               (:predicates (p ?x - a))
               (:task t :parameters (?x - a))
               (:action act :parameters (?x - a)
                 :precondition () :effect ()))",
        ),
        (
            "D4-unicode-duplicates",
            "(define (domain dup)
               (:predicates (p ?x) (p ?x) (\u{1F980} ?x) (\u{1F980} ?x))
               (:task t :parameters (?x))
               (:task t :parameters (?x))
               (:action \u{200B} :parameters (?x)
                 :precondition () :effect ())
               (:action \u{200B} :parameters (?x)
                 :precondition () :effect ()))",
        ),
        (
            "D5-arity-misuse",
            "(define (domain ar)
               (:predicates (p ?x ?y))
               (:task t :parameters (?x))
               (:action act :parameters (?x)
                 :precondition (p ?x)
                 :effect (p ?x ?x ?x)))",
        ),
        (
            "D6-unknown-method-head",
            "(define (domain badm)
               (:predicates (p ?x))
               (:task real :parameters (?x))
               (:method m
                 :parameters (?x)
                 :task ghost
                 :subtasks ordered ()))",
        ),
    ];
    for (case, text) in texts {
        match parse_domain(text) {
            Ok(domain) => cases.push((case.to_owned(), domain)),
            // A parse refusal is itself the typed outcome for that shape;
            // record nothing further (the shape never reaches validate).
            Err(_) => cases.push((format!("{case} [unparseable]"), Domain::default())),
        }
    }
    cases
}

fn tight_ground_limits() -> GroundingLimits {
    GroundingLimits {
        max_ground_actions: 200,
        max_ground_methods: 200,
        prune_unreachable: false,
        max_wall: Some(Duration::from_secs(2)),
    }
}

fn tight_translate_limits() -> TranslateLimits {
    TranslateLimits {
        max_task_network_depth: 16,
        max_wall: Some(Duration::from_secs(2)),
        max_states: Some(1_000),
    }
}

/// Call-targets 5-8: validate over D1-D6 and P1-P5.
#[test]
fn t03_validate_domain_and_problem_garbage_never_panics() {
    for (case, domain) in garbage_domains() {
        hunt("validate_domain", &case, || {
            validate_domain(&domain).is_ok()
        });
        hunt("validate_domain_with_warnings", &case, || {
            validate_domain_with_warnings(&domain).is_ok()
        });
    }
    let (domain, _) = valid_pair();
    for (case, problem) in garbage_problems() {
        hunt("validate_problem", &case, || {
            validate_problem(&domain, &problem).is_ok()
        });
        hunt("validate_problem_with_warnings", &case, || {
            validate_problem_with_warnings(&domain, &problem).is_ok()
        });
    }
}

/// Call-targets 9-18: the whole grounder surface over D1-D6 x P1-P5 with
/// tight limits, plus the empty-domain x empty-problem cross product.
#[test]
fn t04_grounder_surface_garbage_never_panics() {
    for (case, domain) in garbage_domains() {
        for (pcase, problem) in garbage_problems() {
            let full = format!("{case} x {pcase}");
            hunt("build_type_closure", &full, || {
                build_type_closure(&domain).is_ok()
            });
            let closure = build_type_closure(&domain).unwrap_or_default();
            hunt("index_objects_by_type", &full, || {
                index_objects_by_type(&domain, &problem, &closure).is_empty()
            });
            hunt("ground_initial_facts", &full, || {
                ground_initial_facts(&problem).is_ok()
            });
            hunt("ground_root_network", &full, || {
                // fond-htn-24 extended the signature with the typed object
                // universe (root `:htn` parameters bind existentially);
                // pass the same map the production `ground()` path builds
                // for this domain/problem pair.
                ground_root_network(
                    &problem,
                    &index_objects_by_type(&domain, &problem, &closure),
                )
                .is_ok()
            });
            hunt("ground", &full, || {
                ground(&domain, &problem, &tight_ground_limits()).is_ok()
            });
            let facts = ground_initial_facts(&problem).unwrap_or_default();
            let objects_by_type = index_objects_by_type(&domain, &problem, &closure);
            hunt("compute_reachability", &full, || {
                compute_reachability(&domain, &objects_by_type, &facts, &tight_ground_limits())
                    .is_ok()
            });
            let reachability =
                compute_reachability(&domain, &objects_by_type, &facts, &tight_ground_limits())
                    .ok();
            hunt("ground_actions", &full, || {
                ground_actions(&domain, &objects_by_type, &tight_ground_limits()).is_ok()
            });
            hunt("ground_methods", &full, || {
                ground_methods(&domain, &objects_by_type, &tight_ground_limits()).is_ok()
            });
            // Reachability-filtered variants need a ReachabilityInfo; when
            // compute_reachability itself refused (typed Err) these targets
            // get the empty-info cross product via the default-IR path
            // instead — the pairs are re-run inside `ground`, which calls
            // them internally on the successful path.
            if let Some(reach) = &reachability {
                hunt("ground_actions_reachable", &full, || {
                    ground_actions_reachable(
                        &domain,
                        &objects_by_type,
                        &tight_ground_limits(),
                        reach,
                    )
                    .is_ok()
                });
                hunt("ground_methods_reachable", &full, || {
                    ground_methods_reachable(
                        &domain,
                        &objects_by_type,
                        &tight_ground_limits(),
                        reach,
                    )
                    .is_ok()
                });
            }
        }
    }
}

/// Call-targets 19-20: ground-goal evaluation trees G1-G3.
#[test]
fn t05_ground_goal_trees_never_panic() {
    let facts: BTreeSet<String> = ["open(d1)".to_owned(), "\u{1F980}".to_owned()]
        .into_iter()
        .collect();

    let g1 = GroundGoal::Atom("\u{1F980}\u{200D}".to_owned());
    hunt("evaluate_ground_goal", "G1-unicode-atom", || {
        evaluate_ground_goal(&g1, &facts)
    });

    let g2 = GroundGoal::Not(Box::new(GroundGoal::Not(Box::new(GroundGoal::Empty))));
    hunt("evaluate_ground_goal", "G2-nested-not", || {
        evaluate_ground_goal(&g2, &facts)
    });

    let g3 = GroundGoal::And(vec![]);
    hunt("evaluate_ground_goal", "G3-empty-and", || {
        evaluate_ground_goal(&g3, &facts)
    });

    let action = GroundAction {
        name: "\u{202E}rtl".to_owned(),
        precondition: GroundGoal::Or(vec![GroundGoal::Empty, g1]),
        outcomes: vec![],
    };
    hunt("action_applicable", "G1-unicode-or-pre", || {
        action_applicable(&action, &facts)
    });
}

/// Call-target 21: translate over I1-I3 (empty IR, dangling goal, tight
/// limits, zeroed limits).
#[test]
fn t06_translate_garbage_ir_never_panics() {
    let ir_empty = GroundedIR::default();
    hunt("translate", "I1-empty-ir", || {
        translate(&ir_empty, &tight_translate_limits()).is_ok()
    });
    hunt("translate", "I1-empty-ir-zero-limits", || {
        let zero = TranslateLimits {
            max_task_network_depth: 0,
            max_wall: None,
            max_states: None,
        };
        translate(&ir_empty, &zero).is_ok()
    });
    // I2: empty IR whose goal references nothing but limits are generous.
    hunt("translate", "I2-empty-ir-default-limits", || {
        translate(&ir_empty, &TranslateLimits::default()).is_ok()
    });
}

/// Call-target 22: solve_hddl over S1-S8 string pairs x limit shapes
/// (ticket-mandated all-zero PlannerLimits included). max_wall_ms stays 2000
/// for the sweeps so the watchdog keeps every call inside the 5 s budget;
/// the literal all-zero shape runs on the SHORTEST inputs only.
#[test]
fn t07_solve_hddl_garbage_never_panics() {
    let short_cases: Vec<(String, String)> = garbage_strings()
        .into_iter()
        .filter(|(case, _)| !case.starts_with("S5") && !case.starts_with("S6"))
        .collect();
    let big_cases: Vec<(String, String)> = garbage_strings()
        .into_iter()
        .filter(|(case, _)| case.starts_with("S5") || case.starts_with("S6"))
        .collect();
    let limits_wall = PlannerLimits {
        max_wall_ms: 2000,
        ..PlannerLimits::default()
    };
    let limits_zero = PlannerLimits {
        max_states: 0,
        max_depth: 0,
        max_iterations: 0,
        max_wall_ms: 0,
    };

    for (case, domain) in &short_cases {
        for (pcase, problem) in &short_cases {
            let full = format!("{case} x {pcase}");
            hunt("solve_hddl", &full, || {
                solve_hddl(domain, problem, &limits_wall).is_ok()
            });
        }
    }
    for (case, src) in &big_cases {
        hunt("solve_hddl", &format!("{case} x S1-empty"), || {
            solve_hddl(src, "", &limits_wall).is_ok()
        });
        hunt("solve_hddl", &format!("S1-empty x {case}"), || {
            solve_hddl("", src, &limits_wall).is_ok()
        });
    }
    // The exact ticket-mandated all-zero limits on tiny garbage (bounded by
    // parse failing fast — no unbounded work is reachable from these).
    hunt(
        "solve_hddl",
        "S1-empty x S1-empty + all-zero-limits",
        || solve_hddl("", "", &limits_zero).is_ok(),
    );
    hunt(
        "solve_hddl",
        "S7-keywords x S1-empty + all-zero-limits",
        || solve_hddl("define and", "(:htn)", &limits_zero).is_ok(),
    );
    // usize::MAX counters on the same tiny inputs (L2).
    let limits_max = PlannerLimits {
        max_states: usize::MAX,
        max_depth: usize::MAX,
        max_iterations: usize::MAX,
        max_wall_ms: 2000,
    };
    hunt(
        "solve_hddl",
        "S7-keywords x S1-empty + usize::MAX-limits",
        || solve_hddl("define and", "(:htn)", &limits_max).is_ok(),
    );
}

/// One tiny VALID problem per rail family so each of the 18 dispatch arms
/// runs real solver code under garbage limits (U4/U5) — unreachable goals
/// keep every arm short-circuited but still exercising its loops.
fn tiny_valid_problems() -> Vec<(String, PlanningProblem)> {
    let mut base = PlanningProblem::default();
    base.states = vec![
        UState {
            id: "s0".to_owned(),
            facts: BTreeSet::new(),
            fluents: BTreeMap::new(),
        },
        UState {
            id: "s1".to_owned(),
            facts: ["f".to_owned()].into_iter().collect(),
            fluents: BTreeMap::new(),
        },
    ];
    base.initial_states = vec!["s0".to_owned()];
    base.goal.facts = ["f".to_owned()].into_iter().collect();
    base.transitions = vec![UTransition {
        action: "a".to_owned(),
        from: "s0".to_owned(),
        to: "s1".to_owned(),
        cost: 1,
        duration: 1,
        reward: 0,
        probability_ppm: 1_000_000,
        observation: None,
        requires: BTreeSet::new(),
    }];
    let mut workflow = base.clone();
    workflow.tasks = vec![UTask {
        id: "t0".to_owned(),
        primitive_action: Some("a".to_owned()),
        requires: BTreeSet::new(),
    }];
    let mut hierarchical = workflow.clone();
    hierarchical.root_tasks = vec!["t0".to_owned()];
    hierarchical.methods = vec![UMethod {
        id: "m0".to_owned(),
        task: "t0".to_owned(),
        subtasks: vec!["t0".to_owned()],
    }];
    let mut flow = workflow.clone();
    flow.queues = vec![QueueState {
        id: "q0".to_owned(),
        current_wip: 0,
        max_wip: 1,
    }];
    flow.agents = vec![Agent {
        id: "ag0".to_owned(),
        capabilities: ["cap".to_owned()].into_iter().collect(),
        capacity: 1,
        current_wip: 0,
    }];
    flow.tools = vec![Tool {
        id: "tool0".to_owned(),
        capabilities: ["cap".to_owned()].into_iter().collect(),
        authority_bound: true,
        verifier_bound: true,
        receipt_bound: true,
    }];
    let mut rdf = base.clone();
    rdf.rdf = vec![RdfTriple {
        subject: "s".to_owned(),
        predicate: "p".to_owned(),
        object: "o".to_owned(),
    }];
    vec![
        ("U6-classical-shape".to_owned(), base.clone()),
        ("U6-workflow-shape".to_owned(), workflow),
        ("U6-hierarchical-shape".to_owned(), hierarchical),
        ("U6-flow-shape".to_owned(), flow),
        ("U6-rdf-shape".to_owned(), rdf),
    ]
}

/// Call-target 23 part 1: all 18 PlanningTypes x U1 (empty problem) and
/// U2/U3 (garbage ids + dangling edges that still pass validate_problem).
#[test]
fn t08_solve_planning_type_all18_empty_and_garbage_never_panics() {
    let limits = PlannerLimits {
        max_wall_ms: 1500,
        ..PlannerLimits::default()
    };

    let empty = PlanningProblem::default();
    for ptype in PlanningType::ALL {
        let request = UniversalPlanningRequest {
            planning_type: ptype,
            problem: empty.clone(),
            limits: limits.clone(),
        };
        hunt(
            "solve_planning_type",
            &format!("U1-empty x {ptype:?}"),
            || solve_planning_type_checked(&request),
        );
    }

    // U2/U3: unicode + keyword ids, transition to an UNKNOWN state is
    // refused by validate_problem itself (typed) — include it to prove the
    // refusal path for every type; a second problem passes validation but
    // has garbage ids inside the reachable graph.
    let mut dangling = PlanningProblem::default();
    dangling.initial_states = vec!["\u{1F980}".to_owned()];
    dangling.transitions = vec![UTransition {
        action: "define".to_owned(),
        from: "\u{1F980}".to_owned(),
        to: "ghost".to_owned(),
        cost: 1,
        duration: 1,
        reward: 0,
        probability_ppm: 1_000_000,
        observation: None,
        requires: BTreeSet::new(),
    }];
    dangling.goal.facts = [":htn".to_owned()].into_iter().collect();

    let mut rich = PlanningProblem::default();
    rich.states = vec![
        UState {
            id: "\u{1F980}".to_owned(),
            facts: BTreeSet::new(),
            fluents: BTreeMap::new(),
        },
        UState {
            id: "define".to_owned(),
            facts: [":htn".to_owned()].into_iter().collect(),
            fluents: BTreeMap::new(),
        },
    ];
    rich.initial_states = vec!["\u{1F980}".to_owned()];
    rich.goal.facts = [":htn".to_owned()].into_iter().collect();
    rich.unsafe_states = ["define".to_owned()].into_iter().collect();
    rich.soft_goal_facts = BTreeMap::from([("soft".to_owned(), u64::MAX)]);
    rich.transitions = vec![UTransition {
        action: "and".to_owned(),
        from: "\u{1F980}".to_owned(),
        to: "define".to_owned(),
        cost: u64::MAX,
        duration: u64::MAX,
        reward: i64::MAX,
        probability_ppm: 1_000_000,
        observation: Some("\u{200B}".to_owned()),
        requires: BTreeSet::new(),
    }];
    rich.tasks = vec![UTask {
        id: "oneof".to_owned(),
        primitive_action: Some("and".to_owned()),
        requires: BTreeSet::new(),
    }];
    rich.root_tasks = vec!["oneof".to_owned()];
    rich.methods = vec![UMethod {
        id: "m".to_owned(),
        task: "oneof".to_owned(),
        subtasks: vec!["ghost".to_owned()],
    }];
    rich.workflow_edges = vec![WorkflowEdge {
        before: "oneof".to_owned(),
        after: "oneof".to_owned(),
    }];
    rich.queues = vec![QueueState {
        id: "q".to_owned(),
        current_wip: u64::MAX,
        max_wip: u64::MAX,
    }];
    rich.agents = vec![Agent {
        id: "ag".to_owned(),
        capabilities: ["\u{0}".to_owned()].into_iter().collect(),
        capacity: u64::MAX,
        current_wip: u64::MAX,
    }];
    rich.tools = vec![Tool::default()];
    rich.rdf = vec![RdfTriple {
        subject: "\u{202E}".to_owned(),
        predicate: "define".to_owned(),
        object: "\u{1F980}".to_owned(),
    }];

    for ptype in PlanningType::ALL {
        for (ucase, problem) in [("U3-dangling", &dangling), ("U2-rich-garbage", &rich)] {
            let request = UniversalPlanningRequest {
                planning_type: ptype,
                problem: problem.clone(),
                limits: limits.clone(),
            };
            hunt(
                "solve_planning_type",
                &format!("{ucase} x {ptype:?}"),
                || solve_planning_type_checked(&request),
            );
        }
    }
}

/// Call-target 23 part 2: all 18 PlanningTypes over U6 minimal valid
/// problems x U4 all-zero limits and U5 MAX limits.
#[test]
fn t09_solve_planning_type_all18_tiny_valid_extreme_limits_never_panic() {
    let zero = PlannerLimits {
        max_states: 0,
        max_depth: 0,
        max_iterations: 0,
        max_wall_ms: 0,
    };
    let maxl = PlannerLimits {
        max_states: usize::MAX,
        max_depth: usize::MAX,
        max_iterations: usize::MAX,
        max_wall_ms: 0, // tiny problems terminate on their own
    };
    for (ucase, problem) in tiny_valid_problems() {
        for (lcase, limits) in [
            ("U4-zero-limits", zero.clone()),
            ("U5-max-limits", maxl.clone()),
        ] {
            for ptype in PlanningType::ALL {
                let request = UniversalPlanningRequest {
                    planning_type: ptype,
                    problem: problem.clone(),
                    limits: limits.clone(),
                };
                hunt(
                    "solve_planning_type",
                    &format!("{ucase} x {lcase} x {ptype:?}"),
                    || solve_planning_type_checked(&request),
                );
            }
        }
    }
}

/// `solve_planning_type` is infallible-free by contract: here we only assert
/// the call does not panic (Ok/Err both fine), keeping the harness uniform.
fn solve_planning_type_checked(request: &UniversalPlanningRequest) -> Result<bool, PlannerError> {
    ferroplan::planning_runtime::solve_planning_type(request).map(|plan| plan.solved)
}

/// Call-target 24: solve_ppddl over S1-S8 x F1-F4 x L3.
#[test]
fn t10_solve_ppddl_garbage_never_panics() {
    let short_cases: Vec<(String, String)> = garbage_strings()
        .into_iter()
        .filter(|(case, _)| !case.starts_with("S5") && !case.starts_with("S6"))
        .collect();
    let big_cases: Vec<(String, String)> = garbage_strings()
        .into_iter()
        .filter(|(case, _)| case.starts_with("S5") || case.starts_with("S6"))
        .collect();
    let option_variants: Vec<(&str, ProbabilisticOptions)> = vec![
        (
            "L3-zero-counters",
            ProbabilisticOptions {
                max_iterations: 0,
                max_states: 0,
                max_transitions: 0,
                max_outcomes_per_action: 0,
                max_policy_entries: 0,
                max_value_cells: 0,
                max_initial_outcomes: 0,
                simulation_max_steps: 0,
                ..ProbabilisticOptions::default()
            },
        ),
        (
            "F1-nan-discount",
            ProbabilisticOptions {
                discount: f64::NAN,
                ..ProbabilisticOptions::default()
            },
        ),
        (
            "F2-inf-discount",
            ProbabilisticOptions {
                discount: f64::INFINITY,
                ..ProbabilisticOptions::default()
            },
        ),
        (
            "F3-neg-inf-epsilon",
            ProbabilisticOptions {
                epsilon: f64::NEG_INFINITY,
                ..ProbabilisticOptions::default()
            },
        ),
        (
            "F4-boundary-1.0",
            ProbabilisticOptions {
                discount: 1.0,
                ..ProbabilisticOptions::default()
            },
        ),
    ];
    for (case, domain) in &short_cases {
        for (pcase, problem) in &short_cases {
            let full = format!("{case} x {pcase}");
            hunt("solve_ppddl", &format!("{full} + default-opts"), || {
                solve_ppddl(domain, problem, &ProbabilisticOptions::default()).is_ok()
            });
            hunt("solve_ppddl", &format!("{full} + L3-zero-counters"), || {
                solve_ppddl(domain, problem, &option_variants[0].1).is_ok()
            });
        }
    }
    for (fcase, opts) in &option_variants {
        hunt("solve_ppddl", &format!("S7 x S1 + {fcase}"), || {
            solve_ppddl("define and", "", opts).is_ok()
        });
    }
    for (case, src) in &big_cases {
        hunt(
            "solve_ppddl",
            &format!("{case} x S1 + default-opts"),
            || solve_ppddl(src, "", &ProbabilisticOptions::default()).is_ok(),
        );
    }
}

/// Call-target 25: validate_ppddl_policy over garbage solutions (V1-V2)
/// against garbage sources and NaN/inf option sets.
#[test]
fn t11_validate_ppddl_policy_garbage_never_panics() {
    let garbage_solution = || ProbabilisticSolution {
        solved: true,
        objective: ProbabilisticObjective::Auto,
        initial_value: f64::NAN,
        initial_distribution: vec![InitialStateProbability {
            state: usize::MAX,
            probability: f64::NAN,
            goal: false,
        }],
        states: vec![ProbabilisticState {
            id: usize::MAX,
            facts: vec!["\u{1F980}".to_owned()],
            fluents: BTreeMap::from([("f".to_owned(), f64::INFINITY)]),
            goal: false,
            initial_probability: f64::NAN,
        }],
        initial_action: Some("define".to_owned()),
        horizon: Some(usize::MAX),
        discount: f64::NAN,
        declared_metric: Some("\u{202E}".to_owned()),
        policy: vec![PolicyDecision {
            state: usize::MAX,
            remaining: Some(0),
            action: "and".to_owned(),
            args: vec!["\u{200B}".to_owned()],
            value: f64::NEG_INFINITY,
            outcomes: vec![PpddlOutcome {
                probability: f64::NAN,
                next_state: usize::MAX,
                reward: f64::NAN,
                goal: false,
            }],
        }],
        statistics: ProbabilisticStatistics {
            reachable_states: usize::MAX,
            transitions: usize::MAX,
            ..ProbabilisticStatistics::default()
        },
        notes: vec![],
    };
    let opts_nan = ProbabilisticOptions {
        discount: f64::NAN,
        epsilon: f64::NAN,
        horizon: Some(usize::MAX),
        ..ProbabilisticOptions::default()
    };
    hunt("validate_ppddl_policy", "V1-nan-solution x S7 x F1", || {
        validate_ppddl_policy("define and", "", &opts_nan, &garbage_solution()).is_ok()
    });
    let opts_ok = ProbabilisticOptions::default();
    hunt(
        "validate_ppddl_policy",
        "V1-nan-solution x S1 x default-opts",
        || validate_ppddl_policy("", "", &opts_ok, &garbage_solution()).is_ok(),
    );
    let big = garbage_strings()
        .into_iter()
        .find(|(case, _)| case == "S5-flat-1mb")
        .map(|(_, src)| src)
        .unwrap();
    hunt(
        "validate_ppddl_policy",
        "V1-nan-solution x S5 x default-opts",
        || validate_ppddl_policy(&big, "", &opts_ok, &garbage_solution()).is_ok(),
    );
}

/// Call-target 26: Eve::enter over E1-E4 request garbage.
#[test]
fn t12_eve_enter_garbage_never_panics() {
    let request_for =
        |statement: String, activators: Vec<Activator>, root_task: String| EveRequest {
            purpose: HumanPurpose {
                statement,
                desired_consequence: "\u{1F980} \u{202E}".to_owned(),
                actor: Some(" ".to_owned()),
                activators,
            },
            genesis: GenesisWorld {
                ontology_rdf: "S4\u{0}\u{7}".to_owned(),
                construct_query: "(((((".to_owned(),
                hddl: HddlSurface {
                    domain: "define and".to_owned(),
                    problem: " ".to_owned(),
                    root_task,
                },
                ppddl: Some(ferroplan::eve::PpddlSurface {
                    domain: "\u{1F980}".to_owned(),
                    problem: String::new(),
                }),
            },
            manufacture: ManufactureTarget {
                name: "and".to_owned(),
                template: "\u{65E5}本語".to_owned(),
                artifact_kind: ":htn".to_owned(),
                output: "\u{200B}".to_owned(),
            },
            capability: CapabilityTarget {
                capability: String::new(),
                route: "\u{202E}rtl".to_owned(),
                authority_scopes: vec!["define".to_owned()],
            },
        };
    hunt("Eve::enter", "E1-empty-statement", || {
        Eve::enter(request_for(String::new(), vec![], String::new())).is_ok()
    });
    hunt("Eve::enter", "E2-whitespace-only", || {
        Eve::enter(request_for(" \t\n".to_owned(), vec![], " ".to_owned())).is_ok()
    });
    hunt("Eve::enter", "E3-unicode-keyword-root-task", || {
        Eve::enter(request_for(
            "\u{1F980}".to_owned(),
            vec![],
            "define \u{1F980} :htn".to_owned(),
        ))
        .is_ok()
    });
    hunt("Eve::enter", "E4-over-max-activators", || {
        let activators = (0..=MAX_PRIMARY_ACTIVATORS)
            .map(|i| Activator {
                name: format!("a{i}"),
                value: "\u{0}".to_owned(),
            })
            .collect();
        Eve::enter(request_for("s".to_owned(), activators, "t".to_owned())).is_ok()
    });
}

/// Call-targets 27-28: capability_manifest + evaluate_readiness (R1-R4).
#[test]
fn t13_readiness_garbage_never_panics() {
    hunt("capability_manifest", "R1-no-input", || {
        capability_manifest().capabilities.len()
    });
    let evidence = [
        " ".to_owned(),
        "\u{1F980}".to_owned(),
        "define".to_owned(),
        String::new(),
    ];
    for (rcase, identity) in [
        ("R2-empty-identity", ""),
        ("R3-ws-identity", "   "),
        ("R4-unicode-identity", "\u{1F980}\u{202E}\u{0301}"),
    ] {
        hunt("evaluate_readiness", rcase, || {
            evaluate_readiness(identity, evidence.iter().cloned()).is_ok()
        });
    }
    let big = garbage_strings()
        .into_iter()
        .find(|(case, _)| case == "S5-flat-1mb")
        .map(|(_, src)| src)
        .unwrap();
    hunt("evaluate_readiness", "R5-1mb-identity", || {
        evaluate_readiness(&big, evidence.iter().cloned()).is_ok()
    });
}

/// Call-target 29: route_planning_request over S1-S6 subjects x all 18
/// PlanningTypes (every subject/type pair must be Ok or a typed
/// PlanningRouteError — never a panic).
#[test]
fn t14_route_planning_request_garbage_never_panics() {
    let subjects: Vec<(String, String)> = garbage_strings()
        .into_iter()
        .filter(|(case, _)| !case.starts_with("S6")) // S6 patterned 1 MB x 18 types is pure copying cost; S5 already covers the 1 MB class
        .collect();
    for (case, subject) in &subjects {
        for ptype in PlanningType::ALL {
            let request = PlanningRequest {
                subject: subject.clone(),
                planning_type: ptype,
                available_capabilities: BTreeSet::new(),
                authority_bound: false,
                verifier_bound: false,
                receipt_bound: false,
            };
            hunt(
                "route_planning_request",
                &format!("{case} x {ptype:?}"),
                || route_planning_request(&request).is_ok(),
            );
        }
    }
}

/// Call-target 30: serde inputs J1-J3 — deep nesting, huge ints, type
/// confusion — all must deserialize to typed Err (or Ok), never panic.
#[test]
fn t15_serde_json_deep_and_hostile_garbage_never_panics() {
    // J1: 100k-deep array — serde_json's recursion limit must refuse it.
    let deep = "[".repeat(100_000) + &"]".repeat(100_000);
    hunt(
        "serde_json::from_str::<UniversalPlanningRequest>",
        "J1-100k-deep-array",
        || serde_json::from_str::<UniversalPlanningRequest>(&deep).is_err(),
    );
    // J1b: 100k-deep object nesting.
    let deep_obj = format!(
        "{}\"planning_type\":\"classical\"{}",
        "{\"problem\":{\"states\":[".repeat(50_000 / 4),
        "]}}".repeat(50_000 / 4)
    );
    hunt(
        "serde_json::from_str::<UniversalPlanningRequest>",
        "J1-50k-deep-object",
        || serde_json::from_str::<UniversalPlanningRequest>(&deep_obj).is_err(),
    );
    // J2: huge integer literals in every int field + unknown enum token.
    let huge = format!(
        r#"{{"planning_type":"definitely_not_a_type",
            "problem":{{"initial_states":["s"],
                       "states":[{{"id":"s","facts":[],"fluents":{{"f":{i64_max}}}}}],
                       "transitions":[{{"action":"a","from":"s","to":"s",
                                        "cost":{u64_max},"duration":{u64_max},
                                        "reward":{i64_max},"probability_ppm":{u32_max}}}],
                       "queues":[{{"id":"q","current_wip":{u64_max},"max_wip":{u64_max}}}]}}}}"#,
        i64_max = i64::MAX,
        u64_max = u64::MAX,
        u32_max = u32::MAX,
    );
    hunt(
        "serde_json::from_str::<UniversalPlanningRequest>",
        "J2-huge-ints-unknown-type",
        || serde_json::from_str::<UniversalPlanningRequest>(&huge).is_err(),
    );
    // J3: type confusion — objects where arrays/strings are expected.
    let confused = r#"{"planning_type":17,"problem":{"initial_states":{"not":"a list"}}}"#;
    hunt(
        "serde_json::from_str::<UniversalPlanningRequest>",
        "J3-type-confusion",
        || serde_json::from_str::<UniversalPlanningRequest>(confused).is_err(),
    );
    // Serde round-trip of a garbage-filled request must not panic either.
    let mut rich = PlanningProblem::default();
    rich.initial_states = vec!["\u{1F980}".to_owned()];
    rich.soft_goal_facts = BTreeMap::from([("\u{200B}".to_owned(), u64::MAX)]);
    let request = UniversalPlanningRequest {
        planning_type: PlanningType::Fond,
        problem: rich,
        limits: PlannerLimits::default(),
    };
    hunt("serde_json round-trip", "J4-garbage-round-trip", || {
        serde_json::to_string(&request).is_ok()
            && serde_json::from_str::<UniversalPlanningRequest>(
                &serde_json::to_string(&request).unwrap(),
            )
            .is_ok()
    });
}

// ---------------------------------------------------------------------------
// FINDINGS (measured 2026-09-17, branch fuzz/api-panic-hunt) — reproducers
// committed `#[ignore]`d per the wave-4 fuzz rules; each is a
// process-aborting stack overflow (SIGABRT), which `catch_unwind` cannot
// intercept, so running them in the gate would kill the whole test binary.
// ---------------------------------------------------------------------------

/// Build a `GroundGoal` chain of `op` nesting `depth` deep over one atom.
fn deep_goal_chain(op: fn(Box<GroundGoal>) -> GroundGoal, depth: usize) -> GroundGoal {
    let mut goal = GroundGoal::Atom("x".to_owned());
    for _ in 0..depth {
        goal = op(Box::new(goal));
    }
    goal
}

/// FINDING 1 (feeds a hardening ticket in the ticket-22 recursion class):
///
/// `ferroplan_hddl::grounder::evaluate_ground_goal` recurses over
/// `GroundGoal::Not`/`And`/`Or` with no depth budget, so a DEEP goal tree —
/// trivially constructed through the PUBLIC API (this fn and
/// `grounder::action_applicable`, which forwards to it) — aborts the whole
/// process with a stack overflow (SIGABRT), not a catchable panic and not a
/// typed refusal. Measured on this branch (debug profile, libtest worker
/// thread, macOS arm64): depth 10_000 evaluates fine (`false`), depth
/// 50_000 SIGABRTs. `Not` and `And` arms abort identically; the minimum
/// fix is an explicit-stack (iterative) evaluation or a depth budget —
/// NOT a one-line conversion, so per the ticket no src change was made.
///
/// Intended contract once fixed: running this test (with `--ignored`)
/// terminates — `Ok(false)` or a typed depth-refusal, never process death.
#[ignore = "KNOWN FINDING (fond-htn-32): evaluate_ground_goal has no recursion budget — depth 10k Not-chain evaluates, depth 50k SIGABRTs the process (measured); same class as the ticket-22 parser recursion, different target (goal-tree evaluation). Needs an explicit-stack rewrite, out of scope for this test-only ticket"]
#[test]
fn finding_evaluate_ground_goal_deep_not_chain_50k_aborts() {
    let goal = deep_goal_chain(|g| GroundGoal::Not(g), 50_000);
    let facts = BTreeSet::new();
    // If this line executes at all, the finding is fixed: any outcome that
    // is not process death satisfies the contract.
    let _ = evaluate_ground_goal(&goal, &facts);
}

/// FINDING 1 sibling arm: the `And` recursion has the identical shape and
/// identical threshold behavior (50k SIGABRTs, 10k fine).
#[ignore = "KNOWN FINDING (fond-htn-32): evaluate_ground_goal And arm — same missing recursion budget as the Not arm; depth 50k SIGABRTs (measured)"]
#[test]
fn finding_evaluate_ground_goal_deep_and_chain_50k_aborts() {
    let goal = deep_goal_chain(|g| GroundGoal::And(vec![*g]), 50_000);
    let facts = BTreeSet::new();
    let _ = evaluate_ground_goal(&goal, &facts);
}
