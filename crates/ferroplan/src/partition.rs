//! Cut the district into blocks. Goal partitioning.
//!
//! v1 draws borders at the finest grain — one subgoal, one block, no
//! exceptions — and leaves the coarsening to the resolver downstream: it
//! merges blocks only when they turn hostile to each other, the same
//! dynamic grain-size control SGPlan runs (docs/sgplan6-spec.md §2,§5). A
//! later phase can draw smarter borders up front, off the goal-interaction
//! graph — guidance variables, METIS min-cut.

use std::collections::BTreeSet;

use crate::hash::{FxHashMap, FxHashSet};
use crate::packed::PackedTask;
use crate::types::NumPre;

/// One block's contract: positive fact ids owed, numeric comparisons owed.
#[derive(Clone)]
pub struct Subgoal {
    pub pos: Vec<u32>,
    pub num: Vec<NumPre>,
}

impl Subgoal {
    pub fn is_empty(&self) -> bool {
        self.pos.is_empty() && self.num.is_empty()
    }
}

/// Finest cut on the map — one block per fact, one block per numeric goal.
/// Nothing shares a border yet.
pub fn partition(task: &PackedTask) -> Vec<Subgoal> {
    let mut groups = Vec::new();
    for &f in &task.goal_pos {
        groups.push(Subgoal {
            pos: vec![f],
            num: vec![],
        });
    }
    for np in &task.goal_num {
        groups.push(Subgoal {
            pos: vec![],
            num: vec![np.clone()],
        });
    }
    // a goal with no items at all (already-true / empty) -> single empty group
    if groups.is_empty() {
        groups.push(Subgoal {
            pos: vec![],
            num: vec![],
        });
    }
    groups
}

/// Fold block `i` into whichever block sits next to it — the coarsening move.
/// Returns the surviving index. A single block on the map has nowhere to
/// fold into and does nothing — the always-terminate promise holds even
/// against misuse; a stripped debug_assert here used to leave a usize
/// underflow trap for release builds to walk into.
pub fn merge_with_neighbor(groups: &mut Vec<Subgoal>, i: usize) -> usize {
    if groups.len() <= 1 {
        return 0;
    }
    let nb = if i + 1 < groups.len() { i + 1 } else { i - 1 };
    merge_at(groups, i, nb)
}

/// Fold two named blocks together — not neighbors by position, but by grudge:
/// the actual conflicting pair, wherever it sits on the map. Returns the
/// surviving index.
pub fn merge_at(groups: &mut Vec<Subgoal>, i: usize, j: usize) -> usize {
    if i == j || i >= groups.len() || j >= groups.len() || groups.len() <= 1 {
        return i.min(groups.len().saturating_sub(1));
    }
    let (lo, hi) = (i.min(j), i.max(j));
    let removed = groups.remove(hi);
    groups[lo].pos.extend(removed.pos);
    groups[lo].num.extend(removed.num);
    lo
}

fn uf_find(uf: &mut [usize], x: usize) -> usize {
    let mut r = x;
    while uf[r] != r {
        r = uf[r];
    }
    let mut c = x;
    while uf[c] != c {
        let p = uf[c];
        uf[c] = r;
        c = p;
    }
    r
}

fn uf_union(uf: &mut [usize], a: usize, b: usize) {
    let (ra, rb) = (uf_find(uf, a), uf_find(uf, b));
    if ra != rb {
        let (lo, hi) = (ra.min(rb), ra.max(rb));
        uf[hi] = lo;
    }
}

/// Draw the opening borders off the goal-interaction graph — the wire between
/// mutex variables. Two goal facts get linked the instant some operator lights
/// up one's variable while cutting the other's power. Whatever stays connected
/// after that becomes one block; numeric goals stand alone, singletons by
/// nature. An empty `groups` map falls back to the finest cut. Either way the
/// grain doesn't matter for correctness — the resolver still coarsens on
/// conflict, same as always.
pub fn interaction_partition(task: &PackedTask, groups: &[Vec<u32>]) -> Vec<Subgoal> {
    if groups.is_empty() || task.goal_pos.is_empty() {
        return partition(task);
    }
    let mut out = interaction_partition_of(task, groups, &task.goal_pos, &FxHashSet::default());
    for np in &task.goal_num {
        out.push(Subgoal {
            pos: vec![],
            num: vec![np.clone()],
        });
    }
    if out.is_empty() {
        out.push(Subgoal {
            pos: vec![],
            num: vec![],
        });
    }
    out
}

