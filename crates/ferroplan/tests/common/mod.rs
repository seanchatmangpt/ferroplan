//! The seeded HDDL fuzz generator, extracted verbatim from
//! `tests/hddl_fuzz_roundtrip.rs` (ticket fond-htn-31) so the differential
//! fuzz (ticket fond-htn-61) can reuse the exact same machinery instead of
//! duplicating it. Same seeds, same draws: `generate`/`draw_for` are the
//! untouched roundtrip path; `generate_valid`/`draw_valid` add a
//! mutation-hard-off entry point for draws that must be valid by
//! construction.
//!
//! Provenance: self-authored in-repo seeded draws — no koala/corpus content
//! is copied (KOALA POLICY).

#![allow(dead_code)] // shared machinery: each consumer target uses a subset

use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// Seeded RNG (SplitMix64) — deterministic, dependency-free
// ---------------------------------------------------------------------------

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `0..n` (`n == 0` yields 0; never called that way).
    pub fn below(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next_u64() % n
        }
    }
    /// Uniform in `lo..=hi` (inclusive).
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi - lo + 1)
    }
    pub fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }
    pub fn pick_idx(&mut self, len: usize) -> usize {
        self.below(len as u64) as usize
    }
    pub fn range_usize(&mut self, lo: usize, hi: usize) -> usize {
        self.range(lo as u64, hi as u64) as usize
    }
}

// ---------------------------------------------------------------------------
// Sizes + model
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sizes {
    pub types: usize,
    pub preds: usize,
    pub tasks: usize,
    pub actions: usize,
    pub objects: usize,
    /// Upper bound on subtasks per method network.
    pub subs: usize,
    /// Subtasks in the problem's root `:htn` network (always >= 1 — an empty
    /// root network is a `MissingTaskNetwork` validation error).
    pub root_subs: usize,
}

