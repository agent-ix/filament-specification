---
id: TC-976
title: "Authenticated schedulability claim carrier and reader boundary"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-524
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-525
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-526
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-527
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-528
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-529
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-530
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-531
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-532
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-533
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-534
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-535
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-536
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-537
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-418
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-482
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-482-AC-5
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-482-AC-6
    type: verifies
---

# TC-976: Authenticated schedulability claim carrier and reader boundary

## Description

Verify that the checked package carries an authenticated typed handle and
claim-scoped task-set, release and mode-switch relations for each
`claim`/`schedulability` occurrence; that each handle joins only to its exact
FR-482 claim node and occurrence; and that the IR typed read supplies the
FSL-owned FR-418 package route without constructing successors.

## Test Procedure

Read `proposals/checked-package-v2/schedulability-claim-vectors.json` and
`proposals/checked-package-v2/schema.json` by reference. Validate each positive
and schema-refusal row against `$defs.SchedulabilityClaimRef`, including its
required `task_set`, `release_bindings` and `mode_switch_bindings` members and
the two binding definitions. From a valid row, independently remove and add
each row and binding member, and replace `task_set`, `task_name`,
`operation_member`, `selected_model_identity` and
`operation_declaring_identity` with a value of the wrong shape; each wire-shape
defect must refuse `malformed_wire` at its exact row or member. For each
positive case, compile the named TC-967
source case and join `declaration` to the local `claim`/`schedulability` node.
Require one row for each exact `claim` occurrence; SC-01 has occurrence 0 and
the shared-node SC-06 has ordinals 0 and 1. On the `Ctl` fixture with
`release Ctl.a on M::go;` and `mode Ctl hi on M::enter_hi;`, select model
identity `acme/model` and require the exact release and mode-switch objects
defined by FR-322-AC-529 and FR-322-AC-530, in canonical order. Verify that
`operation_declaring_identity` is the selected model's declaring node `M`,
while `operation_member` separately holds `go` or `enter_hi`. Then change the
release task to `z`, the selected model identity to an unselected package, the
declaring identity to one absent from the selected model and the operation
member to one absent from that declaring identity; each strict read must refuse
at the exact binding field and return no typed claim view. With the graph and
claim body held fixed, independently change only `task_set`, only
`release_bindings`, and only `mode_switch_bindings` and verify that the claim
node id and obligation identity are unchanged. For the shared-node SC-06 case,
verify that its two occurrence rows retain their distinct relation arrays.
Construct ascending release and mode-switch arrays, then reverse each array
independently; strict read must admit the ascending form and refuse each
descending form at the first out-of-order row with
`invalid_package`/`invalid-value` and no admitted package.
For each existing
`reader_refusals` case, build the schema-valid package named by its `base_case`,
combining TC-960 and TC-967 source cases when specified; apply the mutation and
run the FR-322 carrier join before the FR-482 body read. For SCC-07, replace
the row declaration with TC-960/SN-01's exact `claim` NodeRef, whose node has
`semantic_form: "state_graph"`. In SCC-10, keep SC-06's two declared
occurrences but only occurrence 0's row; missing occurrence 1 must refuse
coverage. Require the FSL FR-418 package consumer to resolve the same pair,
select bindings by `task_set` and selected model, and use the admitted identities
without raw-wire reconstruction; the IR read performs no task-successor check.

## Expected Results

Positive rows and packages pass the closed wire schema; each is keyed by its
exact declaration-occurrence pair and carries one task-set identifier plus
zero or more release and mode-switch bindings. The SC-01 relation arrays are
empty; the `Ctl` relation fixture admits the exact task/model/operation
identities. SC-06 returns two distinct typed claim views with their own row
relations, despite the shared node and body. A missing task name refuses
`invalid_package`/`invalid-value` at
`/schedulability_claims/{i}/release_bindings/{j}/task_name`; an unselected
model or unresolved operation identity/member refuses the same code and cause
at that exact field and exposes no package view. Schema cases refuse
`malformed_wire`; existing schema-valid carrier mutations retain the exact
`invalid_package`/`invalid-value` or `invalid_package`/`conflicting-definition`
outcomes and loci. Missing coverage refuses at `/schedulability_claims`, and
each descending row or binding array refuses at its first out-of-order row.
The IR returns the admitted typed relations and does not evaluate successors.
FSL consumes those relations and recomputes successors. No
`TaskTransitionTemplate` or `TaskProductCertificate` field is added to
checked-package V2.
