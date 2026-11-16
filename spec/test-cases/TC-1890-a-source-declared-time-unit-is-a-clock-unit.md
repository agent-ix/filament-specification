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

Check that a `duration` aggregate's bounds may be typed by a source-owned clock unit node, and that the reader matches each bound against exactly the node `timestamp_unit` names. Scope: FR-778-AC-1 to FR-778-AC-3.

## Test Procedure

1. Read FR-778-AC-1's package, then the same with `upper` typed by `s`.
2. Read FR-370-AC-20's package, then the same with a bound typed by a source `ms`.
3. Read FR-778-AC-1's package with its row naming `us`, `m` and an affine `Time` unit.

## Expected Results

- Step 1: admitted, no `["scalar_type", "unit", "ms"]` node; then `ill_typed`/`unit-mismatch` at `upper`.
- Step 2: admitted; then `ill_typed`/`unit-mismatch` at the bound.
- Step 3: `missing_declaration`/`missing-name`, then `ill_typed`/`unit-mismatch` twice, at the row's `timestamp_unit`.
