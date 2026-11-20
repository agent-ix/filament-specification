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

The clause's source unit is the unit the row's `source_unit` names (FR-322
"Clause source unit"): the source unit that declares the clause, in the
identity form of a `lock.sources` entry. A reader SHALL resolve `T`, and every
other unit name the clause owns, in that unit. It SHALL NOT infer the unit from
the clause's subject, the source map, the order of `lock.sources` or the
position of any node. A clause over a `Reference<Order>` whose `Order` is a
selected model of another unit, or over a parameter of a record type its own unit declares, is
declared by its own unit either way, and its unit names are that unit's.

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

- The clock unit node SHALL be a linear unit of a dimension named `Time`: its
  dimension node is the dimension whose declaration (source-owned) or FCD
  identity (model-owned) names `Time`, and its target chain composes to an
  offset of zero. A producer writes no package for a clause whose `T` names no
  such unit.
- A reader SHALL check, for each timestamped-event clause that holds a
  `duration` window aggregate and before its aggregates, the clause's
  `temporal_bindings` row: a `timestamp_unit` that is neither the FCD identity
  of a unit of an admitted domain package nor the name of a unit the row's
  `source_unit` declares refuses `missing_declaration`/`missing-name`, even
  when another unit of the package declares that name, and one that names a unit of
  another dimension or an affine unit refuses `ill_typed`/`unit-mismatch`,
  each at the row's `timestamp_unit` pointer and retaining the clock name and
  `T`. A clause with no `duration` window needs no clock unit node, so its
  row keeps FR-322's rule (a non-empty name, such as `RFC3339`) and is not
  checked against the package's units.
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
| FR-778-AC-3 | Negative: FR-778-AC-1's package with its row naming `timestamp_unit` `us` (no such unit) refuses `missing_declaration`/`missing-name` at the row's `timestamp_unit`. | Test (TC-1890) |
| FR-778-AC-4 | Negative: FR-778-AC-1's package with the source unit also declaring `dimension Length` and `m`, and its row naming `timestamp_unit` `m`, refuses `ill_typed`/`unit-mismatch` at the row's `timestamp_unit`. | Test (TC-1890) |
| FR-778-AC-5 | Negative: FR-778-AC-1's package with the source unit also declaring an affine unit of `Time` (non-zero offset), and its row naming that unit, refuses `ill_typed`/`unit-mismatch` at the row's `timestamp_unit`. | Test (TC-1890) |
| FR-778-AC-6 | Each refusal of FR-778-AC-3, FR-778-AC-4 and FR-778-AC-5 retains the clock name and `T`, and is reported before any aggregate check: the same package with a `duration` bound also typed by the source `s` node reports only the row refusal. | Test (TC-1890) |
| FR-778-AC-7 | A clause with no `duration` window whose row names `timestamp_unit` `RFC3339` (TCB-03), a name no unit declares, is admitted. | Test (TC-1890) |
| FR-778-AC-8 | A clause declared in unit `example` over a `Reference<Order>`, where `Order` is declared by `acme/orders`, a selected model of another unit, with `Time`, `s` and `ms` declared in `example`, a row naming `timestamp_unit` `ms` and `source_unit` `{"authority": "agent-ix", "identity": "example"}`, and a `duration` `count` aggregate typed by `example`'s `ms` node, is admitted (`SU-01`; the clause shape and row of `fixtures/positive-all-families.json`, which carries the member). | Test (TC-1890) |
| FR-778-AC-9 | A clause `over (p: Pulse)` in unit `example`, where `example` declares `record Pulse` and `Time`, `s` and `ms`, with a row naming `timestamp_unit` `ms` and `source_unit` `example`, and a `duration` aggregate typed by `example`'s `ms` node, is admitted (`SU-02`). | Test (TC-1890) |
| FR-778-AC-10 | Negative: FR-778-AC-8's package with `Time`, `s` and `ms` declared only by `acme/orders`, the unit that declares the clause's subject, and not by `example`, refuses `missing_declaration`/`missing-name` at the row's `timestamp_unit` (`SU-03`): the reader resolves `ms` in `source_unit` and never in the subject's unit. | Test (TC-1890) |
| FR-778-AC-11 | Negative: FR-778-AC-8's package with its row's `source_unit` naming `{"authority": "agent-ix", "identity": "other"}`, which is not one of the package's `lock.sources`, refuses `invalid_package`/`invalid-value` at `/temporal_bindings/{i}/source_unit`, before any unit resolution (`SU-04`). | Test (TC-1890) |

## Dependencies

- FR-370: the window aggregate atom and the reader's checks.
- FR-414: window aggregate typing. FR-322: node identity and owners.
