//! PDDL3 trajectory-constraint ENFORCEMENT (0.7, docs/roadmap-0.7.md).
//!
//! 0.4.1 through 0.6, every `(:constraints ...)` block got parsed and then
//! shot down clean. 0.7 cuts a narrower fence, operator by operator: the
//! six untimed modal operators (`always`, `sometime`, `at-most-once`,
//! `sometime-after`, `sometime-before`, `at end`) compile down into small
//! **monitor automata** riding the state trajectory — fresh 0-ary monitor
//! facts flipped by `Effect::When` conditional effects welded onto every
//! real action (the grounder and heuristic already know how to eat this).
//! A HARD constraint's acceptance rides into the goal, conjoined; a SOFT
//! `(preference name ...)` constraint (Phase 2) becomes a goal-side
//! `(preference name <acceptance>)`, priced by the PDDL3 metric machinery
//! same as any native preference. Anything this build can't enforce — the
//! timed operators, any constraint parked on a temporal domain — still
//! gets turned away with the operator NAMED. The "never silently ignore"
//! contract narrows here; it never gets deleted.
//!
//! THE OBSERVATION OFFSET (load-bearing): `PackedTask::apply` reads
//! conditional-effect conditions off the SOURCE state, so a monitor riding
//! action a_k is watching S_{k-1}, one step behind. The trajectory
//! S_0..S_n gets covered three ways — S_0 by compile-time evaluation
//! against init (this module), S_0..S_{n-1} by the per-action `When`s, and
//! S_n by the END construction below (0.8) or a goal-side formula
//! (`FF_NO_TRAJ_END=1`, the 0.7 shape). For `sometime-before` that one-step
//! lag is exactly what gives "strictly earlier" its teeth. Every
//! transition condition on one monitor fact stays mutually exclusive, so
//! the add-wins conflict rule can never co-fire a set and a clear on the
//! same bit.
//!
//! THE END CONSTRUCTION (0.8, docs/roadmap-0.8.md Phase 1): a HARD
//! monitor's S_n acceptance check used to ride into the goal conjoined,
//! and several operators throw disjunctions into that mix — the grounder
//! compiles a disjunctive goal into one synthetic REACH-GOAL operator per
//! DNF disjunct, EXPONENTIAL in the monitor count (storage hard fixture:
//! 3^10 = 59,049 ops, docs/roadmap-0.7.md Phase 1, on record). Since 0.8
//! the acceptance rides a forced-terminal synthetic action instead: every
//! real action needs the init-true phase fact `TRAJ-PLANNING` standing;
//! one synthetic 0-ary action `TRAJ-END` strikes it, raises `TRAJ-ENDED`,
//! and carries one `Effect::When` latch per hard monitor (condition = that
//! monitor's acceptance over its bits plus the S_n body, add =
//! `TRAJ{i}-ACC`). Because `When` conditions read the SOURCE state,
//! `TRAJ-END` firing after the last real action is watching exactly S_n.
//! The compiled goal comes out all positive literals — original goal ∧
//! `TRAJ-ENDED` ∧ the ACC facts — so the goal-DNF product never fires
//! again: cost runs LINEAR in monitors (2-3 conditional latches each, on
//! ONE op). SOFT acceptance doesn't move: `(preference name <acc>)`
//! wrappers stay parked in the goal with their S_n bodies intact — they're
//! invisible to the classical grounder's DNF, and the whole PDDL3 metric
//! stack keeps pricing them exactly as before, which is why the 0.7
//! deferral risk simply dissolves. The synthetic `TRAJ-END` step gets
//! stripped from every reported plan by the callers who ran this gate —
//! planner/api filter it by display name, conditionally, never touching
//! the constraint-free path.
//!
//! The independent verifier stays clear of this compilation entirely:
//! `verify.rs` folds the ORIGINAL constraint semantics over its own replay
//! (see [`Fold`]), so the oracle never depends on the compiled monitors.

use std::collections::HashMap;

use crate::pddl3::{combos, subst_formula};
use crate::types::{Action, Constraint, Domain, Effect, Formula, Problem, Sym};

/// Display name of the forced-terminal acceptance action (the 0.8 END
/// construction). Callers who ran [`gate`] strip ops carrying this display
/// name from reported plans; the reserved-name check fences it against
/// user collision whenever a `(:constraints ...)` block exists.
pub const END_ACTION: &str = "TRAJ-END";

/// One ground untimed trajectory-constraint, live.
#[derive(Clone, Debug)]
pub enum Traj {
    Always(Formula),
    Sometime(Formula),
    AtMostOnce(Formula),
    SometimeAfter(Formula, Formula),
    SometimeBefore(Formula, Formula),
    AtEnd(Formula),
}

/// The task's constraint sets, expanded: `Forall` quantifiers ground out,
/// `And` flattened flat, hard and soft (`preference`-wrapped) split clean.
pub struct Expanded {
    pub hard: Vec<Traj>,
    /// `(preference name <constraint>)` INSTANCES. The quantifier-instance
    /// line is drawn exactly where PDDL3 draws it (Gerevini & Long): a
    /// `forall` OUTSIDE the preference multiplies INSTANCES — all sharing
    /// the name, so `(is-violated name)` counts violated instances —
    /// while `and`/`forall` INSIDE the preference body stays ONE instance.
    /// The inner `Vec<Traj>` holds that body's member constraints, and the
    /// instance goes down iff ANY member does (its weight counts at most
    /// once). Anonymous preferences get a deterministic generated name
    /// (`TRAJPREF{n}` in source order), same convention as goal-preference
    /// handling. Enforced since Phase 2: [`compile`] lowers each instance
    /// to monitors plus ONE goal-side `(preference name <acceptance>)`,
    /// priced by the metric machinery.
    pub soft: Vec<(String, Vec<Traj>)>,
}

