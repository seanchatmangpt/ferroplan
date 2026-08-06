//! Soft law, priced. PDDL3.0 preferences and metric optimization — phase 1.
//!
//! The compile (Keyder & Geffner, "Soft goals can be compiled away", JAIR
//! 2009): every `(preference p phi)` in the goal gets a shadow, a 0-ary fact
//! `collected_p`; a `collect_p` action that pays nothing when phi already
//! holds; a `forgo_p` action that pays `w_p` on `(total-cost)` when it
//! doesn't. `collected_p` gets folded into the hard goal, so driving down
//! `total-cost` is driving down the weighted count of broken promises —
//! stacked against whatever real action costs were already on the books.
//!
//! `w_p` reads off the `(is-violated p)` coefficient in the `:metric`.
//! A preference the metric never names costs nothing to break — weight 0,
//! free passage. No metric at all and every broken promise costs the same —
//! weight 1, straight body count.
//!
//! The search runs anytime branch-and-bound over `total-cost`, through
//! `crate::solve_subgoal_bounded` — sound only because the costs never run
//! backward. In scope: metrics linear in `(is-violated …)`, in
//! `(total-cost)`, and in any other monotone numeric term (rovers'
//! `(sum-traverse-cost)`, for one) — `compile()` folds all of it into
//! `total-cost` so the single-cost search optimizes the whole metric, not a
//! slice of it. Anything non-monotone, buried under a forall, or torn apart
//! by division gets flagged and left alone.

use std::collections::{HashMap, HashSet};

use crate::packed::PackedTask;
use crate::search::{plan, solve_subgoal_bounded, ClosureCost, PrefPhi, SatGuidance, SearchCfg};
use crate::types::{
    Action, AssignOp, Domain, Effect, Expr, Formula, MetricDir, Problem, Sym, Term,
};

pub const COST: &str = "TOTAL-COST";
pub const COST_DISP: &str = "(TOTAL-COST)";

/// Is there soft law on this problem, or a metric watching the till?
pub fn is_pddl3(problem: &Problem) -> bool {
    problem.metric.is_some() || goal_has_pref(&problem.goal)
}

fn goal_has_pref(f: &Formula) -> bool {
    match f {
        Formula::Pref(_, _) => true,
        Formula::And(v) | Formula::Or(v) => v.iter().any(goal_has_pref),
        Formula::Not(a) => goal_has_pref(a),
        Formula::Forall(_, a) | Formula::Exists(_, a) => goal_has_pref(a),
        _ => false,
    }
}

fn expr_has_is_violated(e: &crate::types::Expr) -> bool {
    use crate::types::Expr::*;
    match e {
        Fluent(name, _) => name.eq_ignore_ascii_case("is-violated"),
        Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) => {
            expr_has_is_violated(a) || expr_has_is_violated(b)
        }
        Neg(a) => expr_has_is_violated(a),
        Num(_) => false,
    }
}

/// True only when the problem is actually carrying preferences — goal
/// preferences, or a metric watching `is-violated` — not just a plain
/// numeric `:metric` wearing the same coat. Routes `Mode::Auto`: real
/// preferences send it into PDDL3 mode, everything else runs classic FF.
pub fn has_preferences(problem: &Problem) -> bool {
    goal_has_pref(&problem.goal)
        || problem
            .metric
            .as_ref()
            .is_some_and(|(_, e)| expr_has_is_violated(e))
}

/// The old 0.4.1 door-in-the-face — the blanket refusal for PDDL3
/// `(:constraints ...)`.
///
/// Since 0.7 the hard untimed operators get enforced, not turned away —
/// [`crate::constraints::gate`] compiles them straight into monitor
/// automata. The blanket refusal doesn't run the show anymore; it only
/// wakes up behind the `FF_CONSTRAINTS_REJECT=1` hatch, restoring the old
/// reject-everything posture at every gate. Hands back the refusal message
/// when trajectory constraints are actually present, `None` when there's
/// nothing there to turn away.
///
/// A different animal from goal `(preference ...)` soft goals, which the
/// PDDL3 metric path already handles — those live inside the goal formula,
/// not in `.constraints`, and walk free of this gate entirely.
pub(crate) fn unsupported_constraints(domain: &Domain, problem: &Problem) -> Option<String> {
    if domain.constraints.is_empty() && problem.constraints.is_empty() {
        return None;
    }
    Some(
        "PDDL3 trajectory constraints (:constraints — always / sometime / \
         at-most-once / sometime-after / sometime-before / within / hold-during / \
         hold-after) are parsed but not yet enforced; ferroplan cannot honor them \
         and will not silently ignore them. Remove the (:constraints ...) block, or \
         model the requirement as hard goals or PDDL3 goal preferences."
            .to_string(),
    )
}

// ---- formula substitution + quantifier combos (for forall-preferences) ----

fn subst_term(t: &Term, b: &HashMap<Sym, Sym>) -> Term {
    match t {
        Term::Var(v) => b
            .get(v)
            .map(|o| Term::Const(o.clone()))
            .unwrap_or_else(|| t.clone()),
        Term::Const(_) => t.clone(),
    }
}
fn subst_expr(e: &Expr, b: &HashMap<Sym, Sym>) -> Expr {
    match e {
        Expr::Num(n) => Expr::Num(*n),
        Expr::Fluent(f, a) => Expr::Fluent(f.clone(), a.iter().map(|t| subst_term(t, b)).collect()),
        Expr::Add(x, y) => Expr::Add(Box::new(subst_expr(x, b)), Box::new(subst_expr(y, b))),
        Expr::Sub(x, y) => Expr::Sub(Box::new(subst_expr(x, b)), Box::new(subst_expr(y, b))),
        Expr::Mul(x, y) => Expr::Mul(Box::new(subst_expr(x, b)), Box::new(subst_expr(y, b))),
        Expr::Div(x, y) => Expr::Div(Box::new(subst_expr(x, b)), Box::new(subst_expr(y, b))),
        Expr::Neg(x) => Expr::Neg(Box::new(subst_expr(x, b))),
    }
}
pub(crate) fn subst_formula(f: &Formula, b: &HashMap<Sym, Sym>) -> Formula {
    match f {
        Formula::And(v) => Formula::And(v.iter().map(|x| subst_formula(x, b)).collect()),
        Formula::Or(v) => Formula::Or(v.iter().map(|x| subst_formula(x, b)).collect()),
        Formula::Not(a) => Formula::Not(Box::new(subst_formula(a, b))),
        Formula::Atom(p, a) => {
            Formula::Atom(p.clone(), a.iter().map(|t| subst_term(t, b)).collect())
        }
        Formula::Comp(op, l, r) => Formula::Comp(*op, subst_expr(l, b), subst_expr(r, b)),
        Formula::Eq(x, y) => Formula::Eq(subst_term(x, b), subst_term(y, b)),
        Formula::Pref(n, inner) => Formula::Pref(n.clone(), Box::new(subst_formula(inner, b))),
        // inner quantifier may shadow an outer var: don't substitute its own vars
        Formula::Forall(vars, inner) | Formula::Exists(vars, inner) => {
            let mut b2 = b.clone();
            for (v, _) in vars {
                b2.remove(v);
            }
            let inner = Box::new(subst_formula(inner, &b2));
            if matches!(f, Formula::Forall(..)) {
                Formula::Forall(vars.clone(), inner)
            } else {
                Formula::Exists(vars.clone(), inner)
            }
        }
        Formula::True => Formula::True,
        Formula::False => Formula::False,
    }
}
pub(crate) fn combos(vars: &[(Sym, Sym)], objs: &HashMap<Sym, Vec<Sym>>) -> Vec<HashMap<Sym, Sym>> {
    let mut acc = vec![HashMap::new()];
    for (v, ty) in vars {
        let dom: &[Sym] = objs.get(ty).map(|x| x.as_slice()).unwrap_or(&[]);
        let mut next = Vec::new();
        for a in &acc {
            for o in dom {
                let mut m = a.clone();
                m.insert(v.clone(), o.clone());
                next.push(m);
            }
        }
        acc = next;
    }
    acc
}
fn contains_pref(f: &Formula) -> bool {
    match f {
        Formula::Pref(_, _) => true,
        Formula::And(v) | Formula::Or(v) => v.iter().any(contains_pref),
        Formula::Not(a) | Formula::Forall(_, a) | Formula::Exists(_, a) => contains_pref(a),
        _ => false,
    }
}

/// Cut the soft `(preference name phi)` clauses out of an action's
/// precondition, clean. Hands back what's left of the hard precondition
/// and the (name, phi) preference roster that was pulled from it.
fn extract_precond_prefs(f: &Formula, ctr: &mut usize) -> (Formula, Vec<(String, Formula)>) {
    match f {
        Formula::And(v) => {
            let mut hard = Vec::new();
            let mut prefs = Vec::new();
            for x in v {
                let (h, mut ps) = extract_precond_prefs(x, ctr);
                prefs.append(&mut ps);
                if !matches!(h, Formula::True) {
                    hard.push(h);
                }
            }
            (Formula::And(hard), prefs)
        }
        Formula::Pref(name, inner) => {
            let n = name.clone().unwrap_or_else(|| {
                let s = format!("PCPREF{}", *ctr);
                *ctr += 1;
                s
            });
            (Formula::True, vec![(n, (**inner).clone())])
        }
        other => (other.clone(), Vec::new()),
    }
}

/// Split the goal down the middle: hard conjuncts on one side, named
/// preferences on the other. A `(forall (vars) ... preference ...)` spawns
/// one instance per object binding, every instance carrying the same name
/// — so `(is-violated name)` reads as a body count across the whole
/// binding set, exactly what PDDL3 semantics demand.
fn split_goal(
    g: &Formula,
    hard: &mut Vec<Formula>,
    prefs: &mut Vec<(String, Formula)>,
    ctr: &mut usize,
    objs: &HashMap<Sym, Vec<Sym>>,
) {
    match g {
        Formula::And(v) => v.iter().for_each(|f| split_goal(f, hard, prefs, ctr, objs)),
        Formula::Forall(vars, inner) if contains_pref(inner) => {
            for b in combos(vars, objs) {
                split_goal(&subst_formula(inner, &b), hard, prefs, ctr, objs);
            }
        }
        Formula::Pref(name, inner) => {
            let n = name.clone().unwrap_or_else(|| {
                let s = format!("PREF{}", *ctr);
                *ctr += 1;
                s
            });
            prefs.push((n, (**inner).clone()));
        }
        Formula::True => {}
        other => hard.push(other.clone()),
    }
}

/// Tally the metric, term by term: `is-violated p` books to `w[p]`,
/// `total-cost` books its coefficient, any other 0-ary fluent `(f)` — say,
/// `(sum-traverse-cost)` — books to `others[f]`, the flat remainder feeds
/// `konst`. Anything genuinely out of scope — n-ary metric fluents,
/// division, a product with no constant anchor — trips `other` and gets
/// flagged, not counted.
#[allow(clippy::too_many_arguments)]
fn extract(
    e: &Expr,
    scale: f64,
    w: &mut HashMap<String, f64>,
    tc: &mut f64,
    others: &mut HashMap<String, f64>,
    konst: &mut f64,
    other: &mut bool,
) {
    match e {
        Expr::Num(n) => *konst += scale * n,
        Expr::Fluent(name, args) => {
            if name == "IS-VIOLATED" {
                if let Some(Term::Const(p)) = args.first() {
                    *w.entry(p.clone()).or_insert(0.0) += scale;
                }
            } else if name == COST {
                *tc += scale;
            } else if args.is_empty() {
                *others.entry(name.clone()).or_insert(0.0) += scale;
            } else {
                *other = true;
            }
        }
        Expr::Add(a, b) => {
            extract(a, scale, w, tc, others, konst, other);
            extract(b, scale, w, tc, others, konst, other);
        }
        Expr::Sub(a, b) => {
            extract(a, scale, w, tc, others, konst, other);
            extract(b, -scale, w, tc, others, konst, other);
        }
        Expr::Neg(a) => extract(a, -scale, w, tc, others, konst, other),
        Expr::Mul(a, b) => match (&**a, &**b) {
            (Expr::Num(c), _) => extract(b, scale * c, w, tc, others, konst, other),
            (_, Expr::Num(c)) => extract(a, scale * c, w, tc, others, konst, other),
            _ => *other = true,
        },
        Expr::Div(_, _) => *other = true,
    }
}

/// Predicates no action ever touches — added, deleted, never. Whatever
/// truth they carry, they carried it at the initial state and they'll
/// carry it to the end. The static complement of [`modified_functions`].
pub(crate) fn static_predicates(domain: &Domain) -> HashSet<String> {
    fn walk(e: &Effect, out: &mut HashSet<String>) {
        match e {
            Effect::And(v) => v.iter().for_each(|x| walk(x, out)),
            Effect::Add(name, _) | Effect::Del(name, _) => {
                out.insert(name.clone());
            }
            Effect::When(_, e) | Effect::Forall(_, e) => walk(e, out),
            Effect::Num(..) => {}
        }
    }
    let mut modified = HashSet::new();
    for a in &domain.actions {
        walk(&a.effect, &mut modified);
    }
    // the shared monitor block (0.8 Phase 2) rides every monitored action —
    // facts it touches (TRAJ* monitor bits) are NOT static, exactly as when
    // the transitions were appended to each action's own effect
    for e in &domain.monitors {
        walk(e, &mut modified);
    }
    domain
        .predicates
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| !modified.contains(n))
        .collect()
}

