---
id: TC-1163
title: "Progress ledger on the run result"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: verifies
---

# TC-1163: Progress ledger on the run result

## Description

Read and write the `ledger` member of a native-run-result/2 clause-run record (FR-352-AC-18).

## Test Procedure

Read a timestamped record with a one-entry `ledger` and re-encode it; add the same `ledger` to a lasso record; remove `covered` from an entry; repeat a binding; swap two entries.

## Expected Results

- The timestamped record reads back equal and re-encodes to the same bytes.
- Each defect refuses with a located wire diagnostic naming the member.
