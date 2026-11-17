---
id: TC-1925
title: "An await window carries its own clock binding row"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1925: An await window carries its own clock binding row

## Description

Verify that a `temporal_bindings` row may be keyed by a protocol `await`
control node under the compensation-row rules, and that the four
compensation refusals apply to an await row at the same loci. Scope:
FR-322-AC-305 (the `await_cases` of the clock-binding vectors).

Catches a reader that refuses an await row as `timed-node-reference` at its
declaration, and one that admits an await with no row under a
clock-configured lock.

## Test Procedure

1. Read `await_cases` `AW-01` (an await beside a compensation sharing model,
   profile and clock name) and `AW-02` (an await alone) under an
   `event_position` lock selection.
2. Read `AW-03` (the await's row removed), `AW-04` (its row repeated),
   `AW-05` (its `clock_name` changed) and `AW-06` (its configuration a
   `fixed_sample` one).
3. Read `CB-07`, whose row names a `send` control.

## Expected Results

- Step 1: both admitted; the await row is keyed by the await's own
  `node_id`, occurrence 0, and rows ascend by `NodeRef`.
- Step 2: `invalid_package`/`timed-node-reference` at the await's
  `/semantic_graph/nodes/{N}/body`; `conflicting-definition` at the later
  row; `timed-node-reference` at `/temporal_bindings/{i}/clock_name`; then at
  `/temporal_bindings/{i}/clock/kind`.
- Step 3: `invalid_package`/`timed-node-reference` at
  `/temporal_bindings/1/declaration`.
