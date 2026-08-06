//! `ff` — the terminal you talk to the planner through.
//!
//! Same handshake as Metric-FF's `ff -o domain.pddl -f problem.pddl` — old
//! rig, old wire format, still boots clean. Flag `--json` pulls a structured
//! [`ferroplan::Solution`] instead of scrolling text; `--json-request` takes
//! a whole job — domain, problem, options — sealed in one packet, no
//! back-and-forth over the wire.

use std::io::Read;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use ferroplan::{Decomposition, Mode, Options, Search};
use serde::Deserialize;

/// Field readout of a [`Decomposition`]: the contracts laid end to end, each
/// goal riding its own sub-plan, then the stitched whole run underneath.
fn render_decomposition(d: &Decomposition) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    if !d.solved {
        let _ = writeln!(s, "No plan found.");
        for n in &d.notes {
            let _ = writeln!(s, "note: {n}");
        }
        return s;
    }
    if d.monolithic {
        let _ = writeln!(
            s,
            "Goal not decomposable — solved as 1 monolithic contract.\n"
        );
    } else {
        let _ = writeln!(s, "Decomposed into {} contracts:\n", d.contracts.len());
    }
    for c in &d.contracts {
        let _ = writeln!(
            s,
            "── contract {} @ offset {:.3}  ⟶  {}",
            c.index, c.offset, c.goal
        );
        for st in &c.steps {
            let args = if st.args.is_empty() {
                String::new()
            } else {
                format!(" {}", st.args.join(" "))
            };
            match (st.time, st.duration) {
                (Some(t), Some(dur)) => {
                    let _ = writeln!(s, "   {:.3}: ({}{}) [{:.3}]", t, st.action, args, dur);
                }
                (Some(t), None) => {
                    let _ = writeln!(s, "   {:.3}: ({}{})", t, st.action, args);
                }
                _ => {
                    let _ = writeln!(s, "   ({}{})", st.action, args);
                }
            }
        }
        let _ = writeln!(s, "   [contract makespan {:.3}]", c.makespan);
    }
    if let Some(plan) = &d.plan {
        let _ = writeln!(
            s,
            "\nStitched plan: {} steps, makespan {:.3}",
            plan.length,
            plan.makespan.unwrap_or(0.0)
        );
    }
    for n in &d.notes {
        let _ = writeln!(s, "note: {n}");
    }
    s
}

#[derive(Parser, Debug)]
#[command(
    name = "ff",
    version,
    about = "ferroplan — a data-parallel PDDL planner"
)]
struct Cli {
    /// The domain file. PDDL, the rules of the world.
    #[arg(short = 'o', long = "domain", value_name = "DOMAIN")]
    domain: Option<PathBuf>,

    /// The problem file. PDDL, the job you're actually here for.
    #[arg(short = 'f', long = "problem", value_name = "PROBLEM")]
    problem: Option<PathBuf>,

    /// A JSON job `{domain, problem, options}` from FILE — or `-` to read it
    /// straight off stdin, no file needed.
    #[arg(long, value_name = "FILE")]
    json_request: Option<String>,

    /// Cut the classic FF scroll-text; hand back a structured JSON solution.
    #[arg(long)]
    json: bool,

    /// Planning mode. `auto` reads the problem's shape and routes itself.
    #[arg(long, value_enum, default_value_t = ModeArg::Auto)]
    mode: ModeArg,

    /// Search strategy — the ff / library / --json paths all answer to it.
    #[arg(long, value_enum, default_value_t = SearchArg::Auto)]
    search: SearchArg,

    /// Kill helpful-action pruning. EHC's usual shortcut, off the table.
    #[arg(long = "no-helpful")]
    no_helpful: bool,

    /// Best-first's g weight — how much the path already walked counts.
    #[arg(long, default_value_t = 1.0)]
    weight_g: f64,

    /// Best-first's h weight — how much the heuristic's guess counts.
    #[arg(long, default_value_t = 5.0)]
    weight_h: f64,

    /// Ceiling on states evaluated before the search gives up. Default:
    /// whatever the engine trusts.
    #[arg(long, value_name = "N")]
    max_evaluated: Option<usize>,

    /// PDDL3: settle for a satisficing plan over the hard goals instead of
    /// chasing the optimum.
    #[arg(long)]
    satisfice: bool,

    /// Worker threads on the line. 0 lets the engine pick its own crew.
    #[arg(long, default_value_t = 0)]
    threads: usize,

    /// IPC time-stamped plan format — classic text mode only, no JSON.
    #[arg(long)]
    ipc: bool,

