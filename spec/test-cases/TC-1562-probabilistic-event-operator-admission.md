---
id: TC-1562
title: "Probabilistic event operator admission by profile"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-407
    type: verifies
---

# TC-1562: Probabilistic event operator admission by profile

## Description

Verify that a probabilistic event's operators are admitted by the temporal frontend's per-profile rule before the claim check, and that `unbounded-event` is left for the infinite-trace profile.

Scope: FR-407-AC-16.

## Test Procedure

Check a claim with each event below under the named profile through the real frontend boundary, then check the same formula as a temporal clause under the bounded profile, and compare the complete typed results.

## Vectors

| Vector | Profile | Event | Expected result |
| --- | --- | --- | --- |
| EV-01 | bounded | `[always holds(n.healthy)]` | `unsupported_construct` at `always`, owning claim and profile named; not `unbounded-event` |
| EV-02 | bounded | `[eventually[2, *] holds(n.healthy)]` | `unsupported_construct` at the interval; not `unbounded-event` |
| EV-03 | timed | `[eventually holds(P)]` | `unsupported_construct` |
| EV-04 | timed | `[eventually[0 ms, 4 ms] holds(P)]` | Admitted |
| EV-05 | infinite-trace | `[eventually[2, *] holds(P)]` | `invalid_probabilistic_model`/`unbounded-event` |

## Expected Results

Each vector gives exactly its expected result, and EV-01 and EV-02 give the same refusal as the same formula in a temporal clause under the bounded profile.
