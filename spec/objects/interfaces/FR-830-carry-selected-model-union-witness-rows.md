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
  requirement states no version and no pin. The existing `source_unions` member
  and its declaration order are unchanged; model rows are a distinct member, a
  distinct typed row and a distinct join, and a source-union row never stands
  for a model union or the reverse.
- **Encodings are the sidecar's existing ones, not new ones.** A node
  reference (`structural_node`, a payload `node`) is the serde form of the
  package's `CheckedNodeId`, the object `{package, path, ordinal}`, which is the
  `node_id` object of the node in the package; it is compared structurally,
  package then path segments then ordinal, and never converted to text.
  An **original node key** is the shared node-key codec's structural JSON of a
  producer's original `NodeKey`, nested at most 32 keys deep and checked
  iteratively; it is a JSON value, not text, and need not name any emitted node.
  An **original value type** is the complete Exact value-type grammar with
  original node keys (`{"kind":"boolean"}`, `{"kind":"integer"}`,
  `{"kind":"composite","node":<original node key>}`,
  `{"kind":"quantity","unit":{"kind":"declared","node":<original node key>}}`,
  and the option, collection, map, reference, enum, carried-quantity and
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
- Rows are sorted ascending by `structural_node` (package, then path segments,
  then ordinal) in the file; that explicit sort is the format's, so it does not
  follow the order of `source_unions`. A **case** is exactly
  `{rank, position, name, original_case, member, payload}`. The cases are in
  ascending UTF-8 byte order of `name`; `rank` is the case's index in that
  canonical order and `position` its authored index among the selected
  document's `variants`, both JSON integers, so a consumer reads the canonical
  rank and the authored order apart. `original_case` is the case's declaration
  key, the `VariantRecord` `key` of the selected document's variant:
  `{package: <selected_document>, node: <that variant's identity>}`; it is
  carried once, is a new retention of an existing producer identity, and is never
  a case node or an enum `VariantId`. `member` is the sealed union member
  `{declaration, identifier}` (FR-441's pair): `declaration` an original node key
  equal to `settled_key`, and `identifier` the case name.
- `payload` is one of three closed forms:
  `{kind: "none"}`, a nullary case with no `payloadType`;
  `{kind: "data", node, original_type}`, a case whose alias-resolved
  `payloadType` is any admitted type but Clock, `node` the node reference of the
  package node FR-322 step 4 gives that type and `original_type` the producer's
  original value type of the payload; and `{kind: "clock", payload_type}`, a case
  whose alias-resolved `payloadType` is `ix://filament/native/Clock`,
  `payload_type` that identity, which has payload arity 0 in the data sense,
  carries no node and no original value type, and is not a nullary case. A model
  case has data arity 0 or 1; there is no list of payloads.
- The **scope** of a join is `full` (the whole selected environment) or
  `reached` (the closure of one requested node). Under `full`, exactly one row
  per union-family `model`/`variant_type` node of the package is required,
  whether or not any term uses it. Under `reached`, the closure starts at the
  requested node and follows, by an explicit worklist with a visited set so a
  cycle terminates, every edge from a node to the node of a type its selected
  document gives it: a variant case's resolved `payloadType`, a tuple's position
  `typeRef`s, and a record's or object type's field `typeRef`s (FR-322 "Model-owned
  members"). Each node is expanded once and each edge is examined once, so a
  record that reaches a union that reaches the record terminates. A row is needed
  exactly for each union-family node in the closure; a row outside the closure is
  not inspected, charged or retained, and a declaration outside the closure (a
  record with no identity witness, for example) neither refuses nor is required.
  There is no `full` fallback for a `reached` request and no `reached` fallback
  for a `full` one.

### Side-input witnesses

A data payload's original value type is validated against the identity
witnesses the join is given for the payload's node; the reader derives none of
them and adds no carrier. By the form of the payload node:

| Payload node | Original value type | Witness the join reads |
| --- | --- | --- |
| native scalar (`scalar_type` of Boolean, Integer, String, UUID, Timestamp, Bytes, JsonObject, Float32, Float64) | the keyless Exact type of that scalar form (`{"kind":"boolean"}`, ...) | none: the scalar node itself |
| declared unit (`scalar_type`/`unit`, model-owned) | `quantity` of `{"kind":"declared","node":K}` | the `declarations` row of form `declared_unit` for the node: `K` equals its `UnitId::Declared` key |
| selected model tuple (`model`/`tuple_value_type`) | `composite` of `K` | the `SelectedTupleDeclarationRow` for the node: `K` equals its `node_key` |
| selected record (`model`/`record_value_type`) | `composite` of `K` | the `declarations` row of form `record_value_type` for the node: `K` equals its `Record` key |
| selected object type (`model`/`object_type`) | `reference` of the row's `EffectiveId` | the `declarations` row of form `object_type`: the effective id equals its `Object` id |
| selected model union (`model`/`variant_type`, family `union`) | `composite` of `K` | the model-union row of this carrier for the node: `K` equals its `type_handle` |

Keys are compared as decoded `NodeKey`s, not as text. A needed witness the join
is not given is a named refusal (`unavailable_identity_witness` at the payload
node), never a successful unwitnessed key; a key that disagrees with the witness
of its own node, or a constructor that is not the form's, is
`mismatched_identity_witness`. Two witnesses claiming one node or one key are
`conflicting_identity_witness`. The identity file today carries `declarations`
rows (record, object, declared unit, composite union) and no member for selected
model tuples, whose `SelectedTupleDeclarationRow` the emission holds in memory
(filament-spec-language `Emission::selected_tuple_declaration_rows`); on the
bytes-only route a tuple payload therefore has no witness and the join refuses it
as above. That is a named design blocker for the producer (a tuple member of the
file, or the adapter supplying the rows), not a weakening of this contract.

### Checks

The join is a native identity join over the admitted package (as
`with_source_union_rows`), so its refusals are the native typed codes with the
structural locus of the row or payload node and the sidecar pointer
(`/model_unions/{k}/...`); a closed-format failure is the identity-file reader's
`Malformed`. It checks in this order and stops at the first refusal; nothing is
returned for a refused or exhausted join.

1. **Format** (`Malformed`). A missing or extra member, an unknown
   `payload.kind`, a payload member foreign to its kind (a `node` on `none` or
   `clock`), a `settled_key`, `type_handle` or `member.declaration` that is not an
   original node key (or is deeper than 32 keys), an `original_case` that is not a
   declaration key, an `original_type` that is not a member of the original
   value-type grammar, a non-integer `rank` or `position`, and rows not ascending
   in `structural_node` order. There is no enum, `VariantId` or fabricated case-node
   member.
2. **Row identity**, in row order. `structural_node` equal to an earlier row's is
   `conflicting_identity_witness` at `structural_node`. It names a node of the
   package (`unknown_node`) that is a `model`/`variant_type` of family `union`
   (`mismatched_identity_witness`; a `composite_type`/`union` node, an enum-family
   `variant_type`, a tuple or any other node refuses, so a source union and an enum
   are never read as a model union). `selected_document` is a lock selection and
   equals the node's owner `identity` (`mismatched_identity_witness`). `type_handle`
   and `settled_key` each differ from every earlier row's and from the handle of
   every supplied declaration and tuple witness (`conflicting_identity_witness` at
   the member).
3. **Case census.** The case names are exactly the selected document's case
   names, each once: a name repeated is `conflicting_identity_witness` at
   `cases/{j}/name`; the first name the document does not declare is
   `mismatched_identity_witness` at `cases/{j}/name`; a declared name with no case is
   `mismatched_identity_witness` at `cases`; and the array is in ascending UTF-8
   byte order of `name` (`mismatched_identity_witness` at the later case's
   `name`).
4. **Per case**, in array order, `mismatched_identity_witness` at the member:
   `rank` equals the index and `position` equals the name's index among the
   document's `variants`; `original_case` equals `{package: selected_document, node:
   <the variant's identity>}`; `member.identifier` equals `name` byte for byte and
   `member.declaration` equals the row's `settled_key`. The same `name` under two
   unions is two members, because the member differs by its declaration.
5. **Payload**, `mismatched_identity_witness` unless stated. `kind` equals the form
   of the case in the document: `none` without a `payloadType`, `clock` when the
   alias-resolved type is Clock, `data` otherwise (any substitution refuses at
   `payload/kind`), and a `clock` payload whose `payload_type` is not the Clock
   identity refuses at `payload/payload_type`. For `data`, `node` equals the package
   node of the case's resolved type (`unknown_node` when the package holds none,
   mismatched when it holds another). The `original_type` is then joined as
   "Side-input witnesses" states: it decodes as a complete original value type,
   equals, constructor for constructor and leaf for leaf, the Exact type the
   admitted type projection (filament-contract-ir FR-352) gives the payload node's
   form, and every original key inside it equals the witness key of the payload node
   (a key that names a different node's witness, or no witness, refuses; an absent
   necessary witness is `unavailable_identity_witness`). A type with no faithful
   Exact representation is `unsupported_exact_type`.
6. **Coverage**, last: under `full`, a union-family node with no row is
   `incomplete_identity_witness` at `/model_unions`; under `reached`, a reached
   union-family node with no row is `unavailable_identity_witness` at its node. A
   `source_unions` row never supplies the missing row and no enum representation
   stands in.

### Work and storage

The join draws on the one caller-owned `NativeJoinBudget` of filament-contract-ir
FR-358, the same budget the strict read and the native source joins use; there is
no separate or default allowance, and this requirement adds no artificial small
budget, scalar cost formula or new form name. Work and storage stay two
counters under FR-358's units and the accounting `with_source_union_rows`
applies, which this join reuses and does not restate: preflight charges for
uniqueness sets and handle indexes before the first row (the handles of every
supplied declaration, tuple and union witness), then per row its work and its
text (a text of length n charges n + 1 to work and to storage), its containers
(one plus their count), its node references and original value types, and a
prior-copy charge when the retained map is shared by handle; a shared node
handle costs one handle unit and no recursive copy, a borrowed row no storage,
and every charge is admitted before the inspection, reserve, clone or insertion
it pays for. Under `reached`, each closure edge examined is a work charge before
any row. Admitted charges are cumulative across the strict read, this join and
every other join on the budget, including those of a refused call; a denied
charge consumes nothing, nothing is refunded, the original limits are reported,
and the refusal is `exhausted_work` or `exhausted_storage` with its configured
bound and the first unpaid charge, whose kind and full attempted amount are
those of the denied charge. The denial's compact locus names the sidecar row
index and the form of the charge; which native locus source carries it is the
budget owner's choice and is not preselected here. The retained rows are staged
and published only when the whole join succeeds; no partial carrier or
projection is returned.

The per-item amounts are fixed by that policy, not by this requirement; the
published vectors therefore assert relations to a baseline measured on the same
input (the baseline delta admits; one unit less of either counter exhausts that
counter with its first unpaid charge), never a static total.

### Authority boundary

The reader validates consistency and preserves the producer's opaque identities; it does NOT claim provenance authentication from parsing. A row
that passes every join is admitted whether or not the producer that wrote it
held the declaration, and admission is no claim that the opaque keys are
authentic. The carrier adds no digest, signature, pin or catalog.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-830-AC-1 | Over `fixtures/positive-model-tuple-union.json`, a `full`-scope sidecar of two rows, List then Shape, with Shape's cases Both (position 2, declared Pair), Empty (position 1, nullary) and Flag (position 0, native Boolean) in canonical order though authored Flag, Empty, Both, each case's `original_case` the document variant's declaration key and each tuple payload joined to its `SelectedTupleDeclarationRow`, is admitted and its rows returned byte for byte; a Shape that also holds `Tail(List)` joins that payload to List's own `type_handle`. | Test |
| FR-830-AC-2 | Under `reached` scope from Shape, a sidecar holding only Shape's row is admitted; a List row that would refuse is neither inspected nor charged, and under `full` the same rows refuse. | Test |
| FR-830-AC-3 | The `reached` closure over List (List, Cell, List), the tuple Nest, the object type Holder, the record Wrapper that reaches Shape, and the record Node and union Link that reach each other terminates, expands each node once, follows case-payload, tuple-position and record or object field edges, requires a row exactly for the union nodes it reaches, and does not require the witness of an unreached record. | Test |
| FR-830-AC-4 | A case whose payload is Clock is carried as `{kind: "clock", payload_type}` with no node and no original value type, and a nullary case, a Clock case and a data case are never substituted for one another. | Test |
| FR-830-AC-5 | Two unions with a case of one spelling are two rows with two members distinguished by their declarations and two original cases. | Test |
| FR-830-AC-6 | A sidecar row with a missing, extra or foreign member, a key, declaration key or original value type that is not of its codec, a key nested beyond 32, a non-integer `position`, an unknown payload kind or rows out of node-id order is `Malformed`. | Test |
| FR-830-AC-7 | A row whose node, selected document, `type_handle` or `settled_key` is wrong, absent, duplicated or of another node form refuses with the code and locus of "Checks" step 2, each mutation chosen so that no earlier check refuses first. | Test |
| FR-830-AC-8 | A missing, extra, repeated or non-canonically ordered case, and a wrong `rank`, `position`, `original_case` or sealed member, refuses with the code and locus of steps 3 and 4. | Test |
| FR-830-AC-9 | A payload whose kind, Clock identity, node, original value type or key differs from the case's resolved payload or from its node's witness, and a tuple witness that is absent or disagrees, refuses with the code and locus of step 5 and "Side-input witnesses". | Test |
| FR-830-AC-10 | A `full` environment without the row of an unused union, an empty `model_unions`, and a `reached` request without a reached union's row (through a tuple, an object, a record or a recursive record and union) refuse `incomplete_identity_witness` or `unavailable_identity_witness`, with no enum or source-union fallback and no partial result. | Test |
| FR-830-AC-11 | On the shared native budget, the baseline delta of a join admits and returns the same rows; one unit less of work is `exhausted_work` and of storage `exhausted_storage`, each with the first unpaid charge's kind, full attempted amount and row-index locus and no output; consumption accumulates across joins and refusals on one budget. | Test |
| FR-830-AC-12 | A row set that passes every join with producer keys the reader cannot authenticate is admitted with its keys returned unchanged, and the contract states the authority boundary in the words of "Authority boundary". | Test |
| FR-830-AC-13 | A unit payload is the model-owned unit node with a `quantity` original type of the declared unit key its `declared_unit` row carries; a compound unit, another key, another node or another type, and an absent unit witness, refuse with the code and locus of step 5. | Test |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Tuple value types and
  payload-bearing variant types" owns the model union, its cases, payload
  arity and `node_id`.
- [FR-440](FR-440-checked-package-union-and-case-nodes.md) and
  [FR-441](FR-441-union-member-key.md) own the member order and the member key.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md)
  "Identity rows" is the source-side precedent of the sidecar.
- filament-contract-ir FR-358 owns the native work and storage budget, FR-352
  the Exact value-type projection and the typed identity-witness refusals this
  requirement joins to and reuses; the FSL identity file owns the original-key
  and value-type codecs and the `declarations` rows. None is restated.
- Vectors: `proposals/checked-package-v2/model-union-witness-vectors.json`
  over `domain-package-acme-shapes.json` and
  `fixtures/positive-model-tuple-union.json`.

## Status

Draft contract with published vectors; no reader implements it and the vectors
are not yet executed by any harness, so nothing is qualified. Two prerequisites
stand open. The unit, record and Holder vectors extend the Shape fixture by
patches (`E-unit`, `E-records`, the Holder `package_patch`) that no reader has
read; executing them needs a full FCD document extension of
`domain-package-acme-shapes.json` with its dimension, unit and record members.
And the identity file has no member for selected model tuples, so a tuple payload
has no witness on the bytes-only route; the producer must carry or supply the
`SelectedTupleDeclarationRow` for the join to admit it.
