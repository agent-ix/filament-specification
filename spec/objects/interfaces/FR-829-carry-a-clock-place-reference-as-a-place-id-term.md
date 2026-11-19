---
id: FR-829
title: "Carry a clock-place reference as a place-id term"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
  - target: ix://agent-ix/filament-specification/FR-772
    type: references
  - target: ix://agent-ix/filament-specification/FR-376
    type: references
---

# FR-829: Carry a clock-place reference as a place-id term

## Description

`ref(p)`, for a clock place expression `p`, is a data term whose value is the
place id of the place instance `p` denotes (FR-415 "Clock places", FSL FR-230
"Place identity"). The checked package SHALL carry it as the application
`filament.op.clock_reference.ref`, typed by the package's one
`scalar_type`/`clock_reference` node, wherever a data term stands: a function
body, a state clause, a postcondition write, a refinement row and the body or
condition of a population form (`count`, `sum`, `exists`, `all`, `only` of
FR-376 and the state-clause forms over a population).

A place reference is how a data value names a clock. A clock itself holds no
data value (FR-322 "Data part of a clock-bearing record, tuple or union"), so
every data computation over clocks, such as counting the calls whose clock is
the one a field names or picking the one call's clock with `only`, goes
through the place id. The term carries no clock value and no valuation, so it
keeps the timed model's rule that a clock is read only by a clock constraint.

### What a place reference denotes

- **The place instance, never the clock value.** The value of `ref(p)` at a
  state is the place id of the instance named by the owner object, the
  `clock_place` template node of `p`'s path and the slot positions. It reads
  no clock: two states that differ only in clock values give equal place ids,
  and the clock valuation of the current state is not an input of the term.
- **Per instance under a population binder.** In a population form
  `F(x in P where c: e)` whose condition or body holds `ref(x.f)`, the owner
  term is the binder `x` itself, and the term is evaluated once for each object `x`
  visits, giving that object's place id. `count(x in P where ref(x.t) = self.r)`
  counts the objects whose place `t` is the one `self.r` names, and
  `only(x in P where x.n = 1: ref(x.t))` is the place id of the one object
  whose `n` is `1`, undefined with the reason `only-not-one` otherwise
  (FR-376). The binder ranges over the objects the population holds at the
  state, so the owner of every visited place exists.
- **A path through a reference.** A place path that crosses an object
  reference names a place of the object the reference holds at the state:
  `ref(self.r.x)`, with `r: Reference<Call>`, has as its owner term
  the data read `self.r` and names the place `x` of the `Call` it holds; `ref(self.r.ts[0].x)`
  has the same owner term and the slot term `0`. The owner term is that data
  term, typed `Reference<Call>` ("Owner terms" below), and the remaining path is a path of
  that object's type. An owner term of type `Option<Reference<T>>` is not of
  the owner type; the author reads it as `value(self.r)`, under that read's
  presence obligation.
- **No presence obligation on the reference.** A reference names a place
  whether or not the place is present: `ref(self.v)` over `v: Option<Clock>`
  and `ref(self.p.Armed)` over a union place are defined at every state. The
  presence obligation belongs to `deref` (FR-415 "Clock places"), which reads
  the place the reference names. A slot place `ref(self.q[i])` keeps the
  selection obligation of FSL FR-230 "Selection" on its selector, because a
  slot position beyond the collection names no instance of the template.
- **Under a `Map` value there is no place id.** A place under a `Map` value is
  selected by key and has no place id (FR-415 "Element places"), so no
  `filament.op.clock_reference.ref` names such a place.

### Presence in a population form

`present(x.v)` in the condition or body of a population form is the
`presence_read` of FR-322 "Presence" with the binder `x` as its owner term. At
a state its value is the presence of that object's instance of the place:
true when the place is present, false when it is absent (the `Option<Clock>`
absent, the union case inactive, the slot outside the collection). It carries
no obligation, so `count(x in P where present(x.v))` is the number of objects
whose `v` is present, and `sum(x in P: if present(x.v) then 1 else 0)` equals
it. A path through a reference takes the data read as the owner term, as
above. `forall(e in x.q: present(e))` and `exists` in a population form take
FR-772's slot-domain application at the owner term `x` as operand 0, so the
population form and the slot quantifier nest without a new form.

### The type node

