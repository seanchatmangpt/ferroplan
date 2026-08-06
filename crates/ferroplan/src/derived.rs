//! Derived predicates (`:derived` axioms) — burned off before grounding
//! ever sees them.
//!
//! A `:derived` rule hands a predicate's truth to a formula instead of an
//! action effect. Rather than thread that logic through the grounder, the
//! state, and the heuristic, the common case gets cut at the door as a
//! **preprocessing transform**:
//!
//! * **Static** rules — body reads only *static* facts, predicates no
//!   action ever touches, e.g. `reachable` over a fixed map graph. Their
//!   full ground closure gets computed once, right here, a small datalog
//!   fixpoint, then folded into the problem's init facts. Recursion is
//!   fine — reachability needs it.
//!
//! * **Dynamic** rules — body leans on facts actions change — aren't
//!   carried yet. A clear error comes back instead; they'd need a
//!   per-state fixpoint this module doesn't run.
//!
//! What comes out the other side has no `:derived` rules left standing, so
//! the rest of the pipeline — grounding, search, heuristic — never knows
//! the difference.

use std::collections::{HashMap, HashSet};

use crate::types::{Domain, Effect, Formula, Problem, Sym, Term};

/// Burn the `:derived` rules off, hand back the transformed
/// `(domain, problem)`. Static derived facts land in the problem's init;
/// the domain's `derived` list comes back empty. Throws on any dynamic
/// derived predicate.
pub fn compile(domain: &Domain, problem: &Problem) -> Result<(Domain, Problem), String> {
    if domain.derived.is_empty() {
        return Ok((domain.clone(), problem.clone()));
    }

    // Predicates some action can add/delete — the "dynamic" base predicates.
    let dynamic_base = modified_predicates(domain);

    // A derived predicate is dynamic if its body references a dynamic base
    // predicate or a (transitively) dynamic derived predicate. Fixpoint:
    let mut dynamic_derived: HashSet<Sym> = HashSet::new();
    loop {
        let mut changed = false;
        for r in &domain.derived {
            if dynamic_derived.contains(&r.head) {
                continue;
            }
            let mut refs = HashSet::new();
            collect_pred_refs(&r.body, &mut refs);
            if refs
                .iter()
                .any(|p| dynamic_base.contains(p) || dynamic_derived.contains(p))
            {
                dynamic_derived.insert(r.head.clone());
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    if let Some(r) = domain
        .derived
        .iter()
        .find(|r| dynamic_derived.contains(&r.head))
    {
        return Err(format!(
            "derived predicate '{}' depends on facts that actions change; only \
             static derived predicates (e.g. reachability over a fixed map) are \
             supported so far",
            r.head
        ));
    }

    // All derived predicates are static: compute their ground closure over the
    // init facts (a datalog-style fixpoint; recursion supported).
    let objs = objects_by_type(domain, problem);
    let derived_names: HashSet<Sym> = domain.derived.iter().map(|r| r.head.clone()).collect();
    let mut facts: HashSet<(Sym, Vec<Sym>)> = problem.init_atoms.iter().cloned().collect();
    loop {
        let mut added = false;
        for r in &domain.derived {
            for binding in enumerate(&r.params, &objs) {
                if eval(&r.body, &binding, &facts, &objs)? {
                    let args: Vec<Sym> = r.params.iter().map(|(p, _)| binding[p].clone()).collect();
                    if facts.insert((r.head.clone(), args)) {
                        added = true;
                    }
                }
            }
        }
        if !added {
            break;
        }
    }

    // Emit the newly-derived facts into init.
    let original: HashSet<(Sym, Vec<Sym>)> = problem.init_atoms.iter().cloned().collect();
    let mut prob = problem.clone();
    for f in facts {
        if derived_names.contains(&f.0) && !original.contains(&f) {
            prob.init_atoms.push(f);
        }
    }
    let mut dom = domain.clone();
    dom.derived.clear();
    Ok((dom, prob))
}

/// Every predicate name some action effect ever adds or strikes.
fn modified_predicates(domain: &Domain) -> HashSet<Sym> {
    let mut out = HashSet::new();
    for a in &domain.actions {
        collect_effect_preds(&a.effect, &mut out);
    }
    for da in &domain.durative_actions {
        for (_, e) in &da.effects {
            collect_effect_preds(e, &mut out);
        }
    }
    out
}

fn collect_effect_preds(e: &Effect, out: &mut HashSet<Sym>) {
    match e {
        Effect::Add(p, _) | Effect::Del(p, _) => {
            out.insert(p.clone());
        }
        Effect::And(v) => v.iter().for_each(|x| collect_effect_preds(x, out)),
        Effect::When(_, inner) | Effect::Forall(_, inner) => collect_effect_preds(inner, out),
        Effect::Num(..) => {}
    }
}

fn collect_pred_refs(f: &Formula, out: &mut HashSet<Sym>) {
    match f {
        Formula::Atom(p, _) => {
            out.insert(p.clone());
        }
        Formula::And(v) | Formula::Or(v) => v.iter().for_each(|x| collect_pred_refs(x, out)),
        Formula::Not(a) | Formula::Forall(_, a) | Formula::Exists(_, a) | Formula::Pref(_, a) => {
            collect_pred_refs(a, out)
        }
        Formula::Comp(..) | Formula::Eq(..) | Formula::True | Formula::False => {}
    }
}

/// For each type, every object — problem objects plus domain constants —
/// that answers to it or one of its subtypes. `OBJECT` / untyped catches
/// everything.
fn objects_by_type(domain: &Domain, problem: &Problem) -> HashMap<Sym, Vec<Sym>> {
    let all: Vec<(Sym, Sym)> = domain
        .constants
        .iter()
        .chain(problem.objects.iter())
        .cloned()
        .collect();
    let parent: HashMap<Sym, Sym> = domain.type_parent.iter().cloned().collect();
    let is_a = |ot: &str, target: &str| -> bool {
        if target.is_empty() || target == "OBJECT" {
            return true;
        }
        // Hop-bounded like ground::objects_by_type's walk: a cyclic
        // hierarchy (parser-rejected, but Domain fields are public)
        // degrades to "not a subtype", never a hang.
        let mut cur = ot.to_string();
        let mut hops = 0usize;
        loop {
            if cur == target {
                return true;
            }
            match parent.get(&cur) {
                Some(p) => cur = p.clone(),
                None => return false,
            }
            hops += 1;
            if hops > parent.len() {
                return false;
            }
        }
    };
    // every type we might enumerate over: declared types, OBJECT, and the types
    // objects actually have.
    let mut types: HashSet<Sym> = domain.types.iter().cloned().collect();
    types.insert("OBJECT".to_string());
    types.insert(String::new());
    for (_, t) in &all {
        types.insert(t.clone());
    }
    let mut map = HashMap::new();
    for ty in types {
        let v: Vec<Sym> = all
            .iter()
            .filter(|(_, ot)| is_a(ot, &ty))
            .map(|(o, _)| o.clone())
            .collect();
        map.insert(ty, v);
    }
    map
}

/// Every combination of objects across a set of typed parameters, laid out
/// full Cartesian.
fn enumerate(params: &[(Sym, Sym)], objs: &HashMap<Sym, Vec<Sym>>) -> Vec<HashMap<Sym, Sym>> {
    let mut out = vec![HashMap::new()];
    for (name, ty) in params {
        let cands = objs.get(ty).cloned().unwrap_or_default();
        let mut next = Vec::with_capacity(out.len() * cands.len());
        for b in &out {
            for o in &cands {
                let mut nb = b.clone();
                nb.insert(name.clone(), o.clone());
                next.push(nb);
            }
        }
        out = next;
    }
    out
}

fn resolve(t: &Term, b: &HashMap<Sym, Sym>) -> Sym {
    match t {
        Term::Var(v) => b.get(v).cloned().unwrap_or_else(|| v.clone()),
        Term::Const(c) => c.clone(),
    }
}

/// Read a (static) rule body true or false under a binding, against the
/// fact set standing right now.
fn eval(
    f: &Formula,
    b: &HashMap<Sym, Sym>,
    facts: &HashSet<(Sym, Vec<Sym>)>,
    objs: &HashMap<Sym, Vec<Sym>>,
) -> Result<bool, String> {
    match f {
        Formula::True => Ok(true),
        Formula::False => Ok(false),
        Formula::And(v) => {
            for x in v {
                if !eval(x, b, facts, objs)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Formula::Or(v) => {
            for x in v {
                if eval(x, b, facts, objs)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Formula::Not(a) => Ok(!eval(a, b, facts, objs)?),
        Formula::Atom(p, args) => {
            let resolved: Vec<Sym> = args.iter().map(|t| resolve(t, b)).collect();
            Ok(facts.contains(&(p.clone(), resolved)))
        }
        Formula::Eq(x, y) => Ok(resolve(x, b) == resolve(y, b)),
        Formula::Exists(vars, inner) => {
            for bb in enumerate(vars, objs) {
                let mut m = b.clone();
                m.extend(bb);
                if eval(inner, &m, facts, objs)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Formula::Forall(vars, inner) => {
            for bb in enumerate(vars, objs) {
                let mut m = b.clone();
                m.extend(bb);
                if !eval(inner, &m, facts, objs)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Formula::Comp(..) => {
            Err("numeric comparison in a derived rule body is not supported".to_string())
        }
        Formula::Pref(..) => Err("preference in a derived rule body is not supported".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_domain, parse_problem};

    // reachable = transitive closure of a static `link` relation.
    const DOM: &str = "(define (domain g) (:requirements :typing :adl)
      (:types node)
      (:predicates (link ?a ?b - node) (reachable ?a ?b - node) (at ?n - node) (visited ?n - node))
      (:derived (reachable ?a ?b - node)
        (or (link ?a ?b)
            (exists (?c - node) (and (link ?a ?c) (reachable ?c ?b)))))
      (:action go :parameters (?from ?to - node)
        :precondition (and (at ?from) (reachable ?from ?to))
        :effect (and (not (at ?from)) (at ?to) (visited ?to))))";

    #[test]
    fn static_reachability_closure_added_to_init() {
        let prob = "(define (problem p) (:domain g)
          (:objects a b c d - node)
          (:init (at a) (link a b) (link b c) (link c d))
          (:goal (visited d)))";
        let d = parse_domain(DOM).expect("domain");
        let p = parse_problem(prob).expect("problem");
        let (d2, p2) = compile(&d, &p).expect("derived compiles");

        assert!(d2.derived.is_empty(), "derived rules compiled away");
        let reachable: HashSet<(Sym, Vec<Sym>)> = p2
            .init_atoms
            .iter()
            .filter(|(q, _)| q == "REACHABLE")
            .cloned()
            .collect();
        // a reaches b,c,d; b reaches c,d; c reaches d (6 pairs).
        assert!(reachable.contains(&("REACHABLE".into(), vec!["A".into(), "D".into()])));
        assert!(reachable.contains(&("REACHABLE".into(), vec!["B".into(), "D".into()])));
        assert!(reachable.contains(&("REACHABLE".into(), vec!["A".into(), "C".into()])));
        assert_eq!(
            reachable.len(),
            6,
            "transitive closure has 6 reachable pairs"
        );
        // not reachable backwards
        assert!(!reachable.contains(&("REACHABLE".into(), vec!["D".into(), "A".into()])));
    }

    #[test]
    fn dynamic_derived_is_rejected_clearly() {
        // body references `visited`, which the `go` action changes -> dynamic.
        let dom = "(define (domain g) (:requirements :typing :adl)
          (:types node)
          (:predicates (at ?n - node) (visited ?n - node) (toured))
          (:derived (toured) (forall (?n - node) (visited ?n)))
          (:action go :parameters (?to - node) :precondition (at ?to) :effect (visited ?to)))";
        let prob =
            "(define (problem p) (:domain g) (:objects a - node) (:init (at a)) (:goal (toured)))";
        let d = parse_domain(dom).expect("domain");
        let p = parse_problem(prob).expect("problem");
        let err = compile(&d, &p).expect_err("dynamic derived must be rejected");
        assert!(err.contains("TOURED"), "error names the predicate: {err}");
    }
}
