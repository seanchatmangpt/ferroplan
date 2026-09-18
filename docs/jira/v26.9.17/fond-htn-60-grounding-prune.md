---
id: fond-htn-60-grounding-prune
type: oslc_cm:ChangeRequest
requirement: earl:TestRequirement
dcterms:title: "Fix: reachability pruning for >1M-instance groundings (16 IPC domains stuck at caps)"
standing: BLOCKED
branch: fix/grounding-prune
worktree: ~/ferroplan-worktrees/wt-h60
created: 2026-09-18T02:50:00Z
source: fond-htn-43 addendum (cap-raising alone insufficient)
---

Read `_WAVE4-CONTEXT.md`. Ticket 43's measurement: with caps at 1M, 16 IPC-2023 domains still refuse — true ground-instance counts exceed 1,000,000 (refusals at 8–29 s). The leverage is `GroundingLimits::prune_unreachable` (currently default-off) and/or relevancy pruning: most instances are unreachable in these hierarchical domains.

Scope:
1. Audit the existing `prune_unreachable` path (grounder): what it prunes, why it is off by default (soundness or precision note), and what it costs.
2. Make it (or a new goal/task-relevance fixpoint) good enough to bring the 16 stuck domains' ground-instance counts under the default 10k envelope where sound — hierarchical relevance: iteratively keep only actions/methods on some path from the initial task network to primitive effects consumed by preconditions downstream (standard HTN relevance pruning; cite the public technique in a comment, implement fresh).
3. Re-run the 16 domains (read-only /tmp corpus at /tmp/fond-review/HDDL-Parser/tests/ipc/) at DEFAULT caps; append outcomes to `tests/fixtures/ipc-sweep/RESULTS-wavec.md` (append-only).
4. Soundness falsifier: the full existing suite (micro, oracle agreement, canonical) stays green — pruning may only remove provably-irrelevant instances; any behavior change beyond count reduction is a finding.

Gates: `cargo test -p ferroplan-hddl && cargo test -p ferroplan --test htn_ipc2023 --test htn_oracle` exit 0; addendum updated.

## History
| ts | standing | branch+SHA | gates+exits | remaining |
|---|---|---|---|---|
| 2026-09-18T02:50:00Z | BLOCKED | fix/grounding-prune @ 75870de | — | all |
| 2026-09-18T23:30:00Z | PARTIAL_ALIVE | wt-h60 wip landed by A7 as b03ea19 on wave6/land-v26917 (branch fix/grounding-prune had no commits; uncommitted worktree diff applied + attributed) | gates re-run in landing worktree: ferroplan-hddl 178/178 exit 0; htn_ipc2023 13/13; htn_oracle 3/3 (6 documented ignores); ground_caps_plumbing + memory_stress re-pins green in the 62 sweep | remains: 16-domain default-caps re-run (grounding_prune_ipc_rerun #[ignore]d long-run) — corpus /tmp wiped on this machine, EXTERNAL_ABSENT; RESULTS-wavec.md append follows that re-run |
