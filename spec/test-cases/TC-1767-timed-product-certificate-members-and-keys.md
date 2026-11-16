---
id: TC-1767
title: "Timed-product certificate members and keys"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-725
    type: verifies
---

# TC-1767: Timed-product certificate members and keys

## Description

Checks the members, the digital and region key forms and the carriage of the
timed-product certificate over TC-374's `Retx`.

## Test Procedure

Run each vector through the real local Rust boundary of the exact provider and
of a checker run without the provider, and compare the complete typed result
(value, cause and payload) with the expected result. `Retx` and `Deadline` are
TC-374's.

## Vectors

| Vector | Input | Expected result | AC |
| --- | --- | --- | --- |
| TP-01 | `Deadline` at 0.99 over every scheduler, exact | `proved`, `exact-value{99/100}`; `objective.timed` is `{route: digital, unit: ms, scale: 1, form: deadline}`; `x` takes 0 to 2 and the deadline clock 0 to 5 across the keys; the checker accepts | FR-725-AC-1 |
| TP-02 | TP-01 with `send` guarded `x > 1 ms` | `route: region`; two keys differing only in the fractional order of `x` and the deadline clock stay distinct after a round trip; a region key replaced by the digital key of its integer parts fails the comparison | FR-725-AC-2 |
| TP-03 | TP-01's certificate with an added `components` member; the same with `time_locks`; the checked package of TP-01 | `certificate-rejected`, rule `malformed`, at `components` and at `time_locks`; the package has no certificate node | FR-725-AC-3 |

## Expected Results

Each result is exactly as stated.