/// Expand and check a task's `(:constraints ...)` trees. Errors name the
/// unsupported operator — the timed family — or the malformed nesting.
pub fn expand(domain: &Domain, problem: &Problem) -> Result<Expanded, String> {
    let objs = crate::ground::objects_by_type(domain, problem);
    let mut out = Expanded {
        hard: Vec::new(),
        soft: Vec::new(),
    };
    let mut anon = 0usize;
    for c in domain.constraints.iter().chain(problem.constraints.iter()) {
        walk(c, &objs, &HashMap::new(), &mut anon, &mut out)?;
    }
    Ok(out)
}

/// Ground the FORMULA-level quantifiers of a formula — `forall` unrolls
/// into a conjunction, `exists` into a disjunction over the type's
/// objects. The IPC-5 qualitative suite buries these inside modal operators
/// (storage/tpp/trucks, e.g. `(sometime-before (exists (?c - crate) ...)
/// ...)`), and the simple-preferences goals bury them inside preference
/// bodies; expanding keeps every monitor transition ground for the
/// grounder and makes the verifier's own evaluation exact (its formula
/// evaluator never binds quantifiers — `verify.rs` calls this for
/// goal-preference scoring too). An empty type still yields the right
/// constants: `forall` collapses to true (`And []`), `exists` to false
/// (`Or []`).
pub(crate) fn expand_quantifiers(f: &Formula, objs: &HashMap<Sym, Vec<Sym>>) -> Formula {
    match f {
        Formula::Forall(vars, inner) => Formula::And(
            combos(vars, objs)
                .into_iter()
                .map(|b| expand_quantifiers(&subst_formula(inner, &b), objs))
                .collect(),
        ),
        Formula::Exists(vars, inner) => Formula::Or(
            combos(vars, objs)
                .into_iter()
                .map(|b| expand_quantifiers(&subst_formula(inner, &b), objs))
                .collect(),
        ),
        Formula::And(v) => Formula::And(v.iter().map(|x| expand_quantifiers(x, objs)).collect()),
        Formula::Or(v) => Formula::Or(v.iter().map(|x| expand_quantifiers(x, objs)).collect()),
        Formula::Not(a) => Formula::Not(Box::new(expand_quantifiers(a, objs))),
        Formula::Pref(n, a) => Formula::Pref(n.clone(), Box::new(expand_quantifiers(a, objs))),
        other => other.clone(),
    }
}

fn timed_err(op: &str) -> String {
    format!(
        "PDDL3 trajectory constraint `{op}` is time-bounded and not yet \
         enforced (untimed operators — always / sometime / at-most-once / \
         sometime-after / sometime-before / at-end — are). Remove it, or \
         model the requirement without a clock."
    )
}

fn walk(
    c: &Constraint,
    objs: &HashMap<Sym, Vec<Sym>>,
    binding: &HashMap<Sym, Sym>,
    anon: &mut usize,
    out: &mut Expanded,
) -> Result<(), String> {
    match c {
        Constraint::And(v) => {
            for x in v {
                walk(x, objs, binding, anon, out)?;
            }
        }
        Constraint::Forall(vars, inner) => {
            for combo in combos(vars, objs) {
                let mut b = binding.clone();
                b.extend(combo);
                walk(inner, objs, &b, anon, out)?;
            }
        }
        Constraint::Pref(name, inner) => {
            let name = name.clone().unwrap_or_else(|| {
                let s = format!("TRAJPREF{anon}");
                *anon += 1;
                s
            });
            // ONE preference instance per (textual preference × outside
            // binding): `and`/`forall` INSIDE the body collect into the
            // instance's member list — violated iff any member is.
            let mut members = Vec::new();
            walk_members(inner, objs, binding, &mut members)?;
            out.soft.push((name, members));
        }
        _ => {
            let mut members = Vec::new();
            walk_members(c, objs, binding, &mut members)?;
            out.hard.extend(members);
        }
    }
    Ok(())
}

/// Pull the ground member constraints out of one constraint tree — the
/// inside of a preference body, or a hard modal subtree. A preference
/// nested inside a preference is malformed here; PDDL3 gives it no
/// semantics at all.
fn walk_members(
    c: &Constraint,
    objs: &HashMap<Sym, Vec<Sym>>,
    binding: &HashMap<Sym, Sym>,
    members: &mut Vec<Traj>,
) -> Result<(), String> {
    let sub = |f: &Formula| expand_quantifiers(&subst_formula(f, binding), objs);
    match c {
        Constraint::And(v) => {
            for x in v {
                walk_members(x, objs, binding, members)?;
            }
        }
        Constraint::Forall(vars, inner) => {
            for combo in combos(vars, objs) {
                let mut b = binding.clone();
                b.extend(combo);
                walk_members(inner, objs, &b, members)?;
            }
        }
        Constraint::Pref(_, _) => {
            return Err(
                "malformed (:constraints ...): a preference nested inside a \
                 preference has no PDDL3 semantics"
                    .into(),
            )
        }
        Constraint::Always(f) => members.push(Traj::Always(sub(f))),
        Constraint::Sometime(f) => members.push(Traj::Sometime(sub(f))),
        Constraint::AtMostOnce(f) => members.push(Traj::AtMostOnce(sub(f))),
        Constraint::SometimeAfter(a, b) => members.push(Traj::SometimeAfter(sub(a), sub(b))),
        Constraint::SometimeBefore(a, b) => members.push(Traj::SometimeBefore(sub(a), sub(b))),
        Constraint::AtEnd(f) => members.push(Traj::AtEnd(sub(f))),
        Constraint::Within(_, _) => return Err(timed_err("within")),
        Constraint::AlwaysWithin(_, _, _) => return Err(timed_err("always-within")),
        Constraint::HoldDuring(_, _, _) => return Err(timed_err("hold-during")),
        Constraint::HoldAfter(_, _) => return Err(timed_err("hold-after")),
    }
    Ok(())
}

