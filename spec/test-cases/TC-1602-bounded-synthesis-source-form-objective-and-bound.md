---
id: TC-1602
title: "Bounded synthesis source form, objective and bound"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-194
    type: verifies
---

# TC-1602: Bounded synthesis source form, objective and bound

## Description

Verify the `synthesis` source form, the finite-domain rule, the objective and the result kinds. Scope: FR-194-AC-6 to FR-194-AC-12.

## Test Procedure

1. Run `Pick` over `type Digit = Int[0, 9];` with `Pick * Pick = 49` and with `Pick * Pick = 50`.
2. Run it with `minimize Pick` and `maximize Pick` and the block `Pick >= 3`.
3. Check the unbounded domains FR-194-AC-8 lists and the mutations FR-194-AC-9 lists.
4. Run a search with a work limit smaller than its space.
5. Run FR-194-AC-11's counter plan with the `Reach.steps.sum(s => ...)` objective and with the `sum<Cost>` form; parse `s=>1` and `s => 1`; check the non-numeric body.
6. Check each misplaced or malformed lambda of FR-194-AC-12.

## Expected Results

- Step 1: `candidate` 7, `unchecked` then `validated`; `no-candidate` after ten values.
- Step 2: optimal 3 with the three earlier candidates 0, 1 and 2 failed; 9 for `maximize`.
- Step 3: each refusal with the cause and component stated.
- Step 4: `incomplete` naming the limit and visited count, never `no-candidate`.
- Step 5: the plan `inc, inc, inc, double` with objective 8 under both spellings; equal parses; `ill_typed`/`type-mismatch` at the body.
- Step 6: each refusal with the code and place FR-194-AC-12 states.
