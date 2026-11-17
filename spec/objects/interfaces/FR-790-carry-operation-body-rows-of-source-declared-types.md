---
id: FR-790
title: "Carry operation-body rows of source-declared types"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-610
    type: depends_on
---
# FR-790: Carry operation-body rows of source-declared types

## Description

When a checked package carries `operation_bodies`, the carrier SHALL hold one
row per operation member of every source-declared `object_type` node, with
`document` the node's `SourceOwner` `identity`, `declaring` the node,
`operation` the source operation name, and `body` a `function` naming the
`function` node with that `SourceOwner` and the `declaration`
`[<type>, <operation>, "body"]` exactly when the package holds one, and
`no_body` otherwise; and a reader SHALL check those rows as it checks model
rows, joining `document` and the body through the `SourceOwner` and the
`declaration` rather than through a `ModelOwner` member identity.

## Rationale

FR-322-AC-397 requires a row for every operation member of every admitted
`object_type` node and joins each dispatch candidate to the row of the type
declaring its effective operation. A source-declared node is admitted, and
its dispatch candidates carry source body clause functions, so a carrier
without source rows is partial and a source candidate could join no row.
The body derivation is the model rule read through the identity a source
clause function carries: its `SourceOwner` and `declaration` stand where a
model clause function's `ModelOwner` member stands.

## Behavior

- `document` SHALL be the declaring node's `SourceOwner` `identity`; a
  string that is no `SourceOwner` identity of the package refuses
  `missing_declaration`/`missing-selection` at `document`, and another
  owner's identity `invalid_package`/`invalid-value` at `document`.
- `body` SHALL be `function` exactly when the body clause function exists
  (`invalid_package`/`invalid-value` at `body` otherwise), and `body.node`
  SHALL be that clause function (`invalid_package`/`invalid-value` at
  `body/node` for a precondition, another operation's body or another
  owner's function).
- An operation that a source type only inherits has no row; coverage,
  key order and the candidate join are FR-322-AC-397's.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-790-AC-1 | A package with source-declared `Order` (operations `size` with body clause function `["Order", "size", "body"]`, and `bump` without) and `Sub extends Order` (`size` redefining it, body `["Sub", "size", "body"]`), with a dispatch table over `Order` whose candidates are `Order` and `Sub`, carries three rows, each `document` the `SourceOwner` `identity`, `Order.size` and `Sub.size` naming their own body functions and `Order.bump` `no_body`, and is admitted with each candidate joined to its own row. | Test (TC-1919) |
| FR-790-AC-2 | Over the AC-1 package, each refuses: `Order.size`'s `body` `no_body` and `Order.bump`'s `body` a `function`, `invalid_package`/`invalid-value` at `body`; `Order.size`'s `body.node` naming its precondition clause function, `invalid_package`/`invalid-value` at `body/node`; `Sub.size`'s row removed, `invalid_package`/`invalid-value` at `/operation_bodies`; `Order.size`'s `document` `src/other-unit`, `missing_declaration`/`missing-selection` at `document`. | Test (TC-1920) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Operation-body carrier",
  FR-322-AC-397 and "Source-declared object types" items 5, 6 and 9.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md).