/// Incremental trajectory fold for ONE constraint instance — the
/// verifier's own independent semantics, never the compiled monitors. Feed
/// it every state of the replay in order, S_0 first, then ask
/// [`Fold::accepted`].
pub struct Fold<'a> {
    traj: &'a Traj,
    ok: bool,
    seen: bool,    // sometime: φ seen; at-most-once: an episode has closed
    holding: bool, // at-most-once: currently inside a φ episode
    pending: bool, // sometime-after: φ seen, ψ still owed
    safe: bool,    // sometime-before: ψ seen strictly earlier (the
    // strictly-earlier semantics is step()'s ORDER: φ is
    // tested against `safe` BEFORE ψ is recorded into it)
    last: bool, // at-end: φ in the most recent state
}

impl<'a> Fold<'a> {
    pub fn new(traj: &'a Traj) -> Self {
        Fold {
            traj,
            ok: true,
            seen: false,
            holding: false,
            pending: false,
            safe: false,
            last: false,
        }
    }

    /// Watch the trajectory's next state land, through a formula evaluator.
    pub fn step(&mut self, holds: &mut dyn FnMut(&Formula) -> bool) {
        match self.traj {
            Traj::Always(f) => {
                if !holds(f) {
                    self.ok = false;
                }
            }
            Traj::Sometime(f) => {
                if holds(f) {
                    self.seen = true;
                }
            }
            Traj::AtMostOnce(f) => {
                let now = holds(f);
                if now && !self.holding {
                    if self.seen {
                        self.ok = false; // a second episode opened
                    }
                    self.seen = true;
                }
                self.holding = now;
            }
            Traj::SometimeAfter(a, b) => {
                let (fa, fb) = (holds(a), holds(b));
                if fb {
                    self.pending = false;
                } else if fa {
                    self.pending = true;
                }
            }
            Traj::SometimeBefore(a, b) => {
                // check φ against ψ-seen STRICTLY earlier, then record ψ.
                if holds(a) && !self.safe {
                    self.ok = false;
                }
                if holds(b) {
                    self.safe = true;
                }
            }
            Traj::AtEnd(f) => {
                self.last = holds(f);
            }
        }
    }

    /// The verdict, once the last state has come in.
    pub fn accepted(&self) -> bool {
        match self.traj {
            Traj::Always(_) => self.ok,
            Traj::Sometime(_) => self.seen,
            Traj::AtMostOnce(_) => self.ok,
            Traj::SometimeAfter(_, _) => !self.pending,
            Traj::SometimeBefore(_, _) => self.ok,
            Traj::AtEnd(_) => self.last,
        }
    }

    /// The operator's name in plain speech, for verifier reports.
    pub fn op_name(&self) -> &'static str {
        match self.traj {
            Traj::Always(_) => "always",
            Traj::Sometime(_) => "sometime",
            Traj::AtMostOnce(_) => "at-most-once",
            Traj::SometimeAfter(_, _) => "sometime-after",
            Traj::SometimeBefore(_, _) => "sometime-before",
            Traj::AtEnd(_) => "at-end",
        }
    }
}

