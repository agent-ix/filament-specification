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
  witnesses no union.
- A **model-union row** is exactly the closed object
  `{structural_node, selected_document, original_declaration, type_handle,
  settled_key, cases}`. `structural_node` is the `node_id` of the union's
  `model`/`variant_type` node; `selected_document` is the `identity` of its
  `ModelOwner`; `original_declaration` is that owner's `node`; `type_handle` and
  `settled_key` are the producer's original composite type handle and its
  settled declaration key. The two keys, every `original_case` and every
  `original_type` are **opaque**: non-empty text the reader neither decodes nor
  derives from a name, a node id or a payload spelling, returns byte for byte,
  and never treats as equal to any other of these values by resemblance.
- A **case** is exactly `{rank, position, name, document_case, original_case,
  member, payload}`: `rank` the case's index in the canonical order, `position`
  its authored index among the selected document's `variants`, `name` the case
  name, `document_case` that variant's `identity` in the selected document,
  `original_case` the producer's original case identity, and `member` the
  sealed union member `{declaration, identifier}` (FR-441: the declaration key
  and the case identifier, compared componentwise). `rank` and `position` are
  JSON integers; `rank` is never an authored position.
- `payload` is one of three closed forms:
  `{kind: "none"}`, a nullary case with no `payloadType`;
  `{kind: "data", type_kind, payload_type, node, original_type}`, a case whose
  alias-resolved `payloadType` has a node, `type_kind` one of `native`, `unit`
  and `declared`, `payload_type` that resolved type identity, `node` the `node_id`
  of the package node FR-322 step 4 gives that type and `original_type` the
  producer's opaque original value type of the payload; and
  `{kind: "clock", payload_type}`, a case whose alias-resolved `payloadType` is
  `ix://filament/native/Clock`, which has payload arity 0 in the data sense, no
  data node and no `original_type`, and is not a nullary case. A case has at
  most one data payload; there is no list of payloads.
- The **scope** of a join is `full` (the whole selected environment) or
  `reached` (the closure of one requested node). Under `full`, exactly one row
  per union-family `model`/`variant_type` node of the package is required,
  whether or not any term uses it. Under `reached`, the closure starts at the
  requested node and follows, by an explicit worklist with a visited set so a
  cycle terminates, every edge from a node to the node of a type it
  references: a union's data `payload.node`, a tuple's position `typeRef`s and a
  record's or object's field `typeRef`s as FR-322 reads them from the selected
  document. A row is needed exactly for each union-family node in the closure;
  a row outside the closure is not inspected, charged or retained, and an
  unsupported or unrelated declaration outside the closure does not refuse.
  There is no `full` fallback for a `reached` request and no `reached` fallback
  for a `full` one.

### Checks

The reader refuses with the code, cause and locus FR-322 uses for a carrier
row; the locus is an RFC 6901 pointer into the sidecar, so a row's members are
at `/model_unions/{k}/...`. It checks in this order and stops at the first
refusal; nothing is returned for a refused or incomplete join.

1. **Schema.** A missing or extra member, an unknown `payload.kind`, a payload
   member foreign to its kind (a `node` on `none` or `clock`), an empty opaque
   value, and a non-integer `rank` or `position` refuse `malformed_wire` at the
   object or member that breaks the schema. There is no enum, `VariantId` or
   fabricated case-node member.
2. **Row identity**, in row order. `structural_node` is compared against every
   earlier row's (`invalid_package`/`conflicting-definition` at `structural_node`)
   and then against the predecessor's, rows ascending in `node_id` order
   (`invalid_package`/`invalid-value` at the row). It names a node of the package
   (`missing_declaration`/`missing-name`) that is a `model`/`variant_type` of
   family `union` (`ill_typed`/`operator-ineligible`; a `composite_type`/`union`
   node, an enum-family `variant_type`, a tuple or any other node refuses, so a
   source union and an enum are never read as a model union). `selected_document`
   is a lock selection (`missing_declaration`/`missing-selection`) and equals the
   node's owner `identity`, and `original_declaration` equals its owner `node`
   (`invalid_package`/`invalid-value`). `type_handle` and `settled_key` each
   differ from every earlier row's (`invalid_package`/`conflicting-definition`
   at the member).
