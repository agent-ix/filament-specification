---
id: FR-352
title: "Produce and read the native-run-result/2 wire"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-014
    type: references
  - target: ix://agent-ix/filament-specification/FR-351
    type: references
  - target: ix://agent-ix/filament-specification/FR-243
    type: references
  - target: ix://agent-ix/filament-specification/FR-207
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-115
    type: references
  - target: ix://agent-ix/filament-specification/FR-364
    type: references
  - target: ix://agent-ix/filament-specification/FR-414
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-267
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-330
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-031
    type: references
  - target: ix://agent-ix/filament-specification/FR-174
    type: references
  - target: ix://agent-ix/filament-specification/FR-175
    type: references
  - target: ix://agent-ix/filament-specification/FR-360
    type: references
  - target: ix://agent-ix/filament-specification/FR-241
    type: references
  - target: ix://agent-ix/filament-specification/FR-242
    type: references
  - target: ix://agent-ix/filament-specification/FR-290
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-572
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-574
    type: references
---
# FR-352: Produce and read the native-run-result/2 wire

## Description

When a native run result is produced or read, the producer and the strict
reader SHALL use the closed `native-run-result/2` clause-run wire: the base
members below plus the FR-243 settlement `basis` and the FR-351 separating
witness record. When a local projection or realizability result is produced
or read, they SHALL use the same wire's protocol-analysis record
("Protocol-analysis result"). FR-014 supplies the exact-version and
closed-wire reading rules; it does not define run-result members.

## Inputs

A clause run's disposition, package identity and usage, its FR-243 settlement basis and,
where the basis is `decisive-witness` or `decisive-counterexample`, its
FR-351 separating witness record; for a temporal selection, its FSL FR-330
temporal report. For a protocol-analysis record: the analysis kind, its
derived-artifact key, its FR-360 settlement and FR-331 result entry, and for
a decisive counterexample its FR-351 record and the request's
assessed-subject root.

## Outputs

A `native-run-result/2` record, or a located wire diagnostic.

## Behavior

`native-run-result/2` is the CLI `run` serialization of a clause run and the
run-result identifier of FR-014's registry. A producer writes it, and a reader
reads a document as a native run result only when its identifier is exactly
`native-run-result/2`; any other identifier is not a native run result and
refuses before consumption (FR-014 `unsupported-version`). A reader never
infers the identifier from payload content.

## Members

A clause-run record has exactly these top-level members: `format`
(`native-run-result/2`), `disposition`, `usage` and `basis`, plus
`limits` and `witness` under their conditions
below, `extraction` when the run's unit is an extracted Markdown clause
body, and `ledger` under the condition of "Progress ledger" below. Absent
optional members are omitted, never `null`.

The producer SHALL write `extraction` exactly when the clause run's unit is
the body that FSL FR-030 extracted from an original Markdown document (FSL
FR-031). FSL FR-031 owns the meaning of the record it carries. `extraction`
is a closed object with exactly these members, every string a JSON string
and every offset an unsigned JSON integer:

| Member | Value |
| --- | --- |
| `original` | closed object: `authority` and `identity`, the original document's identity labels; `path`, its authored path |
| `body` | closed object: `authority` and `identity`, the body's native identity; `document`, its formal identity |
| `binding` | closed object: `clause`, the authored clause id (the Quire heading id); `package`, the authored package |
| `outcome` | closed object: `availability`, `clauses`, `clause_text` and `diagnostics`, Quire's extraction outcome members in Quire's own encoding, unchanged; `clauses` is omitted when Quire reports no clause population |
| `map` | closed object: `region`, the body's original region as `{start, end}` byte offsets; `segments`, a non-empty array of closed objects `{body_start, body_end, original_start, original_end}` in ascending `body_start` order |
| `original_regions` | present exactly for a refusal whose locus names a body span: a non-empty array, in original-document order, of closed regions `{start, end}`, each endpoint a closed object `{byte, line, column}` |

The strict reader SHALL restore it without losing any member and SHALL refuse
an `extraction`, or a nested object of it, with an unknown, missing,
duplicate or `null` member, with a value of another JSON type, or with an
empty `segments` or `original_regions` array.

