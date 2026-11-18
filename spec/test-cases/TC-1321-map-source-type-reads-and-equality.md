---
id: TC-1321
title: "Map source type, reads and equality"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-144-AC-33 FR-149-AC-28
    type: verifies
---

# TC-1321: Map source type, reads and equality

## Description

Verify the complete-V1 source type `Map<K, V>[a, b]`, its key-type rule, its reads (`m[k]`, `size`, `contains`), its canonical key and the FR-149 `Map` equality row.

## Test Procedure

Select `filament.value.complete/v1`. Declare the map parameters and field of FR-144-AC-33 and supply map values through the FR-181 `map` form. Check and evaluate the reads of FR-144-AC-33 and the equalities of FR-149-AC-28, and check the refused key types and mismatched map types.

## Expected Results

The admitted declarations and reads type and evaluate as FR-144-AC-33 states, an unguarded `m[k]` is `unproved-presence` at `k`, a non-scalar key type, and a domain-package `Ratio` with no constraints (the unbounded `Rational`), is `ill_typed`/`operator-ineligible` at the key type while a bounded `Rational[..]` key is admitted, the equalities give the Booleans FR-149-AC-28 lists independent of entry order, map types differing in bound or key type are `ill_typed`/`type-mismatch` under `=`, and an event-typed value type is `operator-ineligible`.
