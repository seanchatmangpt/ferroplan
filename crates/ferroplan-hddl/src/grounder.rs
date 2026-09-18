//! Grounding: type closure, object indexing, term/atom/goal/effect
//! substitution (including `oneof`/`when` outcome expansion), and full
//! enumeration of ground actions and ground methods.
//!
//! Scope note: by default (`GroundingLimits::prune_unreachable == false`,
//! which is `GroundingLimits::default()`), this pass enumerates the *full*
//! combinatorial grounding (every typed object substitution), bounded only by
//! `GroundingLimits`'s counters. That is unchanged from this crate's original
//! behavior specifically so every pre-existing caller and test — which was
//! written against, and asserts, exact combinatorial ground-instance counts —
//! keeps its current output byte-for-byte.
//!
//! When `prune_unreachable` is set, `ground` instead runs a real
//! delete-relaxation reachability pre-pass (`compute_reachability`) before
//! instantiating ground actions/methods: a monotone, positive-effects-only
//! fixpoint over reachable ground facts, seeded from the initial state, that
//! soundly *over-approximates* real reachability (ignoring delete effects and
//! negative preconditions can only make more things look reachable, never
//! fewer — see `relaxed_satisfiable`). `ground_actions_reachable`/
//! `ground_methods_reachable` then skip instantiating any ground
//! action/method whose ground name that fixpoint never marked reachable,
//! before doing the (more expensive) precondition/effect substitution work.
//! This is the "join-based"/delete-relaxation grounder class FastDownward and
//! PANDA use, minus their incremental-join binding construction: bindings are
//! still built via one full `enumerate_bindings` Cartesian product per action
//! schema (same asymptotic enumeration cost as the unpruned path), and only
//! the *output* ground-instance count and the downstream substitution cost
//! shrink. `translate.rs` separately does its own real reachability analysis
//! (BFS from the initial state) after grounding, so unreachable ground
//! actions never produce transitions either way; this pre-pass additionally
//! avoids paying the combinatorial-enumeration and substitution cost for them
//! in the first place, when opted into.

use crate::ast::*;
use crate::validate;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{Duration, Instant};

/// An error produced by `ground`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroundError {
    /// The domain/problem failed `validate::validate_domain`/
    /// `validate::validate_problem` — those checks run first, before any
    /// grounding work.
    Validation(validate::ValidationError),
    /// The `:types` hierarchy contains a cycle involving the named type,
    /// detected while computing `build_type_closure`.
    TypeCycle(String),
    /// A variable was referenced (in a term, goal, or effect) that was never
    /// bound by the enclosing action/method parameter list or quantifier.
    UnboundVariable(String),
    /// A precondition/goal construct this grounder can't ground in its
    /// current position — e.g. `or`/`imply`/quantifiers nested inside a
    /// `when`-effect condition (see `ast`'s module docs for exact scope).
    UnsupportedPrecondition(String),
    /// `GroundingLimits::max_ground_actions`/`max_ground_methods` was
    /// exceeded — a loud refusal rather than a silently truncated result.
    LimitExceeded(String),
    /// A numeric fluent was declared (`(= (fluent ?params) value)` nested in
    /// `:predicates`) or used in an `increase`/`decrease` effect. Numeric
    /// planning is out of scope for this grounder — the fluent's name is
    /// carried so the caller gets a precise, actionable refusal rather than a
    /// generic failure. See `ast`'s module docs for the lex/parse-but-refuse
    /// rationale.
    UnsupportedNumericFluent(String),
    /// The domain or problem declared a `:constraints` block. Full PDDL 3.0
    /// constraint-GD semantics are out of scope for this grounder — the
    /// constraint's modal-operator keyword (e.g. "always", "sometime-after")
    /// is carried for a precise refusal. See `ast`'s module docs.
    UnsupportedConstraint(String),
    /// `GroundingLimits::max_wall` elapsed before grounding finished. A
    /// loud, precise wall-clock refusal — same "loud refusal, not silent
    /// truncation" discipline as `LimitExceeded` — for the case a
    /// pathological (but not obviously too-large-by-count) domain makes
    /// grounding itself run long: e.g. many cheap-looking action schemas
    /// each individually under `max_ground_actions`/`max_ground_methods`
    /// but expensive to substitute in aggregate, or (before
    /// `prune_unreachable` pruning helps) a slow `compute_reachability`
    /// fixpoint over a large fact universe. Each grounding phase
    /// (`ground_actions`/`ground_actions_reachable`/`ground_methods`/
    /// `ground_methods_reachable`/`compute_reachability`) checks this
    /// independently against its own start time, so `ground`'s total wall
    /// time in the worst case (the `prune_unreachable` path, which runs
    /// three of those phases in sequence) is bounded by roughly
    /// `3 * max_wall`, not by one shared budget across the whole call —
    /// still a hard, finite bound, just not an exact one.
    Timeout { elapsed_ms: u128, limit_ms: u128 },
}

impl fmt::Display for GroundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::TypeCycle(t) => write!(f, "type hierarchy cycle involving '{t}'"),
            Self::UnboundVariable(v) => write!(f, "unbound variable '?{v}'"),
            Self::UnsupportedPrecondition(msg) => write!(f, "unsupported construct: {msg}"),
            Self::LimitExceeded(msg) => write!(f, "grounding limit exceeded: {msg}"),
            Self::UnsupportedNumericFluent(name) => write!(
                f,
                "numeric fluent '{name}' is not supported by this grounder (numeric planning is out of scope)"
            ),
            Self::UnsupportedConstraint(kind) => write!(
                f,
                "':constraints' entry '{kind}' is not supported by this grounder (constraint-GD semantics are out of scope)"
            ),
            Self::Timeout {
                elapsed_ms,
                limit_ms,
            } => write!(
                f,
                "grounding wall-clock limit exceeded: {elapsed_ms}ms elapsed, limit {limit_ms}ms"
            ),
        }
    }
}
impl std::error::Error for GroundError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundingLimits {
    pub max_ground_actions: usize,
    pub max_ground_methods: usize,
    /// When `true`, `ground` runs the delete-relaxation reachability
    /// pre-pass (`compute_reachability`) and skips instantiating any ground
    /// action/method it proves can never fire — see the module docs.
    /// Defaults to `false`: every pre-existing caller/test that relies on
    /// the full combinatorial grounding keeps its exact current
    /// ground-instance counts unless it opts in explicitly.
    pub prune_unreachable: bool,
    /// Wall-clock budget for a single grounding phase
    /// (`ground_actions`/`ground_actions_reachable`/`ground_methods`/
    /// `ground_methods_reachable`/`compute_reachability`), each of which
    /// checks it independently against its own start time (see
    /// `GroundError::Timeout`'s doc comment for why the bound on `ground`'s
    /// *total* wall time is a multiple of this, not exactly this).
    /// `None` means unbounded, matching every pre-existing caller/test's
    /// current (unbounded) behavior; `default()` sets a real bound so a
    /// caller that just uses `GroundingLimits::default()` is protected
    /// without having to opt in.
    pub max_wall: Option<Duration>,
}

impl Default for GroundingLimits {
    fn default() -> Self {
        Self {
            max_ground_actions: 10_000,
            max_ground_methods: 10_000,
            prune_unreachable: false,
            max_wall: Some(Duration::from_secs(10)),
        }
    }
}

