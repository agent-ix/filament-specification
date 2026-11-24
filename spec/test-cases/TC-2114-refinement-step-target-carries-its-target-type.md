---
id: TC-2114
title: "A refinement operation step target carries its target type"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-375
    type: verifies
---
# TC-2114: A refinement operation step target carries its target type

## Description

Verify that an abstract operation step target carries `target_type`, that the
reader requires and checks it, and that two rows over inherited-operation
subtypes are distinguished by it. Scope: FR-375-AC-39 to FR-375-AC-44; the schema vectors are `proposals/checked-package-v2/refinement-step-target-vectors.json`.

## Test Procedure

1. Emit a refinement with two step rows from `Ring` to the inherited operation
   `enq` of `Queue` with `target_type` `FastQueue` and `SlowQueue`.
2. Read the package, then remove `target_type` from one row and set it to `null` in another.
3. Add a protocol node row of FR-375-AC-40. Set `target_type` to `Cell` (does not declare or inherit `enq`) and to a
   node that is no abstract type.
4. Swap the two rows' `target_type` values and compare the canonical bytes.

## Expected Results

- Step 1: both rows admitted; each receiver lifts against its own type.
- Step 2: the reader's missing-member refusal at the step entry.
- Step 3: `invalid_model_binding`/`malformed-declaration` at `target_type`; `ill_typed`/`type-mismatch` at the node-row argument of FR-375-AC-40.
- Step 4: the bytes differ.
