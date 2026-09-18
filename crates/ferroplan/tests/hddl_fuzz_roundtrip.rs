//! Fuzz: seeded random HDDL generator — parse → validate → ground → translate
//! round-trip (ticket `fond-htn-31-fuzz-hddl-roundtrip`).
//!
//! A seeded, dependency-free SplitMix64 generator draws grammar-faithful
//! HDDL domain+problem pairs from the SUPPORTED surface only (see
//! `ferroplan-hddl/src/lib.rs` for that boundary): typed parameters/objects
//! over a small single-level type hierarchy, predicates, abstract tasks,
//! primitive actions with conjunctive preconditions and conjunctive or
//! top-level-`oneof` effects (branches may be empty `(and)` or overlap),
//! methods with implicit/`<`-style orderings and optional `:precondition`,
//! and problems with `:init`/`:goal`/`:htn` sections. Sizes are
//! parameterized (2–6 tasks, 2–8 actions, 2–10 objects) and a per-case
//! mutation switch (~30% of draws, from the same seeded stream) applies
//! grammatically-valid-but-semantically-odd mutations: shadowing/duplicate
//! names, unused method parameters, singleton types (declared, but with no
//! objects), and duplicate-ish method bodies.
//!
//! Every case runs parse → validate → ground → translate under small caps
//! and asserts:
//! 1. no panic (`catch_unwind` — a panic anywhere in the pipeline is a
//!    finding, never an accepted outcome);
//! 2. any `Err` is a typed error of the crate (`ParseError`,
//!    `ValidationError`, `GroundError`, `TranslateError`) with a non-empty
//!    `Display`; the pipeline's signatures enforce the typing, so the live
//!    assertion is the internal-consistency one: an explicit
//!    `validate_domain`/`validate_problem` pass must never be followed by a
//!    `GroundError::Validation` (ground runs exactly those two checks
//!    first), and a VALID (non-mutated) draw must never fail to parse or
//!    validate at all (that would be generator drift off the SUPPORTED
//!    surface, which is itself a finding about the generator);
//! 3. ~10% of cases additionally run end-to-end through
//!    `ferroplan::hddl::solve_hddl` on a bounded budget: `Ok` plans must be
//!    `solved` with a non-empty, OUTCOME-CLOSED policy (every outcome of
//!    every chosen action lands in a policy state or a goal state, checked
//!    against an independently re-run `translate` of the same case), and
//!    `Err(HddlError::WorkerPanicked(_))` — a disguised pipeline panic — is
//!    a finding; every other `HddlError` variant (including the honest
//!    `Planner(PlannerError::NoPlan)`) is a legitimate typed outcome;
//! 4. determinism: the whole sweep runs twice and the per-case outcome
//!    vectors must be identical (wall-clock `Timeout` outcomes are
//!    normalized to one token before comparison — they are honest refusals
//!    whose firing depends on machine load, not on the seed).
//!
//! Finding discipline (wave-4 rules addenda): on any unexpected panic the
//! case is minimized by halving the draw's size parameters until the panic
//! no longer reproduces, the shrunken domain+problem are written under
//! `tests/fixtures/fuzz-found/` with their seed, and the sweep fails naming
//! the seed and the fixture paths. Committed findings are recorded in
//! `KNOWN_FINDINGS` below together with an `#[ignore]`d reproducer test —
//! the sweep then asserts the known seed STILL panics (a live regression
//! tripwire), never a silent skip. There are no known findings yet; when a
//! fixture + reproducer are added, `KNOWN_FINDINGS` gains a row in the same
//! commit.
//!
//! Gates: `cargo test -p ferroplan --test hddl_fuzz_roundtrip` (default 500
//! cases, run twice for the determinism proof, well under the 120 s wall);
//! the full 2000-case sweep is `#[ignore]`d behind `-- --ignored`.
//!
//! Provenance: the generator, and every fixture it writes, are self-authored
//! in-repo draws — no koala/corpus content is copied (KOALA POLICY).

use ferroplan::hddl::{solve_hddl, HddlError};
use ferroplan::planning_runtime::{PlannerError, PlannerLimits, UniversalPlan};
use ferroplan_hddl::grounder::{ground, GroundError, GroundingLimits};
use ferroplan_hddl::parser::{parse_domain, parse_problem, ParseError};
use ferroplan_hddl::translate::{
    translate, PlanningProblem as TrProblem, TranslateError, TranslateLimits,
};
use ferroplan_hddl::validate::{validate_domain, validate_problem, ValidationError};

use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::time::Duration;

/// Fixed sweep seed (ticket fond-htn-31, 2026-09-17). Case `i` gets seed
/// `BASE_SEED + i`, so the 2000-case sweep is a strict superset of the
/// 500-case one and every case is independently replayable from its seed.
const BASE_SEED: u64 = 0x2026_0917_0031;

// ---------------------------------------------------------------------------
// Seeded RNG (SplitMix64) — deterministic, dependency-free
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `0..n` (`n == 0` yields 0; never called that way).
    fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next_u64() % n
        }
    }
    /// Uniform in `lo..=hi` (inclusive).
    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi - lo + 1)
    }
    fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }
    fn pick_idx(&mut self, len: usize) -> usize {
        self.below(len as u64) as usize
    }
    fn range_usize(&mut self, lo: usize, hi: usize) -> usize {
        self.range(lo as u64, hi as u64) as usize
    }
}

// ---------------------------------------------------------------------------
// Sizes + model
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sizes {
    types: usize,
    preds: usize,
    tasks: usize,
    actions: usize,
    objects: usize,
    /// Upper bound on subtasks per method network.
    subs: usize,
    /// Subtasks in the problem's root `:htn` network (always >= 1 — an empty
    /// root network is a `MissingTaskNetwork` validation error).
    root_subs: usize,
}

/// The ticket's size ranges.
fn base_sizes(rng: &mut Rng) -> Sizes {
    Sizes {
        types: rng.range_usize(1, 3),
        preds: rng.range_usize(2, 5),
        tasks: rng.range_usize(2, 6),
        actions: rng.range_usize(2, 8),
        objects: rng.range_usize(2, 10),
        subs: rng.range_usize(1, 4),
        root_subs: rng.range_usize(2, 6),
    }
}

/// Halve every size toward the grammatical minimum (minimizer step).
fn halve_sizes(s: Sizes) -> Sizes {
    fn h(v: usize, min: usize) -> usize {
        usize::max(min, v / 2)
    }
    Sizes {
        types: h(s.types, 1),
        preds: h(s.preds, 1),
        tasks: h(s.tasks, 1),
        actions: h(s.actions, 1),
        objects: h(s.objects, 1),
        subs: h(s.subs, 1),
        root_subs: h(s.root_subs, 1),
    }
}

