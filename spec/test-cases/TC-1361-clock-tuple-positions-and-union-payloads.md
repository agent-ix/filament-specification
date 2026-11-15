---
id: TC-1361
title: "Clock tuple positions and union payloads"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-154
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-415
    type: verifies
---

# TC-1361: Clock tuple positions and union payloads

## Description

Check that a `Clock` tuple position and a `Clock` variant payload are admitted
as clock members, are read and written only as clock places, and hold no data.

## Test Procedure

Use a timed domain package (dense time, unit `ms`) whose object type `Call`
declares `s: Slot` and `p: Phase`, where tuple `Slot` has `p0` typed
`ix://filament/native/Clock` and `p1` typed `Integer`, and variant type `Phase`
has the cases `Armed` (`payloadType` `ix://filament/native/Clock`) and `Idle`
(no payload). Run each vector through FR-154 intake, the FR-322 reader and the
FR-415 checker.

| Vector | Input | Expected |
| --- | --- | --- |
| CP-01 | intake of the package; then the `model-tuple-union-vectors.json` cases `TK-08` and `TK-09` | admitted; both admitted |
| CP-02 | the checked package's clock places | one `clock_place` with path `field(s)`, `tuple(0)` and one with path `field(p)`, `union(Armed)`, each with a `resolved_path` row per step ending at the `Clock` terminal; no node stands for either clock; the `Slot` tuple node and the `Phase` variant node are unchanged by the clock members |
| CP-03 | the guard `present(self.p.Armed) and value(self.p.Armed) >= 1 ms` on an operation; then `value(self.p.Armed) >= 1 ms` alone | one convex part carrying the data predicate `present(self.p.Armed)`, true exactly while `p` is `Armed`; `undefined_expression`/`unproved-presence` at the read |
| CP-04 | the postcondition `self.p = Phase::Armed(0 ms)`; then `self.p = Phase::Idle` | a `clock_reset` of the `Armed` place beside a `union_value` with the empty payload; the place is dropped, with no presence bit in either state |
| CP-05 | the guard `self.s.0 >= 2 ms`; then `self.s.0 = self.s.0` in an invariant | one convex part on the `Slot` place; `invalid_timed_model`/`clock-read-outside-constraint` at the projection |
| CP-06 | a field `ss: Set<Slot>[0, 2]`; a map type keyed by `Slot`; a map type keyed by `Phase` | a clock collection; `ill_typed`/`operator-ineligible` at each key type |
| CP-07 | `s1 = s2` over two `Slot` values, and `p1 = p2` over two `Phase` values | each `invalid_timed_model`/`clock-read-outside-constraint` at the comparison |

## Expected Results

Every vector gives exactly its expected outcome.
