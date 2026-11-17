---
id: TC-1971
title: "Export encodings of state, actions, values and arithmetic"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-812
    type: verifies
---

# TC-1971: Export encodings of state, actions, values and arithmetic

## Description

Verify the state, action and value encodings, definedness guards, division laws, exact and floating-point numbers, sets and text. Scope: FR-812-AC-1 to FR-812-AC-5.

## Test Procedure

1. Export the `Account` model to both targets.
2. Export an invariant dividing by `count`.
3. Export `(-7) div 2` under floor and truncating laws.
4. Export a decimal sum and a `Float64` sum.
5. Export sets built in two orders and the text `"hé"`.

## Expected Results

- Step 1: the stated definitions, all `preserved`.
- Step 2: the definedness conjunct first.
- Step 3: -4 and -3 in both targets.
- Step 4: `3/10`, and the float sum unequal to it.
- Step 5: equal sorted Lean lists; `<<104, 233>>`.