/// Run a preference formula past the facts that will never move — decide
/// what can be decided now, leave the rest standing. Fully-ground atoms on
/// static predicates get read straight from init membership; ground
/// `(= a b)` gets read off symbol identity; the connectives fold through.
/// Anything still carrying uncertainty — numeric comparisons, quantified
/// variables, dynamic predicates — stays exactly as written; the fold
/// never claims more than the facts support, so its output is equivalent
/// in every state the search can actually reach. A phi that folds all the
/// way to `True` can never be broken — zero contribution to the metric,
/// forever — which is the license `compile()` needs to drop it before the
/// Keyder–Geffner expansion even starts.
pub(crate) fn peval_static(
    f: &Formula,
    statics: &HashSet<String>,
    init: &HashSet<(Sym, Vec<Sym>)>,
) -> Formula {
    match f {
        Formula::Atom(p, args) => {
            if !statics.contains(p) {
                return f.clone();
            }
            let consts: Option<Vec<Sym>> = args
                .iter()
                .map(|t| match t {
                    Term::Const(c) => Some(c.clone()),
                    Term::Var(_) => None,
                })
                .collect();
            match consts {
                Some(cs) => {
                    if init.contains(&(p.clone(), cs)) {
                        Formula::True
                    } else {
                        Formula::False
                    }
                }
                None => f.clone(), // still quantified — leave for grounding
            }
        }
        Formula::Eq(Term::Const(a), Term::Const(b)) => {
            if a == b {
                Formula::True
            } else {
                Formula::False
            }
        }
        Formula::Not(inner) => match peval_static(inner, statics, init) {
            Formula::True => Formula::False,
            Formula::False => Formula::True,
            other => Formula::Not(Box::new(other)),
        },
        Formula::And(v) => {
            let mut rest = Vec::new();
            for x in v {
                match peval_static(x, statics, init) {
                    Formula::True => {}
                    Formula::False => return Formula::False,
                    other => rest.push(other),
                }
            }
            if rest.is_empty() {
                Formula::True
            } else {
                Formula::And(rest)
            }
        }
        Formula::Or(v) => {
            let mut rest = Vec::new();
            for x in v {
                match peval_static(x, statics, init) {
                    Formula::True => return Formula::True,
                    Formula::False => {}
                    other => rest.push(other),
                }
            }
            if rest.is_empty() {
                Formula::False
            } else {
                Formula::Or(rest)
            }
        }
        // `forall . True` and `exists . False` hold/fail vacuously even over an
        // empty binding domain; the dual cases depend on domain non-emptiness,
        // so they stay wrapped (conservative).
        Formula::Forall(vars, inner) => match peval_static(inner, statics, init) {
            Formula::True => Formula::True,
            other => Formula::Forall(vars.clone(), Box::new(other)),
        },
        Formula::Exists(vars, inner) => match peval_static(inner, statics, init) {
            Formula::False => Formula::False,
            other => Formula::Exists(vars.clone(), Box::new(other)),
        },
        _ => f.clone(),
    }
}

/// Functions some action's effect touches — the complement of "static", the
/// list of what's still in play.
fn modified_functions(domain: &Domain) -> HashSet<String> {
    fn walk(e: &Effect, out: &mut HashSet<String>) {
        match e {
            Effect::And(v) => v.iter().for_each(|x| walk(x, out)),
            Effect::Num(_, name, _, _) => {
                out.insert(name.clone());
            }
            Effect::When(_, e) | Effect::Forall(_, e) => walk(e, out),
            _ => {}
        }
    }
    let mut out = HashSet::new();
    for a in &domain.actions {
        walk(&a.effect, &mut out);
    }
    out
}

/// Does metric fluent `fname` clear the bar to fold into total-cost — never
/// runs backward, only forward? Clears it only when every effect on it is
/// `(increase fname X)`, X either a non-negative constant or a static
/// fluent whose init values never dip below zero. Returns `Some(reason)`
/// the instant it doesn't.
fn fluent_foldable(domain: &Domain, problem: &Problem, fname: &str) -> Option<String> {
    let modified = modified_functions(domain);
    let static_nonneg = |g: &str| -> bool {
        if modified.contains(g) {
            return false;
        }
        // every init value of g must be >= 0 (default 0 if unspecified)
        problem
            .init_fluents
            .iter()
            .filter(|((n, _), _)| n == g)
            .all(|(_, v)| *v >= 0.0)
    };
    let mut bad: Option<String> = None;
    fn walk(
        e: &Effect,
        fname: &str,
        in_forall: bool,
        static_nonneg: &dyn Fn(&str) -> bool,
        bad: &mut Option<String>,
    ) {
        match e {
            Effect::And(v) => v
                .iter()
                .for_each(|x| walk(x, fname, in_forall, static_nonneg, bad)),
            Effect::When(_, e) => walk(e, fname, in_forall, static_nonneg, bad),
            Effect::Forall(_, e) => walk(e, fname, true, static_nonneg, bad),
            Effect::Num(op, name, _, val) if name == fname => {
                // an increase inside a forall can't be mirrored term-for-term, so
                // treat it as not foldable.
                let ok = !in_forall
                    && matches!(op, AssignOp::Increase)
                    && match val {
                        Expr::Num(n) => *n >= 0.0,
                        Expr::Fluent(g, _) => static_nonneg(g),
                        _ => false,
                    };
                if !ok && bad.is_none() {
                    *bad = Some(format!(
                        "metric fluent ({fname}) is not foldable (not monotone, or under forall)"
                    ));
                }
            }
            _ => {}
        }
    }
    for a in &domain.actions {
        walk(&a.effect, fname, false, &static_nonneg, &mut bad);
    }
    bad
}

/// `coeff * x`, dressed as an Expr — skips the pointless `(* 1 x)` wrapper
/// when coeff is 1.
fn scaled_expr(coeff: f64, x: &Expr) -> Expr {
    if coeff == 1.0 {
        x.clone()
    } else {
        Expr::Mul(Box::new(Expr::Num(coeff)), Box::new(x.clone()))
    }
}

/// Cast a shadow: for every `(increase fname X)` buried in `eff`, mint a
/// matching `(increase total-cost coeff*X)`.
fn collect_cost_mirror(eff: &Effect, fname: &str, coeff: f64, out: &mut Vec<Effect>) {
    match eff {
        Effect::And(v) => v
            .iter()
            .for_each(|x| collect_cost_mirror(x, fname, coeff, out)),
        Effect::When(_, e) => collect_cost_mirror(e, fname, coeff, out),
        Effect::Num(AssignOp::Increase, name, _, val) if name == fname => {
            out.push(Effect::Num(
                AssignOp::Increase,
                COST.to_string(),
                vec![],
                scaled_expr(coeff, val),
            ));
        }
        _ => {}
    }
}

/// Pull the (name, formula) preference roster off a goal — forall-expanded
/// over `objs` — ready for independent scoring and compilation.
pub fn preferences(goal: &Formula, objs: &HashMap<Sym, Vec<Sym>>) -> Vec<(String, Formula)> {
    let mut hard = Vec::new();
    let mut prefs = Vec::new();
    let mut ctr = 0;
    split_goal(goal, &mut hard, &mut prefs, &mut ctr, objs);
    prefs
}

/// The going rate on each preference — one weight per instance name.
/// Weight 1 across the board with no metric watching; otherwise whatever
/// `(is-violated name)`'s coefficient says, 0 if the metric never mentions
/// it at all. Reads both goal preferences and — since 0.7 Phase 2 —
/// `(:constraints ...)` constraint-preferences; they share one
/// `(is-violated name)` ledger and the same defaults.
pub fn pref_weights(domain: &Domain, problem: &Problem) -> HashMap<String, f64> {
    let mut w = HashMap::new();
    let mut tc = 0.0;
    let mut others = HashMap::new();
    let mut other = false;
    let absent = problem.metric.is_none();
    if let Some((dir, e)) = &problem.metric {
        // Same normalization as compile(): a maximize metric's weights are
        // read off the negated (minimized) objective.
        let scale = if matches!(dir, MetricDir::Maximize) {
            -1.0
        } else {
            1.0
        };
        let mut konst = 0.0;
        extract(
            e,
            scale,
            &mut w,
            &mut tc,
            &mut others,
            &mut konst,
            &mut other,
        );
    }
    let objs = crate::ground::objects_by_type(domain, problem);
    let mut out = HashMap::new();
    let mut names: Vec<String> = preferences(&problem.goal, &objs)
        .into_iter()
        .map(|(n, _)| n)
        .collect();
    if let Ok(exp) = crate::constraints::expand(domain, problem) {
        names.extend(exp.soft.into_iter().map(|(n, _)| n));
    }
    for n in names {
        let wn = w.get(&n).copied().unwrap_or(if absent { 1.0 } else { 0.0 });
        out.insert(n, wn);
    }
    out
}

pub struct Compiled {
    pub domain: Domain,
    pub problem: Problem,
    /// True if the original order was to drive it down. Informational —
    /// maximize metrics get flipped and normalized to minimize before
    /// compile-time; see `maximized`.
    pub minimize: bool,
    /// The original order was maximize, and it got turned around by
    /// negation to fit the house's minimize-only frame. The optimizer's
    /// value V maps back to the true metric as `-(V + metric_konst)` — go
    /// through [`Compiled::display_metric`], never do it by hand.
    pub maximized: bool,
    /// The flat term riding on the normalized objective — invisible to the
    /// optimizer (a constant never moves an argmin) but needed the moment
    /// anyone asks what the original metric's number actually is (IPC6
    /// net benefit's `(- CONST ...)` shape).
    pub metric_konst: f64,
    pub n_prefs: usize,
    pub warn_other: bool,
    /// Set the instant the metric steps outside supported territory —
    /// maximize, negative weight, a scaled or non-monotone total-cost.
    /// The caller drops to a satisficing plan rather than optimize an
    /// objective it can't actually verify.
    pub unsupported: Option<String>,
    /// Names of the ghost actions Keyder-Geffner minted — stripped clean
    /// out of the final plan before anyone sees it.
    pub synthetic: HashSet<String>,
    /// (forgo-action name, weight) — the price sheet, one line per
    /// preference instance.
    pub forgos: Vec<(String, f64)>,
    /// Fires when some numeric metric term got folded into total-cost —
    /// mirrored `increase` effects riding on real actions, rovers' travel
    /// costs among them. Tasks carrying this flag route to the legacy
    /// compiled-goal branch-and-bound instead: real-action cost hands it an
    /// honest gradient, and the closure search runs worse there, caught in
    /// continuous tightening churn.
    pub folded_metric: bool,
}

impl Compiled {
    /// Carry the optimizer's minimized number back to what the original
    /// metric would have said. Identity for a plain minimize metric — its
    /// constant sits at 0 in every IPC shape. `-(V + konst)` for a
    /// normalized maximize metric — IPC6 net benefit's `maximize (- 70 X)`
    /// optimizes `minimize X` with konst = -70, and reports back `70 - X`.
    pub fn display_metric(&self, optimized: f64) -> f64 {
        let m = optimized + self.metric_konst;
        if self.maximized {
            -m
        } else {
            m
        }
    }
}

/// Does `total-cost` only ever climb, never fall, anywhere in this domain?
/// Branch-and-bound pruning is only trustworthy if it does. Any decrease,
/// scale, or assign on total-cost breaks that promise outright; an
/// increase stays honest when its amount is a non-negative constant, or a
/// static fluent whose init values never drop below zero — no action
/// reaches it, so it can't turn on you later. That covers the IPC6 shape
/// `(increase (total-cost) (travel-fast ?f1 ?f2))` running through the
/// elevators, crew-planning, and openstacks net-benefit domains.
fn cost_monotone(domain: &Domain, problem: &Problem) -> bool {
    let modified = modified_functions(domain);
    let static_nonneg = |g: &str| -> bool {
        !modified.contains(g)
            && problem
                .init_fluents
                .iter()
                .filter(|((n, _), _)| n == g)
                .all(|(_, v)| *v >= 0.0)
    };
    // Provably non-negative static expression: sums/products/quotients of
    // non-negative constants and static non-negative fluents can never turn
    // negative at apply time, so an increase by one is monotone. Sub/Neg (or
    // any dynamic fluent) could — reject those. Covers every IPC6 cost
    // shape, e.g. crew-planning's
    // `(* (/ (payloadact_length ?pa) 10) (+ (crew_efficiency ?c ?d) ...))`.
    fn nonneg_static(e: &Expr, static_nonneg: &dyn Fn(&str) -> bool) -> bool {
        match e {
            Expr::Num(n) => *n >= 0.0,
            Expr::Fluent(g, _) => static_nonneg(g),
            Expr::Add(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                nonneg_static(a, static_nonneg) && nonneg_static(b, static_nonneg)
            }
            Expr::Sub(..) | Expr::Neg(_) => false,
        }
    }
    fn walk(e: &Effect, static_nonneg: &dyn Fn(&str) -> bool, ok: &mut bool) {
        match e {
            Effect::And(v) => v.iter().for_each(|x| walk(x, static_nonneg, ok)),
            Effect::Num(op, name, _, val)
                if name == COST
                    && !(matches!(op, AssignOp::Increase) && nonneg_static(val, static_nonneg)) =>
            {
                *ok = false;
            }
            _ => {}
        }
    }
    let mut ok = true;
    for a in &domain.actions {
        walk(&a.effect, &static_nonneg, &mut ok);
    }
    ok
}