`Reference<Clock>`, and `Reference<A>` for an alias `A` of `Clock` or a value
type bound to `Clock` by `target`, SHALL be typed by the package's one
`scalar_type`/`clock_reference` node, whose body is `aggregate{[]}` and which
is owned and keyed as the native `scalar_type`/`uuid` node is. The two
spellings name one node, as their place ids name one triple (FSL FR-230:
`object_type` is the effective id of `Clock` after alias resolution). Its
family is `clock_reference`, a native scalar family of FR-322 "Operation
families, groups and constraints" and a member of the `any_value` group. An
`Option<Reference<Clock>>`, a collection of them and a record, tuple or union
member of that type are typed by the usual nodes over this one.

`Reference<Clock>` is a data type: a field, tuple position or union payload
of that type is a data member of its record, tuple or union, never a clock
member. It is in the data part of FR-322 "Data part of a clock-bearing
record, tuple or union": a record field of that type is a data field, read
by name in declaration order like the other data fields, and a tuple position
of that type is counted by the data index `d(p)`, so a `tuple_value` carries a
term for it.

The family has the identities `filament.op.clock_reference.eq` and
`filament.op.clock_reference.ne`, each with two `clock_reference` operands,
result `boolean`, no law, no mode, no member and no leaves, deciding equality
by the place id text (two ids are equal exactly when their texts are byte
equal), so equality is reflexive, symmetric and transitive and `ne` is its
negation. The family has no ordering and no arithmetic: an ordering or
arithmetic identity over it refuses `ill_typed`/`operator-ineligible`. It is
not the `reference` family of an object reference, so `filament.op.reference.eq`,
`filament.op.model.navigate` and every other identity whose operand group does
not list `clock_reference` refuse a `clock_reference` operand
`ill_typed`/`operator-ineligible` at that argument. No `filament.op.collection.sum.*` entry admits this family as its summand. A
sum entry's operands are `collection_kind` and `binder`, and the summand is
the bound of the binder, so a sum whose binder body is typed by the
`clock_reference` node refuses `ill_typed`/`operator-ineligible` at the binder
argument, `/semantic_graph/nodes/{n}/body/arguments/1` of the sum
application, as every binder whose bound fits no family of the entry does.

### The place-reference node

`ref(p)` is the application `filament.op.clock_reference.ref`, operator class
`place_ref`, `semantic_form` `place_reference`, a derived `expression` node.
Its arguments are, in order:

1. a reference to the `clock_place` template node of `p`'s path;
2. the owner term, a `Reference<O>` term of the place's owner type ("Owner terms");
3. one term for each `slot` step of the path, in path order, each of its
   selector's domain (an `Integer` position).

**Owner terms.** The owner position of `filament.op.clock_reference.ref`, of
the place form of `presence_read` and of FR-772's slot domain has one encoding.
It admits, directly and with no wrapper, a term typed `Reference<O>` (the
`composite_type`/`reference` node over the owner object type `O`, or over a
type that conforms to it, "Reference conformance"): a `self` or `other`
parameter, an operation parameter, a population binder (a `value`/`parameter`
of type `Reference<O>`), or a path term that reaches the owner, such as the
field read `self.peer`. A producer writes the `Reference<O>` term itself and never a
`filament.op.model.deref` at an owner position, so one source owner has one
encoding and one node identity, and the
published presence vectors, whose `self` and `other` are
`composite_type`/`reference` parameters, are unchanged. The catalog names
this position by the operand group `place_owner`, whose only family is
`reference`. An `object` term at the owner position, such as the `inner:0`
result of `filament.op.model.deref` over a reference, is not in the group and
refuses `ill_typed`/`operator-ineligible` at the owner operand
(`/body/arguments/1`), so a second encoding of one owner is never admitted. A
`Reference<P>` term with `P` neither `O` nor a type conforming to it refuses
`ill_typed`/`type-mismatch` at that argument, as an owner not of the owner
type does (FR-322-AC-156).

These are the arguments of the `presence_read` of the same place (FR-322
"Presence"), so the place reference and the presence read of one instance
take the same operands. The `result_type` is the `scalar_type`/`clock_reference`
node.

