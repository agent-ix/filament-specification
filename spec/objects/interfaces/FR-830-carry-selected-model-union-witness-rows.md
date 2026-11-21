---
id: FR-830
title: "Carry selected model-union witness rows"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-440
    type: references
  - target: ix://agent-ix/filament-specification/FR-441
    type: references
  - target: ix://agent-ix/filament-specification/FR-610
    type: references
  - target: ix://agent-ix/filament-specification/FR-154
    type: references
---

# FR-830: Carry selected model-union witness rows

## Description

When a producer hands off a checked package whose selected domain documents
declare a `model`/`variant_type` node of family `union` (FR-322 "Tuple value
types and payload-bearing variant types"), it SHALL publish one `model_unions`
witness row per such node in the identity sidecar beside the package, and a
reader SHALL join each row it needs to the package and its selected document as
below, retaining the producer's opaque identities, so that a consumer can name
the declaration, each case, each sealed member and each payload of a model union
without deriving any of them from a name, a graph node or a rendered type
spelling.

## Rationale

A `model`/`variant_type` node carries an empty `aggregate` body and no members
(FR-441): its cases and payloads are read from the selected document, and the
producer's original declaration key, case identities and payload value types
are opaque values no reader can recompute from the package bytes. The rows of
FR-610 "Identity rows" witness source declarations; a model union has no
`composite_type`/`union` node, so the source-union rows cannot stand for it and
this carrier is separate from them. The cases are nested members of the
selected document, not `enum_value` nodes, so the carrier invents no case node
and never falls back to an enum representation.

## Behavior

### Carrier and scope

- `model_unions` is a member of the producer's identity sidecar, beside the
  source rows, and is required, empty or not. It is not a member of the
  checked package, whose `checked_term_data` carrier and schema are unchanged.
  The strict read of the package never receives it; the join runs after the
  strict read, as the joins over FR-610's identity rows do. An empty list
  witnesses no union. The sidecar's format version is the producer's; this
  requirement states no version and no pin.
- **Encodings are the sidecar's existing ones, not new ones.** A node
  reference (`structural_node`, a payload `node`) is the serde form of the
  package's `CheckedNodeId`, the object `{package, path, ordinal}`; the node
  reference of a package is therefore its `node_id` object (the `structural_node`
  of FR-610's rows and the symbolic ids of the fixtures are the same value).
  An **original node key** is the shared node-key codec's structural JSON of a
  producer's original `NodeKey`, nested at most 32 keys deep and checked
  iteratively; it is a JSON value, not text, and need not name any emitted node.
  An **original value type** is the complete Exact value-type grammar with
  original node keys (`{"kind":"boolean"}`, `{"kind":"integer"}`,
  `{"kind":"composite","node":<original node key>}`, and the
  option, collection, map, reference, enum, quantity, carried-quantity and
  bounded-quantity forms of that grammar). A **declaration key** is
  `{package, node}`.
- A **model-union row** is exactly the closed object
  `{structural_node, selected_document, type_handle, settled_key, cases}`.
  `structural_node` is the node reference of the union's `model`/`variant_type`
  node; `selected_document` is the `identity` of its `ModelOwner`; `type_handle`
  is the producer's original composite type key, the key an original value type
  `composite` names for this union; `settled_key` is the settled declaration key
  the sealed members of this union carry. The two are different roles that
  coincide for a model declaration today, and no relation between them is
  required. Both are original node keys, returned byte for byte, never decoded
  into or derived from a name, a node id, a payload spelling or each other.
- A **case** is exactly `{rank, position, name, original_case, member, payload}`.
  `rank` is the case's index in the canonical order and `position` its authored
  index among the selected document's `variants`, both JSON integers, so a
  consumer reads the canonical rank and the authored order apart. `name` is the
  case name. `original_case` is the case's declaration key, the `VariantRecord`
  `key` of the selected document's variant: `{package: <selected_document>,
  node: <that variant's identity>}`; it is carried once and no second case
  identity exists. `member` is the sealed union member `{declaration, identifier}`
  (FR-441's pair): `declaration` an original node key and `identifier` the case
  name.
- `payload` is one of three closed forms:
  `{kind: "none"}`, a nullary case with no `payloadType`;
  `{kind: "data", node, original_type}`, a case whose alias-resolved
  `payloadType` is any admitted type but Clock, `node` the node reference of the
  package node FR-322 step 4 gives that type (a native scalar, a unit, or a
  declared tuple, record, variant or object node) and `original_type` the
  producer's original value type of the payload; and
  `{kind: "clock", payload_type}`, a case whose alias-resolved `payloadType` is
  `ix://filament/native/Clock`, `payload_type` that identity, which has payload
  arity 0 in the data sense, carries no node and no original value type, and is
  not a nullary case. A case has at most one data payload.
- The **scope** of a join is `full` (the whole selected environment) or
  `reached` (the closure of one requested node). Under `full`, exactly one row
  per union-family `model`/`variant_type` node of the package is required,
  whether or not any term uses it. Under `reached`, the closure starts at the
  requested node and follows, by an explicit worklist with a visited set so a
  cycle terminates, every edge from a node to the node of a type its selected
  document gives it: a variant case's resolved `payloadType`, a tuple's position
  `typeRef`s, and a record's or object type's field `typeRef`s (FR-322 "Model-owned
  members"). Each node is expanded once and each edge is examined once. A row
  is needed exactly for each union-family node in the closure; a row outside the
  closure is not inspected, charged or retained, and an unrelated or unsupported
  declaration outside the closure does not refuse. There is no `full` fallback
  for a `reached` request and no `reached` fallback for a `full` one.

### Checks

The reader refuses with the code, cause and locus FR-322 uses for a carrier
row; the locus is an RFC 6901 pointer into the sidecar, so a row's members are
at `/model_unions/{k}/...`. It checks in this order and stops at the first
refusal; nothing is returned for a refused or incomplete join.

1. **Schema.** A missing or extra member, an unknown `payload.kind`, a payload
   member foreign to its kind (a `node` on `none` or `clock`), a `settled_key`,
   `type_handle`, `member.declaration` that is not an original node key (or is
   deeper than 32 keys), an `original_case` that is not a declaration key, an
   `original_type` that is not a member of the original value-type grammar, and
   a non-integer `rank` or `position` refuse `malformed_wire` at the object or
   member that breaks the schema. There is no enum, `VariantId` or fabricated
   case-node member.
2. **Row identity**, in row order. `structural_node` is compared against every
   earlier row's (`invalid_package`/`conflicting-definition` at `structural_node`)
   and then against the predecessor's, rows ascending in node-id order
   (`invalid_package`/`invalid-value` at the row). It names a node of the package
   (`missing_declaration`/`missing-name`) that is a `model`/`variant_type` of
   family `union` (`ill_typed`/`operator-ineligible`; a `composite_type`/`union`
   node, an enum-family `variant_type`, a tuple or any other node refuses, so a
   source union and an enum are never read as a model union). `selected_document`
   is a lock selection (`missing_declaration`/`missing-selection`) and equals the
   node's owner `identity` (`invalid_package`/`invalid-value`). `type_handle` and
   `settled_key` each differ from every earlier row's (`invalid_package`/
   `conflicting-definition` at the member).
3. **Case census.** The case names are exactly the selected document's case
   names, each once. A name repeated is `invalid_package`/`conflicting-definition`
   at `cases/{j}/name`; the first name the document does not declare is
   `invalid_package`/`invalid-value` at `cases/{j}/name`; a declared name with
   no case is `invalid_package`/`invalid-value` at `cases`. The array is in
   ascending UTF-8 byte order of `name` (`invalid_package`/`invalid-value` at the
   later case's `name`), so a case order other than the canonical one refuses.
4. **Per case**, in array order: `rank` equals the index and `position` equals
   the name's index among the document's `variants` (`invalid-value` at the
   member); `original_case` equals `{package: selected_document, node: <the
   variant's identity>}`; `member.identifier` equals `name` byte for byte and
   `member.declaration` equals the row's `settled_key` (`invalid-value` at the
   member). The same `name` under two unions is two members, because the member
   differs by its declaration.
5. **Payload.** `kind` equals the form of the case in the document: `none`
   without a `payloadType`, `clock` when the alias-resolved type is Clock,
   `data` otherwise; any substitution, including a Clock case carried as nullary
   or data and a nullary case carried as Clock, is `invalid_package`/
   `invalid-value` at `payload/kind`, and a `clock` payload whose `payload_type`
   is not the Clock identity is `invalid-value` at `payload/payload_type`. For
   `data`, `node` equals the package node of the case's resolved type (a node
   absent from the package is `missing_declaration`/`missing-name`, another node
   of the package `invalid_package`/`invalid-value`, at the member). The
   `original_type` is then validated against that node: it decodes as a complete
   original value type and equals, constructor for constructor and leaf for
   leaf, the Exact value type the admitted type projection (filament-contract-ir
   FR-352) gives the payload type, where the only values it does not compare
   are the opaque original node keys inside it. Each such key is checked against
   the **supplied witnesses**: where a model-union row of this carrier or a
   selected-declaration row of the identity file names the payload node, the
   key equals that witness's original key (a union node's `type_handle`); a key
   equal to a witness key of a node other than the payload node is refused; a key
   no supplied witness names is preserved as the producer's claim. A mismatch is
   `invalid_package`/`invalid-value` at `payload/original_type`.
6. **Coverage**, last: under `full`, a union-family node with no row, and under
   `reached`, a reached union-family node with no row, refuses
   `invalid_package`/`invalid-value` at `/model_unions`. A `source_unions` row
   never supplies the missing row.

### Work and storage

The join draws on the one caller-owned `NativeJoinBudget` of filament-contract-ir
FR-358, the same budget the strict read and the native source joins use; there is
no separate or default allowance, and this requirement adds no artificial small
budget. Its two counters stay separate and keep FR-358's units.

- **Work** is one unit per bounded inspection step over a closure edge
  (`reached` scope only, before any row), a row, a case, a payload, and each JSON
  value of an original node key, declaration key or original value type compared,
  plus one unit per UTF-8 byte of caller-controlled text scanned (a case name, a
  selected-document identity, a declaration-key text), each charged before the
  step or advance. A row of a closure that is not reached is not visited.
- **Storage** is one unit per newly owned container, retained row, case, key,
  handle, element or entry, plus one unit per byte of newly copied owned text. A
  fixed-size identity handle is one unit; a node reference shared with the
  admitted package by handle is one unit and no recursive copy; a borrowed row
  charges no storage; a deep copy of an original key or value type (a retained
  clone) charges its elements and bytes again, and every charge is admitted
  before the reserve, clone or insertion it pays for. Temporary closure sets and
  worklists use the same rule.
- A charge the budget cannot cover returns `incomplete` with FR-358's first
  unpaid charge: its kind (`work` or `storage`), the full attempted amount, and a
  non-owning locus naming the sidecar row index and the form. Admitted charges
  are cumulative across the strict read, this join and any other join on the
  budget, including those of a refused call; a denied charge consumes nothing,
  nothing is refunded, and the original limits are reported unchanged. The
  retained rows are staged and published only when the whole join succeeds; no
  partial carrier or projection is returned.

The per-item amounts the two units give are fixed by FR-358's policy as applied
to the items above, not by this requirement; the published vectors therefore
assert relations to a baseline measured on the same input (the exact baseline
delta admits; one unit less of either counter is `incomplete` with that
counter's first unpaid charge), never a static total.

### Authority boundary

The reader validates consistency and preserves the producer's opaque identities; it does NOT claim provenance authentication from parsing. A row
that passes every join is admitted whether or not the producer that wrote it
held the declaration, and admission is no claim that the opaque keys are
authentic. The carrier adds no digest, signature, pin or catalog.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-830-AC-1 | Over `fixtures/positive-model-tuple-union.json`, a `full`-scope sidecar of two rows, List then Shape, with Shape's cases Both (position 2, declared Pair), Empty (position 1, nullary) and Flag (position 0, native Boolean) in canonical order though authored Flag, Empty, Both, each case's `original_case` the document variant's declaration key, is admitted and its rows returned byte for byte; a Shape that also holds `Chain(List)` joins that payload to List's own `type_handle`. | Test |
| FR-830-AC-2 | Under `reached` scope from Shape, a sidecar holding only Shape's row is admitted; a List row that would refuse is neither inspected nor charged, and under `full` the same rows refuse. | Test |
| FR-830-AC-3 | The `reached` closure over List (List, Cell, List), over the tuple Nest and over the object type Holder terminates, expands each node once, follows case-payload, tuple-position and object-field edges, and requires a row exactly for the union nodes it reaches. | Test |
| FR-830-AC-4 | A case whose payload is Clock is carried as `{kind: "clock", payload_type}` with no node and no original value type, and a nullary case, a Clock case and a data case are never substituted for one another. | Test |
| FR-830-AC-5 | Two unions with a case of one spelling are two rows with two members distinguished by their declarations and two original cases. | Test |
| FR-830-AC-6 | A sidecar row with a missing, extra or foreign member, a key, declaration key or original value type that is not of its codec, a key nested beyond 32, a non-integer `position` or an unknown payload kind refuses `malformed_wire`. | Test |
| FR-830-AC-7 | A row whose node, selected document, `type_handle`, `settled_key` or position among the rows is wrong, absent, duplicated or of another node form refuses with the code, cause and locus of "Checks" step 2. | Test |
| FR-830-AC-8 | A missing, extra, repeated or non-canonically ordered case, and a wrong `rank`, `position`, `original_case` or sealed member, refuses with the code, cause and locus of steps 3 and 4. | Test |
| FR-830-AC-9 | A payload whose kind, Clock identity, node or original value type (primitive, shape, wrapped or composite key naming another witnessed node) differs from the case's resolved payload refuses with the code, cause and locus of step 5. | Test |
| FR-830-AC-10 | A `full` environment without the row of an unused union, an empty `model_unions`, and a `reached` request without a reached union's row refuse `invalid_package`/`invalid-value` at `/model_unions`, with no enum or source-union fallback and no partial result. | Test |
| FR-830-AC-11 | On the shared native budget, the baseline delta of a join admits and returns the same rows; one unit less of work returns `incomplete` with a first unpaid work charge, one unit less of storage with a first unpaid storage charge, zero work or storage at the first row, with the full attempted amount and the row locus and no output; consumption accumulates across joins and refusals on one budget. | Test |
| FR-830-AC-12 | A row set that passes every join with producer keys the reader cannot authenticate is admitted with its keys returned unchanged, and the contract states the authority boundary in the words of "Authority boundary". | Test |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Tuple value types and
  payload-bearing variant types" owns the model union, its cases, payload
  arity and `node_id`; the reader's refusal vocabulary is its.
- [FR-440](FR-440-checked-package-union-and-case-nodes.md) and
  [FR-441](FR-441-union-member-key.md) own the member order and the member key.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md)
  "Identity rows" is the source-side precedent of the sidecar.
- filament-contract-ir FR-358 owns the native work and storage budget and
  FR-352 the Exact value-type projection this requirement joins to; the FSL
  identity file owns the original-key and value-type codecs. None is restated.
- Vectors: `proposals/checked-package-v2/model-union-witness-vectors.json`
  over `domain-package-acme-shapes.json` and
  `fixtures/positive-model-tuple-union.json`.

## Status

Draft contract with published vectors; no reader implements it and the vectors
are not yet executed by any harness. A unit-typed payload is specified (its
`node` is the unit node and its `original_type` a `quantity`, `carried_quantity`
or `bounded_quantity`) but has no vector: no selected document or package
fixture declares a unit that a variant payload can name, and publishing one
needs a new document and fixture. The `package_patch` of the Holder vector is a
published input that no reader has yet read.
