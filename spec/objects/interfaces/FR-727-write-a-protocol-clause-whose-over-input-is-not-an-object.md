---
id: FR-727
title: "Write a protocol clause whose over input is not an object reference"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-464
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-375
    type: references
---

# FR-727: Write a protocol clause whose over input is not an object reference

## Description

A checked package SHALL carry a protocol clause whose `over` input has any
value type: an object reference, an `option` or collection of object
references, a scalar, a bounded domain, a record, a tuple, an `option` or a
collection of values. The clause's `owner_model` edge (FR-464) SHALL name the
object type the `over` type *reaches*:

- for `Reference<T>`, the object type `T`;
- for `Option<Reference<T>>` and a `Sequence`, `Set` or `Bag` of
  `Reference<T>`, the element object type `T`.

A clause whose `over` type reaches no object type (a scalar, a record, a
tuple, or an `option` or collection of those) SHALL carry no `owner_model`
edge. It belongs to no model's refinement class catalog (FR-375). Its roles
and relationships still name their own model nodes.

### Decision: the edge is optional instead of invented

A protocol over a job number or a request record is a common shape, for
example a saga started for each order line number. Requiring an owner model
for such a clause would force the producer to invent one, such as the model
one of its roles names. That choice is arbitrary when roles name different
models, and it would add the clause to a class catalog it does not belong to.
An absent edge states the fact that the clause is over no model object. The
`option` and collection cases name the element type, because the clause is
still over objects of that model.

## Behavior

- The reader's owner-model edge phase (FR-464) refuses
  - a missing edge on a clause whose `over` type reaches an object type, and
  - an edge on a clause whose `over` type reaches none,

  each `malformed_wire` at the node.
- The edge's typed check is unchanged when the edge is present.
- The `over` binder, its stage reads (FR-464 "Binders") and the slots that
  may read `over` are the same whatever the `over` type.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-727-AC-1 | Clauses over `Boolean`, `Int[0, 3]`, a model record `Parts::Handoff`, a tuple and `Sequence<Integer>` are admitted with no `owner_model` and with dependencies that hold no model object-type node; each with an added `owner_model` edge refuses `malformed_wire` at the node. | Test (TC-1771) |
| FR-727-AC-2 | Clauses over `Option<M::T>` and `Sequence<M::T>` are admitted with `owner_model` naming `M::T`'s object-type node; each with the edge removed refuses `malformed_wire` at the node, and one whose edge names another object type `M::U` is admitted (the edge is authoritative and is not compared with `over`). | Test (TC-1771) |
| FR-727-AC-3 | A model refinement's class catalog (FR-375) over a package holding a clause over `Int[0, 3]` and a clause over `M::T` includes only the clause over `M::T`. | Test (TC-1771) |

## Dependencies

- [FR-464](FR-464-checked-package-protocol-node-bodies.md): the `owner_model`
  edge and its reader phase.
- [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md): the
  class catalog that joins clauses through the edge.

## Status

Proposed complete-V1 boundary contract.
