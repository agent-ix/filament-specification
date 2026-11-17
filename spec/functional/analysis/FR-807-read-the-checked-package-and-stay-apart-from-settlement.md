---
id: FR-807
title: "Read plans from the checked package and keep plan search apart from verification settlement"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-492
    type: references
  - target: ix://agent-ix/filament-specification/FR-290
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
---

# FR-807: Read plans from the checked package and keep plan search apart from verification settlement

## Description

When the planner runs, it SHALL read the declaration, the model, its
operations and rules and the policy clauses only from the checked package's
nodes listed below, and SHALL remain a separate engine from the verification
planner of FR-492: a plan outcome is never a premise result, and a planner
settlement request never carries a plan.

## Inputs

The checked package (checked-package v2 nodes).

## Outputs

The plan space and rules FR-800 to FR-803 use.

## Behavior

### Nodes read

| Node (`node_tag`/`semantic_form`) | Read for |
| --- | --- |
| `claim`/`synthesis_request` | the declaration: `grammar`, `domain`, objective direction and expression, `satisfies` block |
| `model`/`model_root`, `model`/`object_type` | `G`, its fields and its model |
| `bounded_domain`/`integer_range` | `D`, and each bounded parameter domain |
| `bounded_domain`/`model_population` | the population bounds of each object type |
| `state`/`operation_anchor`, `state`/`transition`, `state`/`frame` | each operation, its precondition, postcondition and frame |
| `state`/`state_clause` | the model's state clauses and invariants |
| `state`/`snapshot` | the shape the initial snapshot admits against |
| `temporal`/`temporal_clause` | each named policy clause |
| `timed_model`/* | the time source, clocks, guards, resets, time invariants and urgency of a timed model (FR-808) |
| `function`/`pure_function` | functions the goal, objective or rules call |

The planner SHALL NOT read source text, a display name or a node outside
the package. A plan outcome names the package by its identity and the
declaration by its node.

### Separation from verification settlement

- A `claim`/`synthesis_request` node has no FR-290 declaration family, so
  no FR-331 verification kind applies to it and it derives no premise slot;
  it never appears in a `filament.planner-settlement/v1` request (FR-492).
- A `filament.plan-outcome/v1` document SHALL NOT be read as an FR-331
  result entry or as a premise result. FR-492's admission refuses it as a
  `result` that FR-331's reader refuses.
- The plan search and the verification planner SHALL share no limit, no
  cache and no certificate: `plan_search.*` and `planner.max_work` are
  distinct settings (FR-801).
- A plan's certificates are checked by the core's plan certificate checker
  (FR-650), whose acceptance gives the outcome its FR-331 certification
  value; that value is a label on the plan outcome and settles no
  verification item.
- A reported plan MAY be handed to verification as a subject: the plan's
  steps are a finite execution (FR-181) and a clause checked over that
  execution is an ordinary request item with its own settlement.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-807-AC-1 | A package whose `synthesis_request` node, operation anchors, transitions, state clauses and one temporal clause are present plans as FR-650-AC-4 states; the same package with the temporal clause node removed refuses the policy as FR-806 states, though the source text still holds the clause. | Test (TC-1957) |
| FR-807-AC-2 | A `filament.planner-settlement/v1` request that carries a plan outcome document as a slot `result` refuses `invalid_runtime_input` at `/slots/<i>/result`. | Test (TC-1957) |
| FR-807-AC-3 | Setting `planner.max_work` to 1 leaves a plan run unchanged, and setting `plan_search.max_work` to 1 leaves a planner settlement unchanged. | Test (TC-1957) |
| FR-807-AC-4 | A reported plan's steps replay as an FR-181 finite execution, and the invariant checked over that execution settles as its own item, independent of the plan's certification. | Test (TC-1957) |

## Dependencies

- [FR-492](../../objects/interfaces/FR-492-planner-settlement-request.md),
  [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) and
  [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md).
- The checked-package v2 schema owns the node forms.
