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
---

# FR-831: Carry the authored correlation clauses of protocol controls

## Description

When a producer hands off a checked package whose protocol controls carry
`related` records (FR-466), it MAY publish the optional top-level member
`protocol_correlation_occurrences`, beside `claim_names`, that records for each
related-bearing control occurrence, in source order, which canonical `related`
record each authored correlation clause became and under which local alias it
was written. A reader that receives the member SHALL validate it as below and
expose it as a typed read; a reader that does not receive it SHALL report the
provenance unavailable and SHALL NOT treat the absence as an empty provenance.

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

The row stores no source property. The source context of a group is read from
the admitted `source_map` entry whose full key is `(control, role, ordinal)` =
(`control`, `declaration`, `occurrence.ordinal`): the entry that FR-322
"source_map" keys by `(node_id, role, ordinal)`, that its stage 5 and the
reader's source-map check admit once per key, and that carries nonempty
`regions` whose `source` identities are locked. `occurrence` alone names a role
and an ordinal and no source; a reader never resolves it by ordinal alone, never
falls back to another package's entry, and never reads source bytes from a
`SourceRef`. A control node carries no `owner`, so its entry's regions are its
only source context; where FR-322's owner coherence applies to a node it is
already enforced before this member is read.

A control of a dependency package is resolved only through that package's own
member and `source_map`. A package's member names only controls of that package;
a reader never consults the root package's member for a dependency control, and
a dependency package with no member is unavailable for its own controls whatever
the root carries.

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
FR-322 "Check order". Strict wire validation precedes it. A defect is the first
in this order; `i` is the row's array index.

0. **Charge.** Before the first inspection and before any key index, group
   table, expected-coverage table or borrowed view is allocated, the reader
   charges the `work` meter (FR-322 reader limits, the counter that bounds
   semantic-term validation visits): for each row in array order one unit plus
   the UTF-8 byte length of its `alias`; then, in ascending structural node-id
   order, for each eligible control occurrence one unit plus the number `m` of
   canonical related records it carries (the coverage scan is charged even when
   the member is `[]`). A charge that would pass the limit returns the reader's
   `incomplete` result naming `work`, at `/protocol_correlation_occurrences/{i}`
   for a row charge and at `/protocol_correlation_occurrences` for a scan charge,
   with no partial typed view. An absent member charges nothing.
1. **Row joins**, rows in array order, per row in this order:
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
2. **Key order**, rows in array order against the previous row, by the key
   `(control, occurrence, source_ordinal)` under FR-322's ascending structural
   `NodeRef` order, then `role`, `ordinal`, `source_ordinal`: a repeated key is
   `invalid_package`/`conflicting-definition` at `/protocol_correlation_occurrences/{i}`
   of the later row; a row that sorts before its predecessor is
   `invalid_package`/`invalid-value` at the same locus. A repeated key is checked
   before order.
3. **Ordinals**, per group in table order: the first row whose `source_ordinal`
   differs from its position in its group is `invalid_package`/`invalid-value`
   at `/protocol_correlation_occurrences/{i}/source_ordinal`.
4. **Coverage**, eligible control occurrences in ascending structural order: the
   first occurrence with `m >= 1` and a canonical position no row names (no row
   at all included) is `invalid_package`/`invalid-value` at
   `/protocol_correlation_occurrences`, retaining the control, the occurrence and
   the first uncovered `canonical_index`. This includes `[]` over a package
   whose controls carry related records.

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
  control occurrence in table order, each with the group's source identities
  read from its `source_map` entry. A present `[]` is a complete empty
  provenance, never the same outcome as absence.

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
| FR-831-AC-1 | A package with no member is admitted and its typed read is `unavailable_correlation_provenance`, whether its controls' `related` arrays are empty or not; a consumer that needs the clause order of a control with a nonempty `related` and holds only the absent member refuses with that name and never proceeds with empty premises, and the same package with `[]` is admitted as a complete empty provenance. | Test (TC-2113) |
| FR-831-AC-2 | Rows for zero, one and many clauses of one control are admitted: two or more identical authored clauses give two or more rows of one `canonical_index` over one canonical record; repeated aliases, two aliases of one canonical record and an alias unlike the relationship's model name are admitted; reversing the source order of two clauses is admitted with the `source_ordinal` mapping reversed and the same canonical graph. | Test (TC-2113) |
| FR-831-AC-3 | Every eligible form is admitted: `send`, `receive`, `attempt`, `effect`, `event` and `recommend`; a row naming any other control operator or form refuses `ill_typed`/`operator-ineligible` at `control`. | Test (TC-2113) |
| FR-831-AC-4 | A row whose `control` names no node of the package, a node of another package or a dependency package's control, or whose `occurrence` is not a declaration occurrence of the control with an admitted `source_map` entry under the full key, refuses `invalid_package`/`invalid-value` at `control` or `occurrence`; a dependency control is read only from its own package's member, and no root member or ordinal-only lookup supplies it. | Test (TC-2113) |
| FR-831-AC-5 | A `canonical_index` at or above the control's `related` length, a repeated key, a row out of key order, a gap or a nonzero start of `source_ordinal`, an eligible control occurrence with an uncovered canonical position or no group and `[]` over a nonempty `related` each refuse with the code, cause and locus of "Reader checks and refusal order", the first in that order. | Test (TC-2113) |
| FR-831-AC-6 | A malformed shape (non-array, `null`, missing or extra member, wrong member shape, a role outside the closed set, a lexically invalid `alias`, a negative, fractional or beyond-safe integer) is `malformed_wire` at the first schema error path and is not an admission refusal. | Test (TC-2113) |
| FR-831-AC-7 | Charging is precharged and exact: with the member's rows and the coverage scan charged as stated, the exact limit admits and a limit one short returns `incomplete` naming `work` at the stated locus with no partial view; an absent member charges nothing. | Test (TC-2113) |
| FR-831-AC-8 | Adding duplicate authored clauses leaves every node, canonical triple, node id, `identityRef`, `source_map` entry and `protocol_locations` row unchanged and changes only the member; a package with the member differs in bytes from one without it. | Test (TC-2113) |

## Dependencies

- [FR-466](FR-466-checked-package-protocol-control-bodies.md) owns the six
  `related`-bearing layouts and the canonical `related` order this member is
  keyed to; [FR-322](FR-322-checked-package-artifact.md) owns the optional
  carrier order, the `source_map` key, the owner join and the `protocol_locations`
  `related` end sites, all unchanged; [FR-464](FR-464-checked-package-protocol-node-bodies.md)
  owns the reader order this step follows; FR-822 owns `recommend`.
- FSL FR-731 owns first-failing-clause evaluation in source order; the FSL
  protocol lowering builds, sorts, renames and deduplicates authored records and
  is the producer of this member. None is restated.
- Vectors: `proposals/checked-package-v2/protocol-correlation-vectors.json`,
  verified by TC-2113.

## Status

Draft contract. No reader or producer implements it, no harness executes the
vectors and no existing reader is qualified against them: the vectors are inputs
and expected results for a carrier that does not exist yet. The JSON Schema
change is the only machine-checked part. No implementation, capability pin or
version is chosen here; they follow review of this contract and a reader.
