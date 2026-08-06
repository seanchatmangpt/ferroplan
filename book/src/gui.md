# GUI (`ferroplan-bevy`)

Text goes in, a world comes out. `ferroplan-bevy` is a [Bevy](https://bevyengine.org/)
app that renders a PDDL domain + problem into something you can *see*: a
force-directed graph of the problem, an animated plan, a Blockly-style block
editor for both problem and domain.

```sh
cargo run -p ferroplan-bevy                       # start empty, load via the editor
cargo run -p ferroplan-bevy domain.pddl problem.pddl
```

## Visualize a domain + problem as a graph

Static objects surface as **nodes**, force-laid, per-type icons — a circle for
`location`, a square for the mobile `package`/`truck` types. Static binary
predicates (`road`, say) draw as **edges**. State sits *on* the graph: a
package rests on the node it's `at`. Right-drag pans, scroll zooms, click
inspects a node, drag repositions it.

![A delivery problem as a typed graph: location nodes (circles) joined by `road` edges, with package/truck mobiles (squares) sitting on the node they're currently at.](images/graph.png)
*The problem as a typed graph — locations are circles, mobiles are squares, `road` predicates are edges. The side panel shows the inspected object and the goal.*

No per-domain config, no manual mapping — icons and edge colors read straight
off the PDDL. A **logistics** problem renders the package as a box, trucks and
train as mobiles, rail legs in blue, roads in gray:

![A logistics problem: location nodes joined by gray road edges and a blue rail edge, a package box, and truck/train mobiles.](images/logistics-graph.png)
*Logistics — rail (blue) vs road (gray) edges are distinguished automatically.*

A **job-shop** schedule: machines as octagons, jobs as boxes, stage routing
(`s1→s2→s3`) traced in amber:

![A job-shop problem: stage nodes joined by amber routing edges, machines as octagons, and jobs as boxes.](images/jobshop-graph.png)
*Job-shop — machines (octagons), jobs (boxes), and stage routing (amber).*

## Animate the plan

**S** solves — same `ferroplan::solve` call the CLI makes underneath. **Space**
plays. The plan replays step by step, mobiles sliding edge to edge as each
action fires, the side panel echoing the current step. Arrow keys step
manually. **R** resets to the initial state.

![Mid-animation: a plan is playing, showing "step 5/9: drive rig garage harbor", with a crate having moved to its new node.](images/animation.png)
*A plan animating mid-step — the side panel tracks `step 5/9` while the mobiles move along the graph.*

## Block editor — problems

Drag-and-snap, no syntax to get wrong. The **problem editor** handles typed
objects, init facts, and the goal as nested blocks. **Apply** re-parses and
re-renders the graph live. **Export** writes the PDDL back out.

![The problem block editor: typed OBJECTS, INIT facts (road/pkg-at blocks), and GOAL blocks in a left panel, with the graph updating on Apply.](images/editor-problem.png)
*The problem editor — objects, init, and goal as typed blocks; Apply re-renders the graph, Export writes PDDL.*

## Block editor — domains

The editor doesn't stop at the surface — it goes all the way down to the
**domain**: types and predicates...

![The domain editor: a TYPES section (location/truck/package, each `- object`) and a PREDICATES section (road, truck-at, pkg-at, in) as blocks.](images/editor-domain.png)
*The domain editor — the type hierarchy and predicate signatures as editable blocks.*

...down to the **actions** — parameters, precondition, effect, each as
positive/negative literal blocks. Author or tweak an operator; never open a
text file.

![The action editor: drive/load/unload actions, each with typed parameters and pre/eff literal lists (pos/neg).](images/editor-actions.png)
*The action editor — parameters plus precondition and effect literals (positive / negative) per action.*

> Editor and solver run the same parser, the same `solve` entry point, as the
> `ff` CLI. What you see is what the planner sees — no daylight between them.