The node is identified structurally, as FR-772's slot-domain node is. Its
`node_id` is `{package, path, ordinal}` with the path
`["expression", "place_reference", A1, ..., An, "filament.op.clock_reference.ref", "place_ref", R]`,
one operand segment `Ak` for each argument in order, the first naming the
`clock_place` node, then the operation identity and the operator class, then
the operand segment `R` of the `result_type` node. The `clock_place` node
appears in the path only as that operand segment. Two place references of
one place at one owner term and the same slot terms are one node, and a
reference at `self` and one at a population binder are two.

### Operation catalog entry

The QVC `checked-operation-catalog-v1.json` SHALL carry these entries, in the
catalog's own shape:

```json
{"identity": "filament.op.clock_reference.ref", "operator": "place_ref", "operands": ["place_owner"], "rest": "any_value", "result": "clock_reference", "laws": [], "mode": null, "member": null, "constraints": [{"kind": "place_identity_terms", "operands": [0]}, {"kind": "place_has_id"}], "leaves": null}
{"identity": "filament.op.clock_reference.eq", "operator": "binary", "operands": ["clock_reference", "clock_reference"], "rest": null, "result": "boolean", "laws": [], "mode": null, "member": null, "constraints": [], "leaves": null}
{"identity": "filament.op.clock_reference.ne", "operator": "binary", "operands": ["clock_reference", "clock_reference"], "rest": null, "result": "boolean", "laws": [], "mode": null, "member": null, "constraints": [], "leaves": null}
```

- **Operands and rest.** As for the place form of `filament.op.option.present`,
  the leading `clock_place` reference is checked before the catalog and is
  not a catalog operand. The catalog operands are the owner term (group
  `place_owner`, "Owner terms") and, as `rest`, the slot terms. The group
  `place_owner` is added to FR-322 "Operation families, groups and
  constraints", and FR-772's slot-domain entry takes it for its owner operand
  in place of `object`, in the same release.
- **`place_identity_terms`**: the rest count is `s`, the number of `slot`
  steps of the `clock_place` node's path, and each rest term has its slot
  step's selector domain.
- **`place_has_id`**: no `slot` step of the `clock_place` node's path is a
  `Map` level.
- **Result form** `clock_reference`: the `scalar_type`/`clock_reference` node.

**Catalog changes.** The QVC `checked-operation-catalog-v1.json` changes as
follows, tracked by QSL-2584:

- the family `clock_reference` is added to the families and to
  `groups.any_value`;
- the group `groups.place_owner` is added, with the one family `reference`;
- the three entries above are added;
- operand 0 of `filament.op.option.slots` (FR-772) changes from `object` to
  `place_owner`.

The V2 schemas' `OperationIdentity`, operator-class and `semantic_form`
enumerations SHALL list the three identities, the class `place_ref` and the
form `place_reference` in the same release as the catalog entries, and the
`scalar_type` form enumeration SHALL list `clock_reference` in that release:
the schemas enumerate exactly the catalogued identities (FR-322-AC-17).

### Reader checks

A strict reader SHALL admit a `filament.op.clock_reference.ref` application
under these checks, in this order:

- An argument count other than `2 + s` refuses `ill_typed`/`operator-ineligible`
  at `/body/arguments`.
- A first argument that is not a `clock_place` node refuses
  `ill_typed`/`operator-ineligible` at `/body/arguments/0`; one whose path has
  a `Map` slot step refuses `ill_typed`/`type-mismatch` at `/body/arguments/0`
  (`place_has_id`).
- An owner term not of the owner type, or a slot term not of its step's
  selector domain, refuses `ill_typed`/`type-mismatch` at that argument.
- A `result_type` that is not the `scalar_type`/`clock_reference` node
  refuses `ill_typed`/`type-mismatch` at `/body/result_type`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-829-AC-1 | Over FR-415-AC-15's model (`Call` with clocks `x`, `y` and a field `r: Reference<Clock>`) with the field `q: Sequence<Option<Clock>>[0, 2]` added, the `q` of AC-3 and AC-6, `ref(self.x)` is one `filament.op.clock_reference.ref` application with the arguments the `clock_place` node of `x` and `self`, `result_type` the `scalar_type`/`clock_reference` node, and path `["expression","place_reference",A1,A2,"filament.op.clock_reference.ref","place_ref",R]`; a second `ref(self.x)` in another clause is the same node; `ref(other.x)` is a different node; `ref(self.q[1])` carries the slot term `1`; the field `r` and a `Reference<A>` field (`type A = Clock`) are typed by the same `clock_reference` node; and the strict reader admits the package. | Test (TC-2084) |