/// Burn the soft goals out of the picture — recast the whole job as a
/// classical, cost-priced problem, Keyder–Geffner style.
pub fn compile(domain: &Domain, problem: &Problem) -> Compiled {
    let objs = crate::ground::objects_by_type(domain, problem);
    let mut hard = Vec::new();
    let mut prefs = Vec::new();
    let mut ctr = 0;
    split_goal(&problem.goal, &mut hard, &mut prefs, &mut ctr, &objs);
    let n_prefs_total = prefs.len();

    // STATIC SIMPLIFICATION: a preference whose phi is statically TRUE (e.g. an
    // `imply` whose antecedent tests a static relation like storage's
    // `(connected s1 s2)` on an unconnected pair) can never be violated —
    // contributing exactly 0 to the metric in every reachable state — so it
    // never needs collect/forgo ops or a hard-goal fact. IPC-5 storage's
    // quadratic forall-preference expands to crates²·storeareas² instances, of
    // which ~90%+ are statically satisfied; dropping them here is what makes
    // p03+ (1601/4211 raw instances) searchable at all. Survivors keep the
    // simplified phi (cheaper DNF at grounding). The independent verifier
    // scores from the ORIGINAL goal, so reported metrics are unaffected.
    // `FF_PREF_NO_STATIC=1` restores the blind expansion.
    if std::env::var("FF_PREF_NO_STATIC").is_err() {
        let statics = static_predicates(domain);
        let init: HashSet<(Sym, Vec<Sym>)> = problem.init_atoms.iter().cloned().collect();
        prefs = prefs
            .into_iter()
            .filter_map(|(name, phi)| match peval_static(&phi, &statics, &init) {
                Formula::True => None,
                simplified => Some((name, simplified)),
            })
            .collect();
        if std::env::var("FF_RES_DEBUG").is_ok() && prefs.len() < n_prefs_total {
            eprintln!(
                "[P3] static simplification: dropped {} of {} preference instance(s)",
                n_prefs_total - prefs.len(),
                n_prefs_total
            );
        }
    }

    let mut w = HashMap::new();
    let mut tc = 0.0;
    let mut others: HashMap<String, f64> = HashMap::new();
    let mut konst = 0.0;
    let mut other = false;
    // NORMALIZE the direction: `maximize E` is compiled as `minimize -E` by
    // extracting with scale -1 — IPC6 net-benefit (`maximize (- CONST (+
    // (total-cost) (* (is-violated p) w)...))`) then lands EXACTLY in the
    // supported minimize class (tc = 1, positive weights), and the dropped
    // affine constant is carried in `metric_konst` so reporting can
    // reconstruct the original metric's value (net benefit = -(cost+konst)).
    let maximized = matches!(&problem.metric, Some((MetricDir::Maximize, _)));
    match &problem.metric {
        Some((MetricDir::Minimize, e)) => {
            extract(e, 1.0, &mut w, &mut tc, &mut others, &mut konst, &mut other);
        }
        Some((MetricDir::Maximize, e)) => {
            extract(
                e,
                -1.0,
                &mut w,
                &mut tc,
                &mut others,
                &mut konst,
                &mut other,
            );
        }
        None => {}
    }
    let metric_absent = problem.metric.is_none();
    let _ = tc;

    let mut d = domain.clone();
    let mut p = problem.clone();
    if !d.functions.iter().any(|(n, _)| n == COST) {
        d.functions.push((COST.to_string(), vec![]));
    }
    if !p.init_fluents.iter().any(|((n, _), _)| n == COST) {
        p.init_fluents.push(((COST.to_string(), vec![]), 0.0));
    }

    // FOLD monotone numeric metric terms (e.g. `(sum-traverse-cost)` in rovers)
    // into total-cost: mirror every `(increase f X)` with `(increase total-cost
    // coeff*X)`. Then total-cost == the FULL metric, and the existing single-cost
    // B&B optimizes + reports it correctly. Terms that can't be folded (non-
    // monotone, under forall) are left out and surfaced via `warn_other`.
    let mut metric_other = other;
    let mut folded_metric = false;
    for (fname, &coeff) in &others {
        if coeff == 0.0 {
            continue;
        }
        // A NEGATIVE (post-normalization) coefficient would mirror as a
        // negative increase on total-cost — silently breaking the B&B's
        // monotonicity. A maximized reward fluent lands here; optimize the
        // supported part only and say so.
        if coeff < 0.0 || fluent_foldable(domain, problem, fname).is_some() {
            metric_other = true; // optimize the supported part only
            continue;
        }
        for a in &mut d.actions {
            let mut mirror = Vec::new();
            collect_cost_mirror(&a.effect, fname, coeff, &mut mirror);
            if !mirror.is_empty() {
                folded_metric = true;
                let mut v = vec![a.effect.clone()];
                v.append(&mut mirror);
                a.effect = Effect::And(v);
            }
        }
    }

    // Precondition preferences: split each action with soft preconditions into
    // satisfied/violated variants (same name). The satisfied variant requires the
    // soft condition (free); the violated variant requires its negation and pays
    // the weight. Because the variants are mutually exclusive and the planner
    // applies exactly one per use, `(is-violated p)` counts per-application
    // violations EXACTLY (no over-count from disjunctive negations — applying one
    // grounded op charges the weight once).
    let mut pp_negative = false;
    let mut pp_overflow = false;
    let mut new_actions = Vec::new();
    let mut pctr = 0usize;
    for a in &d.actions {
        let (hard_pre, pprefs) = extract_precond_prefs(&a.precond, &mut pctr);
        if pprefs.is_empty() {
            new_actions.push(a.clone());
            continue;
        }
        let k = pprefs.len();
        if k > 6 {
            pp_overflow = true;
            new_actions.push(Action {
                precond: hard_pre,
                ..a.clone()
            });
            continue;
        }
        for mask in 0u32..(1u32 << k) {
            let mut conj = vec![hard_pre.clone()];
            let mut cost = 0.0;
            for (i, (name, phi)) in pprefs.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    conj.push(Formula::Not(Box::new(phi.clone()))); // violated
                    let raw = w
                        .get(name)
                        .copied()
                        .unwrap_or(if metric_absent { 1.0 } else { 0.0 });
                    if raw < 0.0 {
                        pp_negative = true;
                    }
                    cost += raw.max(0.0);
                } else {
                    conj.push(phi.clone()); // satisfied
                }
            }
            let mut eff = vec![a.effect.clone()];
            if cost != 0.0 {
                eff.push(Effect::Num(
                    AssignOp::Increase,
                    COST.to_string(),
                    vec![],
                    Expr::Num(cost),
                ));
            }
            new_actions.push(Action {
                name: a.name.clone(),
                params: a.params.clone(),
                precond: Formula::And(conj),
                effect: Effect::And(eff),
                // a 2^k variant of a REAL action — it keeps applying the
                // shared monitor block (0.8 Phase 2)
                monitored: a.monitored,
            });
        }
    }
    d.actions = new_actions;

    // End-marker phasing (Keyder–Geffner): simple preferences are evaluated in
    // the FINAL state, so collect/forgo must run only after planning ends — else
    // a preference could be "collected" while transiently true mid-plan. Real
    // actions require (P3PLANNING); `end` flips to (P3ENDED) and freezes the
    // state; collect_p (which checks phi) and forgo_p require (P3ENDED).
    const PLANNING: &str = "P3PLANNING";
    const ENDED: &str = "P3ENDED";
    d.predicates.push((PLANNING.to_string(), vec![]));
    d.predicates.push((ENDED.to_string(), vec![]));
    p.init_atoms.push((PLANNING.to_string(), vec![]));
    for a in &mut d.actions {
        a.precond = Formula::And(vec![
            Formula::Atom(PLANNING.to_string(), vec![]),
            a.precond.clone(),
        ]);
    }
    let mut synthetic = HashSet::new();
    synthetic.insert("P3END".to_string());
    d.actions.push(Action {
        name: "P3END".to_string(),
        params: vec![],
        monitored: false,
        precond: Formula::Atom(PLANNING.to_string(), vec![]),
        effect: Effect::And(vec![
            Effect::Del(PLANNING.to_string(), vec![]),
            Effect::Add(ENDED.to_string(), vec![]),
        ]),
    });

    let mut goal_parts = hard;
    let mut any_negative = false;
    // (forgo-action name, weight) per preference — lets the optimizer force-collect
    // high-weight preferences (forbid forgoing them) during relax-and-tighten.
    let mut forgos: Vec<(String, f64)> = Vec::new();
    for (i, (name, phi)) in prefs.iter().enumerate() {
        let col = format!("P3COLLECTED-{}", i);
        d.predicates.push((col.clone(), vec![]));
        let collect = format!("P3COLLECT-{}", i);
        let forgo = format!("P3FORGO-{}", i);
        synthetic.insert(collect.clone());
        synthetic.insert(forgo.clone());
        // collect: phi must hold in the FINAL (ended) state — free
        d.actions.push(Action {
            name: collect,
            params: vec![],
            precond: Formula::And(vec![Formula::Atom(ENDED.to_string(), vec![]), phi.clone()]),
            effect: Effect::Add(col.clone(), vec![]),
            monitored: false,
        });
        // forgo: skip it, paying its weight (clamped >= 0 to keep cost monotone;
        // a negative weight is flagged unsupported below, not silently applied)
        let raw = w
            .get(name)
            .copied()
            .unwrap_or(if metric_absent { 1.0 } else { 0.0 });
        if raw < 0.0 {
            any_negative = true;
        }
        forgos.push((format!("P3FORGO-{}", i), raw.max(0.0)));
        d.actions.push(Action {
            name: forgo,
            params: vec![],
            monitored: false,
            precond: Formula::Atom(ENDED.to_string(), vec![]),
            effect: Effect::And(vec![
                Effect::Add(col.clone(), vec![]),
                Effect::Num(
                    AssignOp::Increase,
                    COST.to_string(),
                    vec![],
                    Expr::Num(raw.max(0.0)),
                ),
            ]),
        });
        goal_parts.push(Formula::Atom(col, vec![]));
    }
    // require the end marker so the planner closes the planning phase
    goal_parts.push(Formula::Atom(ENDED.to_string(), vec![]));
    p.goal = Formula::And(goal_parts);

    // determine whether the metric is inside the supported (optimizable)
    // class. Maximize is handled by normalization above, so the ladder tests
    // the NORMALIZED coefficients — a maximize whose normalization lands
    // outside the class (e.g. maximized total-cost => tc = -1) still gets an
    // honest refusal below rather than a silently wrong objective.
    let unsupported = if any_negative || pp_negative {
        Some("negative preference weight (cannot be encoded monotonically)".into())
    } else if pp_overflow {
        Some("an action has too many precondition preferences (>6)".into())
    } else if !(tc == 0.0 || tc == 1.0) {
        Some(format!(
            "scaled total-cost coefficient ({}) is not supported",
            tc
        ))
    } else if !cost_monotone(domain, problem) {
        Some("non-monotone total-cost (decrease/scale effects) breaks branch-and-bound".into())
    } else {
        None
    };

    Compiled {
        domain: d,
        problem: p,
        minimize: !maximized,
        maximized,
        metric_konst: konst,
        // full pre-simplification count: statically-satisfied instances are
        // still real preferences (satisfied ones), so reporting stays stable
        n_prefs: n_prefs_total,
        warn_other: metric_other,
        unsupported,
        synthetic,
        forgos,
        folded_metric,
    }
}

/// What the cost fluent reads once `ops` has run clean, start to finish.
pub(crate) fn plan_cost(task: &PackedTask, ops: &[usize], cf: usize) -> f64 {
    let mut s = task.initial();
    for &oi in ops {
        s = task.apply(oi, &s);
    }
    if s.fdef[cf] {
        s.fv[cf]
    } else {
        0.0
    }
}

pub struct MetricResult {
    pub ops: Vec<usize>,
    pub cost: f64,
    pub iterations: usize,
    /// True only when the search burned the whole space dry and proved no
    /// cheaper plan exists. False when a resource bound — MAX_EVAL,
    /// MAX_ITERS — cut it off first; `cost` is then just the best found,
    /// no proof behind it.
    pub proven: bool,
}

/// Drive `cost_fluent` — the total weight of broken promises — down through
/// relax-and-tighten: an EHC first incumbent, then SGPlan-style
/// force-collect tightening (the highest-weight preferences get strong-armed
/// into satisfaction), then a bounded branch-and-bound polish to finish.
/// `forgos` are the (op-id, weight) roster of the synthetic forgo actions.
/// Default per-`occupancy²` weight for the renewable-resource guidance term.
///
/// **Off by default — on purpose, not by oversight.** A swept run
/// (FF_RES_WEIGHT × FF_RES_THRESH on openstacks p01–p05) confirmed a soft
/// occupancy penalty never lowers the metric: small weights make it worse
/// — penalizing live occupancy chokes the start→make→ship pipeline the
/// plan actually needs, so a started-but-unshipped order gets hit twice,
/// once for the forgone preference and once for occupancy — and large
/// thresholds just sit inert. There's a reason for that, not a gap:
/// openstacks runs min-open-stacks scheduling, where an order's products
/// must be built while the order is open, forcing orders sharing a
/// product to stay open together — the MOSP/pathwidth constraint. That's
/// a combinatorial peak/throughput objective, and no per-state penalty
/// touches it. Closing that gap takes the ESPC partition+penalty loop, or
/// a real scheduler — not this term.
///
/// The detection and the concrete-state hook stay on the books as the
/// foundation for capacity-aware scheduling to come — numeric resources,
/// and renewable-resource feasibility in the temporal planner, where
/// capacity is a hard wall, the case that actually matters for durative
/// resource allocation. Override at runtime with `FF_RES_WEIGHT` /
/// `FF_RES_THRESH` to run your own experiment.
const RES_WEIGHT_DEFAULT: i64 = 0;