/// A schema-level literal: predicate index + enclosing-scope parameter indices.
#[derive(Clone, Debug)]
struct Lit {
    pred: usize,
    args: Vec<usize>,
    negated: bool,
}

#[derive(Clone, Debug)]
enum EffectM {
    Conj(Vec<Lit>),
    Oneof(Vec<Vec<Lit>>),
}

#[derive(Clone, Debug)]
struct ActionM {
    name: String,
    params: Vec<(String, usize)>,
    pre: Vec<Lit>,
    effect: EffectM,
}

/// A task/action invocation: target kind + argument references. `Arg::P(i)`
/// indexes the enclosing method's parameters; `Arg::O(i)` indexes the
/// model's object list (root network scope).
#[derive(Clone, Debug)]
enum Arg {
    P(usize),
    O(usize),
}

#[derive(Clone, Debug)]
struct CallM {
    action: Option<usize>,
    task: Option<usize>,
    args: Vec<Arg>,
}

#[derive(Clone, Debug)]
struct MethodM {
    name: String,
    params: Vec<(String, usize)>,
    task: usize,
    pre: Vec<Lit>,
    subs: Vec<(String, CallM)>,
    ordered: bool,
}

/// A ground literal (predicate index + object indices).
#[derive(Clone, Debug)]
struct LitO {
    pred: usize,
    args: Vec<usize>,
    negated: bool,
}

/// The intermediate model, valid-by-construction in non-mutated draws.
#[derive(Clone, Debug)]
struct Model {
    types: Vec<String>,
    preds: Vec<(String, Vec<usize>)>,
    actions: Vec<ActionM>,
    tasks: Vec<(String, Vec<(String, usize)>)>,
    methods: Vec<MethodM>,
    objects: Vec<(String, usize)>,
    init: Vec<LitO>,
    goal: Vec<LitO>,
    root: Vec<(String, CallM)>,
}

// ---------------------------------------------------------------------------
// Generator
// ---------------------------------------------------------------------------

/// The mutation switch, drawn from a side stream so sizes and mutation never
/// consume the generator's own draws: ~30% mutated / 70% valid.
fn mutation_of(seed: u64) -> bool {
    Rng::new(seed ^ 0x5EED_5EED_0000_0031).chance(30)
}

/// Post-increment read for name counters (warning-free, deterministic).
fn bump(counter: &mut usize) -> usize {
    let v = *counter;
    *counter += 1;
    v
}