| FR-829-AC-2 | In a refinement row over a population `calls` of two `Call` objects `c1` (`n = 1`) and `c2` (`n = 2`), `only(c in M::calls where c.n = 2: ref(c.x))` evaluates to the place id of `c2.x`, `count(c in M::calls where ref(c.x) = self.r)` evaluates to `1` when `self.r` holds `c1`'s place id of `x` and to `0` when it holds `c1`'s place id of `y`, and both values are unchanged at a state that differs only in clock values; the `ref` node's owner term is the binder parameter `c` itself, typed `Reference<Call>`, with no `deref`. | Test (TC-2084) |
| FR-829-AC-3 | Over `v: Option<Clock>` and `q: Sequence<Option<Clock>>[0, 2]` on `c1` (`v` present, `q` holding one present and one absent element) and `c2` (`v` absent, `q` empty): `count(c in M::calls where present(c.v))` and `sum(c in M::calls: if present(c.v) then 1 else 0)` are `1`; `all(c in M::calls: exists(e in c.q: present(e)))` is false and `exists(c in M::calls: forall(e in c.q: present(e)))` is true (`c2`'s empty `q`); each `present` is a `presence_read` whose owner term is the binder `c` itself, and each slot quantifier's operand 0 is FR-772's slot-domain application at that owner term. | Test (TC-2084) |
| FR-829-AC-4 | With a field `peer: Reference<Call>`, `ref(self.peer.x)` is the place-reference application whose owner term is the `peer` field read of `self`, typed `Reference<Call>`, it equals `ref(other.x)` exactly at states where `self.peer` holds `other`, and `present(self.peer.v)` is the `presence_read` with the same owner term; `ref(self.v)` over an absent `v` is defined and equals the place id of the instance `self.v`. | Test (TC-2084) |
| FR-829-AC-5 | A record value type `Tag { x: Clock; r: Reference<Clock>; n: Integer }` has the data part `r`, `n`, in that order, and no member for `x`; a `tuple_value` of a tuple `(Clock, Reference<Clock>)` carries one term, the place id; `filament.op.clock_reference.eq` over `ref(self.x)` and `ref(self.x)` is true and `ne` false; and the three identities, the operator class `place_ref`, the form `place_reference` and the `scalar_type` form `clock_reference` are enumerated by the catalog and both V2 schemas together. | Test (TC-2084) |
| FR-829-AC-6 | Negative: the place-reference application over `q`'s element place with no slot term refuses `ill_typed`/`operator-ineligible` at `/body/arguments`; with a `scalar_type` first argument, `ill_typed`/`operator-ineligible` at `/body/arguments/0`; over the `clock_place` of a place under a `Map` value, `ill_typed`/`type-mismatch` at `/body/arguments/0`; with an `Integer` owner term, `ill_typed`/`type-mismatch` at `/body/arguments/1`; with a `Reference<Order>` owner term (an object type `Order` that does not conform to `Call`), `ill_typed`/`type-mismatch` at `/body/arguments/1`; with the binder `c` itself as owner term (as AC-2 writes it), admitted; with the owner term the `filament.op.model.deref` application over `c` (an `object` term), `ill_typed`/`operator-ineligible` at `/body/arguments/1`, and the same for a `presence_read` and an FR-772 slot domain with that owner; with `result_type` the `composite_type`/`reference` node of `Call`, `ill_typed`/`type-mismatch` at `/body/result_type`; a `filament.op.reference.eq` and a `filament.op.integer.lt` over two place references each refuse `ill_typed`/`operator-ineligible` at `/body/arguments/0`; and a `filament.op.collection.sum.integer` whose binder body is a place reference refuses `ill_typed`/`operator-ineligible` at the sum's `/semantic_graph/nodes/{n}/body/arguments/1`. | Test (TC-2084) |

## Dependencies

- FR-322 "Presence", "Structural node identity", "Data part of a clock-bearing
  record, tuple or union" and "Operation families, groups and constraints".
- FR-415 "Clock places" and "Element places": the place id and the
  `deref` presence obligation.
- FR-772: the slot-domain application a presence quantifier ranges over.
- FR-376: the population forms and the `only-not-one` reason.
