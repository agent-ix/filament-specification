---
id: FR-806
title: "Refuse a plan request with a typed cause and a locus"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-272
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-805
    type: references
---

# FR-806: Refuse a plan request with a typed cause and a locus

## Description

When a plan synthesis declaration or a plan request cannot be planned as
written, the checker or the planner SHALL refuse it with exactly one FR-272
code and cause from the table below, located at the source name or the
request member that causes it, before any search, and SHALL write no plan.

## Inputs

The checked `synthesis` declaration and a plan request (FR-805).

## Outputs

One located refusal, exit `20`.

## Behavior

### Check-time refusals

These are found when the package is checked (S3) and refuse the declaration:

| Defect | Code / cause | Locus |
| --- | --- | --- |
| `grammar` names nothing | `missing_declaration`/`missing-name` | the `grammar` name |
| `domain` of a plan synthesis is not an integer range type with both bounds | `unsupported_projection`/`unbounded-extent` | the `domain` name |
| `domain` contains a negative length | `ill_typed`/`type-mismatch` | the `domain` name |
| an operation parameter of `G`'s model has no finite value set (FR-800) | `unsupported_projection`/`unbounded-extent` | the parameter, naming the first unbounded component |
| `satisfies` is not Boolean | `ill_typed`/`non-boolean-root` | the block |
| the objective is not numeric (FR-802) | `ill_typed`/`type-mismatch` | the objective |
| the goal or objective reads `elapsed` over an untimed model, or `probability` over a non-probabilistic model | `missing_declaration`/`missing-name` | the member read |
| the `synthesis` uses a timed or probabilistic model and the unit's profile does not select the layer that model needs | `unsupported_construct` naming the selected and the required layer | the declaration |

### Request-time refusals

These are found when the request is admitted and refuse the request. Each
locus is a pointer into the plan request; a host request surface prefixes
it with the pointer of the member carrying the plan request (FR-805):

| Defect | Code / cause | Locus |
| --- | --- | --- |
| malformed request member (FR-805 "Request") | `invalid_runtime_input` with FR-805's cause | the member's pointer |
| `declaration` names no `synthesis` declaration, or one whose grammar is not a state model object type | `missing_declaration`/`missing-name` | `/declaration` |
| a policy name names no `temporal` declaration | `missing_declaration`/`missing-name` | `/policy/<i>` |
| a policy clause's `over` binding is not an object of `G`'s model | `ill_typed`/`type-mismatch` | `/policy/<i>` |
| the initial snapshot does not admit for `G`'s model | the snapshot's own admission refusal | `/initial` and the snapshot's member |
| an unknown member of `limits` or `accounting` | `invalid_runtime_input`/`unknown-member` (FR-805; FR-461-AC-4 governs only an unknown `--limit` setting name on the CLI) | `/limits/<member>` or `/accounting/<counter>` |

### Never a refusal

- A non-safety policy clause is evaluated, not refused (FR-803).
- A goal no plan reaches is `infeasible`, and a search stopped by a limit is
  `incomplete`; neither is a refusal.
- A plan that fails native replay is a method failure (FR-650), exit `30`,
  not a refusal.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-806-AC-1 | A `domain` naming `Integer`, and an operation parameter `n: Integer`, each refuse `unsupported_projection`/`unbounded-extent` at S3, located at the `domain` name and at the parameter respectively, and no plan request is read. | Test (TC-1956) |
| FR-806-AC-2 | A goal reading `Reach.elapsed` over an untimed model refuses `missing_declaration`/`missing-name` at `elapsed`; a `domain` `Int[-1, 3]` refuses `ill_typed`/`type-mismatch`. | Test (TC-1956) |
| FR-806-AC-3 | A request whose `declaration` names an ordinary `synthesis` over a value type refuses `missing_declaration`/`missing-name` at `/declaration`; a policy over another model refuses `ill_typed`/`type-mismatch` at `/policy/0`. | Test (TC-1956) |
| FR-806-AC-4 | A request whose initial snapshot violates the model's population bound refuses with that snapshot's own admission cause located at `/initial`, exit `20`, and writes no plan. | Test (TC-1956) |
| FR-806-AC-5 | A goal no plan reaches exits `10` as `infeasible`, never `20`. | Test (TC-1956) |

## Dependencies

- [FR-272](../../objects/foundation/FR-272-typed-diagnostic-cause.md) owns the
  codes and causes; [FR-805](FR-805-read-the-plan-request-and-write-the-plan-outcome.md)
  the request.