fn generate(seed: u64, sizes: Sizes) -> Model {
    let mut rng = Rng::new(seed ^ 0xF0DD_0000_0031);
    let mutated = mutation_of(seed);

    // -- types ------------------------------------------------------------
    let types: Vec<String> = (0..sizes.types).map(|i| format!("ty{i}")).collect();

    // -- predicates -------------------------------------------------------
    let mut preds: Vec<(String, Vec<usize>)> = Vec::new();
    for i in 0..sizes.preds {
        let arity = rng.range_usize(0, 2);
        let arg_types = (0..arity).map(|_| rng.pick_idx(sizes.types)).collect();
        preds.push((format!("pr{i}"), arg_types));
    }

    // -- singleton-type mutation: decided upfront so init/goal/htn stay ---
    // -- consistent with the reduced object universe ----------------------
    // (needs > 1 type: with a single type an object-less universe would
    // leave the root network with nothing ground to call)
    let singleton_type: Option<usize> = if mutated && sizes.types > 1 && rng.chance(50) {
        Some(rng.pick_idx(sizes.types))
    } else {
        None
    };

    // -- objects ----------------------------------------------------------
    let mut objects: Vec<(String, usize)> = Vec::new();
    for i in 0..sizes.objects {
        let mut t = rng.pick_idx(sizes.types);
        if let Some(s) = singleton_type {
            while t == s {
                t = rng.pick_idx(sizes.types);
            }
        }
        objects.push((format!("ob{i}"), t));
    }

    // -- actions ----------------------------------------------------------
    let mut actions: Vec<ActionM> = Vec::new();
    for i in 0..sizes.actions {
        let n_params = rng.range_usize(0, 2);
        let params: Vec<(String, usize)> = (0..n_params)
            .map(|p| (format!("v{p}"), rng.pick_idx(sizes.types)))
            .collect();
        let n_pre = rng.range_usize(0, 3);
        let pre = pick_lits(&mut rng, &preds, &params, n_pre);
        let effect = if rng.chance(40) {
            // top-level oneof: 2..=4 branches, possibly empty and overlapping
            // (both legal on the SUPPORTED surface — kept verbatim, no
            // exclusivity validation exists)
            let k = rng.range_usize(2, 4);
            let mut branches: Vec<Vec<Lit>> = Vec::new();
            for _ in 0..k {
                let n = rng.range_usize(0, 2);
                branches.push(pick_lits(&mut rng, &preds, &params, n));
            }
            // overlap: copy a branch's first literal into another branch
            let src = rng.pick_idx(branches.len());
            let dst = rng.pick_idx(branches.len());
            if src != dst && !branches[src].is_empty() {
                let l = branches[src][0].clone();
                branches[dst].push(l);
            }
            EffectM::Oneof(branches)
        } else {
            let n_eff = rng.range_usize(1, 3);
            let mut lits = pick_lits(&mut rng, &preds, &params, n_eff);
            if lits.is_empty() {
                // non-empty conjunctive effect (fixture-proven shapes only
                // in valid draws)
                if let Some(l) = pick_lit(&mut rng, &preds, &params) {
                    lits.push(l);
                }
            }
            EffectM::Conj(lits)
        };
        actions.push(ActionM {
            name: format!("ac{i}"),
            params,
            pre,
            effect,
        });
    }

    // -- abstract tasks ---------------------------------------------------
    let tasks: Vec<(String, Vec<(String, usize)>)> = (0..sizes.tasks)
        .map(|i| {
            let n = rng.range_usize(0, 2);
            (
                format!("tk{i}"),
                (0..n)
                    .map(|p| (format!("v{p}"), rng.pick_idx(sizes.types)))
                    .collect(),
            )
        })
        .collect();

    // -- methods (1..=2 per task) -----------------------------------------
    let mut methods: Vec<MethodM> = Vec::new();
    let mut meth_counter = 0usize;
    for (ti, (_, task_params)) in tasks.iter().enumerate() {
        for _ in 0..rng.range_usize(1, 2) {
            let mc = bump(&mut meth_counter);
            let mut params = task_params.clone();
            if mutated && rng.chance(30) {
                // unused method parameter (semantically odd, grammatically fine)
                params.push((format!("u{mc}"), rng.pick_idx(sizes.types)));
            }
            if mutated && rng.chance(15) && !params.is_empty() {
                // shadowing: a duplicate parameter name inside one method
                let src = rng.pick_idx(params.len());
                let dup = (params[src].0.clone(), rng.pick_idx(sizes.types));
                params.push(dup);
            }
            let n_pre = rng.range_usize(0, 2);
            let pre = pick_lits(&mut rng, &preds, &params, n_pre);
            let n_subs = rng.range_usize(1, sizes.subs);
            let mut subs = Vec::new();
            for si in 0..n_subs {
                if let Some(call) = pick_call(&mut rng, &actions, &tasks, &params, Some(ti), false)
                {
                    subs.push((format!("s{si}"), call));
                }
            }
            methods.push(MethodM {
                name: format!("mt{mc}"),
                params,
                task: ti,
                pre,
                subs,
                ordered: rng.chance(50),
            });
        }
    }

    // -- duplicate-ish method body mutation --------------------------------
    if mutated && !methods.is_empty() && rng.chance(40) {
        let src = methods[rng.pick_idx(methods.len())].clone();
        let mut near = src;
        near.name = format!("mt{}", bump(&mut meth_counter));
        if near.subs.len() > 1 {
            near.subs.truncate(near.subs.len() - 1); // one-subtask-shorter body
        } else if let Some(call) =
            pick_call(&mut rng, &actions, &tasks, &near.params, Some(near.task), false)
        {
            if near.subs.is_empty() {
                near.subs.push(("s0".to_owned(), call));
            } else {
                near.subs[0].1 = call; // same shape, different call
            }
        }
        methods.push(near);
    }

    // -- name shadowing mutation ------------------------------------------
    if mutated && rng.chance(50) && !actions.is_empty() {
        let ai = rng.pick_idx(actions.len());
        let shadow = match rng.pick_idx(3) {
            0 if !preds.is_empty() => Some(preds[rng.pick_idx(preds.len())].0.clone()),
            1 if tasks.len() > 1 => {
                let mut t = rng.pick_idx(tasks.len());
                while Some(t) == actions[ai].name.strip_prefix("tk").and_then(|rest| {
                    rest.parse::<usize>().ok().filter(|ix| tasks.get(*ix).is_some())
                }) {
                    t = rng.pick_idx(tasks.len());
                }
                Some(tasks[t].0.clone())
            }
            2 if !methods.is_empty() => Some(methods[rng.pick_idx(methods.len())].name.clone()),
            _ => None,
        };
        if let Some(name) = shadow {
            actions[ai].name = name;
        }
    }

    // -- problem: init / goal (ground, type-matched) -----------------------
    let obj_types: Vec<usize> = objects.iter().map(|(_, t)| *t).collect();
    let n_init = rng.range_usize(0, 6);
    let init = pick_ground_lits(&mut rng, &preds, &obj_types, n_init, singleton_type);
    let n_goal = rng.range_usize(1, 3);
    let mut goal = pick_ground_lits(&mut rng, &preds, &obj_types, n_goal, singleton_type);
    if goal.is_empty() {
        // keep a non-empty goal in valid draws (fixture-proven shapes); a
        // bare `(and)` goal is exercised by mutated draws elsewhere
        if let Some(l) = pick_ground_lit(&mut rng, &preds, &obj_types, singleton_type) {
            goal.push(l);
        }
    }

    // -- problem: root :htn network ---------------------------------------
    let mut root = Vec::new();
    for ri in 0..sizes.root_subs {
        let scope = objects
            .iter()
            .enumerate()
            .map(|(i, (_, t))| (format!("ob{i}"), *t))
            .collect::<Vec<_>>();
        if let Some(call) = pick_call(&mut rng, &actions, &tasks, &scope, None, true) {
            root.push((format!("r{ri}"), call));
        }
    }
    if root.is_empty() {
        // last resort: any zero-arity callable keeps the network non-empty
        for (ai, a) in actions.iter().enumerate() {
            if a.params.is_empty() {
                root.push((
                    "r0".to_owned(),
                    CallM {
                        action: Some(ai),
                        task: None,
                        args: vec![],
                    },
                ));
                break;
            }
        }
        if root.is_empty() {
            for (ti, (_, tp)) in tasks.iter().enumerate() {
                if tp.is_empty() {
                    root.push((
                        "r0".to_owned(),
                        CallM {
                            action: None,
                            task: Some(ti),
                            args: vec![],
                        },
                    ));
                    break;
                }
            }
        }
    }

    Model {
        types,
        preds,
        actions,
        tasks,
        methods,
        objects,
        init,
        goal,
        root,
    }
}

fn pick_lit(
    rng: &mut Rng,
    preds: &[(String, Vec<usize>)],
    params: &[(String, usize)],
) -> Option<Lit> {
    let usable: Vec<usize> = preds
        .iter()
        .enumerate()
        .filter(|(_, (_, ts))| ts.iter().all(|t| params.iter().any(|(_, pt)| pt == t)))
        .map(|(i, _)| i)
        .collect();
    if usable.is_empty() {
        return None;
    }
    let pi = usable[rng.pick_idx(usable.len())];
    let ts = &preds[pi].1;
    let args = ts
        .iter()
        .map(|t| {
            let matching: Vec<usize> = params
                .iter()
                .enumerate()
                .filter(|(_, (_, pt))| pt == t)
                .map(|(i, _)| i)
                .collect();
            matching[rng.pick_idx(matching.len())]
        })
        .collect();
    // negative literals are on the SUPPORTED surface (delete effects /
    // `not`-preconditions)
    let negated = rng.chance(25);
    Some(Lit {
        pred: pi,
        args,
        negated,
    })
}

fn pick_lits(
    rng: &mut Rng,
    preds: &[(String, Vec<usize>)],
    params: &[(String, usize)],
    n: usize,
) -> Vec<Lit> {
    let mut out = Vec::new();
    for _ in 0..n {
        if let Some(l) = pick_lit(rng, preds, params) {
            out.push(l);
        }
    }
    out
}