`disposition` carries FSL FR-109's clause-run disposition without loss. Its
`stage` and `category` are required; its applicable `truth` and refusal
record are retained. A `compile`, `select` or `admit` refusal retains its
FR-109 `RefusalRecord`, including `code`, `cause` and `locus`. An `evaluate`
refusal, unsupported or incomplete result retains FR-100's exact `outcome`
vocabulary, including `code`, `cause`, `fields`, `locus` and `location` when
that outcome assigns them; its `refused` form is not an empty tag. An
`evaluate` internal failure carries `internal_fault` as a closed object
with the FR-100 `InternalFault`'s string `stage` and `invariant`, and no
`outcome` member. The FSL FR-100 and FR-109
contracts determine which of these members are present for each category;
the `/2` writer and reader SHALL preserve them and SHALL NOT replace an
assigned member with `null`, a default or a generic refusal tag.

For an evaluated FSL FR-115 `Frame` violation, `disposition.refusal`
additionally carries `frame_witness`: the complete FR-115 `FrameWitness`
in FSL FR-267's closed encoding. This member is present exactly for that
case, even though the disposition category is `violation`; the enclosing
refusal's code and cause remain `frame_violation`/`unauthorized-change`.
The frame witness is distinct from the top-level FR-351 `witness`, which
is absent when the frame verdict has `basis: closed-scope`. The strict
reader SHALL restore the FR-115 witness without losing any member and
SHALL refuse a missing, malformed or misplaced `frame_witness`. FR-115
owns its fields and change variants; this requirement adds none.

`usage` is a closed object with `admission` (a closed object of unsigned
`document_bytes`, `objects` and `values`), unsigned
`evaluation_admissions`, and `evaluation_consumed` (an array of closed
objects each holding string `kind` and unsigned `consumed`).

`limits` is a closed object recording every evaluation limit the run was
bounded by, keyed by the limit's setting name ([FR-461](FR-461-name-the-limit-value-and-setting.md)),
each value the effective bound as a canonical decimal string, with every
default written explicitly: for a run over a model trace,
`expansion.max_candidates`. It is present exactly for a run over a
`model` trace, the one run whose evaluation a request-settable limit bounds,
and omitted for every other run, whose documents are unchanged. Intake ceilings (the request and dependent
byte limits) are not members: they bound reading the request, and a request
that is read under them has one result whatever their values. `usage` says
what the run consumed and `limits` the bounds it ran under; both are part
of the document, so two runs differing only in a limit write unequal
documents, and the run's result identity, which is the document's, differs.
A missing `limits` on a run over a `model` trace, `limits` on any other
run, an unknown setting name, or a value that is not a
canonical positive decimal is refused by the reader.

The two settlement members are:

- `basis` — the [FR-243](../../objects/protocol/FR-243-settlement-basis.md)
  settlement basis label, on every clause-run result, as every other verdict
  carries it. An evaluated `success` or `violation` carries
  `decisive-counterexample`, `decisive-witness` or `closed-scope`; a temporal
  `undefined` result is an evaluated `violation` and carries `closed-scope`
  (Temporal disposition, below). A temporal `pending`, `pending-window` or
  `pending-next-event` result, category `pending`, carries `unsettled`. Every
  other disposition (refused, undefined, indeterminate, incomplete, cancelled,
  unsupported) carries `unavailable`.
- `witness` — the FR-351 separating witness record, present exactly when
  `basis` is `decisive-witness` or `decisive-counterexample`, and absent
  otherwise. It is a closed object with these members:

  | Member | Value |
  | --- | --- |
  | `quantifier` | the deciding quantifier's occurrence key: an object with exactly `node_id` (the FR-322 `NodeId` shape), `role` and `ordinal`; absent for a family whose decision has no quantifier occurrence |
  | `deciding_element` | on a clause-run record, the deciding element in the canonical typed value encoding FR-331 canonical assignments use; on a protocol-analysis record, a JSON string holding the deciding node's canonical node-id text ("Protocol-analysis result") |
  | `index` | the element's zero-based position as an unsigned integer; absent for a family that assigns no position |
  | `value_path` | the [FR-207](../../objects/foundation/FR-207-runtime-value-path.md) `runtime_value` wire form |
  | `trace_position` | the trace position, as a canonical decimal string (FR-351 types it `String`); absent for a family that retains no trace. A clause-run record's families retain none, and a protocol-analysis record always carries one |

  An absent component is an absent member, never `null`, zero or an empty
  value. The document encodes as RFC 8785 JSON.

