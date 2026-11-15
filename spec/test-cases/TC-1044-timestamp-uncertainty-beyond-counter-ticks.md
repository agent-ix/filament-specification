---
id: TC-1044
title: "Bounded covers and interval stamps on timestamps"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-160
    type: verifies
---
# TC-1044: Bounded covers and interval stamps on timestamps

## Description

Verifies clock uncertainty on point timestamps through a bounded cover and on
single observations through interval stamps, read with the exact endpoint rules
(FR-160 "Timestamp uncertainty beyond counter ticks").

## Test Procedure

1. Under a bounded cover `u` 2 ms, a request stamped 0 ms and
   `eventually[0,10] holds(ack)` with completeness and watermark through
   17 ms, evaluate with `ack` stamped 5, 10 and 15 ms, then with `u` 0.
2. With no `ack`, evaluate at watermarks 11 and 12 ms.
3. Evaluate `p until[0,5] q` and `eventually[0,5] holds(q)` over observations
   with interval stamps `[3, 5]` and `[4, 6)` and no shared order authority.
4. Supply interval stamps `[5, 5)` and `[6, 5]`, and an interval stamp under a
   trace that declares a bounded cover.

## Expected Results

1. True, indeterminate and false under `u` 2; true, true and false under `u` 0
   (FR-160-AC-9).
2. Pending at 11 ms and false at 12 ms (FR-160-AC-9).
3. `until` refuses as unordered and `eventually` evaluates (FR-160-AC-10).
4. Both malformed stamps are refused as invalid input, the interval stamp
   overrides the cover, and no midpoint is used (FR-160-AC-10).