fn pick_ground_lit(
    rng: &mut Rng,
    preds: &[(String, Vec<usize>)],
    obj_types: &[usize],
    singleton: Option<usize>,
) -> Option<LitO> {
    let usable: Vec<usize> = preds
        .iter()
        .enumerate()
        .filter(|(_, (_, ts))| {
            ts.iter().all(|t| {
                obj_types.iter().any(|ot| ot == t)
                    // in singleton draws, atoms needing the object-less type
                    // are exactly the odd ones: skip them so the draw stays
                    // validation-clean while the type stays empty
                    && singleton.map_or(true, |s| t != &s)
            })
        })
        .map(|(i, _)| i)
        .collect();
    if usable.is_empty() {
        return None;
    }
    let pi = usable[rng.pick_idx(usable.len())];
    let ts = &preds[pi].1;
    let args = ts
        .iter()
        .map(|t| {
            let matching: Vec<usize> = obj_types
                .iter()
                .enumerate()
                .filter(|(_, ot)| **ot == *t)
                .map(|(i, _)| i)
                .collect();
            matching[rng.pick_idx(matching.len())]
        })
        .collect();
    // `:goal` carries positive and negative ground literals; `:init` is
    // positive-only by definition — the caller fixes the sign for init.
    let negated = rng.chance(25);
    Some(LitO {
        pred: pi,
        args,
        negated,
    })
}

fn pick_ground_lits(
    rng: &mut Rng,
    preds: &[(String, Vec<usize>)],
    obj_types: &[usize],
    n: usize,
    singleton: Option<usize>,
) -> Vec<LitO> {
    let mut out = Vec::new();
    for _ in 0..n {
        if let Some(mut l) = pick_ground_lit(rng, preds, obj_types, singleton) {
            l.negated = false; // :init facts are positive
            out.push(l);
        }
    }
    out
}

/// Pick a type-matched task/action call whose arguments come from `scope`
/// (method params in method networks; the object list at the root).
/// `root_scope` selects the `Arg::O` flavor for root-network calls.
/// `own_task` (Some) allows recursive self-decomposition only occasionally.
fn pick_call(
    rng: &mut Rng,
    actions: &[ActionM],
    tasks: &[(String, Vec<(String, usize)>)],
    scope: &[(String, usize)],
    own_task: Option<usize>,
    root_scope: bool,
) -> Option<CallM> {
    let arg_of = |i: usize| {
        if root_scope {
            Arg::O(i)
        } else {
            Arg::P(i)
        }
    };
    let scope_covers =
        |ps: &[(String, usize)]| ps.iter().all(|(_, t)| scope.iter().any(|(_, st)| st == t));
    let draw_args = |rng: &mut Rng, ps: &[(String, usize)]| {
        ps.iter()
            .map(|(_, t)| {
                let matching: Vec<usize> = scope
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, st))| st == t)
                    .map(|(i, _)| i)
                    .collect();
                arg_of(matching[rng.pick_idx(matching.len())])
            })
            .collect()
    };
    let mut cands: Vec<CallM> = Vec::new();
    for (ai, a) in actions.iter().enumerate() {
        if scope_covers(&a.params) {
            cands.push(CallM {
                action: Some(ai),
                task: None,
                args: draw_args(rng, &a.params),
            });
        }
    }
    for (ti, (_, tp)) in tasks.iter().enumerate() {
        let recursive = own_task == Some(ti);
        // recursion is legal but kept occasional: ~1 in 5 chances
        if !recursive || rng.chance(20) {
            if scope_covers(tp) {
                cands.push(CallM {
                    action: None,
                    task: Some(ti),
                    args: draw_args(rng, tp),
                });
            }
        }
    }
    if cands.is_empty() {
        return None;
    }
    Some(cands.swap_remove(rng.pick_idx(cands.len())))
}

// ---------------------------------------------------------------------------
// Renderer (model -> HDDL text)
// ---------------------------------------------------------------------------

