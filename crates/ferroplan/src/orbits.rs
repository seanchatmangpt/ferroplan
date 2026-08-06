//! Goal-respecting object-symmetry orbits (0.14 ext Phase 10) — the lever
//! the 0.13 TMS autopsy called for: temporal-machine-shop was flatlining
//! 0/20 because interchangeable pieces, told apart ONLY by which
//! `(baked-structure p q)` goal pair they happen to be serving, turn every
//! subset-assignment of "which identical piece is baking" into its own
//! distinct visited state — the state space bloats on a distinction that
//! never mattered.
//!
//! The cut: find orbits of interchangeable MEMBER UNITS — single objects,
//! or the goal-pair tuples the TMS shape throws up — then canonicalize
//! every visited key under member relabeling. States that differ only by
//! swapping interchangeable members collapse down to one representative.
//! Plans stay concrete on the way out; only the visited space shrinks.
//!
//! Grounded machinery: every fact/op/fluent display that touches an orbit
//! object gets pulled into a FAMILY — displays sharing one (head,
//! literal/slot pattern), held as a dense table over member coordinates.
//! A member permutation σ then walks the whole grounded task by table
//! lookup, so facts COUPLING several members (TMS grounds `(assemble p
//! q)` across every cross-pair combination) permute in step with the
//! per-member ones instead of breaking the orbit apart.
//!
//! Soundness by construction, conservative at every checkpoint:
//!
//! - Candidate members need matching init profiles (statics and fluents
//!   both), no literal appearance in any action schema, and a clean
//!   per-family CLOSURE check: within each equality-pattern class of
//!   member coordinates, cells read uniformly present or uniformly absent
//!   — proof the grounded task is actually closed under every member
//!   transposition. One violation and detection drops out entirely.
//! - Goal facts must sit per-member and be shared across every member of
//!   their orbit (so the goal SET itself stays σ-invariant); numeric
//!   goals over touched fluents, TILs, derived rules, PDDL3 constraints,
//!   and non-total-time metrics all bail out — any one of them could be
//!   telling members apart where nothing else can see it.
//! - The canonical form σ(s) is a pure function of the state, picked by
//!   sorting per-member signatures — determinism holds, t1 ≡ t8 holds.
//!   Any σ chosen this way is sound: canon(s1) = canon(s2) implies s2 =
//!   σ2⁻¹σ1(s1), a genuine automorphism image (ties can MISS a merge,
//!   never fake one).
//! - Wired into the TEMPORAL visited key only (state bits, relevant
//!   fluent values, and the pending-end agenda's op ids all permute
//!   together); the classical paths never see this machinery, by
//!   construction. Callers owe a σ-invariant `forbidden` mask (the CLI
//!   passes none; Session/tresolve skip orbits entirely — a recorded
//!   call, not an oversight).
//!
//! `FF_NO_ORBIT=1` kills detection at the source.

use crate::hash::FxHashMap;
use crate::packed::{PackedTask, State};
use crate::types::{Domain, Expr, Formula, Problem, Sym, Term};
use std::collections::{BTreeMap, BTreeSet};

/// One orbit: `k` bodies, cut from the same mold. Every member carries the
/// identical template — facts, relevant-fluent slots, ops, same family
/// order — that's the sort key σ reads off. Cross-member wiring lives in
/// the families; the orbit itself doesn't see it.
pub struct Orbit {
    /// member -> fact ids, template order. The manifest.
    pub facts: Vec<Vec<u32>>,
    /// member -> relevant-fluent slot indexes (into `rel_fluents`), template order.
    pub fluent_slots: Vec<Vec<usize>>,
    /// member -> op ids, template order — the raw material for agenda signatures.
    pub ops: Vec<Vec<usize>>,
}

/// Every display sharing one (head, pattern) pours into this table. The
/// pattern locks literals and (orbit, obj-within-member) slots; the table
/// holds the concrete id per member-coordinate tuple, row-major.
/// `u32::MAX` marks the empty cell — nobody home.
struct Family {
    /// orbit index, one per slot position.
    axes: Vec<u16>,
    /// member count, one per slot position.
    dims: Vec<u32>,
    table: Vec<u32>,
}

