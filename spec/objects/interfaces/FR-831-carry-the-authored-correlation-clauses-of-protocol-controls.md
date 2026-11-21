---
id: FR-831
title: "Carry the authored correlation clauses of protocol controls"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-466
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
  - target: ix://agent-ix/filament-specification/FR-822
    type: references
  - target: ix://agent-ix/filament-contract-ir/FR-358
    type: references
---

# FR-831: Carry the authored correlation clauses of protocol controls

## Description

The optional top-level member `protocol_correlation_occurrences`, beside
`claim_names`, records for each related-bearing control occurrence of a checked
package, in source order, which canonical `related` record (FR-466) each
authored correlation clause became and under which local alias it was written.
Four parties act on it, each under its own obligation:

- A producer that hands off a checked package whose protocol controls carry
  `related` records MAY publish the member; when it does, it SHALL record every
  authored clause before deduplication, in source order, with the checked
  `Identifier` spelling of the alias and its actual canonical position.
- A reader that receives the member SHALL validate it as below and SHALL expose
  it as a typed read.
- A reader that does not receive the member SHALL report the provenance
  unavailable and SHALL NOT treat the absence as an empty provenance.
- A consumer that needs the clause order of a control whose canonical `related`
  is nonempty and holds only the unavailable provenance SHALL refuse with
  `unavailable_correlation_provenance` and SHALL NOT proceed with empty or
  guessed premises.

## Rationale

A control's `related` argument is canonical: its records are sorted by
relationship node id then the JCS bytes of `from` and `to`, and two authored
clauses that name one relationship with equal ends are one record
(FR-466 layout, FR-322-AC-388). The authored order, the multiplicity and the
local alias each clause was written under are therefore not recoverable from
the node body. A consumer that reports the first failing clause in source order
(FSL FR-731) needs them, and `protocol_locations` cannot carry them: it is
keyed by the application occurrence and the emitted position, not by the clause.
The checked-package identity of the control, its canonical triples, its node ids
and every existing carrier must not change, so the facts live in a separate
member outside every identity preimage.

## Behavior

### Carrier

`protocol_correlation_occurrences` is an optional top-level member of the
checked package, a JSON array, absent or present and never `null`. It is not a
member of the identity sidecar, of `checked_term_data`, of any node body or of
any preimage. Each row is a closed object of exactly five members:

| Member | Value |
| --- | --- |
| `control` | the control's full `NodeRef` (`package`, `path`, `ordinal`) |
| `occurrence` | the control's `declaration` occurrence `{role, ordinal}` (the existing `Occurrence` form) |
| `canonical_index` | the zero-based position, in that control node's emitted `related` argument, of the canonical record the authored clause became |
| `alias` | the checked local relationship alias exactly as the source spelled it, an `Identifier` of the V2 schema |
| `source_ordinal` | the zero-based source position of the clause among the correlation clauses of that control occurrence |

A row is one authored clause. The group of a control occurrence is its rows by
`source_ordinal`. Two or more rows of one group may carry one `canonical_index`
(identical authored clauses, or two aliases of one relationship with equal ends)
and one `alias` may repeat across `canonical_index` values. The alias is the
clause's local name: it need not equal the relationship's model declaration
name, is never used to select a relationship and is never compared with one.
Semantics reads the canonical record named by `canonical_index`; the alias is
for diagnostics only.

The table is a grouping of authored clauses under one control `Declaration`
occurrence. It is not a dynamic execution, actor or step record.

### Source-unit context

The row stores no source property. The source context of a group is the admitted
`source_map` entry whose full key is `(control, role, ordinal)` = (`control`,
`declaration`, `occurrence.ordinal`): the entry that FR-322 "source_map" keys by
`(node_id, role, ordinal)`, that its stage 5 and the reader's source-map check
admit once per key, and that carries nonempty `regions` whose `source`
identities are locked. The typed read borrows that **entire** entry, every
region of it, from the retained document; it keeps no region subset, copies no
region and reads no source bytes from a `SourceRef`. `occurrence` alone names a
role and an ordinal and no source; a reader never resolves it by ordinal alone
and never falls back to another package's entry.

