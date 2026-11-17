---
id: FR-783
title: "Type a size query at an explicit result domain"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-145
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-144
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-044
    type: references
  - target: ix://agent-ix/filament-specification/FR-339
    type: references
---

# FR-783: Type a size query at an explicit result domain

## Description

When a checker reads `size<N>(c)`, it SHALL type the query `N`, prove that
every length `c` can have lies inside `N`'s domain, and evaluate the query to
the same length `size(c)` returns.

## Rationale

`size<N>(c)` is the typed spelling of `size(c)` that the shared grammar's
`primary` and the state contract's collection table define: "the explicit or
uniquely expected dimensionless integer type must contain every admitted
length, including zero". It lets an author state the integer domain a length
is compared or stored in, such as a model's `M::Count` type, in the same way
`count<N>` and `sum<N>` name theirs. The untyped `size(c)` takes the bound
count's own interval (FR-145), and `size<N>(c)` adds no second meaning: it
returns the same number and only fixes its type. The domain rule is the one
`count<N>` follows, so a length that could leave `N` is refused when the unit
is checked, never discovered by a run that meets a long collection.

## Inputs

A `size<N>(c)` expression: the result type `N` and the operand `c`, an
expression of a `Sequence`, `Set`, `Bag`, `OrderedSet` or `Map` type.

## Outputs

A checked expression of type `N`, or a located refusal; at evaluation, the
length of `c`'s value.

## Behavior

1. **Result type.** `N` is a dimensionless integer type: `Integer`, an
   `Int[lo, hi]`, or a declared or model alias whose definition is one of
   them. `size<N>(c)` has type `N`. Any other `N`, such as `Boolean`,
   `Rational`, a `Quantity<u, ..>`, a record or a collection type, refuses
   `ill_typed`/`type-mismatch` at `N`. An `N` that names no type refuses as an
   unresolved type name does in every other type position.
2. **Operand.** `c` is checked with no expected type and must have a
   collection type FR-144 defines; any other operand type refuses
   `ill_typed`/`type-mismatch` at `c`, exactly as the untyped `size(c)` does.
3. **Range obligation.** The proved length interval of `c` is the bound of
   its collection type, narrowed by the facts in scope that bound `size(c)`
   (FR-146's guard facts, such as `if size(c) <= 3 then .. else ..`). A
   linked check SHALL refuse `undefined_expression`/`unproved-range` at the
   `size` call when that interval is not inside `N`'s domain; the refusal
   carries the required and the proved interval. A collection with no upper
   bound has a proved interval with no upper bound, which only an unbounded
   `N` contains. Zero is an admitted length of every collection whose
   declared minimum is `0`, so its `N` admits `0`.
4. **Value.** `size<N>(c)` evaluates `c` once and returns FR-145's bound
   count, the value `size(c)` returns for the same `c`, as a member of `N`.
   Direct kernel evaluation of a `size<N>(c)` whose length falls outside `N`
   refuses `integer_out_of_domain`/`outside-domain` at the `size` call
   (FR-044), as a `count<N>` does.
5. **Untyped form.** `size(c)` with no type argument keeps its type and value
   (FR-145); writing `<N>` never changes which collection is measured or how
   its occurrences are counted.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-783-AC-1 | For `q` of `Sequence<Integer>[0, 5]`, `Bag<Integer>[0, 5]`, `Set<Integer>[0, 5]`, `OrderedSet<Integer>[0, 5]` and `Map<Int[0, 9], Boolean>[0, 5]`, `size<Int[0, 5]>(q)` and `size<Integer>(q)` check with type `Int[0, 5]` and `Integer`; `size<Int[0, 3]>(q)` refuses `undefined_expression`/`unproved-range` at the `size` call with required `[0, 3]` and proved `[0, 5]`; `if size(q) <= 3 then size<Int[0, 3]>(q) else 0` checks; `size<Int[1, 5]>(q)` refuses `unproved-range` with proved `[0, 5]`, and over `Sequence<Integer>[1, 5]` it checks. | Test (TC-1900) |
| FR-783-AC-2 | `size<Boolean>(q)`, `size<Rational>(q)` and `size<Sequence<Integer>[0, 1]>(q)` refuse `ill_typed`/`type-mismatch` at the type argument; `size<Int[0, 5]>(3)` refuses `ill_typed`/`type-mismatch` at `3`; `size<Undeclared>(q)` refuses as an unresolved type name at `Undeclared`. | Test (TC-1900) |
| FR-783-AC-3 | Over `q` holding `7, 8, 7` as a `Sequence<Integer>[0, 5]`, `size<Int[0, 5]>(q)` and `size(q)` both evaluate to `3`; as a `Set<Integer>[0, 5]` holding `7, 8` both evaluate to `2`; a declared alias `type Count = Int[0, 10];` used as `size<Count>(q)` checks with type `Count`. | Test (TC-1900) |
| FR-783-AC-4 | Direct kernel evaluation of `size<Int[0, 1]>(q)` over three occurrences refuses `integer_out_of_domain`/`outside-domain` at the `size` call. | Test (TC-1900) |

## Dependencies

- [FR-145](FR-145-convert-and-query-collections.md) owns `size` and its bound count, and the `count<N>` result-domain rule this one matches.
- [FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md) owns the collection kinds and their bounds.
- [FR-146](FR-146-check-total-pure-functions.md) owns the guard facts that narrow `size(c)`.
- FR-044 owns `integer_out_of_domain`.
