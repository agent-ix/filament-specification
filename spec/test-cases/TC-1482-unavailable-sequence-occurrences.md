---
id: TC-1482
title: "Unavailable sequence occurrences"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-144
    type: verifies
---

# TC-1482: Unavailable sequence occurrences

## Description

Supplies sequences holding unavailable occurrences and collections that
cannot hold them, and compares sequences holding them, checking admission,
size, bound and equality.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| UO-01 | `Sequence<Int[0, 9]>[0, 3]` supplied `[{"type":"integer","value":"1"}, {"type":"unavailable"}]` | Admitted, two occurrences, the second unavailable; `size` 2 | FR-144-AC-29 |
| UO-02 | The same type supplied three integers and one `unavailable` entry | `cardinality_out_of_bound`/`above-maximum` | FR-144-AC-29 |
| UO-03 | An entry `{"type":"unavailable","value":"1"}` | `invalid_runtime_input`/`unknown-member` | FR-144-AC-29 |
| UO-04 | An `unavailable` entry in a supplied `Set<Int[0, 9]>`, `Bag<Int[0, 9]>` and `OrderedSet<Int[0, 9]>` | Each `invalid_runtime_input`/`invalid-value` at that entry | FR-144-AC-29 |
| UO-05 | `[1, u] = [1, 2, 3]`; `[2, u] = [1, 5]` | Each a completed `false` | FR-144-AC-30 |
| UO-06 | `[1, u] = [1, 2]`; `[u, 2] = [1, 3]` | Each incomplete at `u` | FR-144-AC-30 |

## Expected Results

Only sequences hold unavailable occurrences; they count as occurrences and
never compare equal or unequal.
