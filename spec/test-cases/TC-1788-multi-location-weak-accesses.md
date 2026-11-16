---
id: TC-1788
title: "Multi-location weak accesses decompose and multi-location RMWs lock"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-741
    type: verifies
---

# TC-1788: Multi-location weak accesses decompose and multi-location RMWs lock

## Description

Verify that multi-location weak accesses decompose and multi-location RMWs lock. Scope: FR-741-AC-1, FR-741-AC-2.

Catches a weak-access-shape refusal of a multi-location access.

## Test Procedure

1. Explore a two-field store and a two-field load in sibling threads under `tso` and `ra`; explore a two-location compare-and-swap.

## Expected Results

- Step 1: Stores flush in canonical order; the torn read is reachable; the compare-and-swap is one step, enabled under `tso` only with an empty buffer, never half applied.
