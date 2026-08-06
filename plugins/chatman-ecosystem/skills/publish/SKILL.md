---
name: publish
description: Publish a validated, receipted Ferroplan change through protected GitHub actuation. Use only when the user explicitly requests commit, push, or draft PR publication.
disable-model-invocation: true
effort: high
---

Publish `$ARGUMENTS` — and only after the user has said so, in words, out
loud.

Require all of the following:

- no pending hook events;
- phase vector `epistemic=admitted`, `allocation=allocated`, `planning=validated`, `actuation=publishable`, `drift=stable`, `conformance=conformant`;
- verified allocation and plan receipt envelopes;
- independent validator result with `valid: true`;
- receipt audit with maximum lawful standing.

Then publish only the admitted scope. Prefer a draft pull request.
Protected commands remain subject to the PreToolUse fence. If the hook
refuses, stop there and report the missing frontier — the fence does not
get climbed, only satisfied.
