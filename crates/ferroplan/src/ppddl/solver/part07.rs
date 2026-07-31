/// Canonical identity of one transition in the fully expanded PPDDL graph.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProbabilisticTransitionIdentity {
    pub probability: f64,
    pub next_state: usize,
    /// Transition and goal reward evaluated at time zero. The complete model
    /// identity also binds the declared metric and original model inputs, so a
    /// total-time-dependent expression cannot be substituted silently.
    pub reward_at_time_zero: f64,
    pub goal: bool,
}

/// Canonical identity of every applicable stochastic action at one state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProbabilisticActionIdentity {
    pub state: usize,
    pub action: String,
    pub args: Vec<String>,
    pub outcomes: Vec<ProbabilisticTransitionIdentity>,
}

/// Complete explicit reachable-graph projection used by policy receipts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProbabilisticModelIdentity {
    pub objective: ProbabilisticObjective,
    pub horizon: Option<usize>,
    pub discount: f64,
    pub declared_metric: Option<String>,
    pub initial_distribution: Vec<InitialStateProbability>,
    pub states: Vec<ProbabilisticState>,
    pub actions: Vec<ProbabilisticActionIdentity>,
    pub transition_count: usize,
}

/// Compile a PPDDL model and project its complete reachable transition graph.
///
/// This is an identity/reporting surface, not a second solver. It uses the
/// same normalized compiler and explicit graph that the backward-induction and
/// value-iteration oracle use, then emits a deterministically ordered public
/// projection suitable for hashing and independent replay.
pub fn ppddl_model_identity(
    domain_src: &str,
    problem_src: &str,
    options: &ProbabilisticOptions,
) -> Result<ProbabilisticModelIdentity, PpddlError> {
    options.validate()?;
    let model = compile_model(domain_src, problem_src, options)?;
    let mdp = build_mdp(model, options)?;
    let states = project_states(&mdp);
    let initial_distribution = mdp
        .initial
        .iter()
        .map(|entry| InitialStateProbability {
            state: entry.state,
            probability: entry.probability,
            goal: mdp.goal[entry.state],
        })
        .collect();
    let mut actions = Vec::new();
    for (state, state_actions) in mdp.actions.iter().enumerate() {
        for state_action in state_actions {
            let action = &mdp.model.actions[state_action.action];
            let outcomes = state_action
                .transitions
                .iter()
                .map(|transition| {
                    Ok(ProbabilisticTransitionIdentity {
                        probability: transition.probability,
                        next_state: transition.next,
                        reward_at_time_zero: transition_reward_at(&mdp, transition, 0.0)?,
                        goal: mdp.goal[transition.next],
                    })
                })
                .collect::<Result<Vec<_>, PpddlError>>()?;
            actions.push(ProbabilisticActionIdentity {
                state,
                action: action.base_name.clone(),
                args: action.args.clone(),
                outcomes,
            });
        }
    }
    actions.sort_by(|left, right| {
        left.state
            .cmp(&right.state)
            .then_with(|| left.action.cmp(&right.action))
            .then_with(|| left.args.cmp(&right.args))
    });
    Ok(ProbabilisticModelIdentity {
        objective: mdp.model.objective,
        horizon: options.horizon,
        discount: options.discount,
        declared_metric: mdp.model.metric_text.clone(),
        initial_distribution,
        states,
        actions,
        transition_count: mdp.transitions,
    })
}
