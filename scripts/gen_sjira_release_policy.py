#!/usr/bin/env python3
"""Compile a Semantic-Jira release work graph into an HDDL/FOND fixture.

Composition C01 ("release as policy synthesis", work order FERROPLAN-26922-09):
the v26.9.22 work graph (orders = tasks, depends_on = preconditions, outcomes
ALIVE/REFUTED/BLOCKED nondeterministic) becomes one HDDL domain + problem that
`ferroplan::hddl::solve_hddl` solves for a strong-cyclic FOND policy. The
acceptance test is `cargo test -p ferroplan --test sjira_release_policy`.

    # regenerate from the live release graph (projects it into work-graph.json)
    python3 scripts/gen_sjira_release_policy.py --orders /Users/sac/wt/v26922/orders.json

    # drift gate: re-render domain/problem from the committed work-graph.json
    # and exit 1 if the committed fixture bytes differ
    python3 scripts/gen_sjira_release_policy.py --check

Outputs (crates/ferroplan/tests/fixtures/sjira-release-policy/):

* work-graph.json -- the zero-content projection of orders.json the fixture
  is rendered from: order id, owning repo group, depends_on. Titles and
  acceptance commands are dropped; the source path + sha256 are recorded.
* domain.hddl     -- lifted schema (tasks visit/deliver, actions
  execute/repair/defer, execute = oneof {alive, refuted, blocked}) plus the
  generated sweep: one constant per order and `(sweep)` -> ordered
  `(chunk-NN)` tasks -> ordered `(visit <order>)` tasks in id order.
* problem.hddl    -- one `(depends-on o d)` init fact per edge (one per line,
  so a test can delete exactly one edge), every order pending, goal = every
  order alive, and an :htn root of R ordered `(sweep)` tasks.

The sweep is a two-level tree (chunks of ceil(sqrt(N)) visits) rather than
one flat network of N visits: translate canonicalizes the pending task
network on every composite state in O(|network|^2), so a flat 169-visit
network costs seconds per hundred states while the tree keeps the pending
network under ~2*sqrt(N)+R tasks.

Why sweeps instead of a free choice of the next frontier order: ferroplan's
HDDL pipeline translates to an explicit composite-state graph before the FOND
fixpoint runs. Letting the planner pick any ready order makes the reachable
state set the lattice of downsets of the work graph (>= 2^roots; the v26.9.22
graph has 36 roots), which no explicit-state solver enumerates. Visiting the
orders in `050_frontier.rq`'s `ORDER BY ?id` order, taking an order exactly
when the frontier admits it (pending and every dependency alive) and passing
otherwise, keeps the state space linear in the graph. R is the smallest sweep
count that can admit the whole graph: 1 + the maximum number of id-order
descents (an edge d -> o with o sorted before d) along any dependency path.
The dependency test itself stays with the planner: take/pass and `execute`
carry the depends-on preconditions, so deleting one edge from problem.hddl
changes which visit takes the order.
"""
import argparse
import hashlib
import heapq
import math
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
FIXTURE_DIR = REPO / "crates/ferroplan/tests/fixtures/sjira-release-policy"
DEFAULT_ORDERS = "/Users/sac/wt/v26922/orders.json"
GENERATOR = "scripts/gen_sjira_release_policy.py"


def refuse(msg):
    print(f"REFUSED:SJIRA_WORK_GRAPH: {msg}", file=sys.stderr)
    sys.exit(2)


def project(orders_path):
    """orders.json (repo -> {orders: [...]}) -> the work-graph projection."""
    raw = Path(orders_path).read_bytes()
    doc = json.loads(raw)
    orders = []
    for repo, group in sorted(doc.items()):
        for order in group.get("orders", []):
            orders.append(
                {"id": order["id"], "repo": repo, "deps": sorted(set(order.get("deps", [])))}
            )
    orders.sort(key=lambda o: o["id"])
    return {
        "generator": GENERATOR,
        "source": {"path": str(orders_path), "sha256": hashlib.sha256(raw).hexdigest()},
        "orders": orders,
    }


def hddl_name(order_id):
    name = order_id.lower()
    if not name or any(c.isspace() or c in "();" for c in name):
        refuse(f"order id {order_id!r} is not an HDDL token")
    return name


