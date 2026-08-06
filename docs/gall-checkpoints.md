# Gall Checkpoints for the Chatman Ecosystem

Last updated: 2026-07-29 (session audit, see "Audit log" at the end).

Every checkpoint has to earn its keep at its own scale — no exceptions, no
credit for showing up. Source on disk buys nothing by itself. A checkpoint
clears only when the behavior it claims actually runs, fails the way it's
supposed to when pushed, and leaves evidence someone else can replay.

Standing vocabulary (see `~/.claude/rules/no-overclaiming-rust.md` for the
full discipline this repo runs under): `ALIVE`, `PARTIAL_ALIVE`, `BLOCKED`,
`MOCKED`, `REFUSED`, `UNSUPPORTED`, `UNKNOWN`. A standing may only be
upgraded on exhibited evidence (a command, its output, and what it proves) —
never on source presence alone.

## How to use this file (for any agent picking up work here)

1. Check the "Current standing" line before you touch anything. Don't
   reopen a verdict without new evidence in hand.
2. Pull the next open item off "Recommended Release Sequence" unless a
   specific checkpoint was named.
3. Do the work for real: run the command, read what comes back, write the
   standing down with the exact evidence behind it. No-overclaiming holds
   here — a standing is a claim, and every claim needs a receipt.
4. Log it. Dated entry at the end of "Audit log": what you tried, what you
   found, what moved. Prior entries stay untouched.
5. Anything you build — a script, a vendored tool, a fixture — stays in the
   repo, in its proper place, with a path back to it from here.
6. Never wave a partially-exercised surface through to `ALIVE`.
   `PARTIAL_ALIVE` with the exact blocking hop named beats a false `ALIVE`
   every time.

---

## 0. Constitutional Vocabulary

**Working system**

One vocabulary, held stable across the whole ecosystem, covers:

* observation;
* admission;
* allocation;
* planning;
* manufacture;
* validation;
* actuation;
* receipt;
* refusal;
* standing.

Core laws:

```text
A = μ(O*)
zero unreceipted actuation
source presence ≠ execution evidence
candidate plan ≠ validated plan
grant ≠ execution
```

**Falsifier**

Two repositories use the same term for incompatible objects or authority levels.

**Current standing:** `ALIVE`

> **2026-07-29 cycle update (CE-GALL-23).** The ceiling drops, the standing
> holds. One declared invariant (`validated-plan-requires-candidate`) turned
> out to be dead weight — it carried `requires_any_prior`, a key
> `validate_vector` never reads. "Invariants reject illegal combinations" was
> half theater. Cut it. The lawful count sits unchanged at 136, and that
> unchanged number is the proof the invariant was never doing anything.
> `tests/test_phase_space.py::test_every_invariant_key_is_understood` now
> stands guard against the same rot coming back.


---

## 1. Phase-Space Kernel

**Working system**

Six dimensions, multiplied together, one state:

```text
epistemic
× allocation
× planning
× actuation
× drift
× conformance
```

Every transition is named out loud. Anything off-map gets turned away at the
door. Touch the repository and advanced standing drops back to earth.

**Required proof**

* Every state validates.
* Every declared transition executes.
* Every undeclared transition refuses.
* Invariants reject illegal combinations.
* The manufacturer is active only during `actuation=manufacturing`.

**Current standing:** `ALIVE` for source-law and fixture scope. Watched it
happen live in the 2026-07-29 audit: the `PostToolUse` hook snapped the
canonical phase vector back to baseline the instant a new observation event
landed — no explicit `phase.py transition` call, nobody asked it to.
"Repository mutation collapses advanced standing" fires on its own, not by
polite agreement.

---

## 2. Claude Projection Loads

**Working system**

Drop the marketplace and plugin into a clean Claude Code environment and
watch what surfaces.

Claude Code has to find:

* plugin manifest;
* agents;
* skills;
* hooks;
* monitors;
* MCP server;
* plugin settings;
* user configuration.

**Required proof**

```text
clean plugin cache
→ marketplace add
→ plugin install
→ plugin validate
→ session start
→ no loader errors
```

**Falsifier**

Any declared component is missing, rejected, duplicated, or silently ignored.

**Current standing:** `PARTIAL_ALIVE` (was `UNKNOWN`)

2026-07-29 audit, what the trace showed:
- `claude plugin validate --strict` passes for both the plugin manifest and
  the marketplace manifest.
- All 8 declared agent files, `.mcp.json`, `.lsp.json`,
  `monitors/monitors.json`, `skills/` resolve on disk — no missing,
  duplicated, or silently-ignored component.
- `claude plugin list` shows `chatman-ecosystem@chatman-ecosystem`
  `✔ enabled` in both project and user scope, no loader errors, in the live
  running session.
- **Open defect found**: the installed marketplace clone
  (`~/.claude/plugins/marketplaces/chatman-ecosystem`) is stale/orphaned
  relative to canonical `origin/main`. It sits at commit `75bb6ee`, which
  `git merge-base --is-ancestor 75bb6ee origin/main` confirms is **not an
  ancestor** of the current `origin/main` (`d047fd9` at audit time). Files
  adopted from PR #2 into `main` (`scripts/effective-phase.py`,
  `scripts/actuation-intent.py`, `scripts/grant-actuation.py`,
  `ontology/authority-graph.ttl`) exist in the source repo but are **absent
  from the plugin cache this session actually runs against.**
- Not exercised: a true clean-cache install
  (`marketplace add → install → validate → session start` from an empty
  cache). That requires spawning a separate Claude Code process/cache,
  outside a single session's tool surface — named as the exact blocking hop,
  not silently skipped.

**Next step**: run the marketplace-clone refresh path again (`claude plugin
update chatman-ecosystem` or equivalent), confirm it actually pulls
`d047fd9` or later, then re-run this checkpoint from a genuinely clean cache
— may need an outside harness, a throwaway container, a fresh `$HOME`.

---

## 3. Mechanical Agent Authority

**Working system**

Role ceilings are supposed to hold by machinery, not manners.

* Controller routes but cannot edit.
* Observer observes but cannot edit.
* Allocator allocates but cannot plan or edit.
* Planner plans but cannot edit.
* Validator validates but cannot repair.
* Auditor audits but cannot publish.
* Manufacturer is the sole source editor.
* Manufacturer runs in a worktree.

**Required proof**

Push a direct edit from every non-manufacturing agent, watch it bounce.

Push a manufacture call outside `actuation=manufacturing`, watch it bounce.

**Current standing:** `PARTIAL_ALIVE`

2026-07-29 audit, what the trace showed:

> **2026-07-29 cycle update (CE-GALL-27).** The first bullet below no longer
> holds. `agents/*.md` frontmatter is generated from
> `ontology/authority-graph.ttl` now, so all 8 agents declare `tools:` and the
> source-manufacturer declares `isolation: worktree`. The ODRL
> `SingleActuatorPolicy` checks out non-vacuous under
> `tests/test_authority.py::test_single_actuator_policy_is_enforced`: it permits
> exactly `source-manufacturer`, prohibits 7, and exactly `source-manufacturer`
> can write. **Standing does not move.** The live test below — whether the
> *harness* refuses or the *model* just decides not to — hasn't been re-run
> against the generated frontmatter. "Mechanical, not prompt-level" is still
> asserted, not measured. That single re-run is now the whole gap.

- None of the 8 agent `.md` files under `plugins/chatman-ecosystem/agents/`
  declare a `tools:` frontmatter field. Confirmed the hard way, by this
  session's own Agent-tool listing, which tags every one of the 8
  chatman-ecosystem agents `(Tools: All tools)`. No mechanical denial exists
  at the Claude Code harness level — none.