/// STATIC SIMPLIFICATION — planner-side only; the verifier keeps folding
/// the unsimplified [`expand`] output, so the oracle stays clean of this.
/// Partially evaluate every constraint body against the facts that can
/// never change (`pddl3::peval_static` — static predicates settled by
/// init, `(= a b)` by symbol equality, connectives folded), then DROP any
/// instance whose fold verdict is statically ACCEPTED across every
/// trajectory. This is the only reason the qualitative storage instances
/// compile at all: p03's `forall (?c1 ?c2 - crate ?s1 ?s2 - storearea)
/// (always (imply (... static connected/compatible ...) ...))` expands
/// quadratically, but ~90%+ of the instances simplify down to `always
/// true` — skip the drop and each one survives as a monitor with a `When`
/// transition riding EVERY action, and grounding OOMs a 15 GB container.
/// Survivors keep their simplified body (a cheaper `When` DNF). A
/// statically-VIOLATED instance (`always false`, say) is NEVER dropped —
/// the monitors still have to enforce and price it. `FF_PREF_NO_STATIC=1`
/// restores the blind expansion, the same hatch the goal-preference pass
/// uses.
fn simplify_static(exp: &mut Expanded, domain: &Domain, problem: &Problem) {
    if std::env::var("FF_PREF_NO_STATIC").is_ok() {
        return;
    }
    let statics = crate::pddl3::static_predicates(domain);
    let init: std::collections::HashSet<(Sym, Vec<Sym>)> =
        problem.init_atoms.iter().cloned().collect();
    let peval = |f: &Formula| crate::pddl3::peval_static(f, &statics, &init);
    let t = |f: &Formula| matches!(f, Formula::True);
    let fa = |f: &Formula| matches!(f, Formula::False);
    // Simplify bodies; `None` = statically accepted on every trajectory.
    let simp = |traj: &Traj| -> Option<Traj> {
        match traj {
            Traj::Always(f) => match peval(f) {
                f if t(&f) => None,
                f => Some(Traj::Always(f)),
            },
            Traj::Sometime(f) => match peval(f) {
                f if t(&f) => None,
                f => Some(Traj::Sometime(f)),
            },
            // φ static-true: one episode opens at S_0 and never closes;
            // φ static-false: no episode ever opens — accepted either way.
            Traj::AtMostOnce(f) => match peval(f) {
                f if t(&f) || fa(&f) => None,
                f => Some(Traj::AtMostOnce(f)),
            },
            // ψ in every state, or φ in none: nothing is ever owed.
            Traj::SometimeAfter(a, b) => {
                let (a, b) = (peval(a), peval(b));
                if fa(&a) || t(&b) {
                    None
                } else {
                    Some(Traj::SometimeAfter(a, b))
                }
            }
            // φ in no state: the ordering obligation never triggers.
            // (φ static-true is a VIOLATION at S_0 — kept for the monitors.)
            Traj::SometimeBefore(a, b) => {
                let (a, b) = (peval(a), peval(b));
                if fa(&a) {
                    None
                } else {
                    Some(Traj::SometimeBefore(a, b))
                }
            }
            Traj::AtEnd(f) => match peval(f) {
                f if t(&f) => None,
                f => Some(Traj::AtEnd(f)),
            },
        }
    };
    let h0 = exp.hard.len();
    let m0: usize = exp.soft.iter().map(|(_, ms)| ms.len()).sum();
    exp.hard = exp.hard.iter().filter_map(&simp).collect();
    // Soft: simplify each instance's MEMBERS. An instance whose members all
    // drop is statically SATISFIED — it stays in the list with an empty
    // member vec (compile lowers it to `(preference name true)`), so the
    // pref-instance count the optimizer reports never shrinks; only the
    // monitor machinery for it disappears.
    for (_, members) in exp.soft.iter_mut() {
        *members = members.iter().filter_map(&simp).collect();
    }
    let m1: usize = exp.soft.iter().map(|(_, ms)| ms.len()).sum();
    if std::env::var("FF_RES_DEBUG").is_ok() && (exp.hard.len(), m1) != (h0, m0) {
        eprintln!(
            "[P3] constraint static simplification: dropped {} of {} hard, {} of {} soft member(s)",
            h0 - exp.hard.len(),
            h0,
            m0 - m1,
            m0
        );
    }
}

/// Turn away any input whose own names collide with the generated
/// monitor namespace. A user predicate named `TRAJ0-VIOL`, say, would
/// intern to the SAME grounded fact as a monitor bit — a user effect could
/// then silently clear a hard-constraint violation, exactly the failure
/// class the "never silently ignore" contract exists to forbid. Same risk
/// for a user preference literally named `TRAJPREF{n}`: it would alias an
/// anonymous constraint-preference's generated name in the
/// `(is-violated ...)` namespace. Both get rejected BY NAME — only when a
/// `(:constraints ...)` block is present. This runs from `compile`, never
/// touching the constraint-free no-op path.
fn reject_reserved_names(domain: &Domain, problem: &Problem) -> Result<(), String> {
    let monitor_fact = |n: &str| -> bool {
        // The 0.8 END-construction phase facts are 0-ary and fixed-name.
        if n == "TRAJ-PLANNING" || n == "TRAJ-ENDED" {
            return true;
        }
        let Some(rest) = n.strip_prefix("TRAJ") else {
            return false;
        };
        let mut it = rest.splitn(2, '-');
        let (num, suf) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
        !num.is_empty()
            && num.bytes().all(|b| b.is_ascii_digit())
            && matches!(suf, "VIOL" | "SEEN" | "HOLD" | "PEND" | "SAFE" | "ACC")
    };
    let anon_pref = |n: &str| -> bool {
        n.strip_prefix("TRAJPREF")
            .is_some_and(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))
    };
    for (n, _) in &domain.predicates {
        if monitor_fact(n) {
            return Err(format!(
                "predicate `{n}` collides with ferroplan's reserved trajectory-monitor \
                 namespace (TRAJ{{n}}-VIOL/SEEN/HOLD/PEND/SAFE/ACC, TRAJ-PLANNING, \
                 TRAJ-ENDED) used to compile (:constraints ...); rename the predicate"
            ));
        }
    }
    // A user action named like the synthetic terminal action would be
    // filtered from reported plans by the callers' strip — reject it.
    if let Some(a) = domain.actions.iter().find(|a| a.name == END_ACTION) {
        return Err(format!(
            "action `{}` collides with ferroplan's reserved trajectory \
             end-action name (`{END_ACTION}`) used to compile \
             (:constraints ...); rename the action",
            a.name
        ));
    }
    // USER-written preference names only (generated anonymous names ARE the
    // namespace) — collected from the raw ASTs, before any name generation.
    fn names_c(c: &Constraint, out: &mut Vec<String>) {
        match c {
            Constraint::And(v) => v.iter().for_each(|x| names_c(x, out)),
            Constraint::Forall(_, i) => names_c(i, out),
            Constraint::Pref(n, i) => {
                if let Some(n) = n {
                    out.push(n.clone());
                }
                names_c(i, out);
            }
            _ => {}
        }
    }
    fn names_f(f: &Formula, out: &mut Vec<String>) {
        match f {
            Formula::And(v) | Formula::Or(v) => v.iter().for_each(|x| names_f(x, out)),
            Formula::Not(a) | Formula::Forall(_, a) | Formula::Exists(_, a) => names_f(a, out),
            Formula::Pref(n, a) => {
                if let Some(n) = n {
                    out.push(n.clone());
                }
                names_f(a, out);
            }
            _ => {}
        }
    }
    let mut user = Vec::new();
    for c in domain.constraints.iter().chain(problem.constraints.iter()) {
        names_c(c, &mut user);
    }
    names_f(&problem.goal, &mut user);
    if let Some(n) = user.iter().find(|n| anon_pref(n)) {
        return Err(format!(
            "preference name `{n}` collides with ferroplan's reserved \
             TRAJPREF{{n}} namespace (generated for anonymous constraint \
             preferences); rename the preference"
        ));
    }
    Ok(())
}

