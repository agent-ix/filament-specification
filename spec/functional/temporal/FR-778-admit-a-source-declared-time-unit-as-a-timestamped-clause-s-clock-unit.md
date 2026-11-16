---
id: FR-778
title: "Admit a source-declared time unit as a timestamped clause's clock unit"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-009
    type: implements
  - target: ix://agent-ix/filament-specification/FR-370
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-414
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-965
    type: traces_to
---

# FR-778: Admit a source-declared time unit as a timestamped clause's clock unit

## Description

A timestamped-event clause's clock unit SHALL be the unit its
`temporal_bindings` row's `timestamp_unit` `T` names, whether an admitted
domain package declares it or the clause's own source unit does. When `T` is
the FCD identity of a domain-package unit, the clock unit node is the
model-owned node whose `node_id.path` is `["scalar_type", "unit", T]`; when `T`
is the name of a unit the clause's source unit declares, it is that
source-owned unit node, whose `node_id.path` is its declaration path `[T]` and
which carries the source unit's `SourceOwner`.

A window aggregate is most often written over quantities the author's own unit
declares, such as a p95 latency bound in milliseconds. A rule that admitted
only domain-package units would refuse that use although the package already
holds the one node every quantity of the unit names. An FCD identity is an
`ix://` URI and a declared unit name is an identifier, so `T` names at most one
node.

## Inputs

A checked package's timestamped-event clause, its `temporal_bindings` row and
its `duration` aggregate bounds (FR-370 "Window aggregate atom").

## Outputs

The clause's clock unit node, against which the reader matches each `duration`
bound's `type`.

## Behavior

- The clock unit node SHALL be a linear unit of a dimension named `Time`. A
  `T` that names no such unit has no clock unit node, and a `duration` bound
  then has no encoding: the producer writes no package for the clause.
- A producer SHALL write no second `scalar_type`/`unit` node to stand for a
  source-owned clock unit: the bounds' `type` is the unit's declared node.
- A reader SHALL match a `duration` bound's `type` against the clock unit node
  by `node_id.path` and `semantic_form`, for a source-owned node as for a
  model-owned one, and SHALL refuse a bound typed by any other unit node
  `ill_typed`/`unit-mismatch` at the bound's value (FR-370).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-778-AC-1 | A checked package whose source unit declares `dimension Time`, `s` and `ms` (`1/1000` of `s`), with a timestamped-event clause whose `temporal_bindings` row names `timestamp_unit` `ms` and whose `duration` `count` aggregate has `lower` `0` and `upper` `300000` typed by the source-owned `ms` node (`node_id.path` `["ms"]`, `SourceOwner`), is admitted, and holds no node whose path is `["scalar_type", "unit", "ms"]`; the same clause with `upper` typed by the source `s` node refuses `ill_typed`/`unit-mismatch` at `upper`'s value. | Test (TC-1890) |
| FR-778-AC-2 | The FR-370-AC-20 package, whose row names `ix://acme/rpc/unit/ms`, stays admitted with its bounds typed by the model-owned node; with its unit of compilation also declaring a source `ms`, a bound typed by that source node refuses `ill_typed`/`unit-mismatch` at the bound's value. | Test (TC-1890) |

## Dependencies

- FR-370: the window aggregate atom and the reader's checks.
- FR-414: window aggregate typing. FR-322: node identity and owners.