A reader refuses a `basis` outside the FR-243 labels, a `witness`
present with a basis that is not decisive or absent with one that is, and a
`witness` missing a component its family assigns, carrying a member outside
the table, or substituting a default for an absent component.

### Progress ledger

A clause run over a progressed or timestamped trace returns the progress
ledger that carries its progress assertions to the next run ([FR-094](../temporal/FR-094-interpret-progress-history-and-closure.md)).
The producer SHALL write it as the top-level `ledger` member exactly when the
run's trace carries a progress assertion and the run either accepted that
assertion or was given a ledger: the updated ledger on acceptance, the given
ledger unchanged on a refusal or a resource stop. Every other clause-run record
omits it. `ledger` is a closed object `{"format": "filament.progress-ledger/1",
"entries": [...]}`; each entry is a closed object with `binding` (closed:
`package`, `clause`, `occurrence`, `clock`, `profile`), `watermark`,
`completeness` (`{"complete": {"through": N}}` or `"incomplete"`), `scope` and
`execution` (`"closed"` or `"open"`) and `covered` (an array of closed
`{identity, stamp, fingerprint?}` objects in identity-then-stamp order), entries
in ascending binding order with no binding repeated. A covered entry's
`fingerprint` is the content fingerprint of a record-sample position: the
SHA-256 digest of the sample's canonical RFC 8785 bytes, written `sha256:`
followed by 64 lowercase hexadecimal digits. It is required on a record-sample
entry, whose `identity` is the sample's index, and absent on every other entry.
It is integrity data, not identity: it takes no part in entry order, citation
or keys, and the ledger carries no sample text or value. The strict reader
SHALL restore it without loss and SHALL refuse a `ledger` on a record whose
trace carries no progress, an unknown, missing, duplicate or `null` member, an
out-of-order entry, a repeated binding, a record-sample covered entry without
`fingerprint`, a `fingerprint` on any other covered entry, and a `fingerprint`
not of the `sha256:` form, naming the member. The same bytes are the ledger
a later run request supplies, so a monitor that runs in batches carries its
progress through files.

## Outcome axes

A clause-run document also carries the `axes` object of
[FR-521](FR-521-report-four-independent-outcome-axes.md): its activation,
execution, truth and settlement, with `axes.settlement` equal to the
disposition's `basis`. The `analyze` result of
[FR-520](../tooling/FR-520-carry-analyze-requests-and-results-as-documents.md)
carries temporal dispositions with the kinds and rows this requirement
registers, and no other.

## Temporal disposition

A stage `evaluate` disposition of a temporal selection (FSL FR-330) whose
result is a temporal verdict, a refused `holds` condition or a trace refusal
carries `disposition.temporal`: a closed object in FSL FR-267's encoding
whose `kind` is exactly one of the sixteen rows below. The member is present
exactly for that case. A stage `compile`, `select` or `admit` disposition of a
temporal selection, its internal failure (written with `internal_fault`, as
for every other selection) and every non-temporal selection carry no
`temporal`. A disposition with `temporal` carries no top-level FR-351
`witness`, no `frame_witness`, no `internal_fault` and no `refusal`; its
`outcome`, when present, is in FR-100's `outcome` vocabulary. FSL FR-267
owns the members, the `trace-refused` `variant` spellings and their payloads;
this requirement adds none.

