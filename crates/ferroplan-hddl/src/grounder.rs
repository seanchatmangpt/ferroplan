//! Grounding: type closure, object indexing, term/atom/goal/effect
//! substitution (including `oneof`/`when` outcome expansion), and full
//! enumeration of ground actions and ground methods.
//!
//! Scope note: this pass enumerates the *full* combinatorial grounding (every
//! typed object substitution), bounded by `GroundingLimits`, rather than
//! pruning via delete-relaxation reachability first. That pruning is a real
//! performance optimization the design spec calls out but is not required
//! for correctness on the fixtures this crate targets — `translate.rs`
//! separately does real reachability analysis (BFS from the initial state)
//! before anything reaches the solver, so unreachable ground actions never
//! produce transitions; they just cost one extra combinatorial-enumeration
//! pass here. Left as a named, honest simplification rather than pretending
//! it doesn't matter at scale.

use crate::ast::*;
use crate::validate;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroundError {
    Validation(validate::ValidationError),
    TypeCycle(String),
    UnboundVariable(String),
    UnsupportedPrecondition(String),
    LimitExceeded(String),
}

impl fmt::Display for GroundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::TypeCycle(t) => write!(f, "type hierarchy cycle involving '{t}'"),
            Self::UnboundVariable(v) => write!(f, "unbound variable '?{v}'"),
            Self::UnsupportedPrecondition(msg) => write!(f, "unsupported construct: {msg}"),
            Self::LimitExceeded(msg) => write!(f, "grounding limit exceeded: {msg}"),
        }
    }
}
impl std::error::Error for GroundError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundingLimits {
    pub max_ground_actions: usize,
    pub max_ground_methods: usize,
}

impl Default for GroundingLimits {
    fn default() -> Self {
        Self {
            max_ground_actions: 10_000,
            max_ground_methods: 10_000,
        }
    }
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundAction {
    pub name: String,
    pub pos_pre: BTreeSet<String>,
    pub neg_pre: BTreeSet<String>,
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
    pub subtasks: Vec<GroundSubtask>,
    pub order: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GroundedIR {
    pub actions: Vec<GroundAction>,
    pub methods: Vec<GroundMethod>,
    pub root_tasks: Vec<String>,
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

fn ancestors_of(type_name: &str, types: &TypeDef) -> Result<Vec<String>, GroundError> {
    let mut chain = vec![type_name.to_owned()];
    let mut seen = BTreeSet::new();
    seen.insert(type_name.to_owned());
    let mut cur = type_name.to_owned();
    while let Some(parent) = types.parent.get(&cur) {
        if !seen.insert(parent.clone()) {
            return Err(GroundError::TypeCycle(type_name.to_owned()));
        }
        chain.push(parent.clone());
        cur = parent.clone();
    }
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
    for (child, parent) in &domain.types.parent {
        all_types.insert(child.clone());
        all_types.insert(parent.clone());
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

fn ground_effect(
    effect: &Effect,
    binding: &BTreeMap<String, String>,
) -> Result<Vec<GroundEffectBranch>, GroundError> {
    match effect {
        Effect::Oneof(branches) => branches
            .iter()
            .map(|b| ground_effect_branch(b, binding))
            .collect(),
        other => Ok(vec![ground_effect_branch(other, binding)?]),
    }
}

fn enumerate_bindings(
    params: &[TypedParam],
    objects_by_type: &BTreeMap<String, Vec<String>>,
) -> Vec<BTreeMap<String, String>> {
    let mut bindings = vec![BTreeMap::new()];
    for p in params {
        let choices = objects_by_type
            .get(&p.type_name)
            .cloned()
            .unwrap_or_default();
        let mut next = Vec::with_capacity(bindings.len() * choices.len().max(1));
        for b in &bindings {
            for c in &choices {
                let mut nb = b.clone();
                nb.insert(p.var.clone(), c.clone());
                next.push(nb);
            }
        }
        bindings = next;
    }
    bindings
}

pub fn ground_actions(
    domain: &Domain,
    objects_by_type: &BTreeMap<String, Vec<String>>,
    limits: &GroundingLimits,
) -> Result<Vec<GroundAction>, GroundError> {
    let mut out = Vec::new();
    for action in &domain.actions {
        for binding in enumerate_bindings(&action.params, objects_by_type) {
            if out.len() >= limits.max_ground_actions {
                return Err(GroundError::LimitExceeded(format!(
                    "max_ground_actions ({}) exceeded",
                    limits.max_ground_actions
                )));
            }
            let mut pos_pre = BTreeSet::new();
            let mut neg_pre = BTreeSet::new();
            flatten_goal(&action.precondition, &binding, &mut pos_pre, &mut neg_pre)?;
            let outcomes = ground_effect(&action.effect, &binding)?;
            let args = action
                .params
                .iter()
                .map(|p| binding[&p.var].clone())
                .collect::<Vec<_>>();
            out.push(GroundAction {
                name: atom_key(&action.name, &args),
                pos_pre,
                neg_pre,
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
    let mut out = Vec::new();
    for method in &domain.methods {
        for binding in enumerate_bindings(&method.params, objects_by_type) {
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
            let args = method
                .params
                .iter()
                .map(|p| binding[&p.var].clone())
                .collect::<Vec<_>>();
            out.push(GroundMethod {
                name: atom_key(&method.name, &args),
                task_name: atom_key(&method.task.name, &task_args),
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

pub fn ground_root_tasks(problem: &Problem) -> Result<Vec<String>, GroundError> {
    problem
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
            Ok(atom_key(&st.task.name, &args))
        })
        .collect()
}

pub fn ground(
    domain: &Domain,
    problem: &Problem,
    limits: &GroundingLimits,
) -> Result<GroundedIR, GroundError> {
    validate::validate_domain(domain).map_err(GroundError::Validation)?;
    validate::validate_problem(domain, problem).map_err(GroundError::Validation)?;
    let closure = build_type_closure(domain)?;
    let objects_by_type = index_objects_by_type(domain, problem, &closure);
    let actions = ground_actions(domain, &objects_by_type, limits)?;
    let methods = ground_methods(domain, &objects_by_type, limits)?;
    let initial_facts = ground_initial_facts(problem)?;
    let root_tasks = ground_root_tasks(problem)?;
    Ok(GroundedIR {
        actions,
        methods,
        root_tasks,
        initial_facts,
        goal: problem.goal.clone(),
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
        assert_eq!(ir.root_tasks, vec!["deliver(l1,l2)".to_owned()]);
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

    #[test]
    fn grounding_limits_are_enforced() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        let closure = build_type_closure(&domain).unwrap();
        let objects_by_type = index_objects_by_type(&domain, &problem, &closure);
        let tight = GroundingLimits {
            max_ground_actions: 1,
            max_ground_methods: 10_000,
        };
        let err = ground_actions(&domain, &objects_by_type, &tight).unwrap_err();
        assert!(matches!(err, GroundError::LimitExceeded(_)));
    }
}