/// Cut the synthetic [`END_ACTION`] step out of a grounded op sequence
/// before any reporting surface lays eyes on it. Callers apply this only
/// when [`gate`] compiled the task — never on the constraint-free path,
/// where a user action can legitimately carry any name at all (the fence
/// in [`reject_reserved_names`] runs only when a `(:constraints ...)`
/// block exists, on purpose).
pub(crate) fn strip_end(task: &crate::packed::PackedTask, ops: &mut Vec<usize>) {
    ops.retain(|&oi| task.op_display[oi] != END_ACTION);
}

/// The 0.7 entrypoint gate, shared by `solve`/`decompose`/`run_planner`/
/// `run_ff` so no path can quietly diverge from another: `Ok(None)` means
/// no constraints, the byte-identical no-op path; `Ok(Some(pair))` means
/// untimed constraints — hard AND soft since Phase 2 — compiled into the
/// rewritten task; `Err(msg)` is a NAMED turn-away — the timed operators,
/// any constraint riding a durative-action domain (Phase 3), or the
/// `FF_CONSTRAINTS_REJECT=1` hatch, which restores the 0.4.1 blanket
/// rejection byte-for-byte. It restores *rejection*, never silence.
pub fn gate(domain: &Domain, problem: &Problem) -> Result<Option<(Domain, Problem)>, String> {
    if domain.constraints.is_empty() && problem.constraints.is_empty() {
        return Ok(None);
    }
    if std::env::var("FF_CONSTRAINTS_REJECT").is_ok() {
        return Err(crate::pddl3::unsupported_constraints(domain, problem)
            .unwrap_or_else(|| "trajectory constraints rejected (hatch)".into()));
    }
    if crate::temporal::is_temporal(domain) {
        return Err(
            "trajectory constraints on durative-action (temporal) domains are \
             not yet enforced (the untimed classical path is); remove the \
             (:constraints ...) block or the durative actions"
                .into(),
        );
    }
    compile(domain, problem).map(Some)
}