/// Default `SearchCfg::w_c` for the folded-numeric-metric legacy
/// branch-and-bound. Zero, and zero on purpose: the 2026-07 rovers p01–p08
/// sweep (w_c ∈ {0, 0.25, 0.5, 1, 2, 5}) showed every non-zero weight
/// collapsing quality straight down to the all-forgo floor — p01 went
/// 935.3 to 1162.1, p07 came back with nothing at all. Accumulated cost
/// only ever climbs along a path, so ordering by it buries the deep
/// goal-reaching prefixes the tightening loop needs beneath shallow, cheap
/// decoys, and the bounded searches stop finding any plan under the
/// incumbent. The closure-path probe read neutral, identical metrics
/// either way. What actually moved rovers was the escalating retry — p02
/// climbed 659.3 to 596.7, p05 climbed 649.9 to 523.3. `w_c` stays on the
/// shelf for anyone who wants to run it again, via `FF_PREF_COST_WEIGHT`.
const COST_WEIGHT_FOLDED_DEFAULT: f64 = 0.0;

pub fn metric_optimize(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64)],
    groups: &[Vec<u32>],
    folded_metric: bool,
    threads: usize,
) -> Option<MetricResult> {
    const MAX_ITERS: usize = 10_000;
    let init = task.initial();

    // 1. SATISFACTION GUIDANCE (pure analysis, built before any search) — the
    // earlier "force-collect" variants all failed
    // because under delete-relaxation the free forgo action makes every preference
    // look reachable, so the heuristic was blind to satisfaction (see
    // docs/espc-preferences-spec.md). Instead, bias the B&B open list by a penalty
    // that counts preferences forgone in the CONCRETE state — this sees real
    // satisfaction and gives the search a gradient toward delivering, breaking the
    // all-forgo floor. (It still can't see the openstacks `stacks-avail` resource —
    // that needs the SAS+ partition + penalty loop — so it narrows, not closes, the
    // gap.) Built from each preference's P3COLLECT-i `phi` precondition.
    let mut sat = build_sat_guidance(task, forgos);
    // Resource-aware guidance foundation: detect any renewable "counter" resource
    // the delete-relaxed heuristic is blind to (e.g. openstacks' stacks-avail).
    // The occupancy penalty in SatGuidance is OFF by default — see
    // RES_WEIGHT_DEFAULT for why a soft penalty can't crack openstacks; this stays
    // as the substrate for capacity-aware scheduling. Empty (no-op) on domains
    // with no such resource. Tunable via FF_RES_WEIGHT / FF_RES_THRESH.
    sat.res = crate::resource::detect_resources(task, groups, &task.init_bits);
    if !sat.res.is_empty() {
        sat.res_weight = std::env::var("FF_RES_WEIGHT")
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(RES_WEIGHT_DEFAULT);
        sat.res_thresh = std::env::var("FF_RES_THRESH")
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        if std::env::var("FF_RES_DEBUG").is_ok() {
            let caps: Vec<usize> = sat.res.iter().map(|r| r.members.len() - 1).collect();
            eprintln!(
                "[C3] {} renewable resource(s), capacities {:?}; w={} thresh={}",
                sat.res.len(),
                caps,
                sat.res_weight,
                sat.res_thresh
            );
        }
    }

    // ESPC make-deadline guidance. Penalizes once-only conditional achievements
    // that fire without delivering (openstacks: a product made while its orders
    // still wait — a permanently locked metric loss the delete-relaxed RPG is blind
    // to). Built unconditionally (pure analysis, inert on domains without the
    // structure); only the heap WEIGHT is gated, so the default path stays
    // bit-identical until a flag is set.
    sat.deadline = build_deadline_guidance(task, forgos);
    let refine_cfg = SearchCfg::from_weights(1.0, 5.0, Some(300_000));
    // Cost-aware open-list ordering (see `SearchCfg::w_c`) — experimental,
    // default OFF everywhere: the sweep that was meant to pick a folded-metric
    // default found it collapses rovers instead (see COST_WEIGHT_FOLDED_DEFAULT
    // for the measured post-mortem). `FF_PREF_COST_WEIGHT` enables it for
    // experiments on either metric loop.
    let cost_w = std::env::var("FF_PREF_COST_WEIGHT")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(if folded_metric {
            COST_WEIGHT_FOLDED_DEFAULT
        } else {
            0.0
        });

    // FULL ESPC (FF_ESPC): an adaptive per-trigger penalty-resolution outer loop.
    // It re-solves under fixed penalties, raises the penalty on triggers whose
    // deliveries were missed, and keeps the best plan as an anytime incumbent,
    // terminating at a saddle point / stall / budget. Auto-tunes the penalty per
    // instance (no manual weight) and never claims optimality. See `crate::espc`
    // and docs/espc-preferences-spec.md. Seeded by the same compiled-goal EHC
    // pass as always — this branch is deliberately untouched by the closure path.
    if crate::features::espc() && !sat.deadline.is_empty() {
        let mut best: Option<(Vec<usize>, f64)> = None;
        let first = plan(
            task,
            threads,
            SearchCfg::from_weights(1.0, 5.0, Some(1_500_000)),
            true,
        );
        if let Some(ops) = first.ops {
            let cost = plan_cost(task, &ops, cost_fluent);
            if cost <= 0.0 {
                return Some(MetricResult {
                    ops,
                    cost,
                    iterations: 0,
                    proven: true,
                });
            }
            best = Some((ops, cost));
        }
        let part = build_espc_partition(task, forgos, groups, &sat);
        return crate::espc::espc_optimize(
            task,
            cost_fluent,
            &mut sat,
            best.clone(),
            part,
            threads,
            refine_cfg,
        )
        .map(|r| MetricResult {
            ops: r.ops,
            cost: r.cost,
            iterations: r.iterations,
            proven: false, // anytime: a saddle point is not a global-optimality proof
        })
        .or_else(|| {
            best.map(|(ops, cost)| MetricResult {
                ops,
                cost,
                iterations: 0,
                proven: false,
            })
        });
    }

    // Phase-0 lever (OFF by default): a FIXED deadline penalty for manual sweeps /
    // ablation. With it unset the heap key is bit-identical to the non-ESPC path.
    if !sat.deadline.is_empty() {
        sat.deadline_weight = std::env::var("FF_DEADLINE_WEIGHT")
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0);
        if std::env::var("FF_RES_DEBUG").is_ok() {
            eprintln!(
                "[ESPC] {} deadline pair(s), fixed lambda={}",
                sat.deadline.len(),
                sat.deadline_weight
            );
        }
    }

    // 2. EXACT-CLOSURE METRIC SEARCH (the default for pure-preference metrics):
    // search REAL states only and close the preference bookkeeping with the
    // exact phase tail, instead of searching a hard goal made of hundreds/
    // thousands of `P3COLLECTED-i` facts with a satisfaction-blind heuristic
    // (the storage p03+ wall, and the tpp budget sink). Precondition-preference
    // variant costs on real ops are fine — they accrue in `g`, which the
    // acceptance test sums with the closure exactly. What is NOT routed here is
    // FOLDED numeric metrics (rovers' mirrored traverse costs) route here TOO
    // since 0.5: the 0.4.0 verdict that the closure search measures worse on
    // them (tiny-epsilon tightening churn to MAX_ITERS, a poorer incumbent
    // than the EHC seed) was an artifact of first-improvement restarts — with
    // anytime sweeps the closure path dominates the legacy B&B on every
    // rovers instance (p01 935.3→811.3 ties SGPlan5, p04 485.5→418.7 and p06
    // 664.6→655.7 beat it, p05 483.6 ties; the domain flips to a lead under
    // both quality conventions). `FF_PREF_NUMLEGACY=1` restores the pre-0.5
    // split (folded → legacy); `FF_PREF_COMPILED=1` routes EVERYTHING legacy.
    // Also falls back when the closure search cannot produce an incumbent.
    let numlegacy = folded_metric && std::env::var("FF_PREF_NUMLEGACY").is_ok();
    if !forgos.is_empty() && !numlegacy && std::env::var("FF_PREF_COMPILED").is_err() {
        if let Some(tail) = build_phase_tail(task, forgos) {
            if let Some(r) = metric_optimize_closure(
                task,
                cost_fluent,
                forgos,
                &tail,
                &sat,
                groups,
                threads,
                refine_cfg.with_cost_weight(cost_w),
            ) {
                return Some(r);
            }
        }
    }

    // 3. LEGACY compiled-goal path: EHC seed on the full compiled goal, then the
    // bounded polish B&B from the incumbent. Reached only via `FF_PREF_COMPILED=1`
    // or the closure fallback above. The tightening loop shares the closure
    // path's deterministic eval-count budget and capped-failure escalation
    // (see `metric_optimize_closure`); the 1.5M EHC seed stays outside the
    // budget, mirroring the closure path's free init-tail incumbent.
    let mut bound = f64::INFINITY;
    let mut best: Option<(Vec<usize>, f64)> = None;
    let mut iterations = 0;
    let mut proven = false;
    let first = plan(
        task,
        threads,
        SearchCfg::from_weights(1.0, 5.0, Some(1_500_000)),
        true,
    );
    if let Some(ops) = first.ops {
        let cost = plan_cost(task, &ops, cost_fluent);
        if cost <= 0.0 {
            return Some(MetricResult {
                ops,
                cost,
                iterations: 0,
                proven: true,
            });
        }
        bound = cost;
        best = Some((ops, cost));
    }

    // FORGO-AWARE SECOND SEED (completion pricing) — experimental, opt-in via
    // `FF_PREF_SEED=1`, default OFF after measuring NEUTRAL (2026-07): the
    // idea is to price what a preference COSTS to deliver (the relaxation is
    // blind to it — on rovers a forced traverse round-trip can cost more than
    // the preference's weight, and prefix-cost ordering `w_c` was a measured
    // dead end because cost only grows along a path). Estimate each
    // preference's delivery cost with a cost-aware relaxed plan from the
    // initial state (`heuristic::relaxed_plan_cost`), pre-forgo any preference
    // whose estimate exceeds its weight (forbid its P3COLLECT ops) in ONE
    // extra seeded solve, and keep the cheaper of the two incumbents.
    // MEASURED: on rovers the estimates fire correctly (p01: est 157 vs
    // weight 76.5 → pre-forgo) but the plain EHC seed already lands at the
    // same incumbent cost, and final metrics are identical with the seed on
    // or off (p01–p08). The residual rovers gap lives in the B&B's reachable
    // trade curve, not the seed bound — the diversified restart ladder is
    // what moved it (p04 559.9 → 485.5). Machinery kept for experiments;
    // a wrong estimate can never hurt quality (min of both seeds).
    if !forgos.is_empty() && std::env::var("FF_PREF_SEED").is_ok() {
        let mut sc = crate::heuristic::Scratch::new(task);
        let collect = collect_ops(task);
        let mut banned: Vec<bool> = vec![false; task.n_ops];
        let mut any = false;
        for (i, (_, weight)) in forgos.iter().enumerate() {
            let Some(ops_i) = collect.get(&i) else {
                continue;
            };
            // completion estimate = the cheapest disjunct's relaxed-plan cost
            let mut est = f64::INFINITY;
            for &oi in ops_i {
                let pos: Vec<u32> = task
                    .pre_pos
                    .slice(oi)
                    .iter()
                    .copied()
                    .filter(|&f| {
                        !task.fact_names[f as usize]
                            .to_ascii_uppercase()
                            .starts_with("(P3")
                    })
                    .collect();
                let c = crate::heuristic::relaxed_plan_cost(
                    task,
                    &mut sc,
                    &init.bits,
                    &init.fv,
                    &init.fdef,
                    &pos,
                    task.pre_num.slice(oi),
                    cost_fluent,
                )
                .unwrap_or(f64::INFINITY);
                est = est.min(c);
            }
            if std::env::var("FF_RES_DEBUG").is_ok() {
                eprintln!("[seed] pref {i}: completion est {est:.1} vs weight {weight:.1}");
            }
            if est > *weight {
                for &oi in ops_i {
                    banned[oi] = true;
                }
                any = true;
            }
        }
        if any {
            let (seeded, _evaluated) = crate::search::solve_subgoal_guided(
                task,
                &init,
                &task.goal_pos,
                &task.goal_num,
                &banned,
                threads,
                SearchCfg::from_weights(1.0, 5.0, Some(1_500_000)),
                None,
            );
            if std::env::var("FF_RES_DEBUG").is_ok() {
                let c = seeded.as_ref().map(|o| plan_cost(task, o, cost_fluent));
                eprintln!("[seed] seeded solve: {c:?} vs EHC incumbent {bound:.1}");
            }
            if let Some(ops) = seeded {
                let cost = plan_cost(task, &ops, cost_fluent);
                if cost < bound {
                    if cost <= 0.0 {
                        return Some(MetricResult {
                            ops,
                            cost,
                            iterations: 0,
                            proven: true,
                        });
                    }
                    bound = cost;
                    best = Some((ops, cost));
                }
            }
        }
    }
    let budget = pref_eval_budget();
    let no_escalate = std::env::var("FF_PREF_NO_ESCALATE").is_ok();
    // Anytime in-sweep tightening + the diversified restart ladder, exactly as
    // in the closure loop (acceptance here is plain `final cost < bound` — no
    // closure term). Same hatches (`FF_PREF_GREEDY`, `FF_PREF_NO_RESTARTS`).
    const PROFILES: &[(f64, f64)] = &[(1.0, 2.0), (1.0, 8.0), (2.0, 3.0), (0.0, 1.0)];
    let anytime = std::env::var("FF_PREF_GREEDY").is_err();
    let restarts = anytime && std::env::var("FF_PREF_NO_RESTARTS").is_err();
    let mut spent = 0usize;
    let mut escalated = false;
    let mut rung = 0usize;
    while iterations < MAX_ITERS && spent < budget {
        iterations += 1;
        let cap = if escalated {
            budget - spent
        } else if rung > 0 {
            // Half-size diversification rungs — see the closure loop.
            (refine_cfg.max_eval / 2).min(budget - spent)
        } else {
            refine_cfg.max_eval.min(budget - spent)
        };
        let iter_cfg = if rung == 0 {
            SearchCfg {
                max_eval: cap,
                anytime,
                ..refine_cfg.with_cost_weight(cost_w)
            }
        } else {
            let (wg, wh) = PROFILES[rung - 1];
            SearchCfg {
                max_eval: cap,
                anytime,
                ..SearchCfg::from_weights(wg, wh, Some(cap)).with_cost_weight(cost_w)
            }
        };
        let (opt, evaluated, capped) = solve_subgoal_bounded(
            task,
            &init,
            &task.goal_pos,
            &task.goal_num,
            cost_fluent,
            bound,
            threads,
            iter_cfg,
            Some(&sat),
        );
        spent += evaluated;
        match opt {
            Some(ops) => {
                escalated = false;
                rung = 0; // an improvement re-arms the ladder
                let cost = plan_cost(task, &ops, cost_fluent);
                best = Some((ops, cost));
                if cost <= 0.0 {
                    proven = true; // cannot beat zero cost
                    break;
                }
                bound = cost; // next plan must be strictly cheaper (prune cost >= bound)
            }
            None => {
                // A capped failure is inconclusive: diversify the open-list
                // order first (same bound, different region), then retry the
                // same bound with all remaining budget (see the closure
                // loop's rationale). Proven optimal IFF a retry-exempt
                // failure exhausted the space un-capped.
                if capped && !no_escalate {
                    if restarts && rung < PROFILES.len() && budget > spent {
                        rung += 1;
                        continue;
                    }
                    if budget.saturating_sub(spent) > cap {
                        escalated = true;
                        rung = 0;
                        continue;
                    }
                }
                proven = !capped;
                break;
            }
        }
    }
    best.map(|(ops, cost)| MetricResult {
        ops,
        cost,
        iterations,
        proven,
    })
}

