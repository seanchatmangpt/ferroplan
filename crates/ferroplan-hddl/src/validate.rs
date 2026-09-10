//! Pre-grounding static checks. Real checks (not a stub) but deliberately
//! bounded — this is not a full HDDL well-formedness verifier. Currently
//! covers:
//!
//! - Every method/subtask task reference resolves to a declared task or
//!   action, and a method's own task call matches that task's declared
//!   arity (the original checks).
//! - Every predicate referenced in an action's precondition or effect was
//!   declared in `:predicates` (`ValidationError::UndefinedPredicate`).
//! - Every type referenced in a typed parameter list (predicates, numeric
//!   fluents, tasks, actions, methods) was declared in `:types`
//!   (`ValidationError::UndefinedType`).
//! - No task, predicate, or type name is declared more than once
//!   (`ValidationError::DuplicateDefinition`).

use crate::ast::{Domain, Effect, GoalDesc, Literal, Problem, TypedParam};
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateKind {
    Task,
    Predicate,
    Type,
}

impl fmt::Display for DuplicateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Task => "task",
            Self::Predicate => "predicate",
            Self::Type => "type",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    UnknownTaskOrAction(String),
    ArityMismatch {
        task: String,
        expected: usize,
        found: usize,
    },
    /// A predicate name appears in an action's `:precondition` or `:effect`
    /// that was never declared in the domain's `:predicates` block.
    UndefinedPredicate(String),
    /// A type name appears in a typed parameter list (a predicate, numeric
    /// fluent, task, action, or method parameter) that was never declared in
    /// the domain's `:types` block, and is not the built-in `object` type.
    UndefinedType(String),
    /// The same task, predicate, or type name is declared more than once.
    DuplicateDefinition {
        kind: DuplicateKind,
        name: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTaskOrAction(name) => {
                write!(f, "'{name}' is neither a declared task nor an action")
            }
            Self::ArityMismatch {
                task,
                expected,
                found,
            } => write!(f, "'{task}' expects {expected} argument(s), found {found}"),
            Self::UndefinedPredicate(name) => {
                write!(
                    f,
                    "predicate '{name}' is used but never declared in :predicates"
                )
            }
            Self::UndefinedType(name) => {
                write!(f, "type '{name}' is used but never declared in :types")
            }
            Self::DuplicateDefinition { kind, name } => {
                write!(f, "{kind} '{name}' is declared more than once")
            }
        }
    }
}
impl std::error::Error for ValidationError {}

fn is_known_task_or_action(domain: &Domain, name: &str) -> bool {
    domain.tasks.iter().any(|t| t.name == name) || domain.actions.iter().any(|a| a.name == name)
}

/// First name that occurs more than once in `names`, in iteration order, or
/// `None` if every name is unique.
fn first_duplicate<'a, I: IntoIterator<Item = &'a str>>(names: I) -> Option<&'a str> {
    let mut seen = BTreeSet::new();
    names.into_iter().find(|&name| !seen.insert(name))
}

/// The full set of type names the domain's `:types` block makes available:
/// every declared child name (`TypeDef::declared`, which — unlike
/// `TypeDef::parent` — includes types whose parent is the implicit `object`),
/// every name used as a parent (`TypeDef::parent`'s values, e.g. a type
/// referenced only as a supertype), and the always-available built-in
/// `object` type itself.
fn declared_type_names(domain: &Domain) -> BTreeSet<&str> {
    let mut names: BTreeSet<&str> = BTreeSet::new();
    names.insert("object");
    for name in &domain.types.declared {
        names.insert(name.as_str());
    }
    for parent in domain.types.parent.values() {
        names.insert(parent.as_str());
    }
    names
}

fn declared_predicate_names(domain: &Domain) -> BTreeSet<&str> {
    domain.predicates.iter().map(|p| p.name.as_str()).collect()
}

/// Every typed parameter list in the domain that can carry a type reference:
/// predicate params, numeric-fluent params, task params, action params, and
/// method params.
fn all_typed_params(domain: &Domain) -> impl Iterator<Item = &TypedParam> {
    domain
        .predicates
        .iter()
        .flat_map(|p| p.params.iter())
        .chain(domain.numeric_fluents.iter().flat_map(|f| f.params.iter()))
        .chain(domain.tasks.iter().flat_map(|t| t.params.iter()))
        .chain(domain.actions.iter().flat_map(|a| a.params.iter()))
        .chain(domain.methods.iter().flat_map(|m| m.params.iter()))
}

