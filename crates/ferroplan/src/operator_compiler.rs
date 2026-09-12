//! Pure compilation from a generic STRIPS-like operator description into
//! Ferroplan's real universal-planning `Task`/`Transition` structs.
//!
//! This is CONSTRUCT-only: it manufactures planning data and never executes
//! an action or grants authority.

use crate::planning_runtime::{State, Task, Transition};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

const PROBABILITY_ONE_PPM: u32 = 1_000_000;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorEffects {
    #[serde(default)]
    pub add: BTreeSet<String>,
    #[serde(default)]
    pub delete: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSpec {
    pub name: String,
    #[serde(default)]
    pub preconditions: BTreeSet<String>,
    #[serde(default)]
    pub effects: OperatorEffects,
    #[serde(default = "default_cost")]
    pub cost: u64,
}

fn default_cost() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledOperator {
    pub task: Task,
    pub transitions: Vec<Transition>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorCompileError {
    EmptyName,
    ConflictingEffect(String),
    NoApplicableState,
    MissingTargetState {
        from: String,
        facts: BTreeSet<String>,
    },
    AmbiguousTargetState {
        from: String,
        candidates: Vec<String>,
    },
}

impl fmt::Display for OperatorCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "operator name must not be empty"),
            Self::ConflictingEffect(fact) => {
                write!(f, "fact `{fact}` appears in both add and delete effects")
            }
            Self::NoApplicableState => write!(f, "operator has no applicable state"),
            Self::MissingTargetState { from, facts } => write!(
                f,
                "operator applied at state `{from}` but no explicit target state matches facts {facts:?}"
            ),
            Self::AmbiguousTargetState { from, candidates } => write!(
                f,
                "operator applied at state `{from}` but target state is ambiguous: {candidates:?}"
            ),
        }
    }
}

impl std::error::Error for OperatorCompileError {}

/// Compile one generic operator against an explicit admitted state set.
///
/// Preconditions select source states. Add/delete effects are applied to the
/// source fact set and the resulting fact+fluent state must already exist in
/// `states`; compilation never invents world states. One deterministic
/// transition is emitted per applicable state.
pub fn compile_operator(
    spec: &OperatorSpec,
    states: &[State],
) -> Result<CompiledOperator, OperatorCompileError> {
    if spec.name.trim().is_empty() {
        return Err(OperatorCompileError::EmptyName);
    }

    if let Some(fact) = spec.effects.add.intersection(&spec.effects.delete).next() {
        return Err(OperatorCompileError::ConflictingEffect(fact.clone()));
    }

    let task = Task {
        id: spec.name.clone(),
        primitive_action: Some(spec.name.clone()),
        requires: spec.preconditions.clone(),
    };

    let mut transitions = Vec::new();

    for source in states
        .iter()
        .filter(|state| spec.preconditions.is_subset(&state.facts))
    {
        let mut target_facts = source.facts.clone();
        for fact in &spec.effects.delete {
            target_facts.remove(fact);
        }
        target_facts.extend(spec.effects.add.iter().cloned());

        let candidates: Vec<&State> = states
            .iter()
            .filter(|candidate| {
                candidate.facts == target_facts && candidate.fluents == source.fluents
            })
            .collect();

        let target = match candidates.as_slice() {
            [] => {
                return Err(OperatorCompileError::MissingTargetState {
                    from: source.id.clone(),
                    facts: target_facts,
                })
            }
            [target] => *target,
            many => {
                return Err(OperatorCompileError::AmbiguousTargetState {
                    from: source.id.clone(),
                    candidates: many.iter().map(|state| state.id.clone()).collect(),
                })
            }
        };

        transitions.push(Transition {
            action: spec.name.clone(),
            from: source.id.clone(),
            to: target.id.clone(),
            cost: spec.cost,
            duration: 1,
            reward: 0,
            probability_ppm: PROBABILITY_ONE_PPM,
            observation: None,
            requires: spec.preconditions.clone(),
        });
    }

    if transitions.is_empty() {
        return Err(OperatorCompileError::NoApplicableState);
    }

    transitions.sort_by(|left, right| (&left.from, &left.to).cmp(&(&right.from, &right.to)));

    Ok(CompiledOperator { task, transitions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    fn facts(items: &[&str]) -> BTreeSet<String> {
        items.iter().map(|item| (*item).to_string()).collect()
    }

    fn state(id: &str, items: &[&str]) -> State {
        State {
            id: id.to_string(),
            facts: facts(items),
            fluents: BTreeMap::new(),
        }
    }

    #[test]
    fn compiles_operator_to_real_task_and_transition() {
        let states = [state("s0", &["observed"]), state("s1", &["admitted"])];
        let spec = OperatorSpec {
            name: "admit".into(),
            preconditions: facts(&["observed"]),
            effects: OperatorEffects {
                add: facts(&["admitted"]),
                delete: facts(&["observed"]),
            },
            cost: 3,
        };

        let compiled = compile_operator(&spec, &states).expect("operator compiles");
        assert_eq!(compiled.task.primitive_action.as_deref(), Some("admit"));
        assert_eq!(compiled.transitions.len(), 1);
        assert_eq!(compiled.transitions[0].from, "s0");
        assert_eq!(compiled.transitions[0].to, "s1");
        assert_eq!(compiled.transitions[0].cost, 3);
    }

    #[test]
    fn refuses_to_invent_missing_target_state() {
        let states = [state("s0", &["observed"])];
        let spec = OperatorSpec {
            name: "admit".into(),
            preconditions: facts(&["observed"]),
            effects: OperatorEffects {
                add: facts(&["admitted"]),
                delete: facts(&["observed"]),
            },
            cost: 1,
        };

        assert!(matches!(
            compile_operator(&spec, &states),
            Err(OperatorCompileError::MissingTargetState { .. })
        ));
    }

    #[test]
    fn refuses_conflicting_effects() {
        let states = [state("s0", &["x"])];
        let spec = OperatorSpec {
            name: "bad".into(),
            preconditions: facts(&["x"]),
            effects: OperatorEffects {
                add: facts(&["x"]),
                delete: facts(&["x"]),
            },
            cost: 1,
        };

        assert_eq!(
            compile_operator(&spec, &states),
            Err(OperatorCompileError::ConflictingEffect("x".into()))
        );
    }
}
