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
  - target: ix://agent-ix/filament-specification/FR-803
    type: references
  - target: ix://agent-ix/filament-specification/FR-153
    type: references
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
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

One located refusal. Its exit is the exit of its code's FR-301 outcome
category; no refusal route sets an exit of its own. `incomplete_population`
is incomplete, exit `22`. The other codes these tables name are refusals,
exit `20`. A refusal passed on from snapshot admission exits as its own code
does.

## Behavior

### Check-time refusals

These are found when the package is checked (S3) and refuse the declaration:

| Defect | Code / cause | Locus |
| --- | --- | --- |
| `grammar` names nothing | `missing_declaration`/`missing-name` | the `grammar` name |
| `domain` of a plan synthesis is not an integer range type with both bounds | `unsupported_projection`/`unbounded-extent` | the `domain` name |
| `domain` contains a negative length | `ill_typed`/`type-mismatch` | the `domain` name |
| an operation parameter of `G`'s model has no finite value set (FR-800) | `unsupported_projection`/`unbounded-extent` | the parameter, naming the first unbounded component |
| repeated parameter names cannot share FR-322's arguments record after FR-154/FR-151 normalization | `unsupported_construct`/`not-yet-implemented` | the synthesis's `grammar` name; the existing diagnostic message names the parameter and first conflicting pair in operation/parameter declaration order |
| `satisfies` is not Boolean | `ill_typed`/`non-boolean-root` | the block |
| the objective is not numeric (FR-802) | `ill_typed`/`type-mismatch` | the objective |
| the goal or objective reads `elapsed` over an untimed model, or `probability` over a non-probabilistic model | `missing_declaration`/`missing-name` | the member read |
| the `synthesis` uses a timed or probabilistic model and the unit's profile does not select the layer that model needs | `unsupported_construct` naming the selected and the required layer | the declaration |

[FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md#synthesis-request-nodes)
owns shared-argument normalization, the support boundary, the diagnostic
feature name and conflict-pair selection.

### Request-time refusals

These are found when the request is admitted and refuse the request. Each
locus is a pointer into the plan request; a host request surface prefixes
it with the pointer of the member carrying the plan request (FR-805):

| Defect | Code / cause | Locus |
| --- | --- | --- |
| malformed request member (FR-805 "Request") | `invalid_runtime_input` with FR-805's cause | the member's pointer |
| `declaration` names no `synthesis` declaration, or, in a `filament.plan-request/v1` document, one whose grammar is not a state model object type (under a host surface's synthesis request such a declaration runs as a value synthesis, FR-823) | `missing_declaration`/`missing-name` | `/declaration` |
| a policy name names no `temporal` declaration | `missing_declaration`/`missing-name` | `/policy/<i>` |
| a policy clause's `over` binding is not an object of `G`'s model | `ill_typed`/`type-mismatch` | `/policy/<i>` |
| the initial snapshot does not admit for `G`'s model | the snapshot's own admission refusal | `/initial` and the snapshot's member |
| the initial snapshot lists a population with `complete: false` | `incomplete_population`/`incomplete-scope` (FR-153), exit `22` | `/initial` |
| the initial snapshot omits a population that overlaps a named policy clause's `over` type (FR-803) | `incomplete_population`/`missing-required-artifact` (FR-153), exit `22` | `/initial` |
| an unknown member of `limits` or `accounting` | `invalid_runtime_input`/`unknown-member` (FR-805; FR-461-AC-4 governs only an unknown `--limit` setting name on the CLI) | `/limits/<member>` or `/accounting/<counter>` |

A `complete: false` population is refused by snapshot admission, so that
refusal SHALL be decided before the policy name and policy `over` binding
refusals. The refusal of an omitted population needs each policy clause's
`over` type, so it SHALL be decided after every named policy clause has
resolved and its `over` binding has been accepted. An omitted population that the
snapshot itself refers to is refused earlier, by snapshot admission.

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
| FR-806-AC-3 | A `filament.plan-request/v1` document whose `declaration` names an ordinary `synthesis` over a value type refuses `missing_declaration`/`missing-name` at `/declaration`; a policy over another model refuses `ill_typed`/`type-mismatch` at `/policy/0`. | Test (TC-1956) |
| FR-806-AC-4 | A request whose initial snapshot violates the model's population bound refuses with that snapshot's own admission cause located at `/initial` and writes no plan; the bound violation is a refusal, exit `20`. This criterion fixes no exit for another admission refusal. | Test (TC-1956) |
| FR-806-AC-5 | A goal no plan reaches exits `10` as `infeasible`, never `20`. | Test (TC-1956) |
| FR-806-AC-6 | TC-2037's normalized shared-argument conflicts give `unsupported_construct`/`not-yet-implemented` at S3, exit `20`, at the synthesis grammar name, with the exact feature string defined by FR-322 and the existing declaration/profile cause context and deterministic parameter/operation-pair names in the diagnostic. No checked package or plan request/outcome is produced; optional `X` and required `Option<X>` do not create a conflict merely by their spelling. | Test (TC-2037) |
| FR-806-AC-7 | Over the counter with the policy list `["NotFour", "Nowhere"]`, where `Nowhere` names no declaration: a snapshot listing `counters` with `complete: false` refuses `incomplete_population`/`incomplete-scope` at `/initial`, exit `22`, and a snapshot that omits `counters` refuses `missing_declaration`/`missing-name` at `/policy/1`, exit `20`; neither writes a plan outcome. | Test (TC-1956) |
| FR-806-AC-8 | Over the counter with the policy list `["NotFour"]` and a snapshot that omits the `counters` population and holds no reference to it, the request refuses `incomplete_population`/`missing-required-artifact` at `/initial`, exit `22`, and writes no plan. | Test (TC-1956) |

## Dependencies

- [FR-272](../../objects/foundation/FR-272-typed-diagnostic-cause.md) owns the
  codes and causes; [FR-805](FR-805-read-the-plan-request-and-write-the-plan-outcome.md)
  the request.
