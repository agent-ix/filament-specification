---
id: TC-2041
title: "Timed liveness results carry the non-Zeno premise"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-416
    type: verifies
---

# TC-2041: Timed liveness results carry the non-Zeno premise

## Description

Verify the `non_zeno` result-entry member of every result decided on a timed liveness product, its absence elsewhere, its certification and its agreement across the zone and digital-clock routes. Scope: FR-416-AC-10.

## Test Procedure

1. Over `Rpc` with `T = 4 ms`, decide the response claim, its `reply` self-loop variant and its `echo` variant on the zone route.
2. Decide `always eventually holds(true)` over `Stall`.
3. Decide `NoLateReply` and the time-lock-freedom item over `Rpc`.
4. Change the stated `non_zeno` of the reply-loop proof's certificate to `not-required` and check it.
5. Decide the closed MITL claim `Within` on the zone and digital-clock routes.

## Expected Results

- Step 1: `proved` with `not-required`, `proved` with `assumed`, `refuted` with `checked`.
- Step 2: `inconclusive`, `no-admitted-behaviour`, with `refuted`.
- Step 3: no `non_zeno` member on either result.
- Step 4: the certificate is rejected.
- Step 5: both proofs carry the same `non_zeno`.