/// The ticket's size ranges.
pub fn base_sizes(rng: &mut Rng) -> Sizes {
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
pub fn halve_sizes(s: Sizes) -> Sizes {
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
pub struct Model {
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
pub fn mutation_of(seed: u64) -> bool {
    Rng::new(seed ^ 0x5EED_5EED_0000_0031).chance(30)
}

/// Post-increment read for name counters (warning-free, deterministic).
fn bump(counter: &mut usize) -> usize {
    let v = *counter;
    *counter += 1;
    v
}

pub fn generate(seed: u64, sizes: Sizes) -> Model {
    generate_inner(seed, sizes, true)
}

/// Forced-VALID draw: the mutation switch is hard-off, so the draw is
/// valid-by-construction (differential fuzz, ticket fond-htn-61). The root
/// task network is clamped to 1-2 subtasks: every root branch must ground
/// for the instance to be alive at all, and the external oracle's grounder
/// emits an empty, unclassifiable instance when the whole network grounds
/// to nothing.
pub fn generate_valid(seed: u64, sizes: Sizes) -> Model {
    let clamped = Sizes {
        root_subs: sizes.root_subs.clamp(1, 2),
        ..sizes
    };
    generate_inner(seed, clamped, false)
}

fn generate_inner(seed: u64, sizes: Sizes, allow_mutation: bool) -> Model {
    let mut rng = Rng::new(seed ^ 0xF0DD_0000_0031);
    let mutated = allow_mutation && mutation_of(seed);

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
        // actions[0] stays precondition-free: it is the guaranteed-live
        // primitive leaf for the open first methods below (a random
        // precondition pool starves the reachable fragment and the external
        // oracle grounds the whole instance to nothing)
        let n_pre = if i == 0 { 0 } else { rng.range_usize(0, 3) };
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
    // The FIRST method of every task is open: no precondition, subtasks
    // drawn only from precondition-free actions (falling back to an empty
    // body when none is type-coverable). Without it, random preconditions
    // starve the reachable fragment — every root branch dead-ends and the
    // instance grounds to nothing.
    let mut methods: Vec<MethodM> = Vec::new();
    let mut meth_counter = 0usize;
    for (ti, (_, task_params)) in tasks.iter().enumerate() {
        let mut first_of_task = true;
        for _ in 0..rng.range_usize(1, 2) {
            let open = first_of_task;
            first_of_task = false;
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
            let n_pre = if open { 0 } else { rng.range_usize(0, 2) };
            let pre = pick_lits(&mut rng, &preds, &params, n_pre);
            let n_subs = if open { 1 } else { rng.range_usize(1, sizes.subs) };
            let mut subs = Vec::new();
            for si in 0..n_subs {
                let call = pick_call(
                    &mut rng,
                    &actions,
                    &tasks,
                    &params,
                    Some(ti),
                    false,
                    open,
                )
                .or_else(|| {
                    if open {
                        pick_call(&mut rng, &actions, &tasks, &params, Some(ti), false, false)
                    } else {
                        None
                    }
                });
                if let Some(call) = call {
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
            pick_call(&mut rng, &actions, &tasks, &near.params, Some(near.task), false, false)
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
    let init = dense_init(&mut rng, &preds, &obj_types, singleton_type);

    // Goal atoms are drawn PROVABLE-in-principle (added by some positive
    // effect, or already true at init) except for a ~10% dead-goal share
    // kept for honest NOSOLUTION coverage. A static-false goal atom makes
    // the external oracle's grounder answer "Goal is unreachable" with an
    // EMPTY instance file — its serializer then crashes instead of the
    // pipeline emitting a verdict.
    let effected: BTreeSet<usize> = {
        let mut s = BTreeSet::new();
        for a in &actions {
            match &a.effect {
                EffectM::Conj(lits) => {
                    for l in lits.iter().filter(|l| !l.negated) {
                        s.insert(l.pred);
                    }
                }
                EffectM::Oneof(branches) => {
                    for b in branches {
                        for l in b.iter().filter(|l| !l.negated) {
                            s.insert(l.pred);
                        }
                    }
                }
            }
        }
        s
    };
    let dead_goal = rng.chance(10);
    let mut goal: Vec<LitO> = if dead_goal {
        let n_goal = rng.range_usize(1, 3);
        pick_ground_lits(&mut rng, &preds, &obj_types, n_goal, singleton_type)
    } else {
        let mut cands: Vec<LitO> = Vec::new();
        for (pi, (_, ts)) in preds.iter().enumerate() {
            let usable = ts.iter().all(|t| obj_types.iter().any(|ot| ot == t))
                && singleton_type.map_or(true, |s| !ts.iter().any(|t| t == &s));
            if !usable {
                continue;
            }
            if effected.contains(&pi) {
                for args in atom_combos(ts, &obj_types, 40) {
                    cands.push(LitO {
                        pred: pi,
                        args,
                        negated: false,
                    });
                }
            } else {
                // static predicate: only the exact init-true atoms are
                // provable
                for l in init.iter().filter(|l| l.pred == pi) {
                    cands.push(LitO {
                        pred: pi,
                        args: l.args.clone(),
                        negated: false,
                    });
                }
            }
        }
        let n_goal = rng.range_usize(1, 3);
        let mut g: Vec<LitO> = Vec::new();
        for _ in 0..n_goal {
            if cands.is_empty() {
                break;
            }
            let idx = rng.pick_idx(cands.len());
            let mut atom = cands.swap_remove(idx);
            // negative goals only over deletable (effected) predicates;
            // `:init` facts are positive, but a goal may demand absence
            if effected.contains(&atom.pred) && rng.chance(20) {
                atom.negated = true;
            }
            g.push(atom);
        }
        if g.is_empty() {
            if let Some(l) = init.first() {
                g.push(LitO {
                    pred: l.pred,
                    args: l.args.clone(),
                    negated: false,
                });
            }
        }
        g
    };

    // -- problem: root :htn network ---------------------------------------
    let mut root = Vec::new();
    for ri in 0..sizes.root_subs {
        let scope = objects
            .iter()
            .enumerate()
            .map(|(i, (_, t))| (format!("ob{i}"), *t))
            .collect::<Vec<_>>();
        if let Some(call) = pick_call(&mut rng, &actions, &tasks, &scope, None, true, false) {
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

/// All argument-index combinations for one predicate's type signature over
/// `obj_types` (bounded by `cap`, deterministic truncation), or an empty
/// vector when some argument type is carried by no object.
fn atom_combos(ts: &[usize], obj_types: &[usize], cap: usize) -> Vec<Vec<usize>> {
    if ts.iter().any(|t| obj_types.iter().all(|ot| ot != t)) {
        return Vec::new();
    }
    let mut combos: Vec<Vec<usize>> = vec![vec![]];
    for t in ts {
        let matching: Vec<usize> = obj_types
            .iter()
            .enumerate()
            .filter(|(_, ot)| **ot == *t)
            .map(|(i, _)| i)
            .collect();
        let mut next = Vec::new();
        for c in &combos {
            for &m in &matching {
                let mut c2 = c.clone();
                c2.push(m);
                next.push(c2);
            }
        }
        combos = next;
        if combos.len() > cap {
            combos.truncate(cap);
        }
    }
    combos
}

/// Dense initial state: sample ~30% of the type-consistent positive ground
/// atoms (bounded per predicate) so method/action preconditions are widely
/// satisfiable at the initial state. A sparse random init dead-ends most
/// decomposition paths; the external oracle's grounder then emits an EMPTY
/// instance (initial abstract task -1, unclassifiable by its serializer)
/// instead of a verdict.
fn dense_init(
    rng: &mut Rng,
    preds: &[(String, Vec<usize>)],
    obj_types: &[usize],
    singleton: Option<usize>,
) -> Vec<LitO> {
    let mut atoms: Vec<LitO> = Vec::new();
    for (pi, (_, ts)) in preds.iter().enumerate() {
        if singleton.map_or(false, |s| ts.iter().any(|t| t == &s)) {
            continue; // singleton-type mutation keeps this type object-less
        }
        for args in atom_combos(ts, obj_types, 40) {
            if rng.chance(30) {
                atoms.push(LitO {
                    pred: pi,
                    args,
                    negated: false,
                });
            }
        }
        if atoms.len() > 100 {
            break;
        }
    }
    atoms
}

/// Pick a type-matched task/action call whose arguments come from `scope`
/// (method params in method networks; the object list at the root).
/// `root_scope` selects the `Arg::O` flavor for root-network calls.
/// `own_task` (Some) allows recursive self-decomposition only occasionally.
/// `no_pre_only` restricts action candidates to precondition-free actions
/// and drops abstract-task candidates entirely (the open-method path: the
/// decomposition branch must stay reachable from the initial state).
fn pick_call(
    rng: &mut Rng,
    actions: &[ActionM],
    tasks: &[(String, Vec<(String, usize)>)],
    scope: &[(String, usize)],
    own_task: Option<usize>,
    root_scope: bool,
    no_pre_only: bool,
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
        if no_pre_only && !a.pre.is_empty() {
            continue;
        }
        if scope_covers(&a.params) {
            cands.push(CallM {
                action: Some(ai),
                task: None,
                args: draw_args(rng, &a.params),
            });
        }
    }
    if !no_pre_only {
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

pub const PROVENANCE_FUZZ_T31: &[&str] = &[
    ";; fuzz-found: self-authored seeded draw (ticket fond-htn-31,",
    ";; in-repo generator, no external corpus content)",
];

/// Provenance header for the differential-fuzz VALID draws (ticket 61).
pub const PROVENANCE_DIFF_T61: &[&str] = &[
    ";; differential-fuzz: self-authored seeded VALID draw (ticket fond-htn-61,",
    ";; in-repo generator, no external corpus content)",
];

pub fn render(model: &Model, problem_name: &str) -> (String, String) {
    render_with_provenance(model, problem_name, PROVENANCE_FUZZ_T31)
}

/// [`render`] with an explicit provenance header (each entry is one comment
/// line; the trailing newline is appended here).
pub fn render_with_provenance(
    model: &Model,
    problem_name: &str,
    header: &[&str],
) -> (String, String) {
    let mut d = String::new();
    for line in header {
        d.push_str(line);
        d.push('\n');
    }
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
                // every edge carries the explicit `<` operator — the form both
                // consumers accept (ferroplan's `parse_order_edges` takes
                // `(< a b)`/`(a < b)`/bare adjacency pairs; the external
                // pandaPI oracle requires `<`)
                let edges = m
                    .subs
                    .windows(2)
                    .map(|w| format!("(< {} {})", w[0].0, w[1].0))
                    .collect::<Vec<_>>()
                    .join(" ");
                d.push_str(&format!(" :ordering (and {edges})"));
            }
        }
        d.push_str(")\n");
    }
    d.push_str(")\n");

    let mut p = String::new();
    for line in header {
        p.push_str(line);
        p.push('\n');
    }
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
pub fn draw_for(seed: u64, sizes: Sizes) -> (String, String) {
    let model = generate(seed, sizes);
    render(&model, &format!("fzp-{seed}"))
}

/// The exact size vector a sweep derives for `seed` (side stream, so sizes
/// and mutation never consume the generator's own draws).
pub fn sizes_for(seed: u64) -> Sizes {
    let mut r = Rng::new(seed ^ 0x51A5_0000_0031);
    base_sizes(&mut r)
}

/// The forced-VALID draw (mutation hard-off) with ticket-61 provenance,
/// for the differential fuzz.
pub fn draw_valid(seed: u64, sizes: Sizes) -> (String, String) {
    let model = generate_valid(seed, sizes);
    render_with_provenance(&model, &format!("dzp-{seed}"), PROVENANCE_DIFF_T61)
}