- Live test: spawned `rdf-observer` (its own prose reads "You do not edit
  source, execute plans, or authorize actuation") and pointed it at a
  throwaway file outside the repo. It refused — but on its own recognizance,
  reading the instruction as suspicious and declining, not because the
  harness ever blocked the `Edit` call. A different model, a different mood,
  and that edit goes through with nothing standing in the way.
- Read on it: role separation right now is **prompt-level compliance**, not
  **mechanical enforcement**. The checkpoint's own name — "Mechanical Agent
  Authority" — isn't earned by what sits in `main`.
- PR #2 (`agent/v26.7.29-claude-projection`, still open/draft, not merged)
  proposes exactly this fix: every agent declaring `tools:`, denying
  `Write`/`Edit`/`NotebookEdit` to everyone but `source-manufacturer`
  (isolated in a worktree). See PR #2 status below for why it's still stuck.

**Next step**: add `tools:` allow/deny lists to each of the 8 agent
frontmatter files — the smallest cut of PR #2's rewrite that would actually
move this checkpoint — then re-run the same live refusal test. This time
the expectation is a harness-level tool-permission error, not a model's
change of heart.

---

## 4. Bounded Lifecycle Observation

**Working system**

Claude's hooks throw off observation candidates at every seam:

* startup;
* resume;
* clear;
* compact;
* fork;
* tool success;
* tool failure;
* tool batch completion;
* configuration change;
* worktree creation;
* worktree removal;
* session stop.

Hooks do not directly manufacture semantic truth.

```text
hook event
→ observation candidate
≠ admitted phase transition
```

**Required proof**

Every supported event emits a deterministic candidate with stable identifiers and digests.

**Falsifier**

A hook advances canonical phase state without admission.

**Current standing:** `PARTIAL_ALIVE`

2026-07-29 audit note: watched `PostToolUse` fire on every Bash/Edit/Write
call this session, no exceptions, *regardless of whether the mutation ever
touched the tracked repo* — a `Bash` call scribbling into `/tmp` still threw
a ledger event. Defensible: bounded observation, not scoped filtering. Still
worth flagging — it means the pending-event count can carry events with zero
actual repo diff, and the observation/replan cycle has to eat those cleanly.
It does: `session_observe` came back `fact_surprises: []` and
`remaining_plan_valid: true` on the no-diff events, every time.

---

## 5. Effective Phase Projection

**Working system**

Canonical phase state gets folded together with whatever's still pending.

One pending mutation and the effective state drops straight to:

```text
observed
× unallocated
× unplanned
× sealed
× drifted
× unknown
```

no matter what an older snapshot swears to.

**Required proof**

1. Advance the canonical state.
2. Emit an unadmitted mutation event.
3. Verify that effective state collapses.
4. Admit the event frontier.
5. Verify that state can advance again only with new evidence.

**Current standing:** `ALIVE` for unit-fixture scope — and run live,
end-to-end, in the 2026-07-29 session, not just against fixtures. Pushed
the canonical vector to `receipted/stable`, made a real commit, watched
`PostToolUse` snap the canonical vector back to baseline on its own, then
closed the loop twice in the same session
(`session_observe` → `session_think` → CMCA →
`bind_allocation_receipt` → `validate` → `bind_plan_receipt` →
`loop.py admit` → `phase.py transition`) — once over a real source commit,
once over a no-diff `/tmp` Bash observation. Both cycles landed clean: a
0-pending ledger, a `stable` phase vector.

---

## 6. Generated Artifact Ownership

**Working system**

Every generated Claude projection artifact carries its own paperwork:

* canonical owner;
* generator identity;
* source digest;
* projection digest;
* regeneration command;
* mutation policy.

The generated guard reads the ownership registry rather than a hard-coded file list.

**Required proof**

* Direct edit of a generated artifact refuses.
* Editing its canonical source permits regeneration.
* Regeneration produces deterministic output.
* Repeated generation is byte-identical.

**Falsifier**

A tracked projection can be hand-edited without changing its admitted source.

**Current standing:** `PARTIAL_ALIVE`

Ownership and refusal law are on the books. Full ggen generation and receipt binding are still open ground. Not re-audited in the 2026-07-29 pass.

---

## 7. Combined Ferroplan MCP Authority

**Working system**

One stdio MCP server, and it carries the whole bounded tool surface:

* parsing;
* solving;
* validation;
* decomposition;
* persistent sessions;
* observation;
* bounded thinking;
* CMCA;
* canonical digests;
* allocation receipts;
* plan receipts;
* receipt verification.

**Required proof**

```text
initialize
→ tools/list
→ resources/list
→ invoke all tools
→ malformed-input refusals
→ clean shutdown
```

**Current standing:** `ALIVE` for compile and test scope.

`cargo check --workspace` and `cargo test --workspace` came back green
every time it was thrown in the 2026-07-29 session, before the commit and
after it. Every MCP tool actually used this session
(`session_open`/`session_observe`/`session_think`/`session_status`,
`cmca_allocate`, `bind_allocation_receipt`, `bind_plan_receipt`, `validate`,
`verify_receipt`) behaved as documented, including refusing malformed input
(out-of-bounds `parent` index, cyclic `parent` ancestry, tampered receipt).

---

## 8. Top-Level CMCA Allocation

> **2026-07-29 cycle update (CE-GALL-28) — partial retraction.** Bad ground.
> The prior evidence for the 8×10 happy path being "exercised repeatedly with
> real receipts" turns out to have been run over a **fabricated** frontier —
> a surface that doesn't even exist in the repository. Withdrawn. In its
> place: the canonical frontier from `profiles/work-surfaces.json`
> (`candidates_digest a473833974c74522`), accepted live, allocating
> *differently* from what was claimed. The four refusals below still haven't
> been tested at the allocator — `surfaces.py`'s refusals fire pre-flight and
> never reach them.


**Working system**

An admitted repository observation kicks out exactly:

```text
8 candidates × 10 factors
```

CMCA returns bounded shares and binds:

* candidate array;
* factor order;
* allocation output;
* BCINR-CMCA revision;
* observation frontier;
* predecessor receipt.

**Required proof**

* Exactly eight candidates accepted.
* Seven or nine candidates refused.
* Wrong factor count refused.
* Wrong BCINR revision refused.
* Tampered allocation result refused.
* Repeated input produces identical allocation evidence.

**Current standing:** `PARTIAL_ALIVE`

The 8-candidate/10-factor happy path ran clean, repeatedly, this session —
real allocation receipts, bound and admitted. The refusal cases (7/9
candidates, wrong factor count, wrong BCINR revision, tampered allocation
result) weren't **all** individually re-checked in the 2026-07-29 pass —
only the receipt-tamper case (Checkpoint 19) and CMCA's own
parent-index/cycle refusals (Checkpoint 9) were.

**Next step**: run the four untested refusal cases, on the record, before
this moves past `PARTIAL_ALIVE`.

---

## 9. Recursive Multifractal Allocation

**Working system**

Any admitted CMCA node can turn around and become the root of another
eight-node frontier.

```text
parent allocation
→ selected node
→ local observation
→ eight local candidates
→ local allocation
→ local receipt
→ consequence returned upward
```

Every descent binds the parent allocation receipt. Every return binds the local result.

**Required proof**

* Depth one allocation.
* Depth two allocation.
* Parent receipt mismatch refusal.
* Cyclic ancestry refusal.
* Missing return consequence refusal.
* Deterministic replay at each depth.

**Current standing:** `PARTIAL_ALIVE`

2026-07-29 audit, what the trace showed:
- `cmca_allocate` takes per-candidate `parent` indices inside a single call
  and builds a real tree out of them — interior (parent) nodes come back
  `share: 0`, all the allocation mass cascades down to leaf nodes. Genuine,
  confirmed behavior, checked not assumed.
- Out-of-bounds parent index gets turned away: `"candidate \`orphan-bad-parent\`
  has invalid parent 99"`.
- Cyclic parent chain gets turned away: `"parent relation contains a cycle
  through 0"`.
- **Gap found**: `bind_allocation_receipt`'s only chaining field is a flat
  `previous_receipt` — a sequential predecessor, nothing more. No
  parent-allocation-receipt field, no "selected node" field, no
  "consequence returned upward" field anywhere. True cross-call recursive
  descent — what the checkpoint's "Working system" diagram actually asks
  for — is **architecturally absent from the MCP tool schema**. Not
  untested. Absent. The in-array tree support above is real, but it's a
  narrower, different animal than what this checkpoint wants.

**Next step**: decide whether recursive CMCA gets modeled as (a) a new MCP
tool/field for parent-receipt-bound descent, or (b) written down as
out-of-scope with the checkpoint's "Working system" text narrowed to match
what's actually there (single-call tree allocation). Either way, close the
mismatch — don't leave it hanging.

---

## 10. MFW/POWL Planner Routing

**Working system**

MFW or POWL v2 calls which planner rail gets to answer a planning request.

Ferroplan is one deterministic implementation working under that law — not the law itself.

```text
admitted planning request
→ planner selection
→ Ferroplan candidate
→ validation
→ promotion or refusal
```

**Required proof**

* Planner identity and version are bound.
* Routing is deterministic for the same admitted request.
* Unsupported domains produce typed refusal.
* A candidate plan cannot self-promote.

**Current standing:** `UNSUPPORTED`

Direct Ferroplan planning is real. Constitutional planner routing isn't wired yet. Not re-audited in the 2026-07-29 pass — standing unchanged.

---

## 11. Isolated Source Manufacture

**Working system**

One admitted plan step, run inside an isolated Git worktree, sealed off from the rest.

The manufacturer touches only:

* the selected plan step;
* tightly coupled generated outputs;
* explicitly admitted dependencies.

**Required proof**

* Worktree is created.
* Exact base commit is recorded.
* Change remains inside admitted scope.
* Main working tree remains untouched.
* Worktree cleanup is deterministic.
* Mutation emits a new observation candidate.
* Advanced standing collapses after manufacture.

**Current standing:** `UNSUPPORTED` (was `UNKNOWN`)

2026-07-29 audit: swept `plugins/chatman-ecosystem/` and found no
worktree-related script, profile, ontology file — nothing. This isn't
"untested." There's no mechanism to test in the first place. Closest thing
on the horizon is PR #2's still-unmerged "Isolate and bound the source
manufacturer agent" commit (`7bb5239ce7922e5c790080ed3ec0c0d9ecaa4771`),
absent from `main`. This session's actual manufacturing step (the
`.claude/settings.json` model pin) went straight into the main working
tree, no isolated worktree involved — consistent with "not built yet," not
a defect in the work itself.

**Next step**: either adopt PR #2's worktree-isolation commit (would need
its own review given it also changes agent tool grants — see Checkpoint 3),
or write a standalone `scripts/manufacture-in-worktree.py` that: creates a
worktree at the current HEAD, records the base commit SHA, applies exactly
the admitted plan step's diff, runs build+test inside the worktree, and
either merges back (fast-forward only) or reports failure without touching
the main tree.

---

## 12. Verification Ladder

**Working system**

Evidence climbs, rung by distinct rung:

```text
unit
→ integration
→ end-to-end
→ chaos
→ stress
→ benchmark
→ independent validator
```

Each rung has its own executor and claim ceiling.

**Required proof**

* Lower-rung success cannot imply higher-rung success.
* Failed checks remain failed.
* Unavailable executors produce `UNKNOWN`.
* Independent validation records executable identity and input digests.

**Current standing:** `PARTIAL_ALIVE`

Projection fixtures and MCP tests read green across the board. The full ladder is still short a few rungs. Not re-audited in the 2026-07-29 pass beyond what Checkpoint 13 (VAL) newly unlocks.

---

## 13. Independent PDDL Validation

> **2026-07-29 cycle update (CE-GALL-30) — downgraded.** Standing drops to
> `PARTIAL_ALIVE`, reason `MOCKED`. MCP `validate` hands back the prose string
> `"Plan valid"`, but `bind_plan_receipt` wants a boolean `valid` — so someone
> constructs the verdict by hand, exactly as `skills/admit/SKILL.md:15`
> instructs. Every `validator_result` bound this cycle was hand-fabricated.
> "Independent" is currently false in the receipt path.


**Working system**

A planner-independent validator — VAL, say — checks the exact emitted plan against the exact domain and problem. No trust extended.

Ferroplan replaying its own work is useful. It is not independent evidence.

**Required proof**

* Valid plan accepted.
* Invalid plan refused.
* Tampered plan refused.
* Domain or problem digest mismatch refused.
* Validator executable identity is recorded.
* Validator output is bound into the receipt.

**Current standing:** `PARTIAL_ALIVE` (was `UNSUPPORTED`)

2026-07-29 audit: vendored and built the real thing — independently-sourced
VAL (`KCL-Planning/VAL`) via `benchmarks/get-val.sh`, landing at
`benchmarks/.val/VAL/build/bin/Validate` (gitignored, self-contained). Its
pinned CMakeLists wouldn't configure against current cmake without
`-DCMAKE_POLICY_VERSION_MINIMUM=3.5` — worth patching `get-val.sh` to pass
that flag by default so the next run doesn't hit the same wall.

Pointed the built `Validate` binary at this session's actual bound
domain/problem/plan — no toy fixture:
- Valid plan → `Plan valid`, exit 0.
- Reordered/tampered plan (same actions, wrong order) → `Plan failed to
  execute`, exit 1.
- Truncated plan (goal not reached) → `Goal not satisfied` / `Plan
  invalid`, exit 1.
- Mismatched problem (wrong init state) → `Plan failed to execute`, exit 1.

All four required behaviors hold, and hold with genuine engine independence
— this is real, not Ferroplan grading its own homework.

**Not yet done**: wiring VAL into the release loop, and binding VAL's
output — not Ferroplan's own `validate` — into the `validator_result` field
of a bound receipt envelope. `validator_result_digest` in every receipt
bound so far still points back to `ferroplan.validate`, not VAL.

**Next step**: patch `get-val.sh` with the cmake policy flag; add a
`FERROPLAN_VAL` env-var check to whatever produces `validator_result`
payloads so VAL's output, when it's there, is what actually gets bound.

---

## 14. Canonical Admission Receipts

> **2026-07-29 cycle update (CE-GALL-31) — sharpened into a refutation.** Not
> "not re-verified." **Absent.** `verify_chain` doesn't exist. `previous_receipt`
> gets format-checked and nothing more — 64 hex characters, never looked up
> against anything real — so any well-formed hex string chains cleanly, and
> `None` looks identical to a break.


**Working system**

Allocation and plan evidence get folded into canonical BLAKE3 envelopes, sealed shut.

A plan receipt binds:

* admitted observation frontier;
* allocation receipt;
* planner identity;
* domain and problem;
* candidate plan;
* independent validator result;
* predecessor receipt.

**Required proof**

* Canonicalization is deterministic.
* Payload digest recomputes.
* Receipt recomputes.
* Wrong predecessor refuses.
* Reordering refuses or canonicalizes identically.
* Payload-only tampering refuses.
* Chain forks are detected.