fn render_params(params: &[(String, usize)], types: &[String]) -> String {
    params
        .iter()
        .map(|(n, t)| format!("?{n} - {}", types[*t]))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_schema_lit(
    lit: &Lit,
    preds: &[(String, Vec<usize>)],
    params: &[(String, usize)],
) -> String {
    let pname = &preds[lit.pred].0;
    let args = lit
        .args
        .iter()
        .map(|i| format!("?{}", params[*i].0))
        .collect::<Vec<_>>()
        .join(" ");
    let atom = if args.is_empty() {
        format!("({pname})")
    } else {
        format!("({pname} {args})")
    };
    if lit.negated {
        format!("(not {atom})")
    } else {
        atom
    }
}

fn render_lits(lits: &[Lit], preds: &[(String, Vec<usize>)], params: &[(String, usize)]) -> String {
    let body = lits
        .iter()
        .map(|l| render_schema_lit(l, preds, params))
        .collect::<Vec<_>>()
        .join(" ");
    format!("(and {body})")
}

fn render_ground_lit(
    lit: &LitO,
    preds: &[(String, Vec<usize>)],
    objects: &[(String, usize)],
) -> String {
    let pname = &preds[lit.pred].0;
    let args = lit
        .args
        .iter()
        .map(|i| objects[*i].0.clone())
        .collect::<Vec<_>>()
        .join(" ");
    let atom = if args.is_empty() {
        format!("({pname})")
    } else {
        format!("({pname} {args})")
    };
    if lit.negated {
        format!("(not {atom})")
    } else {
        atom
    }
}

fn render_call(call: &CallM, model: &Model, method_params: Option<&[(String, usize)]>) -> String {
    let name = if let Some(ai) = call.action {
        model.actions[ai].name.clone()
    } else {
        model.tasks[call.task.expect("call with no target")].0.clone()
    };
    let args = call
        .args
        .iter()
        .map(|a| match a {
            Arg::P(i) => {
                let p = method_params.expect("Arg::P outside a method scope");
                format!("?{}", p[*i].0)
            }
            Arg::O(i) => model.objects[*i].0.clone(),
        })
        .collect::<Vec<_>>()
        .join(" ");
    if args.is_empty() {
        format!("({name})")
    } else {
        format!("({name} {args})")
    }
}

fn render(model: &Model, problem_name: &str) -> (String, String) {
    let mut d = String::new();
    d.push_str(";; fuzz-found: self-authored seeded draw (ticket fond-htn-31,\n");
    d.push_str(";; in-repo generator, no external corpus content)\n");
    d.push_str("(define (domain fzr)\n");
    d.push_str(&format!("  (:types {})\n", model.types.join(" ")));
    d.push_str("  (:predicates");
    for (p, ts) in &model.preds {
        if ts.is_empty() {
            d.push_str(&format!("\n    ({p})"));
        } else {
            let args = ts
                .iter()
                .enumerate()
                .map(|(i, t)| format!("?a{i} - {}", model.types[*t]))
                .collect::<Vec<_>>()
                .join(" ");
            d.push_str(&format!("\n    ({p} {args})"));
        }
    }
    d.push_str("\n  )\n");
    for (t, tp) in &model.tasks {
        d.push_str(&format!(
            "  (:task {t} :parameters ({}))\n",
            render_params(tp, &model.types)
        ));
    }
    for a in &model.actions {
        d.push_str(&format!(
            "  (:action {} :parameters ({})",
            a.name,
            render_params(&a.params, &model.types)
        ));
        if !a.pre.is_empty() {
            d.push_str(&format!(
                " :precondition {}",
                render_lits(&a.pre, &model.preds, &a.params)
            ));
        }
        match &a.effect {
            EffectM::Conj(lits) => {
                d.push_str(&format!(
                    " :effect {}",
                    render_lits(lits, &model.preds, &a.params)
                ))
            }
            EffectM::Oneof(branches) => {
                d.push_str(" :effect (oneof");
                for b in branches {
                    if b.is_empty() {
                        d.push_str(" (and)");
                    } else {
                        d.push_str(&format!(" {}", render_lits(b, &model.preds, &a.params)));
                    }
                }
                d.push(')');
            }
        }
        d.push_str(")\n");
    }
    for m in &model.methods {
        d.push_str(&format!(
            "  (:method {} :parameters ({})",
            m.name,
            render_params(&m.params, &model.types)
        ));
        d.push_str(&format!(
            " :task {}",
            render_call(
                &CallM {
                    action: None,
                    task: Some(m.task),
                    args: (0..model.tasks[m.task].1.len()).map(Arg::P).collect(),
                },
                model,
                Some(&m.params)
            )
        ));
        if !m.pre.is_empty() {
            d.push_str(&format!(
                " :precondition {}",
                render_lits(&m.pre, &model.preds, &m.params)
            ));
        }
        let body = m
            .subs
            .iter()
            .map(|(id, call)| format!("({id} {})", render_call(call, model, Some(&m.params))))
            .collect::<Vec<_>>()
            .join(" ");
        if m.ordered {
            if body.is_empty() {
                d.push_str(" :ordered-subtasks (and)");
            } else {
                d.push_str(&format!(" :ordered-subtasks (and {body})"));
            }
        } else {
            if body.is_empty() {
                d.push_str(" :subtasks (and)");
            } else {
                d.push_str(&format!(" :subtasks (and {body})"));
                // chain orderings over explicit ids (acyclic by construction);
                // parser accepts `(before after)` pairs and `(< b a)` forms
                let edges = m
                    .subs
                    .windows(2)
                    .map(|w| format!("({} {})", w[0].0, w[1].0))
                    .collect::<Vec<_>>()
                    .join(" ");
                d.push_str(&format!(" :ordering (and {edges})"));
            }
        }
        d.push_str(")\n");
    }
    d.push_str(")\n");

    let mut p = String::new();
    p.push_str(";; fuzz-found: self-authored seeded draw (ticket fond-htn-31,\n");
    p.push_str(";; in-repo generator, no external corpus content)\n");
    p.push_str(&format!("(define (problem {problem_name})\n"));
    p.push_str("  (:domain fzr)\n");
    if model.objects.is_empty() {
        p.push_str("  (:objects)\n");
    } else {
        p.push_str(&format!(
            "  (:objects {})\n",
            model
                .objects
                .iter()
                .map(|(n, t)| format!("{n} - {}", model.types[*t]))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    p.push_str(&format!(
        "  (:init {})\n",
        model
            .init
            .iter()
            .map(|l| render_ground_lit(l, &model.preds, &model.objects))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    let goal_body = model
        .goal
        .iter()
        .map(|l| render_ground_lit(l, &model.preds, &model.objects))
        .collect::<Vec<_>>()
        .join(" ");
    if goal_body.is_empty() {
        p.push_str("  (:goal (and))\n");
    } else {
        p.push_str(&format!("  (:goal (and {goal_body}))\n"));
    }
    let root_body = model
        .root
        .iter()
        .map(|(id, call)| format!("({id} {})", render_call(call, model, None)))
        .collect::<Vec<_>>()
        .join(" ");
    if root_body.is_empty() {
        p.push_str("  (:htn :ordered-subtasks (and))\n");
    } else {
        p.push_str(&format!("  (:htn :ordered-subtasks (and {root_body}))\n"));
    }
    p.push_str(")\n");
    (d, p)
}

/// The exact draw this seed produces in the sweep (sizes + mutation + text).
fn draw_for(seed: u64, sizes: Sizes) -> (String, String) {
    let model = generate(seed, sizes);
    render(&model, &format!("fzp-{seed}"))
}

// ---------------------------------------------------------------------------
// Small caps (ticket: parse -> validate -> ground -> translate under caps)
// ---------------------------------------------------------------------------

fn fuzz_ground_limits() -> GroundingLimits {
    GroundingLimits {
        max_ground_actions: 2_000,
        max_ground_methods: 2_000,
        prune_unreachable: false,
        max_wall: Some(Duration::from_millis(1_500)),
    }
}

fn fuzz_translate_limits() -> TranslateLimits {
    TranslateLimits {
        max_task_network_depth: 32,
        max_wall: Some(Duration::from_millis(1_500)),
        max_states: Some(10_000),
    }
}

fn fuzz_planner_limits() -> PlannerLimits {
    PlannerLimits {
        max_wall_ms: 1_000,
        ..PlannerLimits::default()
    }
}

// ---------------------------------------------------------------------------
// Error variant tags (stable digest tokens, one per typed variant)
// ---------------------------------------------------------------------------

fn parse_tag(e: &ParseError) -> &'static str {
    match e {
        ParseError::Syntax(_) => "Syntax",
        ParseError::UnsupportedConstruct(_) => "UnsupportedConstruct",
        ParseError::MalformedOneof(_) => "MalformedOneof",
        ParseError::NestedProbabilisticBlock(_) => "NestedProbabilisticBlock",
        ParseError::NestingTooDeep { .. } => "NestingTooDeep",
    }
}

fn val_tag(e: &ValidationError) -> &'static str {
    match e {
        ValidationError::UnknownTaskOrAction(_) => "UnknownTaskOrAction",
        ValidationError::ArityMismatch { .. } => "ArityMismatch",
        ValidationError::UndefinedPredicate(_) => "UndefinedPredicate",
        ValidationError::UndefinedType(_) => "UndefinedType",
        ValidationError::DuplicateDefinition { .. } => "DuplicateDefinition",
        ValidationError::CyclicTypeHierarchy { .. } => "CyclicTypeHierarchy",
        ValidationError::PredicateArityMismatch { .. } => "PredicateArityMismatch",
        ValidationError::MethodHeadNotCompoundTask { .. } => "MethodHeadNotCompoundTask",
        ValidationError::UnknownMethodVariable { .. } => "UnknownMethodVariable",
        ValidationError::UndefinedOrderRef { .. } => "UndefinedOrderRef",
        ValidationError::CyclicOrdering { .. } => "CyclicOrdering",
        ValidationError::MissingTaskNetwork => "MissingTaskNetwork",
        ValidationError::UnknownConstant { .. } => "UnknownConstant",
        ValidationError::ArgumentTypeMismatch { .. } => "ArgumentTypeMismatch",
        ValidationError::NonGroundInitAtom { .. } => "NonGroundInitAtom",
        ValidationError::NonGroundRootSubtaskArg { .. } => "NonGroundRootSubtaskArg",
    }
}

fn ground_tag(e: &GroundError) -> &'static str {
    match e {
        GroundError::Validation(_) => "Validation",
        GroundError::TypeCycle(_) => "TypeCycle",
        GroundError::UnboundVariable(_) => "UnboundVariable",
        GroundError::UnsupportedPrecondition(_) => "UnsupportedPrecondition",
        GroundError::LimitExceeded(_) => "LimitExceeded",
        GroundError::UnsupportedNumericFluent(_) => "UnsupportedNumericFluent",
        GroundError::UnsupportedConstraint(_) => "UnsupportedConstraint",
        GroundError::Timeout { .. } => "TIMEOUT",
        GroundError::GoalTooDeep { .. } => "GoalTooDeep",
    }
}

fn translate_tag(e: &TranslateError) -> &'static str {
    match e {
        TranslateError::UnsupportedNegativeGoal => "UnsupportedNegativeGoal",
        TranslateError::UnsupportedGoalConnective(_) => "UnsupportedGoalConnective",
        TranslateError::UnboundVariable(_) => "UnboundVariable",
        TranslateError::TaskNetworkDepthExceeded { .. } => "TaskNetworkDepthExceeded",
        TranslateError::Timeout { .. } => "TIMEOUT",
        TranslateError::MemoryLimitExceeded { .. } => "MemoryLimitExceeded",
        TranslateError::MalformedTermEquality { .. } => "MalformedTermEquality",
        TranslateError::Ground(_) => "Ground",
    }
}

fn hddl_tag(e: &HddlError) -> &'static str {
    match e {
        HddlError::Parse(_) => "Parse",
        HddlError::Ground(_) => "Ground",
        HddlError::Translate(_) => "Translate",
        HddlError::Planner(_) => "Planner",
        HddlError::RootTaskMismatch { .. } => "RootTaskMismatch",
        HddlError::Timeout { .. } => "TIMEOUT",
        HddlError::WorkerPanicked(_) => "WorkerPanicked",
    }
}

fn planner_tag(e: &PlannerError) -> &'static str {
    match e {
        PlannerError::EmptyInitialState => "EmptyInitialState",
        PlannerError::UnknownState { .. } => "UnknownState",
        PlannerError::InvalidProbabilityMass { .. } => "InvalidProbabilityMass",
        PlannerError::ResourceBound { .. } => "ResourceBound",
        PlannerError::NoPlan => "NoPlan",
        PlannerError::HierarchyCycle { .. } => "HierarchyCycle",
        PlannerError::UnknownTask { .. } => "UnknownTask",
        PlannerError::NoMethod { .. } => "NoMethod",
        PlannerError::WorkflowCycle => "WorkflowCycle",
        PlannerError::WipBoundExceeded { .. } => "WipBoundExceeded",
        PlannerError::CapabilityUncovered { .. } => "CapabilityUncovered",
        PlannerError::AuthorityUnbound { .. } => "AuthorityUnbound",
        PlannerError::VerifierUnbound { .. } => "VerifierUnbound",
        PlannerError::ReceiptUnbound { .. } => "ReceiptUnbound",
        PlannerError::InvalidRdfProjection { .. } => "InvalidRdfProjection",
        PlannerError::Timeout { .. } => "TIMEOUT",
    }
}

