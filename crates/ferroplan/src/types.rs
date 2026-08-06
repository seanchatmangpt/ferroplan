//! Raw signal off the parser's wire: the AST, and the numeric intermediate
//! representation shared downstream by grounding and the heuristic. The
//! *grounded* form is the packed, data-oriented dispatch — that lives in
//! `packed.rs`.

pub type Sym = String;

/// A ghost fluent, 0-ary, reserved. The parser plants it wherever `?duration`
/// shows up inside an expression (PDDL2.1 duration-dependent effects and
/// conditions). Real fluent names never start with `?` — no collision, ever —
/// and the temporal snap compiler swaps in the action's real duration
/// expression before the grid gets grounded.
pub const DURATION_PSEUDO: &str = "?DURATION";

/// Static on the line. A PDDL parse error, tagged with the 1-based source
/// line where the signal dropped.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[error("line {line}: {message}")]
pub struct ParseError {
    pub line: u32,
    pub message: String,
}

impl ParseError {
    pub fn new(line: u32, message: impl Into<String>) -> Self {
        ParseError {
            line,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(Sym),
    Const(Sym),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompOp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Num(f64),
    Fluent(Sym, Vec<Term>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Formula {
    And(Vec<Formula>),
    Or(Vec<Formula>),
    Not(Box<Formula>),
    Atom(Sym, Vec<Term>),
    Comp(CompOp, Expr, Expr),
    /// ADL quantified preconditions, sweeping typed variables like a scanner
    /// over the grid.
    Forall(Vec<(Sym, Sym)>, Box<Formula>),
    Exists(Vec<(Sym, Sym)>, Box<Formula>),
    /// ADL object equality `(= a b)` — identity check, distinct from the
    /// numeric `Comp(Eq, ..)` wire.
    Eq(Term, Term),
    /// PDDL3 `(preference [name] phi)` — a soft ask, not a hard order.
    /// Classical planners flatten it to `True` and walk past; the
    /// metric/optimizer (sgp) is the only one that reads the fine print.
    Pref(Option<Sym>, Box<Formula>),
    True,
    False,
}

/// PDDL3 `:metric` optimization heading — which way the needle should move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricDir {
    Minimize,
    Maximize,
}

/// PDDL3 `(:constraints ...)` trajectory constraint — the standing orders
/// a plan has to answer to over its whole run, not just at the finish line.
/// The six untimed modal operators are enforced on the classical path since
/// 0.7, compiled down into monitor automata by [`crate::constraints`]; the
/// timed ones (`Within`, `HoldDuring`, `HoldAfter`, ...) get parsed off the
/// wire but flagged and refused by name — no clock on this grid yet.
#[derive(Clone, Debug)]
pub enum Constraint {
    And(Vec<Constraint>),
    Forall(Vec<(Sym, Sym)>, Box<Constraint>),
    Pref(Option<Sym>, Box<Constraint>),
    Always(Formula),
    Sometime(Formula),
    AtMostOnce(Formula),
    SometimeAfter(Formula, Formula),
    SometimeBefore(Formula, Formula),
    AtEnd(Formula),
    Within(f64, Formula),
    AlwaysWithin(f64, Formula, Formula),
    HoldDuring(f64, f64, Formula),
    HoldAfter(f64, Formula),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssignOp {
    Assign,
    Increase,
    Decrease,
    ScaleUp,
    ScaleDown,
}

#[derive(Clone, Debug)]
pub enum Effect {
    Add(Sym, Vec<Term>),
    Del(Sym, Vec<Term>),
    Num(AssignOp, Sym, Vec<Term>, Expr),
    And(Vec<Effect>),
    /// ADL conditional effect `(when condition effect)` — the trigger stays
    /// dark until the condition lights up.
    When(Formula, Box<Effect>),
    /// ADL universal effect `(forall (vars) effect)` — one dispatch, fanned
    /// out across every binding.
    Forall(Vec<(Sym, Sym)>, Box<Effect>),
}

#[derive(Clone, Debug)]
pub struct Action {
    pub name: Sym,
    pub params: Vec<(Sym, Sym)>,
    pub precond: Formula,
    pub effect: Effect,
    /// This action is wired into the domain's shared monitor block
    /// ([`Domain::monitors`]) on top of its own effect (0.8 Phase 2,
    /// docs/roadmap-0.8.md) — the grid is watching, not just the action
    /// itself. Set by `constraints::compile` on every action live at gate
    /// time; always `false` on fresh-parsed actions and on synthetic actions
    /// spun up after the gate (P3 bookkeeping, TRAJ-END) — those never got
    /// patched into the wire.
    pub monitored: bool,
}

/// When, on the timeline, a PDDL2.1 durative-action condition or effect
/// keys in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeSpec {
    /// `at start` — fires the instant the dispatch opens.
    Start,
    /// `at end` — fires the instant the dispatch closes.
    End,
    /// `over all` — an invariant the signal must hold clean through, no
    /// static, no drift (conditions only).
    All,
}

/// A durative action's duration constraint — the window it's allowed to run
/// in. A fixed `(= ?duration e)` clamps both bounds to `e`; an inequality
/// leaves the open side `None`. The decision-epoch search always takes the
/// **shortest feasible** duration (the lower bound), and the validator will
/// pass any duration landing in `[min, max]`.
#[derive(Clone, Debug)]
pub struct Duration {
    /// Floor (`>=` / `=`). `None` means no floor at all — only a ceiling given.
    pub min: Option<Expr>,
    /// Ceiling (`<=` / `=`). `None` means no ceiling at all — only a floor given.
    pub max: Option<Expr>,
}

impl Duration {
    /// A fixed duration, `(= ?duration e)` — no slack either side.
    pub fn fixed(e: Expr) -> Self {
        Duration {
            min: Some(e.clone()),
            max: Some(e),
        }
    }
    /// The bound the search actually commits to: the floor (shortest
    /// feasible) when there is one, otherwise the ceiling. `None` only when
    /// the duration is wide open — unconstrained static, no clamp anywhere.
    pub fn chosen(&self) -> Option<&Expr> {
        self.min.as_ref().or(self.max.as_ref())
    }
}

/// A PDDL2.1 `:durative-action` — a dispatch with a clock on it.
#[derive(Clone, Debug)]
pub struct DurativeAction {
    pub name: Sym,
    pub params: Vec<(Sym, Sym)>,
    /// Duration constraint: fixed `(= ?duration e)`, or an inequality range.
    pub duration: Duration,
    pub conditions: Vec<(TimeSpec, Formula)>,
    /// Effects only ever land `at start` / `at end` — `over all` is not a
    /// legal effect, only a legal watch.
    pub effects: Vec<(TimeSpec, Effect)>,
}

#[derive(Clone, Debug)]
pub struct Domain {
    pub name: Sym,
    pub requirements: Vec<Sym>,
    pub types: Vec<Sym>,
    pub type_parent: Vec<(Sym, Sym)>,
    pub constants: Vec<(Sym, Sym)>,
    pub predicates: Vec<(Sym, Vec<Sym>)>,
    pub functions: Vec<(Sym, Vec<Sym>)>,
    pub actions: Vec<Action>,
    pub durative_actions: Vec<DurativeAction>,
    pub constraints: Vec<Constraint>,
    /// `:derived` rules (axioms) — signal computed, never dispatched.
    /// Compiled away before grounding by [`crate::derived::compile`]: static
    /// rules (body over static facts, e.g. `reachable` off the map) resolve
    /// straight into init facts; dynamic non-recursive rules get inlined
    /// into preconditions and goals.
    pub derived: Vec<DerivedRule>,
    /// The shared monitor-transition block (0.8 Phase 2,
    /// docs/roadmap-0.8.md): fully-ground `Effect::When` transitions wired
    /// by `constraints::compile`, applied by every action carrying
    /// [`Action::monitored`]. The grounder builds this block ONCE and
    /// broadcasts it across every monitored op — the transitions are
    /// byte-identical for every binding of every action, so per-op copies
    /// (the monitor-count times ground-action blowup that ran storage
    /// qualpref p07/p08 out to 15 GB, pure blackout) would be dead
    /// duplication. The parser never emits this; it sits empty on every
    /// constraint-free input.
    pub monitors: Vec<Effect>,
}

/// A PDDL `:derived` rule `(:derived (head ?params) body)` — the head
/// predicate's truth is computed off `body`, not carried by any action's
/// effect wire.
#[derive(Clone, Debug)]
pub struct DerivedRule {
    pub head: Sym,
    pub params: Vec<(Sym, Sym)>,
    pub body: Formula,
}

/// A PDDL2.2 timed initial literal: `(at <time> <literal>)` in `:init` — a
/// fact that flips true (`add`) or false (`!add`) at a fixed absolute
/// `time`, on its own clock, no action pulling the trigger. Only meaningful
/// under temporal planning.
#[derive(Clone, Debug)]
pub struct TimedLiteral {
    pub time: f64,
    pub add: bool,
    pub pred: Sym,
    pub args: Vec<Sym>,
}

#[derive(Clone, Debug)]
pub struct Problem {
    pub name: Sym,
    pub domain_name: Sym,
    pub objects: Vec<(Sym, Sym)>,
    pub init_atoms: Vec<(Sym, Vec<Sym>)>,
    pub init_fluents: Vec<((Sym, Vec<Sym>), f64)>,
    /// Timed initial literals (PDDL2.2): exogenous facts scheduled onto the
    /// timeline at fixed absolute times.
    pub til: Vec<TimedLiteral>,
    pub goal: Formula,
    pub constraints: Vec<Constraint>,
    pub metric: Option<(MetricDir, Expr)>,
}

// ---------------------------------------------------------------------------
// Numeric IR over grounded fluent ids (used by packed task + heuristic).
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum NExpr {
    Num(f64),
    Fluent(u32),
    Add(Box<NExpr>, Box<NExpr>),
    Sub(Box<NExpr>, Box<NExpr>),
    Mul(Box<NExpr>, Box<NExpr>),
    Div(Box<NExpr>, Box<NExpr>),
    Neg(Box<NExpr>),
}

impl NExpr {
    pub fn collect_fluents(&self, out: &mut Vec<u32>) {
        match self {
            NExpr::Num(_) => {}
            NExpr::Fluent(i) => out.push(*i),
            NExpr::Neg(a) => a.collect_fluents(out),
            NExpr::Add(a, b) | NExpr::Sub(a, b) | NExpr::Mul(a, b) | NExpr::Div(a, b) => {
                a.collect_fluents(out);
                b.collect_fluents(out);
            }
        }
    }
    pub fn eval(&self, fv: &[f64], def: &[bool]) -> Option<f64> {
        Some(match self {
            NExpr::Num(n) => *n,
            NExpr::Fluent(i) => {
                let i = *i as usize;
                if !def[i] {
                    return None;
                }
                fv[i]
            }
            NExpr::Neg(a) => -a.eval(fv, def)?,
            NExpr::Add(a, b) => a.eval(fv, def)? + b.eval(fv, def)?,
            NExpr::Sub(a, b) => a.eval(fv, def)? - b.eval(fv, def)?,
            NExpr::Mul(a, b) => a.eval(fv, def)? * b.eval(fv, def)?,
            NExpr::Div(a, b) => a.eval(fv, def)? / b.eval(fv, def)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct NumPre {
    pub op: CompOp,
    pub lhs: NExpr,
    pub rhs: NExpr,
}

#[derive(Clone, Debug)]
pub struct NumEff {
    pub op: AssignOp,
    pub target: u32,
    pub value: NExpr,
}

pub fn eval_numpre(np: &NumPre, fv: &[f64], def: &[bool]) -> Option<bool> {
    let l = np.lhs.eval(fv, def)?;
    let r = np.rhs.eval(fv, def)?;
    Some(match np.op {
        CompOp::Lt => l < r,
        CompOp::Le => l <= r,
        CompOp::Eq => (l - r).abs() < 1e-6,
        CompOp::Ge => l >= r,
        CompOp::Gt => l > r,
    })
}