3. **Case census.** The case names are exactly the selected document's case
   names, each once. A name repeated is `invalid_package`/`conflicting-definition`
   at `cases/{j}/name`; the first name the document does not declare is
   `invalid_package`/`invalid-value` at `cases/{j}/name`; a declared name with
   no case is `invalid_package`/`invalid-value` at `cases`. The array is in
   ascending UTF-8 byte order of `name` (`invalid_package`/`invalid-value` at the
   later case's `name`), so a case order other than the canonical one refuses.
4. **Per case**, in array order: `rank` equals the index and `position` equals
   the name's index among the document's `variants` (`invalid-value` at the
   member); `document_case` equals that variant's `identity`;
   `member.identifier` equals `name` byte for byte and `member.declaration`
   equals the row's `settled_key`; `original_case` differs from every earlier
   case's, in this row and in earlier rows
   (`invalid_package`/`conflicting-definition` at `original_case`). The same
   `name` under two unions is two members, because the member differs by its
   declaration.
5. **Payload.** `kind` equals the form of the case in the document: `none`
   without a `payloadType`, `clock` when the alias-resolved type is Clock,
   `data` otherwise; any substitution, including a Clock case carried as
   nullary or data and a nullary case carried as Clock, is
   `invalid_package`/`invalid-value` at `payload/kind`. For `data`,
   `payload_type` equals the alias-resolved identity of the case's
   `payloadType`, `type_kind` is the kind of that type, and `node` equals the
   package node of that type (a node absent from the package is
   `missing_declaration`/`missing-name`, another node of the package
   `invalid_package`/`invalid-value`, at the member). `original_type` is
   carried and never checked against `payload_type` or `node`.
6. **Coverage**, last: under `full`, a union-family node with no row, and under
   `reached`, a reached union-family node with no row, refuses
   `invalid_package`/`invalid-value` at `/model_unions`. A `source_unions` row
   never supplies the missing row.

### Work and storage

All inspection and retention of the join consumes the reader's single native
budget, the one the strict read of the package draws on, charged before the
operation it pays for; there is no separate or default budget for the join.
The units are: one for each closure edge examined (`reached` scope only, before
any row), one for each row, one for each case, one more for each `data`
payload, and the UTF-8 byte length of every string value of a row (object keys
and numbers excluded) charged once when the row is inspected and once more when
the row is retained, the retention charge falling after the row's checks
succeed and before the row is cloned. A charge the budget cannot cover returns
`incomplete` carrying the typed denial `{phase, locus, requested, remaining}`,
`phase` one of `edge`, `visit` and `retain`. Exhaustion is checked before
any later check, an earlier refusal wins over a later exhaustion, retained
rows are staged and published only when the whole join succeeds, and no partial
carrier or projection is returned. Exactly the total admits; one unit less
returns `incomplete` at the last charge.

### Authority boundary

The reader validates consistency and preserves the producer's opaque
identities; it does NOT claim provenance authentication from parsing. A row
that passes every join is admitted whether or not the producer that wrote it
held the declaration, and admission is no claim that the opaque values are
authentic. The carrier adds no digest, signature, pin or catalog.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-830-AC-1 | Over `fixtures/positive-model-tuple-union.json`, a `full`-scope sidecar of two rows, List then Shape, with Shape's cases Both (position 2, `declared` Pair), Empty (position 1, `none`) and Flag (position 0, `native` Boolean) in canonical order though authored Flag, Empty, Both, is admitted and its rows returned byte for byte. | Test |
| FR-830-AC-2 | Under `reached` scope from Shape, a sidecar holding only Shape's row is admitted; a List row that would refuse is neither inspected nor charged, and under `full` the same rows refuse. | Test |
| FR-830-AC-3 | The `reached` closure over List (List, Cell, List) and over the tuple Nest terminates, expands each node once and requires a row exactly for the union nodes it reaches. | Test |
| FR-830-AC-4 | A case whose payload is Clock is carried as `{kind: "clock", payload_type}` with no node and no `original_type`, and a nullary case, a Clock case and a data case are never substituted for one another. | Test |
| FR-830-AC-5 | Two unions with a case of one spelling are two rows with two members distinguished by their declarations. | Test |
| FR-830-AC-6 | A sidecar row with a missing, extra or foreign member, an empty opaque value, a non-integer `position` or an unknown payload kind refuses `malformed_wire`. | Test |
| FR-830-AC-7 | A row whose node, document, original declaration, `type_handle`, `settled_key` or position among the rows is wrong, absent, duplicated or of another node form refuses with the code, cause and locus of "Checks" step 2. | Test |
| FR-830-AC-8 | A missing, extra, repeated or non-canonically ordered case, and a wrong `rank`, `position`, `document_case`, sealed member or duplicated `original_case`, refuses with the code, cause and locus of steps 3 and 4. | Test |
| FR-830-AC-9 | A payload whose kind, `payload_type`, `type_kind` or node differs from the case's resolved payload refuses with the code, cause and locus of step 5. | Test |
| FR-830-AC-10 | A `full` environment without the row of an unused union, an empty `model_unions`, and a `reached` request without a reached union's row refuse `invalid_package`/`invalid-value` at `/model_unions`, with no enum or source-union fallback and no partial result. | Test |
| FR-830-AC-11 | The exact total budget admits; zero returns `incomplete` at the first row visit, and one unit less than the total returns `incomplete` at the last row's retention charge with the typed denial and no output. | Test |
| FR-830-AC-12 | A row set that passes every join with producer values the reader cannot authenticate is admitted with its opaque values returned unchanged, and the contract states the authority boundary in the words of "Authority boundary". | Test |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Tuple value types and
  payload-bearing variant types" owns the model union, its cases, payload
  arity and `node_id`; the reader's budget and refusal vocabulary are its.
- [FR-440](FR-440-checked-package-union-and-case-nodes.md) and
  [FR-441](FR-441-union-member-key.md) own the member order and the member key.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md)
  "Identity rows" is the source-side precedent of the sidecar.
- Vectors: `proposals/checked-package-v2/model-union-witness-vectors.json`
  over `domain-package-acme-shapes.json` and
  `fixtures/positive-model-tuple-union.json`.

## Status

Draft contract with published vectors; no reader implements it and the vectors
are not yet executed by any harness.