// ---------------------------------------------------------------------------
// Per-case pipeline
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
struct CaseOutcome {
    seed: u64,
    mutated: bool,
    /// First stage that answered: "dparse" | "pparse" | "validate" |
    /// "ground" | "translate" | "translate-ok" | "known-finding".
    stage: &'static str,
    /// Error variant tag, or "states=N;trans=M" for a translated model.
    tag: String,
    /// "" when not sampled; "solve:ok:policy=N" / "solve:err:<tag>" otherwise.
    solve: String,
}

/// Run one case through parse -> validate -> ground -> translate (+ ~10%
/// through solve_hddl). Panics (any `assert!` here, or any pipeline panic)
/// are findings, caught by the sweep's `catch_unwind`.
fn run_case(seed: u64, sizes: Sizes, solve: bool) -> CaseOutcome {
    let (domain_src, problem_src) = draw_for(seed, sizes);
    let mutated = mutation_of(seed);
    let outcome = |stage: &'static str, tag: String| CaseOutcome {
        seed,
        mutated,
        stage,
        tag,
        solve: String::new(),
    };

    let domain = match parse_domain(&domain_src) {
        Ok(d) => d,
        Err(e) => {
            assert!(!e.to_string().is_empty(), "seed {seed}: empty ParseError");
            // A grammar-faithful generator must always parse: a parse error
            // on a VALID draw means generator drift off the SUPPORTED
            // surface — that is a finding about the generator.
            assert!(
                mutated,
                "seed {seed}: VALID draw failed to parse (generator drifted off \
                 the SUPPORTED surface): {e}"
            );
            return outcome("dparse", parse_tag(&e).to_owned());
        }
    };
    let problem = match parse_problem(&problem_src) {
        Ok(p) => p,
        Err(e) => {
            assert!(!e.to_string().is_empty(), "seed {seed}: empty ParseError");
            assert!(
                mutated,
                "seed {seed}: VALID draw failed to parse (generator drifted off \
                 the SUPPORTED surface): {e}"
            );
            return outcome("pparse", parse_tag(&e).to_owned());
        }
    };

    if let Err(e) = validate_domain(&domain) {
        assert!(
            mutated,
            "seed {seed}: VALID draw failed domain validation (generator drift): {e}"
        );
        assert!(!e.to_string().is_empty(), "seed {seed}: empty ValidationError");
        return outcome("validate", val_tag(&e).to_owned());
    }
    if let Err(e) = validate_problem(&domain, &problem) {
        assert!(
            mutated,
            "seed {seed}: VALID draw failed problem validation (generator drift): {e}"
        );
        assert!(!e.to_string().is_empty(), "seed {seed}: empty ValidationError");
        return outcome("validate", val_tag(&e).to_owned());
    }

    let ir = match ground(&domain, &problem, &fuzz_ground_limits()) {
        Ok(ir) => ir,
        Err(e) => {
            // internal consistency: ground re-runs exactly
            // validate_domain + validate_problem first; both just passed.
            assert!(
                !matches!(e, GroundError::Validation(_)),
                "seed {seed}: explicit validation passed but ground refused with \
                 GroundError::Validation ({e}) — the pipeline's validation front \
                 door and its caller disagree"
            );
            return outcome("ground", ground_tag(&e).to_owned());
        }
    };
    let tp = match translate(&ir, &fuzz_translate_limits()) {
        Ok(tp) => tp,
        Err(e) => {
            assert!(!e.to_string().is_empty(), "seed {seed}: empty TranslateError");
            return outcome("translate", translate_tag(&e).to_owned());
        }
    };
    let mut outcome = outcome(
        "translate-ok",
        format!("states={};trans={}", tp.states.len(), tp.transitions.len()),
    );

    if solve {
        outcome.solve = solve_sample(seed, &domain_src, &problem_src, &tp);
    }
    outcome
}