def analyse(graph):
    """Validate the graph (unique ids, known deps, acyclic); return (ids, deps, sweeps)."""
    ids = [o["id"] for o in graph["orders"]]
    if len(ids) != len(set(ids)):
        refuse("duplicate order ids")
    names = [hddl_name(i) for i in ids]
    if len(names) != len(set(names)):
        refuse("order ids collide after lowercasing")
    deps = {o["id"]: o["deps"] for o in graph["orders"]}
    for oid, ds in deps.items():
        for d in ds:
            if d not in deps:
                refuse(f"{oid} depends on unknown order {d}")
            if d == oid:
                refuse(f"{oid} depends on itself")
    # Kahn with an id-ordered heap: detects cycles and yields a topological
    # order to run the sweep-level DP over.
    indeg = {i: len(deps[i]) for i in ids}
    children = {i: [] for i in ids}
    for oid in ids:
        for d in deps[oid]:
            children[d].append(oid)
    heap = [i for i in ids if indeg[i] == 0]
    heapq.heapify(heap)
    topo = []
    while heap:
        oid = heapq.heappop(heap)
        topo.append(oid)
        for c in children[oid]:
            indeg[c] -= 1
            if indeg[c] == 0:
                heapq.heappush(heap, c)
    if len(topo) != len(ids):
        refuse("dependency cycle: " + ", ".join(sorted(i for i in ids if indeg[i] > 0)))
    rank = {oid: n for n, oid in enumerate(ids)}
    level = {}
    for oid in topo:
        level[oid] = max(
            (level[d] + (1 if rank[oid] < rank[d] else 0) for d in deps[oid]), default=0
        )
    sweeps = 1 + max(level.values(), default=0)
    return ids, deps, sweeps


def render(graph):
    ids, deps, sweeps = analyse(graph)
    src = graph["source"]
    graph_sha = hashlib.sha256(canonical_json(graph).encode()).hexdigest()
    names = [hddl_name(i) for i in ids]
    n_edges = sum(len(v) for v in deps.values())
    header = (
        f";; GENERATED by {GENERATOR} from work-graph.json (sha256 {graph_sha}).\n"
        f";; Source: {src['path']} (sha256 {src['sha256']}). DO NOT EDIT;\n"
        f";; regenerate, then `python3 {GENERATOR} --check`.\n"
        ";; Work order FERROPLAN-26922-09 (composition C01: release as policy synthesis).\n"
        f";; {len(ids)} orders, {n_edges} dependency edges, {sweeps} frontier sweeps.\n"
    )
    width = max(1, math.isqrt(max(len(names) - 1, 0)) + 1)
    chunks = [names[i : i + width] for i in range(0, len(names), width)]
    chunk_ids = [f"chunk-{n:02d}" for n in range(1, len(chunks) + 1)]
    d = [header.rstrip("\n"), "(define (domain sjira-release)", "  (:types order)"]
    d.append("  (:constants")
    d += [f"    {n}" for n in names]
    d.append("    - order)")
    d.append(DOMAIN_SCHEMA.rstrip("\n"))
    d.append("  ;; one frontier sweep: every order visited in 050_frontier.rq ORDER BY ?id order")
    d.append("  (:task sweep :parameters ())")
    d += [f"  (:task {c} :parameters ())" for c in chunk_ids]
    d.append("  (:method sweep-chunks")
    d.append("    :parameters ()")
    d.append("    :task (sweep)")
    d.append("    :precondition (and)")
    d.append("    :ordered-subtasks (and")
    d += [f"      ({c} ({c}))" for c in chunk_ids]
    d.append("    ))")
    for c, members in zip(chunk_ids, chunks):
        d.append(f"  (:method {c}-visits")
        d.append("    :parameters ()")
        d.append(f"    :task ({c})")
        d.append("    :precondition (and)")
        d.append("    :ordered-subtasks (and")
        d += [f"      (v-{n} (visit {n}))" for n in members]
        d.append("    ))")
    d.append(")")
    domain = "\n".join(d) + "\n"

    p = [header.rstrip("\n"), "(define (problem sjira-release-v26-9-22)"]
    p.append("  (:domain sjira-release)")
    p.append(f"  ;; {sweeps} ordered frontier sweeps over the {len(ids)} orders")
    p.append("  (:htn :ordered-subtasks (and")
    p += [f"    (s{s} (sweep))" for s in range(1, sweeps + 1)]
    p.append("  ))")
    p.append("  (:init")
    p += [f"    (pending {n})" for n in names]
    p.append("    ;; depends_on edges: (depends-on <order> <dependency>), one per line")
    for i in ids:
        for dep in deps[i]:
            p.append(f"    (depends-on {hddl_name(i)} {hddl_name(dep)})")
    p.append("  )")
    p.append("  (:goal (and")
    p += [f"    (alive {n})" for n in names]
    p.append("  )))")
    return domain, "\n".join(p) + "\n"


