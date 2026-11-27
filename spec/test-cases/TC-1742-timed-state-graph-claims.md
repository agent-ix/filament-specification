---
id: TC-1742
title: "State-graph claims over a timed model subject"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-712
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-391
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-393
    type: verifies
---

# TC-1742: State-graph claims over a timed model subject

## Description

Check the three state-graph forms over TC-369's timed `Rpc` vector: timed
reachability, the `E(P)` fixpoint, path counting that ignores delays, exact
rational evidence, and routing that excludes untimed backends.
Scope: FR-712-AC-1 to FR-712-AC-12.

## Test Procedure

1. Check `possible` of `phase = Replied` and of `phase = TimedOut` at `T = 4 ms`.
2. Check `always possible` of `phase = Idle`, then over the model without `reset`.
3. Check `unique path` from `phase = Idle` to `phase = Replied` with and without `reset`.
4. Name a backend listing the forms that offers neither timed method; repeat step 1 with `options.method` `digital-clocks`.
5. Check `possible` of `phase = Replied` and of `phase = TimedOut` over the Zeno-only variant of `Rpc` (FR-712-AC-5).
6. Run TC-349 SG-06, SG-07, SG-09, SG-11 and SG-12 on the `zones` route. For each fixture, add a clock `z` with initial value `0 ms`, invariant `z <= 0 ms` in every state, and reset `z := 0 ms` on every operation. This timed overlay preserves the fixture's state and transition graph and admits no positive delay.
7. Read the steps of step 1's `possible` witness, of step 2's trap `stem` without `reset`, of step 3's path pair with `reset` (`stem`, `first` and `second`), and of the `undefined` stems of SG-07, SG-09, SG-11 and SG-12 from step 6, and validate against the `filament.backend-provider/v1` `GraphStep` schema a step with `delay` `{"numerator": "0", "denominator": "1"}`, the same step with no `delay`, with `delay` `"0"` and with a `delay` that has no `denominator` (FR-712-AC-11).
8. Over FR-712-AC-10's two-clock subject, check `always possible` and `unique path` on the `zones` route, with `one`'s guard as given and as `y >= 2 ms and x < 2 ms`; check a certificate that lists whole zones of the subject with `one` never enabled.
9. Over FR-712-AC-12's immediate-escape subject, check `always possible` and `unique path` on the `zones` route; repeat with the further step from `phase` 1 to 3; then check `unique path` with the further step from `phase` 0 to 2.

## Expected Results

- Step 1: holds with a replayed `send`, `reply` witness and a delay in `[1, 3]`; then does not hold.
- Step 2: holds with the `state_graph` part's `ef_nodes` in the certificate; then refuted ending at `phase` `Replied`.
- Step 3: refuted by the `reset` cycle; holds without `reset`.
- Step 4: `unsupported` `unsupported-requested-capability`; with `digital-clocks`, the same verdicts with basis `exhaustive`.
- Step 5: `Replied` proved with `zone-certified` basis and no `admitted` member, `TimedOut` refuted, and neither is refused for a missing divergence witness.
- Step 6: SG-06 settles from its depth-1 trap as `refuted`, `closed-scope`; SG-07 settles from its depth-1 undefined predicate as `refuted`, `decisive-counterexample`, cause `undefined-evaluation`. SG-09 settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation` with the `audit(a)` guard's transition identity and `absent-key` reason (the guard is not disabled). SG-11 settles the same cause from the undefined `Lost` terminal predicate with no predicate or transition member in `where`. SG-12 settles the same cause from the undefined initial state constraint with no predicate or transition member in `where`.
- Step 7: every step read carries `delay`, and every step of the step 6 stems carries `{"numerator": "0", "denominator": "1"}`; the schema admits the step with the rational `delay` and the step with none, and refuses the string and the `delay` with no `denominator`.
- Step 8: `refuted` for both claims as given, `proved` for both with the second guard, and the certificate rejected (FR-712-AC-10).
- Step 9: `inconclusive`, `lasso-not-concretized`, with no counterexample; then `refuted` for both with a `trap` stem ending in `phase` 3; then `refuted` with a path pair (FR-712-AC-12).