A control node carries no `owner` (FR-466), so there is no `SourceOwner`
coherence to check for a control: the entry's regions are its only source
context, and FR-322's owner coherence, where it applies to a node, is already
enforced before this member is read. A control has one declaring construct, so
FR-464 admits one `declaration` occurrence of it (ordinal 0); no admitted
package holds a control with two declaration occurrences, and a row naming
`occurrence.ordinal` 1 of a control that has one refuses at `occurrence`.

A control of a dependency package is resolved only through that package's own
member and own `source_map`: a package's member names only controls of that
package, a reader never consults the root package's member for a dependency
control, and a dependency package with no member is unavailable for its own
controls whatever the root carries.

### Eligible controls and coverage

An **eligible control occurrence** is each occurrence of role `declaration` of
each `protocol`/`control` node whose operator is one of `send`, `receive`,
`attempt`, `effect`, `event` or `recommend` (the six FR-466 layouts that carry
`related`; `recommend` is the FR-822 control and is not dropped). No other
operator, form or role is eligible.

A present member is a **complete** claim: for every eligible control occurrence
of the package, the group's `canonical_index` values cover every position
`0..m-1` of the control's emitted `related` array (`m` records), and a control
occurrence whose `related` is empty has no row. `[]` is valid exactly when every
eligible control occurrence has `m = 0`. Source ordinals of one group run
`0..k-1` with no gap, `k >= 1` when `m >= 1`.

The member claims nothing about authenticity or about source completeness the
graph alone cannot witness: a reader cannot prove that an omitted trailing
duplicate clause, or the exact authored spelling of an alias, was recorded.
Capturing **every** checked authored clause before deduplication, in source
order, with the checked `Identifier` spelling, and mapping each to its actual
canonical position (never by looking up the alias or relationship key), is
producer conformance; the reader checks consistency only.

### Reader checks and refusal order

The step runs after every protocol node has passed FR-464 "Reader order" (steps 1
to 7) and after the `checked_term_data` records, as the last carrier step of
FR-322 "Check order". Strict wire validation precedes it. The reader makes
**one pass over the rows in array order, then one coverage pass**, and decides
each charge and each defect at the position the pass reaches it: a defect or a
denied charge is the first one met in this order, and no scan, index or table is
built outside it. `i` is the row's array index.

**Row `i`** is handled in this order:

1. **Charge** (`work`): `1 + B + L` units, `B` the UTF-8 byte length of
   `control.package` plus the byte length of every segment of `control.path` (an
   index segment counts the bytes of its decimal numeral) and `L` the UTF-8 byte
   length of `alias`, charged before the row is inspected. The join resolution, the
   key comparison with the predecessor and the `source_map` lookup examine only this
   row's `control` text and `alias`, so `B` and `L` pay for every byte they read and
   no unmetered length scan precedes the charge. The `alias` and the control text
   are borrowed from the retained document and are neither copied nor charged as
   storage; a reader that copies either SHALL charge the copied bytes first.
2. **Joins**, in this order:
   1. `control` names a node of this package, else `invalid_package`/
      `invalid-value` at `/protocol_correlation_occurrences/{i}/control` (an
      absent node, a node of another package and a dependency package's node
      alike);
   2. that node is a `protocol`/`control` node of an eligible operator, else
      `ill_typed`/`operator-ineligible` at `.../control`;
   3. `occurrence` is `declaration` and the control carries it and an admitted
      `source_map` entry exists for the full key, else `invalid_package`/
      `invalid-value` at `.../occurrence`;
   4. `canonical_index < m`, else `invalid_package`/`invalid-value` at
      `.../canonical_index`.
3. **Key**, against the previous row, by the full key (`control`,
   `occurrence.ordinal`, `source_ordinal`): `control` under FR-322's lexicographic
   `NodeRef` order (package in UTF-8 byte order, then path segment by segment with a
   shorter path first, then ordinal), then `occurrence.ordinal` numerically, then
   `source_ordinal` numerically. `occurrence.role` is not an ordering member: join 3
   admits only `declaration`, so every row that reaches this step carries that one
   constant role and no order over roles exists to define. An equal full key is
   `invalid_package`/`conflicting-definition` at
   `/protocol_correlation_occurrences/{i}`; a key that sorts before its
   predecessor's is `invalid_package`/`invalid-value` at the same locus. An equal
   key is checked before order.
