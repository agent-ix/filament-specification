---
id: TC-2017
title: "Recommend steps carry typed records and keep state"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-822
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-426
    type: verifies
---

# TC-2017: Recommend steps carry typed records and keep state

## Description

Verify the `recommend` step, its recommendation record and its place in
traces and replay. Scope: FR-822-AC-3 and FR-426-AC-22.

## Test Procedure

1. Explore a clause holding `recommend Advise by Auditor on
   Orders::Order::refund contracts [] as (r: Boolean) { true };` from an order
   with `total = 5`, and a thread at `recommend Advise by R on M::Cell::bump
   contracts [BumpPre]` over cells with `v` 0 and 1.
2. Take a counterexample through a `recommend` step and replay it unchanged,
   then with the record's `arguments` altered.
3. Check the clause with a guard that reads `r`.

## Expected Results

- Step 1: one `recommend` step per application whose selected precondition
  holds (none where `v` is 1); each identity has `step` `recommend` and a
  `recommendation` record of `type` `protocol-recommendation` with `node`,
  `role`, `operation`, `receiver` and `arguments`; every successor keeps the
  predecessor's `semantic` key member and compensation registrations.
- Step 2: the counterexample carries the record in its step; the unchanged
  replay reproduces; the altered replay refuses.
- Step 3: `ill_typed`/`type-mismatch` at the read.
