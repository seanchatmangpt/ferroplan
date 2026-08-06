//! Sniffing out the fuel gauge nobody wired to the dashboard.
//!
//! Some problems run on a *renewable resource with a hard ceiling* —
//! openstacks' `stacks-avail`, a crew pool, a machine count, a power
//! budget — coded as a one-hot **count chain**: a mutex group of levels
//! `0..=C`, operators that *consume* (kill level `n`, light level `n-1`)
//! and *restore* (kill `n`, light `n+1`). One level lit at a time, nothing
//! else.
//!
//! The delete-relaxed RPG ([`crate::heuristic::relaxed_to`]) is **blind**
//! to it — the `(not (level n))` delete gets dropped, every level lights
//! up at once, "infinite capacity" as far as the heuristic's concerned,
//! zero gradient telling the search to mind the tank. The fix (see
//! [`crate::search::SatGuidance`]) is a penalty scored off the
//! **concrete** state, which still sees which level is actually live.
//! This module hunts the chain and precomputes each member's
//! **occupancy** — how much of the resource is burned at that level,
//! distance from the full/initial end — loaded and ready for that
//! penalty.
//!
//! Detection reads domain-blind (keys off the consume/restore *shape*,
//! never a predicate name) and stays conservative: a group only qualifies
//! if its level-transition operators trace a single clean path through
//! every member, initial state parked at one end — the full-capacity
//! edge. Anything less orderly gets waved off; non-resource domains never
//! feel it.

use crate::hash::{FxHashMap, FxHashSet};
use crate::packed::PackedTask;

/// A renewable counter, flagged and mapped: each member fact id keyed to
/// the occupancy — units burned — when that member is the live level.
pub struct ResourceVar {
    /// `(member fact id, occupancy)`. Occupancy 0 sits at the full/initial level.
    pub members: Vec<(u32, u32)>,
}

impl ResourceVar {
    /// Read the gauge off the concrete state `bits`. Defensive 0 if no
    /// member is lit — a one-hot counter should always show exactly one.
    #[inline]
    pub fn occupancy(&self, bits: &[u64]) -> u32 {
        for &(f, occ) in &self.members {
            if crate::bitset::test(bits, f as usize) {
                return occ;
            }
        }
        0
    }
}