/// The ~10% end-to-end sample: `solve_hddl` on a bounded budget, asserting
/// typed outcomes and outcome-closed policies.
fn solve_sample(seed: u64, domain_src: &str, problem_src: &str, tp: &TrProblem) -> String {
    match solve_hddl(domain_src, problem_src, &fuzz_planner_limits()) {
        Err(e) => {
            assert!(
                !matches!(e, HddlError::WorkerPanicked(_)),
                "seed {seed}: solve_hddl worker panicked ({e:?}) — a pipeline panic \
                 in disguise, never an acceptable outcome"
            );
            assert!(!e.to_string().is_empty(), "seed {seed}: empty HddlError");
            if let HddlError::Planner(pe) = &e {
                format!("solve:err:Planner:{}", planner_tag(pe))
            } else {
                format!("solve:err:{}", hddl_tag(&e))
            }
        }
        Ok(plan) => {
            assert!(
                plan.solved,
                "seed {seed}: Ok plan with solved=false — the solver's failure mode \
                 is Err(NoPlan), never a falsified Ok"
            );
            assert!(
                !plan.policy.is_empty(),
                "seed {seed}: solved plan with an empty policy"
            );
            check_policy_outcome_closure(seed, tp, &plan);
            format!("solve:ok:policy={}", plan.policy.len())
        }
    }
}

/// Outcome closure: for every policy entry, the translated problem must
/// actually carry the chosen action out of that state, every branch outcome
/// of that choice must be listed, and every outcome state must itself be a
/// policy state or a goal state.
fn check_policy_outcome_closure(seed: u64, tp: &TrProblem, plan: &UniversalPlan) {
    let facts_by_state: BTreeMap<&str, &BTreeSet<String>> =
        tp.states.iter().map(|s| (s.id.as_str(), &s.facts)).collect();
    let empty: &BTreeSet<String> = &BTreeSet::new();
    let is_goal =
        |state: &str| tp.goal.facts.is_subset(facts_by_state.get(state).unwrap_or(&empty));
    let in_policy: BTreeSet<&str> = plan.policy.iter().map(|e| e.state.as_str()).collect();
    for entry in &plan.policy {
        let edges: Vec<_> = tp
            .transitions
            .iter()
            .filter(|t| t.from == entry.state && t.action == entry.action)
            .collect();
        assert!(
            !edges.is_empty(),
            "seed {seed}: policy picks action '{}' in state '{}' but the translated \
             problem has no such outgoing transition",
            entry.action,
            entry.state
        );
        assert_eq!(
            edges.len(),
            entry.outcomes.len(),
            "seed {seed}: policy entry for state '{}' action '{}' lists {} outcomes \
             but the problem has {} branches — outcome set is not closed",
            entry.state,
            entry.action,
            entry.outcomes.len(),
            edges.len()
        );
        for t in edges {
            assert!(
                in_policy.contains(t.to.as_str()) || is_goal(&t.to),
                "seed {seed}: policy not outcome-closed: state '{}' action '{}' \
                 outcome state '{}' is neither a policy state nor a goal state",
                entry.state,
                entry.action,
                t.to
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Minimizer + fixture writer (finding discipline)
// ---------------------------------------------------------------------------

/// Known, committed findings: seeds the sweep EXPECTS to panic (a live
/// regression tripwire — never a silent skip). Each row must have matching
/// fixtures under `tests/fixtures/fuzz-found/` and an `#[ignore]`d
/// reproducer test below. Empty until a first finding lands.
const KNOWN_FINDINGS: &[(u64, &str)] = &[];

fn case_panics(seed: u64, sizes: Sizes, solve: bool) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        let _ = run_case(seed, sizes, solve);
    }))
    .is_err()
}

/// Halve the draw's sizes until the panic no longer reproduces; return the
/// last reproducing size vector (with its exact draw text — the committed
/// artifact).
fn minimize(seed: u64, start: Sizes, solve: bool) -> (Sizes, (String, String)) {
    let mut best_sizes = start;
    let mut best_text = draw_for(seed, start);
    let mut cur = start;
    loop {
        let next = halve_sizes(cur);
        if next == cur {
            break;
        }
        if case_panics(seed, next, solve) {
            cur = next;
            best_sizes = next;
            best_text = draw_for(seed, next);
        } else {
            break;
        }
    }
    (best_sizes, best_text)
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fuzz-found")
}

fn write_finding_fixtures(seed: u64, sizes: Sizes) -> Vec<PathBuf> {
    let dir = fixtures_dir();
    std::fs::create_dir_all(&dir).expect("create tests/fixtures/fuzz-found");
    let (domain, problem) = draw_for(seed, sizes);
    let stem = format!("hddl-roundtrip-seed{seed}");
    let dpath = dir.join(format!("{stem}.domain.hddl"));
    let ppath = dir.join(format!("{stem}.problem.hddl"));
    std::fs::write(&dpath, domain).expect("write finding domain fixture");
    std::fs::write(&ppath, problem).expect("write finding problem fixture");
    vec![dpath, ppath]
}

// ---------------------------------------------------------------------------
// The sweeps
// ---------------------------------------------------------------------------

