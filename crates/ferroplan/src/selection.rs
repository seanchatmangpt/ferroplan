//! FIELD DISPATCH — the exact preference-subset SELECTION run, PDDL3 metric lane.
//!
//! Ground truth from the wreck (docs/forensics-tpp.md): on zero-action-cost
//! preference terrain, the whole score is decided the instant the end state
//! picks its jointly-satisfiable preference subset. SGPlan5's tpp p05 number
//! is that pick, closed-form, nothing left on the table. h-guided search
//! never sees it — it can't hold goods5 at L2 just so goods6 falls into
//! place downstream. So don't guide. Solve the pick cold, as a small
//! combinatorial kill, then hand the chosen facts to the planner as one hard
//! TARGET (see `pddl3::metric_optimize_closure`).
//!
//! The rig, built from what compile()/grounding already hand us:
//! - One VARIABLE per invariant mutex group any preference disjunct touches
//!   (domain: the facts that show up in some disjunct, plus ⊥ — nothing
//!   chosen), plus a bare boolean var for every disjunct fact standing alone.
//! - A preference reads SATISFIED the moment one DNF disjunct — the
//!   `P3COLLECT` op's non-P3 precondition facts — has every fact lit.
//! - Minimize violated weight. DFS branch-and-bound: variables fall in
//!   descending touched-weight order, values in descending
//!   immediately-satisfied-weight order, branches die the instant
//!   violated-so-far clears best-so-far. A hard node cap keeps worst case
//!   from running forever; whatever's best when the cap hits is what ships
//!   (storage's p08 class runs thousands of instances deep — the cap is
//!   load-bearing, not a nicety).
//!
//! The `bound` that comes back is ADMISSIBLE-OPTIMISTIC — an upper hand, not
//! a guess. Per-fact relaxed reachability (already implied by grounding)
//! ignores the joint resource caps that actually bite (tpp's market supply,
//! storage's crates-need-somewhere-to-sit), and ungrouped complement facts
//! aren't forced to exclude each other. Nothing real can beat that bound —
//! so when `final metric == bound`, that equality alone is the optimality
//! proof, no further argument needed. Preferences riding a numeric
//! precondition or with no groupable shape get counted satisfied (keeps the
//! bound honest) but are never handed to the target — decoration, not signal.

use crate::hash::FxHashMap;
use crate::packed::PackedTask;

pub struct Selection {
    /// The kill list — facts locked in per selected preference index:
    /// `(pref_idx, disjunct facts)`.
    pub chosen: Vec<(usize, Vec<u32>)>,
    /// The upper hand: admissible-optimistic violated-weight bound across the
    /// whole task.
    pub bound: f64,
    /// The DFS ran out of clock and hit its node cap. What's returned is
    /// best-found, not proven optimal — the bound can't be trusted as a
    /// proof anymore. Callers: do not TARGET a capped selection. Confirmed
    /// live: on storage p08's thousand-instance class, targeting a capped
    /// junk pick torches the seed slice for nothing and starves the
    /// tightening loop — 83 collapsing to 104.
    pub capped: bool,
}

/// One demand a disjunct puts on a variable: lock to the fact (`positive`),
/// or steer clear of it — a compiled `(NOT ...)` complement fact, modeled as
/// a ≠-lock on its positive twin's variable. That's the wire that couples
/// `not (stored g1 level3)` back to the stored-level mutex group, and it's
/// what lets the solver find coordinated plays like g5@L2-so-g6-can-match.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Atom {
    var: usize,
    fact: u32,
    positive: bool,
}

struct Pref {
    idx: usize,
    weight: f64,
    disjuncts: Vec<Vec<Atom>>,
}

const NODE_CAP: usize = 200_000;

