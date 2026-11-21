---
id: TC-1950
title: "Plan steps, arguments and goal over a state model"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-800
    type: verifies
---

# TC-1950: Plan steps, arguments and goal over a state model

## Description

Verify the plan value, reference and created-object arguments, native invocation identity, the empty plan, undefined goals and unbounded parameters. Scope: FR-800-AC-1 to FR-800-AC-6.

## Test Procedure

1. Plan two `drain` steps over the `Server` population `n1`, `n2` and read `Reach.steps`.
2. Add `spawn` creating `n3` and plan with it.
3. Plan with a goal true in the initial state.
4. Plan with a goal undefined when `divisor` is 0.
5. Check a model with an unbounded `Integer` parameter.
6. Use TC-1951's native invocation fixture with goal `Reach.final.v = 2`
   instead of false. Read the selected step, and replay it from the bound
   initial state; then change its receiver or its ordinal while retaining
   the originally recorded post-state. Verify the same edits through
   certificate checking.
7. Restrict `choose`'s postcondition to `result = 0` and final `v = 2`, so
   the invocation has one outcome. Separately use a receiverless invocation
   with one unchanged-state outcome. Read both source plan step values and
   wire steps under the existing value encodings.

## Expected Results

- Step 1: two step records with indexes, operation, arguments and states equal to the runtime's.
- Step 2: `n3` is a candidate only after `spawn`.
- Step 3: the empty plan, length 0.
- Step 4: the undefined plan does not reach the goal.
- Step 5: refused at the parameter as FR-806 states.
- Step 6: the reported step has receiver `d0` in its FR-181 reference
  identity and ordinal 2, selecting `(0, S2)`. Replay reproduces the
  unchanged step; changing the receiver or ordinal cannot silently select
  another outcome and is rejected by replay and certificate verification.
- Step 7: the receiver-bearing singleton retains its receiver and omits
  the ordinal; the receiverless singleton omits both. Neither omission is
  represented by an invented placeholder receiver or explicit ordinal 0.
