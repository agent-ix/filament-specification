---
id: TC-1820
title: "Time-divergent and elapsed-quantile certificates"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-753
    type: verifies
---

# TC-1820: Time-divergent and elapsed-quantile certificates

## Description

Proves and certifies items over TC-374's `Retx` with a cycle that takes no
time, a retry cycle that does, elapsed quantiles and a time-lock.

## Test Procedure

Run each vector through the exact provider and a checker run without it, and
compare the complete typed result with the expected result. `M` is
`elapsed from holds(not m.delivered) until holds(m.delivered) within 4 ms`.

## Vectors

| Vector | Input | Expected result | AC |
| --- | --- | --- | --- |
| TD-01 | `Retx` with `toggle` (always enabled, flips `spun`, no time): `Deadline` at 0.99 | `proved`, `exact-value{99/100}`; certificate accepted, one value per toggle pair | FR-753-AC-1 |
| TD-02 | TD-01 with every undecided value 0 | `inequality-fails` at a state of a toggle component | FR-753-AC-2 |
| TD-03 | `expected elapsed until holds(m.delivered) >= 1 ms`; the same with the initial value `1 ms` | `proved` on `10/9 ms`; `inequality-fails` | FR-753-AC-3 |
| TD-04 | `quantile 0.5 of M <= 2 ms`; `quantile 0.5 of M <= 1 ms`, and a certificate with initial value `1/2` | `proved`, transform minimum `2/5`, certificate accepted; `refuted`, transform minimum `−1/2`, `inequality-fails` | FR-753-AC-4 |
| TD-05 | `quantile 0.5 of M >= 1 ms`; `quantile 0.9 of M >= 2 ms` with an `upper` certificate of initial value 0 | `proved`, `region` `upper` certificate with initial value `−1/2` accepted; `refuted`, `inequality-fails` at the initial state | FR-753-AC-5 |
| TD-06 | `Retx` under a workload weighting `send` 1 with `delay Msg::send ~ discrete { 5 ms: 1 }`: `probability <= 0.999 [eventually[0 ms, 4 ms] holds(m.delivered)]`; a certificate valuing the initial state 1 | `proved`, `exact-value{0}`, certificate accepted; `inequality-fails` | FR-753-AC-6 |

## Expected Results

Each value is the exact rational stated.
