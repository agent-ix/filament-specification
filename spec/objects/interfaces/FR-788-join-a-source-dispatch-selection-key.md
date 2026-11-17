---
id: FR-788
title: "Join a source dispatch selection key"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-150
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-610
    type: depends_on
---
# FR-788: Join a source dispatch selection key

## Description

When a checked-package reader validates an FR-494 dispatch table whose
receiver context is a source-declared object type node, it SHALL admit each
candidate's `selection_key` exactly when the key equals the FR-150
`EffectiveId` over the candidate's source declaration key (FR-610
"Declaration keys"), `{owner_effective_type: null, original: {package: I,
node: "ix://I/<type>"}}`, with `I` the `identity` of the `SourceOwner` of a
source-declared object type node of the package whose qualified name is
`[<type>]`, and SHALL refuse every other key `invalid_package`/`invalid-value`
at the candidate's `selection_key`.

## Rationale

FR-322-AC-290 requires the selection key to be the producer's exact FR-150
`EffectiveId`, matched against the candidate's admitted effective identity
and never inferred from a name. A source candidate's effective identity is
its source declaration key, which the reader can recompute from the
candidate node's `SourceOwner` and `declaration` alone, so the reader checks
the emitted key by recomputing it. A bare type name in `node` would be a name
the reader resolves, which FR-322-AC-290 forbids; a `package` other than the
owner's identity would name no declaration of the package.

## Behavior

- The reader SHALL find, for the key's `original.package` `I` and
  `original.node`, a source-declared `model`/`object_type` node whose
  `SourceOwner` `identity` is `I` and whose qualified name `[<type>]` gives
  `ix://I/<type>` equal to `original.node`; no other match is admitted.
- `owner_effective_type` SHALL be `null`.
- The matched node SHALL be the context or reach it through the identity
  rows' `supertypes` (FR-610 "Identity rows"); otherwise the dispatch join
  refuses the candidate as a model candidate outside the family is refused.
- Candidate order, uniqueness of keys and the selected-operation witness are
  FR-322-AC-290's.
- Wherever a checked package renders the type of a source-declared object
  type or native record as text (a clock place's `resolved_path`
  `declaring_type` and `member_type`, FR-789), the rendering SHALL be the
  `node` of its source declaration key, `ix://I/<type>`, as a domain
  package's type renders as its IR node identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-788-AC-1 | A package holding source-declared `Order` and `Sub extends Order` of one `SourceOwner` with `identity` `src/orders-unit`, and a dispatch table over `Order` with candidates whose `selection_key`s are `{null, {src/orders-unit, ix://src/orders-unit/Order}}` and `{null, {src/orders-unit, ix://src/orders-unit/Sub}}`, is admitted, the candidates in that order. | Test (TC-1915) |
| FR-788-AC-2 | Over the AC-1 package, each refuses `invalid_package`/`invalid-value` at `/checked_term_data/dispatch_tables/0/candidates/0/selection_key`: `original.node` `Order` (the bare name); `original.node` `ix://acme/unit/Order` with `package` `acme/unit` (the checked package's name); `original.package` `src/other-unit`, no `SourceOwner` of the package; `original.node` `ix://src/orders-unit/Ghost`, no such type; and a non-null `owner_effective_type`. | Test (TC-1916) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Source-declared object types"
  item 7 and FR-322-AC-290.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md)
  "Declaration keys" and "Identity rows".
