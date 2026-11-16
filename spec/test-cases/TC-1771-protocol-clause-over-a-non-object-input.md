---
id: TC-1771
title: "A protocol clause over a non-object input carries no owner model edge"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-727
    type: verifies
---

# TC-1771: A protocol clause over a non-object input carries no owner model edge

## Description

Check the `owner_model` edge for each kind of `over` type: absent when no object type is reached, the element object type for an option or a collection of references.

Scope: FR-727-AC-1, FR-727-AC-2, FR-727-AC-3.

## Test Procedure

1. Read clauses over `Boolean`, `Int[0, 3]`, `Parts::Handoff`, a tuple and `Sequence<Integer>` with no edge, then each with an added edge.
2. Read clauses over `Option<M::T>` and `Sequence<M::T>` with the edge to `M::T`, with no edge and with the edge to `M::U`.
3. Build the FR-375 class catalog over a package that holds a clause over `Int[0, 3]` and one over `M::T`.

## Expected Results

- Step 1: admitted without the edge; each with the edge refuses `malformed_wire` at the node.
- Step 2: admitted with either edge; refused `malformed_wire` without one.
- Step 3: the catalog holds only the clause over `M::T`.