impl Family {
    fn flat(&self, coords: &[u16]) -> usize {
        let mut ix = 0usize;
        for (d, &c) in coords.iter().enumerate() {
            ix = ix * self.dims[d] as usize + c as usize;
        }
        ix
    }
    /// The id this cell becomes under a member permutation
    /// (`sigma[orbit][src] = dst`) — where the shadow falls after σ moves.
    fn map(&self, coords: &[u16], sigma: &[Vec<u16>]) -> u32 {
        let mut ix = 0usize;
        for (d, &c) in coords.iter().enumerate() {
            ix = ix * self.dims[d] as usize + sigma[self.axes[d] as usize][c as usize] as usize;
        }
        self.table[ix]
    }
    /// [`Family::map`], but σ arrives as a closure — skips building the
    /// per-orbit permutation vectors when all the pairwise stabilizer
    /// checks want is a function call.
    fn map_with(&self, coords: &[u16], sigma: impl Fn(u16, u16) -> u16) -> u32 {
        let mut ix = 0usize;
        for (d, &c) in coords.iter().enumerate() {
            ix = ix * self.dims[d] as usize + sigma(self.axes[d], c) as usize;
        }
        self.table[ix]
    }
    /// Closure check under every member swap: cells sharing an equality
    /// pattern — same orbit only, different orbits never touch — must all
    /// be present or all be absent, no stragglers. σ preserves which
    /// same-orbit coordinates match and which don't, so uniform classes
    /// are exactly the signature of "closed under the whole product group."
    fn closed(&self) -> bool {
        let n = self.axes.len(); // ≤ 16, enforced at creation
        let mut coords = vec![0u16; n];
        let mut classes: FxHashMap<u128, bool> = FxHashMap::default();
        loop {
            let mut code: u128 = 0;
            for d in 0..n {
                let mut c = d as u128;
                for e in 0..d {
                    if self.axes[e] == self.axes[d] && coords[e] == coords[d] {
                        c = e as u128;
                        break;
                    }
                }
                code = (code << 8) | c;
            }
            let present = self.table[self.flat(&coords)] != u32::MAX;
            if *classes.entry(code).or_insert(present) != present {
                return false;
            }
            // odometer
            let mut d = n;
            loop {
                if d == 0 {
                    return true;
                }
                d -= 1;
                coords[d] += 1;
                if (coords[d] as u32) < self.dims[d] {
                    break;
                }
                coords[d] = 0;
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Pat {
    Lit(String),
    Slot(u16, u8),
}

/// Family index, still being assembled, for one id space (facts, ops, fluents).
struct FamSet {
    idx: FxHashMap<(String, Vec<Pat>), u32>,
    fams: Vec<Family>,
    /// (id, family, coords) — a trace for every display that got touched.
    touch: Vec<(u32, u32, Vec<u16>)>,
}

/// Hard ceiling on table cells, summed across every family. A grounded
/// space that runs wild — huge same-type object counts crossed with high
/// arity — bails out of detection clean rather than eating all the memory
/// in the room. TMS instance 20 sits nowhere near this wall.
const CELL_CAP: usize = 1 << 22;

impl FamSet {
    fn new() -> Self {
        FamSet {
            idx: FxHashMap::default(),
            fams: Vec::new(),
            touch: Vec::new(),
        }
    }
    /// Book one display into the ledger. `record` is the gate on the touch
    /// (rewrite) list: a STATIC fact still checks into its family table —
    /// closure still has to confirm the automorphism fixes statics — but
    /// its bit never moves, init-constant and σ-invariant, so the per-node
    /// rewrite passes over it. `Ok(true)` = made contact, `Ok(false)` = no
    /// orbit object here, `Err(())` = blew the table budget.
    fn add(
        &mut self,
        disp: &str,
        id: u32,
        owner: &FxHashMap<String, (u16, u16, u8)>,
        k: &[u32],
        cells: &mut usize,
        record: bool,
    ) -> Result<bool, ()> {
        let (head, args) = parse(disp);
        let mut pats = Vec::with_capacity(args.len());
        let mut axes = Vec::new();
        let mut coords: Vec<u16> = Vec::new();
        for a in args {
            match owner.get(a.as_str()) {
                Some(&(o, m, oi)) => {
                    pats.push(Pat::Slot(o, oi));
                    axes.push(o);
                    coords.push(m);
                }
                None => pats.push(Pat::Lit(a)),
            }
        }
        if coords.is_empty() {
            return Ok(false);
        }
        use std::collections::hash_map::Entry;
        let fam = match self.idx.entry((head, pats)) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(v) => {
                if axes.len() > 16 {
                    return Err(()); // closure's class code packs 8 bits/position
                }
                let dims: Vec<u32> = axes.iter().map(|&o| k[o as usize]).collect();
                let mut size = 1usize;
                for &d in &dims {
                    size = size.saturating_mul(d as usize);
                }
                *cells += size;
                if *cells > CELL_CAP {
                    return Err(());
                }
                let f = self.fams.len() as u32;
                v.insert(f);
                self.fams.push(Family {
                    axes,
                    dims,
                    table: vec![u32::MAX; size],
                });
                f
            }
        };
        let f = &mut self.fams[fam as usize];
        let ix = f.flat(&coords);
        f.table[ix] = id;
        if record {
            self.touch.push((id, fam, coords));
        }
        Ok(true)
    }
}

/// `FF_ORBIT_DEBUG=1` cracks the channel open — the whispered reason
/// detection bailed, for probe eyes only. The planner itself stays silent.
fn odbg(msg: impl FnOnce() -> String) {
    if std::env::var("FF_ORBIT_DEBUG").is_ok() {
        eprintln!("orbit: {}", msg());
    }
}

/// Strips a grounded display string down to `(head with NOT folded in, args)`.
fn parse(disp: &str) -> (String, Vec<String>) {
    let inner = disp
        .trim()
        .trim_start_matches("(NOT ")
        .trim_start_matches('(')
        .trim_end_matches(')');
    let mut it = inner.split_whitespace();
    let head = format!(
        "{}{}",
        if disp.trim_start().starts_with("(NOT ") {
            "NOT "
        } else {
            ""
        },
        it.next().unwrap_or("")
    );
    (head, it.map(|s| s.to_string()).collect())
}

pub struct OrbitMap {
    pub orbits: Vec<Orbit>,
    /// op id -> (orbit, member, template) — the paper trail for
    /// per-member agenda signatures.
    pub op_owner: FxHashMap<usize, (usize, usize, usize)>,
    fact_fams: Vec<Family>,
    fact_touch: Vec<(u32, u32, Vec<u16>)>,
    op_fams: Vec<Family>,
    op_touch: FxHashMap<usize, (u32, Vec<u16>)>,
    flu_fams: Vec<Family>,
    flu_touch: Vec<(u32, u32, Vec<u16>)>,
}

/// Sweep the lifted problem for orbits, then burn them into the grounded
/// task. `None` = no exploitable symmetry out there (or `FF_NO_ORBIT=1`
/// killed the scan).
pub fn detect(domain: &Domain, problem: &Problem, task: &PackedTask) -> Option<OrbitMap> {
    if std::env::var("FF_NO_ORBIT").is_ok() {
        return None;
    }
    // Anything that could distinguish members OUTSIDE the grounded
    // fact/op/fluent spaces bails wholesale: scheduled exogenous events,
    // axioms expanded at ground time, trajectory constraints, and metrics
    // beyond total-time (an asymmetric metric could make merged states
    // quality-distinct).
    if !problem.til.is_empty()
        || !domain.derived.is_empty()
        || !domain.constraints.is_empty()
        || !problem.constraints.is_empty()
    {
        odbg(|| "TILs / derived rules / constraints present".into());
        return None;
    }
    if let Some((_, e)) = &problem.metric {
        fn only_total_time(e: &Expr) -> bool {
            match e {
                Expr::Num(_) => true,
                Expr::Fluent(f, args) => args.is_empty() && f.eq_ignore_ascii_case("total-time"),
                Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                    only_total_time(a) && only_total_time(b)
                }
                Expr::Neg(a) => only_total_time(a),
            }
        }
        if !only_total_time(e) {
            odbg(|| "metric reads more than total-time".into());
            return None;
        }
    }

    // ---- 1. lifted-level candidates ------------------------------------
    // Object type map (problem objects + domain constants).
    let mut ty: FxHashMap<&str, &str> = FxHashMap::default();
    for (o, t) in problem.objects.iter().chain(domain.constants.iter()) {
        ty.insert(o.as_str(), t.as_str());
    }
    // Objects an action schema names literally can never be relabeled.
    let mut named: BTreeSet<String> = BTreeSet::new();
    fn note(t: &Term, named: &mut BTreeSet<String>) {
        if let Term::Const(c) = t {
            named.insert(c.to_ascii_uppercase());
        }
    }
    fn walk_f(f: &Formula, named: &mut BTreeSet<String>) {
        match f {
            Formula::Atom(_, args) => args.iter().for_each(|t| note(t, named)),
            Formula::Eq(a, b) => {
                note(a, named);
                note(b, named);
            }
            Formula::And(fs) | Formula::Or(fs) => fs.iter().for_each(|g| walk_f(g, named)),
            Formula::Not(g) | Formula::Pref(_, g) => walk_f(g, named),
            Formula::Exists(_, g) | Formula::Forall(_, g) => walk_f(g, named),
            Formula::Comp(..) | Formula::True | Formula::False => {}
        }
    }
    fn walk_e(e: &crate::types::Effect, named: &mut BTreeSet<String>) {
        use crate::types::Effect as E;
        match e {
            E::Add(_, args) | E::Del(_, args) | E::Num(_, _, args, _) => {
                args.iter().for_each(|t| note(t, named))
            }
            E::And(es) => es.iter().for_each(|x| walk_e(x, named)),
            E::When(f, x) => {
                walk_f(f, named);
                walk_e(x, named);
            }
            E::Forall(_, x) => walk_e(x, named),
        }
    }
    for a in &domain.actions {
        walk_f(&a.precond, &mut named);
        walk_e(&a.effect, &mut named);
    }

    // Init profile per object, ONE pass over init: multiset of (pred,
    // position, other-args-with-self-abstracted) over init atoms and
    // fluents. Statics included — an automorphism must fix them too.
    let mut prof: FxHashMap<String, BTreeMap<String, usize>> = FxHashMap::default();
    for (pred, args) in &problem.init_atoms {
        for (i, a) in args.iter().enumerate() {
            let others: Vec<String> = args
                .iter()
                .enumerate()
                .map(|(j, x)| {
                    if j == i {
                        "<SELF>".into()
                    } else {
                        x.to_ascii_uppercase()
                    }
                })
                .collect();
            *prof
                .entry(a.to_ascii_uppercase())
                .or_default()
                .entry(format!("{pred} {}", others.join(" ")))
                .or_default() += 1;
        }
    }
    for ((f, args), v) in &problem.init_fluents {
        for (i, a) in args.iter().enumerate() {
            let others: Vec<String> = args
                .iter()
                .enumerate()
                .map(|(j, x)| {
                    if j == i {
                        "<SELF>".into()
                    } else {
                        x.to_ascii_uppercase()
                    }
                })
                .collect();
            *prof
                .entry(a.to_ascii_uppercase())
                .or_default()
                .entry(format!("={f} {} {v}", others.join(" ")))
                .or_default() += 1;
        }
    }
    let profile = |o: &str| -> String { format!("{:?}", prof.get(&o.to_ascii_uppercase())) };

    // Conjunctive goal atoms only — any other goal shape bails (a numeric
    // or ADL goal could distinguish members in ways this pass can't see).
    fn collect_goal(f: &Formula, out: &mut Option<Vec<(Sym, Vec<String>)>>) {
        match f {
            Formula::And(fs) => fs.iter().for_each(|g| collect_goal(g, out)),
            Formula::Atom(p, args) => {
                let mut a = Vec::new();
                for t in args {
                    match t {
                        Term::Const(c) => a.push(c.to_ascii_uppercase()),
                        Term::Var(_) => {
                            *out = None;
                            return;
                        }
                    }
                }
                if let Some(v) = out.as_mut() {
                    v.push((p.clone(), a));
                }
            }
            Formula::True => {}
            _ => *out = None,
        }
    }
    let mut collected = Some(Vec::new());
    collect_goal(&problem.goal, &mut collected);
    if collected.is_none() {
        odbg(|| "goal is not a conjunction of ground atoms".into());
    }
    let goal_atoms: Vec<(Sym, Vec<String>)> = collected?;
    let mut goal_count: FxHashMap<&str, usize> = FxHashMap::default();
    for (_, args) in &goal_atoms {
        for a in args {
            *goal_count.entry(a.as_str()).or_default() += 1;
        }
    }

    // Member units. Singletons: objects in NO goal atom. Pairs: the two
    // objects of a binary goal atom, each appearing in exactly that one
    // goal atom. Unit key groups interchangeable candidates.
    #[derive(Clone)]
    struct Unit {
        objs: Vec<String>, // uppercase
        key: String,
    }
    let mut units: Vec<Unit> = Vec::new();
    let mut in_unit: BTreeSet<String> = BTreeSet::new();
    for (pred, args) in &goal_atoms {
        if args.len() == 2
            && args[0] != args[1]
            && args.iter().all(|a| {
                goal_count.get(a.as_str()) == Some(&1)
                    && !named.contains(a)
                    && ty.contains_key(a.as_str())
            })
        {
            let sig = format!(
                "PAIR {pred} {} {} {} {}",
                ty[args[0].as_str()],
                ty[args[1].as_str()],
                profile(&args[0]),
                profile(&args[1])
            );
            units.push(Unit {
                objs: args.clone(),
                key: sig,
            });
            in_unit.extend(args.iter().cloned());
        }
    }
    for (o, t) in problem.objects.iter() {
        let up = o.to_ascii_uppercase();
        if goal_count.contains_key(up.as_str()) || in_unit.contains(&up) || named.contains(&up) {
            continue;
        }
        units.push(Unit {
            key: format!("SOLO {t} {}", profile(o)),
            objs: vec![up.clone()],
        });
        in_unit.insert(up);
    }

    // Group units into candidate orbits (same key, ≥2 members). This runs
    // BEFORE any grounded-task scan so the common no-symmetry case exits
    // on lifted work alone (elevator grounds ~10^6 displays; parsing them
    // to learn "no candidates anyway" cost more than the answer).
    let group = |units: &[Unit]| -> Vec<Vec<usize>> {
        let mut groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (i, u) in units.iter().enumerate() {
            groups.entry(u.key.as_str()).or_default().push(i);
        }
        groups.into_values().filter(|v| v.len() >= 2).collect()
    };
    if group(&units).is_empty() {
        odbg(|| format!("no candidate groups ({} units, all size 1)", units.len()));
        return None;
    }

    // Objects that can matter to a visited key: named by some op, some
    // DYNAMIC fact (added/deleted by an op, conditional effects included),
    // or some relevant fluent. Anything else (sokoban's wall squares,
    // whose only grounded trace is a static IS-NONGOAL) can never vary a
    // member signature — drop its unit before it mints a do-nothing orbit.
    // Static facts also skip the per-node rewrite: with equal init
    // profiles, σ maps a constant-true bit to a constant-true bit.
    let mut dynamic = vec![false; task.n_facts];
    for oi in 0..task.n_ops {
        for &f in task.add.slice(oi) {
            dynamic[f as usize] = true;
        }
        for &f in task.del.slice(oi) {
            dynamic[f as usize] = true;
        }
        for ce in task.cond_effs(oi) {
            for &f in ce.add.iter().chain(ce.del.iter()) {
                dynamic[f as usize] = true;
            }
        }
    }
    // No op-display scan: an object whose signature could ever vary
    // appears in some dynamic fact (a durative op's RUNNING token names
    // every parameter) or relevant fluent; op-args-only objects are
    // signature-empty dead weight. Elevator grounds ~10^6 op displays —
    // skipping them keeps a fruitless detect under 100ms.
    let mut active: BTreeSet<String> = BTreeSet::new();
    for (id, disp) in task.fact_names.iter().enumerate() {
        if dynamic[id] {
            active.extend(parse(disp).1);
        }
    }
    for &fid in task.rel_fluents.iter() {
        active.extend(parse(&task.fluent_names[fid as usize]).1);
    }
    units.retain(|u| u.objs.iter().all(|o| active.contains(o)));

    let candidate_orbits: Vec<Vec<&Unit>> = group(&units)
        .into_iter()
        .map(|v| v.into_iter().map(|i| &units[i]).collect())
        .collect();
    if candidate_orbits.is_empty() || candidate_orbits.len() > u16::MAX as usize {
        odbg(|| "no candidate groups after the active-object filter".into());
        return None;
    }
    odbg(|| {
        let sizes: Vec<usize> = candidate_orbits.iter().map(|m| m.len()).collect();
        format!("candidate orbits {sizes:?}")
    });

    // ---- 2. materialize against the grounded task ----------------------
    // object -> (orbit, member, obj-within-member).
    let mut owner: FxHashMap<String, (u16, u16, u8)> = FxHashMap::default();
    let k: Vec<u32> = candidate_orbits.iter().map(|m| m.len() as u32).collect();
    for (oi, members) in candidate_orbits.iter().enumerate() {
        if members.len() > u16::MAX as usize {
            return None;
        }
        for (mi, u) in members.iter().enumerate() {
            for (xi, o) in u.objs.iter().enumerate() {
                owner.insert(o.clone(), (oi as u16, mi as u16, xi as u8));
            }
        }
    }

    let mut cells = 0usize;
    let mut facts = FamSet::new();
    for (id, disp) in task.fact_names.iter().enumerate() {
        facts
            .add(disp, id as u32, &owner, &k, &mut cells, dynamic[id])
            .ok()?;
    }
    let mut ops = FamSet::new();
    for (id, disp) in task.op_display.iter().enumerate() {
        ops.add(disp, id as u32, &owner, &k, &mut cells, true)
            .ok()?;
    }
    // Fluents: only the RELEVANT ones exist for the visited key; a
    // relevant fluent whose image is missing or irrelevant is a closure
    // hole and bails below.
    let mut flu = FamSet::new();
    for &fid in task.rel_fluents.iter() {
        flu.add(
            &task.fluent_names[fid as usize],
            fid,
            &owner,
            &k,
            &mut cells,
            true,
        )
        .ok()?;
    }
    let check = |fs: &FamSet, what: &str, name: &dyn Fn(u32) -> String| -> bool {
        for (fi, fam) in fs.fams.iter().enumerate() {
            if !fam.closed() {
                odbg(|| {
                    let rep = fs
                        .touch
                        .iter()
                        .find(|(_, f, _)| *f as usize == fi)
                        .map(|(id, _, _)| name(*id))
                        .unwrap_or_default();
                    format!("{what} family not closed, e.g. {rep}")
                });
                return false;
            }
        }
        true
    };
    if !check(&facts, "fact", &|id| task.fact_names[id as usize].clone())
        || !check(&ops, "op", &|id| task.op_display[id as usize].clone())
        || !check(&flu, "fluent", &|id| task.fluent_names[id as usize].clone())
    {
        return None;
    }

    // Per-member signature templates: families whose axes all sit in ONE
    // orbit contribute their diagonal (all-coordinates-equal) cells, one
    // per member, aligned by family order. Closure makes the diagonal
    // uniformly present or absent.
    let n_orbits = candidate_orbits.len();
    let mut orb_facts: Vec<Vec<Vec<u32>>> = (0..n_orbits)
        .map(|o| vec![Vec::new(); k[o] as usize])
        .collect();
    let mut orb_slots: Vec<Vec<Vec<usize>>> = (0..n_orbits)
        .map(|o| vec![Vec::new(); k[o] as usize])
        .collect();
    let mut orb_ops: Vec<Vec<Vec<usize>>> = (0..n_orbits)
        .map(|o| vec![Vec::new(); k[o] as usize])
        .collect();
    let mut op_owner: FxHashMap<usize, (usize, usize, usize)> = FxHashMap::default();
    let diagonal = |fam: &Family, m: u16| -> u32 {
        let coords = vec![m; fam.axes.len()];
        fam.table[fam.flat(&coords)]
    };
    let single_orbit = |fam: &Family| -> Option<usize> {
        let o = *fam.axes.first()?;
        fam.axes.iter().all(|&a| a == o).then_some(o as usize)
    };
    for fam in &facts.fams {
        if let Some(o) = single_orbit(fam) {
            // Static diagonals stay out of the signature: their bits are
            // init-constant, so they can never distinguish members.
            if diagonal(fam, 0) != u32::MAX
                && (0..k[o]).all(|m| dynamic[diagonal(fam, m as u16) as usize])
            {
                for m in 0..k[o] {
                    orb_facts[o][m as usize].push(diagonal(fam, m as u16));
                }
            }
        }
    }
    let slot_of: FxHashMap<u32, usize> = task
        .rel_fluents
        .iter()
        .enumerate()
        .map(|(s, &f)| (f, s))
        .collect();
    for fam in &flu.fams {
        if let Some(o) = single_orbit(fam) {
            if diagonal(fam, 0) != u32::MAX {
                for m in 0..k[o] {
                    orb_slots[o][m as usize].push(slot_of[&diagonal(fam, m as u16)]);
                }
            }
        }
    }
    for fam in &ops.fams {
        if let Some(o) = single_orbit(fam) {
            if diagonal(fam, 0) != u32::MAX {
                for m in 0..k[o] {
                    let op = diagonal(fam, m as u16) as usize;
                    let tj = orb_ops[o][m as usize].len();
                    orb_ops[o][m as usize].push(op);
                    op_owner.insert(op, (o, m as usize, tj));
                }
            }
        }
    }
    let orbits: Vec<Orbit> = (0..n_orbits)
        .map(|o| Orbit {
            facts: std::mem::take(&mut orb_facts[o]),
            fluent_slots: std::mem::take(&mut orb_slots[o]),
            ops: std::mem::take(&mut orb_ops[o]),
        })
        .collect();
    // Nothing state-bearing to permute anywhere -> no reduction possible.
    if orbits
        .iter()
        .all(|o| o.facts[0].is_empty() && o.fluent_slots[0].is_empty() && o.ops[0].is_empty())
    {
        odbg(|| "no orbit has any state-bearing signature".into());
        return None;
    }

    // Goal invariance: every goal fact must be untouched, or a
    // single-orbit diagonal fact whose WHOLE diagonal is in the goal (the
    // goal set is then fixed by every σ). Numeric goals reading a touched
    // fluent bail.
    let goal_set: std::collections::HashSet<u32> = task.goal_pos.iter().copied().collect();
    let fact_of: FxHashMap<u32, (u32, &Vec<u16>)> = facts
        .touch
        .iter()
        .map(|(id, fam, c)| (*id, (*fam, c)))
        .collect();
    for &g in task.goal_pos.iter() {
        if let Some(&(famix, coords)) = fact_of.get(&g) {
            let fam = &facts.fams[famix as usize];
            let (Some(o), true) = (single_orbit(fam), coords.iter().all(|&c| c == coords[0]))
            else {
                odbg(|| format!("cross-member goal fact {}", task.fact_names[g as usize]));
                return None;
            };
            for m in 0..k[o] {
                if !goal_set.contains(&diagonal(fam, m as u16)) {
                    odbg(|| {
                        format!(
                            "goal fact {} not orbit-uniform",
                            task.fact_names[g as usize]
                        )
                    });
                    return None;
                }
            }
        }
    }
    let mut goal_fluents: Vec<u32> = Vec::new();
    for np in task.goal_num.iter() {
        np.lhs.collect_fluents(&mut goal_fluents);
        np.rhs.collect_fluents(&mut goal_fluents);
    }
    let touched_fluents: std::collections::HashSet<u32> =
        flu.touch.iter().map(|(id, _, _)| *id).collect();
    if goal_fluents.iter().any(|f| touched_fluents.contains(f)) {
        odbg(|| "numeric goal reads a touched fluent".into());
        return None;
    }

    Some(OrbitMap {
        orbits,
        op_owner,
        fact_fams: facts.fams,
        fact_touch: facts.touch,
        op_fams: ops.fams,
        op_touch: ops
            .touch
            .into_iter()
            .map(|(id, fam, c)| (id as usize, (fam, c)))
            .collect(),
        flu_fams: flu.fams,
        flu_touch: flu.touch,
    })
}

impl OrbitMap {
    /// Burns off the relabeling — the canonical visited key. Per orbit,
    /// sort members on their (fact-bits, fluent-values, pending-agenda)
    /// signature to fix σ, then run the WHOLE key through the family
    /// tables: per-member and cross-member facts, relevant fluents,
    /// pending-end agenda ops, all of it. Returns (canonical StateKey,
    /// canonical agenda). Sound under any σ you throw at it — the
    /// signature sort just biases π-twin states to land on the same key.
    pub fn canonical_key(
        &self,
        task: &PackedTask,
        state: &State,
        agenda: &[(i64, usize)],
    ) -> (crate::packed::StateKey, Vec<(i64, usize)>) {
        let mut sigma: Vec<Vec<u16>> = Vec::with_capacity(self.orbits.len());
        let mut identity = true;
        for (oi, orbit) in self.orbits.iter().enumerate() {
            let k = orbit.facts.len();
            // signature per member: fact bits (template order), fluent
            // (defined, value) pairs, this member's pending agenda
            // entries (time, template) — src index last as tiebreak.
            #[allow(clippy::type_complexity)]
            let mut sig: Vec<(Vec<bool>, Vec<(bool, i64)>, Vec<(i64, usize)>, usize)> =
                Vec::with_capacity(k);
            for mi in 0..k {
                let fb: Vec<bool> = orbit.facts[mi]
                    .iter()
                    .map(|&f| crate::bitset::test(&state.bits, f as usize))
                    .collect();
                let fvals: Vec<(bool, i64)> = orbit.fluent_slots[mi]
                    .iter()
                    .map(|&slot| {
                        let fid = task.rel_fluents[slot] as usize;
                        (state.fdef[fid], (state.fv[fid] * 1000.0).round() as i64)
                    })
                    .collect();
                let mut pend: Vec<(i64, usize)> = agenda
                    .iter()
                    .filter_map(|&(t, op)| match self.op_owner.get(&op) {
                        Some(&(o, m, tj)) if o == oi && m == mi => Some((t, tj)),
                        _ => None,
                    })
                    .collect();
                pend.sort_unstable();
                sig.push((fb, fvals, pend, mi));
            }
            sig.sort();
            let mut dest = vec![0u16; k];
            for (j, (_, _, _, src)) in sig.iter().enumerate() {
                dest[*src] = j as u16;
                if j != *src {
                    identity = false;
                }
            }
            sigma.push(dest);
        }
        let mut ag: Vec<(i64, usize)> = agenda.to_vec();
        if identity {
            ag.sort_unstable();
            return (task.state_key(state), ag);
        }
        // σ is a bijection on each touched id space (closure-checked at
        // detection), so writing every image exactly once from the
        // PRISTINE source state is a complete, alias-free rewrite.
        let mut bits = state.bits.clone();
        let mut fv = state.fv.clone();
        let mut fdef = state.fdef.clone();
        for (f, fam, coords) in &self.fact_touch {
            let nf = self.fact_fams[*fam as usize].map(coords, &sigma) as usize;
            if crate::bitset::test(&state.bits, *f as usize) {
                crate::bitset::set(&mut bits, nf);
            } else {
                crate::bitset::clear(&mut bits, nf);
            }
        }
        for (fid, fam, coords) in &self.flu_touch {
            let nf = self.flu_fams[*fam as usize].map(coords, &sigma) as usize;
            fv[nf] = state.fv[*fid as usize];
            fdef[nf] = state.fdef[*fid as usize];
        }
        for e in ag.iter_mut() {
            if let Some((fam, coords)) = self.op_touch.get(&e.1) {
                e.1 = self.op_fams[*fam as usize].map(coords, &sigma) as usize;
            }
        }
        ag.sort_unstable();
        let canon = State { bits, fv, fdef };
        (task.state_key(&canon), ag)
    }

    /// Stabilizer classes, the quiet weapon for GENERATION-side symmetry
    /// skipping (0.15 Phase 1): per orbit, cluster members whose pairwise
    /// TRANSPOSITION provably leaves the whole state untouched — every
    /// fact bit, fluent value and definedness, every pending-agenda entry
    /// maps to its twin (cross-member facts included; the per-member
    /// signature alone won't cut it — `(STRUCTURE m1 x)` true against
    /// `(STRUCTURE m2 x)` false is enough to make m1 and m2 strangers even
    /// when their own facts match). Two ops on the same template, same
    /// class of members, spawn π-equivalent successors — so expansion
    /// mints only the first: the duplicate never gets born, no cleanup
    /// after the fact needed. Swap-fixes chains transitively across
    /// pairwise checks ((a c) = (a b)(b c)(a b)), so greedy class
    /// assignment is exact, no approximation.
    pub fn stabilizer_classes(&self, state: &State, agenda: &[(f64, usize)]) -> Vec<Vec<u16>> {
        let mut out = Vec::with_capacity(self.orbits.len());
        for (oi, orbit) in self.orbits.iter().enumerate() {
            let k = orbit.facts.len();
            let mut class: Vec<u16> = (0..k as u16).collect();
            for a in 0..k {
                if class[a] != a as u16 {
                    continue;
                }
                #[allow(clippy::needless_range_loop)]
                for b in (a + 1)..k {
                    if class[b] == b as u16
                        && self.swap_fixes(oi as u16, a as u16, b as u16, state, agenda)
                    {
                        class[b] = a as u16;
                    }
                }
            }
            out.push(class);
        }
        out
    }

    /// Class-canonical GENERATION key for `op`, read through `classes`
    /// (from [`Self::stabilizer_classes`]): two ops keying equal are
    /// shadows of each other under some state-fixing σ — same family,
    /// same stabilizer-class reps per coordinate, same equality pattern
    /// among same-orbit coordinates (so `(REL m1 m2)` never blurs into a
    /// hypothetical `(REL m3 m3)`). Any permutation inside a class factors
    /// into class transpositions, each one state-fixing, so the composite
    /// σ fixes it too. `None` = op touches no orbit — always generate,
    /// nothing to skip.
    pub fn gen_key(&self, op: usize, classes: &[Vec<u16>]) -> Option<(u32, Vec<u16>)> {
        let (fam_ix, coords) = self.op_touch.get(&op)?;
        let fam = &self.op_fams[*fam_ix as usize];
        let mut key = Vec::with_capacity(coords.len() * 2);
        for (d, &c) in coords.iter().enumerate() {
            key.push(classes[fam.axes[d] as usize][c as usize]);
            let mut pat = d as u16;
            #[allow(clippy::needless_range_loop)]
            for e in 0..d {
                if fam.axes[e] == fam.axes[d] && coords[e] == c {
                    pat = e as u16;
                    break;
                }
            }
            key.push(pat);
        }
        Some((*fam_ix, key))
    }

    /// The test itself: does swapping (a b) inside orbit `oi` leave
    /// `state` + `agenda` untouched?
    fn swap_fixes(&self, oi: u16, a: u16, b: u16, state: &State, agenda: &[(f64, usize)]) -> bool {
        let sig = |orb: u16, c: u16| -> u16 {
            if orb == oi {
                if c == a {
                    b
                } else if c == b {
                    a
                } else {
                    c
                }
            } else {
                c
            }
        };
        let touches = |fam_axes: &[u16], coords: &[u16]| -> bool {
            fam_axes
                .iter()
                .zip(coords.iter())
                .any(|(&ax, &c)| ax == oi && (c == a || c == b))
        };
        for (f, fam, coords) in &self.fact_touch {
            let fam = &self.fact_fams[*fam as usize];
            if touches(&fam.axes, coords) {
                let nf = fam.map_with(coords, sig);
                if crate::bitset::test(&state.bits, *f as usize)
                    != crate::bitset::test(&state.bits, nf as usize)
                {
                    return false;
                }
            }
        }
        for (fid, fam, coords) in &self.flu_touch {
            let fam = &self.flu_fams[*fam as usize];
            if touches(&fam.axes, coords) {
                let nf = fam.map_with(coords, sig) as usize;
                let (i, j) = (*fid as usize, nf);
                if state.fdef[i] != state.fdef[j]
                    || (state.fdef[i] && (state.fv[i] - state.fv[j]).abs() > 1e-9)
                {
                    return false;
                }
            }
        }
        // Agenda: every touched pending entry's image must be pending at the
        // SAME time. σ is an involution, so one direction suffices.
        for &(t, op) in agenda {
            if let Some((fam, coords)) = self.op_touch.get(&op) {
                let fam = &self.op_fams[*fam as usize];
                if touches(&fam.axes, coords) {
                    let nop = fam.map_with(coords, sig) as usize;
                    if nop != op && !agenda.iter().any(|&(t2, o2)| o2 == nop && t2 == t) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
