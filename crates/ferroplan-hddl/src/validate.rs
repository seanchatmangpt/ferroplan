//! Pre-grounding static checks: every method/subtask task reference must
//! resolve to a declared task or action, and a method's own task call must
//! match that task's declared arity. Real checks (not a stub) but
//! deliberately bounded — this is not a full HDDL well-formedness verifier.

use crate::ast::{Domain, Problem};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    UnknownTaskOrAction(String),
    ArityMismatch {
        task: String,
        expected: usize,
        found: usize,
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
        }
    }
}
impl std::error::Error for ValidationError {}

fn is_known_task_or_action(domain: &Domain, name: &str) -> bool {
    domain.tasks.iter().any(|t| t.name == name) || domain.actions.iter().any(|a| a.name == name)
}

pub fn validate_domain(domain: &Domain) -> Result<(), ValidationError> {
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
}
