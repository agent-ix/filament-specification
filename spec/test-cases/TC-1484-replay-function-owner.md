---
id: TC-1484
title: "Replay of an imported function by its owner"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-323
    type: verifies
---

# TC-1484: Replay of an imported function by its owner

## Description

Builds `function` replay requests with and without `function_owner` for a
unit `q` importing the library `test/units`, validates them against
`proposals/conformance-corpus-v1/schema.json` and executes them.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| FO-01 | `{"function": "big", "function_owner": "test/units"}` with `test/units` as the one `dependencies` entry | Schema-valid; executes the library's `big`, arguments joined by the library's parameter node ids; round-trips with its owner | FR-323-AC-36 |
| FO-02 | FO-01 with the unit also declaring `big`; and FO-01 without `function_owner` | The owner-carrying request executes the library's `big`; the ownerless one the unit's | FR-323-AC-36 |
| FO-03 | `function_owner` beside `frame_operation`, and beside `state_clause` | Each fails the schema and refuses `invalid_runtime_input`/`unknown-member` at `/replay/function_owner` at decode, before any recompile | FR-323-AC-37 |
| FO-04 | `function_owner` `test/absent`; and, for a unit importing only `test/wrap` that imports `test/units`, `function_owner` `test/units` | Each `missing_import`/`missing-selection` at `/replay/function_owner` | FR-323-AC-37 |
| FO-05 | Owner-carrying requests for `nope` and for a function whose signature names a record of `test/units` | `missing_declaration`/`missing-name` and `ill_typed`/`operator-ineligible`; no call, no charge, no result | FR-323-AC-37 |

## Expected Results

The owner selects the library whose function replay executes, and every
misuse refuses before a call.