**Current standing:** `PARTIAL_ALIVE`

Core MCP receipt tests pass. `verify_receipt` recomputation and tamper
detection reconfirmed live in the 2026-07-29 audit (Checkpoint 19).
Wrong-predecessor and fork-detection cases weren't individually re-checked
this pass — carried over from the prior standing.

---

## 15. Structured BRCE Intent

**Working system**

A protected command doesn't run — it gets translated into an exact `ActuationIntent` carrying:

* actor;
* operation;
* target;
* argument digest;
* expected preconditions;
* required receipt;
* authority;
* reversibility;
* requested consequence.

The instant the intent exists, the original call is dead.

**Required proof**

* Protected command creates an intent.
* Intent digest is deterministic.
* Original call does not execute.
* Unprotected commands do not create false protected intents.
* Equivalent commands canonicalize consistently.

**Current standing:** `ALIVE` for fixture scope.

2026-07-29 audit: `scripts/actuation-intent.py` and
`scripts/grant-actuation.py` sit in the source repo, adopted from PR #2 per
`docs/notes/pr2-claude-projection-ideas-adopted.md` — but they're **absent
from the installed plugin cache** this session actually runs against, and
**not wired into `hooks.json`**. Standing holds at fixture scope. Source
presence still isn't execution evidence, no matter how tempting.

---

## 16. Derived Execution Grant

**Working system**

A separate admission step checks the intent against:

* current effective phase;
* admitted receipt frontier;
* validator evidence;
* authority graph;
* user authorization;
* scope constraints.

Clear that, and it cuts a short-lived `DerivedExecutionGrant`.

**Required proof**

* Missing receipt refuses.
* Stale phase refuses.
* Pending observations refuse.
* Wrong command digest refuses.
* Expired grant refuses.
* Reused grant refuses.
* Grant cannot change intent scope.

**Current standing:** `PARTIAL_ALIVE`

Grant construction exists (`scripts/grant-actuation.py`, unwired — see
Checkpoint 15). Live Claude execution hasn't been run against it yet.

---

## 17. Protected Actuation Execution

**Working system**

The exact protected operation gets one more shot — this time riding the exact verified grant.

Examples:

* Git push;
* draft PR creation;
* merge;
* package publication;
* destructive filesystem operation;
* state-changing HTTP call.

**Required proof**

* Exact command succeeds with valid grant.
* Modified command refuses.
* Missing grant refuses.
* Expired grant refuses.
* Scope expansion refuses.
* The executor records actual exit status and effects.

**Current standing:** `UNKNOWN`

Not attempted in the 2026-07-29 pass — there's no execution pipeline to point at yet; Checkpoints 15/16 have to be wired first.

---

## 18. Execution Attestation

**Working system**

Actual execution leaves a mark — an `ExecutionAttestation` binding:

* grant;
* executor identity;
* command digest;
* start and completion time;
* exit status;
* stdout/stderr commitments;
* resulting object identifiers;
* resulting repository state.

```text
grant ≠ execution
execution attestation = evidence of consequence
```

**Required proof**

A valid grant with no execution cannot produce an attestation.

A failed execution produces a failure attestation, not success.

**Current standing:** `UNSUPPORTED`

No attestation object type, no executor — neither exists yet. Unchanged from prior audit.

---

## 19. Receipt-Chain Replay

> **2026-07-29 cycle update.** New evidence: a five-link chain
> (`755a2057 → c1520c61 → d56006af → eb8e4645 → d72f17f0`), the last four links
> bound over canonical CMCA inputs and `project-world.py`'s live projection.
> New refutation: "a forked predecessor refuses" is **false** — see
> CE-GALL-31. Tamper detection on a single link still stands.


**Working system**

Run it back from genesis — the complete chain replays:

```text
observation
→ admission
→ allocation
→ planning
→ manufacture
→ validation
→ intent
→ grant
→ execution
→ attestation
```

The mutable phase snapshot never gets trusted as anything more than a cache.

**Required proof**

* Replay reconstructs the same state.
* Missing event refuses.
* Reordered event refuses.
* Forked predecessor refuses.
* Tampered payload refuses.
* Snapshot disagreement is detected.
* Rebuilding the cache produces the same phase vector.

**Current standing:** `PARTIAL_ALIVE`

2026-07-29 audit: ran `verify_receipt` on a real, session-bound plan
envelope — `valid: true`, both `payload_digest` and `receipt` recomputing
exactly. Zero out just the `receipt` field on that same envelope and it
comes back `payload_digest_valid: true, receipt_valid: false, valid: false`
— tamper detection confirmed on live data, not a fixture. Full cross-system
replay — observation through attestation, the entire chain — still doesn't
exist, since the intent/grant/execution/attestation legs (15–18) are only
partially wired.

---

## 20. Closed Self-Hosting Loop

> **2026-07-29 cycle update — net honest downgrade.** Two things pulling in
> opposite directions. Strengthened: two further closes over canonical
> inputs and the live world projection, with `session_observe` →
> `session_think` returning `decision: follow`, `searched: false` — a suffix
> held without a search, real evidence of a working persistent mind. **But**
> this checkpoint's required proof is a traversal "without manual phase
> fabrication," and both closes fabricated the validator verdict
> (CE-GALL-30), nine manual steps apiece, because `loop.py close` still
> isn't built. Read the earlier claim about prior closes meeting this bar
> with that same qualification hanging over it.


**Working system**

Ferroplan turns the Chatman ecosystem on itself:

```text
observe Ferroplan
→ allocate frontier
→ plan
→ manufacture in worktree
→ observe drift
→ validate
→ admit
→ audit
→ publish draft PR
→ attest execution
→ replay
```

No role bleeds into another.

**Required proof**

One complete repository change traverses the loop without manual phase fabrication or unreceipted protected actuation.

**Current standing:** `PARTIAL_ALIVE`

2026-07-29 audit: this session ran the full observe → allocate → plan →
manufacture → observe-drift → validate → admit loop **twice**, end to end,
against two different repository mutations — a real `.claude/settings.json`
commit, and a no-diff Bash observation. Both produced bound, verifiable
receipts and a `stable/receipted` phase vector with a 0-pending ledger. The
strongest evidence yet for this checkpoint's core claim. Still missing,
against the checkpoint's own diagram: worktree-isolated manufacture
(Checkpoint 11), draft-PR publication under a structured intent/grant
(Checkpoints 15–17), execution attestation (Checkpoint 18). The loop that
exists is real. The loop as specified is not yet whole.

---

## 21. v26.7.29 Crown

**Working system**

One exact release commit, carrying proof of the complete lawful Claude projection.

Required crown evidence:

1. Clean marketplace installation
2. Strict plugin validation
3. Agent authority refusals
4. Lifecycle candidate generation
5. Effective-phase collapse
6. Top-level CMCA allocation
7. Recursive CMCA allocation
8. Deterministic candidate plan
9. Isolated worktree manufacture
10. Projection regeneration
11. Independent VAL validation
12. Receipt binding
13. Tamper replay
14. Structured protected intent
15. Derived execution grant
16. Draft PR publication
17. Execution attestation
18. Full-chain replay

**Current standing:** `PARTIAL_ALIVE`

PR #2 (`agent/v26.7.29-claude-projection`) is the only draft going after
this whole surface at once. As of the 2026-07-29 audit it's still
`OPEN`/draft, 0 reviews, head commit `d88488608f41` (55 commits), CI mixed:
the `Chatman Ecosystem` workflow's `projection-law` and `ferroplan-mcp` jobs
pass, but the plain `CI / test` job reads `FAILURE`. Not touched further
this pass — resolve the CI failure and get the PR reviewable before it gets
treated as the crown vehicle.

---

# Recommended Release Sequence

Next bounded checkpoints, in order:

```text
1. Clean Claude installation
2. Live agent-authority refusal tests
3. Worktree manufacture
4. VAL integration
5. Recursive CMCA runtime
6. Full receipt replay
7. Intent/grant protected publication
8. Execution attestation
9. Closed self-hosting loop
10. v26.7.29 crown
```

The decisive rule, no exceptions:

> **Do not build the crown directly. Make each checkpoint independently useful, independently falsifiable, and reusable by the next checkpoint.**

---

# Checkpoints 22–33 — the DX architecture cycle

Twelve checkpoints, all landed in the 2026-07-29 architecture cycle (branch
`chatman-dx-cycle`). Every one sits at `PARTIAL_ALIVE` or lower, and every one
is snagged on the same single hop: **no clean-worktree replay outside the
originating session has been done, and nothing is pushed.** The promotion
law bars `ALIVE` on that alone, however green the suite reads — which is why
clearing it is one action, not twelve.

This isn't a policy on paper. It's mechanized:
`plugins/chatman-ecosystem/tests/test_receipts.py` refuses any receipt claiming
`ALIVE` without `replayed_outside_session`, a non-null `negative_falsifier`, and
a sealed commit — and `test_promotion_law_actually_refuses` is that check's own
falsifier, watching the watcher.

---

## Control Plane Executable Under Test (CE-GALL-22)

**Working system**

The Python control plane is a tested surface now — and a test that reaches for the live ledger gets refused, not just frowned at.

Before this, the plugin ran zero tests and CI never so much as glanced at
`plugins/`: nine scripts, ~2.5k lines total, "verified" by a prose checklist
that `py_compile`d three of them and called it a day.