/// The shared tightening allowance, counted in evaluated states — never
/// the clock — spent by both metric branch-and-bound loops alike. Results
/// come out the same regardless of thread count.
fn pref_eval_budget() -> usize {
    std::env::var("FF_PREF_EVAL_BUDGET")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(2_000_000)
}

/// Each preference's DNF disjunct, laid out as fact-sets — the
/// `P3COLLECT-i` ops' non-P3 positive precondition facts, one set per
/// disjunct. Same extraction the guidance and the seeds already run,
/// surfaced once here for the selection layer.
fn pref_dnf(
    task: &PackedTask,
    forgos: &[(usize, f64)],
) -> crate::hash::FxHashMap<usize, Vec<Vec<u32>>> {
    let collect = collect_ops(task);
    let mut dnf: crate::hash::FxHashMap<usize, Vec<Vec<u32>>> = crate::hash::FxHashMap::default();
    for (i, _) in forgos.iter().enumerate() {
        let Some(ops_i) = collect.get(&i) else {
            continue;
        };
        let djs: Vec<Vec<u32>> = ops_i
            .iter()
            .filter(|&&oi| task.pre_num.slice(oi).is_empty())
            .map(|&oi| {
                task.pre_pos
                    .slice(oi)
                    .iter()
                    .copied()
                    .filter(|&f| {
                        !task.fact_names[f as usize]
                            .to_ascii_uppercase()
                            .starts_with("(P3")
                    })
                    .collect()
            })
            .collect();
        if !djs.is_empty() {
            dnf.insert(i, djs);
        }
    }
    dnf
}

/// The selection seed (0.6 headline; docs/forensics-tpp.md): solve the
/// preference-subset selection exactly (`crate::selection`), then aim at
/// the chosen facts as one concrete hard-goal target — the kind of
/// coordination the h-guided search structurally cannot see coming (goods5
/// held at L2 purely so goods6 can match it later). Nothing in the model
/// guarantees the chosen facts are jointly reachable, so a failed target
/// run bans the fact with the costliest relaxed completion — the best
/// deterministic guess at the culprit — re-selects, and tries again,
/// bounded by `MAX_REPAIRS` and its own slice of the budget. Returns the
/// incumbent candidate plus the selection bound; when `final == bound`,
/// that's proof, not a guess, that it's optimal. All of it billed to the
/// caller's deterministic budget.
type SeedOutcome = (Option<(Vec<usize>, f64)>, Option<f64>, usize);

#[allow(clippy::too_many_arguments)]
fn selection_seed(
    task: &PackedTask,
    cost_fluent: usize,
    groups: &[Vec<u32>],
    forgos: &[(usize, f64)],
    tail: &PhaseTail,
    p3_mask: &[bool],
    threads: usize,
    cfg: SearchCfg,
    budget: usize,
) -> SeedOutcome {
    // ONE joint attempt: per-fact unreachability is fully handled by the
    // probes BEFORE the attempt (tpp's supply caps), and a target that is
    // jointly infeasible despite clean probes fails for reasons no ban or
    // core-subset retry repairs (both measured: storage's counting
    // infeasibility, trucks' shared-timeline scheduling — the retry only
    // added wall time, p08 213 s → 341 s, and changed nothing).
    const MAX_REPAIRS: usize = 8;
    let dbg = std::env::var("FF_RES_DEBUG").is_ok();
    let weights: Vec<f64> = forgos.iter().map(|&(_, w)| w).collect();
    let dnf = pref_dnf(task, forgos);
    let init = task.initial();
    let real_goals: Vec<u32> = task
        .goal_pos
        .iter()
        .copied()
        .filter(|&f| {
            !task.fact_names[f as usize]
                .to_ascii_uppercase()
                .starts_with("(P3")
        })
        .collect();
    let mut banned: crate::hash::FxHashSet<u32> = crate::hash::FxHashSet::default();
    let mut spent = 0usize;
    let seed_slice = budget / 4;
    // Singleton pre-probe cap: an actually-reachable end-state fact on these
    // domains solves in hundreds of evals; a supply-capped one exhausts. A
    // capped probe is INCONCLUSIVE unreachability but conclusive "too hard
    // to be a seed target" — banning it only weakens the seed, never the
    // final result (min-incumbent).
    const PROBE_CAP: usize = 5_000;
    let mut probed: crate::hash::FxHashMap<u32, bool> = crate::hash::FxHashMap::default();
    let mut bound_out = None;
    for round in 0..=MAX_REPAIRS {
        let Some(sel) = crate::selection::select(task, groups, &weights, &dnf, &banned) else {
            break;
        };
        if sel.capped {
            // Node-capped DFS: best-found assignment, not the model optimum —
            // neither a trustworthy target nor an admissible bound. Skip the
            // whole layer rather than burn the seed slice on a junk target
            // (measured: storage p08, 83 → 104).
            if dbg {
                eprintln!("[sel] skip: selection DFS node-capped");
            }
            return (None, None, spent);
        }
        if round == 0 {
            bound_out = Some(sel.bound); // the un-banned bound is the admissible one
        }
        // Pre-probe every chosen fact not yet true at init; ban the suspects.
        let mut newly_banned = false;
        let chosen_facts: Vec<u32> = {
            let mut v: Vec<u32> = sel
                .chosen
                .iter()
                .flat_map(|(_, fs)| fs.iter().copied())
                .filter(|&f| !crate::bitset::test(&init.bits, f as usize))
                .collect();
            v.sort_unstable();
            v.dedup();
            v
        };
        for &f in &chosen_facts {
            if spent >= seed_slice {
                break;
            }
            let ok = *probed.entry(f).or_insert_with(|| {
                let (ops, evaluated) = crate::search::solve_subgoal_guided(
                    task,
                    &init,
                    &[f],
                    &[],
                    p3_mask,
                    threads,
                    SearchCfg {
                        max_eval: PROBE_CAP,
                        ..cfg
                    },
                    None,
                );
                spent += evaluated;
                ops.is_some()
            });
            if !ok && banned.insert(f) {
                newly_banned = true;
                if dbg {
                    eprintln!("[sel] probe bans {}", task.fact_names[f as usize]);
                }
            }
        }
        if newly_banned {
            continue; // re-select under the new bans before attempting a target
        }
        // Joint target attempt with the remaining slice.
        let mut target: Vec<u32> = real_goals.clone();
        target.extend(chosen_facts.iter().copied());
        target.sort_unstable();
        target.dedup();
        if target.is_empty() || spent >= seed_slice {
            break;
        }
        let stage_cfg = SearchCfg {
            max_eval: cfg.max_eval.min(seed_slice - spent),
            ..cfg
        };
        let (ops, evaluated) = crate::search::solve_subgoal_guided(
            task,
            &init,
            &target,
            &task.goal_num,
            p3_mask,
            threads,
            stage_cfg,
            None,
        );
        spent += evaluated;
        match ops {
            Some(mut ops) => {
                let mut s = init.clone();
                for &oi in &ops {
                    s = task.apply(oi, &s);
                }
                let Some(tail_ops) = apply_tail(task, &mut s, tail) else {
                    break;
                };
                ops.extend(tail_ops);
                if !task.goal_met_with(&s, &task.goal_pos, &task.goal_num) {
                    break;
                }
                let cost = plan_cost(task, &ops, cost_fluent);
                if dbg {
                    eprintln!(
                        "[sel] round {round}: target of {} facts reached, cost {cost} \
                         (selection bound {:.1}), {spent} evals",
                        target.len(),
                        sel.bound
                    );
                }
                return (Some((ops, cost)), bound_out, spent);
            }
            None => {
                // Joint failure with individually-probed facts = interaction
                // the model cannot express (counting/scheduling). No repair
                // helps (measured); give up and keep the loop's full budget.
                if dbg {
                    eprintln!("[sel] round {round}: joint target failed; no seed");
                }
                break;
            }
        }
    }
    if dbg {
        eprintln!("[sel] no reachable selection target ({spent} evals spent)");
    }
    (None, bound_out, spent)
}