| `kind` | Members beside `kind` | `category` | `truth` | `basis` | `outcome` |
| --- | --- | --- | --- | --- | --- |
| `holds` | none | `success` | `true` | `closed-scope` | absent |
| `violated` | `position`, `counterexample` | `violation` | `false` | `closed-scope` | absent |
| `undefined` | `position`, `counterexample` | `violation` | `false` | `closed-scope` | `undefined` with the undefined value's FR-100 reason |
| `insufficient-data` | `position`, `count`, `min_count` | `undefined` | absent | `unavailable` | `undefined` with reason `insufficient-data` |
| `pending` | none | `pending` | absent | `unsettled` | absent |
| `pending-window` | `position`, `deadline` | `pending` | absent | `unsettled` | absent |
| `pending-next-event` | `position` | `pending` | absent | `unsettled` | absent |
| `indeterminate` | `position`, `reason` | `indeterminate` | absent | `unavailable` | absent |
| `missing-fairness-premise` | `constraint` | `unsupported` | absent | `unavailable` | `refused`, `unsupported_projection`/`missing-fairness-premise` |
| `incomplete` | none | `incomplete` | absent | `unavailable` | FR-100's `incomplete` with its limit |
| `activation-incomplete` | `position`, `guard` | `incomplete` | absent | `unavailable` | absent |
| `window-limit-reached` | `limit`, `value`, `setting` | `incomplete` | absent | `unavailable` | `incomplete` with limit `max_window_values` |
| `progress-limit-reached` | `limit` (`node count`), `value`, `setting` (`progress.covered_positions`) | `incomplete` | absent | `unavailable` | absent |
| `progress-contradicted` | `dimension`, `clock`, `retained`, `asserted` | `refusal` | absent | `unavailable` | FR-100's `refused` form with code `invalid_runtime_input`, cause `invalid-value` and the contradiction's `fields` |
| `condition-refused` | `position` | the record code's category | absent | `unavailable` | FR-100's `refused` form with the record's `code`, `cause`, `fields` and `locus` |
| `trace-refused` | `variant` and the variant's payload | the refusal code's category | absent | `unavailable` | `refused` with the refusal's catalog `code` and `cause` |

The `outcome` vocabulary admits two values beside FR-100's, each in one row
only: the undefined reason `insufficient-data`
([FR-414](../temporal/FR-414-evaluate-window-aggregates.md)'s cause, the
window holding fewer filtered values than `min_count`) only in the
`insufficient-data` row, and the incomplete limit `max_window_values`, which
names a run limit and is not an accounting counter of `usage`, only in the
`window-limit-reached` row. Every other disposition carrying either is
refused: a temporal `undefined` or `incomplete` row, and a disposition of a
non-temporal selection.

The two pending categories and the six finite-window and progress rows
exist because a finite-window or progress-carrying temporal run (FSL FR-501,
FR-502, FR-504) can stop in states that the ten older rows cannot tell
apart, and a consumer acts differently on each:

- `pending`, `pending-window` and `pending-next-event` carry the category
  `pending`, not `inconclusive`: the verdict is not settled yet but will be
  once more of the trace is observed (by the window's `deadline`, a
  watermark tick, or at the next event), which an `inconclusive` analysis
  verdict does not promise.
- `indeterminate` carries the category `indeterminate`: the observed trace
  cannot settle the verdict at `position` for the stated `reason`, a closed
  object whose `kind` is `missing-valuation` (with `position`, `atom` and,
  for an instance-bound atom, `instance` and `also_held_by` together),
  `conflicting-valuation` (with `position` and `atom`), `incomplete-input`,
  `missing-history` or `straddles-bound` (no other member).
- `activation-incomplete` is an `on each` activation whose guard could not be
  established at `position`; `guard` is the guard's checked location in
  FR-100's outcome `location` form. It spends no budget, so it writes no
  `outcome`.
- `progress-limit-reached` is the progress ledger's covered-position capacity
  (`limit` exactly `node count`, `setting` exactly
  `progress.covered_positions`); it is not an accounting counter, so it
  writes no `outcome`.
- `progress-contradicted` is a supplied progress assertion that contradicts
  the retained one: `dimension` is `watermark`, `completeness` or `closure`,
  `clock` the binding's clock name, and `retained` and `asserted` each a
  closed object whose `kind` is `watermark` (with `watermark`), `complete`
  (with `through`), `incomplete`, `decision-scope` or
  `surrounding-execution` (each with `closure`, `closed` or `open`), or
  `position` (with `identity` and `stamp`); the outcome `fields` equal the
  record derived from those members.

`position`, `deadline`, `value`, `count` and `min_count` are unsigned JSON
integers. The categories `pending` and `indeterminate` are admitted only in
their rows, and a kind outside the sixteen is unregistered and refused.

The `counterexample` member is the report's FR-364 observed-arm
`TemporalCounterexample` (`trace` `observed`, `initial_state` and
`over_binding` `null`), whose `trace_position` equals `position`.

