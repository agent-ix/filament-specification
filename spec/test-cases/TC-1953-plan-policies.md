---
id: TC-1953
title: "Plan policies: pruning, completed-plan evaluation and terminal stutter"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-803
    type: verifies
---

# TC-1953: Plan policies: pruning, completed-plan evaluation and terminal stutter

## Description

Verify safety pruning, non-safety and bounded policies, model invariants and infeasibility under policy. Scope: FR-803-AC-1 to FR-803-AC-5.

## Test Procedure

1. Plan FR-650-AC-1's counter under `always holds(c.value != 4)`.
2. Plan it under `eventually holds(c.value = 3)`.
3. Plan it under `eventually[0, 2] holds(c.value >= 4)`.
4. Add the invariant `value <= 10`; name a policy over another model.
5. Plan under `always holds(c.value != 8)`.

## Expected Results

- Step 1: no node with value 4.
- Step 2: `optimal` `inc, inc, inc, double`.
- Step 3: `inc, double, double` admitted; prefix `inc, inc` pruned.
- Step 4: the step to 12 excluded; the foreign policy refused per FR-806.
- Step 5: `infeasible` with an accepted closure certificate.