fn goal_desc_predicates<'a>(goal: &'a GoalDesc, out: &mut Vec<&'a str>) {
    match goal {
        GoalDesc::Empty => {}
        GoalDesc::Atom(a) => out.push(&a.predicate),
        GoalDesc::Not(g) => goal_desc_predicates(g, out),
        GoalDesc::And(gs) | GoalDesc::Or(gs) => {
            for g in gs {
                goal_desc_predicates(g, out);
            }
        }
        GoalDesc::Imply(a, b) => {
            goal_desc_predicates(a, out);
            goal_desc_predicates(b, out);
        }
    }
}

fn effect_predicates<'a>(effect: &'a Effect, out: &mut Vec<&'a str>) {
    match effect {
        Effect::Empty => {}
        Effect::Literal(Literal::Pos(a)) | Effect::Literal(Literal::Neg(a)) => {
            out.push(&a.predicate)
        }
        Effect::And(es) | Effect::Oneof(es) => {
            for e in es {
                effect_predicates(e, out);
            }
        }
        Effect::When(g, e) => {
            goal_desc_predicates(g, out);
            effect_predicates(e, out);
        }
        // Numeric-fluent references live in a separate namespace
        // (`:predicates`-nested `NumericFluentDecl`, not `PredicateDef`) and
        // are already refused at grounding time by
        // `GroundError::UnsupportedNumericFluent` — not this pass's concern.
        Effect::Increase(_, _) | Effect::Decrease(_, _) => {}
    }
}

fn check_duplicate_definitions(domain: &Domain) -> Result<(), ValidationError> {
    if let Some(name) = first_duplicate(domain.tasks.iter().map(|t| t.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Task,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.predicates.iter().map(|p| p.name.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Predicate,
            name: name.to_owned(),
        });
    }
    if let Some(name) = first_duplicate(domain.types.declared.iter().map(|s| s.as_str())) {
        return Err(ValidationError::DuplicateDefinition {
            kind: DuplicateKind::Type,
            name: name.to_owned(),
        });
    }
    Ok(())
}

fn check_undefined_types(domain: &Domain) -> Result<(), ValidationError> {
    let declared = declared_type_names(domain);
    for param in all_typed_params(domain) {
        if !declared.contains(param.type_name.as_str()) {
            return Err(ValidationError::UndefinedType(param.type_name.clone()));
        }
    }
    Ok(())
}

fn check_undefined_predicates(domain: &Domain) -> Result<(), ValidationError> {
    let declared = declared_predicate_names(domain);
    for action in &domain.actions {
        let mut used = Vec::new();
        goal_desc_predicates(&action.precondition, &mut used);
        effect_predicates(&action.effect, &mut used);
        for name in used {
            if !declared.contains(name) {
                return Err(ValidationError::UndefinedPredicate(name.to_owned()));
            }
        }
    }
    Ok(())
}

pub fn validate_domain(domain: &Domain) -> Result<(), ValidationError> {
    check_duplicate_definitions(domain)?;
    check_undefined_types(domain)?;
    check_undefined_predicates(domain)?;

    for method in &domain.methods {
        let task_def = domain
            .tasks
            .iter()
            .find(|t| t.name == method.task.name)
            .ok_or_else(|| ValidationError::UnknownTaskOrAction(method.task.name.clone()))?;
        if task_def.params.len() != method.task.args.len() {
            return Err(ValidationError::ArityMismatch {
                task: method.task.name.clone(),
                expected: task_def.params.len(),
                found: method.task.args.len(),
            });
        }
        for subtask in &method.network.subtasks {
            if !is_known_task_or_action(domain, &subtask.task.name) {
                return Err(ValidationError::UnknownTaskOrAction(
                    subtask.task.name.clone(),
                ));
            }
        }
    }
    Ok(())
}

