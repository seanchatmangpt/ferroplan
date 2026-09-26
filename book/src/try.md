# Try it in your browser

Compiled to **WebAssembly**. Runs **entirely client-side**. No server, no
install, nothing leaves the machine.

> ### [▶ Open the live planner demo](./demo/index.html)

**Pick a built-in example** from the dropdown — gripper, numeric resources, ADL,
derived axioms, PDDL3 preferences, temporal/durative, logistics, a job shop, one
of the RPG-world scenarios (including a **border** case: the point where a
monolithic goal stops solving in one shot and has to be broken apart) — or drop in
**your own** PDDL **domain** + **problem**.
Pick a mode (`auto` routes by problem features), hit **Plan**. The solve runs
in-browser, same Rust planner core, compiled to WASM. Gripper loads first — a plan
on screen before you touch anything.

**Save it for offline.** The header's **Save offline** button keeps a copy of
the demo in your browser — the three pages, the example corpus and the WASM
planner, a few megabytes — so it opens and plans with no network at all
(airplane, lecture hall, the site being down). Nothing is stored until you
ask. The copy refreshes itself in the background whenever you visit online, a
new deploy replaces it whole, and the button reports what it holds (files,
size, the build it came from); the **✕** beside it removes the copy. The page
is also installable as an app from the browser's menu. Your browser has to
allow service workers on the origin (it is a secure page, so the only place
this is hidden is a plain `file://` open).

## The visual GUI, in your browser

The full Bevy GUI runs in-browser too — graph visualizer, plan animation, the
color-coded block editor. Larger download. Give it a moment.

> ### [▶ Open the visualizer & block editor](./gui/index.html)

Keys: **E** toggles the editor, **Tab** switches problem/domain, **S** solves,
**Space** plays the plan; drag nodes, scroll to zoom, click to inspect.

## The living bazaar

A replay of a real multi-mind `Session` tick-loop — two planners, one barter
world, run with and without contention claims (see
[Game embedding](./session.md)):

> ### [▶ Open the living-bazaar replay](./demo/bazaar-live.html)

Everything else — CLI, library, install — is the rest of the docs.
