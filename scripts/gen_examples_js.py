#!/usr/bin/env python3
"""Deterministic generator for crates/ferroplan-wasm/web/examples.js (DOMAINS).

  gen_examples_js.py           write the file
  gen_examples_js.py --check   exit 1 if the committed file differs from the regeneration

SPEC below is the recorded provenance: per problem, either a repo file (read
at generation time, `trail` trailing newlines stripped) or an inline entry pinned by
sha256 (no source file exists in the repo; its text is carried forward from
the current examples.js and any edit trips the pin). Edit SPEC or the source
PDDL files, then regenerate; never hand-edit examples.js.
"""
import hashlib, json, os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "crates/ferroplan-wasm/web/examples.js")
HEAD = "// Auto-generated: DOMAINS for the two-level demo picker (domain -> graded problems).\nexport const DOMAINS = [\n"
SPEC = json.loads(r'''[
 {
  "key": "gripper",
  "label": "Gripper — the classic (STRIPS)",
  "note": "Classic STRIPS: a robot ferries balls between two rooms.",
  "domain": {
   "inline_sha256": "595e59e9c8551727db348da8aeab5c5d87b40e086262ea32f416d0e6846611e2"
  },
  "problems": [
   {
    "key": "gripper",
    "label": "baseline",
    "note": "Classic STRIPS: a robot ferries balls between two rooms.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "9b8f71858c74436aa9c6fa9f13cb30254a228e47c893e5ac81fcf3f7caf672a8"
    }
   }
  ]
 },
 {
  "key": "numeric",
  "label": "Fuel & travel — numeric resources",
  "note": "Numeric fluents: fuel gates travel — refuel to afford the trip.",
  "domain": {
   "file": ".claude/skills/ferroplan/examples/numeric/domain.pddl",
   "trail": 1
  },
  "problems": [
   {
    "key": "numeric",
    "label": "baseline",
    "note": "Numeric fluents: fuel gates travel — refuel to afford the trip.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": ".claude/skills/ferroplan/examples/numeric/problem.pddl",
     "trail": 1
    }
   }
  ]
 },
 {
  "key": "adl",
  "label": "Courier — ADL (forall + conditional)",
  "note": "ADL: one move carries every held parcel (forall + when), instead of a separate action per cargo.",
  "domain": {
   "file": ".claude/skills/ferroplan/examples/adl/domain.pddl",
   "trail": 1
  },
  "problems": [
   {
    "key": "adl",
    "label": "baseline",
    "note": "ADL: one move carries every held parcel (forall + when), instead of a separate action per cargo.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": ".claude/skills/ferroplan/examples/adl/problem.pddl",
     "trail": 1
    }
   }
  ]
 },
 {
  "key": "axioms",
  "label": "Reachable regions — derived axioms",
  "note": "Derived axioms: reachability computed as the transitive closure of the static link graph.",
  "domain": {
   "file": ".claude/skills/ferroplan/examples/axioms/domain.pddl",
   "trail": 1
  },
  "problems": [
   {
    "key": "axioms",
    "label": "baseline",
    "note": "Derived axioms: reachability computed as the transitive closure of the static link graph.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": ".claude/skills/ferroplan/examples/axioms/problem.pddl",
     "trail": 1
    }
   }
  ]
 },
 {
  "key": "preferences",
  "label": "Grab loot if convenient — PDDL3 preferences",
  "note": "PDDL3: a soft goal the planner satisfies only when it doesn't block the hard goals (minimizes is-violated).",
  "domain": {
   "file": ".claude/skills/ferroplan/examples/preferences/domain.pddl",
   "trail": 1
  },
  "problems": [
   {
    "key": "preferences",
    "label": "baseline",
    "note": "PDDL3: a soft goal the planner satisfies only when it doesn't block the hard goals (minimizes is-violated).",
    "mode": "pddl3",
    "flags": "",
    "problem": {
     "file": ".claude/skills/ferroplan/examples/preferences/problem.pddl",
     "trail": 1
    }
   }
  ]
 },
 {
  "key": "logistics",
  "label": "Logistics — trucks, a train & depots",
  "note": "Durative domain (:durative-actions + :numeric-fluents), so --mode auto routes to the temporal planner and reports makespan. IMPORTANT BORDER: this domain only solves the single-package / single-vehicle shape (one crate, one truck-or-train, any number of hops). It is documented in examples/logistics/README.md that adding a 2nd package, a 2nd contributing vehicle, or a truck->train transshipment HANDOFF makes it unsolvable (a converging-contributions >=2 limitation) -- I empirically reconfirmed this: 2-package and truck->train-handoff drafts all failed to solve and were dropped. The existing example p3-p8 (which attempt transshipment/multi-package) also do NOT solve on this binary. The 4 problems below therefore ramp complexity via network SIZE and ROUTE CHOICE while honoring the single-package/single-vehicle invariant, and every one is verified solved.",
  "domain": {
   "file": "examples/logistics/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "corridor-one-crate",
    "label": "Corridor delivery, 1 crate (simple)",
    "note": "One truck carries a single crate three hops along a linear road corridor a-b-c-d; the baseline load/drive/unload shape.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "4d17bb1dde3cabd594c5d17fda8f75020d90eef0e4cad5c0863c4e51424e8f2b"
    }
   },
   {
    "key": "star-fetch-deliver",
    "label": "Star network, dead-head + deliver (medium)",
    "note": "Truck parked at one spoke must dead-head through the hub to fetch a crate at a different spoke, then route it out across a relay to a far depot.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "82435e2cd0e467d8308b922964f1825dc02f1548e711a0607f7496cc24e412d4"
    }
   },
   {
    "key": "rail-line-out-and-back",
    "label": "Five-depot rail line, out-and-back haul (medium-hard)",
    "note": "A single train mid-line must first run west to collect the crate at rA, then run the full four-hop rail spine east to rE -- exercises the train + rail-depot machinery, no roads.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "5ab7d061fa4c65c22776142a7dca5695d28ffe77eed773ce2c226180bf162505"
    }
   },
   {
    "key": "city-grid-diagonal-haul",
    "label": "4x4 city road grid, corner-to-corner haul (complex)",
    "note": "16 intersections / 48 one-way road records with non-uniform (and two congested) travel times; the truck dead-heads from one corner to the crate in another, then threads the whole grid diagonally to a third corner amid many parallel routes.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "dbbbb726f14a14a75588032f217d56fa9dce1a77b9a94e78fe63da166d252a55"
    }
   }
  ]
 },
 {
  "key": "jobshop",
  "label": "Job shop — machine scheduling",
  "note": "Each job is a fixed sequence of operations; every operation runs on a machine that processes one op at a time (free-token exclusion consumed at-start, restored at-end). Durative + numeric proctime fluents; mode auto routes it to the temporal planner, which overlaps independent jobs across free machines to compress makespan.",
  "domain": {
   "file": "examples/jobshop/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "jobshop-small-3x3",
    "label": "Small 3x3 (warm-up)",
    "note": "Classic 3-job by 3-stage by 3-machine shop where the three jobs must interleave on shared machines; the smallest case that already shows real resource contention.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "366009354543b433543c4eae93c59f419b8c80a5008cf5621e5195fcf61f2e33"
    }
   },
   {
    "key": "jobshop-medium-8x6",
    "label": "Medium 8x6 (contention)",
    "note": "8 jobs chase 6 machines over 6-stage routes, so two jobs always queue for the same machine and the planner must stagger starts to keep makespan low.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "12fef0eea0982230c98ee627783b9f9b43fd51d9eeb2072c6ffda31f5cdd564e"
    }
   },
   {
    "key": "jobshop-large-25x10",
    "label": "Large 25x10 (~2.5k groundings)",
    "note": "25 jobs run 10-stage routes across 10 machines (~2500 operate groundings); a substantial schedule that still solves in a fraction of a second.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "inline_sha256": "443d6a34e1bec3ac6e0163329f37d48958a4ae75967f2420095773b4c67f1eb7"
    }
   }
  ]
 },
 {
  "key": "temporal",
  "label": "Overlapping work — durative / temporal",
  "note": "PDDL2.1 durative actions with an over-all invariant — note the timed plan and makespan.",
  "domain": {
   "file": ".claude/skills/ferroplan/examples/temporal/domain.pddl",
   "trail": 1
  },
  "problems": [
   {
    "key": "temporal",
    "label": "baseline",
    "note": "PDDL2.1 durative actions with an over-all invariant — note the timed plan and makespan.",
    "mode": "temporal",
    "flags": "",
    "problem": {
     "file": ".claude/skills/ferroplan/examples/temporal/problem.pddl",
     "trail": 1
    }
   }
  ]
 },
 {
  "key": "villagers",
  "label": "Villagers — generic recipe planner (abstract RPG)",
  "note": "Generic data-driven crafting model: three actions (walk/gather/craft) cover any map and any single-input recipe via :init data; durations are a (total-time) cost the :metric minimizes. Solved with --mode pddl3 (the metric optimizer), flags \"\". A conditional laden-walk penalty (load >5 => +2 per walk) makes routing/load order matter. The four problems form a clean complexity ramp: a single errand, a two-step chain, the existing 7-node township, and a 12-node big town with three multi-tier chains and a 4-part goal whose optimal plan is 59 steps yet solves in ~1s.",
  "domain": {
   "file": "examples/villagers/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "gather-errand-villagers",
    "label": "Gather errand (tiny)",
    "note": "Simplest possible: walk home->grove, gather 2 wood, craft 1 plank — one source, one workshop, one goal.",
    "mode": "pddl3",
    "flags": "",
    "problem": {
     "inline_sha256": "0895e2c813cfeb9d1591faaad151a1c3a4bea52716ea8ccd03a4882af5e032ce"
    }
   },
   {
    "key": "two-step-chain-villagers",
    "label": "Two-step chain (small)",
    "note": "4-node map, two-tier chain ore->bar->tool: gather 2 ore, smelt 1 bar, forge 1 tool, sequencing two workshops.",
    "mode": "pddl3",
    "flags": "",
    "problem": {
     "inline_sha256": "892259317b1d1c197963d9a65947fbd683f5a645c5124105de61d0f3c2c85f27"
    }
   },
   {
    "key": "township-villagers",
    "label": "Township (medium)",
    "note": "The existing 7-node township: two chains (wood->plank, ore->bar->tool), goal of 2 tools + 3 planks, laden-walk penalty in play; matches README total-time 38.",
    "mode": "pddl3",
    "flags": "",
    "problem": {
     "file": "examples/villagers/township.pddl",
     "trail": 0
    }
   },
   {
    "key": "big-town-villagers",
    "label": "Big town (large/complex)",
    "note": "12-node map, 3 gather sources, three multi-tier chains (wood->plank, ore->bar->tool, clay->brick) and a 4-part goal (3 tools, 4 planks, 5 bricks, 2 spare bars); optimal plan is 59 steps — 18 walks, 24 gathers, 17 crafts — solving in ~1s.",
    "mode": "pddl3",
    "flags": "",
    "problem": {
     "inline_sha256": "596b58eba49b5827eda3f07058fb54aeb30e1c9173dcf3e44009fc426a3dc67d"
    }
   }
  ]
 },
 {
  "key": "rpg-world",
  "label": "RPG world — durative crafting + decomposer",
  "note": "Graded temporal series showcasing the in-browser temporal features: plain temporal handles a linear contract; FF_TDEMAND restores the gradient for converging numeric DAGs (steel from cold) and multi-path numeric goals (coin via mint/sell/haul); FF_TDEMAND+FF_TDECOMP carves a from-raw structural goal (a built-square forall over house slots) that plain and tdemand-only both fail. Every problem inits all 78 zero-arity fluents the domain declares. All four verified with the release binary under --mode temporal --json --threads 1 (25s timeout); flag choices confirmed minimal (P2/P3 fail plain; P4 fails plain AND tdemand-only).",
  "domain": {
   "inline_sha256": "393c1500af04e61bb89e96a9489852cb1715238fb8c141e04e2f83c5e271969c"
  },
  "problems": [
   {
    "key": "villagers-starter",
    "label": "Starter contract — gather, mill, sell (tiny)",
    "note": "Simplest tier: a 3-location loop where one woodcutter travels to a forest, chops logs, walks to the mill-market, saws planks and sells them for coin — a clean linear gather->process->trade chain that solves on plain temporal with no extra features.",
    "mode": "temporal",
    "flags": "",
    "problem": {
     "inline_sha256": "c25d340a932c5a3d3bc369f1feb137e476f4e19ccccfb8ff093d4179481b46c3"
    }
   },
   {
    "key": "foundry-steel",
    "label": "Deep foundry — steel x2 from cold (converging DAG)",
    "note": "Multi-round converging DAG: each steel needs an ingot (ore+charcoal) plus coal, so both intermediates are needed twice from scratch — the relaxation goes flat here, so it FAILS on plain temporal and is solved by the FF_TDEMAND converging-resource demand term.",
    "mode": "temporal",
    "flags": "tdemand",
    "problem": {
     "inline_sha256": "f03a453ac30306166b65118db7b9e5e8ad9fc9c4b0396c4336c2b24774063e25"
    }
   },
   {
    "key": "mint-and-trade",
    "label": "Mint & trade — coin x18 via interchangeable paths (multi-path numeric)",
    "note": "A deep multi-path numeric goal: coin is producible three interchangeable ways (sell planks, mint bullion, haul cargo). Plain temporal dithers between paths and FAILS; FF_TDEMAND regresses the demand down the recipe DAG and commits to a path (here mining/refining gold->bullion->mint).",
    "mode": "temporal",
    "flags": "tdemand",
    "problem": {
     "inline_sha256": "3eb7d2115304711e74b2902868dd90e897415823935ceebfcbd662c146e72820"
    }
   },
   {
    "key": "villagers-township",
    "label": "Found a hamlet — square + houses + well (DECOMPOSER, ~25s)",
    "note": "A conjunctive/structural goal (build-square needs ALL houses, plus a well) — needs the partition-and-resolve DECOMPOSER (tdemand,tdecomp). ~25s in-browser: use Web Worker mode so the tab stays responsive.",
    "mode": "temporal",
    "flags": "tdemand,tdecomp",
    "problem": {
     "inline_sha256": "fc80b15ee8f7fa22f89e8e6729f6205ada769908de644eee30bcb55b58b9d008"
    },
    "slow": true
   }
  ]
 },
 {
  "key": "cabin",
  "label": "Log cabin — fell trees, mill lumber, raise it",
  "note": "A deep, linear crafting chain: the goal pulls ONE long sequence — fell trees, saw planks / hew beams / split shingles, mine+smelt+forge nails, dig sand + fire glass, quarry stone — then build IN ORDER: foundation → walls → roof → floor → door → windows → finish. Numeric/classical (mode auto); the full cabin is a ~52-step plan.",
  "domain": {
   "file": "examples/cabin/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "raise-frame",
    "label": "Frame & roof a shelter (~26 steps)",
    "note": "Day one: get a roof up — fell, mill, forge a few nails, quarry stone, then lay the foundation, raise the walls, frame the roof (goal: the shell). Solves instantly.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "examples/cabin/raise-frame.pddl",
     "trail": 0
    }
   },
   {
    "key": "raise-cabin",
    "label": "The whole log cabin (~52 steps, ~7s)",
    "note": "The whole job end to end: fell ~a dozen trees, mill all the lumber, forge the nails, fire the glass, then build the cabin, door, and windows in order. A ~52-step plan — use Web Worker mode so the page stays responsive (~7s).",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "examples/cabin/raise-cabin.pddl",
     "trail": 0
    },
    "slow": true
   }
  ]
 },
 {
  "key": "cabin-crew",
  "label": "Log cabin, parallel crew — watch makespan drop",
  "note": "The DURATIVE twin of the log cabin. The planner SCHEDULES the same 34-step job across a crew (one job per worker at a time), so independent work — chopping, mining, digging — overlaps. Switch crew size and watch the makespan fall: more workers finish faster. Uses the demand heuristic + the new concurrent scheduler (tdemand,tconc).",
  "domain": {
   "file": "examples/cabin/crew.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "crew-solo",
    "label": "1 worker — makespan 109 (everything serial)",
    "note": "One pair of hands does the whole job in sequence: the baseline.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/crew-solo.pddl",
     "trail": 0
    }
   },
   {
    "key": "crew-pair",
    "label": "2 workers — makespan 63",
    "note": "Two workers overlap the independent gathering/processing chains: ~42% faster for the same work.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/crew-pair.pddl",
     "trail": 0
    }
   },
   {
    "key": "crew-trio",
    "label": "3 workers — makespan 47",
    "note": "Three workers (watch t=0: three fell-tree at once); makespan bottoms out toward the serial build tail.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/crew-trio.pddl",
     "trail": 0
    }
   }
  ]
 },
 {
  "key": "cabin-skilled",
  "label": "Log cabin, skilled crew — tasks need the right specialist",
  "note": "Now tasks require SKILLS: only a sawyer may mill (saw/hew/split), only a smith may smelt + forge. The scheduler routes every skill-gated task to a worker who actually has the skill (look at the plan: in \"specialists\" every SAW-PLANKS is ANA and every SMELT/FORGE is BEN). Cross-train the crew and the skilled work spreads across everyone.",
  "domain": {
   "file": "examples/cabin/crew-skilled.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "skilled-specialists",
    "label": "Specialists — 1 sawyer, 1 smith, 1 labourer",
    "note": "Milling can only go to the sawyer (ANA), smelting/forging only to the smith (BEN); the labourer (CAL) gathers and builds.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/skilled-specialists.pddl",
     "trail": 0
    }
   },
   {
    "key": "skilled-crosstrained",
    "label": "Cross-trained — everyone has both skills",
    "note": "All three can mill and smith, so the skilled chains parallelise across the whole crew.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/skilled-crosstrained.pddl",
     "trail": 0
    }
   }
  ]
 },
 {
  "key": "forge-order",
  "label": "Forge order — the smith is the bottleneck",
  "note": "A smithing-heavy job (forge 80 nails): smelting + forging are SMITH-ONLY, mining is general labour. Watch the makespan as the crew changes — adding plain labourers barely helps (the lone smith caps it), but adding a second SMITH (same crew size) cuts ~a third off. Skill scarcity, not headcount, is what bottlenecks the job.",
  "domain": {
   "file": "examples/cabin/crew-skilled.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "forge-1smith",
    "label": "1 smith, 3 workers — makespan 65 (the bottleneck)",
    "note": "One smith does every smelt + forge in sequence; the two labourers can only mine.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/forge-1smith.pddl",
     "trail": 0
    }
   },
   {
    "key": "forge-1smith-crowd",
    "label": "1 smith, 5 workers — makespan 62 (more hands barely help)",
    "note": "Two extra labourers, but they can't smith — the lone smith still caps the job.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/forge-1smith-crowd.pddl",
     "trail": 0
    }
   },
   {
    "key": "forge-2smith",
    "label": "2 smiths, 3 workers — makespan 44 (hire a smith!)",
    "note": "Same crew size as the first, but the smithing now runs on two — a big drop.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/forge-2smith.pddl",
     "trail": 0
    }
   },
   {
    "key": "forge-3smith",
    "label": "3 smiths, 3 workers — makespan 38",
    "note": "A third smith helps less: now ore supply + the smelt→forge dependency start to bind.",
    "mode": "temporal",
    "flags": "tdemand,tconc",
    "problem": {
     "file": "examples/cabin/forge-3smith.pddl",
     "trail": 0
    }
   }
  ]
 },
 {
  "key": "elevators-costs",
  "label": "Elevators — action costs (IPC-2008)",
  "note": "IPC6 :action-costs — the metric is real cost, not plan length: the anytime cost sweep trades a longer plan for a cheaper one (first plan cost 100, swept to 54). Slow and fast elevators cost different amounts per floor.",
  "domain": {
   "file": "benchmarks/ipc/costs/elevators08/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "elevators-costs",
    "label": "baseline",
    "note": "IPC6 :action-costs — cheapest beats shortest: the sweep takes the first plan from cost 100 to 54.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "benchmarks/ipc/costs/elevators08/p01.pddl",
     "trail": 0
    }
   }
  ]
 },
 {
  "key": "elevators-netben",
  "label": "Elevators — net benefit (IPC-2008)",
  "note": "Oversubscription: every goal is SOFT with a utility, actions cost — maximize utility minus cost. The empty plan is legal; serving a passenger is worth it only when the ride costs less than the reward.",
  "domain": {
   "file": "benchmarks/ipc/netben/elevators08/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "elevators-netben",
    "label": "baseline",
    "note": "Net benefit: maximize normalizes onto the minimize B&B; the reported metric is the achieved net benefit.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "benchmarks/ipc/netben/elevators08/p01.pddl",
     "trail": 0
    }
   }
  ]
 },
 {
  "key": "barman",
  "label": "Barman — the landmark rung (IPC-2011, hard)",
  "note": "Long goal-interaction chains where the FF heuristic plateaus — EHC gives up, and the LAMA-style rung (landmark counting + preferred operators) carries the search. HARD: expect roughly half a minute in the browser (single-threaded WASM; ~5 s native).",
  "domain": {
   "file": "benchmarks/ipc/costs/barman11/domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "barman",
    "label": "baseline",
    "note": "Landmarks keep a progress gradient across the relaxed-plan plateau — this instance was unsolved at any budget before the rung.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "benchmarks/ipc/costs/barman11/p01.pddl",
     "trail": 0
    }
   }
  ]
 },
 {
  "key": "bazaar-chain",
  "label": "Bazaar — wants-gated barter (the game track)",
  "note": "A vendor releases goods only for the item it wants, so getting the top item means an 11-hop trade-up chain. The fixture behind the 0.13/0.14 many-minds work — see the Bazaar tab for the live multi-mind replay.",
  "domain": {
   "file": "benchmarks/bench/bazaar-chain-domain.pddl",
   "trail": 0
  },
  "problems": [
   {
    "key": "bazaar-chain",
    "label": "solo chain (11 hops)",
    "note": "One trader, one chain: heuristic-transparent — solves in a blink at any depth.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "benchmarks/bench/bazaar-chain.pddl",
     "trail": 0
    }
   },
   {
    "key": "bazaar-chain-x2m",
    "label": "crossed chains, two travellers",
    "note": "Two chains cross at every vendor; the joint goal needs 22 trades and the delete relaxation cannot see the contention.",
    "mode": "auto",
    "flags": "",
    "problem": {
     "file": "benchmarks/bench/bazaar-chain-x2m.pddl",
     "trail": 0
    }
   }
  ]
 }
]''')


