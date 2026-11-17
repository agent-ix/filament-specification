---
id: TC-1926
title: "A top-level float pair selects the float equality schedule"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1926: A top-level float pair selects the float equality schedule

## Description

Verify the `float` equality schedule of `equality_decisions` and its
refusals. Scope: FR-322-AC-288 (the `float` schedule).

Catches a producer that records a float pair under `plan`, and a reader that
admits `float` for a composite holding a float leaf or for two different
float types.

## Test Procedure

1. Read the schema's `EqualityDecision.schedule` enumeration.
2. Read `EQ-11-float32` and `EQ-12-float64`, and `EQ-04-plan`.
3. Read `EQ-R8-float-pair-under-plan`, `EQ-R9-float-over-composite`,
   `EQ-R10-float-over-two-float-types` and `EQ-R11-float-over-non-float`.

## Expected Results

- Step 1: `float` is enumerated beside `text`, `enum`, `quantity` and `plan`.
- Step 2: each float row has schedule `float`, equal operand source types, no
  conversion target, no unit rows and no enum members; the record row stays
  `plan`.
- Step 3: each refuses `invalid_package`/`invalid-value` at
  `/checked_term_data/equality_decisions/0/schedule`.
