---
id: TC-1483
title: "Collection queries over unavailable occurrences"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-145
    type: verifies
---

# TC-1483: Collection queries over unavailable occurrences

## Description

Evaluates every query and conversion form over `q: Sequence<Int[0, 9]>`
holding an unavailable occurrence `u` before and after the decisive
occurrence, checking the completed values, the incomplete outcomes, their
loci and their charges.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| UQ-01 | `[1, 1, u]`: `contains(q, 1)`; `exists(x in q: x = 1)`; `forall(x in q: x = 2)` | Completed `true`, `true`, `false`; `u` not visited | FR-145-AC-13 |
| UQ-02 | `[u, 1]`: the same three queries; `[1, u]`: `forall(x in q: x = 1)` | Each incomplete at `u` | FR-145-AC-13 |
| UQ-03 | `[u, 1]` and `[1, u]`: `filter`, `map`, `count`, `sum` | Each incomplete at `u`, no partial output | FR-145-AC-14 |
| UQ-04 | `removeOne(q, 1)` on `[1, u]` and `[u, 1]`; `dropFirst(q)` on `[u, 1]` and `[1, u]`; `size(q)` on `[1, u]` | `[u]`, incomplete; `[1]`, `[u]`; 2 | FR-145-AC-14 |
| UQ-05 | Every incomplete outcome of UQ-02 to UQ-04 | Code `unavailable_observation`, cause `missing-required-valuation`, value path of `q` plus `index` 0 on `[u, 1]` and 1 on `[1, u]`; a visit charged for `u`, no retain; `contains` charges no `collection.member-walk` for `u` | FR-145-AC-15 |

## Expected Results

Short-circuiting queries complete only when the decisive occurrence precedes
every unavailable one; whole-source forms meet `u` as incomplete.
