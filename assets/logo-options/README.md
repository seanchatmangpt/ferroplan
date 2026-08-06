# ferroplan — alternative logo concepts

Four signals pulled from the static, four distinct angles on "ferroplan" (a fast
PDDL planner, forged in Rust). Each concept ships a wordmark (`concept-N.svg`) and a
square favicon-style mark (`concept-N-mark.svg`). All SVGs are fully self-contained —
no external fonts (`system-ui, sans-serif`), no external images, nothing phoning
home. They're built to read on both light and dark backgrounds: marks lean on solid
fills, wordmarks steer clear of pure-black/white glyphs.

To compare against the current brand, see `../logo.svg` and `../logo-mark.svg`.

## Concept 1 — Forged iron (anvil + spark)

**Direction:** the "ferro" — iron, metal — taken literally. An angular anvil
silhouette takes the hit, throws a spark: the instant a plan gets forged. The spark
doubles as the goal/output accent.
**Rationale:** the most concrete, memorable take; it says "metal" and "made" the
moment you see it, no graph-search explainer required. Bold weight reads as solidity
and speed at once.
**Palette (new):** steel slate `#64748b` + forge-spark amber `#f59e0b`. A
deliberate break from the indigo brand, committing all the way to the metal theme —
the warm amber pops hard on either background.

## Concept 2 — Solution route

**Direction:** planning as a route through the dark. Faint pruned candidate
branches fan out in grey, dead ends left visible; one bold indigo path threads from
a start node through a waypoint to the goal — the one route the planner actually
found and kept.
**Rationale:** communicates "this finds a path through possibilities" at a
glance — the core value of a planner — while staying clean and minimal. The hollow
goal node reads as a target, a flag planted at the end of the search.
**Palette (brand riff):** indigo `#6c5ce7` route, slate `#94a3b8` pruned
branches, emerald `#10b981` goal (a slightly punchier green than the original
`#a8d24a`, for stronger contrast on light backgrounds).

## Concept 3 — "fp" monogram

**Direction:** a minimal, geometric `fp` monogram set in a rounded-square tile.
The `f` and `p` are stroke-built and share a baseline — a tight, modern lockup, no
wasted stroke.
**Rationale:** the most flexible and scalable option — a single-color tile that
holds up as an app icon, a terminal favicon, a social avatar shrunk down to a
thumbnail, where a graph mark would just blur to noise. Quiet and confident.
**Palette (brand):** indigo `#6c5ce7` tile with white glyphs. The tile fill
guarantees contrast on any background; recolor the tile without ever touching the
glyphs.

## Concept 4 — State-space search

**Direction:** an explicit state-space graph, the machinery laid bare. A small
lattice of states with dim "frontier" edges; one highlighted indigo branch is
expanded from the root (square = explored) down to the green goal state.
**Rationale:** the most technically literal of the four — it depicts search itself,
which lands hard with a PDDL / automated-planning audience. Closest in spirit to the
existing node-graph mark, but reframed around *search* (root, frontier, goal) rather
than a generic graph.
**Palette (brand):** indigo `#6c5ce7` expanded path, grey `#9aa0a6` frontier,
green `#a8d24a` goal — the original three-color palette, kept for continuity.
