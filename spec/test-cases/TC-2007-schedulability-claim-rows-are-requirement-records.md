---
id: TC-2007
title: "Schedulability claim rows are the package's task-set requirement records"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2007: Schedulability claim rows are the package's task-set requirement records

## Description

Verify that a checked package's `schedulability_claims` rows give one
`schedulability` request item per task-set claim. Scope: FR-322-AC-456.

## Test Procedure

1. Compile the `Ctl` unit with `CtlFp`, `CtlEdf` and `R`; write request items
   from its `schedulability_claims` rows.
2. Compile a unit with no task-set claim.

## Expected Results

- Step 1: three `schedulability` items in row order, `bounded`, with their
  policies.
- Step 2: no `schedulability` item.
