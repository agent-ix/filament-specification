---
id: TC-1080
title: "Analyze request and result document pair"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-520
    type: verifies
---

# TC-1080: Analyze request and result document pair

## Description

Reads `native-analyze/1` requests and checks the result items against the
FR-331 and FR-352 wire. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| AN-01 | One item naming a holding temporal clause | One result item: valid FR-331 request item with `request_index` 0 and explicit limits, record `proved`, valid FR-331 accounting record | FR-520-AC-1 |
| AN-02 | Items carrying `request_index`, `extent`, `candidates` | Whole request refused `invalid-request`, exit `20` | FR-520-AC-2 |
| AN-03 | One item naming a violated clause | FR-331 counterexample entry and replay target; every category and label registered by FR-352 | FR-520-AC-3 |
| AN-04 | Partial-order reduction on a bounded MLTL claim beside a second item | First item `inconclusive` `reduction-not-preserving`; second settles as alone | FR-520-AC-4 |

## Expected Results

The request round-trips through the FR-331 item vocabulary and the result
carries only registered kinds.