/// Pull the shape out of the wreckage, model it, solve it cold. `dnf` hands
/// over each preference's disjunct fact sets — the caller's already pulled
/// these for the guidance/seed rig, no rework here. `banned` names facts the
/// repair loop already proved dead: their disjuncts get cut on sight, and a
/// preference left with no surviving disjunct reads VIOLATED — the ban is
/// ground truth, the bound doesn't get to pretend otherwise.
pub fn select(
    task: &PackedTask,
    groups: &[Vec<u32>],
    weights: &[f64],
    dnf: &FxHashMap<usize, Vec<Vec<u32>>>,
    banned: &crate::hash::FxHashSet<u32>,
) -> Option<Selection> {
    // fact → variable id: mutex-group index, or a fresh boolean var per
    // ungrouped fact (allocated below on first sight).
    let mut var_of: FxHashMap<u32, usize> = FxHashMap::default();
    for (gi, g) in groups.iter().enumerate() {
        for &f in g {
            var_of.insert(f, gi);
        }
    }
    let mut n_vars = groups.len();
    // `(NOT <p>)` complement fact → its positive twin, by grounded name.
    let mut twin: FxHashMap<u32, u32> = FxHashMap::default();
    {
        let mut by_name: FxHashMap<&str, u32> = FxHashMap::default();
        for (f, name) in task.fact_names.iter().enumerate() {
            by_name.insert(name.as_str(), f as u32);
        }
        for (f, name) in task.fact_names.iter().enumerate() {
            let up = name.to_ascii_uppercase();
            if let Some(inner) = up.strip_prefix("(NOT ").and_then(|s| s.strip_suffix(')')) {
                if let Some(&pf) = by_name.get(inner) {
                    twin.insert(f as u32, pf);
                } else if let Some(&pf) = by_name.get(inner.to_ascii_lowercase().as_str()) {
                    twin.insert(f as u32, pf);
                }
            }
        }
    }
    let fresh = |f: u32, var_of: &mut FxHashMap<u32, usize>, n_vars: &mut usize| -> usize {
        *var_of.entry(f).or_insert_with(|| {
            let v = *n_vars;
            *n_vars += 1;
            v
        })
    };

    // Model each preference; unmodelable ones count satisfied (admissible),
    // banned-out ones count violated (the ban is ground truth).
    let mut prefs: Vec<Pref> = Vec::new();
    let mut forced_violated = 0.0;
    for (i, &weight) in weights.iter().enumerate() {
        let Some(djs) = dnf.get(&i) else {
            continue;
        };
        let mut disjuncts: Vec<Vec<Atom>> = Vec::new();
        let mut trivially_true = false;
        let mut any_banned = false;
        for facts in djs {
            if facts.is_empty() {
                trivially_true = true;
                break;
            }
            if facts
                .iter()
                .any(|f| banned.contains(f) && !twin.contains_key(f))
            {
                any_banned = true;
                continue;
            }
            let mut req: Vec<Atom> = facts
                .iter()
                .map(|&f| match twin.get(&f) {
                    // A complement fact constrains its positive twin's
                    // variable to NOT take the twin's value.
                    Some(&pf) => Atom {
                        var: fresh(pf, &mut var_of, &mut n_vars),
                        fact: pf,
                        positive: false,
                    },
                    None => Atom {
                        var: fresh(f, &mut var_of, &mut n_vars),
                        fact: f,
                        positive: true,
                    },
                })
                .collect();
            req.sort_unstable();
            req.dedup();
            // Drop internally-inconsistent disjuncts: two different Eq values
            // on one variable, or Eq and Neq of the same fact. An Eq that
            // implies a same-variable Neq subsumes it.
            let ok = req.iter().all(|a| {
                req.iter().all(|b| {
                    a == b
                        || a.var != b.var
                        || (a.positive != b.positive && a.fact != b.fact)
                        || (!a.positive && !b.positive)
                })
            });
            if ok {
                let keep: Vec<bool> = req
                    .iter()
                    .map(|a| {
                        a.positive
                            || !req
                                .iter()
                                .any(|b| b.positive && b.var == a.var && b.fact != a.fact)
                    })
                    .collect();
                let mut it = keep.iter();
                req.retain(|_| *it.next().unwrap());
                disjuncts.push(req);
            }
        }
        if trivially_true {
            continue;
        }
        if disjuncts.is_empty() {
            if any_banned {
                forced_violated += weight; // every route runs through a banned fact
            }
            // else: unsatisfiable-by-structure — counted satisfied (optimistic).
            continue;
        }
        prefs.push(Pref {
            idx: i,
            weight,
            disjuncts,
        });
    }
    if prefs.len() < 2 {
        return None;
    }

    // Variable domains = values a positive atom demands, ⊥ implicit (Neq
    // atoms add no values — ⊥ or any other value satisfies them).
    let mut domain: Vec<Vec<u32>> = vec![Vec::new(); n_vars];
    for p in &prefs {
        for d in &p.disjuncts {
            for a in d {
                if a.positive && !domain[a.var].contains(&a.fact) {
                    domain[a.var].push(a.fact);
                }
            }
        }
    }
    for d in &mut domain {
        d.sort_unstable();
    }

    // Order variables by descending total weight of touching preferences.
    let mut touch_w: Vec<f64> = vec![0.0; n_vars];
    for p in &prefs {
        let mut seen: Vec<usize> = p
            .disjuncts
            .iter()
            .flat_map(|d| d.iter().map(|a| a.var))
            .collect();
        seen.sort_unstable();
        seen.dedup();
        for v in seen {
            touch_w[v] += p.weight;
        }
    }
    let mut order: Vec<usize> = (0..n_vars).filter(|&v| !domain[v].is_empty()).collect();
    order.sort_by(|&a, &b| {
        touch_w[b]
            .partial_cmp(&touch_w[a])
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.cmp(&b))
    });

    // DFS branch-and-bound over assignments. `assign[v]`: None = undecided,
    // Some(0) = ⊥, Some(f+1) = fact f chosen.
    struct Dfs<'a> {
        prefs: &'a [Pref],
        domain: &'a [Vec<u32>],
        order: &'a [usize],
        assign: Vec<Option<u32>>,
        best_cost: f64,
        best_assign: Vec<Option<u32>>,
        nodes: usize,
    }
    impl Dfs<'_> {
        /// Casualties already locked in under the partial assignment on the
        /// table. An atom reads CONTRADICTED when its variable's already been
        /// decided against it: an Eq needs its exact value or it's dead, a
        /// Neq only dies on that one exact value — ⊥ and still-undecided
        /// both let it live.
        fn split(&self) -> (f64, f64) {
            let mut forced = 0.0;
            for p in self.prefs {
                let dead = p.disjuncts.iter().all(|d| {
                    d.iter().any(|a| match self.assign[a.var] {
                        Some(x) if a.positive => x != a.fact + 1,
                        Some(x) => x == a.fact + 1,
                        None => false,
                    })
                });
                if dead {
                    forced += p.weight;
                }
            }
            (forced, 0.0)
        }
        fn go(&mut self, depth: usize) {
            self.nodes += 1;
            if self.nodes > NODE_CAP {
                return;
            }
            let (forced, _) = self.split();
            if forced >= self.best_cost {
                return; // cannot improve
            }
            if depth == self.order.len() {
                self.best_cost = forced;
                self.best_assign = self.assign.clone();
                return;
            }
            let v = self.order[depth];
            // Try each value ordered by immediately-satisfied weight, then ⊥.
            let mut vals: Vec<(f64, u32)> = self.domain[v]
                .iter()
                .map(|&f| {
                    let w: f64 = self
                        .prefs
                        .iter()
                        .filter(|p| {
                            p.disjuncts
                                .iter()
                                .any(|d| d.iter().any(|a| a.positive && a.var == v && a.fact == f))
                        })
                        .map(|p| p.weight)
                        .sum();
                    (w, f)
                })
                .collect();
            vals.sort_by(|a, b| {
                b.0.partial_cmp(&a.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then(a.1.cmp(&b.1))
            });
            for (_, f) in vals {
                self.assign[v] = Some(f + 1);
                self.go(depth + 1);
                if self.nodes > NODE_CAP {
                    return;
                }
            }
            self.assign[v] = Some(0); // ⊥
            self.go(depth + 1);
            self.assign[v] = None;
        }
    }
    let mut dfs = Dfs {
        prefs: &prefs,
        domain: &domain,
        order: &order,
        assign: vec![None; n_vars],
        best_cost: f64::INFINITY,
        best_assign: vec![None; n_vars],
        nodes: 0,
    };
    // Seed the incumbent with the all-⊥ assignment (everything positive
    // violated) so a capped search still returns something.
    dfs.best_cost = prefs.iter().map(|p| p.weight).sum::<f64>() + 1e-9;
    dfs.go(0);

    // Read out the satisfied preferences and their chosen disjuncts.
    let capped = dfs.nodes > NODE_CAP;
    let assign = dfs.best_assign;
    let mut chosen: Vec<(usize, Vec<u32>)> = Vec::new();
    let mut bound = forced_violated;
    for p in &prefs {
        let sat = p.disjuncts.iter().find(|d| {
            d.iter().all(|a| {
                if a.positive {
                    matches!(assign[a.var], Some(x) if x == a.fact + 1)
                } else {
                    !matches!(assign[a.var], Some(x) if x == a.fact + 1)
                }
            })
        });
        match sat {
            // Only positive atoms become TARGET facts — a Neq is enforced by
            // the mutex group once the group's chosen value is achieved (or
            // by simply not achieving the fact).
            Some(d) => chosen.push((
                p.idx,
                d.iter().filter(|a| a.positive).map(|a| a.fact).collect(),
            )),
            None => bound += p.weight,
        }
    }
    if chosen.is_empty() {
        return None;
    }
    Some(Selection {
        chosen,
        bound,
        capped,
    })
}