4. **Ordinal**: the first row of a `(control, occurrence)` group has
   `source_ordinal` 0 and each later row of the group one more than the previous
   row, else `invalid_package`/`invalid-value` at
   `/protocol_correlation_occurrences/{i}/source_ordinal`.

A defect of row `i` therefore precedes any defect or charge of row `i + 1`: a row
that fails its joins is refused before a later row's own join, order or ordinal
defect is looked at, and within one row a join defect precedes a key defect.

**After the last row passes**, the reader requests the typed view and runs the
**coverage** pass:

5. **View** (`read_storage`): one block of `G` bytes, `G` the number of distinct
   `(control, occurrence)` groups times the size of the native group record
   (control reference, occurrence, the borrowed whole `source_map` entry, the row
   span), charged in full before it is requested. A member with no row requests
   no block.
6. **Coverage**: every `protocol`/`control` node of the package in ascending
   structural node-id order, one charge each (`work`): `1 + Bn` for a node that is
   not eligible and `1 + Bn + O + m` for an eligible one, `Bn` the byte length of
   the node id counted as `B` counts a `NodeRef`, `O` its declaration occurrences
   and `m` its canonical related records, charged before the node is inspected.
   Every node-id comparison of the walk against the next group's control reads at
   most `Bn` bytes of that node, so the charge pays for it; the pass is metered even
   when the member is `[]`.
   For an eligible node with
   `m >= 1`, a block of `m` one-byte flags (`read_storage`, `m` bytes) is charged
   after the node's work charge and before it is requested; a block of zero bytes
   is not charged. The first eligible occurrence with `m >= 1` and a canonical
   position no row names (no row at all included) is `invalid_package`/
   `invalid-value` at `/protocol_correlation_occurrences`, retaining the control,
   the occurrence and the first uncovered `canonical_index`; this includes `[]`
   over a package whose controls carry related records. An eligible occurrence
   with `m = 0` and a row has already refused at its row's `canonical_index`
   join.

**Budget.** The charges are made on the one caller-owned `NativeJoinBudget` of
the strict read: the work counter and the read-storage counter that
filament-contract-ir FR-358 owns and defines, in its units (FR-322 defines
neither, and this requirement takes both from FR-358). The budget is cumulative
across the strict read of the package, every earlier join and this step, never
reset and never refunded: a denied charge consumes nothing. This contract adds no
limit field and claims no counter of its own. A denied charge returns the
reader's `incomplete` result naming the counter (`work` or `read_storage`) and the
full amount attempted, at `/protocol_correlation_occurrences/{i}` for a row charge
and at `/protocol_correlation_occurrences` for every view, scan and flag charge,
with no partial typed view. An absent member charges nothing.

The vectors state every limit as a **baseline plus an offset**, never a static
total: the baseline is each counter's consumption immediately before this step,
measured on the same input, because it depends on the strict read of the whole
package.

The closed schema refuses as `malformed_wire`, at the row or member that breaks
it: a `null` or non-array member, a non-object row, a missing or extra member, a
`control` or `occurrence` of the wrong shape, an `occurrence.role` outside the
six `Occurrence` roles, an `alias` that is not an `Identifier` (empty, leading
digit, a character outside `[A-Za-z0-9_]`, not a string), and a
`canonical_index` or `source_ordinal` that is negative, fractional or above
9007199254740991.

### Typed read

A reader that admitted the package exposes one of two outcomes and no third:

- `unavailable_correlation_provenance` when the member is absent. The package is
  admitted and every other read is unchanged. A consumer that needs the clause
  order of a control whose canonical `related` is nonempty and receives it SHALL
  refuse with that name and SHALL NOT proceed with empty or guessed premises. It
  is distinct from `incomplete` (a work limit) and from every refusal above.