def load_carried():
    """Inline texts, keyed by sha256, taken from the current examples.js."""
    carried = {}
    try:
        s = open(OUT, encoding="utf-8").read()
        for x in json.loads(s.split("export const DOMAINS = ", 1)[1].rstrip().rstrip(";")):
            for t in [x["domain"]] + [p["problem"] for p in x["problems"]]:
                carried[hashlib.sha256(t.encode()).hexdigest()] = t
    except (OSError, ValueError, IndexError):
        pass
    return carried


def resolve(src, carried):
    if "file" in src:
        with open(os.path.join(ROOT, src["file"]), encoding="utf-8") as f:
            t = f.read()
        n = src.get("trail", 0)
        if n:
            if not t.endswith("\n" * n):
                sys.exit("%s lost its %d trailing newline(s)" % (src["file"], n))
            t = t[:-n]
        return t
    t = carried.get(src["inline_sha256"])
    if t is None:
        sys.exit("inline entry %s not recoverable from examples.js" % src["inline_sha256"][:12])
    return t


def render():
    carried = load_carried()
    out = []
    for e in SPEC:
        x = {k: e[k] for k in ("key", "label", "note")}
        x["domain"] = resolve(e["domain"], carried)
        x["problems"] = []
        for p in e["problems"]:
            q = {k: v for k, v in p.items() if k != "problem"}
            q["problem"] = resolve(p["problem"], carried)
            x["problems"].append(q)
        out.append("  " + json.dumps(x, separators=(",", ":"), ensure_ascii=False))
    return HEAD + ",\n".join(out) + "\n];\n"


def main():
    new = render()
    if "--check" in sys.argv:
        cur = open(OUT, encoding="utf-8").read()
        if cur != new:
            sys.stderr.write("examples.js is stale: run scripts/gen_examples_js.py\n")
            sys.exit(1)
        print("examples.js up to date (%d domains)" % len(SPEC))
        return
    with open(OUT, "w", encoding="utf-8") as f:
        f.write(new)
    print("wrote %s (%d bytes)" % (OUT, len(new.encode())))


if __name__ == "__main__":
    main()
