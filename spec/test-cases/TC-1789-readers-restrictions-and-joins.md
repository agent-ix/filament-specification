---
id: TC-1789
title: "Weak readers load relaxed, orderings are restricted and non-all joins explore"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-741
    type: verifies
---

# TC-1789: Weak readers load relaxed, orderings are restricted and non-all joins explore

## Description

Verify that weak readers load relaxed, orderings are restricted and non-all joins explore. Scope: FR-741-AC-3, FR-741-AC-4.

Catches any remaining weak-access-shape answer.

## Test Procedure

1. Explore a reader of a shared location; a `release` load; a `join any … outstanding cancel` under `tso`; then TC-389's corpus.

## Expected Results

- Step 1: One successor per visible value; the load is `relaxed` with its restriction listed; the cancelled store flushes after the join; no item settles `weak-access-shape`.
