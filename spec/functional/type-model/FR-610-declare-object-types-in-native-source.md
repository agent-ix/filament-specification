---
id: FR-610
title: "Declare object types, populations and operations in native source"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-150
    type: references
  - target: ix://agent-ix/filament-specification/FR-151
    type: references
  - target: ix://agent-ix/filament-specification/FR-153
    type: references
  - target: ix://agent-ix/filament-specification/FR-154
    type: references
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-340
    type: references
  - target: ix://agent-ix/filament-specification/FR-342
    type: references
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
  - target: ix://agent-ix/filament-specification/FR-465
    type: references
---

# FR-610: Declare object types, populations and operations in native source

## Description

When a source unit declares an object type, the compiler SHALL admit that
object type with its fields, identity fields, supertypes, closed populations
and operations.

The compiler SHALL give a source-declared object type the meaning an object
type of a selected domain package has (FR-150, FR-151, FR-153), owned by the
source unit that declares it.

When the compiler emits a checked package, the compiler SHALL write each
source-declared object type with the node forms a domain-package object type
uses, so that a consumer reads it from the package and its identity rows
alone.

A unit can then state a model and the rules over it in one source, with no
domain package document. The types it declares stay distinct from every
domain-package declaration, whatever their names.

## Inputs

- A complete-V1 source unit under `filament.model.complete/v1`, with zero or
  more `object-decl` declarations (shared grammar "Complete-V1 facet
  grammar").
- The unit's `SourceOwner` (authority, identity) and the checked package name.
- The unit's selected domain packages, if any.

## Outputs

- The source-declared object types, fields, populations and operations, checked
  under the same rules as a domain package's.
- In the checked package: one `model`/`object_type` node per source-declared
  object type, its `relation`/`population` nodes, operation anchors, frames,
  clause functions, `operation_effects` rows and dispatch-table rows (FR-322
  "Source-declared object types"), and one source-object identity row per
  type.
- Or a located refusal and no checked package.

## Behavior

### Grammar

The shared grammar's `object-decl`, `object-member`, `population-member`,
`operation-member`, `frame-clause`, `field-target` and `delete-target`
productions are the surface of this requirement, and its complete-V1
`type-name`, `operation-name` and `population-name` admit the one-segment
source name beside the two-segment model name. A one-segment name SHALL
resolve only to a source declaration of the unit and a two-segment name only
to a declaration of a selected domain package; neither falls back to the
other. In `role w on M`, a one-segment `M` that is a model alias of the unit
names that alias's whole model.

### Meaning

A source-declared object type SHALL check and evaluate exactly as an object
type of a selected domain package with the same fields, identity fields,
supertypes, populations, operations, frames and clauses: effective view and
normalization (FR-150), conformance, redefinition, abstract types and closed
dispatch (FR-151), and population queries (FR-153). Its operations take the
unit's `pre`, `post` and `body` declarations that name them, in source order,
as their contract clauses and body. Every outcome, value and charge that an
evaluation gives over a source-declared type SHALL equal what the same
evaluation gives over that domain-package twin.

A source object type declares no relationship, component, endpoint, interface
feature, state machine or process; a source field declares no `subsets` or
field redefinition; and a source object type neither specializes nor is
specialized by a domain-package type. Those are domain-package constructs,
and a unit that needs them selects a domain package that declares them.

### Member types

A field, operation parameter and operation result of a source object type
SHALL admit every type a native `record` field admits, with the same checks,
the same refusals and the same package form:

- the builtin scalars `Boolean` and `Integer`, `Int[lo, hi]`,
  `Rational[..]`, `Decimal[..]`, `Float32` and `Float64` (selecting the IEEE
  profile as a float-bearing record field does), `Text[..]`;
- `Quantity<u>` and `Quantity<u, C>` over a declared unit (FR-142);
- `Clock`, under the timed rules a clock-typed record field follows;
- `Option<T>`, `Reference<T>` over a source-declared or domain-package object
  type, and `Sequence`, `Set`, `Bag` and `OrderedSet` of any admitted element
  type, a nested collection and an `Option` of a collection included;
- a native `enum`, `record`, `tuple`, `union` or `type` alias of the unit,
  and a domain-package value, record, tuple or variant type named through a
  model alias.

The package form of a member is a binding of the object type node's body
whose type node is the node the same type has as a native record field: a
native enum's `scalar_type`/`enum` node, a native record's or tuple's
declared node, a union's node (FR-440), a bounded numeric or text type's
`bounded_domain` node, a quantity's unit or compound quantity node, and a
collection's or option's `composite_type` node. No member type has a form of
its own on the object type.

A type a native record field refuses SHALL be refused for an object member
with the same code and cause at the type, and no other type is refused for an
object member. Admitting the record-field rule exactly gives one member-type
rule in the language, so a type moved from a `record` into an `object` keeps
its meaning and its refusals.

### Identity

A source-declared object type is identified by its owner, the declaring
unit's `SourceOwner`, and its qualified name `[<type name>]`. Its checked node
id is a declared node's structural id `{package, [<type name>], ordinal}`, so
a change to a field, operation or population leaves the id unchanged and a
rename changes it. A domain-package object type is identified by its
`ModelOwner` (domain package identity and IR node identity) and carries no
`declaration`.

The two identities SHALL never compare equal and SHALL never stand for one
another. A source type `Order` and a domain-package type `M::Order` are
different types with different node ids, owners and identity rows. In source,
a source object type or population whose name equals another declaration of
the unit, a model alias, a profile alias or an import alias SHALL be refused
`ambiguous_declaration`/`ambiguous-name` at every declaration of that name.
In the package, FR-322 "Source-declared object types" refuses a node that
mixes the two owner forms.

### Package form

The checked package carries a source-declared object type, its populations,
anchors, frames, clause functions, operation effects and dispatch rows in the
forms FR-322 "Source-declared object types" states. Protocol clauses and roles
over a source-declared type join through its `SourceOwner` (FR-464
`owner_model`, FR-465 role `model`).

### Identity rows

A consumer reading the package from its bytes has no domain document for a
source-declared type. The producer SHALL publish one source-object identity
row per emitted source-declared `model`/`object_type` node:
`structural_node`, `source_authority`, `source_identity` (the type's qualified
source name), `abstract`, `supertypes` (the direct supertype node ids in
source order), `identity_fields` (own, or inherited when it declares none),
`members` (one entry per field of the effective view, in the node body's
binding order: name, presence and member type) and `populations` (one `{name,
symmetric}` entry per population the type itself declares, in source order),
keyed by (`source_authority`, `source_identity`). A consumer SHALL read field
types, identity fields, supertypes, abstractness and populations of a
source-declared type from these rows wherever it reads them from an admitted
domain document for a model-owned type, and SHALL refuse a row whose
authority names no `SourceOwner` of the package, a row whose node is not a
source-declared object type of the package or whose `source_identity`
disagrees with the node's `declaration`, and a source-declared node no row
names.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-610-AC-1 | The shared grammar's complete-V1 productions parse `object Order identity (total) { total: Int[0, 9]; note: Text[0, 8; nfc]?; population orders; operation bump(): Boolean modifies self.total; }`, `object Sub extends Order { operation size(): Integer redefines Order::size; }` and `abstract object Shape { population shapes; }`, each part in source order. `operation step()` with no `;`, `modifies self.` with no field name, and a parameter named `modifies` each refuse `invalid_syntax`/`unexpected-token` at the offending token. | Test (TC-1440) |
| FR-610-AC-2 | In one unit that selects a domain package declaring `Order` as `M`, `on Order` resolves to the source type and `on M::Order` to the model type, and neither resolves to the other; `role w on M` names the whole model; `object M` beside `model M` refuses `ambiguous_declaration`/`ambiguous-name` at both. | Test (TC-1440) |
| FR-610-AC-3 | A source-declared `Order` is emitted as one self-typed `model`/`object_type` node with `declaration` `["Order"]`, the unit's `SourceOwner`, a body binding `total` then `note` (with its `optional` group) and one `declaration` occurrence; `Sub extends Order` binds the inherited `total` and `note`; `Shape` has `aggregate{[]}`. Adding a field to `Order` leaves its node id unchanged and renaming it changes the id. A `field` member `total` on `Order`'s node resolves to its binding with no domain document, and `ghost` refuses `missing_declaration`/`missing-name`. | Test (TC-1441) |
| FR-610-AC-4 | The source `Order` node and a domain package's `M::Order` node have different node ids and owners. Each refuses: the source `Order` node with its owner replaced by a `ModelOwner` (`malformed_wire` at its `owner`); a `model`/`object_type` node with a `SourceOwner` and no `declaration` (`malformed_wire` at its `owner`); the source node with a `SourceOwner` joining no `lock.sources` entry (`missing_declaration`/`missing-selection` at its `node_id`). A `relation`/`population` node with a `declaration` and a `SourceOwner` passes the schema and a `relation`/`relationship` node with a `declaration` fails it. | Test (TC-1441) |
| FR-610-AC-5 | For each of these member types, an `object` field, an operation parameter or result of that type checks exactly when a native `record` field of that type checks, and the member's binding names the type node the record field's binding names: a native `enum`, a native `record`, a `union` with a payload member, a `tuple`, a `type` alias, `Decimal[0, 100; 2, 2; exact]`, `Rational[0, 9; 1, 4]`, `Float64`, `Quantity<m>` over a declared unit, `Option<Sequence<Integer>[0, 3]>` and `Sequence<Set<Integer>[0, 2]>[0, 2]`. | Test (TC-1442) |
| FR-610-AC-6 | A field typed `Population<Order>[3]` is refused at the type with the same code and cause as a native record field typed `Population<Order>[3]`, and a field typed by a name that is a source function is refused `missing_declaration`/`missing-name` at the name, as for a record field. | Test (TC-1442) |
| FR-610-AC-7 | A `protocol_clause` whose `owner_model` is a source-declared object type node and a static role whose `model` targets it are admitted, joined through the node's `SourceOwner`; the same targets with a `SourceOwner` that joins no `lock.sources` entry refuse `missing_declaration`/`missing-selection` at the target's `node_id`; a `owner_model` on a source-declared record node refuses `invalid_model_binding`/`malformed-declaration`. | Test (TC-1443) |
| FR-610-AC-8 | A source-declared operation's effects row carries `document` the unit's source identity, `declaring` the source node, `provenance.origin` `source_declaration_operation_frame` and `provenance.member` `Order::bump`; its authored precondition clause function `Qs` carries `declaration` `["Order", "size", "precondition", "Qs"]` and the `SourceOwner`. The identity rows of `Order`, `Sub` and `Shape` read back with their `abstract`, `supertypes`, `identity_fields`, `members` and `populations`; a row of a foreign authority, a row naming a frame node and a missing `Order` row each refuse. | Test (TC-1443) |

## Dependencies

- [FR-150](FR-150-preserve-model-normalization-provenance.md),
  [FR-151](FR-151-resolve-conformance-redefinition-dispatch.md) and
  [FR-153](FR-153-query-closed-model-environments.md): the meaning a
  source-declared type shares with a domain-package type.
- [FR-154](FR-154-admit-domain-package-model.md): domain-package declarations
  the source types stay distinct from.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  "Source-declared object types": the package form and the owner join.
- [FR-464](../../objects/interfaces/FR-464-checked-package-protocol-node-bodies.md)
  and
  [FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md):
  protocol owners and roles over a source-declared type.
- The shared grammar (`proposals/filament-v1/shared-grammar.md`): the
  `object-decl` productions.

## Status

Accepted complete-V1 requirement.
