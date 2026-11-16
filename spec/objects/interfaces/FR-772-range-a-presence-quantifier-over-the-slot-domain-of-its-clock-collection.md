---
id: FR-772
title: "Range a presence quantifier over the slot domain of its clock collection"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-760
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
---

# FR-772: Range a presence quantifier over the slot domain of its clock collection

## Description

The quantifier of FR-760, `forall(e in self.q: present(e))` or `exists`, over
a clock collection place, binds `e` as a `parameter` of the selector's
domain (FR-322 "Presence"). Its first operand SHALL be the **slot domain**
of that collection at the owner term. The slot domain is the set of the
collection's live slots, written by their selectors: the positions
`0 .. size - 1` of a `Sequence` in ascending order, and the keys present in
a `Map` in key order. It is not the collection's element values, which for
a clock collection are clocks and have no value form. It is not their
presences either, because the quantifier needs a selector to bind `e`.
FR-415-AC-20's meaning follows: `forall` is true over an empty slot domain,
and `exists` is false over it.

### The slot-domain node

The slot domain is the application `filament.op.option.slots`, with form
`presence_read`, a derived `expression` node. Its arguments are, in order:

1. a reference to the `clock_place` template node of the collection's
   element place, the place whose last path step is the quantified `slot`
   step: the same node that the `present(e)` in the body references;
2. the owner term, as for `present(p)`;
3. one term for each `slot` step of that path except the last, in path
   order, each of its selector's domain. A collection directly under a field
   has none.

Its `result_type` is `Set<D>` with `D` the last slot step's selector domain,
or the `collection_bounds` `[0, N]` node over it when the collection's
declared maximum is `N`. The quantifier application's operand 0 references
this node, and its binder parameter has `semantic_type` `D`.

### Node identity when the quantifier names the clock place directly

When the source quantifies over the place itself (`e in self.q`), the
slot-domain node is still a derived node outside every recursion group.
Its `node_id` is `{package, path, ordinal}`, with `path`
`["expression", "presence_read", "filament.op.option.slots", {"operand": 0}, {"operand": 1}, ...]`:
one operand segment for each argument in order, the first naming the
`clock_place` node. The `clock_place` node appears in the path only as
that operand segment, never as an embedded `node_id` object and never by
its declaration path ("Structural node identity": a path never holds a
`node_id` object). `ordinal` is producer-assigned, so two quantifiers over
one place at one owner term share one slot-domain node.

### Reader rule I2 must admit

A strict reader SHALL admit, as operand 0 (the `collection_kind` operand) of
`filament.op.collection.forall` and `filament.op.collection.exists`, a
`filament.op.option.slots` application beside every collection-typed term,
under these checks:

- The application has `2 + (s - 1)` arguments, `s` being the number of
  `slot` steps of its `clock_place` node's path, `s >= 1`. Any other count
  refuses `ill_typed`/`operator-ineligible` at `/body/arguments`. A first
  argument that is not a `clock_place` node whose path ends in a `slot` step
  refuses `ill_typed`/`operator-ineligible` at `/body/arguments/0`.
- An owner term not of the owner type, or a slot term not of its step's
  selector domain, refuses `ill_typed`/`type-mismatch` at that argument.
- A quantifier over a slot domain whose binder parameter's `semantic_type`
  is not `D` refuses `ill_typed`/`type-mismatch` at the binder.
- A `filament.op.option.slots` application anywhere other than operand 0 of
  `forall` or `exists` refuses `ill_typed`/`operator-ineligible` at its
  node.

The rule that refuses a non-collection first operand
`ill_typed`/`operator-ineligible` is otherwise unchanged.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-772-AC-1 | FR-760-AC-1's package whose `forall` operand 0 is the `filament.op.option.slots` application over `q`'s element `clock_place` node and `self`, with no slot term, `result_type` the `collection_bounds` `[0, 2]` node over `Set<Integer>`, path `["expression","presence_read","filament.op.option.slots",{"operand":0},{"operand":1}]`, and binder parameter typed `Integer`, is admitted by the strict reader. The same package with an `exists` quantifier and a second quantifier over `self.q` shares the one slot-domain node. | Test (TC-1862) |
| FR-772-AC-2 | Negative: the slot-domain application with an extra slot term or with no owner refuses `ill_typed`/`operator-ineligible` at `/body/arguments`; with operand 0 a `scalar_type` node, at `/body/arguments/0`; with the binder typed `Boolean`, `ill_typed`/`type-mismatch` at the binder; used as the operand of `filament.op.collection.count`, `ill_typed`/`operator-ineligible` at its node; and a slot-domain path that embeds the `clock_place` `node_id` object refuses `malformed_wire` at that path segment. | Test (TC-1862) |

## Dependencies

- FR-760: the quantifier's scope in the predicate.
- FR-322 "Presence", "Structural node identity" and "Quantifier domain".
- FR-415-AC-20: the meaning of `forall` and `exists` over live slots.