/// The partitioned closure seed — ESPC increment 3, generalized past
/// deadline pairs: build a high-quality incumbent out of per-component
/// stages before the monolithic tightening loop even gets a turn.
/// Motivation, measured not guessed: the remaining tpp/pathways/trucks
/// tails are direction-bound — identical metrics even at 4× the eval
/// budget — so the fix has to be a structurally different plan
/// constructor, not a bigger number. The construction, step by step:
///
/// 1. Candidate selection — for every unsatisfied preference, price its
///    cheapest positive disjunct with a cost-aware relaxed plan off the
///    initial state (`heuristic::relaxed_plan_cost`); keep it only if the
///    estimate stays under its violation weight — deliverable at a
///    profit. Real hard-goal facts enter as mandatory, no pricing needed.
/// 2. Components — union-find over the candidates' facts through the
///    invariant-synthesis mutex variables; two candidates interact only
///    when their facts share a variable, ungrouped facts stand alone.
///    Needs at least 2 components before this diverges from the
///    monolithic path at all.
/// 3. Composition — one P3-masked, satisfaction-guided stage per
///    component, run on the evolving state in deterministic order (min
///    fact id first). Mandatory facts from finished components are
///    protected — no op gets to delete them. An infeasible stage drops
///    its priciest optional preference and tries again; a stage that
///    can't even hold its mandatory facts kills the seed outright.
/// 4. The exact phase tail closes the bookkeeping, and the composed plan
///    becomes the tightening loop's opening incumbent only if it beats
///    the plain init-tail one. Every eval the stages spend gets charged
///    against the same deterministic budget the loop is already burning.
///
/// `FF_PREF_MONO=1` shuts the composed seed off — falls back to the
/// monolithic path, bit-for-bit compatible.
#[allow(clippy::too_many_arguments)]
fn compose_pref_seed(
    task: &PackedTask,
    cost_fluent: usize,
    groups: &[Vec<u32>],
    forgos: &[(usize, f64)],
    tail: &PhaseTail,
    sat: &SatGuidance,
    p3_mask: &[bool],
    threads: usize,
    cfg: SearchCfg,
    budget: usize,
) -> Option<(Vec<usize>, f64, usize)> {
    use crate::types::eval_numpre;
    let init = task.initial();
    let mut spent = 0usize;

    // 1. Candidates: mandatory real goals + profitably-satisfiable preferences.
    struct Cand {
        pos: Vec<u32>,
        num: Vec<crate::types::NumPre>,
        est: f64,
        value: f64, // weight - est (mandatory: +inf); the mutex-conflict tiebreak
        mandatory: bool,
    }
    let mut cands: Vec<Cand> = Vec::new();
    for &g in task.goal_pos.iter().filter(|&&f| {
        !task.fact_names[f as usize]
            .to_ascii_uppercase()
            .starts_with("(P3")
    }) {
        cands.push(Cand {
            pos: vec![g],
            num: Vec::new(),
            est: 0.0,
            value: f64::INFINITY,
            mandatory: true,
        });
    }
    let collect = collect_ops(task);
    let mut sc = crate::heuristic::Scratch::new(task);
    for (i, _) in forgos.iter().enumerate() {
        let Some(ops_i) = collect.get(&i) else {
            continue;
        };
        let weight = forgos[i].1;
        let mut cheapest: Option<(Vec<u32>, Vec<crate::types::NumPre>, f64)> = None;
        let mut already = false;
        for &oi in ops_i {
            let pos: Vec<u32> = task
                .pre_pos
                .slice(oi)
                .iter()
                .copied()
                .filter(|&f| {
                    !task.fact_names[f as usize]
                        .to_ascii_uppercase()
                        .starts_with("(P3")
                })
                .collect();
            let num = task.pre_num.slice(oi).to_vec();
            let true_now = pos
                .iter()
                .all(|&f| crate::bitset::test(&init.bits, f as usize))
                && num
                    .iter()
                    .all(|np| eval_numpre(np, &init.fv, &init.fdef) == Some(true));
            if true_now {
                already = true; // phi holds at init: nothing to chase
                break;
            }
            let est = crate::heuristic::relaxed_plan_cost(
                task,
                &mut sc,
                &init.bits,
                &init.fv,
                &init.fdef,
                &pos,
                &num,
                cost_fluent,
            )
            .unwrap_or(f64::INFINITY);
            if cheapest.as_ref().map_or(true, |(_, _, c)| est < *c) {
                cheapest = Some((pos, num, est));
            }
        }
        if already {
            continue;
        }
        if let Some((pos, num, est)) = cheapest {
            if est <= weight && !pos.is_empty() {
                cands.push(Cand {
                    pos,
                    num,
                    est,
                    value: weight - est,
                    mandatory: false,
                });
            }
        }
    }
    let dbg = std::env::var("FF_RES_DEBUG").is_ok();
    if cands.len() < 2 {
        if dbg {
            eprintln!("[seed3] skip: {} candidate(s)", cands.len());
        }
        return None;
    }

    let mut var_of: crate::hash::FxHashMap<u32, usize> = crate::hash::FxHashMap::default();
    for (gi, g) in groups.iter().enumerate() {
        for &f in g {
            var_of.insert(f, gi);
        }
    }

    // 1b. Mutex-conflict pruning: two OPTIONAL candidates claiming DIFFERENT
    // facts of the same mutex group are jointly unsatisfiable at the end
    // (tpp's per-goods `stored g level1..4` ladder — at most one level holds),
    // so a stage goal containing both is infeasible by construction and only
    // burns budget. Keep the best-value claimant per (group, distinct-fact)
    // conflict; the tail forgoes the dropped ones. Deterministic (groups in
    // index order, min-index tiebreak). Mandatory candidates always win.
    {
        let mut claimed: crate::hash::FxHashMap<usize, (u32, usize)> =
            crate::hash::FxHashMap::default();
        let mut drop = vec![false; cands.len()];
        for ci in 0..cands.len() {
            for k in 0..cands[ci].pos.len() {
                let f = cands[ci].pos[k];
                let Some(&gi) = var_of.get(&f) else { continue };
                match claimed.entry(gi) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert((f, ci));
                    }
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        let (held_f, held_ci) = *o.get();
                        if held_f == f || drop[held_ci] {
                            if drop[held_ci] {
                                o.insert((f, ci));
                            }
                            continue; // same fact: compatible
                        }
                        // Different facts of one group: keep the better value.
                        let (a, b) = (held_ci, ci);
                        let better_b = cands[b].value.partial_cmp(&cands[a].value)
                            == Some(std::cmp::Ordering::Greater);
                        if better_b {
                            drop[a] = true;
                            o.insert((f, b));
                        } else {
                            drop[b] = true;
                        }
                    }
                }
            }
        }
        let before = cands.len();
        let mut keep = drop.iter().map(|d| !d);
        cands.retain(|_| keep.next().unwrap());
        if dbg && cands.len() != before {
            eprintln!(
                "[seed3] mutex pruning: {before} -> {} candidate(s)",
                cands.len()
            );
        }
        if cands.len() < 2 {
            return None;
        }
    }

    // 2. Union-find through mutex variables.
    let var = |f: u32| var_of.get(&f).copied().unwrap_or(groups.len() + f as usize);
    let mut uf: Vec<usize> = (0..cands.len()).collect();
    fn find(uf: &mut [usize], mut x: usize) -> usize {
        while uf[x] != x {
            uf[x] = uf[uf[x]];
            x = uf[x];
        }
        x
    }
    let mut owner: crate::hash::FxHashMap<usize, usize> = crate::hash::FxHashMap::default();
    for (ci, cand) in cands.iter().enumerate() {
        for &f in &cand.pos {
            let v = var(f);
            match owner.entry(v) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    let a = find(&mut uf, ci);
                    let b = find(&mut uf, *o.get());
                    uf[a.max(b)] = a.min(b);
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(ci);
                }
            }
        }
    }
    let mut comp_of: crate::hash::FxHashMap<usize, Vec<usize>> = crate::hash::FxHashMap::default();
    for ci in 0..cands.len() {
        let r = find(&mut uf, ci);
        comp_of.entry(r).or_default().push(ci);
    }
    if comp_of.len() < 2 {
        if dbg {
            eprintln!(
                "[seed3] skip: {} candidate(s) collapse into {} component(s)",
                cands.len(),
                comp_of.len()
            );
        }
        return None;
    }
    let mut comps: Vec<Vec<usize>> = comp_of.into_values().collect();
    comps.sort_by_key(|members| {
        members
            .iter()
            .flat_map(|&ci| cands[ci].pos.iter().copied())
            .min()
            .unwrap_or(u32::MAX)
    });

    // 3. Compose: one protected, sat-guided stage per component. Stage
    // attempts run at a TENTH of the loop's per-probe cap — a stage that
    // needs more is not composing cheaply and gets its priciest preference
    // dropped instead — and the whole composition may spend at most a
    // QUARTER of the budget (an infeasible-joint-goal component would
    // otherwise burn the tightening loop's entire allowance on retries).
    let stage_cap = (cfg.max_eval / 10).max(1_000);
    let seed_budget = budget / 4;
    let mut state = init.clone();
    let mut plan: Vec<usize> = Vec::new();
    let mut protected: crate::hash::FxHashSet<u32> = crate::hash::FxHashSet::default();
    for members in &comps {
        let mut alive: Vec<usize> = members.clone();
        loop {
            // Facts still to achieve for the alive members in the CURRENT state.
            let satisfied = |ci: usize| {
                cands[ci]
                    .pos
                    .iter()
                    .all(|&f| crate::bitset::test(&state.bits, f as usize))
                    && cands[ci]
                        .num
                        .iter()
                        .all(|np| eval_numpre(np, &state.fv, &state.fdef) == Some(true))
            };
            alive.retain(|&ci| !satisfied(ci));
            if alive.is_empty() {
                break;
            }
            let mut goal: Vec<u32> = alive
                .iter()
                .flat_map(|&ci| cands[ci].pos.iter().copied())
                .collect();
            goal.sort_unstable();
            goal.dedup();
            let nums: Vec<crate::types::NumPre> = alive
                .iter()
                .flat_map(|&ci| cands[ci].num.iter().cloned())
                .collect();
            let forbidden: Vec<bool> = (0..task.n_ops)
                .map(|oi| {
                    p3_mask.get(oi).copied().unwrap_or(false)
                        || task.del.slice(oi).iter().any(|f| protected.contains(f))
                })
                .collect();
            if spent >= seed_budget {
                if dbg {
                    eprintln!("[seed3] abort: seed budget exhausted mid-composition");
                }
                return None;
            }
            let stage_cfg = SearchCfg {
                max_eval: stage_cap.min(seed_budget - spent),
                ..cfg
            };
            let (ops, evaluated) = crate::search::solve_subgoal_guided(
                task,
                &state,
                &goal,
                &nums,
                &forbidden,
                threads,
                stage_cfg,
                Some(sat),
            );
            spent += evaluated;
            match ops {
                Some(ops) => {
                    for &oi in &ops {
                        state = task.apply(oi, &state);
                    }
                    plan.extend(ops);
                    break;
                }
                None => {
                    // Drop the priciest optional member and retry; a stage that
                    // cannot meet its mandatory facts sinks the whole seed.
                    let worst = alive
                        .iter()
                        .copied()
                        .filter(|&ci| !cands[ci].mandatory)
                        .max_by(|&a, &b| {
                            cands[a]
                                .est
                                .partial_cmp(&cands[b].est)
                                .unwrap_or(std::cmp::Ordering::Equal)
                                .then(a.cmp(&b))
                        });
                    match worst {
                        Some(w) => alive.retain(|&ci| ci != w),
                        None => {
                            if dbg {
                                eprintln!("[seed3] abort: mandatory facts infeasible in a stage");
                            }
                            return None;
                        }
                    }
                }
            }
        }
        for &ci in members {
            if cands[ci].mandatory {
                protected.extend(cands[ci].pos.iter().copied());
            }
        }
    }

    // 4. Close the bookkeeping and validate the composed plan.
    let Some(tail_ops) = apply_tail(task, &mut state, tail) else {
        if dbg {
            eprintln!("[seed3] abort: phase tail failed on the composed state");
        }
        return None;
    };
    plan.extend(tail_ops);
    if !task.goal_met_with(&state, &task.goal_pos, &task.goal_num) {
        if dbg {
            eprintln!("[seed3] abort: composed state fails the full goal");
        }
        return None; // never expected; an invalid seed must not become the incumbent
    }
    let cost = plan_cost(task, &plan, cost_fluent);
    Some((plan, cost, spent))
}