/// Burn the untimed constraints into the domain/problem: monitor
/// predicates plus per-action `When` transitions, per the module-level
/// table. A HARD constraint's acceptance rides the forced-terminal
/// `TRAJ-END` action's conditional latches, leaving the hard goal
/// literal-only (the 0.8 END construction; `FF_NO_TRAJ_END=1` restores
/// the 0.7 goal-side conjunction). A SOFT (`preference`-wrapped)
/// constraint's acceptance becomes a goal-side
/// `(preference name <acceptance>)` — the PDDL3 metric machinery
/// (`pddl3::compile`'s collect/forgo pricing, the closure optimizer, the
/// selection layer) then scores it exactly like a native goal preference,
/// because a monitor's final-state acceptance formula reads true iff the
/// constraint held across the whole trajectory. Hands back the rewritten
/// pair. Throws on timed operators, naming them.
pub fn compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> {
    reject_reserved_names(domain, problem)?;
    let mut exp = expand(domain, problem)?;
    simplify_static(&mut exp, domain, problem);

    let mut d = domain.clone();
    let mut p = problem.clone();
    if exp.hard.is_empty() && exp.soft.is_empty() {
        // Everything statically proven (or the block held only such
        // instances): enforced-by-proof, nothing to monitor — but the
        // constraints are still CONSUMED, not left dangling on the pair.
        d.constraints.clear();
        p.constraints.clear();
        return Ok((d, p));
    }

    let mut goal_conj: Vec<Formula> = vec![p.goal.clone()];
    // Per-action transition effects, accumulated then appended to every action.
    let mut transitions: Vec<Effect> = Vec::new();

    // Emit ONE member constraint's monitor (facts + transitions) and return
    // its acceptance conjuncts. `i` is the global monitor index — hard
    // instances first, then soft members, one shared namespace.
    fn emit(
        i: usize,
        t: &Traj,
        d: &mut Domain,
        p: &mut Problem,
        transitions: &mut Vec<Effect>,
        problem: &Problem,
    ) -> Vec<Formula> {
        // S_0 evaluation happens against the raw init atom set of the
        // ORIGINAL problem (user formulas can never reference the monitor
        // facts we add — `reject_reserved_names` enforces the premise).
        let init_holds = |f: &Formula| eval_static(f, problem);
        let atom = |n: &str| Formula::Atom(n.to_string(), vec![]);
        let add = |n: &str| Effect::Add(n.to_string(), vec![]);
        let del = |n: &str| Effect::Del(n.to_string(), vec![]);
        let declare = |d: &mut Domain, p: &mut Problem, n: &str, init_true: bool| {
            d.predicates.push((n.to_string(), vec![]));
            if init_true {
                p.init_atoms.push((n.to_string(), vec![]));
            }
        };
        // The constraint's ACCEPTANCE over S_0..S_n: monitor state ∧ the
        // goal-side S_n check.
        let mut acc: Vec<Formula> = Vec::new();
        match t {
            Traj::Always(f) => {
                let viol = format!("TRAJ{i}-VIOL");
                declare(d, p, &viol, !init_holds(f));
                transitions.push(Effect::When(
                    Formula::Not(Box::new(f.clone())),
                    Box::new(add(&viol)),
                ));
                acc.push(Formula::Not(Box::new(atom(&viol))));
                acc.push(f.clone()); // S_n
            }
            Traj::Sometime(f) => {
                let seen = format!("TRAJ{i}-SEEN");
                declare(d, p, &seen, init_holds(f));
                transitions.push(Effect::When(f.clone(), Box::new(add(&seen))));
                acc.push(Formula::Or(vec![atom(&seen), f.clone()]));
            }
            Traj::AtMostOnce(f) => {
                let hold = format!("TRAJ{i}-HOLD");
                let seen = format!("TRAJ{i}-SEEN");
                let viol = format!("TRAJ{i}-VIOL");
                let f0 = init_holds(f);
                declare(d, p, &hold, f0);
                declare(d, p, &seen, f0);
                declare(d, p, &viol, false);
                // second rising edge (φ ∧ ¬HOLD ∧ SEEN) → VIOL; then episode
                // tracking. Conditions are mutually exclusive per fact.
                transitions.push(Effect::When(
                    Formula::And(vec![
                        f.clone(),
                        Formula::Not(Box::new(atom(&hold))),
                        atom(&seen),
                    ]),
                    Box::new(add(&viol)),
                ));
                transitions.push(Effect::When(
                    Formula::And(vec![f.clone(), Formula::Not(Box::new(atom(&hold)))]),
                    Box::new(Effect::And(vec![add(&seen), add(&hold)])),
                ));
                transitions.push(Effect::When(
                    Formula::And(vec![Formula::Not(Box::new(f.clone())), atom(&hold)]),
                    Box::new(del(&hold)),
                ));
                acc.push(Formula::Not(Box::new(atom(&viol))));
                // S_n rising edge: φ now, not holding into it, already seen.
                acc.push(Formula::Not(Box::new(Formula::And(vec![
                    f.clone(),
                    Formula::Not(Box::new(atom(&hold))),
                    atom(&seen),
                ]))));
            }
            Traj::SometimeAfter(a, b) => {
                let pend = format!("TRAJ{i}-PEND");
                declare(d, p, &pend, init_holds(a) && !init_holds(b));
                transitions.push(Effect::When(b.clone(), Box::new(del(&pend))));
                transitions.push(Effect::When(
                    Formula::And(vec![a.clone(), Formula::Not(Box::new(b.clone()))]),
                    Box::new(add(&pend)),
                ));
                // accepted iff nothing pending after S_n's own φ/ψ resolve.
                acc.push(Formula::Or(vec![
                    b.clone(),
                    Formula::And(vec![
                        Formula::Not(Box::new(atom(&pend))),
                        Formula::Not(Box::new(a.clone())),
                    ]),
                ]));
            }
            Traj::SometimeBefore(a, b) => {
                let safe = format!("TRAJ{i}-SAFE");
                let viol = format!("TRAJ{i}-VIOL");
                declare(d, p, &safe, init_holds(b));
                declare(d, p, &viol, init_holds(a)); // φ(S_0): nothing earlier
                                                     // source-state reads give "strictly earlier" for free.
                transitions.push(Effect::When(
                    Formula::And(vec![a.clone(), Formula::Not(Box::new(atom(&safe)))]),
                    Box::new(add(&viol)),
                ));
                transitions.push(Effect::When(b.clone(), Box::new(add(&safe))));
                acc.push(Formula::Not(Box::new(atom(&viol))));
                acc.push(Formula::Or(vec![
                    Formula::Not(Box::new(a.clone())),
                    atom(&safe),
                ]));
            }
            Traj::AtEnd(f) => {
                acc.push(f.clone());
            }
        }
        acc
    }

    // Hard monitors: acceptance conjuncts collected per monitor. The 0.8
    // default lowers them onto the TRAJ-END latches below (linear); the
    // FF_NO_TRAJ_END hatch restores the 0.7 goal-side conjunction (whose
    // disjunctive members DNF-multiply into REACH-GOAL ops — exponential).
    let mut idx = 0usize;
    let mut hard_acc: Vec<Vec<Formula>> = Vec::new();
    for t in &exp.hard {
        hard_acc.push(emit(idx, t, &mut d, &mut p, &mut transitions, problem));
        idx += 1;
    }
    for (name, members) in &exp.soft {
        // ONE goal-side preference per instance: accepted iff EVERY member
        // accepted (a conjunctive body is violated at most once — PDDL3).
        // An instance whose members were all statically proven lowers to
        // `(preference name true)`: never violated, still COUNTED.
        let mut acc: Vec<Formula> = Vec::new();
        for t in members {
            acc.extend(emit(idx, t, &mut d, &mut p, &mut transitions, problem));
            idx += 1;
        }
        let body = match acc.len() {
            0 => Formula::True,
            1 => acc.pop().unwrap(),
            _ => Formula::And(acc),
        };
        goal_conj.push(Formula::Pref(Some(name.clone()), Box::new(body)));
    }

    // The monitor transitions ride every real action. Since 0.8 Phase 2
    // (docs/roadmap-0.8.md) they travel as the domain's SHARED block —
    // `d.monitors` plus a per-action `monitored` flag — and the grounder
    // grounds them ONCE, sharing the conditional-effect block across all
    // monitored ops. The transitions are fully ground and byte-identical
    // for every binding of every action, so the 0.7 per-action AST append
    // (grounded and stored per op) was pure duplication — the monitor-count
    // x ground-action product that OOM'd storage qualpref p07/p08.
    // `FF_NO_COND_SHARE=1` restores the 0.7 per-action append byte-for-byte.
    if !transitions.is_empty() {
        if std::env::var("FF_NO_COND_SHARE").is_ok() {
            for act in &mut d.actions {
                let mut v = vec![act.effect.clone()];
                v.extend(transitions.iter().cloned());
                act.effect = Effect::And(v);
            }
        } else {
            for act in &mut d.actions {
                act.monitored = true;
            }
            d.monitors = transitions.clone();
        }
    }

    // Lower the hard acceptance (docs/roadmap-0.8.md Phase 1).
    if !hard_acc.is_empty() {
        if std::env::var("FF_NO_TRAJ_END").is_ok() {
            // 0.7 shape: S_n acceptance as goal conjuncts. Kept reachable so
            // the exponential baseline stays measurable (house convention).
            for acc in hard_acc {
                goal_conj.extend(acc);
            }
        } else {
            // THE END CONSTRUCTION. TRAJ-END is created AFTER the transition
            // append above, so it carries NO monitor transitions — only the
            // acceptance latches, which read S_n as their source state and
            // never touch monitor bits (no add-wins interaction possible).
            let atom = |n: &str| Formula::Atom(n.to_string(), vec![]);
            d.predicates.push(("TRAJ-PLANNING".to_string(), vec![]));
            d.predicates.push(("TRAJ-ENDED".to_string(), vec![]));
            p.init_atoms.push(("TRAJ-PLANNING".to_string(), vec![]));
            // Every real action plans only while the phase is open; the P3
            // bookkeeping ops pddl3::compile creates LATER never gain this
            // precondition — they stay applicable after the freeze, so the
            // mixed hard+soft plan shape is real* -> TRAJ-END -> P3END ->
            // collect/forgo (pinned by test).
            for act in &mut d.actions {
                act.precond = Formula::And(vec![act.precond.clone(), atom("TRAJ-PLANNING")]);
            }
            let mut end_eff: Vec<Effect> = vec![
                Effect::Del("TRAJ-PLANNING".to_string(), vec![]),
                Effect::Add("TRAJ-ENDED".to_string(), vec![]),
            ];
            for (k, acc) in hard_acc.into_iter().enumerate() {
                let accf = format!("TRAJ{k}-ACC");
                d.predicates.push((accf.clone(), vec![]));
                let cond = match acc.len() {
                    1 => acc.into_iter().next().unwrap(),
                    _ => Formula::And(acc),
                };
                end_eff.push(Effect::When(
                    cond,
                    Box::new(Effect::Add(accf.clone(), vec![])),
                ));
                goal_conj.push(atom(&accf));
            }
            goal_conj.push(atom("TRAJ-ENDED"));
            d.actions.push(Action {
                name: END_ACTION.to_string(),
                params: vec![],
                precond: atom("TRAJ-PLANNING"),
                effect: Effect::And(end_eff),
                // TRAJ-END carries only the ACC latches — it must NOT
                // observe (the trajectory ends at S_n, its source state).
                monitored: false,
            });
        }
    }

    p.goal = Formula::And(goal_conj);
    d.constraints.clear();
    p.constraints.clear();
    Ok((d, p))
}

