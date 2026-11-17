---
id: TC-1973
title: "Canonical export text and injective names"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-814
    type: verifies
---

# TC-1973: Canonical export text and injective names

## Description

Verify keyword-safe and injective names, canonical layout, dependency order and the name map. Scope: FR-814-AC-1 to FR-814-AC-5.

## Test Procedure

1. Export operations named `MODULE`, `Next` and `theorem`.
2. Export names `a_b`, `a__b`, `Shop::Order` and one with `é`.
3. Export one request twice.
4. Export a function used by an invariant; reorder two independent declarations.
5. Read a TLC trace back through the name map.

## Expected Results

- Step 1: prefixed names; both observers parse.
- Step 2: distinct mangled names, `_uE9_` for `é`.
- Step 3: byte-identical ASCII text with the stated layout.
- Step 4: the function first; only node order changes.
- Step 5: the FSL nodes.