- the complete table, when the member is present, as borrowed rows grouped by
  control occurrence in table order, each group holding its whole `source_map`
  entry (every region, borrowed). A present `[]` is a complete empty
  provenance, never the same outcome as absence. The read is all or nothing: a
  denied charge or a refusal yields no view of any prefix of the table.

### Authority and invariance

The member adds no node, changes no `node_id`, body, `dependencies`,
`semantic_type`, `source_map` entry, `checked_term_data` row or
`protocol_locations` row, and is in no identity preimage or `identityRef`.
Canonical `related` triples are unchanged by duplicate authored clauses: a
program with one clause and a program that repeats it twice or more produce the
same nodes, canonical triples, node ids, `identityRef`, `source_map` and
`protocol_locations`, and differ only in the member. The member is part of the
package bytes: a package with it and one without it are different bytes, and
every invariance claim over whole package bytes (for example the golden
comparison of FR-322-AC-473) is scoped to packages that do not carry it.
Reversing the source order of clauses changes the rows' `source_ordinal` to
`canonical_index` mapping and no canonical fact.

A first-failing-clause consumer keeps the source-order oracle of FSL FR-731
unchanged: it evaluates the group's rows in `source_ordinal` order, takes the
canonical relationship from the record at `canonical_index` and names the row's
`alias` in its diagnostic. The vectors are static inputs and expected reader
results; they do not execute that dynamic replay.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-831-AC-1 | A package with no member is admitted and its typed read is `unavailable_correlation_provenance`, whether its controls' `related` arrays are empty or not; the same package carrying `[]` is admitted as a complete empty provenance when every eligible control occurrence has an empty `related`, and refuses at the member (FR-831-AC-5) when any carries a record. | Test (TC-2113) |
| FR-831-AC-2 | Rows for zero, one and many clauses of one control are admitted: two or more identical authored clauses give two or more rows of one `canonical_index` over one canonical record; repeated aliases, two aliases of one canonical record and an alias unlike the relationship's model name are admitted; reversing the source order of two clauses is admitted with the `source_ordinal` mapping reversed and the same canonical graph. | Test (TC-2113) |
| FR-831-AC-3 | Every eligible form is admitted: `send`, `receive`, `attempt`, `effect`, `event` and `recommend`; a row naming any other control operator or form refuses `ill_typed`/`operator-ineligible` at `control`. | Test (TC-2113) |
| FR-831-AC-4 | A row whose `control` names no node of the package, a node of another package or a dependency package's control, or whose `occurrence` is not a declaration occurrence of the control with an admitted `source_map` entry under the full key, refuses `invalid_package`/`invalid-value` at `control` or `occurrence`; a dependency control is read only from its own package's member, and no root member or ordinal-only lookup supplies it. | Test (TC-2113) |
| FR-831-AC-5 | In one pass over the rows, each row is charged, then joined, then keyed against its predecessor, then ordinal-checked; a `canonical_index` at or above the control's `related` length, an equal key (`conflicting-definition`, before order), a row out of key order, a gap or a nonzero start of `source_ordinal` each refuse with the code, cause and locus of "Reader checks and refusal order", the first met in that order, so a defect of an earlier row precedes any defect of a later one and a join defect precedes a key defect of the same row. After the last row, an eligible control occurrence with an uncovered canonical position or no group, and `[]` over a nonempty `related`, refuse at the member. | Test (TC-2113) |
| FR-831-AC-6 | A malformed shape (non-array, `null`, missing or extra member, wrong member shape, a role outside the closed set, a lexically invalid `alias`, a negative, fractional or beyond-safe integer) is `malformed_wire` at the first schema error path and is not an admission refusal. | Test (TC-2113) |
| FR-831-AC-7 | Charging draws on the one `NativeJoinBudget` of filament-contract-ir FR-358 (its work and read-storage counters, no new limit field and no separate counter) and is interleaved with the pass, never preceded by an unmetered scan: a row costs `1 + B + L` work before it is inspected, `B` the bytes of its control package and path segment texts and `L` the bytes of its alias; each `protocol`/`control` node costs `1 + Bn` work, plus `O + m` when eligible, before it is inspected, even for `[]`; the view block `G` and each eligible node's `m` flag bytes are charged to `read_storage` in full before they are requested. Over the measured baseline the exact offset admits; one unit short returns `incomplete` naming the counter, the first unpaid amount and the locus of the vectors, with no partial view and no reset of earlier consumption; a charge denied by earlier consumption is denied at the same first unpaid charge; an absent member charges nothing. | Test (TC-2113) |
| FR-831-AC-8 | A producer that lowers one source with an authored clause written once, the same source with the clause repeated, and the same source with the clauses in the opposite order emits the same nodes, canonical `related` triples, node ids, `identityRef`, `source_map` entries and `protocol_locations` rows, and the three packages differ only in the member's rows. | Test (TC-2113) |
| FR-831-AC-9 | The typed read of a group borrows the whole admitted `source_map` entry of the full key `(control, declaration, occurrence.ordinal)` with every region; a dependency control is read from its own package's member and entry and never from the root's; a row naming a second declaration occurrence of a control that has one (FR-464) refuses `invalid_package`/`invalid-value` at `occurrence`. | Test (TC-2113) |
| FR-831-AC-10 | Every vector case is a complete published package that is schema-valid with and without the member; its rows are consistent with the package under the full key order (control, occurrence ordinal, source ordinal), the per-group ordinals, the `source_map` lookup by the full key and the coverage of every canonical position, each read from the package; and every budget expectation equals the charges recomputed from the case's package and rows. | Test (TC-2113) |
| FR-831-AC-11 | The member is a separate top-level member: a package that carries it equals the same package without it, or with other rows, in every other member. | Test (TC-2113) |
| FR-831-AC-12 | A consumer that needs the clause order of a control whose canonical `related` is nonempty and holds only an absent member refuses with `unavailable_correlation_provenance` and uses no empty or guessed premises; a control whose `related` is empty needs no clause order and is not refused; a complete table supplies the clauses in source order. | Test (TC-2113) |