/// Read a (assumed ground) formula against the raw init atom set — S_0
/// for the monitor's own wake-up state. Numeric comparisons check against
/// init fluents; an unknown fluent reads the comparison false.
fn eval_static(f: &Formula, p: &Problem) -> bool {
    match f {
        Formula::True => true,
        Formula::False => false,
        Formula::And(v) => v.iter().all(|x| eval_static(x, p)),
        Formula::Or(v) => v.iter().any(|x| eval_static(x, p)),
        Formula::Not(a) => !eval_static(a, p),
        Formula::Pref(_, a) => eval_static(a, p),
        Formula::Forall(_, a) | Formula::Exists(_, a) => eval_static(a, p),
        Formula::Eq(a, b) => a == b,
        Formula::Atom(name, args) => p.init_atoms.iter().any(|(n, a)| {
            n.eq_ignore_ascii_case(name)
                && a.len() == args.len()
                && a.iter().zip(args).all(|(x, t)| match t {
                    crate::types::Term::Const(c) => x.eq_ignore_ascii_case(c),
                    crate::types::Term::Var(_) => false,
                })
        }),
        Formula::Comp(op, l, r) => {
            let ev = |e: &crate::types::Expr| eval_init_expr(e, p);
            match (ev(l), ev(r)) {
                (Some(l), Some(r)) => match op {
                    crate::types::CompOp::Lt => l < r,
                    crate::types::CompOp::Le => l <= r,
                    crate::types::CompOp::Eq => (l - r).abs() < 1e-6,
                    crate::types::CompOp::Ge => l >= r,
                    crate::types::CompOp::Gt => l > r,
                },
                _ => false,
            }
        }
    }
}

fn eval_init_expr(e: &crate::types::Expr, p: &Problem) -> Option<f64> {
    use crate::types::Expr::*;
    Some(match e {
        Num(n) => *n,
        Fluent(name, args) => {
            let ((_, _), v) = p.init_fluents.iter().find(|((n, a), _)| {
                n.eq_ignore_ascii_case(name)
                    && a.len() == args.len()
                    && a.iter().zip(args).all(|(x, t)| match t {
                        crate::types::Term::Const(c) => x.eq_ignore_ascii_case(c),
                        crate::types::Term::Var(_) => false,
                    })
            })?;
            *v
        }
        Add(a, b) => eval_init_expr(a, p)? + eval_init_expr(b, p)?,
        Sub(a, b) => eval_init_expr(a, p)? - eval_init_expr(b, p)?,
        Mul(a, b) => eval_init_expr(a, p)? * eval_init_expr(b, p)?,
        Div(a, b) => eval_init_expr(a, p)? / eval_init_expr(b, p)?,
        Neg(a) => -eval_init_expr(a, p)?,
    })
}