/// The exact-closure metric optimizer (see `metric_optimize` step 2):
/// anytime branch-and-bound, each iteration walking real states under a
/// metric-bounded gate — `cost-so-far + closure(state) < bound`, closure
/// being the exact weight the phase tail is about to forgo — then bolting
/// the tail on at the end. Every valid compiled plan factors as a real
/// prefix, a `P3END`, and a collect/forgo permutation whose optimal
/// closure IS the tail — so a search that runs the space dry, no cap,
/// proves optimality outright.
///
/// The opening incumbent is the tail run straight off the initial state —
/// whenever the real hard goal already holds there, which is always true
/// on the pure-preference IPC-5 tracks — so even the biggest instances
/// report a metric instantly, no wait. The tightening budget is a
/// deterministic count of evaluated states (`FF_PREF_EVAL_BUDGET`,
/// default 2M) — never the clock — so results hold steady no matter how
/// many threads are running. Returns `None`, routing to the legacy
/// fallback, only when no incumbent could be built at all.
#[allow(clippy::too_many_arguments)]
fn metric_optimize_closure(
    task: &PackedTask,
    cost_fluent: usize,
    forgos: &[(usize, f64)],
    tail: &PhaseTail,
    sat: &SatGuidance,
    groups: &[Vec<u32>],
    threads: usize,
    cfg: SearchCfg,
) -> Option<MetricResult> {
    const MAX_ITERS: usize = 10_000;
    let real_pos: Vec<u32> = task
        .goal_pos
        .iter()
        .copied()
        .filter(|&f| {
            !task.fact_names[f as usize]
                .to_ascii_uppercase()
                .starts_with("(P3")
        })
        .collect();
    let closure = build_closure_cost(task, forgos);
    // The search explores real states only: every synthetic op is masked.
    let forbidden: Vec<bool> = (0..task.n_ops)
        .map(|oi| {
            let n = task.op_display[oi].to_ascii_uppercase();
            n == "P3END" || n.starts_with("P3COLLECT-") || n.starts_with("P3FORGO-")
        })
        .collect();
    let init = task.initial();

    // Trivial incumbent: close the initial state directly. Instant coverage —
    // this is what puts storage p05-p08 on the board at all.
    let mut best: Option<(Vec<usize>, f64)> = None;
    if task.goal_met_with(&init, &real_pos, &task.goal_num) {
        let mut s = init.clone();
        if let Some(tail_ops) = apply_tail(task, &mut s, tail) {
            if task.goal_met_with(&s, &task.goal_pos, &task.goal_num) {
                let cost = plan_cost(task, &tail_ops, cost_fluent);
                if cost <= 0.0 {
                    return Some(MetricResult {
                        ops: tail_ops,
                        cost,
                        iterations: 0,
                        proven: true,
                    });
                }
                best = Some((tail_ops, cost));
            }
        }
    }

    let budget = pref_eval_budget();
    let mut spent = 0usize;
    let mut iterations = 0usize;
    let mut proven = false;
    let mut sel_bound: Option<f64> = None;

    // SELECTION SEED (the 0.6 headline, default ON; `FF_PREF_NO_SELECT=1`
    // restores 0.5.1): exact preference-subset selection planned as a
    // hard-goal target. See `selection_seed` and docs/forensics-tpp.md.
    // Like the legacy path's EHC seed, its (bounded, deterministic) evals
    // stay OUTSIDE the tightening budget — charging them starved the loop
    // exactly on the instances where the joint target is infeasible
    // (measured: storage p08, 83 → 104). Measured wins: tpp p05 89 → 80
    // (bound 79 = the forensics optimum, reproduced by the solver), p06
    // 104 → 101 (exact tie with SGPlan5), p07 110 → 103.
    if std::env::var("FF_PREF_NO_SELECT").is_err() && task.goal_num.is_empty() {
        let (seeded, bound, _evals) = selection_seed(
            task,
            cost_fluent,
            groups,
            forgos,
            tail,
            &forbidden,
            threads,
            cfg,
            budget,
        );
        sel_bound = bound;
        if let Some((ops, cost)) = seeded {
            if best.as_ref().map_or(true, |(_, c)| cost < *c) {
                if cost <= 0.0 {
                    return Some(MetricResult {
                        ops,
                        cost,
                        iterations: 0,
                        proven: true,
                    });
                }
                best = Some((ops, cost));
            }
        }
    }

    // PARTITIONED CLOSURE SEED (increment 3) — experimental, opt-in via
    // `FF_PREF_SEED3=1`, default OFF after measuring NEUTRAL (2026-07): with
    // mutex-conflict pruning the composition genuinely works (tpp p05 composes
    // a 99 incumbent vs the init-tail 105; p07 120 vs 135; pathways p05 9 vs
    // 10.2) — but the anytime+ladder tightening loop reaches the same final
    // metric from either starting bound on every instance measured, and an
    // aborted composition wastes up to a quarter of the budget. The mechanism
    // is kept as the substrate for real per-stage λ pricing (the un-built rest
    // of increment 3); composition-as-seeding alone does not move finals.
    if std::env::var("FF_PREF_SEED3").is_ok() && task.goal_num.is_empty() {
        let dbg = std::env::var("FF_RES_DEBUG").is_ok();
        match compose_pref_seed(
            task,
            cost_fluent,
            groups,
            forgos,
            tail,
            sat,
            &forbidden,
            threads,
            cfg,
            budget,
        ) {
            Some((ops, cost, evals)) => {
                spent += evals;
                if best.as_ref().map_or(true, |(_, c)| cost < *c) {
                    if dbg {
                        eprintln!(
                            "[seed3] composed incumbent {cost} (was {:?}), {evals} evals",
                            best.as_ref().map(|(_, c)| *c)
                        );
                    }
                    if cost <= 0.0 {
                        return Some(MetricResult {
                            ops,
                            cost,
                            iterations: 0,
                            proven: true,
                        });
                    }
                    best = Some((ops, cost));
                } else if dbg {
                    eprintln!(
                        "[seed3] composed {cost} LOST to incumbent {:?} ({evals} evals)",
                        best.as_ref().map(|(_, c)| *c)
                    );
                }
            }
            None => {
                if dbg {
                    eprintln!("[seed3] no composition");
                }
            }
        }
    }
    let mut bound = best.as_ref().map_or(f64::INFINITY, |(_, c)| *c);

    // ESCALATION: a tightening probe that hits its per-iteration eval cap
    // without finding a cheaper plan is INCONCLUSIVE, not a reason to abandon
    // the optimization — with budget remaining, retry the SAME bound with ALL
    // of it (not a doubling ladder: a retry at the same bound+cfg re-treads a
    // deterministic prefix, so intermediate rungs only re-pay it; `evaluated`
    // is actual usage, so a large cap that succeeds early costs only what it
    // used). This is what makes FF_PREF_EVAL_BUDGET the real contract — before
    // it, one failed 300k sweep ended the loop with millions unspent. All
    // quantities are deterministic eval counts, so t1≡t8 is preserved.
    // `FF_PREF_NO_ESCALATE=1` restores break-on-first-capped-failure.
    //
    // ANYTIME TIGHTENING (see `SearchCfg::anytime`): each sweep tightens its
    // bound in place on every acceptance and keeps draining, so a restart (and
    // its deterministic prefix re-tread) happens once per CAP instead of once
    // per improvement. `FF_PREF_GREEDY=1` restores first-improvement sweeps.
    //
    // DIVERSIFIED RESTART LADDER: a capped no-improvement sweep is not just
    // inconclusive — it says the current h-ordering cannot reach a better
    // plan in this region (measured: pouring the whole budget into the same
    // direction re-treads the same prefix and changes nothing). Before the
    // final all-remaining escalation, rotate the open-list weights through a
    // fixed profile ladder — each rung orders the frontier differently
    // (h-greedier / g-heavier / pure-h), exploring a genuinely different
    // region under the SAME bound. Fully deterministic (fixed profiles, eval-
    // count budgets); an improvement resets the ladder to the default rung.
    // `FF_PREF_NO_RESTARTS=1` disables the ladder alone.
    const PROFILES: &[(f64, f64)] = &[(1.0, 2.0), (1.0, 8.0), (2.0, 3.0), (0.0, 1.0)];
    let no_escalate = std::env::var("FF_PREF_NO_ESCALATE").is_ok();
    let anytime = std::env::var("FF_PREF_GREEDY").is_err();
    let restarts = anytime && std::env::var("FF_PREF_NO_RESTARTS").is_err();
    let mut escalated = false;
    let mut rung = 0usize; // 0 = default profile; 1..=len = PROFILES
    while iterations < MAX_ITERS && spent < budget {
        iterations += 1;
        let cap = if escalated {
            budget - spent
        } else if rung > 0 {
            // Diversification rungs run at HALF the probe cap: they exist to
            // find a different region fast, and the budget they don't spend
            // is what keeps the final full-budget escalation strong (measured:
            // full-size rungs starved it and gave back tpp p04 / trucks p07).
            (cfg.max_eval / 2).min(budget - spent)
        } else {
            cfg.max_eval.min(budget - spent)
        };
        let iter_cfg = if rung == 0 {
            SearchCfg {
                max_eval: cap,
                anytime,
                ..cfg
            }
        } else {
            let (wg, wh) = PROFILES[rung - 1];
            SearchCfg {
                max_eval: cap,
                anytime,
                w_c: cfg.w_c,
                ..SearchCfg::from_weights(wg, wh, Some(cap))
            }
        };
        let (opt, evaluated, capped) = crate::search::solve_closure_bounded(
            task,
            &real_pos,
            &task.goal_num,
            cost_fluent,
            bound,
            &closure,
            &forbidden,
            threads,
            iter_cfg,
            Some(sat),
        );
        spent += evaluated;
        match opt {
            Some(mut ops) => {
                escalated = false;
                rung = 0; // an improvement re-arms the whole ladder
                let mut s = init.clone();
                for &oi in &ops {
                    s = task.apply(oi, &s);
                }
                let Some(tail_ops) = apply_tail(task, &mut s, tail) else {
                    break; // never expected (P3 ops are masked); keep the incumbent
                };
                if !task.goal_met_with(&s, &task.goal_pos, &task.goal_num) {
                    break; // safety: an invalid composition must not become the incumbent
                }
                ops.extend(tail_ops);
                let cost = plan_cost(task, &ops, cost_fluent);
                best = Some((ops, cost));
                if cost <= 0.0 {
                    proven = true;
                    break;
                }
                bound = cost;
            }
            None => {
                if capped && !no_escalate {
                    // Diversify first: same bound, different open-list order.
                    if restarts && rung < PROFILES.len() && budget > spent {
                        rung += 1;
                        continue;
                    }
                    if budget.saturating_sub(spent) > cap {
                        escalated = true; // ladder spent: all remaining budget, default order
                        rung = 0;
                        continue;
                    }
                }
                proven = !capped;
                break;
            }
        }
    }

    best.map(|(ops, cost)| MetricResult {
        ops,
        cost,
        // The selection bound is ADMISSIBLE (per-fact reachability is
        // optimistic), so matching it proves optimality even when the
        // eval budget was exhausted.
        proven: proven || sel_bound.is_some_and(|b| (cost - b).abs() < 1e-6),
        iterations,
    })
}

/// `P3COLLECT-i` op ids, one bucket per preference index, ascending — the
/// shared scan behind the satisfaction guidance, the deadline guidance,
/// the phase tail, and the closure cost, all reading off the same table.
/// A phi whose DNF splits into several disjuncts grounds to several ops,
/// all sharing the name `P3COLLECT-i` — one per disjunct — which is why
/// the value is a Vec: phi holds the instant any one of them is
/// applicable. A single-op map here once silently kept just one arbitrary
/// disjunct — a quiet bug that made the tail forgo preferences that were
/// actually satisfied, on `imply`/`exists` phis.
fn collect_ops(task: &PackedTask) -> std::collections::HashMap<usize, Vec<usize>> {
    let mut collect_op: std::collections::HashMap<usize, Vec<usize>> =
        std::collections::HashMap::new();
    for oi in 0..task.n_ops {
        if let Some(rest) = task.op_display[oi]
            .to_ascii_uppercase()
            .strip_prefix("P3COLLECT-")
        {
            if let Ok(i) = rest.trim().parse::<usize>() {
                collect_op.entry(i).or_default().push(oi);
            }
        }
    }
    collect_op
}

/// Op ids for the deterministic post-search phase tail — the closing
/// ceremony: `P3END` freezes the state, then each preference gets settled
/// in fixed order, its first applicable `P3COLLECT-i` disjunct if phi
/// holds (paid for free), `P3FORGO-i` if it doesn't (pays the weight).
/// Exact, not a guess — once `P3END` fires, the state is locked and each
/// preference's collected fact stands alone, so collect-iff-applicable is
/// the provably optimal closure of whatever state the search actually
/// landed in. Used by the default closure-metric optimizer and the
/// partitioned-ESPC composition. `None` only when the compile carries no
/// `P3END` at all — meaning this was never a preference task.
pub struct PhaseTail {
    pub end_op: usize,
    /// `(collect_ops [one per phi disjunct, empty means always-forgo],
    /// forgo_op)` — one line per preference, kept in preference order.
    pub prefs: Vec<(Vec<usize>, usize)>,
}

pub(crate) fn build_phase_tail(task: &PackedTask, forgos: &[(usize, f64)]) -> Option<PhaseTail> {
    let end_op = (0..task.n_ops).find(|&oi| task.op_display[oi].eq_ignore_ascii_case("P3END"))?;
    let mut collect = collect_ops(task);
    let mut prefs = Vec::with_capacity(forgos.len());
    for (i, &(forgo_op, _)) in forgos.iter().enumerate() {
        prefs.push((collect.remove(&i).unwrap_or_default(), forgo_op));
    }
    Some(PhaseTail { end_op, prefs })
}

/// Build the exact closure-cost table ([`ClosureCost`]) off the compiled
/// `P3COLLECT-i` ops — one DNF disjunct per collect op, its positive
/// precondition stripped of the `P3*` control facts, its numeric
/// precondition left in, weighted by the preference's forgo cost.
/// Zero-weight preferences never make the table — forgoing them costs
/// nothing, so they can't move the metric either way.
pub(crate) fn build_closure_cost(task: &PackedTask, forgos: &[(usize, f64)]) -> ClosureCost {
    let mut collect = collect_ops(task);
    let mut prefs = Vec::new();
    for (i, &(_, weight)) in forgos.iter().enumerate() {
        if weight <= 0.0 {
            continue;
        }
        let disjuncts: Vec<(Vec<u32>, Vec<crate::types::NumPre>)> = collect
            .remove(&i)
            .unwrap_or_default()
            .into_iter()
            .map(|oi| {
                let pos: Vec<u32> = task
                    .pre_pos
                    .slice(oi)
                    .iter()
                    .copied()
                    .filter(|&f| {
                        !task.fact_names[f as usize]
                            .to_ascii_uppercase()
                            .starts_with("(P3")
                    })
                    .collect();
                (pos, task.pre_num.slice(oi).to_vec())
            })
            .collect();
        prefs.push((weight, PrefPhi { disjuncts }));
    }
    ClosureCost { prefs }
}

/// Close the books on `state` — run the exact phase tail: `P3END` freezes
/// the planning phase, then each preference in fixed order takes its
/// first applicable `P3COLLECT-i` disjunct if there is one, free, else
/// `P3FORGO-i`, which pays the weight. Returns the tail ops and walks
/// `state` through every one of them. `None` the moment an op turns out
/// inapplicable — a searched plan that already fired `P3END`, say —
/// and callers treat that as an invalid composition and fall back rather
/// than risk corrupting the plan.
pub(crate) fn apply_tail(
    task: &PackedTask,
    state: &mut crate::packed::State,
    tail: &PhaseTail,
) -> Option<Vec<usize>> {
    let mut ops = Vec::with_capacity(1 + tail.prefs.len());
    if !task.op_applicable(tail.end_op, state) {
        return None;
    }
    *state = task.apply(tail.end_op, state);
    ops.push(tail.end_op);
    for (collects, forgo) in &tail.prefs {
        let oi = collects
            .iter()
            .copied()
            .find(|&c| task.op_applicable(c, state))
            .unwrap_or(*forgo);
        if !task.op_applicable(oi, state) {
            return None;
        }
        *state = task.apply(oi, state);
        ops.push(oi);
    }
    Some(ops)
}

