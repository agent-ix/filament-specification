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

Verify safety pruning, non-safety and bounded policies, model invariants, infeasibility under policy
and policy clauses with no instance. Scope: FR-803-AC-1 to FR-803-AC-8
and FR-803-AC-10 to FR-803-AC-12.

## Test Procedure

1. Plan FR-650-AC-1's counter under `always holds(c.value != 4)`.
2. Plan it under `eventually holds(c.value = 3)`.
3. Plan it under `eventually[0, 2] holds(c.value >= 4)`.
4. Add the invariant `value <= 10`; name a policy over another model.
5. Plan under `always holds(c.value != 8)`.
6. Empty the snapshot's `counters` population and set domain `Int[0, 0]`.
   Run goal `true` with no policy, with `NotFour` and with `SomeTime`; then
   goal `false` with no policy and with `NotFour`.
7. Add the object type `Spare`, its empty population and the clause
   `SpareHolds` (`always holds(true)` over `s: M::Spare`); with `c` at 1 and
   goal value 8 plan under `["SomeTime"]` and under
   `["SomeTime", "SpareHolds"]`.
8. Hold `a` and `b` in `counters`, set domain `Int[0, 0]` and goal `true`,
   and plan under `NotFour` with both at 0, then with `b` at 4.
9. Empty `counters` and request the policies `["NotFour", "Nowhere"]`.
10. List `counters` with `complete: false` and request `NotFour`.
11. Omit `counters` from the snapshot, set domain `Int[0, 0]` and goal
    `true`, and request `NotFour`.

## Expected Results

- Step 1: no node with value 4.
- Step 2: `optimal` `inc, inc, inc, double`.
- Step 3: `inc, double, double` admitted; prefix `inc, inc` pruned.
- Step 4: the step to 12 excluded; the foreign policy refused per FR-806.
- Step 5: `infeasible` with an accepted closure certificate.
- Step 6: goal `true` is `optimal` with the empty plan without a policy and
  `inconclusive` `no-instance` with each policy; goal `false` is `infeasible`
  without a policy and `inconclusive` `no-instance` with `NotFour`.
- Step 7: `optimal` `inc, inc, inc, double`, then `inconclusive`
  `no-instance`.
- Step 8: `optimal` empty plan with `states` 1 and `work` 3; `infeasible`
  with `states` 1 and `work` 1.
- Step 9: `missing_declaration`/`missing-name` at `/policy/1`, no plan
  outcome.
- Step 10: `incomplete_population`/`incomplete-scope` at `/initial`, no plan
  outcome.
- Step 11: `incomplete_population`/`missing-required-artifact` at
  `/initial` before any search, no plan outcome.
