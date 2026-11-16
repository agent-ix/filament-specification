---
id: TC-1774
title: "The recovery binder is bound at cend, one successor per recovery record"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-730
    type: verifies
---

# TC-1774: The recovery binder is bound at cend, one successor per recovery record

## Description

Check that `cend` binds a recovery predicate's binder to each record of the
recovery type, that a predicate that does not read it binds none, and that
replay and supplied executions use the bound record.

Scope: FR-730-AC-1, FR-730-AC-2, FR-730-AC-3, FR-730-AC-4.

## Test Procedure

1. Explore TC-380's `Pay` with `recover (r: M::Recovery) { r.bal = 0 }`, `bal: Int[0, 1]`, and check that every finished instance is `recovered`.
2. Explore the same protocol with the predicate `triggered`.
3. Replay step 1's counterexample, then the same counterexample with the recovery record changed to `bal` 0.
4. Check a supplied execution whose recovery observation reports `bal` 0, then the same execution with the observation removed.

## Expected Results

- Step 1: `cend` has two successors (`recovered` at `bal` 0, `unrecovered` at `bal` 1); the claim is refuted with `r.bal` 1 in the counterexample.
- Step 2: `cend` has one successor; the verdict is unchanged.
- Step 3: the first replay reproduces `unrecovered`; the second does not reproduce.
- Step 4: `recovered`; with no observation the status is incomplete, never `unrecovered`.
