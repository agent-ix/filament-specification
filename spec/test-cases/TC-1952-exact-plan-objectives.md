---
id: TC-1952
title: "Exact plan objectives and their classification"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-802
    type: verifies
---

# TC-1952: Exact plan objectives and their classification

## Description

Verify exact decimal, rational and quantity objectives, negative step costs, floating-point objectives and the default objective. Scope: FR-802-AC-1 to FR-802-AC-6.

## Test Procedure

1. Minimize a decimal per-step cost over three steps.
2. Minimize a rational sum; exceed `accounting.integer_bits`.
3. Minimize a sum of `30 s` and `1 min` steps.
4. Minimize a step cost that is negative on one step.
5. Minimize a `Float64` objective that is NaN or undefined on some plans.
6. Maximize `final.value`; declare no objective.

## Expected Results

- Step 1: exactly `0.25`, `potential`.
- Step 2: exactly `1`; `incomplete` naming `accounting.integer_bits`.
- Step 3: the two plans tie at `60 s`.
- Step 4: `closure` with `objective.negative_step`.
- Step 5: excluded plans counted; ranking by `totalOrder`.
- Step 6: `closure`; length minimized.
