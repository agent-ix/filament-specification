---
id: TC-1880
title: "An execute call settles its function's requirement items when the request asks"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-775
    type: verifies
---

# TC-1880: An execute call settles its function's requirement items when the request asks

## Description

Check that an `execute` request asking for settlement returns one negotiated terminal record per item of the called function beside an unchanged evaluation, and refuses a claim that is not an item of the call. Scope: FR-775-AC-1 to FR-775-AC-3.

## Test Procedure

1. Execute a function with three requirement items, asking for settlement, and again not asking.
2. Name an unregistered backend; then name a registered process provider.
3. Name a claim of another function.

## Expected Results

- Step 1: the same evaluation; three records in package order, then none.
- Step 2: three `unknown-backend` records, category refusal; one request with every item and the executed package's bytes, after negotiation.
- Step 3: refused before negotiation; no provider called.