DOMAIN_SCHEMA = """  (:predicates
    (depends-on ?o - order ?d - order)
    (pending ?o - order)
    (alive ?o - order)
    (refuted ?o - order)
    (blocked ?o - order))
  (:task visit :parameters (?o - order))
  (:task deliver :parameters (?o - order))
  ;; frontier admits ?o: pending and every dependency ALIVE -> deliver it now
  (:method visit-take
    :parameters (?o - order)
    :task (visit ?o)
    :precondition (and (pending ?o)
      (forall (?d - order) (imply (depends-on ?o ?d) (alive ?d))))
    :ordered-subtasks (and (t1 (deliver ?o))))
  ;; already ALIVE, or some dependency not yet ALIVE -> leave it for a later sweep
  (:method visit-pass
    :parameters (?o - order)
    :task (visit ?o)
    :precondition (or (alive ?o)
      (exists (?d - order) (and (depends-on ?o ?d) (not (alive ?d)))))
    :ordered-subtasks (and))
  (:method deliver-done
    :parameters (?o - order)
    :task (deliver ?o)
    :precondition (and (alive ?o))
    :ordered-subtasks (and))
  (:method deliver-run
    :parameters (?o - order)
    :task (deliver ?o)
    :precondition (and (pending ?o))
    :ordered-subtasks (and (t1 (execute ?o)) (t2 (deliver ?o))))
  (:method deliver-repair
    :parameters (?o - order)
    :task (deliver ?o)
    :precondition (and (refuted ?o))
    :ordered-subtasks (and (t1 (repair ?o)) (t2 (deliver ?o))))
  (:method deliver-defer
    :parameters (?o - order)
    :task (deliver ?o)
    :precondition (and (blocked ?o))
    :ordered-subtasks (and (t1 (defer ?o)) (t2 (deliver ?o))))
  ;; run the order's acceptance: the court's verdict is nondeterministic
  (:action execute
    :parameters (?o - order)
    :precondition (and (pending ?o)
      (forall (?d - order) (imply (depends-on ?o ?d) (alive ?d))))
    :effect (oneof
      (and (not (pending ?o)) (alive ?o))
      (and (not (pending ?o)) (refuted ?o))
      (and (not (pending ?o)) (blocked ?o))))
  ;; REFUTED -> repair the candidate, then re-run acceptance
  (:action repair
    :parameters (?o - order)
    :precondition (and (refuted ?o))
    :effect (and (not (refuted ?o)) (pending ?o)))
  ;; BLOCKED -> defer until the blocking term clears, then re-run acceptance
  (:action defer
    :parameters (?o - order)
    :precondition (and (blocked ?o))
    :effect (and (not (blocked ?o)) (pending ?o)))
"""


def canonical_json(graph):
    return json.dumps(graph, indent=2, sort_keys=True) + "\n"


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--orders", help=f"orders.json to project (e.g. {DEFAULT_ORDERS})")
    ap.add_argument("--out", default=str(FIXTURE_DIR))
    ap.add_argument(
        "--check",
        action="store_true",
        help="re-render from the committed work-graph.json; exit 1 on drift",
    )
    args = ap.parse_args()
    out = Path(args.out)
    if args.check:
        graph = json.loads((out / "work-graph.json").read_text())
        domain, problem = render(graph)
        drift = [
            name
            for name, text in (
                ("work-graph.json", canonical_json(graph)),
                ("domain.hddl", domain),
                ("problem.hddl", problem),
            )
            if (out / name).read_text() != text
        ]
        if drift:
            print("DRIFT " + " ".join(drift))
            return 1
        _, deps, sweeps = analyse(graph)
        print(
            f"CONFORMANT {len(deps)} orders, {sum(map(len, deps.values()))} edges, "
            f"{sweeps} sweeps, source sha256 {graph['source']['sha256']}"
        )
        return 0
    graph = project(args.orders or DEFAULT_ORDERS)
    domain, problem = render(graph)
    out.mkdir(parents=True, exist_ok=True)
    (out / "work-graph.json").write_text(canonical_json(graph))
    (out / "domain.hddl").write_text(domain)
    (out / "problem.hddl").write_text(problem)
    _, deps, sweeps = analyse(graph)
    print(
        f"wrote {out}: {len(deps)} orders, {sum(map(len, deps.values()))} edges, "
        f"{sweeps} sweeps"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
