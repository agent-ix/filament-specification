---
id: TC-2026
title: "Source operation-body coverage joins the identity rows"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-790
    type: verifies
---

# TC-2026: Source operation-body coverage joins the identity rows

## Description

Verify that the identity-row join checks that the `operation_bodies` carrier holds exactly one row per operation the source types' identity rows list. Scope: FR-790-AC-3.

## Test Procedure

1. Join the FR-790-AC-1 package with its identity rows.
2. Remove `Order.bump`'s row, and separately add an `Order.ghost` row, and join each.

## Expected Results

- Step 1: the carrier is admitted.
- Step 2: the removed row refuses `invalid_package`/`invalid-value` at `/operation_bodies` and the added row at its `operation`; the strict read admits both packages.
