---
id: TC-1890
title: "A source-declared time unit is a clock unit"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-778
    type: verifies
---

# TC-1890: A source-declared time unit is a clock unit

## Description

Check that a `duration` aggregate's bounds may be typed by a source-owned clock unit node, that the reader matches each bound against exactly the node `timestamp_unit` names, and that it resolves that name in the unit the row's `source_unit` names. Scope: FR-778-AC-1 to FR-778-AC-11.

## Test Procedure

1. Read FR-778-AC-1's package, then the same with `upper` typed by `s`.
2. Read FR-370-AC-20's package, then the same with a bound typed by a source `ms`.
3. Read FR-778-AC-1's package with its row naming `us`, `m` and an affine `Time` unit, each also with a bound typed by `s`.
4. Read the clause of TCB-03, whose row names `RFC3339` and which has no `duration` window.
5. Decide the `source_unit_cases` `SU-01` to `SU-04` of `proposals/checked-package-v2/temporal-clock-binding-vectors.json`.

## Expected Results

- Step 1: admitted, no `["scalar_type", "unit", "ms"]` node; then `ill_typed`/`unit-mismatch` at `upper`.
- Step 2: admitted; then `ill_typed`/`unit-mismatch` at the bound.
- Step 3: `missing_declaration`/`missing-name`, then `ill_typed`/`unit-mismatch` twice, at the row's `timestamp_unit`, each retaining the clock name and `T` and reported instead of the bound's refusal.
- Step 4: admitted.
- Step 5: `SU-01` and `SU-02` admitted; `SU-03` `missing_declaration`/`missing-name` at the row's `timestamp_unit`; `SU-04` `invalid_package`/`invalid-value` at the row's `source_unit`.
