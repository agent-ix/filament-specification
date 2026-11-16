---
id: TC-1769
title: "Strict-clock and every-scheduler delay certificates"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-752
    type: verifies
---

# TC-1769: Strict-clock and every-scheduler delay certificates

## Description

Proves and certifies TC-374's `Retx` with a strict guard and with a `discrete`
delay under every scheduler, and a tie decided by the scheduler.

## Test Procedure

Run each vector through the exact provider and a checker run without it, and
compare the complete typed result with the expected result.

## Vectors

| Vector | Input | Expected result | AC |
| --- | --- | --- | --- |
| RS-01 | `Retx` with guard `x > 1 ms`: `Deadline` at 0.99; `probability <= 0.999` of the same event | `proved`, `exact-value{99/100}`; `proved`, `exact-value{999/1000}`; both `region` certificates accepted | FR-752-AC-1 |
| RS-02 | RS-01 with a `digital` certificate; a `region` maximum certificate with initial value `9999/10000` | objective mismatch; `inequality-fails` | FR-752-AC-2 |
| RS-03 | `Retx` with `delay Msg::send ~ discrete { 1 ms: 1, 2 ms: 1 }`: `Deadline` over every scheduler; the same at 0.995 | minimum and maximum `159129/160000`, `proved`, certificate accepted; `refuted`, and the minimum's `exact` certificate for `>= 0.995` is rejected `threshold-missed` at the initial state | FR-752-AC-3 |
| RS-04 | RS-03's claim with the values of the distribution-free reading | `inequality-fails` | FR-752-AC-4 |
| RS-05 | FR-752-AC-5's race under every scheduler; under a workload weighting `A` and `B` 1 | maximum `1`, minimum `0`, `probability <= 0.5` `refuted`; maximum `1/2` | FR-752-AC-5 |

## Expected Results

Each value is the exact rational stated.