    /// Don't solve — check a supplied plan FILE against the domain/problem
    /// under ferroplan's own semantics. Reads the shape and knows classical
    /// from temporal on sight.
    #[arg(long, value_name = "FILE")]
    validate: Option<PathBuf>,

    /// A temporal goal too big to swallow whole: split it into ordered,
    /// solvable contracts, print the breakdown and the stitched plan
    /// (`--json` for the wire form).
    #[arg(long)]
    decompose: bool,
}

impl Cli {
    fn to_options(&self) -> Options {
        Options {
            mode: self.mode.into(),
            search: self.search.into(),
            helpful_actions: !self.no_helpful,
            weight_g: self.weight_g,
            weight_h: self.weight_h,
            threads: self.threads,
            max_evaluated: self.max_evaluated,
            optimize: !self.satisfice,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ModeArg {
    Auto,
    Ff,
    Partition,
    Pddl3,
    Temporal,
    Portfolio,
}

impl From<ModeArg> for Mode {
    fn from(m: ModeArg) -> Self {
        match m {
            ModeArg::Auto => Mode::Auto,
            ModeArg::Ff => Mode::Ff,
            ModeArg::Portfolio => Mode::Portfolio,
            ModeArg::Partition => Mode::Partition,
            ModeArg::Pddl3 => Mode::Pddl3,
            ModeArg::Temporal => Mode::Temporal,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SearchArg {
    Auto,
    Ehc,
    BestFirst,
    EhcThenBestFirst,
}

impl From<SearchArg> for Search {
    fn from(s: SearchArg) -> Self {
        match s {
            SearchArg::Auto => Search::Auto,
            SearchArg::Ehc => Search::Ehc,
            SearchArg::BestFirst => Search::BestFirst,
            SearchArg::EhcThenBestFirst => Search::EhcThenBestFirst,
        }
    }
}

#[derive(Deserialize)]
struct JobRequest {
    /// The world, in PDDL source.
    domain: String,
    /// The job, in PDDL source.
    problem: String,
    /// Solver options — any subset; whatever's left unsaid falls to default.
    #[serde(default)]
    options: Options,
}

fn read_source(path: &str) -> Result<String> {
    if path == "-" {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        Ok(s)
    } else {
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path))
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // (1) JSON job request: self-contained {domain, problem, options} -> Solution JSON
    if let Some(req_path) = &cli.json_request {
        let raw = read_source(req_path)?;
        let req: JobRequest = serde_json::from_str(&raw).context("parsing JSON job request")?;
        let sol = ferroplan::solve(&req.domain, &req.problem, &req.options)?;
        println!("{}", serde_json::to_string_pretty(&sol)?);
        std::process::exit(if sol.solved { 0 } else { 1 });
    }

    // (2) file-based: -o / -f
    let (domain, problem) = match (&cli.domain, &cli.problem) {
        (Some(d), Some(p)) => (
            std::fs::read_to_string(d).with_context(|| format!("reading {}", d.display()))?,
            std::fs::read_to_string(p).with_context(|| format!("reading {}", p.display()))?,
        ),
        _ => bail!("need both -o <domain> and -f <problem> (or --json-request <file>)"),
    };

    // (2a) validate a supplied plan instead of solving
    if let Some(plan_path) = &cli.validate {
        let plan_src = std::fs::read_to_string(plan_path)
            .with_context(|| format!("reading {}", plan_path.display()))?;
        match ferroplan::plan::validate_plan(&domain, &problem, &plan_src) {
            Ok(ferroplan::plan::Validity::Valid) => {
                println!("Plan valid");
                std::process::exit(0);
            }
            Ok(ferroplan::plan::Validity::Invalid(why)) => {
                println!("Plan invalid: {}", why);
                std::process::exit(1);
            }
            Err(e) => bail!("validate: {}", e),
        }
    }

    let opts = cli.to_options();

    // (2b) decompose a temporal goal into contracts instead of a flat solve
    if cli.decompose {
        let d = ferroplan::decompose(&domain, &problem, &opts)?;
        if cli.json {
            println!("{}", serde_json::to_string_pretty(&d)?);
        } else {
            print!("{}", render_decomposition(&d));
        }
        std::process::exit(if d.solved { 0 } else { 1 });
    }

    if cli.json {
        let sol = ferroplan::solve(&domain, &problem, &opts)?;
        println!("{}", serde_json::to_string_pretty(&sol)?);
        std::process::exit(if sol.solved { 0 } else { 1 });
    }

    // classic text output (drop-in)
    let (text, code) = match cli.mode {
        ModeArg::Ff => ferroplan::run_ff(&domain, &problem, &opts),
        _ => ferroplan::run_planner(&domain, &problem, &opts, cli.ipc),
    };
    print!("{}", text);
    std::process::exit(code);
}