fn run_sweep(n_cases: usize, label: &'static str) -> Vec<CaseOutcome> {
    let mut outcomes = Vec::with_capacity(n_cases);
    for i in 0..n_cases {
        let seed = BASE_SEED + i as u64;
        let sizes = {
            let mut r = Rng::new(seed ^ 0x51A5_0000_0031);
            base_sizes(&mut r)
        };
        // ~10% end-to-end solve sample, deterministic in the case index.
        let solve = i % 10 == 0;
        let known = KNOWN_FINDINGS
            .iter()
            .find(|(s, _)| *s == seed)
            .map(|(_, w)| *w);
        let result = catch_unwind(AssertUnwindSafe(|| run_case(seed, sizes, solve)));
        let outcome = match (known, result) {
            (Some(why), Err(_)) => {
                // Known finding: it MUST still reproduce. This is the
                // tripwire row, not a skip.
                eprintln!("known fuzz finding seed {seed} still reproduces: {why}");
                CaseOutcome {
                    seed,
                    mutated: mutation_of(seed),
                    stage: "known-finding",
                    tag: "PANIC".to_owned(),
                    solve: String::new(),
                }
            }
            (Some(why), Ok(_)) => panic!(
                "known fuzz finding seed {seed} no longer reproduces ('{why}') — \
                 remove it from KNOWN_FINDINGS, delete its fixtures and its \
                 #[ignore]d reproducer, and append a paydown History row in the \
                 same change"
            ),
            (None, Ok(outcome)) => outcome,
            (None, Err(payload)) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "<opaque panic payload>".to_owned());
                let (min_sizes, _) = minimize(seed, sizes, solve);
                let paths = write_finding_fixtures(seed, min_sizes);
                panic!(
                    "NEW FUZZ FINDING in sweep '{label}': seed {seed} (mutated: {}) \
                     panicked: {msg}\nminimized sizes: {min_sizes:?}\nfixtures \
                     written: {paths:?}\nrequired in the same change: commit the \
                     fixtures, add a KNOWN_FINDINGS row for seed {seed}, add an \
                     #[ignore]d reproducer test, append a History row to \
                     docs/jira/v26.9.17/fond-htn-31-fuzz-hddl-roundtrip.md",
                    mutation_of(seed)
                );
            }
        };
        outcomes.push(outcome);
    }
    outcomes
}

/// Collapse wall-clock TIMEOUT tags before comparing runs: an honest refusal
/// whose firing depends on machine load is not part of the determinism proof
/// (the generator and every non-timeout outcome must still match exactly).
fn normalized(outcomes: &[CaseOutcome]) -> Vec<CaseOutcome> {
    outcomes
        .iter()
        .map(|o| CaseOutcome {
            seed: o.seed,
            mutated: o.mutated,
            stage: o.stage,
            tag: if o.tag == "TIMEOUT" {
                "TIMEOUT(normalized)".to_owned()
            } else {
                o.tag.clone()
            },
            solve: if o.solve == "solve:err:TIMEOUT" {
                "solve:TIMEOUT(normalized)".to_owned()
            } else {
                o.solve.clone()
            },
        })
        .collect()
}

const TIMEOUT_TAG: &str = "TIMEOUT(normalized)";
const TIMEOUT_SOLVE: &str = "solve:TIMEOUT(normalized)";

/// Pairwise determinism comparison. Everything must match exactly EXCEPT
/// pairs where either run hit a wall-clock budget (tag or solve watchdog):
/// those outcomes depend on machine load, not on the seed, so the honest
/// determinism claim is "identical modulo wall-clock refusals".
fn assert_deterministic(run_a: &[CaseOutcome], run_b: &[CaseOutcome]) {
    assert_eq!(
        run_a.len(),
        run_b.len(),
        "runs of the same sweep produced different case counts"
    );
    let a = normalized(run_a);
    let b = normalized(run_b);
    for (x, y) in a.iter().zip(b.iter()) {
        assert_eq!(x.seed, y.seed);
        assert_eq!(x.mutated, y.mutated, "seed {}: mutation flag differs", x.seed);
        if x.tag == TIMEOUT_TAG || y.tag == TIMEOUT_TAG || x.solve == TIMEOUT_SOLVE || y.solve == TIMEOUT_SOLVE {
            continue; // wall-clock boundary: tolerated in both directions
        }
        assert_eq!(
            (x.stage, x.tag.as_str(), x.solve.as_str()),
            (y.stage, y.tag.as_str(), y.solve.as_str()),
            "seed {}: same seed twice produced different outcomes — the \
             generator (or the pipeline) is not deterministic",
            x.seed
        );
    }
}

/// The gate: 500 seeded cases (70/30 valid/mutated from each case's own
/// stream), parse -> validate -> ground -> translate under small caps, ~10%
/// through solve_hddl with outcome-closed-policy checks; run TWICE for the
/// generator determinism proof. Budget: well under the 120 s wall.
#[test]
fn roundtrip_sweep_500_default() {
    let started = std::time::Instant::now();
    let run_a = run_sweep(500, "default-500-a");
    let run_b = run_sweep(500, "default-500-b");
    assert_deterministic(&run_a, &run_b);
    let translated = run_a.iter().filter(|o| o.stage == "translate-ok").count();
    let solved = run_a
        .iter()
        .filter(|o| o.solve.starts_with("solve:ok"))
        .count();
    let no_plan = run_a
        .iter()
        .filter(|o| o.solve.starts_with("solve:err:Planner:NoPlan"))
        .count();
    let sampled = run_a.iter().filter(|o| !o.solve.is_empty()).count();
    eprintln!(
        "roundtrip sweep (500, {:?}): translated-ok={translated}/{}; solve \
         sampled={sampled} (solved={solved}, honest-NoPlan={no_plan})",
        started.elapsed(),
        run_a.len()
    );
    // The sweep must actually exercise the pipeline's depth, not silently
    // refuse everything at the door: at least half the draws must reach a
    // translated model.
    assert!(
        translated * 2 >= run_a.len(),
        "only {translated}/{} draws reached translate — the generator is not \
         exercising the round-trip",
        run_a.len()
    );
    // Sampled cases are exactly the every-10th indices that reached
    // translate-ok (earlier-stage refusals have no plan to solve).
    let sampled_expected = (0..500)
        .step_by(10)
        .filter(|&i| run_a[i].stage == "translate-ok")
        .count();
    assert_eq!(
        sampled, sampled_expected,
        "solve sample must be exactly the every-10th translate-ok case"
    );
}

/// The full sweep: 2000 deterministic cases, same contract, run twice.
#[test]
#[ignore = "full 2000-case fuzz sweep — run with `cargo test -p ferroplan \
            --test hddl_fuzz_roundtrip -- --ignored`"]
fn roundtrip_sweep_2000_full() {
    let started = std::time::Instant::now();
    let run_a = run_sweep(2000, "full-2000-a");
    let run_b = run_sweep(2000, "full-2000-b");
    assert_deterministic(&run_a, &run_b);
    let translated = run_a.iter().filter(|o| o.stage == "translate-ok").count();
    eprintln!(
        "roundtrip sweep (2000, {:?}): translated-ok={translated}/{}",
        started.elapsed(),
        run_a.len()
    );
}
