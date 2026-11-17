---
id: FR-789
title: "Join the clock places of a source timed model"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-610
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
---
# FR-789: Join the clock places of a source timed model

## Description

When a checked-package reader validates a `timed_model` root whose `model`
is a source-declared `model`/`object_type` node, it SHALL join every
`resolved_path` row of the root's `clock_place` nodes to source rows: a
`field` row whose `declaring_type` is the source declaration key `node` of a
source-declared object type of the root's `SourceOwner` to that type's
identity row (FR-610 "Identity rows"), and a row naming a native record of
that `SourceOwner` to the field binding of the record's declared node. It
SHALL refuse a row that joins none of them `invalid_package`/
`malformed-clock-place` at the row.

## Rationale

FR-322-AC-124 joins every row to the declaration of its path step so that a
forged path refuses from the package alone. A source timed model has no
selected document; its declarations are its source-declared nodes, their
identity rows and the declared nodes of its native records, which carry each
fact a row states. Joining through them keeps the one rule, a row joins the
declaration that owns its step, and keeps a source model's clocks apart from
a selected model's: a source root never resolves a row through a selected
document.

## Behavior

- A row's `declaring_type` SHALL be `ix://I/<type>` for `I` the root
  `SourceOwner` `identity` and `<type>` a source-declared object type or
  native record of that owner.
- For an object type, `member_identity` SHALL be `<declaring_type>/<field>`
  for a `members` entry of its identity row named by the path step, and
  `member_type` SHALL equal that entry's rendered member type, presence
  included as FR-322-AC-157 states for `Option`.
- For a native record, the same SHALL hold against the binding of that field
  in the record node's body.
- `tuple`, `union` and `slot` rows, the terminal `Clock` row, the aggregate
  order and the repeated-path rule are unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-789-AC-1 | A source timed model with `SourceOwner` `identity` `I`, `record W { c: Clock; }`, `object O { w: W; d: Clock; }` and `time O dense unit ms;`, whose root's `model` is `O`'s node, is admitted with two places: `d`, row `{ix://I/O, ix://I/O/d, ix://filament/native/Clock}`, and `w.c`, rows `{ix://I/O, ix://I/O/w, ix://I/W}` then `{ix://I/W, ix://I/W/c, ix://filament/native/Clock}`. | Test (TC-1917) |
| FR-789-AC-2 | Over the AC-1 package, each refuses `invalid_package`/`malformed-clock-place` at `/semantic_graph/nodes/{i}/body/members/2/value/members/{j}` of the edited row: `d`'s `member_identity` `ix://I/O/e`; `d`'s `declaring_type` `ix://J/O` for an identity `J` of no `SourceOwner` of the root; `d`'s `member_type` `Integer`; and `w.c`'s second row's `declaring_type` `ix://acme/orders/Order`, a selected model's type. | Test (TC-1918) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Timed-model nodes" and
  "Source-declared object types" item 8.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md)
  "Declaration keys" and "Identity rows".