The strict reader SHALL refuse, naming the member: a `temporal` at a stage
other than `evaluate` or on a non-temporal selection; a `temporal` together
with a top-level `witness`, a `frame_witness`, an `internal_fault` or a
`refusal`; an unknown `kind` or `variant`; a member the row does not list, or
one it lists absent or `null`; a `category`, `truth`, `basis` or `outcome`
other than the row's; and a `counterexample` that FR-364's replayer refuses
before replay or whose `trace_position` differs from `position`. Writing a
temporal disposition and reading it back SHALL restore an equal disposition,
each `trace-refused` variant and its payload included.

## Protocol-analysis result

A `native-run-result/2` document is either a clause-run record (above) or a
protocol-analysis record. A protocol-analysis record is the result of one
`local-projection` ([FR-174](../protocol/FR-174-project-global-protocols.md))
or `realizability` ([FR-175](../protocol/FR-175-analyze-protocol-realizability.md))
analysis. It has exactly these top-level members: `format`
(`native-run-result/2`), `analysis` and `basis`, plus `witness` under the
condition in "Members". It carries no `disposition`, `usage` or
`extraction`, and a clause-run record carries no `analysis`. A reader tells
the two forms apart by which of `analysis` and `disposition` is present,
never by other payload content.

A producer writes a protocol-analysis record for an analysis that returned a
result: a `proved`, `refuted`, `inconclusive` or `unsupported` result, or a
run that a limit stopped. A request refused before analysis (FR-331
`declined`) and an engine failure (FR-241 execution `failed`) return no
result and get no protocol-analysis record. Their FR-331 terminal record is
the item's only result.

`analysis` is a closed object with exactly these members:

| Member | Value |
| --- | --- |
| `kind` | The [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) kind: `local-projection` or `realizability`. |
| `key` | The derived-artifact key of FR-174 or FR-175 "Derived-artifact identity" for that kind, as a closed JSON object with one member per key member in plain JSON (strings, integers, arrays and objects). It is not the FR-331 canonical typed value encoding. Its `kind` member is `projection` or `realizability`, and FSL FR-572 owns the other members' names and encoding. A digest never stands in for a member, and the key holds no assessed-subject root (FR-351 "Assessed subject's root"). |
| `label` | The [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md) label. |
| `execution` | The [FR-241](../../objects/protocol/FR-241-assessment-execution-disposition.md) execution disposition. |
| `truth` | The [FR-242](../../objects/protocol/FR-242-truth-disposition.md) truth disposition. |
| `settlement` | The item's [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) result entry without its `request_index`, `backend`, `kind` and `counterexample` members. It holds `result` and the members FR-331 "Result entries" requires for that result, spelled as FR-331 spells them: `basis` and `certification` on `proved`, nothing further on `refuted`, `cause` on `inconclusive` and `incomplete`, and exactly one of `cause` and `diagnostic` on `unsupported`. Its `basis` is the FR-331 proof basis (`exhaustive`), a different vocabulary from the record's top-level FR-243 settlement `basis`. |

`label`, `execution`, `truth` and the top-level `basis` SHALL form one of
FR-360's combinations. `settlement.result` SHALL correspond to that
combination: FR-360 `proved` is `proved`, `refuted` is `refuted`,
`inconclusive` is `inconclusive` with a cause FR-331 admits for the
record's `kind`, `unsupported` is `unsupported`, and `failed` with
execution `resource-incomplete` is `incomplete` with
`limit-reached{limit, value, setting}`. A `bound-reached` cause's `depth`
equals the key's `max_depth`. FR-360 `failed` with execution `failed` (an
engine failure) has no protocol-analysis record, so no record carries it.

A `witness` on a protocol-analysis record is the FR-351 record of the
decisive counterexample. It has the protocol family's shape:

- `quantifier` and `index` are absent, because the family has no quantifier
  occurrence and assigns no position.
- `deciding_element` is a JSON string. The string is the RFC 8785 canonical
  text of the FR-322 `NodeId` wire object of the deciding node: the violated
  obligation for realizability, or the divergent action's global node for
  projection. A string that does not read back as a node id and re-encode to
  the same bytes is refused.
- `value_path` is rooted at the request's assessed-subject root, copied
  verbatim, with an empty `steps` array (FR-351 "Assessed subject's root").
