---
id: TC-1959
title: "Plans over probabilistic models"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-809
    type: verifies
---

# TC-1959: Plans over probabilistic models

## Description

Verify probability-1 goals, maximized probabilities, policies over every outcome and exact expectations. Scope: FR-809-AC-1 to FR-809-AC-4.

## Test Procedure

1. Plan `try` and `retry` toward `done`.
2. Maximize `Reach.probability` with `try` only over lengths 1 to 2.
3. Add a policy that `try` breaks with probability 1/10.
4. Minimize an expected per-step cost of 1 or 3.

## Expected Results

- Step 1: `optimal` `retry`.
- Step 2: `try, try` at exactly 3/4.
- Step 3: no plan contains `try`.
- Step 4: expectation exactly 2 per step.