/// [`interaction_partition`]'s engine room, stripped down for the
/// partitioned-ESPC circuit (`crate::espc`): components drawn over an explicit
/// subset of positive goals, with certain wires marked off-limits — shared
/// guidance variables that must never carry an edge. A goal fact sitting on
/// one of those still gets its own block; the shared variable just never
/// pulls two blocks together (it's billed separately, priced as a global
/// constraint by the λ schedule, per docs/espc-preferences-spec.md
/// "increment 2"). Numeric goals and the empty-goal fallback stay the
/// caller's problem. Feed it `goals = &task.goal_pos` and no exclusions and
/// it runs byte-for-byte as the old `interaction_partition` did — same
/// component order the classical resolver expects to walk.
pub fn interaction_partition_of(
    task: &PackedTask,
    groups: &[Vec<u32>],
    goals: &[u32],
    excluded_vars: &FxHashSet<usize>,
) -> Vec<Subgoal> {
    // fact id -> variable id (mutex group index); ungrouped facts are unique.
    let mut var_of: FxHashMap<u32, usize> = FxHashMap::default();
    for (gi, g) in groups.iter().enumerate() {
        for &f in g {
            var_of.insert(f, gi);
        }
    }
    let base = groups.len();
    let var = |f: u32| -> usize { var_of.get(&f).copied().unwrap_or(base + f as usize) };

    // each goal fact is a node; map its variable -> node index, EXCEPT excluded
    // (shared/guidance) variables, which must never carry an interaction edge.
    let n = goals.len();
    let mut var_to_goal: FxHashMap<usize, usize> = FxHashMap::default();
    for (gi, &f) in goals.iter().enumerate() {
        let v = var(f);
        if v < base && excluded_vars.contains(&v) {
            continue;
        }
        var_to_goal.entry(v).or_insert(gi);
    }

    let mut uf: Vec<usize> = (0..n).collect();
    for oi in 0..task.n_ops {
        let added: BTreeSet<usize> = task
            .add
            .slice(oi)
            .iter()
            .filter_map(|&f| var_to_goal.get(&var(f)).copied())
            .collect();
        let deleted: BTreeSet<usize> = task
            .del
            .slice(oi)
            .iter()
            .filter_map(|&f| var_to_goal.get(&var(f)).copied())
            .collect();
        for &a in &added {
            for &d in &deleted {
                if a != d {
                    uf_union(&mut uf, a, d);
                }
            }
        }
    }

    let mut comp: FxHashMap<usize, Vec<u32>> = FxHashMap::default();
    for (gi, &f) in goals.iter().enumerate() {
        let r = uf_find(&mut uf, gi);
        comp.entry(r).or_default().push(f);
    }
    comp.into_values()
        .map(|pos| Subgoal { pos, num: vec![] })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ground::{ground, Outcome};
    use crate::parser::{parse_domain, parse_problem};

    // Two goals coupled ONLY through a token mutex variable: `grab` achieves
    // (done1) while deleting (tok-a) — the variable that goal (tok-b) sits on.
    const DOM: &str = "
    (define (domain t) (:requirements :strips)
      (:predicates (done1) (tok-a) (tok-b))
      (:action grab :precondition (tok-a)
        :effect (and (done1) (not (tok-a)) (tok-b)))
      (:action swap :precondition (tok-b)
        :effect (and (not (tok-b)) (tok-a))))";
    const PRB: &str = "(define (problem p) (:domain t)
      (:init (tok-a)) (:goal (and (done1) (tok-b))))";

    fn task_and_ids() -> (crate::packed::PackedTask, u32, u32, u32) {
        let d = parse_domain(DOM).expect("domain parses");
        let p = parse_problem(PRB).expect("problem parses");
        let task = match ground(&d, &p, 1) {
            Outcome::Task(t) => t,
            _ => panic!("grounds to a task"),
        };
        let fid = |name: &str| {
            task.fact_names
                .iter()
                .position(|n| n == name)
                .unwrap_or_else(|| panic!("fact {name} not found in {:?}", task.fact_names))
                as u32
        };
        let (d1, ta, tb) = (fid("(DONE1)"), fid("(TOK-A)"), fid("(TOK-B)"));
        (task, d1, ta, tb)
    }

    #[test]
    fn shared_variable_merges_goals_by_default() {
        let (task, _d1, ta, tb) = task_and_ids();
        let groups = vec![vec![ta, tb]]; // the token mutex variable
                                         // grab adds (done1) [goal 1's var] and deletes (tok-a) [goal 2's var] -> edge.
        let comps = interaction_partition_of(&task, &groups, &task.goal_pos, &FxHashSet::default());
        assert_eq!(comps.len(), 1, "coupled goals merge into one component");
        assert_eq!(comps[0].pos.len(), 2);
    }

    #[test]
    fn excluded_shared_variable_never_merges() {
        let (task, _d1, ta, tb) = task_and_ids();
        let groups = vec![vec![ta, tb]];
        let mut excluded = FxHashSet::default();
        excluded.insert(0usize); // the token variable is a global-constraint var
        let comps = interaction_partition_of(&task, &groups, &task.goal_pos, &excluded);
        assert_eq!(
            comps.len(),
            2,
            "an excluded guidance variable must not be a merge reason"
        );
    }

    #[test]
    fn interaction_partition_matches_of_with_defaults() {
        let (task, _d1, ta, tb) = task_and_ids();
        let groups = vec![vec![ta, tb]];
        let old = interaction_partition(&task, &groups);
        let new = interaction_partition_of(&task, &groups, &task.goal_pos, &FxHashSet::default());
        // no numeric goals in this task, so the wrapper adds nothing on top
        let key = |sg: &Subgoal| {
            let mut v = sg.pos.clone();
            v.sort_unstable();
            v
        };
        let mut a: Vec<_> = old.iter().map(key).collect();
        let mut b: Vec<_> = new.iter().map(key).collect();
        a.sort();
        b.sort();
        assert_eq!(a, b, "wrapper and core must produce identical components");
    }
}