**Current standing:** `PARTIAL_ALIVE` (`NO_FALSIFIER`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-22.json`

**Positive witness:** `tests (whole suite)` (plugins/chatman-ecosystem/tests) — the Python control plane went from zero tests and zero CI coverage to a suite gating every change

**Negative falsifier:** none. Recorded, not hidden — a checkpoint without an executing negative fixture cannot be promoted.

- Non-claim: the autouse isolation fixture is an assertion, not a falsifier: no test deliberately leaks, so it has never fired
- Non-claim: the CI `plugin` job has never run -- the branch is unpushed

---

## Derived Combination Census (CE-GALL-23)

**Working system**

An invariant that reads a key no evaluator ever touches isn't an invariant — it's dead code wearing a badge. The lawful-vector count has to be *derived* from the invariant set, never asserted alongside it as a separate act of faith.

`validated-plan-requires-candidate` carried `requires_any_prior`, a key
`validate_vector` never reads. The naive repair — renaming it to `requires_any`
— would have been wrong: `planning` is single-valued, so requiring
`planning=candidate` while `planning=validated` is unsatisfiable, and the rule
would have forbidden every validated vector. The transitions table already
enforces the intent exactly (`["candidate","validated"]` is the only in-edge),
so the invariant was redundant as well as inert.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-23.json`

**Positive witness:** `test_lawful_count_is_pinned` (plugins/chatman-ecosystem/tests/test_phase_space.py) — 648 raw / 136 lawful / exactly 1 publishable, all derived rather than asserted beside the invariants

**Negative falsifier:** `test_every_invariant_fires_at_least_once` (plugins/chatman-ecosystem/tests/test_phase_space.py) — re-adding the deleted validated-plan-requires-candidate invariant (key requires_any_prior, never read by validate_vector) makes this fail. The lawful count staying at 136 after deletion is independent proof the invariant was inert

- Non-claim: nothing external validates that the 136 lawful vectors are the *right* 136

---

## Machine-First Output Contract (CE-GALL-24)

**Working system**

A payload's `schema` URN is stamped on at construction, rejected on mismatch — the model's identity, not a string some caller gets to hand it. JSON is the default serialization, blind to tty, so the contract reads identical whether a human, a hook, or CI is the one calling.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-24.json`

**Positive witness:** `test_emitted_payload_validates_against_its_committed_schema` (plugins/chatman-ecosystem/tests/test_generated.py) — what is emitted satisfies what is published, for every registered model

**Negative falsifier:** `test_check_detects_a_tampered_projection` (plugins/chatman-ecosystem/tests/test_generated.py) — proves generate.py build --check is not a no-op; verified by hand against a tampered schema, which exited 1

- Non-claim: 6 of roughly 30 emitted payloads are registered; the coverage ratio is measured nowhere and is left UNKNOWN

---

## Fail-Closed Hook Guard (CE-GALL-25)

**Working system**

Any exception thrown before a hook handler even starts turns into a refusal *shaped for the event it's standing in for* — never a bare traceback, never a silent exit 0 sneaking through on a deny path.

The shapes aren't interchangeable, and getting one wrong turns a refusal into
a no-op: `Stop` wants a top-level `decision`, `PreToolUse` wants a nested
`permissionDecision`, `PostToolUse` can't refuse at all. The guard imports
nothing but the standard library — it's the last line standing when
everything else has already failed to load.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-25.json`

**Positive witness:** `test_guard_uses_only_the_standard_library` (plugins/chatman-ecosystem/tests/test_hookguard.py) — the last line of defence cannot itself fail on the dependency it is guarding against

**Negative falsifier:** `test_import_failure_produces_a_refusal` (plugins/chatman-ecosystem/tests/test_hookguard.py) — a simulated ImportError yields a refusal shaped for the event, never a traceback and never a silent exit 0 on a deny path

- Non-claim: no live Claude Code session has been observed honoring a hookguard refusal; runtime acceptance of the emitted shapes is UNKNOWN and is not fixable by more unit tests

---

## Resolution From Anywhere (CE-GALL-26)

**Working system**

The MCP server finds its binary and its roots from any working directory
you drop it in, every steering variable stripped away — and it reaches for
a binary already built before it reaches for a `cargo run` that rebuilds
from scratch.

The old resolver walked four parents up from the launcher and called that
"finding the project." Fine under the repository layout — lands right on
the repo root. Broken under the *installed cache* layout, the only layout a
real user ever runs: it lands on `cache/<marketplace>`, which has no
`crates/` in it at all, so the launcher exited 69 while a perfectly good
built binary sat waiting in `target/debug`. A depth-counted walk can't carry
weight across two different layouts.

**Current standing:** `PARTIAL_ALIVE` (`NO_FALSIFIER`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-26.json`

**Positive witness:** `MCP initialize handshake from /tmp` (plugins/chatman-ecosystem/scripts/run-ferroplan-mcp.sh) — previously exit 69 while a built binary sat in target/debug; the 4-parents-up walk was calibrated for the repo layout and wrong under the install layout

**Negative falsifier:** `test_unresolved_binary_is_never_rendered_as_a_shell_argv` (plugins/chatman-ecosystem/tests/test_roots.py) — an unresolved binary rendered as the empty string would hand a launcher `exec ""`; it now refuses

- Non-claim: the /tmp handshake was run by hand once this session and is NOT a test; no automated regression covers the exact defect that was fixed

---

## Canonical CMCA Frontier Grounded In Real Surfaces (CE-GALL-28)

**Working system**

The 8×10 frontier the allocator sees now traces back to real repository
surfaces — every declared path exists on disk, checked. Arity was never the
whole story: a well-formed frontier built over fictional surfaces is still
a well-formed lie.

Deliberately kept separate from §8, not folded into it. §8's four allocator
refusals (7 candidates, 9 candidates, 9 factors, wrong BCINR revision) are
still untested; `surfaces.py`'s refusals fire *pre-flight* and don't count
as allocator behavior.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-28.json`

**Positive witness:** `cmca_allocate over the canonical frontier` (plugins/chatman-ecosystem/profiles/work-surfaces.json) — accepted live, and allocates differently from the fabricated frontier: correctness 0.1449 top with a 0.112-0.145 spread, versus the invented 0.161 top on a surface that does not exist

**Negative falsifier:** `test_declared_surface_paths_exist_in_the_repository` (plugins/chatman-ecosystem/tests/test_surfaces.py) — found four surfaces pointing at nonexistent paths on its first run: crates/ferroplan/src/{temporal,search,heuristic,ground} are .rs files, and they sat on the two highest-allocated surfaces

- Non-claim: the ten factor VALUES are a modelling choice with no external validation; only their grounding is claimed
- Non-claim: surfaces.py refusals are pre-flight and must NOT be counted as allocator refusals -- checkpoint 8's four allocator refusals remain untested

---

## Standing Vocabulary Single Source (CE-GALL-29)

**Working system**

The standing vocabulary answers to one source now —
`ontology/chatman-ecosystem.ttl` — and every consumer is just a projection
of it, checked by `generate.py build --check`.

There used to be three vocabularies talking past each other: `loop.py` took
four values, this document listed seven, and the canonical set in `~/mfw`
`AGENTS.md:122-133` runs six. `BLOCKED`, `MOCKED`, `REFUSED` could get
claimed here and never make it into the ledger; `BUILD_BROKEN` could land
in the ledger but never get claimed. Before this landed, **this checkpoint
couldn't even write down its own standing.**

`MOCKED` and `REFUSED` are demoted now — reasons, not standings in their own
right. `MOCKED` explains why a standing gets capped: a surface handing back
a fabricated value is still partly working, and `PARTIAL_ALIVE` records that
where a bare `MOCKED` would lose it. `REFUSED` is a run outcome, not a
verdict — a lawful refusal is the system doing its job, and calling it a
standing would confuse evidence *for* promotion with actual brokenness.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-29.json`

**Positive witness:** `test_ledger_cli_accepts_every_standing` (plugins/chatman-ecosystem/tests/test_standing.py) — loop.py went from four values to the canonical six, projected from the ontology

**Negative falsifier:** `test_loop_state_model_refuses_an_invented_standing` (plugins/chatman-ecosystem/tests/test_standing.py) — a seventh vocabulary cannot slip in through the model

- Non-claim: before this cycle, this checkpoint's own standing could not be recorded: loop.py accepted four values and BLOCKED was not among them

---

## Independent Validator Verdict (CE-GALL-30)

**Refuted claim**

MCP `validate` hands back the prose string `"Plan valid"`. `bind_plan_receipt`
wants a `validator_result` carrying a boolean `valid`. The two don't
compose — someone has to construct the verdict by hand, and
`skills/admit/SKILL.md:15` says to do exactly that.

**The `validator_result` field of every receipt bound during the 2026-07-29
cycle was hand-fabricated.** Both loop closes' independence claims are
false because of it. Recorded here, not swept along quietly.

**Current standing:** `PARTIAL_ALIVE` (`MOCKED`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-30.json`

**Negative falsifier:** none. Recorded, not hidden — a checkpoint without an executing negative fixture cannot be promoted.

- Non-claim: the validator_result field of EVERY receipt bound this session was hand-fabricated, so the independence claim of both closes is false

**Blocked by:** CE-GALL-31

---

## Receipt Chain Traversal (CE-GALL-31)

**Absent capability**

`verify_chain` does not exist. `previous_receipt` is validated by format only —
64 hexadecimal characters — and never looked up, so any well-formed hex string
is an acceptable predecessor and `None` is indistinguishable from a break.

The five-link chain from this cycle
(`755a2057 → c1520c61 → d56006af → eb8e4645 → d72f17f0`) proves the individual
links *recompute*. Nothing more. Zero evidence the chain is actually a chain.
§14's claim that "chain forks are detected" isn't untested — it's absent.

**Current standing:** `UNSUPPORTED` (`DEPENDENCY_MISSING`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-31.json`

**Negative falsifier:** none. Recorded, not hidden — a checkpoint without an executing negative fixture cannot be promoted.

- Non-claim: the 5-link chain 755a2057 -> c1520c61 -> d56006af -> eb8e4645 -> d72f17f0 is evidence that links recompute, and zero evidence that the chain is a chain

---

## Ledger Anchoring (CE-GALL-32)

**Open defect**

The ledger key is `sha256(realpath(cwd))[:24]` — run a command from a
subdirectory and a second ledger for the same repository springs into
existence, silently. Four exist today.

It demonstrated itself, unprompted, in the session that documented it: the
`Stop` hook blocked on 47 pending events in the `plugins/chatman-ecosystem`
ledger while the repository ledger read 0 pending — two ledgers, two
stories, same repo. The fix — anchoring to the git toplevel via
`roots.project_root()` — is built but not wired into `loop.py`/`phase.py`,
so the fork comes right back on the next `cd`.

**Blast radius corrected upward (2026-07-29).** The earlier text implied two
copies of `project_key`. There are **six**, and
`grep -rn 'def project_key' plugins/chatman-ecosystem/scripts/` names all of
them:

- `scripts/effective-phase.py:47`
- `scripts/phase.py:69`
- `scripts/grant-actuation.py:56`
- `scripts/actuation-intent.py:82`
- `scripts/event-summary.py:50`
- `scripts/loop.py:53`

`roots.project_root()` is wired into none of the six. This makes any per-copy
repair a partial fix by construction: the ledgers only reconverge when all six
agree, so five corrected copies leave the fork intact.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-32.json`

**Negative falsifier:** `live demonstration during this session` (plugins/chatman-ecosystem/scripts/plugin_data.py) — the defect demonstrated itself in the session that documented it -- an unambiguous, reproducible negative

- Non-claim: four ledgers exist for one repository, keyed by whatever cwd a command ran from
- Non-claim: no test asserts the six copies agree, so the count above is a grep result and not a defended invariant

**Update (2026-07-29):** `roots.project_key`/`project_directory` now anchor at `roots.project_root()` (all six former copies already import from `roots.py` as of `6e9b81a`); verified `project_key('.') == project_key('plugins/chatman-ecosystem')` and added `test_project_key_is_identical_for_cwd_and_its_subdirectory` in `plugins/chatman-ecosystem/tests/test_roots.py` as positive witness. Standing raised to `PARTIAL_ALIVE` — partial because no test yet asserts the six *callers* observe one ledger end-to-end (only the shared `roots.py` primitive is covered).

---

## Admission Frontier TOCTOU (CE-GALL-33)

**Open defect**

`loop.py:368` sets `admitted_event_count = event_count` — a blanket
watermark, blind to the `observation_frontier` the envelope is actually
supposed to attest to. Anything landing between binding an envelope and
running `admit` gets marked admitted without ever showing up in a receipt.

Caught it in this cycle's acceptance run: the envelope declared
`event_count: 142`; `admit` wrote `admitted_event_count: 143`. One event,
slipped through, unaccounted for.

The system's whole claim rests on state entering only through admitted
observations. Here's the crack in that claim, and no test covers it.

**Citation corrected (2026-07-29).** This section previously cited
`loop.py:388`. The file has shifted; `:388` is now the plan-digest format
check. The current line, verified by
`grep -n 'admitted_event_count.*event_count' scripts/loop.py`, is **`:368`**.

**Claim ceiling: this is not a one-line fix.** `observation_frontier` has no
schema anywhere in this repository —
`grep -rn observation_frontier plugins/chatman-ecosystem/ | grep -v receipts/`
returns nothing. It is typed as a bare `Value` in the Rust binder, and no
producer in this repository constructs one. So "read the envelope's declared
frontier instead of the live count" has nothing to read: a frontier schema and
a producer must exist first. The falsifier is therefore recorded as absent with
reason `DEPENDENCY_MISSING` rather than as a prose observation, because no
executing fixture can be written against a type that does not exist.

**Current standing:** `PARTIAL_ALIVE` (`DEPENDENCY_MISSING`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-33.json`

**Negative falsifier:** none — `DEPENDENCY_MISSING`. The 142/143 discrepancy is
a real observation, but it is not a Gall-checkpoint negative fixture.

**Blocked by:** an `observation_frontier` schema, and a producer that constructs one

- Non-claim: no test covers this; the defect is recorded, not fixed
- Non-claim: nothing here shows the frontier-aware admission is designable, only that it is not yet buildable

---

## Canonical Bash Mutation Classifier (CE-GALL-34)

**Defect fixed this cycle** (commit `1a9ab50`)

Two defects, one surface, both closed by folding the classifier into
`scripts/bash_classify.py`.

*Divergence.* Three copies of `MUTATING_BASH` were floating around —
`loop.py`, `phase.py`, `event-summary.py` — and they didn't agree. `phase.py`
dropped the publication class, so `git push` logged a ledger event but
never collapsed the phase vector: the ledger and the phase engine holding
two different beliefs about the same command.

*Prefix matching.* No git subcommand alternation carried a trailing
boundary, so prefixes matched loose. Bit this session in the middle of a
real run: `git merge-base --is-ancestor` and `git branch --show-current`,
both read-only, both matched `PROTECTED_BASH`, both blocked a legitimate
push. `rm\b` was the only branch that had the boundary right — a sign the
gap was an oversight, not a design call.

**What separates the real fix from a near-miss.** `\b` alone doesn't cut it.
`-` reads as a non-word character, so `commit\b` still matches
`commit-graph` — a `\b`-only patch would have kept misclassifying
`git commit-graph verify` while looking correct on the surface. The fix
uses `(?![\w-])`.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-34.json`

**Positive witness:** `test_phase_agrees_with_loop_on_publication_class`
(plugins/chatman-ecosystem/tests/test_bash_classify.py:91) — pins the divergence
itself rather than one copy's behaviour.

**Negative falsifier:** `test_protected_boundary`
(plugins/chatman-ecosystem/tests/test_bash_classify.py:102) — asserts the exact
read-only commands from the incident are not protected while `git push origin
main` and `git reset --hard` are. Removing the trailing boundary fails it;
weakening it to `\b` still fails the `commit-graph` case in the sibling table.

- Non-claim: the fix is not replayed outside this session, so it is capped at `PARTIAL_ALIVE` under the promotion law regardless of the suite being green
- Non-claim: nothing mechanically forbids a fourth copy of the classifier being reintroduced elsewhere; single-sourcing is a convention here, not an invariant

---

## Session Lifecycle Bookends (CE-GALL-35)

**Working system.** `session_open`, `session_status`, `session_close` — no
dedicated checkpoint had ever pinned them down before this entry. They only
showed up as steps buried inside `session_protocol.rs`'s longer happy-path
chain (`session_open` → `session_observe` → `session_set_goal` →
`session_think` → `session_advance` → `session_status` → `session_close`)
and inside a separate "never-opened session" refusal test. Nothing tested
the three bookend tools as a surface of their own: does `session_open`
ground state that `session_status` actually reflects? Does `session_close`
leave a session where reuse fails lawfully, not silently?

A new test file, `crates/ferroplan-mcp/tests/session_lifecycle_bookends.rs`,
drives the built `ferroplan-mcp` binary over stdio — same harness pattern as
`session_protocol.rs` — to answer exactly that: open a session against a
small valid STRIPS domain+problem, check `session_status` echoes the
grounded `session_id`/`domain_digest`/`problem_digest`/`goal_met`/`cursor`,
close it, then push on reuse of the closed `session_id`.

**Open defect / correction to the original plan.** The test was drafted on
two assumptions — that `session_status` exposes a `goal` field, and that a
second `session_close` on an already-closed session refuses with
`unknown session` the way `session_status`/`session_advance`/`session_observe`
do. Both wrong, and only running the test against the live server surfaced
it, not review:

- `session_status`'s real schema
  (`urn:chatman:ferroplan-session-status:v1`) has no `goal` field. It reports
  `cursor`, `epoch`, `goal_met`, `domain_digest`, `problem_digest`,
  `plan_length`, `remaining_plan_valid`, `receipt_chain_head`;
- a second `session_close` on an already-closed `session_id` is **not** a
  tool-level error. It returns `isError: false` with `closed: false` — an
  idempotent no-op — while `session_status` on that same closed id returns
  `isError: true` containing `unknown session`. The three tools do not
  converge on one failure mode for post-close reuse: `session_close`
  degrades gracefully where `session_status` refuses.

The test's assertions got rewritten against what was actually observed, not what had been guessed.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-35.json`

**Positive witness:** `open_status_close_bookends_agree_on_session_state`
(crates/ferroplan-mcp/tests/session_lifecycle_bookends.rs:97) — opens a
session, asserts `session_status` reflects the exact digests and goal state
`session_open` grounded, then closes it and asserts `closed: true`.

**Negative falsifier:** `session_status_after_close_refuses_lawfully`
(crates/ferroplan-mcp/tests/session_lifecycle_bookends.rs:191) — after
`session_close`, calls `session_status` on the same session_id and asserts a
lawful `isError: true` / `unknown session` refusal (not a crash or stale
success), then calls `session_close` a second time and asserts the real
idempotent `closed: false` response instead of a fabricated second refusal.

- Non-claim: not replayed outside this session, so it is capped at
  `PARTIAL_ALIVE` under the promotion law regardless of both tests passing
- Non-claim: only `session_status` and `session_close` reuse-after-close paths
  were probed; `session_open` with `replace: false` reusing a closed
  session_id (a fresh open, since close removes the id from the live map
  entirely) was not exercised and is not claimed here
- Non-claim: no concurrency scenario (close racing another call on the same
  session_id) is covered by these two tests; `session_protocol.rs` has a
  separate, unrelated concurrency test for a different bug

---

## Goal Retarget and Cursor Advance (CE-GALL-36)

**Working system**

`session_set_goal` and `session_advance` — wired into
`full_session_lifecycle` in `session_protocol.rs` as a happy-path step, and
never once given a SKILL.md or a Gall-checkpoint of their own before this
cycle. This checkpoint puts dedicated positive and negative witnesses on
each tool's real behavior, not just its cameo in a broader lifecycle test.

`session_set_goal` retargets a live `Session` to a new ground conjunction
over the already-interned fact space (`crates/ferroplan/src/session.rs:929`)
— no regrounding, no re-parse of the world. `session_advance` moves the
session's execution cursor forward over a stored plan
(`crates/ferroplan-mcp/src/session.rs:596`), refusing any advance that would
push the cursor past the plan's actual length.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-36.json`

**Positive witness:** `set_goal_retargets_and_advance_moves_cursor_on_a_real_plan`
(crates/ferroplan-mcp/tests/session_goal_advance.rs:139) — opens a session
against a three-action sequential domain, plans the original goal `(s)` (a
real 3-step plan), retargets mid-session to a different ground conjunction
`(q)`, confirms via `session_status` that the cursor reset and the epoch
advanced, then confirms via a fresh `session_think` that the retarget
actually changed what gets planned — the new plan for `(q)` is 1 step, not
3, a real plan-shape change rather than a status flag flipping. A final
`session_advance` with `completed_steps=1` over that real plan is confirmed
via `session_status`.

**Negative falsifier:** `advance_beyond_plan_length_is_refused_and_cursor_is_unchanged`
(crates/ferroplan-mcp/tests/session_goal_advance.rs:290) — plans a real
3-step sequence, then calls `session_advance` with `completed_steps` 1000
past the plan's actual length. The tool genuinely refuses (a tool-level
`isError` naming the plan-length bound, per `do_session_advance`'s
`next > plan_length` guard) — this is the TRUE observed behavior, checked by
running the call rather than assumed. `session_status` confirms the rejected
call left the cursor at 0, and a following in-range advance still succeeds.

- Non-claim: the fix is not replayed outside this session, so it is capped
  at `PARTIAL_ALIVE` under the promotion law regardless of both tests being
  green
- Non-claim: `session_set_goal`'s own negative path (a malformed or
  unreachable-atom goal) was not exercised as a second falsifier here — only
  `session_advance`'s out-of-range `completed_steps` was run negative;
  `crates/ferroplan/src/session.rs`'s `set_goal_rejects_unknown_and_adl` unit
  test covers that path at the library layer, but no MCP-tool-level witness
  for it exists yet under this checkpoint

---

## Structured Validate Verdict (CE-GALL-38)

**Defect fixed this cycle (re-witnessed, not authored by this checkpoint)**

CE-GALL-30 caught MCP `validate` returning the prose string `"Plan valid"` —
incompatible with `bind_plan_receipt`'s boolean `valid` requirement, forcing
the hand-fabrication `skills/admit/SKILL.md:15` instructs. This checkpoint
put that exact claim back in front of the live tool at the current commit,
instead of trusting the old doc's word for it.

Two direct calls to `mcp__plugin_chatman-ecosystem_ferroplan__validate`
against a trivial 1-action STRIPS domain (`(at-a)` -> `(at-b)`):

* Valid plan (`step 1: (move)`) ->
  `{"reason":null,"schema":"urn:ferroplan:plan-validation:v1","valid":true}`
* Invalid plan (`step 1: (nonexistent-action)`) ->
  `{"reason":"plan action \`NONEXISTENT-ACTION \` not a grounded op","schema":"urn:ferroplan:plan-validation:v1","valid":false}`

Both structured JSON, both a native boolean `valid` field, both tagged
`urn:ferroplan:plan-validation:v1` — no prose in sight. **CE-GALL-30's
refuted claim doesn't reproduce at this commit**: the composition gap it
named — prose in, bool required by `bind_plan_receipt` — is closed for the
tool's raw output shape. This upgrades the *mechanical* half of CE-GALL-30's
finding. CE-GALL-30's own section stays untouched, standing as the
historical record of when and why the gap first got written down.

**What this checkpoint does not claim.** `skills/admit/SKILL.md:15` still
reads as a manual instruction ("independent validator result containing
`valid: true`") rather than "pass `validate`'s own `valid` field through" —
the callers weren't audited or touched here, only the raw tool response
shape got re-verified. CE-GALL-13/CE-GALL-30's separate, still-open worry
about genuine engine independence — Ferroplan's `validate` grading its own
plan versus an outside validator like VAL — stands untouched. A structured
verdict clears the prose/bool composition problem. It doesn't, on its own,
restore independence.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-38.json`

**Positive witness:** `test_valid_plan_yields_boolean_true_directly_usable_by_bind_plan_receipt`
(plugins/chatman-ecosystem/tests/test_validate_verdict.py:64) — pins the raw
JSON captured from a live `validate` call on a valid plan and asserts the
`valid` field is a native bool directly usable in `validator_result`, with
no prose parsing step.

**Negative falsifier:** `test_invalid_plan_yields_boolean_false_with_reason_no_hand_fabrication`
(plugins/chatman-ecosystem/tests/test_validate_verdict.py:73) — pins the raw
JSON from a live `validate` call with a plan referencing a non-grounded
action against the same domain/problem, and asserts the tool reports
`valid: false` with a `reason`, again without string coercion.

- Non-claim: not replayed outside this session, so capped below `ALIVE` regardless of the suite being green
- Non-claim: the callers (`skills/admit/SKILL.md`, any script building `validator_result`) were not changed or audited for whether they still hand-fabricate instead of reading the field through — only the tool's own response shape was re-verified
- Non-claim: this does not touch or resolve CE-GALL-13/CE-GALL-30's separate open question of genuine engine-independent validation (VAL vs. Ferroplan's own `validate`)

---

## True Recursive CMCA Descent (CE-GALL-37)

**Supersedes/extends Checkpoint 9's cross-call-descent gap**

Checkpoint 9's 2026-07-29 audit found `bind_allocation_receipt`'s only
chaining field is a flat `previous_receipt`, and called "true cross-call
recursive descent" "architecturally absent from the MCP tool schema." Right
about `bind_allocation_receipt` — but blind to `cmca_allocate_recursive`, a
separate MCP tool (`crates/ferroplan-mcp/src/session.rs`,
`tool_cmca_allocate_recursive`) that already builds exactly what Checkpoint
9's "Working system" diagram asked for: a `root` frontier of eight admitted
candidates, then zero or more `descents`, each naming a
`selected_parent_node` id pulled from the frontier one depth up and
supplying a fresh local eight-candidate frontier of its own. Each depth's
payload carries `parent_payload_digest`, and it's checked, not just
declared, to equal the previous depth's real `allocation_payload_digest`.

This checkpoint put that tool through its paces directly — driving it live
via `mcp__plugin_chatman-ecosystem_ferroplan__cmca_allocate_recursive`,
running its existing suite plus one new Rust integration test.

**Required proof**

* Depth one allocation. Confirmed (existing test
  `cmca_recursive_depth_one_matches_plain_cmca_allocate`, and live).
* Depth two allocation with a real bound parent digest. Confirmed (existing test
  `cmca_recursive_depth_two_binds_the_real_parent_digest`, and live).
* Depth three allocation, chained through depth two's own digest (not depth one's).
  Confirmed by a new test added this session,
  `cmca_recursive_three_depth_chain_binds_digests_all_the_way_down`, and by a live
  three-depth call in this session.
* Cyclic ancestry refusal across non-adjacent depths. Confirmed (existing test
  `cmca_recursive_refuses_cyclic_ancestry`, and live).
* Unknown-parent-node refusal. Confirmed (existing test
  `cmca_recursive_refuses_an_unknown_selected_parent_node`, and live).
* Deterministic replay. Confirmed (existing test
  `cmca_recursive_is_deterministic_across_repeated_calls`).
* Malformed-depth refusal collapses the whole chain, no partial result. Confirmed
  (existing test `cmca_recursive_refuses_the_whole_chain_on_a_bad_depth`).

**What is still open, named rather than omitted.** Checkpoint 9's "parent
receipt mismatch refusal" and "missing return consequence refusal" items
aren't modeled by `cmca_allocate_recursive` — no mechanism exists for a
child depth's result to be rejected or re-consumed by its parent depth
after the fact, and `bind_allocation_receipt` still carries only that flat
`previous_receipt` field. The gap Checkpoint 9 found in that specific tool
hasn't moved. What closes here is narrower, and precise:
`cmca_allocate_recursive` is a real, tested, live-confirmed cross-call
recursive descent tool — distinct from both the in-array `parent`-index
tree Checkpoint 9's earlier audit exercised and from the receipt-binding
surface Checkpoint 9's gap language was really aimed at.

**Live tool calls made this session** (not just tests):

1. Positive: a real three-depth `cmca_allocate_recursive` call (root `root-0..7`, depth
   two `d2-0..7` selecting `root-0`, depth three `d3-0..7` selecting `d2-0`) succeeded.
   Depth 2's `parent_payload_digest` (`da48260...`) equalled depth 1's
   `allocation_payload_digest` exactly; depth 3's `parent_payload_digest`
   (`a26836f...`) equalled depth 2's exactly.
2. Negative — cyclic ancestry: reusing root id `r0` as both the entry to depth two and
   (with `r0` also listed among depth two's own admitted candidates) the entry to depth
   three produced `depth 3: cyclic ancestry -- \`r0\` already selected earlier in this
   descent chain`. Note: a first attempt at this falsifier, where `r0` was reused as
   depth-three's selector *without* `r0` being present among depth two's own candidate
   ids, produced the sibling **unknown-parent** refusal instead
   (`was not an admitted candidate id at depth 2`) — a genuine, informative near-miss,
   not the cyclic-ancestry path, until the id was made a legitimately admitted depth-two
   candidate.
3. Negative — unknown parent: a depth-two descent naming `node-never-admitted` as
   `selected_parent_node` (never present in the root frontier) produced
   `depth 2: selected_parent_node \`node-never-admitted\` was not an admitted candidate
   id at depth 1`.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-37.json`

**Positive witness:** `cmca_recursive_three_depth_chain_binds_digests_all_the_way_down`
(crates/ferroplan-mcp/tests/session_protocol.rs:590) — a new test asserting a genuine
three-depth cross-call chain binds each depth's `parent_payload_digest` to the real,
distinct digest of the depth immediately before it.

**Negative falsifier:** `cmca_recursive_refuses_cyclic_ancestry`
(crates/ferroplan-mcp/tests/session_protocol.rs:540) — asserts a depth-three descent
re-selecting an id already used to enter depth two is refused, confirmed live with the
exact error text above.

- Non-claim: not replayed outside this session, so capped below `ALIVE` under the
  promotion law regardless of the suite being green
- Non-claim: does not resolve `bind_allocation_receipt`'s flat-chaining gap named in
  Checkpoint 9 — that remains open and is not touched here
- Non-claim: "return consequence" / parent-side re-validation of a child depth's result
  is still unmodeled; `cmca_allocate_recursive` only builds and chains downward

---

## Receipt Chain Fork Detection (CE-GALL-39)

**Open defect** — tests a gap named in Checkpoint CE-GALL-31

CE-GALL-31 caught chain-fork detection missing entirely: `verify_chain`
doesn't exist, `previous_receipt` is format-checked only (64 hex, never
looked up). This checkpoint builds the fork for real against the running
`ferroplan-mcp` server and asks a plain question: does anything in this
repository catch it?

**What was built**

1. A trivial one-action PDDL domain/problem was solved and validated.
2. `cmca_allocate` / `bind_allocation_receipt` produced a real allocation
   receipt over eight candidates.
3. `session_open` + `session_think` on the trivial domain produced a real
   plan.
4. `bind_plan_receipt` bound a root envelope **A** (`previous_receipt: null`,
   receipt `2cc3d1a6...`).
5. `bind_plan_receipt` was called **twice more**, each time with
   `previous_receipt` set to A's receipt but a different
   `observation_frontier` payload, producing two divergent children **B1**
   (receipt `c50bec8e...`) and **B2** (receipt `c7d4829e...`) that both claim
   A as their predecessor — a genuine fork, not a synthesized one.
6. `verify_receipt` was called on B1 and B2 **individually**.

**Observed result**

Both calls returned `valid: true`. Each envelope is fully self-consistent —
its own payload digest and receipt recompute, and its declared
`previous_receipt` is a well-formed hex string — and neither call has any way
to learn a sibling exists. `verify_receipt`'s contract has no field for
"does another receipt already claim this predecessor." The fork is silently
accepted by every check this repository's MCP surface exposes.

A corpus scan (mirroring CE-GALL-31's own falsifying command,
`grep -rn verify_chain crates/ plugins/`, plus a walk of every script under
`scripts/`) found no chain-walking or branch-detection capability anywhere.
The only place "fork" is mentioned outside CE-GALL-31/34's own prose is
`agents/receipt-auditor.md`, which instructs an LLM to "reject ... forked
heads unless the fork is explicitly admitted" — but that is a markdown
prompt, not an invocable script or MCP tool, and it was not exercised against
B1/B2 in this checkpoint.

**Required proof**

* A fork (two receipts declaring the same predecessor) can be constructed
  against the live tools, not just described.
* `verify_receipt` (or any other capability in this repository) detects it.

**Current standing:** `UNSUPPORTED` (`DEFECT_OPEN`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-39.json`

**Negative falsifier:**
`test_verify_receipt_accepts_both_branches_of_the_fork_individually`
(plugins/chatman-ecosystem/tests/test_fork_detection.py:97) — pins the exact
live-tool outputs for B1 and B2 above and asserts both verify as `valid:
true` with no sibling/fork signal in either result. This is the checkpoint's
falsifier: the fork is real, constructed, and undetected.

**Blocked by:** a `verify_chain`-equivalent tool or script that looks up
whether a claimed predecessor already has another admitted child

- Non-claim: this does not build fork detection — it demonstrates its
  absence with an executing, non-mocked fixture
- Non-claim: `agents/receipt-auditor.md` names fork rejection as an
  intended behavior for an LLM auditor; this checkpoint does not evaluate
  whether an LLM following that prompt would catch B1/B2, only that no
  mechanical tool does

---

## Full 17-Tool Dogfood Chain (CE-GALL-40)

**Capstone over CE-GALL-35..39**

A prior audit turned up no test that ran all 17 `ferroplan-mcp` tools in one
continuous chained flow — just overlapping subsets, scattered across
`session_protocol.rs`, `session_lifecycle_bookends.rs`,
`session_goal_advance.rs`, the Python fork/validate fixtures. This
checkpoint answers "does the ecosystem actually dogfood every
`ferroplan-mcp` tool" with a receipt, not a shrug.

**Working system**

A small two-action STRIPS domain (`at-a -> at-b -> at-c`) drives one
continuous JSON-RPC session over stdio, straight through the built
`ferroplan-mcp` binary:

```text
parse (domain) -> parse (problem) -> solve
-> session_open -> session_observe -> session_set_goal -> session_think
-> session_advance -> cmca_allocate -> cmca_allocate_recursive
-> canonical_digest -> bind_allocation_receipt -> validate
-> bind_plan_receipt -> verify_receipt -> session_status -> session_close
```

16 of the 17 tools are touched this way — `decompose` was deliberately not
called and is named as a gap below, not silently skipped.
`canonical_digest` is called directly on a session_think-derived value, not
just implicitly inside `bind_*`. `cmca_allocate_recursive` is exercised with
a real root-plus-one-descent (2 depths), asserting the descent's
`parent_payload_digest` equals the root's real `allocation_payload_digest`.
`bind_plan_receipt`'s `previous_receipt` is deliberately left `null` — the
plan envelope and the allocation envelope are siblings referencing a shared
session via the `allocation_receipt` field, not predecessor/successor plan
envelopes; chaining `previous_receipt` to the allocation receipt would
overclaim a plan-envelope lineage that does not exist yet, since this is the
only plan envelope bound in this session.

The same trace ran live first, via direct
`mcp__plugin_chatman-ecosystem_ferroplan__*` tool calls in the authoring
session, then got formalized into a re-runnable Rust fixture driving the
binary over stdio — the more faithful "one continuous flow" transport,
matching the harness pattern the existing `session_*` test files already
use. That live run turned up a genuine finding, not a guess dressed up
after the fact as an assertion:
`bind_allocation_receipt` refuses `cmca_allocate_recursive`'s raw `depths`
payload (`allocation_result lacks payload.bcinr_revision`,
`crates/ferroplan-mcp/src/admission.rs:188-190`) because its schema expects
the flat shape `cmca_allocate` itself returns. The chain works around this
by binding the recursive result's root-depth payload (which does carry
`bcinr_revision`) with the depth-2 chain folded in under an explicit
`recursive_extension` field, so the descent is still recorded in the bound
receipt rather than silently dropped — but `bind_allocation_receipt` itself
has no native field for a multi-depth recursive allocation, the same
architectural gap CE-GALL-9 named for the flat `previous_receipt` field.

**Current standing:** `PARTIAL_ALIVE` (`NO_REPLAY`)

**Receipt:** `plugins/chatman-ecosystem/receipts/CE-GALL-40.json`

**Positive witness:** `full_seventeen_tool_dogfood_chain`
(crates/ferroplan-mcp/tests/dogfood_chain.rs:136) — one spawned
`ferroplan-mcp` process, one continuous stdio transport, 16 tools called in
order, with real assertions on each response's shape (not just
`isError: false`): `solve`'s 2-step plan, `session_think`'s retargeted
1-step plan, `cmca_allocate_recursive`'s digest chaining across the descent,
`verify_receipt`'s `valid: true`, `validate`'s structured
`urn:ferroplan:plan-validation:v1` shape. Run:
`cargo test -p ferroplan-mcp --test dogfood_chain -- --nocapture` — 2
passed, 0 failed.

**Negative falsifier:** `session_status_after_close_refuses_unknown_session`
(crates/ferroplan-mcp/tests/dogfood_chain.rs:543) — opens a session, closes
it, then calls `session_status` on the same session_id inside the same live
process, and asserts a lawful `isError: true` refusal containing
`unknown session`, not a stale success or a crash. This reproduces
CE-GALL-35's already-documented close-then-status refusal mechanism,
re-exercised here as this checkpoint's own executing falsifier rather than
assumed from that prior checkpoint.

- Non-claim: not replayed outside this session, so capped at
  `PARTIAL_ALIVE` under the promotion law regardless of the test being green
- Non-claim: `decompose` was not called — this checkpoint does not cover it,
  and it is the one tool in the server's surface with zero coverage from
  this dogfood chain
- Non-claim: `bind_allocation_receipt` has no native field for a
  `cmca_allocate_recursive` result; the `recursive_extension` workaround
  records the descent but is not a schema-level capability
- Non-claim: the negative falsifier only exercises the
  session_status-after-close path; a tampered-receipt or
  `validator_result: false` falsifier was not additionally run — one
  falsifier was chosen and executed for real rather than several run
  shallowly
- Non-claim: `verify_receipt`'s scope is unchanged — as CE-GALL-39 already
  established, it only checks an envelope's own self-consistency and has no
  chain-fork or sibling-detection capability; using it here on the plan
  envelope does not imply otherwise
- Non-claim: `session_advance` moving the cursor does not itself apply
  action effects to the session's world state — `goal_met` stayed `false`
  after advancing past the retargeted 1-step plan; only `session_observe`
  admits new facts, consistent with CE-GALL-36

---

# Audit log

## 2026-07-29 — CE-GALL-39, receipt chain fork detection

Went after the gap CE-GALL-31 named: `verify_chain` doesn't exist, so
nothing checks whether two different receipts claim the same predecessor.
Built the fork for real against the live `ferroplan-mcp` server —
`solve`/`validate` on a trivial one-action domain, `cmca_allocate`/
`bind_allocation_receipt` over eight candidates, `session_open`/
`session_think`, then `bind_plan_receipt` three times: root A, then two
children B1 and B2, both declaring A as `previous_receipt` with different
`observation_frontier` payloads. Ran `verify_receipt` on B1 and B2
separately; both came back `valid: true`. Neither carries any signal a
sibling exists — `verify_receipt` recomputes digests and checks the
declared predecessor is well-formed hex, nothing more. A corpus scan
(`grep -rn verify_chain crates/ plugins/`, plus a walk of every script
under `scripts/`) found no chain-walking or branch-detection capability
anywhere; the only "fork" mention outside CE-GALL-31/34's own prose is
`agents/receipt-auditor.md`, a markdown prompt for an LLM auditor, not an
invocable tool. Logged as `UNSUPPORTED` / `DEFECT_OPEN` with an executing
negative falsifier (`plugins/chatman-ecosystem/tests/test_fork_detection.py`,
4/4 passing) that pins the exact live-tool receipts and verification
results instead of describing the gap in prose. `blocked_by` names the
missing `verify_chain` tool. Full suite re-run after the addition: 373
passed, zero regressions.

## 2026-07-29 — CE-GALL-38, re-witnessing CE-GALL-30's validate claim

Called the live `mcp__plugin_chatman-ecosystem_ferroplan__validate` tool
directly — not from old docs — against a trivial 1-action STRIPS domain,
once for a valid plan, once for an invalid one (nonexistent grounded
action). Both raw responses came back structured JSON
(`{"reason":..., "schema":"urn:ferroplan:plan-validation:v1", "valid":bool}`),
not the prose string `"Plan valid"` CE-GALL-30 recorded. Wrote and ran
`plugins/chatman-ecosystem/tests/test_validate_verdict.py` (4 tests, all
passed, real pytest run, no mocking of the response shape — the fixtures are
the literal captured tool output) pinning both the valid and invalid raw
responses and asserting the `valid` field is a native bool usable directly
in `bind_plan_receipt`'s `validator_result`. Bound
`plugins/chatman-ecosystem/receipts/CE-GALL-38.json`,
`PARTIAL_ALIVE (NO_REPLAY)`. Left CE-GALL-30's own section untouched as
historical record; CE-GALL-38 upgrades the mechanical (prose-vs-bool)
finding while explicitly not claiming the callers were audited or that
engine independence (CE-GALL-13's VAL question) is resolved.

## 2026-07-29 — parallel-agent iteration (branch `chatman-dx-cycle`)

Three agents, working in parallel, disjoint file sets. Two feature commits
landed: `63a8a70` (Rust), `1a9ab50` (canonical Bash classification). The
suite climbed from 251 to 308 tests. This entry is the receipt-and-document
pass laid over that work.

**Corrections to existing receipts** — recorded because a stale receipt is
worse than a missing one. It's evidence pointing at the wrong line:

- CE-GALL-33 cited `loop.py:388` for the admission TOCTOU. The file's moved
  on — `:388` is now the plan-digest format check, the true line is `:368`.
  Follow the old citation and you'd audit an unrelated check and walk away
  finding nothing wrong;
- CE-GALL-33 also picked up an explicit **claim ceiling**. It read as if a
  one-line fix would close it. It can't: `observation_frontier` has no
  schema anywhere in this repository, sits as a bare `Value` in the Rust
  binder, has no producer. The falsifier moved from a prose observation to
  declared-absent with reason `DEPENDENCY_MISSING`, and `blocked_by` now
  names the two artifacts that have to exist first;
- CE-GALL-32 **understated its own blast radius**. The receipt implied two
  copies of `project_key`; the grep turns up six (`effective-phase.py:47`,
  `phase.py:69`, `grant-actuation.py:56`, `actuation-intent.py:82`,
  `event-summary.py:50`, `loop.py:53`). Changes the shape of the defect, not
  just its size — with six copies, any per-copy repair is a partial fix by
  construction.

CE-GALL-34 opened for the `MUTATING_BASH` prefix/divergence defect, closed
by `1a9ab50`, carrying an executing falsifier — `PARTIAL_ALIVE` / `NO_REPLAY`,
because the promotion law's boundary is the session and none of this has
been replayed outside it.

**The most interesting result of the iteration: the implementing agents
corrected the brief they were handed.** Both corrections surfaced by
building, not by reviewing, and neither was in the plan:

- the empty-plan case was specified as *parseable but trivially satisfied*.
  Measured, it is **unparseable** — a different failure at a different layer,
  and the test written to the brief would have asserted the wrong thing;
- the Bash boundary fix was specified as adding `\b`. That is **insufficient**:
  `-` is a non-word character, so `commit\b` still matches `commit-graph`. The
  landed fix uses `(?![\w-])`. A `\b` patch would have passed review, looked
  correct, and left `git commit-graph verify` misclassified.

Same failure mode, caught twice: a plausible specification a real run
refutes. Recorded here, not quietly absorbed — the whole value of running
agents in parallel is that the one holding the file gets to disagree with
the one holding the plan.

## 2026-07-29 — DX architecture cycle (branch `chatman-dx-cycle`)

Seven commits. 141 tests where none stood before, plus a separate CI
`plugin` job so a plugin failure never hides behind a Rust one.

Added checkpoints 22–26, 28 (new working systems) and 29–33 (recorded
negatives). Every one sits at `PARTIAL_ALIVE` or lower, every one snagged on
the same hop: no clean-worktree replay outside this session, nothing
pushed. The promotion law bars `ALIVE` on that alone, however green the
suite reads.

The canonical definition of a Gall checkpoint got pulled back from `~/mfw`, where
it lives as a formal glossary symbol
(`mfw-math/15-galls-law-evolutionary-construction.omdoc:37`): *"the smallest
closed, receipted transformation proving one complete category transition with
explicit inputs, outputs, refusals, and verification."* `~/bcinr` supplied the
rule that a falsifier must execute and be non-vacuous — "a genuine
Gall-checkpoint negative fixture, not a comment describing one". `~/wasm4pm`
supplied the promotion law, now mechanized in `tests/test_receipts.py`.

**Standing changes.** 1 ceiling narrowed (an invariant was inert). 3 blocking
hop changed — its audit finding "no agent declares `tools:`" is now false. 8
partially retracted — the prior happy-path evidence used a fabricated frontier
including a nonexistent surface. 13 **downgraded** to `PARTIAL_ALIVE` +
`MOCKED`. 14 sharpened from "not re-verified" to **absent**. 19 gained a
five-link chain and lost the fork-refusal claim. 20 net honest downgrade: two
more closes, but both fabricated the validator verdict.

**Defects the new tests turned up while being written** — none of them known when the cycle was planned:

- four surfaces pointed at nonexistent paths — `crates/ferroplan/src/{temporal,
  search,heuristic,ground}` are `.rs` files, and they sat on the two
  highest-allocated surfaces;
- SHACL's first-ever run caught `ce:maxTurns` declaring
  `rdfs:range xsd:positiveInteger` while every value parsed as `xsd:integer` —
  the ontology's own declared range unsatisfied by its own data;
- the human projection of an unresolved binary was the empty string, which
  would have handed a launcher `exec ""`.

**Two corrections to earlier claims made in this same session** — worth more on the record than folded into a quiet edit:

- the MCP resolution failure was first blamed on `env.setdefault` preserving an
  empty variable. Measured: the variables are *unset*, so `setdefault` fires.
  The real cause was a four-parents-up walk calibrated for the repository
  layout landing on `cache/<marketplace>` under the install layout;
- the inert invariant was first going to be "fixed" by renaming
  `requires_any_prior` to `requires_any`. That would have forbidden every
  `planning=validated` vector and deleted the state from the reachable space.
  Deletion was correct.

**The ledger fragmentation defect demonstrated itself during the very
session that documented it** (CE-GALL-32): the `Stop` hook blocked on 47
pending events in the `plugins/chatman-ecosystem` ledger while the
repository ledger read 0.

**Left undone, named rather than swept aside:** MCP `validate` still
returns prose so the validator verdict stays fabricated (CE-GALL-30);
`verify_chain` doesn't exist (CE-GALL-31); ledger anchoring is built but
unwired (CE-GALL-32); the admission TOCTOU sits open and untested
(CE-GALL-33); `loop.py close` isn't built, so both closes ran nine manual
steps apiece; nothing is pushed, and `main` has none of it.

**Clean-clone replay ran, and it does NOT promote anything.** At seal
`2ee20a5` the tree was cloned to a fresh path, checked out at the sealed
commit with a verified-clean worktree, run with all four steering variables
cleared: 251 passed, `generate.py build --check` clean. Real evidence —
kills two failure modes, a dirty worktree and environment leaking from the
authoring shell.

Deliberately **not** recorded as `replayed_outside_session`. The promotion
law's boundary is the *session*, not the process, and here's the third
failure mode a clone can't touch: the agent replaying is the same agent
that wrote the tests and chose which ones to run. `wasm4pm` made the
identical call, demoting its own receipts from `ALIVE` down to
`PARTIAL_ALIVE` pending a genuinely independent replay. The flag stays
`false` until someone else — or a later session — runs it.

**The one action that promotes 22–26, 28, 29 to `ALIVE`:** clone to a fresh
path, check out the sealed commit, and run `pytest` plus
`generate.py build --check` outside this session. Then set
`replayed_outside_session` and `sealed_at_commit` in each receipt.

## 2026-07-29 — first full pass

Ran checkpoints 2, 3, 9, 13, 19 all the way to real evidence — commands and
output shown inline above. Confirmed existence/non-existence for 11 and
15–18 without attempting new implementation. Upgraded: 2 (`UNKNOWN` →
`PARTIAL_ALIVE`), 13 (`UNSUPPORTED` → `PARTIAL_ALIVE`), 11 (`UNKNOWN` →
`UNSUPPORTED` — sharpened, not upgraded). Sharpened without moving the
label: 3, 9, 19. Left untouched: 0, 1, 4, 5, 6, 7, 8, 10, 12, 14, 16, 17,
18, 20, 21 — either re-confirmed from existing evidence or plainly out of
this pass's scope.

Concrete artifacts left behind by this pass:
- `benchmarks/.val/VAL/build/bin/Validate` — real vendored VAL binary
  (gitignored, not committed; rebuild with `sh benchmarks/get-val.sh
  -DCMAKE_POLICY_VERSION_MINIMUM=3.5` if the plain script fails to
  configure).
- This file (`docs/gall-checkpoints.md`), stood up for the first time —
  before it, the checkpoint spec lived only in chat history, one re-derived
  inconsistency away from drifting apart every session.

Named next steps, not yet started: patch `get-val.sh`'s cmake invocation;
add `tools:` frontmatter to the 8 agents (Checkpoint 3); decide and
implement recursive CMCA's actual schema shape (Checkpoint 9); write a
worktree-manufacture script (Checkpoint 11); resolve PR #2's `CI / test`
failure or supersede it.

## 2026-07-29 — Session Lifecycle Bookends (CE-GALL-35)

Opened CE-GALL-35 for `session_open`/`session_status`/`session_close` — the
three MCP session tools nobody had ever given a dedicated checkpoint or
test to (they only surfaced as intermediate steps buried inside
`session_protocol.rs`'s longer chains). Added
`crates/ferroplan-mcp/tests/session_lifecycle_bookends.rs` with one positive
witness (open → status reflects grounded state → close reports `closed:
true`) and one negative falsifier (status on a closed session refuses with
`unknown session`; a second close on a closed session is checked against its
actual observed behavior). Both tests run for real against the built
`ferroplan-mcp` binary: `cargo test -p ferroplan-mcp --test
session_lifecycle_bookends` — 2 passed, 0 failed.

The negative test's first draft figured a double-close would also refuse
with `unknown session`. Not against the live server: double-close returns
`isError: false` / `closed: false`, an idempotent no-op, not a refusal. The
assertion got rewritten to match the real response instead of asserting
the wrong thing just to look tidy. `session_status`'s schema took the same
correction mid-draft — no `goal` field; the real fields are
`cursor`/`epoch`/`goal_met`/`domain_digest`/`problem_digest`/`plan_length`/
`remaining_plan_valid`/`receipt_chain_head`.

Standing: `PARTIAL_ALIVE` / `NO_REPLAY`, same cap as every other checkpoint in
this file — not replayed outside the authoring session. Receipt:
`plugins/chatman-ecosystem/receipts/CE-GALL-35.json`. No other checkpoint,
receipt, or test file was touched.

## 2026-07-29 — Goal Retarget and Cursor Advance (CE-GALL-36)

Opened CE-GALL-36 for `session_set_goal` and `session_advance` — neither
one had a dedicated checkpoint before this, both only ever showed up as
steps inside `session_protocol.rs`'s longer happy-path chain.

Added `crates/ferroplan-mcp/tests/session_goal_advance.rs` with one positive
witness and one negative falsifier, both run for real against the built
`ferroplan-mcp` binary over stdio (same harness as `session_protocol.rs`):
`cargo test -p ferroplan-mcp --test session_goal_advance -- --nocapture` — 2
passed, 0 failed.

The positive witness plans a real 3-step sequential domain to its original
goal, retargets mid-session to a different ground conjunction, and confirms
the retarget by replanning — the new plan is a genuinely different shape (1
step, not 3), not merely a status flag. `session_advance` then moves the
cursor over that real plan and `session_status` confirms it.

The negative falsifier pushed `session_advance` with `completed_steps` far
past the plan's real length and checked the TRUE observed response instead
of assuming a refusal. The tool refuses clean — a tool-level `isError`
naming the plan-length bound (`do_session_advance`'s `next > plan_length`
guard in `crates/ferroplan-mcp/src/session.rs`) — and a follow-up
`session_status` confirms the rejected call left the cursor untouched. No
silent-acceptance surprise on this path. The honest negative result here is
a working refusal, not a discovered gap.

Standing: `PARTIAL_ALIVE` / `NO_REPLAY`, same cap as every other checkpoint
in this file — not replayed outside the authoring session. Receipt:
`plugins/chatman-ecosystem/receipts/CE-GALL-36.json`. `session_set_goal`'s
own negative path (malformed/unreachable goal atoms) remains unexercised at
the MCP-tool layer and is named as a non-claim in the checkpoint section — it
is covered only by a library-layer unit test
(`crates/ferroplan/src/session.rs::set_goal_rejects_unknown_and_adl`). No
other checkpoint, receipt, or test file was touched.

## 2026-07-29 — CE-GALL-37, true recursive CMCA descent

Closed the specific gap CE-GALL-9 named: "true cross-call recursive descent
... architecturally absent from the MCP tool schema." True for
`bind_allocation_receipt`'s flat `previous_receipt` field — but blind to
`cmca_allocate_recursive`, a separate, already-implemented tool
(`crates/ferroplan-mcp/src/session.rs`) that chains a `root` frontier
through zero or more `descents`, each one binding `parent_payload_digest`
to the real previous depth's `allocation_payload_digest`.

Ran the existing six-test `cmca_recursive_*` suite in
`crates/ferroplan-mcp/tests/session_protocol.rs` (`cargo test -p ferroplan-mcp
--test session_protocol cmca_recursive`, exit 0, all passed) and added a
seventh, `cmca_recursive_three_depth_chain_binds_digests_all_the_way_down`,
to cover a genuine three-depth chain rather than depth two only. Also drove
`cmca_allocate_recursive` live via the MCP tool three times this session: a
three-depth positive chain (digests bound correctly at every depth), a
cyclic-ancestry refusal (confirmed with the exact expected error text after
one informative near-miss that tripped the sibling unknown-parent path
instead), and an unknown-parent-node refusal.

Standing: `PARTIAL_ALIVE` / `NO_REPLAY`. Receipt:
`plugins/chatman-ecosystem/receipts/CE-GALL-37.json`. CE-GALL-9's own section
was left untouched as historical record; `blocked_by` on the new receipt is
empty because this checkpoint stands on its own tool, not on CE-GALL-9's
resolution. What CE-GALL-9 still names as open — `bind_allocation_receipt`'s
flat chaining field, and "return consequence" propagation back up a chain —
remains open and unaddressed by this work.

## 2026-07-29 — CE-GALL-40, full 17-tool dogfood chain (capstone)

Closed the gap the CE-GALL-35..39 session's audit found: nowhere had a test
run all 17 `ferroplan-mcp` tools in one continuous chained flow. Ran 16 of
the 17 tools live first, via direct
`mcp__plugin_chatman-ecosystem_ferroplan__*` tool calls in this session, on a
small two-action STRIPS domain (`at-a -> at-b -> at-c`): `parse` (domain,
problem) -> `solve` -> `session_open` -> `session_observe` ->
`session_set_goal` -> `session_think` -> `session_advance` ->
`cmca_allocate` -> `cmca_allocate_recursive` (root + 1 descent) ->
`canonical_digest` -> `bind_allocation_receipt` -> `validate` ->
`bind_plan_receipt` -> `verify_receipt` -> `session_status` ->
`session_close`, then a negative falsifier: `session_status` on the same
session_id immediately after `session_close` — confirmed live as a lawful
`unknown session` refusal, matching CE-GALL-35's already-documented
mechanism. `decompose` was deliberately not called; named as the one
uncovered tool rather than fabricated or silently skipped.

That live run turned up a real defect, not just a confirmed guess:
`bind_allocation_receipt` refused `cmca_allocate_recursive`'s raw `depths`
payload with `allocation_result lacks payload.bcinr_revision`
(`crates/ferroplan-mcp/src/admission.rs:188-190`) — its schema expects the
flat shape `cmca_allocate` itself returns, not the recursive tool's own
output shape. Worked around it by binding the recursive result's
root-depth payload (which does carry `bcinr_revision`), the depth-2 chain
folded in under an explicit `recursive_extension` field.

Formalized the whole trace as a re-runnable Rust fixture,
`crates/ferroplan-mcp/tests/dogfood_chain.rs`, driving the built
`ferroplan-mcp` binary over stdio in one continuous JSON-RPC session — same
harness pattern as `session_lifecycle_bookends.rs` / `session_protocol.rs`,
the more faithful transport for "one continuous flow" than separate
agent-session MCP calls. Ran it for real:
`cargo test -p ferroplan-mcp --test dogfood_chain -- --nocapture` — 2
passed (the chain, and the falsifier), 0 failed.

Standing: `PARTIAL_ALIVE` / `NO_REPLAY`, same cap as every other checkpoint
in this file. Receipt: `plugins/chatman-ecosystem/receipts/CE-GALL-40.json`.
`decompose`'s genuine decomposition behavior remains fully unexercised by
this checkpoint — that is a named gap, not a claim. `bind_plan_receipt`'s
`previous_receipt` was deliberately left `null`, documented in the
checkpoint section as a sibling-not-successor relationship, not an
oversight. CE-GALL-35 through CE-GALL-39's own sections, receipts, and test
files were not touched — this entry only adds new material.
