---
id: TC-1890
title: "A source-declared time unit is a clock unit"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-778
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1890: A source-declared time unit is a clock unit

## Description

Check that a `duration` aggregate's bounds may be typed by a source-owned clock unit node, that the reader matches each bound against exactly the node `timestamp_unit` names, and that it resolves that name in the unit the row's `source_unit` names. Scope: FR-778-AC-1 to FR-778-AC-13 and FR-322-AC-511.

## Test Procedure

1. Read FR-778-AC-1's package, then the same with `upper` typed by `s`.
2. Read FR-370-AC-20's package, then the same with a bound typed by a source `ms`.
3. Read FR-778-AC-1's package with its row naming `us`, `m` and an affine `Time` unit, each also with a bound typed by `s`.
4. Read the clause of TCB-03, whose row names `RFC3339` and which has no `duration` window.
5. Decide the `source_unit_cases` `SU-01` to `SU-05` of `proposals/checked-package-v2/temporal-clock-binding-vectors.json`.
6. Build FR-322-AC-511's finite-domain hyper clause `Fast` in unit `example` and decide it, then each refusal variant FR-322-AC-511 and FR-778-AC-13 list (the hyper `claim_names` row without, with a foreign and with a differing `source_unit`; a `source_unit` on a state-clause row and on a trace row; the units declared only by `acme/orders`).

## Expected Results

- Step 1: admitted, no `["scalar_type", "unit", "ms"]` node; then `ill_typed`/`unit-mismatch` at `upper`.
- Step 2: admitted; then `ill_typed`/`unit-mismatch` at the bound.
- Step 3: `missing_declaration`/`missing-name`, then `ill_typed`/`unit-mismatch` twice, at the row's `timestamp_unit`, each retaining the clock name and `T` and reported instead of the bound's refusal.
- Step 4: admitted.
- Step 5: `SU-01` and `SU-02` admitted; `SU-03` `missing_declaration`/`missing-name` at the row's `timestamp_unit`; `SU-04` `invalid_package`/`invalid-value` at the row's `source_unit`; `SU-05` `invalid_package`/`timed-node-reference` at the second row.
- Step 6: admitted with both trace rows' clock unit `example`'s `ms` node; then each refusal at the code, cause and place FR-322-AC-511 and FR-778-AC-13 state, the missing `source_unit` reported before any unit resolution.
