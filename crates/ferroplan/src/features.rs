//! Kill-switches for the whole engine, wired process-wide, no wire trace.
//!
//! Every feature runs on a *default* setting and answers to two masters: the env
//! var, a shout from the CLI shell, and the in-process override, a whisper for
//! **WASM** runtimes where `std::env::set_var` blows up on contact with
//! `wasm32-unknown-unknown` — and for embedded riders like the `sim_core` game,
//! stowed away inside someone else's process. Env *reads* stay quiet on wasm, so a
//! getter that checks both channels never trips the alarm.
//!
//! The override runs **tri-state** (`Unset` / `On` / `Off`) — `Unset` drops back to
//! default-plus-env, `On`/`Off` are orders, no negotiation. Matters more since
//! `tdemand` went live by default: a WASM caller needs a hard kill, not a suggestion,
//! and a plain bool "override OR env" can't cut that clean. Set the override once,
//! before the first shot fires — before `solve`.
use std::sync::atomic::{AtomicU8, Ordering::Relaxed};

// Tri-state override packed into an AtomicU8.
const UNSET: u8 = 0;
const ON: u8 = 1;
const OFF: u8 = 2;

static TDEMAND: AtomicU8 = AtomicU8::new(UNSET);
static TDECOMP: AtomicU8 = AtomicU8::new(UNSET);
static TCONC: AtomicU8 = AtomicU8::new(UNSET);
static ESCALATE: AtomicU8 = AtomicU8::new(UNSET);
static ESPC: AtomicU8 = AtomicU8::new(UNSET);

/// Slam the overrides in place (e.g. from the WASM `flags` arg). Each bool is a
/// standing order for this solve and every one after — `true` lights the feature,
/// `false` cuts it, no default gets a vote. Nobody inherits the last caller's ghost.
/// To hand control back to default-plus-env, call [`clear_overrides`].
pub fn set_overrides(tdemand: bool, tdecomp: bool, tconc: bool) {
    TDEMAND.store(if tdemand { ON } else { OFF }, Relaxed);
    TDECOMP.store(if tdecomp { ON } else { OFF }, Relaxed);
    TCONC.store(if tconc { ON } else { OFF }, Relaxed);
}

/// In-process cutoff for the escalation ladder (see [`escalate`]) — the WASM /
/// embedded stand-in for `FF_NO_ESCALATE`, since env *writes* die on wasm32.
/// Holds until [`clear_overrides`] wipes it.
pub fn set_escalate_override(on: bool) {
    ESCALATE.store(if on { ON } else { OFF }, Relaxed);
}

/// In-process cutoff for the ESPC penalty loop (see [`espc`]) — the WASM /
/// embedded stand-in for flipping `FF_ESPC` on or off. Holds until
/// [`clear_overrides`] wipes it.
pub fn set_espc_override(on: bool) {
    ESPC.store(if on { ON } else { OFF }, Relaxed);
}

/// Wipe every in-process override back to `Unset` — hand the decision back to
/// default-plus-env, no orders standing.
pub fn clear_overrides() {
    TDEMAND.store(UNSET, Relaxed);
    TDECOMP.store(UNSET, Relaxed);
    TCONC.store(UNSET, Relaxed);
    ESCALATE.store(UNSET, Relaxed);
    ESPC.store(UNSET, Relaxed);
}

#[inline]
fn resolve(state: &AtomicU8, default: bool) -> bool {
    match state.load(Relaxed) {
        ON => true,
        OFF => false,
        _ => default,
    }
}