/// Hunt for renewable counter resources buried in the synthesized mutex `groups`.
///
/// `init` — the initial-state bitset — marks the full-capacity level. Only
/// groups whose consume/restore operators trace one clean path over every
/// member, initial level pinned at an endpoint, make it back alive.
pub fn detect_resources(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Vec<ResourceVar> {
    let mut out = Vec::new();
    for g in groups {
        // Need a real counter (capacity >= 2, i.e. >= 3 levels) to be worth it.
        if g.len() < 3 {
            continue;
        }
        let gset: FxHashSet<u32> = g.iter().copied().collect();

        // Level-transition edges: an operator that deletes exactly one member and
        // adds exactly one *other* member moves the resource one level.
        let mut adj: FxHashMap<u32, FxHashSet<u32>> = FxHashMap::default();
        for &f in g {
            adj.entry(f).or_default();
        }
        for oi in 0..task.n_ops {
            let mut dels = task
                .del
                .slice(oi)
                .iter()
                .copied()
                .filter(|f| gset.contains(f));
            let mut adds = task
                .add
                .slice(oi)
                .iter()
                .copied()
                .filter(|f| gset.contains(f));
            let (d0, a0) = (dels.next(), adds.next());
            // exactly one deleted member + exactly one added member
            if let (Some(a), None, Some(b), None) = (d0, dels.next(), a0, adds.next()) {
                if a != b {
                    adj.get_mut(&a).unwrap().insert(b);
                    adj.get_mut(&b).unwrap().insert(a);
                }
            }
        }

        // The transitions must form a simple path over ALL members: every node has
        // degree 1 (the two endpoints) or 2 (interior).
        let mut endpoints = Vec::new();
        let mut shape_ok = true;
        for &f in g {
            match adj[&f].len() {
                1 => endpoints.push(f),
                2 => {}
                _ => {
                    shape_ok = false;
                    break;
                }
            }
        }
        if !shape_ok || endpoints.len() != 2 {
            continue;
        }

        // The initial level must be one endpoint (the full-capacity end), so that
        // occupancy = distance from init grows monotonically as the resource is
        // consumed.
        let start = match endpoints
            .iter()
            .copied()
            .find(|&f| crate::bitset::test(init, f as usize))
        {
            Some(f) => f,
            None => continue,
        };

        // Walk the path from the full end; ordinal = occupancy.
        let mut members = Vec::with_capacity(g.len());
        let mut prev: Option<u32> = None;
        let mut cur = start;
        let mut occ = 0u32;
        loop {
            members.push((cur, occ));
            let next = adj[&cur].iter().copied().find(|&n| Some(n) != prev);
            match next {
                Some(n) => {
                    prev = Some(cur);
                    cur = n;
                    occ += 1;
                }
                None => break,
            }
        }

        // Sanity: a clean path visits every member exactly once.
        if members.len() == g.len() {
            out.push(ResourceVar { members });
        }
    }
    out
}

/// The trip-bound accountant (0.14 ext Phase 11, the semantic-landmark
/// rung). Every resource-linked goal — one whose achievers touch a
/// counter level, transport's `drop` restoring `capacity` — burns one
/// unit of a shared pool per delivery cycle. Clearing `unmet` of them
/// costs at least `⌈unmet / pool⌉` rounds, no way around it. Folded in as
/// a best-first ORDERING term (`FF_RESLM=<w>`) — never a pruning bound.
/// The delete relaxation can't see the counter (levels just pile up);
/// this reads the CONCRETE state instead.
pub struct TripBound {
    /// Goal facts whose achievers reach into a counter level.
    pub goals: Vec<u32>,
    /// Total pool capacity: Σ max occupancy across every counter found.
    pub pool: i64,
}

impl TripBound {
    /// `⌈unmet linked goals / pool⌉`, read off the concrete state `bits`.
    #[inline]
    pub fn trips(&self, bits: &[u64]) -> i64 {
        let unmet = self
            .goals
            .iter()
            .filter(|&&g| !crate::bitset::test(bits, g as usize))
            .count() as i64;
        (unmet + self.pool - 1) / self.pool
    }
}

/// Assemble the trip bound for a task, or come back `None` when no
/// counter resource or linked goal turns up — the term is a no-op by
/// construction in that case, nothing to meter.
pub fn trip_bound(task: &PackedTask, groups: &[Vec<u32>], init: &[u64]) -> Option<TripBound> {
    let res = detect_resources(task, groups, init);
    if res.is_empty() {
        return None;
    }
    let members: FxHashSet<u32> = res
        .iter()
        .flat_map(|r| r.members.iter().map(|&(f, _)| f))
        .collect();
    let pool: i64 = res
        .iter()
        .map(|r| r.members.iter().map(|&(_, o)| o as i64).max().unwrap_or(0))
        .sum();
    if pool == 0 {
        return None;
    }
    let goals: Vec<u32> = task
        .goal_pos
        .iter()
        .copied()
        .filter(|&g| {
            task.add_by_fact.slice(g as usize).iter().any(|&oi| {
                task.add
                    .slice(oi as usize)
                    .iter()
                    .chain(task.del.slice(oi as usize).iter())
                    .any(|f| members.contains(f))
            })
        })
        .collect();
    (!goals.is_empty()).then_some(TripBound { goals, pool })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ground::{ground, Outcome};
    use crate::parser::{parse_domain, parse_problem};

    // A minimal renewable counter (the openstacks stacks-avail mechanic): `avail`
    // is a one-hot level; `consume` lowers it, `restore` raises it.
    const DOM: &str = "(define (domain ctr) (:requirements :typing)
      (:types count)
      (:predicates (avail ?s - count) (nxt ?lo ?hi - count))
      (:action consume :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?b ?a))
        :effect (and (not (avail ?a)) (avail ?b)))
      (:action restore :parameters (?a ?b - count)
        :precondition (and (avail ?a) (nxt ?a ?b))
        :effect (and (not (avail ?a)) (avail ?b))))";
    const PROB: &str = "(define (problem ctr1) (:domain ctr)
      (:objects c0 c1 c2 c3 - count)
      (:init (avail c3) (nxt c0 c1) (nxt c1 c2) (nxt c2 c3))
      (:goal (avail c0)))";

    #[test]
    fn detects_counter_and_orders_occupancy_from_full() {
        let d = parse_domain(DOM).expect("domain");
        let p = parse_problem(PROB).expect("problem");
        let task = match ground(&d, &p, 1) {
            Outcome::Task(t) => t,
            _ => panic!("expected a task"),
        };
        let groups = crate::invariants::synthesize(&d, &task);
        let res = detect_resources(&task, &groups, &task.init_bits);

        assert_eq!(res.len(), 1, "exactly one counter resource detected");
        let r = &res[0];
        assert_eq!(r.members.len(), 4, "4 levels (capacity 3)");
        // The initial/full level (avail c3) is occupancy 0; occupancies are 0..=3.
        assert_eq!(r.occupancy(&task.init_bits), 0, "full level => 0 in use");
        let mut occs: Vec<u32> = r.members.iter().map(|&(_, o)| o).collect();
        occs.sort_unstable();
        assert_eq!(occs, vec![0, 1, 2, 3], "monotone occupancy along the chain");
    }

    // Transport-shaped micro fixture: one truck, capacity 2 (chain c0-c1-c2),
    // three package goals — the trip bound must read ⌈3/2⌉ = 2 at init.
    const TDOM: &str = "(define (domain tinytrans)
      (:requirements :strips :typing)
      (:types loc pkg cap)
      (:predicates (tat ?l - loc) (pat ?p - pkg ?l - loc) (pin ?p - pkg)
                   (cap ?c - cap) (nxt ?a ?b - cap))
      (:action mv :parameters (?a ?b - loc)
        :precondition (tat ?a) :effect (and (not (tat ?a)) (tat ?b)))
      (:action pick :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pat ?p ?l) (nxt ?a ?b) (cap ?b))
        :effect (and (not (pat ?p ?l)) (pin ?p) (cap ?a) (not (cap ?b))))
      (:action drop :parameters (?p - pkg ?l - loc ?a ?b - cap)
        :precondition (and (tat ?l) (pin ?p) (nxt ?a ?b) (cap ?a))
        :effect (and (not (pin ?p)) (pat ?p ?l) (cap ?b) (not (cap ?a)))))";
    // Three locations, so each package's pat/pin mutex group is a STAR
    // (pin borders every location) and is rightly rejected as a counter —
    // only the capacity chain qualifies, as in the real transport corpus.
    const TPROB: &str = "(define (problem tt1) (:domain tinytrans)
      (:objects l1 l2 l3 - loc p1 p2 p3 - pkg c0 c1 c2 - cap)
      (:init (tat l1) (pat p1 l1) (pat p2 l1) (pat p3 l1)
             (cap c2) (nxt c0 c1) (nxt c1 c2))
      (:goal (and (pat p1 l2) (pat p2 l2) (pat p3 l3))))";

    #[test]
    fn trip_bound_reads_demand_over_capacity() {
        let d = parse_domain(TDOM).expect("domain");
        let p = parse_problem(TPROB).expect("problem");
        let task = match ground(&d, &p, 1) {
            Outcome::Task(t) => t,
            _ => panic!("expected a task"),
        };
        let groups = crate::invariants::synthesize(&d, &task);
        let tb = trip_bound(&task, &groups, &task.init_bits)
            .expect("capacity chain + linked goals detected");
        assert_eq!(tb.pool, 2, "one truck, capacity 2");
        assert_eq!(tb.goals.len(), 3, "all three deliveries are linked");
        assert_eq!(tb.trips(&task.init_bits), 2, "ceil(3/2) rounds at init");
    }
}