/// Checked by every grounding-phase loop below (see each function's own
/// `start` binding) once per outer-loop iteration. Cheap: `Instant::now()`
/// is a single syscall/vDSO read, negligible next to the substitution work
/// already happening per binding/round.
fn check_wall_deadline(start: Instant, limits: &GroundingLimits) -> Result<(), GroundError> {
    if let Some(max_wall) = limits.max_wall {
        let elapsed = start.elapsed();
        if elapsed > max_wall {
            return Err(GroundError::Timeout {
                elapsed_ms: elapsed.as_millis(),
                limit_ms: max_wall.as_millis(),
            });
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GroundConditional {
    pub pos_cond: BTreeSet<String>,
    pub neg_cond: BTreeSet<String>,
    pub add: BTreeSet<String>,
    pub del: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GroundEffectBranch {
    pub add: BTreeSet<String>,
    pub del: BTreeSet<String>,
    pub conditional: Vec<GroundConditional>,
    /// This outcome's declared weight from a `(:probabilistic ...)` effect
    /// block (see `probabilistic::preprocess` and
    /// `ast::ActionDef::probability_weights`), carried onto the branch during
    /// grounding by `ground_effect`. Kept as raw source text, parsed only
    /// where it's used (`translate::outcome_ppms`) -- the same discipline
    /// `ast::ActionDef::probability_weights` documents. `None` for a plain
    /// (weightless) `oneof` branch or a deterministic effect.
    pub probability_weight: Option<String>,
}

/// A fully-ground (all terms already substituted to object names) goal
/// formula, evaluable directly against a ground fact set via
/// `evaluate_ground_goal`. This is the ground-time counterpart of
/// `ast::GoalDesc` — see `ground_goal` for the substitution step that
/// produces one from an `ast::GoalDesc` plus a variable binding.
///
/// `GoalDesc::Imply` has no dedicated variant here: `ground_goal` lowers
/// `(imply a b)` to `Or(Not(a), b)` at grounding time (classical material
/// implication — vacuously true when `a` doesn't hold), so `evaluate_ground_goal`
/// only ever needs to know about `And`/`Or`/`Not`/`Atom`/`Empty`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroundGoal {
    Empty,
    Atom(String),
    Not(Box<GroundGoal>),
    And(Vec<GroundGoal>),
    Or(Vec<GroundGoal>),
}

/// Substitute `binding` into `goal`, producing a `GroundGoal` ready for
/// `evaluate_ground_goal`. Handles the full `GoalDesc` grammar this crate
/// supports in preconditions: `and`/`or`/`not`/`imply`/`forall`/`exists`/
/// atom/empty. `objects_by_type` is the typed object universe quantifiers
/// range over (see the `Forall`/`Exists` arms below).
fn ground_goal(
    goal: &GoalDesc,
    binding: &BTreeMap<String, String>,
    objects_by_type: &BTreeMap<String, Vec<String>>,
) -> Result<GroundGoal, GroundError> {
    Ok(match goal {
        GoalDesc::Empty => GroundGoal::Empty,
        GoalDesc::Atom(a) => GroundGoal::Atom(subst_atom(a, binding)?),
        GoalDesc::Not(inner) => {
            GroundGoal::Not(Box::new(ground_goal(inner, binding, objects_by_type)?))
        }
        GoalDesc::And(parts) => GroundGoal::And(
            parts
                .iter()
                .map(|p| ground_goal(p, binding, objects_by_type))
                .collect::<Result<_, _>>()?,
        ),
        GoalDesc::Or(parts) => GroundGoal::Or(
            parts
                .iter()
                .map(|p| ground_goal(p, binding, objects_by_type))
                .collect::<Result<_, _>>()?,
        ),
        GoalDesc::Imply(ante, conseq) => GroundGoal::Or(vec![
            GroundGoal::Not(Box::new(ground_goal(ante, binding, objects_by_type)?)),
            ground_goal(conseq, binding, objects_by_type)?,
        ]),
        // Quantifier expansion: extend `binding` with every combination the
        // bound variables can take (the exact Cartesian-product enumeration
        // `enumerate_bindings` already performs for action/method parameter
        // grounding), ground `body` under each extended binding, and fold
        // the results into `And`/`Or`. Recursion handles nested quantifiers
        // (a `Forall` inside an `Exists`, etc.) for free, since each
        // recursive call carries the accumulated binding forward and a
        // same-named inner variable naturally shadows an outer one
        // (`BTreeMap::extend` overwrites the key).
        GoalDesc::Forall(vars, body) => GroundGoal::And(
            enumerate_bindings(vars, objects_by_type)
                .into_iter()
                .map(|extra| {
                    let mut full = binding.clone();
                    full.extend(extra);
                    ground_goal(body, &full, objects_by_type)
                })
                .collect::<Result<_, _>>()?,
        ),
        GoalDesc::Exists(vars, body) => GroundGoal::Or(
            enumerate_bindings(vars, objects_by_type)
                .into_iter()
                .map(|extra| {
                    let mut full = binding.clone();
                    full.extend(extra);
                    ground_goal(body, &full, objects_by_type)
                })
                .collect::<Result<_, _>>()?,
        ),
    })
}

/// `:goal`-specific quantifier expansion: same expansion strategy as
/// `ground_goal`'s `Forall`/`Exists` arms, but stays in `ast::GoalDesc`
/// space (via `subst_atom_ast` instead of `subst_atom`) instead of
/// collapsing straight to `GroundGoal`. `GroundedIR::goal` is kept as an
/// `ast::GoalDesc` specifically so `translate::flatten_goal` can keep
/// distinguishing a bare negative-literal goal (`Not(Atom)`, compiled to a
/// synthetic marker fact) from other shapes — the same reason this crate
/// already carries two parallel `flatten_goal` functions (this module's,
/// for preconditions/`when`-conditions, and `translate.rs`'s, for `:goal`)
/// rather than one shared one. Called once, on the whole problem `:goal`,
/// with an empty starting binding — `:goal` has no parameters of its own,
/// so the only variables that can appear are ones a `Forall`/`Exists` here
/// introduces.
fn expand_goal_quantifiers(
    goal: &GoalDesc,
    binding: &BTreeMap<String, String>,
    objects_by_type: &BTreeMap<String, Vec<String>>,
) -> Result<GoalDesc, GroundError> {
    Ok(match goal {
        GoalDesc::Empty => GoalDesc::Empty,
        GoalDesc::Atom(a) => GoalDesc::Atom(subst_atom_ast(a, binding)?),
        GoalDesc::Not(inner) => GoalDesc::Not(Box::new(expand_goal_quantifiers(
            inner,
            binding,
            objects_by_type,
        )?)),
        GoalDesc::And(parts) => GoalDesc::And(
            parts
                .iter()
                .map(|p| expand_goal_quantifiers(p, binding, objects_by_type))
                .collect::<Result<_, _>>()?,
        ),
        GoalDesc::Or(parts) => GoalDesc::Or(
            parts
                .iter()
                .map(|p| expand_goal_quantifiers(p, binding, objects_by_type))
                .collect::<Result<_, _>>()?,
        ),
        GoalDesc::Imply(a, b) => GoalDesc::Imply(
            Box::new(expand_goal_quantifiers(a, binding, objects_by_type)?),
            Box::new(expand_goal_quantifiers(b, binding, objects_by_type)?),
        ),
        GoalDesc::Forall(vars, body) => GoalDesc::And(
            enumerate_bindings(vars, objects_by_type)
                .into_iter()
                .map(|extra| {
                    let mut full = binding.clone();
                    full.extend(extra);
                    expand_goal_quantifiers(body, &full, objects_by_type)
                })
                .collect::<Result<_, _>>()?,
        ),
        GoalDesc::Exists(vars, body) => GoalDesc::Or(
            enumerate_bindings(vars, objects_by_type)
                .into_iter()
                .map(|extra| {
                    let mut full = binding.clone();
                    full.extend(extra);
                    expand_goal_quantifiers(body, &full, objects_by_type)
                })
                .collect::<Result<_, _>>()?,
        ),
    })
}

/// Evaluate a fully-ground goal formula against a ground fact set. This is
/// the general-purpose replacement for the old flat
/// "pos-subset-and-no-neg-overlap" applicability check: that check is exactly
/// `evaluate_ground_goal` on an `And`-of-`Atom`/`Not(Atom)` formula, but
/// `Or`/`Imply` cannot be represented as a flat positive/negative literal
/// set, so any precondition using them must be evaluated recursively instead.
pub fn evaluate_ground_goal(goal: &GroundGoal, facts: &BTreeSet<String>) -> bool {
    match goal {
        GroundGoal::Empty => true,
        GroundGoal::Atom(a) => facts.contains(a),
        GroundGoal::Not(inner) => !evaluate_ground_goal(inner, facts),
        GroundGoal::And(parts) => parts.iter().all(|p| evaluate_ground_goal(p, facts)),
        GroundGoal::Or(parts) => parts.iter().any(|p| evaluate_ground_goal(p, facts)),
    }
}

/// Whether `action` is applicable in a state with exactly `facts` true —
/// i.e. whether its (ground, possibly `or`/`imply`-bearing) precondition
/// holds. Thin wrapper over `evaluate_ground_goal` kept here so callers don't
/// need to reach into `GroundAction::precondition` themselves.
pub fn action_applicable(action: &GroundAction, facts: &BTreeSet<String>) -> bool {
    evaluate_ground_goal(&action.precondition, facts)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundAction {
    pub name: String,
    /// The action's precondition, fully ground and ready to evaluate via
    /// `evaluate_ground_goal`/`action_applicable`. Replaces the old flat
    /// `pos_pre`/`neg_pre` sets, which could only represent a pure
    /// conjunction of literals — insufficient once `or`/`imply` are legal in
    /// a precondition (see `ast::GoalDesc`).
    pub precondition: GroundGoal,
    /// Exactly one of these branches occurs when the action is applied.
    /// Length 1 for a deterministic action, >1 when the source effect was a
    /// top-level `oneof`.
    pub outcomes: Vec<GroundEffectBranch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundSubtask {
    pub id: String,
    pub task_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundMethod {
    pub name: String,
    pub task_name: String,
    /// The method's ground applicability condition, from
    /// `ast::MethodDef::precondition`, ready to evaluate directly via
    /// `evaluate_ground_goal`. `GroundGoal::Empty` (always holds) for a
    /// method with no declared `:precondition`. `translate::translate` checks
    /// this before offering the method as a decomposition branch, the same
    /// way it already checks `GroundAction::precondition` before offering an
    /// execution move.
    pub precondition: GroundGoal,
    pub subtasks: Vec<GroundSubtask>,
    pub order: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GroundedIR {
    pub actions: Vec<GroundAction>,
    pub methods: Vec<GroundMethod>,
    /// Was `root_tasks: Vec<String>`. Reuses `GroundSubtask` (id = the
    /// original `:htn` subtask id, task_name = its ground name) — the exact
    /// same shape `GroundMethod::subtasks` already uses, for the same reason:
    /// `translate.rs` needs the id to build stable task-network addresses.
    pub root_subtasks: Vec<GroundSubtask>,
    /// Order edges among `root_subtasks`, as raw ast subtask ids (before,
    /// after) — same convention as `GroundMethod::order`.
    pub root_order: Vec<(String, String)>,
    pub initial_facts: BTreeSet<String>,
    pub goal: GoalDesc,
}

pub(crate) fn atom_key(predicate: &str, args: &[String]) -> String {
    if args.is_empty() {
        predicate.to_owned()
    } else {
        format!("{predicate}({})", args.join(","))
    }
}

/// Every type reachable from `type_name` by following declared `child -
/// parent` edges (the union of its transitive ancestors — a multiply
/// declared type contributes each of its declared parents), ending with
/// `object`. A diamond (two different paths reaching the same ancestor) is
/// ordinary in multi-parent hierarchies and not a cycle; only a type that
/// reappears on the CURRENT path is (`GroundError::TypeCycle` — white/grey/
/// black depth-first walk, so shared ancestors visited through a finished
/// sibling branch don't false-positive).
fn ancestors_of(type_name: &str, types: &TypeDef) -> Result<Vec<String>, GroundError> {
    fn visit(
        current: &str,
        types: &TypeDef,
        grey: &mut BTreeSet<String>,
        black: &mut BTreeSet<String>,
        out: &mut Vec<String>,
    ) -> Result<(), GroundError> {
        grey.insert(current.to_owned());
        out.push(current.to_owned());
        for parent in types.parents.get(current).into_iter().flatten() {
            if grey.contains(parent) {
                return Err(GroundError::TypeCycle(parent.clone()));
            }
            if black.contains(parent) {
                continue;
            }
            visit(parent, types, grey, black, out)?;
        }
        grey.remove(current);
        black.insert(current.to_owned());
        Ok(())
    }

    let mut grey: BTreeSet<String> = BTreeSet::new();
    let mut black: BTreeSet<String> = BTreeSet::new();
    let mut chain: Vec<String> = Vec::new();
    visit(type_name, types, &mut grey, &mut black, &mut chain)?;
    if chain.last().map(String::as_str) != Some("object") {
        chain.push("object".to_owned());
    }
    Ok(chain)
}

/// type -> {type} ∪ every type that is a (transitive) subtype of it.
pub fn build_type_closure(
    domain: &Domain,
) -> Result<BTreeMap<String, BTreeSet<String>>, GroundError> {
    let mut all_types = BTreeSet::new();
    all_types.insert("object".to_owned());
    for (child, parents) in &domain.types.parents {
        all_types.insert(child.clone());
        for parent in parents {
            all_types.insert(parent.clone());
        }
    }
    let param_lists = domain
        .predicates
        .iter()
        .map(|p| &p.params)
        .chain(domain.tasks.iter().map(|t| &t.params))
        .chain(domain.actions.iter().map(|a| &a.params))
        .chain(domain.methods.iter().map(|m| &m.params));
    for params in param_lists {
        for p in params {
            all_types.insert(p.type_name.clone());
        }
    }
    for o in &domain.constants {
        all_types.insert(o.type_name.clone());
    }

    let mut descendants: BTreeMap<String, BTreeSet<String>> = all_types
        .iter()
        .map(|t| (t.clone(), BTreeSet::new()))
        .collect();
    for ty in &all_types {
        for ancestor in ancestors_of(ty, &domain.types)? {
            descendants.entry(ancestor).or_default().insert(ty.clone());
        }
    }
    Ok(descendants)
}

pub fn index_objects_by_type(
    domain: &Domain,
    problem: &Problem,
    closure: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, Vec<String>> {
    let mut objects: Vec<&TypedObject> = domain
        .constants
        .iter()
        .chain(problem.objects.iter())
        .collect();
    objects.sort_by(|a, b| a.name.cmp(&b.name));
    closure
        .iter()
        .map(|(ty, descendants)| {
            let names = objects
                .iter()
                .filter(|o| descendants.contains(&o.type_name))
                .map(|o| o.name.clone())
                .collect::<Vec<_>>();
            (ty.clone(), names)
        })
        .collect()
}

fn subst_term(term: &Term, binding: &BTreeMap<String, String>) -> Result<String, GroundError> {
    match term {
        Term::Const(c) => Ok(c.clone()),
        Term::Var(v) => binding
            .get(v)
            .cloned()
            .ok_or_else(|| GroundError::UnboundVariable(v.clone())),
    }
}

fn subst_atom(
    atom: &AtomicFormula,
    binding: &BTreeMap<String, String>,
) -> Result<String, GroundError> {
    let args = atom
        .args
        .iter()
        .map(|t| subst_term(t, binding))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(atom_key(&atom.predicate, &args))
}

/// Like `subst_atom`, but keeps the AST shape (`Term::Const` args) instead
/// of collapsing straight to a ground atom-key string. Used by
/// `expand_goal_quantifiers`, which must hand back an `ast::GoalDesc` (see
/// that function's doc comment for why `:goal` needs the AST shape rather
/// than `GroundGoal`).
fn subst_atom_ast(
    atom: &AtomicFormula,
    binding: &BTreeMap<String, String>,
) -> Result<AtomicFormula, GroundError> {
    let args = atom
        .args
        .iter()
        .map(|t| Ok(Term::Const(subst_term(t, binding)?)))
        .collect::<Result<Vec<_>, GroundError>>()?;
    Ok(AtomicFormula {
        predicate: atom.predicate.clone(),
        args,
    })
}

fn flatten_goal(
    goal: &GoalDesc,
    binding: &BTreeMap<String, String>,
    pos: &mut BTreeSet<String>,
    neg: &mut BTreeSet<String>,
) -> Result<(), GroundError> {
    match goal {
        GoalDesc::Empty => Ok(()),
        GoalDesc::Atom(a) => {
            pos.insert(subst_atom(a, binding)?);
            Ok(())
        }
        GoalDesc::Not(inner) => match inner.as_ref() {
            GoalDesc::Atom(a) => {
                neg.insert(subst_atom(a, binding)?);
                Ok(())
            }
            _ => Err(GroundError::UnsupportedPrecondition(
                "'not' of a non-atomic goal description is out of scope".to_owned(),
            )),
        },
        GoalDesc::And(parts) => {
            for p in parts {
                flatten_goal(p, binding, pos, neg)?;
            }
            Ok(())
        }
        GoalDesc::Or(_) | GoalDesc::Imply(_, _) => Err(GroundError::UnsupportedPrecondition(
            "'or'/'imply' in a goal description is out of scope".to_owned(),
        )),
        GoalDesc::Forall(_, _) | GoalDesc::Exists(_, _) => {
            Err(GroundError::UnsupportedPrecondition(
                "'forall'/'exists' inside a 'when'-condition is out of scope".to_owned(),
            ))
        }
    }
}

fn collect_effect(
    effect: &Effect,
    binding: &BTreeMap<String, String>,
    branch: &mut GroundEffectBranch,
) -> Result<(), GroundError> {
    match effect {
        Effect::Empty => Ok(()),
        Effect::Literal(Literal::Pos(a)) => {
            branch.add.insert(subst_atom(a, binding)?);
            Ok(())
        }
        Effect::Literal(Literal::Neg(a)) => {
            branch.del.insert(subst_atom(a, binding)?);
            Ok(())
        }
        Effect::And(parts) => {
            for p in parts {
                collect_effect(p, binding, branch)?;
            }
            Ok(())
        }
        Effect::When(cond, inner) => {
            let mut pos_cond = BTreeSet::new();
            let mut neg_cond = BTreeSet::new();
            flatten_goal(cond, binding, &mut pos_cond, &mut neg_cond)?;
            let mut inner_branch = GroundEffectBranch::default();
            collect_effect(inner, binding, &mut inner_branch)?;
            if !inner_branch.conditional.is_empty() {
                return Err(GroundError::UnsupportedPrecondition(
                    "nested 'when' is out of scope".to_owned(),
                ));
            }
            branch.conditional.push(GroundConditional {
                pos_cond,
                neg_cond,
                add: inner_branch.add,
                del: inner_branch.del,
            });
            Ok(())
        }
        Effect::Oneof(_) => Err(GroundError::UnsupportedPrecondition(
            "'oneof' is only permitted at the top of an action's effect".to_owned(),
        )),
        Effect::Increase(fluent, _) | Effect::Decrease(fluent, _) => Err(
            GroundError::UnsupportedNumericFluent(fluent.predicate.clone()),
        ),
    }
}

fn ground_effect_branch(
    effect: &Effect,
    binding: &BTreeMap<String, String>,
) -> Result<GroundEffectBranch, GroundError> {
    let mut branch = GroundEffectBranch::default();
    collect_effect(effect, binding, &mut branch)?;
    Ok(branch)
}

/// `weights`, when present, is the declaring action's
/// `probability_weights` -- one raw-text weight per top-level `oneof`
/// branch, in declaration order (see `ast::ActionDef::probability_weights`).
/// Ignored for a non-`oneof` effect (a deterministic effect has exactly one
/// outcome and no weight to carry).
fn ground_effect(
    effect: &Effect,
    binding: &BTreeMap<String, String>,
    weights: Option<&[String]>,
) -> Result<Vec<GroundEffectBranch>, GroundError> {
    match effect {
        Effect::Oneof(branches) => branches
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let mut branch = ground_effect_branch(b, binding)?;
                branch.probability_weight = weights.and_then(|w| w.get(i)).cloned();
                Ok(branch)
            })
            .collect(),
        other => Ok(vec![ground_effect_branch(other, binding)?]),
    }
}

/// Whether a fully-ground precondition is possibly satisfiable under the
/// delete relaxation: positive atoms must be in `facts`; `not` (and, by
/// extension, `imply`'s lowered `Or(Not(_), _)`) is treated as *always*
/// satisfiable rather than checked against `facts`. This is the standard
/// delete-relaxation move (ignore delete effects) extended permissively to
/// negative preconditions too: `facts` only ever grows in
/// `compute_reachability`'s fixpoint, so there is no sound way to falsify a
/// negative literal against it without also tracking closed-world absence —
/// treating `not` as always-true keeps this check *sound* (it can under-prune
/// by treating something as reachable when it isn't, but can never
/// over-prune a genuinely reachable action) at the cost of some pruning
/// precision on domains that lean on negative preconditions.
fn relaxed_satisfiable(goal: &GroundGoal, facts: &BTreeSet<String>) -> bool {
    match goal {
        GroundGoal::Empty => true,
        GroundGoal::Atom(a) => facts.contains(a),
        GroundGoal::Not(_) => true,
        GroundGoal::And(parts) => parts.iter().all(|p| relaxed_satisfiable(p, facts)),
        GroundGoal::Or(parts) => parts.iter().any(|p| relaxed_satisfiable(p, facts)),
    }
}

/// Result of the delete-relaxation reachability pre-pass (`compute_reachability`):
/// the monotone set of ground facts optimistically reachable from the initial
/// state, and the exact set of ground action names (`GroundAction::name`
/// strings) whose precondition became `relaxed_satisfiable` at some point
/// during that fixpoint. Sound as an over-approximation of real reachability:
/// every fact/action reachable in the real, non-relaxed transition system is
/// also reachable here, so filtering on `reachable_actions` never discards a
/// genuinely-reachable ground action.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReachabilityInfo {
    pub facts: BTreeSet<String>,
    pub reachable_actions: BTreeSet<String>,
}

/// Run the delete-relaxation reachability fixpoint over `domain`'s actions:
/// starting from `initial_facts`, repeatedly find action bindings whose
/// ground precondition is `relaxed_satisfiable` against the facts discovered
/// so far, union in *all* of their possible add-effects (every `oneof`
/// outcome, and every conditional branch's `add` set unconditionally — both
/// permissive choices that only ever enlarge the relaxed-reachable set,
/// preserving soundness), and repeat until nothing new is discovered.
///
/// Each action schema's binding list is built once up front via
/// `enumerate_bindings` (the same Cartesian-product cost `ground_actions`
/// pays) rather than rebuilt every fixpoint round; only the applicability
/// check re-runs per round, against a set of not-yet-proven-reachable
/// bindings that shrinks monotonically. This is a plain fixpoint, not a
/// join-based/incremental grounder — it does not itself avoid the
/// combinatorial binding enumeration, only the cost of substituting and
/// storing a full `GroundAction` for instances later proven unreachable.
pub fn compute_reachability(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    initial_facts: &BTreeSet<String>,
    limits: &GroundingLimits,
) -> Result<ReachabilityInfo, GroundError> {
    let start = Instant::now();
    let per_action_bindings: Vec<(&ActionDef, Vec<BTreeMap<String, String>>)> = domain
        .actions
        .iter()
        .map(|a| (a, enumerate_bindings(&a.params, objects_by_type)))
        .collect();

    let mut facts = initial_facts.clone();
    let mut reachable_actions: BTreeSet<String> = BTreeSet::new();
    let mut pending: Vec<BTreeSet<usize>> = per_action_bindings
        .iter()
        .map(|(_, bindings)| (0..bindings.len()).collect())
        .collect();

    loop {
        let mut changed = false;
        for (i, (action, bindings)) in per_action_bindings.iter().enumerate() {
            let still_pending: Vec<usize> = pending[i].iter().copied().collect();
            for bidx in still_pending {
                check_wall_deadline(start, limits)?;
                let binding = &bindings[bidx];
                let precondition = ground_goal(&action.precondition, binding, objects_by_type)?;
                if !relaxed_satisfiable(&precondition, &facts) {
                    continue;
                }
                pending[i].remove(&bidx);
                let args = action
                    .params
                    .iter()
                    .map(|p| binding[&p.var].clone())
                    .collect::<Vec<_>>();
                reachable_actions.insert(atom_key(&action.name, &args));
                for outcome in ground_effect(
                    &action.effect,
                    binding,
                    action.probability_weights.as_deref(),
                )? {
                    for a in outcome.add {
                        if facts.insert(a) {
                            changed = true;
                        }
                    }
                    for cond in outcome.conditional {
                        for a in cond.add {
                            if facts.insert(a) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    Ok(ReachabilityInfo {
        facts,
        reachable_actions,
    })
}

/// The declared task name a ground task-instance name (e.g. `"drive(l1,l2)"`)
/// was built from — everything before the first `(`, or the whole string for
/// a zero-arity task. Used to tell whether a `GroundSubtask` names a
/// primitive task (matches a domain action's declared name) or a compound one
/// (decomposed further by other methods).
fn task_base_name(ground_task_name: &str) -> &str {
    ground_task_name
        .split('(')
        .next()
        .unwrap_or(ground_task_name)
}

/// Lazy Cartesian-product binding generator for a parameter list against a
/// typed object universe. Produces exactly the same bindings, in exactly the
/// same order, that eagerly building the full product up front would (see
/// `enumerate_bindings`, which is now a thin `.collect()` over this) — but
/// one binding at a time, using O(#params) working memory and O(#params) time
/// per `next()` call, rather than materializing all M^N combinations before
/// a caller sees the first one.
///
/// This is the fix for the DoS vector documented at this module's top: a
/// caller enforcing a `GroundingLimits` counter (`ground_actions`,
/// `ground_actions_reachable`, `ground_methods`, `ground_methods_reachable`)
/// can check the counter after each `next()` and `return` as soon as it's
/// exceeded, so the total cost of a rejected high-arity schema is bounded by
/// the limit, never by the schema's full (possibly astronomical) Cartesian
/// product size.
///
/// Ordering: bindings are produced with the *last* parameter varying
/// fastest — a plain mixed-radix odometer over `choices`, incrementing from
/// the least-significant (last) position and carrying left on overflow. This
/// matches the original nested-loop construction (`for b in bindings { for c
/// in choices { ... } }`, outer loop over already-built prefixes, inner loop
/// over the newest parameter's choices) exactly, so switching to this lazy
/// form changes no existing test's expected ground order or count.
struct BindingIter {
    vars: Vec<String>,
    choices: Vec<Vec<String>>,
    idx: Vec<usize>,
    /// Zero-param schema: exactly one binding (the empty map) exists.
    /// `Some(false)` before it's been yielded, `Some(true)` after. `None`
    /// when `vars` is non-empty, where this field is unused.
    zero_param_yielded: Option<bool>,
    exhausted: bool,
}

impl BindingIter {
    fn new(params: &[TypedParam], objects_by_type: &BTreeMap<String, Vec<String>>) -> Self {
        let vars: Vec<String> = params.iter().map(|p| p.var.clone()).collect();
        let choices: Vec<Vec<String>> = params
            .iter()
            .map(|p| {
                objects_by_type
                    .get(&p.type_name)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect();
        // A non-empty param list with any zero-choice param has an empty
        // product (matches the original: once `choices` is empty for one
        // param, every subsequent `next` stays `vec![]`).
        let exhausted = !vars.is_empty() && choices.iter().any(|c| c.is_empty());
        let idx = vec![0; vars.len()];
        let zero_param_yielded = if vars.is_empty() { Some(false) } else { None };
        Self {
            vars,
            choices,
            idx,
            zero_param_yielded,
            exhausted,
        }
    }
}

impl Iterator for BindingIter {
    type Item = BTreeMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(yielded) = self.zero_param_yielded {
            if yielded {
                return None;
            }
            self.zero_param_yielded = Some(true);
            return Some(BTreeMap::new());
        }
        if self.exhausted {
            return None;
        }
        let mut binding = BTreeMap::new();
        for i in 0..self.vars.len() {
            binding.insert(self.vars[i].clone(), self.choices[i][self.idx[i]].clone());
        }
        // Advance the odometer from the least-significant (last) position,
        // carrying left on overflow.
        let mut pos = self.vars.len() - 1;
        loop {
            self.idx[pos] += 1;
            if self.idx[pos] < self.choices[pos].len() {
                break;
            }
            self.idx[pos] = 0;
            if pos == 0 {
                self.exhausted = true;
                break;
            }
            pos -= 1;
        }
        Some(binding)
    }
}

/// Eager convenience wrapper over `BindingIter` for call sites that either
/// need the full list at once (quantifier expansion, which folds every
/// extension into one `And`/`Or`) or need random access into it by index
/// (`compute_reachability`'s pending-binding-set bookkeeping) — neither has a
/// `GroundingLimits` counter to check incrementally against, so eager
/// materialization is what they already need. Grounding call sites that
/// *do* have a counter to enforce (`ground_actions` et al.) use `BindingIter`
/// directly instead, specifically so they never pay for this materialization
/// — see `BindingIter`'s doc comment.
fn enumerate_bindings(
    params: &[TypedParam],
    objects_by_type: &BTreeMap<String, Vec<String>>,
) -> Vec<BTreeMap<String, String>> {
    BindingIter::new(params, objects_by_type).collect()
}

pub fn ground_actions(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) -> Result<Vec<GroundAction>, GroundError> {
    let start = Instant::now();
    let mut out = Vec::new();
    for action in &domain.actions {
        // `BindingIter` directly, not `enumerate_bindings` — see its doc
        // comment: this lets the `max_ground_actions` check below run after
        // every single binding, so a high-arity/high-object schema is
        // rejected after producing only `max_ground_actions + 1` bindings,
        // never the full (possibly astronomical) Cartesian product.
        for binding in BindingIter::new(&action.params, objects_by_type) {
            check_wall_deadline(start, limits)?;
            if out.len() >= limits.max_ground_actions {
                return Err(GroundError::LimitExceeded(format!(
                    "max_ground_actions ({}) exceeded",
                    limits.max_ground_actions
                )));
            }
            let precondition = ground_goal(&action.precondition, &binding, objects_by_type)?;
            let outcomes = ground_effect(
                &action.effect,
                &binding,
                action.probability_weights.as_deref(),
            )?;
            let args = action
                .params
                .iter()
                .map(|p| binding[&p.var].clone())
                .collect::<Vec<_>>();
            out.push(GroundAction {
                name: atom_key(&action.name, &args),
                precondition,
                outcomes,
            });
        }
    }
    Ok(out)
}

/// Like `ground_actions`, but for each binding first computes only the cheap
/// ground name (a plain parameter substitution — no precondition/effect
/// traversal) and skips the rest of the instantiation entirely when that name
/// is not in `reachability.reachable_actions`. Sound: `reachable_actions` is
/// an over-approximation of real reachability (see `compute_reachability`),
/// so this never drops an action that could actually occur.
pub fn ground_actions_reachable(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    reachability: &ReachabilityInfo,
) -> Result<Vec<GroundAction>, GroundError> {
    let start = Instant::now();
    let mut out = Vec::new();
    for action in &domain.actions {
        // `BindingIter` directly (see `ground_actions`'s comment on the same
        // pattern) so the limit check below never waits on a fully
        // materialized Cartesian product.
        for binding in BindingIter::new(&action.params, objects_by_type) {
            check_wall_deadline(start, limits)?;
            let args = action
                .params
                .iter()
                .map(|p| binding[&p.var].clone())
                .collect::<Vec<_>>();
            let name = atom_key(&action.name, &args);
            if !reachability.reachable_actions.contains(&name) {
                continue;
            }
            if out.len() >= limits.max_ground_actions {
                return Err(GroundError::LimitExceeded(format!(
                    "max_ground_actions ({}) exceeded",
                    limits.max_ground_actions
                )));
            }
            let precondition = ground_goal(&action.precondition, &binding, objects_by_type)?;
            let outcomes = ground_effect(
                &action.effect,
                &binding,
                action.probability_weights.as_deref(),
            )?;
            out.push(GroundAction {
                name,
                precondition,
                outcomes,
            });
        }
    }
    Ok(out)
}

pub fn ground_methods(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) -> Result<Vec<GroundMethod>, GroundError> {
    let start = Instant::now();
    let mut out = Vec::new();
    for method in &domain.methods {
        // `BindingIter` directly (see `ground_actions`'s comment on the same
        // pattern) so the `max_ground_methods` check never waits on a fully
        // materialized Cartesian product.
        for binding in BindingIter::new(&method.params, objects_by_type) {
            check_wall_deadline(start, limits)?;
            if out.len() >= limits.max_ground_methods {
                return Err(GroundError::LimitExceeded(format!(
                    "max_ground_methods ({}) exceeded",
                    limits.max_ground_methods
                )));
            }
            let task_args = method
                .task
                .args
                .iter()
                .map(|t| subst_term(t, &binding))
                .collect::<Result<Vec<_>, _>>()?;
            let subtasks = method
                .network
                .subtasks
                .iter()
                .map(|st| {
                    let args = st
                        .task
                        .args
                        .iter()
                        .map(|t| subst_term(t, &binding))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(GroundSubtask {
                        id: st.id.clone(),
                        task_name: atom_key(&st.task.name, &args),
                    })
                })
                .collect::<Result<Vec<_>, GroundError>>()?;
            let order = method
                .network
                .order
                .iter()
                .map(|e| (e.before.clone(), e.after.clone()))
                .collect();
            let precondition = ground_goal(&method.precondition, &binding, objects_by_type)?;
            let args = method
                .params
                .iter()
                .map(|p| binding[&p.var].clone())
                .collect::<Vec<_>>();
            out.push(GroundMethod {
                name: atom_key(&method.name, &args),
                task_name: atom_key(&method.task.name, &task_args),
                precondition,
                subtasks,
                order,
            });
        }
    }
    Ok(out)
}

/// Like `ground_methods`, but skips a ground method instance whenever one of
/// its own ground subtasks names a *primitive* task (its base name matches a
/// domain action's declared name) that `reachability.reachable_actions`
/// proved can never fire. Compound subtasks (base name matches no action) are
/// never used to prune — this crate does not propagate reachability through
/// the compound-task/method decomposition graph, so a method is only pruned
/// on positive proof about one of its own primitive subtasks, never on an
/// unverified guess about a nested method chain several levels down. That
/// makes this sound in the same sense `ground_actions_reachable` is: it only
/// ever removes a ground method already proven unusable, never a possibly-
/// usable one.
pub fn ground_methods_reachable(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
    reachability: &ReachabilityInfo,
) -> Result<Vec<GroundMethod>, GroundError> {
    let start = Instant::now();
    let action_names: BTreeSet<&str> = domain.actions.iter().map(|a| a.name.as_str()).collect();
    let mut out = Vec::new();
    for method in &domain.methods {
        // `BindingIter` directly (see `ground_actions`'s comment on the same
        // pattern) so the `max_ground_methods` check never waits on a fully
        // materialized Cartesian product.
        for binding in BindingIter::new(&method.params, objects_by_type) {
            check_wall_deadline(start, limits)?;
            let task_args = method
                .task
                .args
                .iter()
                .map(|t| subst_term(t, &binding))
                .collect::<Result<Vec<_>, _>>()?;
            let subtasks = method
                .network
                .subtasks
                .iter()
                .map(|st| {
                    let args = st
                        .task
                        .args
                        .iter()
                        .map(|t| subst_term(t, &binding))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(GroundSubtask {
                        id: st.id.clone(),
                        task_name: atom_key(&st.task.name, &args),
                    })
                })
                .collect::<Result<Vec<_>, GroundError>>()?;
            let has_unreachable_primitive_subtask = subtasks.iter().any(|st| {
                action_names.contains(task_base_name(&st.task_name))
                    && !reachability.reachable_actions.contains(&st.task_name)
            });
            if has_unreachable_primitive_subtask {
                continue;
            }
            if out.len() >= limits.max_ground_methods {
                return Err(GroundError::LimitExceeded(format!(
                    "max_ground_methods ({}) exceeded",
                    limits.max_ground_methods
                )));
            }
            let order = method
                .network
                .order
                .iter()
                .map(|e| (e.before.clone(), e.after.clone()))
                .collect();
            let precondition = ground_goal(&method.precondition, &binding, objects_by_type)?;
            let args = method
                .params
                .iter()
                .map(|p| binding[&p.var].clone())
                .collect::<Vec<_>>();
            out.push(GroundMethod {
                name: atom_key(&method.name, &args),
                task_name: atom_key(&method.task.name, &task_args),
                precondition,
                subtasks,
                order,
            });
        }
    }
    Ok(out)
}

fn ground_term_const(term: &Term) -> Result<String, GroundError> {
    match term {
        Term::Const(c) => Ok(c.clone()),
        Term::Var(v) => Err(GroundError::UnboundVariable(v.clone())),
    }
}

pub fn ground_initial_facts(problem: &Problem) -> Result<BTreeSet<String>, GroundError> {
    problem
        .init
        .iter()
        .map(|a| {
            let args = a
                .args
                .iter()
                .map(ground_term_const)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(atom_key(&a.predicate, &args))
        })
        .collect()
}

/// Was `ground_root_tasks`. Grounds the problem's root `:htn` task network
/// into `(root_subtasks, root_order)`, preserving both subtask ids and the
/// order edges among them — needed by `translate.rs` to build stable
/// task-network addresses and restrict the BFS to actual decompositions of
/// the root network, not just precondition-satisfying ground actions.
#[allow(clippy::type_complexity)]
pub fn ground_root_network(
    problem: &Problem,
) -> Result<(Vec<GroundSubtask>, Vec<(String, String)>), GroundError> {
    let subtasks = problem
        .htn
        .subtasks
        .iter()
        .map(|st| {
            let args = st
                .task
                .args
                .iter()
                .map(ground_term_const)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(GroundSubtask {
                id: st.id.clone(),
                task_name: atom_key(&st.task.name, &args),
            })
        })
        .collect::<Result<Vec<_>, GroundError>>()?;
    let order = problem
        .htn
        .order
        .iter()
        .map(|e| (e.before.clone(), e.after.clone()))
        .collect();
    Ok((subtasks, order))
}

/// Ground a parsed `Domain`+`Problem` pair into a `GroundedIR`: every
/// combination of typed objects substituted into every action/method
/// (subject to `limits`), the initial state as ground fact strings, and the
/// problem's `:goal` with any quantifiers expanded.
///
/// Runs `validate::validate_domain`/`validate::validate_problem` first, then
/// refuses (before any actual grounding work) any domain/problem using a
/// numeric fluent or a `:constraints` block — both are lexed/parsed by this
/// crate but are out of scope to ground (see `ast`'s module docs). When
/// `limits.prune_unreachable` is set, a delete-relaxation reachability
/// pre-pass (`compute_reachability`) is used to skip instantiating ground
/// actions/methods that can never fire; otherwise every typed-object
/// combination is enumerated (bounded by `limits.max_ground_actions`/
/// `max_ground_methods`) — see the module docs for the full comparison.
///
/// # Errors
///
/// Returns `GroundError::Validation` if pre-grounding validation fails,
/// `GroundError::UnsupportedNumericFluent`/`UnsupportedConstraint` for an
/// out-of-scope construct, `GroundError::LimitExceeded` if grounding would
/// exceed `limits`, `GroundError::UnboundVariable` for a variable used
/// without a binding, and `GroundError::TypeCycle` for a cyclic `:types`
/// hierarchy.
///
/// # Examples
///
/// ```
/// use ferroplan_hddl::grounder::{ground, GroundingLimits};
/// use ferroplan_hddl::parser::{parse_domain, parse_problem};
///
/// let domain = parse_domain(r#"
///     (define (domain doors)
///       (:predicates (open ?d))
///       (:action open-door
///         :parameters (?d)
///         :precondition ()
///         :effect (open ?d)))
/// "#).unwrap();
/// let problem = parse_problem(r#"
///     (define (problem doors-p1)
///       (:domain doors)
///       (:objects d1)
///       (:init)
///       (:goal (open d1))
///       (:htn :ordered-subtasks (open-door d1)))
/// "#).unwrap();
///
/// let ir = ground(&domain, &problem, &GroundingLimits::default())
///     .expect("domain and problem ground cleanly");
/// assert_eq!(ir.actions.len(), 1);
/// assert_eq!(ir.actions[0].name, "open-door(d1)");
/// ```
pub fn ground(
    domain: &Domain,
    problem: &Problem,
    limits: &GroundingLimits,
) -> Result<GroundedIR, GroundError> {
    // Loud, precise refusal for constructs this pass lexes/parses but does
    // not (yet) ground — see the `ast` module docs. Checked before anything
    // else so a numeric-fluent or `:constraints` domain never falls through
    // to a confusing downstream failure.
    if let Some(f) = domain.numeric_fluents.first() {
        return Err(GroundError::UnsupportedNumericFluent(f.name.clone()));
    }
    if let Some(c) = domain.constraints.first() {
        return Err(GroundError::UnsupportedConstraint(c.kind.clone()));
    }
    if let Some(c) = problem.constraints.first() {
        return Err(GroundError::UnsupportedConstraint(c.kind.clone()));
    }
    validate::validate_domain(domain).map_err(GroundError::Validation)?;
    validate::validate_problem(domain, problem).map_err(GroundError::Validation)?;
    let closure = build_type_closure(domain)?;
    let objects_by_type = index_objects_by_type(domain, problem, &closure);
    let initial_facts = ground_initial_facts(problem)?;
    let (actions, methods) = if limits.prune_unreachable {
        let reachability = compute_reachability(domain, &objects_by_type, &initial_facts, limits)?;
        let actions = ground_actions_reachable(domain, &objects_by_type, limits, &reachability)?;
        let methods = ground_methods_reachable(domain, &objects_by_type, limits, &reachability)?;
        (actions, methods)
    } else {
        let actions = ground_actions(domain, &objects_by_type, limits)?;
        let methods = ground_methods(domain, &objects_by_type, limits)?;
        (actions, methods)
    };
    let (root_subtasks, root_order) = ground_root_network(problem)?;
    Ok(GroundedIR {
        actions,
        methods,
        root_subtasks,
        root_order,
        initial_facts,
        goal: expand_goal_quantifiers(&problem.goal, &BTreeMap::new(), &objects_by_type)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_domain, parse_problem};

    const FIXTURE_A_DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../fixtures/a/problem.hddl");
    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_C_PROBLEM: &str = include_str!("../fixtures/c/problem.hddl");
    const FIXTURE_E_DOMAIN: &str = include_str!("../fixtures/e/domain.hddl");
    const FIXTURE_E_PROBLEM: &str = include_str!("../fixtures/e/problem.hddl");

    /// Deterministic (no reliance on real elapsed wall time from a slow
    /// operation): backdates `start` by a fixed offset larger than the
    /// configured `max_wall`, so `check_wall_deadline` must observe
    /// `elapsed > max_wall` on every run regardless of machine speed.
    #[test]
    fn check_wall_deadline_times_out_once_elapsed_exceeds_max_wall() {
        let limits = GroundingLimits {
            max_wall: Some(Duration::from_millis(10)),
            ..GroundingLimits::default()
        };
        let backdated_start = Instant::now() - Duration::from_millis(50);
        let err = check_wall_deadline(backdated_start, &limits).unwrap_err();
        match err {
            GroundError::Timeout {
                elapsed_ms,
                limit_ms,
            } => {
                assert!(
                    elapsed_ms >= 50,
                    "expected >=50ms elapsed, got {elapsed_ms}"
                );
                assert_eq!(limit_ms, 10);
            }
            other => panic!("expected Timeout, got {other:?}"),
        }
    }

    #[test]
    fn check_wall_deadline_is_ok_within_budget() {
        let limits = GroundingLimits {
            max_wall: Some(Duration::from_secs(10)),
            ..GroundingLimits::default()
        };
        assert!(check_wall_deadline(Instant::now(), &limits).is_ok());
    }

    #[test]
    fn check_wall_deadline_never_fires_when_max_wall_is_none() {
        let limits = GroundingLimits {
            max_wall: None,
            ..GroundingLimits::default()
        };
        let ancient_start = Instant::now() - Duration::from_secs(3600);
        assert!(check_wall_deadline(ancient_start, &limits).is_ok());
    }

    /// End-to-end confirmation that `ground()` itself surfaces the timeout
    /// (not just the private helper): a `max_wall` of zero duration means
    /// the very first `check_wall_deadline` call inside `ground_actions`
    /// (its `start` is `Instant::now()` taken at entry, and any nonzero time
    /// passes before the first binding's check) is already "elapsed", so
    /// fixture A -- which has real, non-empty ground work to do -- reliably
    /// refuses with `GroundError::Timeout` on every run, not just a rare
    /// slow one.
    #[test]
    fn ground_refuses_with_timeout_when_max_wall_is_zero() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        let limits = GroundingLimits {
            max_wall: Some(Duration::from_nanos(0)),
            ..GroundingLimits::default()
        };
        let err = ground(&domain, &problem, &limits).unwrap_err();
        assert!(
            matches!(err, GroundError::Timeout { .. }),
            "expected Timeout, got {err:?}"
        );
    }

    #[test]
    fn probabilistic_effect_weights_are_ground_onto_outcomes() {
        let domain = parse_domain(FIXTURE_E_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_E_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let action = ir
            .actions
            .iter()
            .find(|a| a.name == "toss-coin")
            .expect("toss-coin ground instance present");
        assert_eq!(action.outcomes.len(), 2);
        assert_eq!(action.outcomes[0].probability_weight.as_deref(), Some("3"));
        assert_eq!(action.outcomes[1].probability_weight.as_deref(), Some("1"));
    }

    #[test]
    fn fixture_a_grounds_expected_counts() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        // pickup/?l (1 param) x2 objects, drive/?a?b (2 params) x4, dropoff/?l x2
        assert_eq!(ir.actions.len(), 2 + 4 + 2);
        // m-deliver/?from?to (2 params) x4 object combinations
        assert_eq!(ir.methods.len(), 4);
        assert_eq!(
            ir.initial_facts,
            ["at(l1)", "connected(l1,l2)"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        );
        assert_eq!(ir.root_subtasks.len(), 1);
        assert_eq!(ir.root_subtasks[0].task_name, "deliver(l1,l2)");
        assert!(ir.root_order.is_empty());
        let m = ir
            .methods
            .iter()
            .find(|m| m.name == "m-deliver(l1,l2)")
            .expect("m-deliver(l1,l2) ground instance present");
        assert_eq!(m.task_name, "deliver(l1,l2)");
        assert_eq!(m.subtasks.len(), 3);
        assert_eq!(m.order.len(), 2);
    }

    #[test]
    fn oneof_effect_produces_one_outcome_per_branch() {
        let domain = parse_domain(FIXTURE_C_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_C_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let action = ir
            .actions
            .iter()
            .find(|a| a.name == "cross-bridge(l1,l2,l3)")
            .expect("cross-bridge(l1,l2,l3) ground instance present");
        assert_eq!(action.outcomes.len(), 2);
        let success = &action.outcomes[0];
        assert!(success.add.contains("at(l2)"));
        assert!(success.del.contains("at(l1)"));
        let blocked = &action.outcomes[1];
        assert!(blocked.add.contains("at(l3)"));
        assert!(blocked.del.contains("at(l1)"));
    }

    const FIXTURE_D_DOMAIN: &str = include_str!("../fixtures/d/domain.hddl");
    const FIXTURE_D_PROBLEM: &str = include_str!("../fixtures/d/problem.hddl");

    /// Ground fixture D (an `or`/`imply` precondition domain) once and hand
    /// back its two single-object ground actions, keyed by name, for the
    /// applicability tests below.
    fn ground_fixture_d() -> (GroundAction, GroundAction) {
        let domain = parse_domain(FIXTURE_D_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_D_PROBLEM).unwrap();
        let ir = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let enter = ir
            .actions
            .iter()
            .find(|a| a.name == "enter(l1)")
            .expect("enter(l1) ground instance present")
            .clone();
        let signal = ir
            .actions
            .iter()
            .find(|a| a.name == "signal(l1)")
            .expect("signal(l1) ground instance present")
            .clone();
        (enter, signal)
    }

    fn facts(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn fixture_d_grounds_or_precondition_into_a_ground_goal() {
        let (enter, _signal) = ground_fixture_d();
        // (and (at ?l) (or (open ?l) (unlocked ?l))) grounded with ?l = l1.
        match &enter.precondition {
            GroundGoal::And(parts) => {
                assert_eq!(parts.len(), 2);
                assert!(matches!(&parts[0], GroundGoal::Atom(a) if a == "at(l1)"));
                match &parts[1] {
                    GroundGoal::Or(branches) => {
                        assert_eq!(branches.len(), 2);
                        assert!(matches!(&branches[0], GroundGoal::Atom(a) if a == "open(l1)"));
                        assert!(matches!(&branches[1], GroundGoal::Atom(a) if a == "unlocked(l1)"));
                    }
                    other => panic!("expected the 'or' sub-formula, got {other:?}"),
                }
            }
            other => panic!("expected an 'and' ground goal, got {other:?}"),
        }
    }

    #[test]
    fn or_precondition_is_applicable_when_either_disjunct_holds() {
        let (enter, _signal) = ground_fixture_d();
        // Neither `open(l1)` nor `unlocked(l1)` holds: the `or` fails, so
        // `enter` is not applicable even though `at(l1)` holds.
        assert!(!action_applicable(&enter, &facts(&["at(l1)"])));
        // `open(l1)` alone satisfies the `or`.
        assert!(action_applicable(&enter, &facts(&["at(l1)", "open(l1)"])));
        // `unlocked(l1)` alone also satisfies the `or`.
        assert!(action_applicable(
            &enter,
            &facts(&["at(l1)", "unlocked(l1)"])
        ));
        // Both satisfy it a fortiori.
        assert!(action_applicable(
            &enter,
            &facts(&["at(l1)", "open(l1)", "unlocked(l1)"])
        ));
        // Missing `at(l1)` entirely: not applicable regardless of the `or`.
        assert!(!action_applicable(
            &enter,
            &facts(&["open(l1)", "unlocked(l1)"])
        ));
    }

    #[test]
    fn imply_precondition_is_vacuously_true_when_antecedent_is_false() {
        let (_enter, signal) = ground_fixture_d();
        // (imply (open ?l) (unlocked ?l)): `open(l1)` false makes the
        // implication vacuously true, so `signal` is applicable on `at(l1)`
        // alone, with neither `open` nor `unlocked` present.
        assert!(action_applicable(&signal, &facts(&["at(l1)"])));
    }

    #[test]
    fn imply_precondition_is_false_when_antecedent_holds_and_consequent_does_not() {
        let (_enter, signal) = ground_fixture_d();
        // `open(l1)` true, `unlocked(l1)` false: the implication is violated.
        assert!(!action_applicable(&signal, &facts(&["at(l1)", "open(l1)"])));
    }

    #[test]
    fn imply_precondition_holds_when_both_antecedent_and_consequent_hold() {
        let (_enter, signal) = ground_fixture_d();
        assert!(action_applicable(
            &signal,
            &facts(&["at(l1)", "open(l1)", "unlocked(l1)"])
        ));
    }

    #[test]
    fn imply_precondition_holds_when_only_consequent_holds() {
        let (_enter, signal) = ground_fixture_d();
        // `open(l1)` false, `unlocked(l1)` true: antecedent false is already
        // sufficient (vacuous truth), consequent being true too changes
        // nothing — still applicable.
        assert!(action_applicable(
            &signal,
            &facts(&["at(l1)", "unlocked(l1)"])
        ));
    }

    const NUMERIC_FLUENT_DOMAIN: &str = r#"(define (domain numeric-fluent-d)
      (:types vehicle)
      (:predicates
        (idle ?v - vehicle)
        (= (fuel-level ?v - vehicle) 0))
      (:action wait
        :parameters (?v - vehicle)
        :precondition (idle ?v)
        :effect (and (idle ?v))))"#;

    const CONSTRAINTS_DOMAIN: &str = r#"(define (domain constraints-d)
      (:types loc)
      (:predicates (at ?l - loc))
      (:constraints (and (always (at ?l))))
      (:action noop
        :parameters (?l - loc)
        :precondition ()
        :effect (and (at ?l))))"#;

    const INCREASE_EFFECT_DOMAIN: &str = r#"(define (domain increase-effect-d)
      (:types vehicle)
      (:predicates (idle ?v - vehicle))
      (:action refuel
        :parameters (?v - vehicle)
        :precondition (idle ?v)
        :effect (and (increase (fuel-level ?v) 10))))"#;

    fn minimal_problem_with_object(
        domain_name: &str,
        type_name: &str,
        root_action: &str,
    ) -> Problem {
        let src = format!(
            r#"(define (problem p)
              (:domain {domain_name})
              (:objects o1 - {type_name})
              (:init)
              (:goal ())
              (:htn :subtasks (and (r1 ({root_action} o1)))))"#
        );
        crate::parser::parse_problem(&src).expect("minimal problem parses")
    }

    /// Multi-parent diamonds (`a -> {b, c}`, `b -> d`, `c -> d`) are the
    /// ordinary shape of real competition hierarchies (IPC-2023 PO_UM-Translog's
    /// `Regular_Truck -> {Regular_Vehicle, Truck} -> Vehicle`): the closure
    /// must treat the shared ancestor reached through both branches as ONE
    /// type, not as a cycle, and every branch's ancestors must appear.
    #[test]
    fn type_closure_treats_multi_parent_diamonds_as_diamonds_not_cycles() {
        let domain = parse_domain(
            r#"(define (domain diamond-types)
              (:types
                b - d
                c - d
                a - b
                a - c
                d)
              (:predicates (p ?x - d))
              (:action noop :parameters () :precondition () :effect ()))"#,
        )
        .unwrap();
        let closure = build_type_closure(&domain).expect("diamond hierarchy is not a cycle");
        let d_descendants = &closure["d"];
        for expected in ["a", "b", "c"] {
            assert!(
                d_descendants.contains(expected),
                "d's descendants must contain {expected}: {d_descendants:?}"
            );
        }
        // `a`'s own ancestor walk covers BOTH parents.
        let b_descendants = &closure["b"];
        assert!(b_descendants.contains("a"));
        let c_descendants = &closure["c"];
        assert!(c_descendants.contains("a"));
    }

    /// A genuine cycle (`a -> b -> a`) still refuses with `TypeCycle` — the
    /// diamond tolerance above must not weaken the real tripwire.
    #[test]
    fn type_closure_refuses_genuine_cycle_with_typed_error() {
        let domain = parse_domain(
            r#"(define (domain cyclic-types)
              (:types a - b b - a)
              (:predicates (p ?x - object))
              (:action noop :parameters () :precondition () :effect ()))"#,
        )
        .unwrap();
        let err = build_type_closure(&domain).unwrap_err();
        assert!(
            matches!(err, GroundError::TypeCycle(ref t) if t == "a" || t == "b"),
            "expected TypeCycle, got {err:?}"
        );
    }

    #[test]
    fn refuses_numeric_fluent_declaration_with_typed_error() {
        let domain = parse_domain(NUMERIC_FLUENT_DOMAIN).unwrap();
        let problem = minimal_problem_with_object("numeric-fluent-d", "vehicle", "wait");
        let err = ground(&domain, &problem, &GroundingLimits::default()).unwrap_err();
        assert!(
            matches!(err, GroundError::UnsupportedNumericFluent(ref n) if n == "fuel-level"),
            "expected UnsupportedNumericFluent(\"fuel-level\"), got {err:?}"
        );
    }

    #[test]
    fn refuses_constraints_block_with_typed_error() {
        let domain = parse_domain(CONSTRAINTS_DOMAIN).unwrap();
        let problem = minimal_problem_with_object("constraints-d", "loc", "noop");
        let err = ground(&domain, &problem, &GroundingLimits::default()).unwrap_err();
        assert!(
            matches!(err, GroundError::UnsupportedConstraint(ref k) if k == "always"),
            "expected UnsupportedConstraint(\"always\"), got {err:?}"
        );
    }

    #[test]
    fn refuses_increase_effect_with_typed_error_even_without_fluent_declaration() {
        // No `(= (fuel-level ...) ...)` declaration anywhere in this domain —
        // the refusal must still fire from the effect-grounding path
        // (`collect_effect`), not just the `domain.numeric_fluents` fast path.
        let domain = parse_domain(INCREASE_EFFECT_DOMAIN).unwrap();
        assert!(domain.numeric_fluents.is_empty());
        let problem = minimal_problem_with_object("increase-effect-d", "vehicle", "refuel");
        let err = ground(&domain, &problem, &GroundingLimits::default()).unwrap_err();
        assert!(
            matches!(err, GroundError::UnsupportedNumericFluent(ref n) if n == "fuel-level"),
            "expected UnsupportedNumericFluent(\"fuel-level\"), got {err:?}"
        );
    }

    #[test]
    fn grounding_limits_are_enforced() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        let closure = build_type_closure(&domain).unwrap();
        let objects_by_type = index_objects_by_type(&domain, &problem, &closure);
        let tight = GroundingLimits {
            max_ground_actions: 1,
            max_ground_methods: 10_000,
            prune_unreachable: false,
            ..GroundingLimits::default()
        };
        let err = ground_actions(&domain, &objects_by_type, &tight).unwrap_err();
        assert!(matches!(err, GroundError::LimitExceeded(_)));
    }

    /// Regression test for the DoS vector documented at the top of this
    /// module: a single action/method schema with high parameter arity over
    /// a type with many objects has a Cartesian product (`M^N`) that can be
    /// astronomically larger than any reasonable `GroundingLimits` counter.
    /// 10 parameters over 50 objects of the same type is 50^10 (~9.8e15)
    /// bindings -- if `ground_actions`/`enumerate_bindings` ever
    /// materialized that full product (or even a sizable prefix of it far
    /// beyond the limit) before checking `max_ground_actions`, this test
    /// would hang or exhaust memory well past the 1-second budget below,
    /// instead of returning almost immediately. With the fix
    /// (`BindingIter` enumerating lazily, one binding per `next()`, checked
    /// against the limit every iteration), only `max_ground_actions + 1`
    /// bindings are ever built before the limit trips.
    fn high_arity_schema_domain_and_problem(
        domain_keyword: &str,
        num_objects: usize,
    ) -> (Domain, Problem) {
        let objects = (1..=num_objects)
            .map(|i| format!("o{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let params = (1..=10)
            .map(|i| format!("?v{i} - t"))
            .collect::<Vec<_>>()
            .join(" ");
        let domain_src = match domain_keyword {
            "action" => format!(
                r#"(define (domain high-arity-action-d)
                  (:types t)
                  (:predicates (p ?a - t))
                  (:action noop
                    :parameters ({params})
                    :precondition ()
                    :effect (and (p ?v1))))"#
            ),
            "method" => format!(
                r#"(define (domain high-arity-method-d)
                  (:types t)
                  (:predicates (p ?a - t))
                  (:task do-it :parameters (?v1 - t))
                  (:method m-high-arity
                    :parameters ({params})
                    :task (do-it ?v1)
                    :subtasks ()))"#
            ),
            other => panic!("unexpected domain_keyword {other}"),
        };
        let domain_name = if domain_keyword == "action" {
            "high-arity-action-d"
        } else {
            "high-arity-method-d"
        };
        let problem_src = format!(
            r#"(define (problem high-arity-p)
              (:domain {domain_name})
              (:objects {objects} - t)
              (:init)
              (:goal ())
              (:htn :subtasks ()))"#
        );
        let domain = parse_domain(&domain_src).expect("high-arity domain parses");
        let problem = parse_problem(&problem_src).expect("high-arity problem parses");
        (domain, problem)
    }

    #[test]
    fn ground_actions_short_circuits_on_a_high_arity_schema_instead_of_materializing_the_full_product(
    ) {
        // 10 params x 50 objects/type = 50^10 (~9.8e15) naive bindings.
        let (domain, problem) = high_arity_schema_domain_and_problem("action", 50);
        let closure = build_type_closure(&domain).unwrap();
        let objects_by_type = index_objects_by_type(&domain, &problem, &closure);
        let tight = GroundingLimits {
            max_ground_actions: 5,
            max_ground_methods: 10_000,
            prune_unreachable: false,
            ..GroundingLimits::default()
        };

        let start = std::time::Instant::now();
        let err = ground_actions(&domain, &objects_by_type, &tight).unwrap_err();
        let elapsed = start.elapsed();

        assert!(matches!(err, GroundError::LimitExceeded(_)));
        assert!(
            elapsed < std::time::Duration::from_secs(1),
            "ground_actions took {elapsed:?} -- the full Cartesian product must \
             have been (at least partially) materialized before the limit check; \
             expected the limit to trip after only a handful of bindings"
        );
    }

    #[test]
    fn ground_methods_short_circuits_on_a_high_arity_schema_instead_of_materializing_the_full_product(
    ) {
        // Same shape as the action test above, but for `ground_methods`.
        let (domain, problem) = high_arity_schema_domain_and_problem("method", 50);
        let closure = build_type_closure(&domain).unwrap();
        let objects_by_type = index_objects_by_type(&domain, &problem, &closure);
        let tight = GroundingLimits {
            max_ground_actions: 10_000,
            max_ground_methods: 5,
            prune_unreachable: false,
            ..GroundingLimits::default()
        };

        let start = std::time::Instant::now();
        let err = ground_methods(&domain, &objects_by_type, &tight).unwrap_err();
        let elapsed = start.elapsed();

        assert!(matches!(err, GroundError::LimitExceeded(_)));
        assert!(
            elapsed < std::time::Duration::from_secs(1),
            "ground_methods took {elapsed:?} -- the full Cartesian product must \
             have been (at least partially) materialized before the limit check; \
             expected the limit to trip after only a handful of bindings"
        );
    }

    #[test]
    fn binding_iter_matches_eager_enumeration_order_and_count() {
        // Direct check that the lazy `BindingIter` odometer produces exactly
        // the same bindings, in exactly the same order, that the original
        // eager nested-loop `enumerate_bindings` did -- the property this
        // whole fix depends on for byte-identical existing-test output.
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        let closure = build_type_closure(&domain).unwrap();
        let objects_by_type = index_objects_by_type(&domain, &problem, &closure);
        let drive = domain
            .actions
            .iter()
            .find(|a| a.name == "drive")
            .expect("drive action present");
        let via_iter: Vec<_> = BindingIter::new(&drive.params, &objects_by_type).collect();
        let via_eager = enumerate_bindings(&drive.params, &objects_by_type);
        assert_eq!(via_iter, via_eager);
        assert_eq!(via_iter.len(), 4); // 2 locations, 2 params -> 2*2
    }

    #[test]
    fn default_grounding_limits_do_not_prune() {
        // `prune_unreachable` must default to `false` so every existing
        // caller/test that asserts exact combinatorial counts is unaffected.
        assert!(!GroundingLimits::default().prune_unreachable);
    }

    #[test]
    fn reachability_pruning_drops_the_unreachable_drive_pairs_on_fixture_a() {
        // Fixture A has 2 locations (l1, l2), so the unpruned "drive"
        // schema's Cartesian product is all 2*2 = 4 ordered pairs, but only
        // `connected(l1,l2)` is ever a fact (static; no action adds
        // `connected`), and only `at(l1)` holds initially — so `drive(l1,l2)`
        // is the only ground instance that can ever fire.
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();

        let naive = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        let drive_naive = naive
            .actions
            .iter()
            .filter(|a| a.name.starts_with("drive("))
            .count();
        assert_eq!(drive_naive, 4);

        let pruned_limits = GroundingLimits {
            prune_unreachable: true,
            ..GroundingLimits::default()
        };
        let pruned = ground(&domain, &problem, &pruned_limits).unwrap();
        let drive_pruned: Vec<&str> = pruned
            .actions
            .iter()
            .filter(|a| a.name.starts_with("drive("))
            .map(|a| a.name.as_str())
            .collect();
        assert_eq!(drive_pruned, vec!["drive(l1,l2)"]);
        // pickup/dropoff are single-parameter over the same 2 objects and
        // both reachable (pickup(l1) from `at(l1)`, dropoff(l2) once
        // `drive(l1,l2)` makes `at(l2)` reachable), so only the quadratic
        // "drive" schema shrinks here — the overall action count still drops.
        assert!(pruned.actions.len() < naive.actions.len());
    }

    const CHAIN_DOMAIN: &str = r#"(define (domain chain-d)
      (:types loc)
      (:predicates
        (at ?l - loc)
        (connected ?a - loc ?b - loc))
      (:action drive
        :parameters (?a - loc ?b - loc)
        :precondition (and (at ?a) (connected ?a ?b))
        :effect (and (not (at ?a)) (at ?b))))"#;

    const CHAIN_PROBLEM: &str = r#"(define (problem chain-p)
      (:domain chain-d)
      (:objects l1 l2 l3 l4 l5 l6 l7 l8 l9 l10 - loc)
      (:init
        (at l1)
        (connected l1 l2) (connected l2 l3) (connected l3 l4) (connected l4 l5)
        (connected l5 l6) (connected l6 l7) (connected l7 l8) (connected l8 l9)
        (connected l9 l10))
      (:goal ())
      (:htn :ordered-subtasks (and (r1 (drive l1 l2)))))"#;

    /// Synthetic domain sized (per the requesting task) at 8-10 objects
    /// across a couple of types worth of structure: 10 `loc` objects and a
    /// single binary-arity action (`drive`) whose only static-fact
    /// precondition (`connected ?a ?b`) holds for just 9 of the 10*10 = 100
    /// ordered pairs the naive Cartesian product enumerates. Real numbers:
    /// naive combinatorial count = 100, delete-relaxation-pruned count = 9 —
    /// exactly the chain edges reachable by repeatedly driving from `l1`.
    #[test]
    fn reachability_pruning_shrinks_ground_action_count_on_a_sparse_chain_domain() {
        let domain = parse_domain(CHAIN_DOMAIN).unwrap();
        let problem = parse_problem(CHAIN_PROBLEM).unwrap();

        let naive = ground(&domain, &problem, &GroundingLimits::default()).unwrap();
        assert_eq!(naive.actions.len(), 10 * 10, "naive combinatorial count");

        let pruned_limits = GroundingLimits {
            prune_unreachable: true,
            ..GroundingLimits::default()
        };
        let pruned = ground(&domain, &problem, &pruned_limits).unwrap();
        assert_eq!(pruned.actions.len(), 9, "pruned reachable count");
        assert!(
            pruned.actions.len() < naive.actions.len(),
            "pruned ({}) must be smaller than naive ({})",
            pruned.actions.len(),
            naive.actions.len()
        );

        let mut pruned_names: Vec<&str> = pruned.actions.iter().map(|a| a.name.as_str()).collect();
        pruned_names.sort();
        assert_eq!(
            pruned_names,
            vec![
                "drive(l1,l2)",
                "drive(l2,l3)",
                "drive(l3,l4)",
                "drive(l4,l5)",
                "drive(l5,l6)",
                "drive(l6,l7)",
                "drive(l7,l8)",
                "drive(l8,l9)",
                "drive(l9,l10)",
            ]
        );
    }

    // -- forall/exists quantifier expansion at grounding time ---------------

    const FORALL_DOMAIN: &str = r#"(define (domain forall-d)
      (:types loc)
      (:predicates (visited ?l - loc))
      (:action finish
        :parameters ()
        :precondition (forall (?l - loc) (visited ?l))
        :effect (and (visited l1))))"#;

    fn forall_problem(objects: &str) -> Problem {
        let src = format!(
            r#"(define (problem forall-p)
              (:domain forall-d)
              (:objects {objects})
              (:init)
              (:goal ())
              (:htn :subtasks (and (r1 (finish)))))"#
        );
        crate::parser::parse_problem(&src).expect("forall problem parses")
    }

    fn ground_finish_action(domain_src: &str, problem: &Problem) -> GroundAction {
        let domain = parse_domain(domain_src).unwrap();
        let ir = ground(&domain, problem, &GroundingLimits::default()).unwrap();
        ir.actions
            .iter()
            .find(|a| a.name == "finish")
            .expect("finish ground instance present")
            .clone()
    }

    #[test]
    fn forall_precondition_grounds_to_conjunction_over_all_objects_of_type() {
        let problem = forall_problem("l1 l2 l3 - loc");
        let finish = ground_finish_action(FORALL_DOMAIN, &problem);
        match &finish.precondition {
            GroundGoal::And(parts) => assert_eq!(parts.len(), 3),
            other => panic!("expected an 'and' ground goal, got {other:?}"),
        }
        // Only 2 of 3 locations visited: the conjunction fails.
        assert!(!action_applicable(
            &finish,
            &facts(&["visited(l1)", "visited(l2)"])
        ));
        // All 3 visited: the conjunction holds.
        assert!(action_applicable(
            &finish,
            &facts(&["visited(l1)", "visited(l2)", "visited(l3)"])
        ));
    }

    const EXISTS_DOMAIN: &str = r#"(define (domain exists-d)
      (:types loc)
      (:predicates (visited ?l - loc))
      (:action finish
        :parameters ()
        :precondition (exists (?l - loc) (visited ?l))
        :effect (and (visited l1))))"#;

    #[test]
    fn exists_precondition_grounds_to_disjunction_over_all_objects_of_type() {
        let problem = forall_problem("l1 l2 l3 - loc");
        let finish = ground_finish_action(EXISTS_DOMAIN, &problem);
        match &finish.precondition {
            GroundGoal::Or(parts) => assert_eq!(parts.len(), 3),
            other => panic!("expected an 'or' ground goal, got {other:?}"),
        }
        // No location visited: the disjunction fails.
        assert!(!action_applicable(&finish, &facts(&[])));
        // Any one object satisfying the body is enough.
        assert!(action_applicable(&finish, &facts(&["visited(l2)"])));
    }

    #[test]
    fn forall_over_type_with_no_objects_is_vacuously_true() {
        // Zero `loc` objects declared: `enumerate_bindings` over zero
        // choices yields zero extended bindings, so the conjunction is
        // `And(vec![])`, vacuously true.
        let problem = forall_problem("");
        let finish = ground_finish_action(FORALL_DOMAIN, &problem);
        assert_eq!(finish.precondition, GroundGoal::And(vec![]));
        assert!(action_applicable(&finish, &facts(&[])));
    }

    #[test]
    fn exists_over_type_with_no_objects_is_vacuously_false() {
        let problem = forall_problem("");
        let finish = ground_finish_action(EXISTS_DOMAIN, &problem);
        assert_eq!(finish.precondition, GroundGoal::Or(vec![]));
        assert!(!action_applicable(&finish, &facts(&[])));
    }

    const NESTED_QUANTIFIER_DOMAIN: &str = r#"(define (domain nested-quantifier-d)
      (:types loc)
      (:predicates (p ?x - loc ?y - loc))
      (:action finish
        :parameters ()
        :precondition (forall (?x - loc) (exists (?y - loc) (p ?x ?y)))
        :effect (and (p l1 l1))))"#;

    #[test]
    fn nested_quantifiers_thread_the_extended_binding_correctly() {
        // (forall (?x) (exists (?y) (p ?x ?y))) over {l1, l2}: satisfied
        // when every ?x has *some* ?y with p(?x,?y) true -- ?x stays fixed
        // per outer conjunct while ?y ranges over the inner disjunction.
        let problem = forall_problem("l1 l2 - loc");
        let finish = ground_finish_action(NESTED_QUANTIFIER_DOMAIN, &problem);
        // p(l1,l2) covers x=l1; p(l2,l1) covers x=l2. Neither x has both
        // witnesses in the same fact, but each x has at least one.
        assert!(action_applicable(
            &finish,
            &facts(&["p(l1,l2)", "p(l2,l1)"])
        ));
        // Only x=l1 has a witness; x=l2 has none -- forall fails.
        assert!(!action_applicable(&finish, &facts(&["p(l1,l2)"])));
    }
}