/// How hard the temporal-demand instinct pushes. Started as a back-alley opt-in,
/// `FF_TDEMAND`, then went legit in v0.2 — **default-on at the `Numeric` tier** —
/// but only the numeric-goal half of the trade. The predicate-threshold half is
/// still bad news: it misreads a `(>= (avail) 1)` guard on a net-zero pool as real
/// demand and serializes work that should run parallel, bleeding makespan on
/// renewable-resource jobs. So the clean win — multi-round *numeric* goals like
/// `steel >= 2`, `grain >= 10`, `coin >= 15` — ships live. The structural half
/// stays under lock, opt-in only.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DemandMode {
    /// Lights out. No demand guidance, no relevance pruning — the engine runs
    /// exactly as it did before v0.2, bit for bit.
    Off,
    /// Standing default since v0.2: demand seeded from NUMERIC goals alone, no
    /// predicate-threshold intel. Goal-relevance pruning rides along (v0.3.0),
    /// backed by an unmasked full pass so nothing slips through; `FF_NOREL` cuts
    /// pruning on its own if you need it gone.
    Numeric,
    /// Full contact (`FF_TDEMAND`, whole-solve): demand also seeded from
    /// predicate-goal thresholds, for the conjunctive/structural jobs that need it.
    /// The escalation ladder retries here automatically after a default-tier miss
    /// (see [`escalate`]) — the flag's real job now is jumping the line, going
    /// straight to this tier first.
    Full,
}

/// Read the room: resolve the active demand tier from override, then env, then
/// default, in that pecking order.
pub fn demand_mode() -> DemandMode {
    match TDEMAND.load(Relaxed) {
        ON => DemandMode::Full,
        OFF => DemandMode::Off,
        _ => {
            if std::env::var("FF_TDEMAND").is_ok() {
                DemandMode::Full
            } else if std::env::var("FF_NO_TDEMAND").is_ok() {
                DemandMode::Off
            } else {
                DemandMode::Numeric
            }
        }
    }
}

/// True if *any* demand seed got built at all — `Numeric` or `Full`, doesn't matter
/// which. Predicate-threshold seeding is locked behind [`demand_mode`] `== Full`
/// separately; goal-relevance pruning tags along on any tier that isn't `Off`
/// (unless `FF_NOREL` says otherwise).
pub fn tdemand() -> bool {
    demand_mode() != DemandMode::Off
}

/// The partition-and-resolve decomposer, temporal path — cut the job into pieces
/// small enough to actually solve. Stays dark until `FF_TDECOMP` says otherwise.
pub fn tdecomp() -> bool {
    resolve(&TDECOMP, std::env::var("FF_TDECOMP").is_ok())
}

/// The fallback ladder in [`crate::temporal::solve`]: default search dies, so try
/// the `Full` demand tier next, then hand the wreckage to the decomposer as a last
/// resort. Each rung fires only after the one above it goes dark — nothing that
/// already solves clean gets touched, the ladder only spends extra cycles chasing
/// down what would otherwise be a dead end. Runs by default; `FF_NO_ESCALATE`
/// (or [`set_escalate_override`]`(false)` in-process) pulls the ladder alone,
/// and `FF_NO_TDEMAND` — the master switch back to the pristine pre-v0.2 path —
/// takes it down too.
pub fn escalate() -> bool {
    resolve(&ESCALATE, std::env::var("FF_NO_ESCALATE").is_err())
}

/// The concurrent-scheduling pass: repack a temporal plan across the domain's
/// actors so the crew clears the job faster, makespan shaved to the bone. See
/// [`crate::tsched`]. Dark until `FF_TCONC` lights it up.
pub fn tconc() -> bool {
    resolve(&TCONC, std::env::var("FF_TCONC").is_ok())
}

/// The ESPC penalty-resolution loop, riding the PDDL3 metric path (see
/// [`crate::espc`]). **Live by default since 0.5** — the outer budget runs off a
/// deterministic eval pool now, `FF_ESPC_EVAL_BUDGET`, so the same run gives the
/// same answer no matter how many threads or which machine. It only wakes when
/// the compiled task carries once-only conditional-achievement deadline pairs —
/// openstacks-shaped work; anything without that shape falls through to the plain
/// metric B&B, and the flag is a confirmed no-op there. `FF_NO_ESPC` pulls the plug
/// (back to pre-0.5 behavior); `FF_ESPC` still answers to its name but does
/// nothing new. In-process trigger: [`set_espc_override`].
pub fn espc() -> bool {
    resolve(&ESPC, std::env::var("FF_NO_ESPC").is_err())
}
