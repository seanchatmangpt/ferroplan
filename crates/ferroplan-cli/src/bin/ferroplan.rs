//! `ferroplan` — full-planning CLI with a PPDDL command family.

use std::io::BufRead;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use ferroplan::{
    explain_policy, parse_ppddl, plan_full, simulate_ppddl, verify_policy, FullPlanningRequest,
    PolicySession, ProbabilisticObjective, ProbabilisticOptions, ProbabilisticSolution,
    RiskConstraint,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(
    name = "ferroplan",
    version,
    about = "Ferroplan full planning under deterministic and probabilistic semantics"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// PPDDL parsing, synthesis, verification, simulation, explanation, and sessions.
    Ppddl {
        #[command(subcommand)]
        command: PpddlCommand,
    },
}

#[derive(Subcommand, Debug)]
enum PpddlCommand {
    Parse(ModelFiles),
    Solve(SolveArgs),
    Validate(ValidateArgs),
    Simulate(SimulateArgs),
    Explain(ExplainArgs),
    /// Run a persistent PolicySession controlled by JSON-lines commands.
    Session(SessionArgs),
}

#[derive(Args, Debug, Clone)]
struct ModelFiles {
    #[arg(short = 'o', long, value_name = "DOMAIN")]
    domain: PathBuf,
    #[arg(short = 'f', long, value_name = "PROBLEM")]
    problem: PathBuf,
}

impl ModelFiles {
    fn read(&self) -> Result<(String, String)> {
        Ok((read_file(&self.domain)?, read_file(&self.problem)?))
    }
}

#[derive(Args, Debug, Clone)]
struct PolicyOptionsArgs {
    /// Finite decision horizon. Ignored with --infinite.
    #[arg(long, default_value_t = 64)]
    horizon: usize,
    /// Use bounded infinite-horizon value iteration.
    #[arg(long)]
    infinite: bool,
    #[arg(long, default_value_t = 1.0)]
    discount: f64,
    #[arg(long, default_value_t = 1e-10)]
    epsilon: f64,
    #[arg(long, default_value_t = 10_000)]
    max_iterations: usize,
    #[arg(long, value_enum, default_value_t = ObjectiveArg::Auto)]
    objective: ObjectiveArg,
    #[arg(long, default_value_t = 100_000)]
    max_states: usize,
    #[arg(long, default_value_t = 2_000_000)]
    max_transitions: usize,
    #[arg(long, default_value_t = 200_000)]
    max_policy_entries: usize,
    #[arg(long, default_value_t = 0)]
    threads: usize,
}

impl PolicyOptionsArgs {
    fn options(&self) -> ProbabilisticOptions {
        ProbabilisticOptions {
            objective: self.objective.into(),
            horizon: if self.infinite {
                None
            } else {
                Some(self.horizon)
            },
            discount: self.discount,
            epsilon: self.epsilon,
            max_iterations: self.max_iterations,
            max_states: self.max_states,
            max_transitions: self.max_transitions,
            max_policy_entries: self.max_policy_entries,
            threads: self.threads,
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ObjectiveArg {
    Auto,
    MaximizeGoalProbability,
    MinimizeGoalProbability,
    MaximizeExpectedReward,
    MinimizeExpectedReward,
    MaximizeExpectedMetric,
    MinimizeExpectedMetric,
}

impl From<ObjectiveArg> for ProbabilisticObjective {
    fn from(value: ObjectiveArg) -> Self {
        match value {
            ObjectiveArg::Auto => Self::Auto,
            ObjectiveArg::MaximizeGoalProbability => Self::MaximizeGoalProbability,
            ObjectiveArg::MinimizeGoalProbability => Self::MinimizeGoalProbability,
            ObjectiveArg::MaximizeExpectedReward => Self::MaximizeExpectedReward,
            ObjectiveArg::MinimizeExpectedReward => Self::MinimizeExpectedReward,
            ObjectiveArg::MaximizeExpectedMetric => Self::MaximizeExpectedMetric,
            ObjectiveArg::MinimizeExpectedMetric => Self::MinimizeExpectedMetric,
        }
    }
}

#[derive(Args, Debug, Clone, Default)]
struct ConstraintArgs {
    #[arg(long)]
    min_goal_probability: Option<f64>,
    #[arg(long)]
    max_unsafe_probability: Option<f64>,
    #[arg(long)]
    min_expected_reward: Option<f64>,
    #[arg(long)]
    max_expected_cost: Option<f64>,
    /// Exact projected fact label used to classify unsafe states.
    #[arg(long)]
    unsafe_fact: Option<String>,
}

impl ConstraintArgs {
    fn constraints(&self) -> Vec<RiskConstraint> {
        let mut constraints = Vec::new();
        if let Some(value) = self.min_goal_probability {
            constraints.push(RiskConstraint::MinimumGoalProbability(value));
        }
        if let Some(value) = self.max_unsafe_probability {
            constraints.push(RiskConstraint::MaximumUnsafeReachability(value));
        }
        if let Some(value) = self.min_expected_reward {
            constraints.push(RiskConstraint::MinimumExpectedReward(value));
        }
        if let Some(value) = self.max_expected_cost {
            constraints.push(RiskConstraint::MaximumExpectedCost(value));
        }
        constraints
    }
}

#[derive(Args, Debug)]
struct SolveArgs {
    #[command(flatten)]
    model: ModelFiles,
    #[command(flatten)]
    options: PolicyOptionsArgs,
    #[command(flatten)]
    constraints: ConstraintArgs,
}

#[derive(Args, Debug)]
struct ValidateArgs {
    #[command(flatten)]
    model: ModelFiles,
    #[command(flatten)]
    options: PolicyOptionsArgs,
    #[command(flatten)]
    constraints: ConstraintArgs,
    #[arg(long, value_name = "POLICY_JSON")]
    policy: PathBuf,
}

#[derive(Args, Debug)]
struct SimulateArgs {
    #[command(flatten)]
    model: ModelFiles,
    #[command(flatten)]
    options: PolicyOptionsArgs,
    #[arg(long, default_value_t = 1_000)]
    episodes: usize,
    #[arg(long, default_value_t = 1)]
    seed: u64,
}

#[derive(Args, Debug)]
struct ExplainArgs {
    #[arg(long, value_name = "POLICY_JSON")]
    policy: PathBuf,
    #[arg(long)]
    state: usize,
    #[arg(long)]
    remaining: Option<usize>,
}

#[derive(Args, Debug)]
struct SessionArgs {
    #[command(flatten)]
    model: ModelFiles,
    #[command(flatten)]
    options: PolicyOptionsArgs,
    #[command(flatten)]
    constraints: ConstraintArgs,
    /// JSON-lines command script; `-` reads stdin.
    #[arg(long, default_value = "-")]
    script: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum SessionCommand {
    Status,
    Decide,
    AwaitObservation,
    Observe { state: usize },
    Advance { state: usize },
    SetGoal { goal: String },
    SetObjective { objective: ProbabilisticObjective },
    Replan,
    Close,
}

fn read_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

fn read_solution(path: &Path) -> Result<ProbabilisticSolution> {
    serde_json::from_str(&read_file(path)?).with_context(|| format!("parsing {}", path.display()))
}

fn write_json(value: &impl serde::Serialize) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn run_session(args: SessionArgs) -> Result<()> {
    let (domain, problem) = args.model.read()?;
    let constraints = args.constraints.constraints();
    let mut session = PolicySession::new(
        domain,
        problem,
        args.options.options(),
        constraints,
        args.constraints.unsafe_fact,
    )?;
    let reader: Box<dyn BufRead> = if args.script == "-" {
        Box::new(std::io::BufReader::new(std::io::stdin()))
    } else {
        Box::new(std::io::BufReader::new(std::fs::File::open(&args.script)?))
    };
    write_json(&json!({"event": "opened", "status": session.status()}))?;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let command: SessionCommand = serde_json::from_str(&line)
            .with_context(|| format!("parsing session command at line {}", line_number + 1))?;
        let output = match command {
            SessionCommand::Status => json!({"status": session.status()}),
            SessionCommand::Decide => {
                json!({"decision": session.decide()?, "status": session.status()})
            }
            SessionCommand::AwaitObservation => {
                session.mark_awaiting_observation()?;
                json!({"status": session.status()})
            }
            SessionCommand::Observe { state } => {
                session.observe(state)?;
                json!({"status": session.status()})
            }
            SessionCommand::Advance { state } => {
                session.advance(state)?;
                json!({"status": session.status()})
            }
            SessionCommand::SetGoal { goal } => {
                session.set_goal(&goal)?;
                json!({"status": session.status()})
            }
            SessionCommand::SetObjective { objective } => {
                session.set_objective(objective)?;
                json!({"status": session.status()})
            }
            SessionCommand::Replan => {
                session.replan()?;
                json!({"status": session.status()})
            }
            SessionCommand::Close => {
                session.close();
                write_json(&json!({"event": "closed", "status": session.status()}))?;
                break;
            }
        };
        write_json(&output)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Ppddl { command } => match command {
            PpddlCommand::Parse(model) => {
                let (domain, problem) = model.read()?;
                write_json(&parse_ppddl(&domain, &problem))
            }
            PpddlCommand::Solve(args) => {
                let (domain, problem) = args.model.read()?;
                let constraints = args.constraints.constraints();
                let result = plan_full(FullPlanningRequest::Probabilistic {
                    domain,
                    problem,
                    options: args.options.options(),
                    constraints,
                    unsafe_fact: args.constraints.unsafe_fact,
                })?;
                write_json(&result)
            }
            PpddlCommand::Validate(args) => {
                let (domain, problem) = args.model.read()?;
                let solution = read_solution(&args.policy)?;
                let report = verify_policy(
                    &domain,
                    &problem,
                    &args.options.options(),
                    &solution,
                    &args.constraints.constraints(),
                    args.constraints.unsafe_fact.as_deref(),
                )?;
                write_json(&report)?;
                if !report.valid {
                    std::process::exit(1);
                }
                Ok(())
            }
            PpddlCommand::Simulate(args) => {
                let (domain, problem) = args.model.read()?;
                write_json(&simulate_ppddl(
                    &domain,
                    &problem,
                    &args.options.options(),
                    args.episodes,
                    args.seed,
                )?)
            }
            PpddlCommand::Explain(args) => write_json(&explain_policy(
                &read_solution(&args.policy)?,
                args.state,
                args.remaining,
            )?),
            PpddlCommand::Session(args) => run_session(args),
        },
    }
}