#[cfg(test)]
mod grounding_cost {
    //! Heavy fixtures, docs/roadmap-0.7.md Phase 1 acceptance: the
    //! grounding cost of a hard-`(:constraints ...)` overlay riding
    //! vendored IPC-5 instances — conditional-effect count and grounding
    //! wall time, measured against the unconstrained input. Run it with
    //! `cargo test -p ferroplan --release --lib grounding_cost -- --ignored --nocapture`
    //!
    //! On record (0.8 Phase 1, the END construction, docs/roadmap-0.8.md):
    //! the goal-DNF product is GONE — storage p05 with 10 at-most-once
    //! monitors dropped 59,969 ops (59,049 REACH-GOAL) down to 921 ops
    //! (0 REACH-GOAL, one TRAJ-END), ground ~2.2 s -> ~0.8 s; trucks p03
    //! with 3 monitors 1,083 (18 REACH-GOAL) -> 1,066. Conditional-effect
    //! counts grew only by the linear ACC latches (3 per at-most-once
    //! monitor: storage 36,800 -> 36,830). The remaining monitor x op
    //! When-product (36,830 cond effects) is Phase 2's target now. The
    //! asserts below LOCK the one-extra-op shape — a goal-DNF regression
    //! would re-explode it.

    /// Parse, gate (compiling any constraints), ground, then report
    /// `(ops, facts, conditional effects, ground millis)`. Also prints the
    /// monitor count and how many ops are synthetic REACH-GOAL disjunct
    /// ops — the goal-DNF cost of the monitors' S_n acceptance checks.
    fn measure(dom: &str, prob: &str, label: &str) -> (usize, usize, usize, u128) {
        let d = crate::parser::parse_domain(dom).expect("domain");
        let p = crate::parser::parse_problem(prob).expect("problem");
        let (d, p) = crate::derived::compile(&d, &p).expect("derived");
        let monitors = super::expand(&d, &p).expect("expand").hard.len();
        let (d, p) = match super::gate(&d, &p).expect("gate") {
            Some(pair) => pair,
            None => (d, p),
        };
        let t0 = crate::clock::Clock::now();
        let task = crate::ground::ground_task(&d, &p, 1).expect("ground");
        let ms = t0.elapsed_ms();
        let cond: usize = (0..task.n_ops).map(|oi| task.n_cond_effs(oi)).sum();
        let goal_ops = (0..task.n_ops)
            .filter(|&oi| task.op_display[oi].starts_with("REACH-GOAL"))
            .count();
        println!(
            "{label}: {} monitors, {} ops ({} REACH-GOAL), {} facts, \
             {} conditional effects, ground {} ms",
            monitors, task.n_ops, goal_ops, task.n_facts, cond, ms
        );
        (task.n_ops, task.n_facts, cond, ms)
    }

    /// Slot a `(:constraints ...)` block in right before the problem's
    /// final paren.
    fn overlay(prob: &str, constraints: &str) -> String {
        let i = prob.rfind(')').expect("problem has a closing paren");
        format!("{}(:constraints {}){}", &prob[..i], constraints, &prob[i..])
    }

    #[test]
    #[ignore = "heavy: grounding-cost measurement (docs/roadmap-0.7.md Phase 1)"]
    fn storage_p05_hard_overlay() {
        let base = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../benchmarks/ipc/pref/storage"
        );
        let dom = std::fs::read_to_string(format!("{base}/domain.pddl")).unwrap();
        let prob = std::fs::read_to_string(format!("{base}/p05.pddl")).unwrap();
        let (o0, f0, c0, _) = measure(&dom, &prob, "storage p05 unconstrained");
        // "each hoist lifts each crate at most once" — forall expands at the
        // constraint level, so every monitor body stays ground.
        let hard = overlay(
            &prob,
            "(forall (?h - hoist ?c - crate) (at-most-once (lifting ?h ?c)))",
        );
        let (o1, f1, c1, _) = measure(&dom, &hard, "storage p05 + hard overlay");
        assert!(f1 > f0, "monitor facts must appear ({f0} -> {f1})");
        assert!(c1 > c0, "monitor transitions must appear ({c0} -> {c1})");
        // 0.8 END construction: the ONLY op added is TRAJ-END — 10 monitors
        // used to cost 3^10 = 59,049 REACH-GOAL goal-DNF ops here.
        assert_eq!(
            o1,
            o0 + 1,
            "goal-DNF product must stay gone (docs/roadmap-0.8.md Phase 1)"
        );
    }

    #[test]
    #[ignore = "heavy: grounding-cost measurement (docs/roadmap-0.7.md Phase 1)"]
    fn trucks_p03_hard_overlay() {
        let base = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../benchmarks/ipc/pref/trucks"
        );
        let dom = std::fs::read_to_string(format!("{base}/domain.pddl")).unwrap();
        let prob = std::fs::read_to_string(format!("{base}/p03.pddl")).unwrap();
        let (o0, f0, c0, _) = measure(&dom, &prob, "trucks p03 unconstrained");
        // "a truck parks at each location at most once"
        let hard = overlay(
            &prob,
            "(forall (?t - truck ?l - location) (at-most-once (at ?t ?l)))",
        );
        let (o1, f1, c1, _) = measure(&dom, &hard, "trucks p03 + hard overlay");
        assert!(f1 > f0, "monitor facts must appear ({f0} -> {f1})");
        assert!(c1 > c0, "monitor transitions must appear ({c0} -> {c1})");
        // 0.8 END construction: +1 op (TRAJ-END), zero REACH-GOAL ops.
        assert_eq!(
            o1,
            o0 + 1,
            "goal-DNF product must stay gone (docs/roadmap-0.8.md Phase 1)"
        );
    }
}
