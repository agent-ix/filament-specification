---
id: TC-1768
title: "Timed-product certificate checker independence"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-751
    type: verifies
---

# TC-1768: Timed-product certificate checker independence

## Description

Tampers with the members, keys and values of TC-1767's TP-01 certificate and
runs the checker in a separate process over the four timed cases.

## Test Procedure

Run each vector through a checker that contains no part of the provider, over
the checked package and the certificate bytes, and compare the complete typed
result with the expected result.

## Vectors

| Vector | Input | Expected result | AC |
| --- | --- | --- | --- |
| CI-01 | TP-01's certificate with one value altered; with its identity changed | `certificate-rejected`, `inequality-fails` at the first failing state; `invalid_runtime_input`/`conflicting-identity` before the product is rebuilt | FR-751-AC-1 |
| CI-02 | TP-01's certificate with `route: region`; with `scale: 2`; with a key whose cap of `x` is 2 instead of 3 | objective mismatch, `invalid_runtime_input`/`invalid-value` (twice); `malformed` at `values` | FR-751-AC-2 |
| CI-03 | TP-01's certificate with one reachable state removed; with one unreachable key added | `malformed` at `values` (twice) | FR-751-AC-3 |
| CI-04 | The certificates of FR-752-AC-1, FR-752-AC-3, FR-753-AC-1 and FR-753-AC-4, each also with one value altered, in a separate process | each accepted; each altered one rejected | FR-751-AC-4 |
| CI-05 | The certificate of `expected elapsed until holds(false) >= 3 ms` over every scheduler, with empty `values` and `ranking`; the same with one value row added; the certificate of `expected elapsed until holds(m.delivered) >= 1 ms` with its rows removed | accepted; `certificate-rejected`, `malformed`; `malformed` | FR-751-AC-5 |

## Expected Results

Each result is exactly as stated; no vector is accepted on the provider's word.