pub fn validate_problem(domain: &Domain, problem: &Problem) -> Result<(), ValidationError> {
    for subtask in &problem.htn.subtasks {
        if !is_known_task_or_action(domain, &subtask.task.name) {
            return Err(ValidationError::UnknownTaskOrAction(
                subtask.task.name.clone(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_domain, parse_problem};

    const FIXTURE_A_DOMAIN: &str = include_str!("../fixtures/a/domain.hddl");
    const FIXTURE_A_PROBLEM: &str = include_str!("../fixtures/a/problem.hddl");
    const FIXTURE_B_DOMAIN: &str = include_str!("../fixtures/b/domain.hddl");
    const FIXTURE_C_DOMAIN: &str = include_str!("../fixtures/c/domain.hddl");
    const FIXTURE_D_DOMAIN: &str = include_str!("../fixtures/d/domain.hddl");

    #[test]
    fn fixture_a_validates_clean() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        let problem = parse_problem(FIXTURE_A_PROBLEM).unwrap();
        assert!(validate_domain(&domain).is_ok());
        assert!(validate_problem(&domain, &problem).is_ok());
    }

    #[test]
    fn rejects_undeclared_subtask_reference() {
        let mut domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        domain.methods[0].network.subtasks[0].task.name = "no-such-task".to_owned();
        let err = validate_domain(&domain).unwrap_err();
        assert!(matches!(err, ValidationError::UnknownTaskOrAction(ref n) if n == "no-such-task"));
    }

    // -- undefined predicate references -------------------------------------

    const UNDEFINED_PREDICATE_DOMAIN: &str = r#"
        (define (domain undefined-predicate)
          (:types loc)
          (:predicates
            (at ?l - loc))
          (:action move
            :parameters (?l - loc)
            :precondition (at ?l)
            :effect (and (not (at ?l)) (holding ?l))))
    "#;

    #[test]
    fn rejects_undefined_predicate_in_effect() {
        let domain = parse_domain(UNDEFINED_PREDICATE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::UndefinedPredicate("holding".to_owned())
        );
    }

    const DEFINED_PREDICATE_DOMAIN: &str = r#"
        (define (domain defined-predicate)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (holding ?l - loc))
          (:action move
            :parameters (?l - loc)
            :precondition (at ?l)
            :effect (and (not (at ?l)) (holding ?l))))
    "#;

    #[test]
    fn accepts_action_using_only_declared_predicates() {
        let domain = parse_domain(DEFINED_PREDICATE_DOMAIN).unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    // -- undefined type references -------------------------------------------

    const UNDEFINED_TYPE_DOMAIN: &str = r#"
        (define (domain undefined-type)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (holds ?v - vehicle))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn rejects_undefined_type_in_predicate_params() {
        let domain = parse_domain(UNDEFINED_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(err, ValidationError::UndefinedType("vehicle".to_owned()));
    }

    const DEFINED_TYPE_DOMAIN: &str = r#"
        (define (domain defined-type)
          (:types loc vehicle)
          (:predicates
            (at ?l - loc)
            (holds ?v - vehicle))
          (:action noop
            :parameters ()
            :precondition ()
            :effect ()))
    "#;

    #[test]
    fn accepts_predicate_params_using_only_declared_types() {
        let domain = parse_domain(DEFINED_TYPE_DOMAIN).unwrap();
        assert!(validate_domain(&domain).is_ok());
    }

    /// A type declared with an implicit `object` parent (no trailing
    /// `- type`) must still count as declared — regression guard for the gap
    /// where `TypeDef::parent` alone (which drops object-parented entries)
    /// would have wrongly flagged this as undefined.
    #[test]
    fn accepts_object_parented_type_used_in_action_params() {
        let domain = parse_domain(FIXTURE_A_DOMAIN).unwrap();
        assert!(check_undefined_types(&domain).is_ok());
    }

    // -- duplicate definitions ------------------------------------------------

    const DUPLICATE_TASK_DOMAIN: &str = r#"
        (define (domain duplicate-task)
          (:types loc)
          (:predicates (at ?l - loc))
          (:task deliver :parameters (?l - loc))
          (:task deliver :parameters (?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_task_definition() {
        let domain = parse_domain(DUPLICATE_TASK_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Task,
                name: "deliver".to_owned(),
            }
        );
    }

    const DUPLICATE_PREDICATE_DOMAIN: &str = r#"
        (define (domain duplicate-predicate)
          (:types loc)
          (:predicates
            (at ?l - loc)
            (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_predicate_definition() {
        let domain = parse_domain(DUPLICATE_PREDICATE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Predicate,
                name: "at".to_owned(),
            }
        );
    }

    const DUPLICATE_TYPE_DOMAIN: &str = r#"
        (define (domain duplicate-type)
          (:types loc loc)
          (:predicates (at ?l - loc))
          (:action noop :parameters () :precondition () :effect ()))
    "#;

    #[test]
    fn rejects_duplicate_type_definition() {
        let domain = parse_domain(DUPLICATE_TYPE_DOMAIN).unwrap();
        let err = validate_domain(&domain).unwrap_err();
        assert_eq!(
            err,
            ValidationError::DuplicateDefinition {
                kind: DuplicateKind::Type,
                name: "loc".to_owned(),
            }
        );
    }

    #[test]
    fn accepts_fixtures_b_c_d_with_no_new_false_positives() {
        for (name, src) in [
            ("b", FIXTURE_B_DOMAIN),
            ("c", FIXTURE_C_DOMAIN),
            ("d", FIXTURE_D_DOMAIN),
        ] {
            let domain = parse_domain(src)
                .unwrap_or_else(|e| panic!("fixture {name} domain should parse: {e}"));
            assert!(
                validate_domain(&domain).is_ok(),
                "fixture {name} domain should validate cleanly under the new checks"
            );
        }
    }
}