/// Build the partitioned-ESPC subproblems — increment 2, see
/// `crate::espc`: interaction components drawn over the real goal, since
/// the compiled `P3*` bookkeeping goals get closed by the phase tail
/// instead. The detected renewable resource variables — openstacks'
/// `stacks-avail` chain — stay off the wiring entirely; that shared
/// coupling gets priced by the λ schedule as a global constraint, never
/// solved inside any single subproblem. `None`, falling back to the
/// monolithic loop, when the compile shape doesn't fit: no phase tail,
/// numeric goals in the mix, no real positive goals, or fewer than 2
/// components to work with.
fn build_espc_partition(
    task: &PackedTask,
    forgos: &[(usize, f64)],
    groups: &[Vec<u32>],
    sat: &SatGuidance,
) -> Option<crate::espc::EspcPartition> {
    if !task.goal_num.is_empty() {
        return None; // components carry positive facts only; don't drop a numeric goal
    }
    let tail = build_phase_tail(task, forgos)?;
    let real_goals: Vec<u32> = task
        .goal_pos
        .iter()
        .copied()
        .filter(|&f| {
            !task.fact_names[f as usize]
                .to_ascii_uppercase()
                .starts_with("(P3")
        })
        .collect();
    if real_goals.is_empty() {
        return None;
    }
    // Global-constraint variables: any mutex group carrying a detected renewable
    // resource member (detect_resources accepts whole groups, so member-overlap
    // identifies exactly the accepted group indices).
    let res_member: crate::hash::FxHashSet<u32> = sat
        .res
        .iter()
        .flat_map(|r| r.members.iter().map(|&(f, _)| f))
        .collect();
    let excluded: crate::hash::FxHashSet<usize> = groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.iter().any(|f| res_member.contains(f)))
        .map(|(gi, _)| gi)
        .collect();
    let mut comps =
        crate::partition::interaction_partition_of(task, groups, &real_goals, &excluded);
    // Deterministic composition order regardless of hash-map component order.
    comps.sort_by_key(|c| c.pos.iter().min().copied().unwrap_or(u32::MAX));
    if comps.len() < 2 {
        return None;
    }
    // Stage-goal enrichment map: deliverable D → the real goals structurally
    // tied to it. D's conditional-achievement CONDITION facts name the party the
    // delivery is FOR (openstacks: `delivered(o,p)` fires on `started(o)`), and
    // a goal fact claims D when one of the goal's ACHIEVER ops requires such a
    // condition fact (`ship-order(o)` adds `shipped(o)` and requires
    // `started(o)`), so the stage solving that goal also tries to earn its own
    // preferences (see `EspcPartition::assoc`).
    let deliverables: crate::hash::FxHashSet<u32> =
        sat.deadline.iter().map(|&(_, d, _)| d).collect();
    let mut by_cond: crate::hash::FxHashMap<u32, Vec<u32>> = crate::hash::FxHashMap::default();
    for oi in 0..task.n_ops {
        for ce in task.cond_effs(oi) {
            for &d in &ce.add {
                if !deliverables.contains(&d) {
                    continue;
                }
                for &c in &ce.cond_pos {
                    by_cond.entry(c).or_default().push(d);
                }
            }
        }
    }
    let mut assoc: crate::hash::FxHashMap<u32, Vec<u32>> = crate::hash::FxHashMap::default();
    for &g in &real_goals {
        let mut ds: Vec<u32> = task
            .add_by_fact
            .slice(g as usize)
            .iter()
            .flat_map(|&oi| task.pre_pos.slice(oi as usize))
            .filter_map(|p| by_cond.get(p))
            .flatten()
            .copied()
            .collect();
        if !ds.is_empty() {
            ds.sort_unstable();
            ds.dedup();
            assoc.insert(g, ds);
        }
    }
    if std::env::var("FF_RES_DEBUG").is_ok() {
        eprintln!(
            "[ESPC] partition: {} component(s) over {} real goal(s), {} excluded var(s), {} enriched goal(s)",
            comps.len(),
            real_goals.len(),
            excluded.len(),
            assoc.len()
        );
    }
    Some(crate::espc::EspcPartition { comps, tail, assoc })
}

/// Build the metric satisfaction guidance — for each preference, its full
/// phi in DNF ([`PrefPhi`], one disjunct per `P3COLLECT-i` op, so
/// `imply`/`exists` preferences steer correctly) and a heap penalty scaled
/// off its forgo weight. Two exclusions keep the gradient honest, not
/// self-deceiving:
/// - phi that's unachievable — no collect ops at all — or trivially true;
///   a constant penalty can't order anything, so don't pretend it does;
/// - phi already satisfied at the initial state (unless
///   `FF_PREF_BARRIER=1`) — penalizing its transient dips throws up a
///   wall in front of every trajectory that would otherwise improve
///   things (tpp's weight-16 `p4A` has to dip during any real delivery
///   run). The real protection lives downstream, in the exact closure
///   acceptance on the final state. Guidance pulls toward what hasn't
///   been earned yet — it doesn't punish passing through.
fn build_sat_guidance(task: &PackedTask, forgos: &[(usize, f64)]) -> SatGuidance {
    let mut collect_op = collect_ops(task);
    let init = task.initial();
    // Init-satisfied preferences are KEPT in the guidance since 0.5.1 — the
    // forensics on tpp p05 (docs/forensics-tpp.md) showed excluding them
    // makes the search blind to high-weight TRAP preferences (`not (stored
    // goods1 level3)` is satisfied at init; the guidance then rewards
    // trampling it for a cheaper positive pref, and every restart-ladder
    // profile inherits the blindness). Re-measured on the 0.5 engine the
    // exclusion's original justification no longer holds: keeping them wins
    // storage p05–p08 (31/121/124/148 → 25/43/60/83 — an 8/8 domain sweep vs
    // SGPlan5), tpp p05/p07/p08 (−4/−7/−18), pathways p06 (−1.9), at the
    // cost of pathways p05 alone (6 → 6.5, a win becoming an exact tie).
    // `FF_PREF_NO_BARRIER=1` restores the 0.4–0.5.0 exclusion;
    // `FF_PREF_BARRIER` is accepted (now redundant).
    let keep_barrier = std::env::var("FF_PREF_NO_BARRIER").is_err();
    let mut prefs = Vec::new();
    for (i, (_, weight)) in forgos.iter().enumerate() {
        let disjuncts: Vec<(Vec<u32>, Vec<crate::types::NumPre>)> = collect_op
            .remove(&i)
            .unwrap_or_default()
            .into_iter()
            .map(|oi| {
                let pos: Vec<u32> = task
                    .pre_pos
                    .slice(oi)
                    .iter()
                    .copied()
                    .filter(|&f| {
                        !task.fact_names[f as usize]
                            .to_ascii_uppercase()
                            .starts_with("(P3")
                    })
                    .collect();
                (pos, task.pre_num.slice(oi).to_vec())
            })
            .collect();
        if disjuncts.is_empty() || disjuncts.iter().any(|(p, n)| p.is_empty() && n.is_empty()) {
            continue; // unachievable or trivially-true phi: constant, can't guide
        }
        let phi = PrefPhi { disjuncts };
        if !keep_barrier && phi.holds(&init) {
            continue; // init-satisfied: no barrier on its transient dips
        }
        // Init-satisfied preferences guide at FULL weight. A half-weight
        // variant was measured (2026-07, the pathways-p05 recovery attempt)
        // and REJECTED: it left pathways p05 at 6.5 and gave back large
        // storage wins (p05 25 → 46, p07 60 → 75). The p05 win→tie trade is
        // the recorded cost of the barrier, not a tunable.
        prefs.push((phi, (weight * 100.0).round().max(1.0) as i64));
    }
    SatGuidance {
        prefs,
        res: Vec::new(),
        res_weight: 0,
        res_thresh: 0,
        deadline: Vec::new(),
        deadline_weight: 0,
    }
}

/// Build ESPC make-deadline guidance (see [`SatGuidance::deadline`]). For
/// each preference deliverable fact `D` — pulled from every `P3COLLECT-i`
/// `phi`, same extraction [`build_sat_guidance`] runs — find the op whose
/// conditional effect adds `D`, and that op's one unconditional add `M`:
/// the once-only trigger, `(made p)` for instance, that can fire at most
/// once because the op demands its own trigger be absent first. Emits
/// `(M, D, value)`, `value` being the summed weight of every preference
/// that needs `D` — so a deliverable shared across the weight-1/2/4 chain
/// gets valued highest of all. Comes back empty on domains that lack this
/// conditional-achievement shape entirely (inert, not broken), and always
/// in a deterministic order — never at the mercy of hashmap iteration.
fn build_deadline_guidance(task: &PackedTask, forgos: &[(usize, f64)]) -> Vec<(u32, u32, i64)> {
    use std::collections::{HashMap, HashSet};
    // P3COLLECT-i op per preference index (mirrors build_sat_guidance).
    let collect_op = collect_ops(task);
    // value[D] = Σ weight over preferences whose phi (P3COLLECT precondition,
    // minus synthetic P3* control facts) contains the deliverable fact D.
    let mut value: HashMap<u32, i64> = HashMap::new();
    for (i, (_, weight)) in forgos.iter().enumerate() {
        let Some(ops) = collect_op.get(&i) else {
            continue;
        };
        let w = (*weight).round().max(1.0) as i64;
        // union of the pref's disjunct facts, deduped so a fact shared by
        // several disjuncts of the SAME pref is valued once (single-disjunct
        // phi — openstacks — is unchanged).
        let mut facts: Vec<u32> = ops
            .iter()
            .flat_map(|&oi| task.pre_pos.slice(oi))
            .copied()
            .filter(|&f| {
                !task.fact_names[f as usize]
                    .to_ascii_uppercase()
                    .starts_with("(P3")
            })
            .collect();
        facts.sort_unstable();
        facts.dedup();
        for f in facts {
            *value.entry(f).or_insert(0) += w;
        }
    }
    if value.is_empty() {
        return Vec::new();
    }
    // For each deliverable D, find its conditional achiever op and that op's
    // UNIQUE unconditional trigger M (skip ops with non-unique unconditional adds —
    // they aren't the clean once-only achiever this models).
    //
    // 0.8 Phase 3 (docs/roadmap-0.8.md): the SHARED monitor block is
    // deliberately NOT scanned for deliverables. Its conditional adds are
    // trajectory-monitor bits (TRAJ*, riding every monitored op), and pairing
    // them made ESPC engage on monitor-compiled tasks through ARTIFACTS
    // rather than real once-only achievement structure — then OOM its
    // monolithic tightening pass on the monitor-widened states (storage
    // qualpref p05–p08; dmesg-confirmed ~16 GB inside one pass, below every
    // eval budget). Without monitor pairs those tasks fall through to the
    // closure optimizer — exactly the known-good behavior the scoreboard
    // documented as `FF_NO_ESPC=1` — while real deliverables (openstacks'
    // per-op `When` adds live in the op's OWN cond row) keep their pairs.
    // `FF_ESPC_TRAJ_PAIRS=1` restores the 0.7 monitor-artifact pairing.
    let traj_pairs = std::env::var("FF_ESPC_TRAJ_PAIRS").is_ok();
    let mut out: Vec<(u32, u32, i64)> = Vec::new();
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    for oi in 0..task.n_ops {
        let uncond = task.add.slice(oi);
        if uncond.len() != 1 {
            continue;
        }
        let trigger = uncond[0];
        let include_shared = traj_pairs && task.monitored[oi];
        let shared_iter = task.shared_cond.iter().filter(|_| include_shared);
        for ce in task.cond.slice(oi).iter().chain(shared_iter) {
            for &d in &ce.add {
                if let Some(&val) = value.get(&d) {
                    if seen.insert((trigger, d)) {
                        out.push((trigger, d, val));
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out
}

#[cfg(test)]
mod monitor_pairs {
    //! 0.8 Phase 3 (docs/roadmap-0.8.md): the shared monitor block must
    //! never feed ESPC's deadline-pair detection. Monitor bits are
    //! artifacts, not once-only achievement structure — pairing them let
    //! ESPC engage on monitor-widened tasks, and it OOM'd there (storage
    //! qualpref p05–p08).

    #[test]
    fn shared_monitor_adds_emit_no_deadline_pairs() {
        // A soft (sometime (on)): the collect precondition values both
        // TRAJ0-SEEN and ON, and FLIP-ON is a clean unique-unconditional-add
        // trigger — under the 0.7 per-op scan this task emitted the
        // (ON -> TRAJ0-SEEN) monitor-artifact pair; the shared-block scan
        // must emit nothing (assumes FF_ESPC_TRAJ_PAIRS unset, like every
        // env-sensitive default in this suite).
        let dom = "(define (domain sw)
          (:requirements :strips :constraints)
          (:predicates (on) (off) (lamp))
          (:action flip-on :precondition (off) :effect (and (not (off)) (on)))
          (:action flip-off :precondition (on) :effect (and (not (on)) (off)))
          (:action light :precondition (on) :effect (lamp)))";
        let prob = "(define (problem sw-1) (:domain sw) (:init (off)) (:goal (off))
             (:constraints (preference pv (sometime (on)))))";
        let d = crate::parser::parse_domain(dom).unwrap();
        let p = crate::parser::parse_problem(prob).unwrap();
        let (d, p) = crate::derived::compile(&d, &p).unwrap();
        let (d, p) = crate::constraints::gate(&d, &p).unwrap().unwrap();
        let c = super::compile(&d, &p);
        let task = crate::ground::ground_task(&c.domain, &c.problem, 1).unwrap();
        assert!(
            !task.shared_cond.is_empty(),
            "the monitor block must exist for this test to mean anything"
        );
        let forgos: Vec<(usize, f64)> = c
            .forgos
            .iter()
            .filter_map(|(name, w)| {
                task.op_display
                    .iter()
                    .position(|disp| disp == name)
                    .map(|oi| (oi, *w))
            })
            .collect();
        let pairs = super::build_deadline_guidance(&task, &forgos);
        assert!(
            pairs.is_empty(),
            "monitor-artifact deadline pairs must not be emitted: {pairs:?}"
        );
    }
}