- `trace_position` is the canonical decimal string of the zero-based index of
  the step at which the divergence or violation occurs, in the
  counterexample's ordered event sequence. FSL FR-574 says which sequence
  each counterexample form indexes.

The record carries the result's key, settlement and FR-351 record, and not
the projected graph, strategy witness or counterexample body. Those stay on
the analysis result itself (FR-174, FR-175).

The strict reader SHALL refuse with a located wire diagnostic naming the
member: a document with both `analysis` and `disposition`, or with neither;
`analysis` beside `usage` or `extraction`; an unknown, missing, duplicate or
`null` member of `analysis`; a `kind` other than the two; a `key` missing a
member of its kind's key (`invalid_runtime_input`/`missing-member` naming
the member, as FR-174 and FR-175 require of a reader); a `label`,
`execution`, `truth` and `basis` that are not one FR-360 combination; a
`label` `failed` with `execution` `failed`; a `settlement` whose `result`
does not correspond to the label, whose `bound-reached` `depth` differs from
the key's `max_depth`, or that FR-331 "Result entries" refuses when read as
a result entry with the four omitted members disregarded (their absence is
not a defect, and their presence in `settlement` is `unknown-member`); and a protocol-analysis `witness` with a
`quantifier` or `index`, a `deciding_element` that is not the canonical text
of a node id, a `value_path` with a non-empty `steps` array or a root subject
other than `object`, or a `trace_position` that is absent or not a
canonical decimal string. Writing a protocol-analysis record and reading it
back SHALL restore an equal record.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-352-AC-1 | A clause run's result is written as `native-run-result/2` carrying the base members defined here, `basis` and, with a decisive basis, `witness`, and the strict reader admits it. | Test |
| FR-352-AC-2 | A document whose identifier is not exactly `native-run-result/2`, including a document with no identifier, refuses before consumption (FR-014 `unsupported-version`), whatever its payload holds. | Test |
| FR-352-AC-3 | `witness` is present exactly when `basis` is `decisive-witness` or `decisive-counterexample`, and absent otherwise. | Test |
| FR-352-AC-6 | Every `native-run-result/2` clause-run result carries `basis`: `decisive-counterexample` with a `witness` for a state `forall` refuted at an element, `decisive-witness` with a `witness` for an `exists` satisfied at an element, `closed-scope` with no `witness` for a truth decided elsewhere, and `unavailable` with no `witness` for a refused, undefined, incomplete, cancelled or unsupported result. | Test |
| FR-352-AC-7 | A `witness` serializes with exactly the members `quantifier`, `deciding_element`, `index`, `value_path` and `trace_position` its family assigns; a reader refuses an unknown `basis`, a `witness` whose presence disagrees with the basis, a missing assigned member, an extra member, and a `null` or default standing for an absent member. | Test |
| FR-352-AC-8 | A clause-run document with the declared top-level members and the FR-100/FR-109 disposition and usage reads. A refused outcome retains its assigned `code`, `cause`, `fields`, `locus` and `location`; an internal failure carries `internal_fault` with stage and invariant and no outcome. Independently dropping an assigned member, adding an unknown member, or encoding an absent optional member as `null` refuses with a located wire diagnostic. | Test |
| FR-352-AC-9 | A FSL FR-115 Frame violation carries `disposition.refusal.frame_witness`, retaining its complete typed invocation, pre and post selections, population, change and checked permission under FSL FR-267's encoding. A relationship link change is carried as the `link` change, which identifies the link by the relationship's `DeclarationKey`, its `difference` (`added` or `removed`), its `source` and `target` references and the frame's `modifies`, with `population` the source end's; a link that FR-106 check 11.5 leaves out carries no `frame_witness`. The report has `basis: closed-scope` and no top-level FR-351 `witness`. The strict reader restores it, and refuses a missing, malformed or misplaced `frame_witness`, and a `link` change missing a member or with a `difference` other than `added` or `removed`, with a located wire diagnostic. | Test |
| FR-352-AC-10 | Each of the sixteen `disposition.temporal` kinds is written at stage `evaluate` of a temporal selection with exactly its row's members, `category`, `truth`, `basis` and `outcome`, no top-level `witness` and no `refusal`, and reads back to an equal disposition; a `trace-refused` disposition with `variant` `empty-loop` and one with `variant` `tuple-arity`, `expected` 2 and `found` 1 each read back to the same variant and payload. | Test (TC-255, TC-1485) |
| FR-352-AC-11 | A `pending` disposition reads with category `pending`, no `truth` and `basis` `unsettled`; an `undefined` disposition reads with category `violation`, `truth` `false` and `basis` `closed-scope`; an `insufficient-data` disposition reads with `outcome` `{"kind": "undefined", "reason": "insufficient-data"}`; a `window-limit-reached` disposition reads with `outcome` `{"kind": "incomplete", "limit": "max_window_values"}`; the reason `insufficient-data` in an `undefined` disposition, the limit `max_window_values` in an `incomplete` disposition, and either on a non-temporal state clause run each refuse. | Test (TC-255, TC-1485) |
| FR-352-AC-12 | Independently: a `temporal` on a stage `admit` disposition, on a non-temporal clause run, or beside a top-level `witness`, an `internal_fault` or a `refusal`; `kind` `held`; `variant` `empty-hoop`; a `violated` disposition without `counterexample`, with `position` `null`, or with an extra member; a `counterexample` whose `trace_position` differs from `position`; and a `pending` disposition with `basis` `unavailable` or with `truth` each refuse with a located wire diagnostic naming the member. | Test (TC-255) |
| FR-352-AC-13 | A clause run over an extracted Markdown clause body (FSL FR-031) is written with an `extraction` member under FSL FR-267's encoding and reads back with every member equal; a clause run over a unit given as bytes carries no `extraction`. Independently, an `extraction` with an unknown top-level member `source`, one with `map` missing, one with `binding` `null`, and one whose `map.segments` is `[]` each refuse with a located wire diagnostic naming `extraction` and the member. | Test |
| FR-352-AC-14 | A `local-projection` and a `realizability` result are each written as a protocol-analysis record with exactly `format`, `analysis`, `basis` and, with a decisive basis, `witness`, and read back to an equal record: a `proved` result with `closed-scope`; a `refuted` result with `decisive-counterexample` and a `witness` whose `quantifier` and `index` are absent, whose `deciding_element` is the deciding node's canonical node-id text, whose `value_path` root equals the request's `assessed_root` with empty `steps`, and whose `trace_position` is a decimal string; an `inconclusive` result with `unsettled` and `bound-reached{depth}` (`depth` the key's `max_depth`) on each kind, `replay-disagreed` on each kind, and each of `premise-not-stated`, `unforced-refutation`, `strategy-replay-failed`, `cut-count` and `undecided-evaluation` on a `realizability` record; an `unsupported` result with `unavailable`; and a limit-stopped result, FR-360 `failed` with execution `resource-incomplete` and `settlement` `incomplete` `limit-reached`. A declined item and an engine failure write no protocol-analysis record. | Test (TC-255) |
| FR-352-AC-15 | Independently: a protocol-analysis record with `disposition` added; with `analysis` removed; with `usage` added; `kind` `refinement`; a `key` member removed; `label` `proved` with `truth` `pending`; `label` `refuted` with `settlement.result` `inconclusive`; `label` `failed` with `execution` `failed`; a `bound-reached` `depth` other than the key's `max_depth`; `cut-count` on a `local-projection` record; a `witness` with `index` 0; a `deciding_element` that is a node id written with its members reordered; a `value_path` with one `member` step; and `trace_position` `"02"` each refuse with a located wire diagnostic naming the member; the removed key member is refused `invalid_runtime_input`/`missing-member`. | Test (TC-255) |
| FR-352-AC-16 | Each of these temporal dispositions reads at stage `evaluate` with exactly its row's members, `category`, `truth` (absent), `basis` and `outcome`, and reads back equal: `{"kind": "pending-window", "position": 0, "deadline": 5}` (`pending`, `unsettled`); `{"kind": "pending-next-event", "position": 3}` (`pending`, `unsettled`); `indeterminate` at position 0 with each of the five `reason` kinds, the `missing-valuation` reason with `instance` `c1` and `also_held_by` `["c2"]` included (`indeterminate`, `unavailable`); `activation-incomplete` at position 1 with a state-clause `guard` location (`incomplete`, `unavailable`, no `outcome`); `{"kind": "progress-limit-reached", "limit": "node count", "value": 4, "setting": "progress.covered_positions"}` (`incomplete`, `unavailable`, no `outcome`); and a watermark `progress-contradicted` with `retained` `{"kind": "watermark", "watermark": 100}` and `asserted` `{"kind": "watermark", "watermark": 60}` (`refusal`, `unavailable`, outcome code `invalid_runtime_input`, cause `invalid-value`). | Test (TC-1485) |
| FR-352-AC-17 | Independently, each refuses with a located wire diagnostic naming the member: a `temporal` whose `kind` is outside the sixteen (`pending-forever`, `held`); category `pending` on an `incomplete` row and category `indeterminate` on a `violated` row; category `inconclusive` on a `pending` row; a `pending-window` without `deadline`; an `indeterminate` `missing-valuation` reason with `instance` and without `also_held_by`; an `activation-incomplete` or `progress-limit-reached` row with an `outcome`; a `progress-limit-reached` row whose `setting` is not `progress.covered_positions`; and a `progress-contradicted` row whose outcome `fields` differ from its members. | Test (TC-1485) |
| FR-352-AC-18 | A timestamped clause-run record with a `ledger` of one entry reads back equal and re-encodes to the same bytes; the same `ledger` on a record of a lasso run, an entry missing `covered`, two entries for one binding, and entries out of binding order each refuse with a located diagnostic naming the member. | Test (TC-1163) |
| FR-352-AC-19 | A clause-run record over a `model` trace written with no `max_candidates` in the request carries `limits` `{"expansion.max_candidates": "1000000"}`, and one written with `max_candidates` 1 carries `{"expansion.max_candidates": "1"}`; the two records are unequal. A record over a `finite` trace carries no `limits`. The reader refuses a `model`-trace record without `limits`, a `finite`-trace record with `limits`, an unknown setting name, and the values `"0"`, `"01"` and `1` (a JSON number). | Test (TC-1701) |
| FR-352-AC-20 | A clause-run record over a `progressed` record-sample trace carries a `ledger` whose covered entries are each `{identity, stamp, fingerprint}`, `identity` the sample's index and `fingerprint` `sha256:` plus 64 lowercase hexadecimal digits, with no sample text or value; a snapshot-position covered entry carries no `fingerprint`. The reader restores both without loss and refuses a record-sample entry without `fingerprint`, a snapshot-position entry with one, a `fingerprint` with uppercase digits or 63 digits, and a covered entry with any other extra member. | Test (TC-1910) |

## Dependencies

- [FR-014](FR-014-read-exact-wire.md) supplies the closed-wire contract and
  the identifier refusal.
- [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supplies the
  `basis` labels.
- [FR-351](../../objects/protocol/FR-351-separating-witness-record.md) supplies
  the record `witness` carries, and
  [FR-207](../../objects/foundation/FR-207-runtime-value-path.md) its value
  path.
- [US-011](../../usecase/US-011-analyze-complete-models.md) supplies the
  replay/comparability need this wire serves.
- FSL FR-115 owns the evaluated frame witness; FSL FR-267 owns its closed
  serialization inside the clause-run disposition.
- FSL FR-031 owns the meaning of the extraction record of an extracted
  clause run; this requirement fixes its member set and types, and FSL FR-267
  writes and reads it as the top-level `extraction` member.
- [FR-174](../protocol/FR-174-project-global-protocols.md) and
  [FR-175](../protocol/FR-175-analyze-protocol-realizability.md) own the
  analyses a protocol-analysis record reports and their derived-artifact
  keys; FSL FR-572 owns the key's member encoding and FSL FR-574 the
  counterexample's event sequence. [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  supplies the labels and [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  the result entry and its causes.
- FSL FR-330 owns the temporal clause-run report, and FSL FR-267 the closed
  `disposition.temporal` encoding, its members and its `trace-refused`
  variants. [FR-364](../temporal/FR-364-replay-temporal-counterexamples.md)
  owns the observed-arm counterexample it carries.

## References

- Linear STD-144; FSL ADR-031 SW-10, SW-11, R-1 to R-4.
- TC-255 exercises the native run-result wire.
- Linear QSL-1898; FSL ADR-036 PA-24.