## Dependencies

- [FR-466](FR-466-checked-package-protocol-control-bodies.md) owns the six
  `related`-bearing layouts and the canonical `related` order this member is
  keyed to; [FR-322](FR-322-checked-package-artifact.md) owns the optional
  carrier order, the `source_map` key, the owner join and the `protocol_locations`
  `related` end sites, all unchanged; [FR-464](FR-464-checked-package-protocol-node-bodies.md)
  owns the reader order this step follows; FR-822 owns `recommend`.
- filament-contract-ir FR-358 owns the `NativeJoinBudget`, its work counter and
  its read-storage counter with their units, which this step charges and does not
  restate; FR-322 defines none of them. The IR reader that will carry this step
  owns the charges' implementation.
- FSL FR-731 owns first-failing-clause evaluation in source order; the FSL
  protocol lowering builds, sorts, renames and deduplicates authored records and
  is the producer of this member. None is restated.
- Vectors: `proposals/checked-package-v2/protocol-correlation-vectors.json`, built on complete published packages (the base `fixtures/protocol-node-base.json` plus RFC 6902 `ops`, and the complete dependency package `fixtures/protocol-correlation-dependency.json`), verified by TC-2113.
- Schema: `related` of `receive`, `attempt`, `effect` and `event` uses the same records term as `send`, and `recommend` has its own layout rule and operator spelling, as FR-466 lays them out; before this contract the schema admitted a nonempty `related` only on `send` and no `recommend` control.

## Status

Draft contract. No reader, producer or consumer implements it: the existing strict reader refuses the member as `unknown_member` (`UnknownMember`), so every vector that carries the member states the expected result of a reader that has the carrier, and none is executed against any reader. FR-831-AC-1 to AC-5, AC-7 to AC-9 and AC-12 are therefore pending verification by the IR reader (FR-358 budget included), the FSL producer (AC-8, with real checked source variants) and a consumer (AC-12), and the existing FSL guard on missing provenance stays until that capability and its pin land. Executed now, as data and schema qualification only: the complete packages and every case edit against the V2 schema (AC-10), the schema decisions on the member (AC-6), the rows' consistency with their packages and the recomputed charge model (AC-10) and the member's separateness (AC-11). No implementation, capability pin or version is chosen here; they follow review of this contract and a reader.
