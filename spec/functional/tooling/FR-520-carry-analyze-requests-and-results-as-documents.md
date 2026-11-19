---
id: FR-520
title: "Carry an analyze request and its result as one document pair"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-570
    type: references
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-300
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-352
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-382
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-521
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-620
    type: traces_to
  - target: ix://agent-ix/filament-spec-language/FR-621
    type: traces_to
  - target: ix://agent-ix/filament-spec-language/FR-710
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-286
    type: depends_on
---
# FR-520: Carry an analyze request and its result as one document pair

## Description

When a tool decides claims over every behaviour of a subject from a file, the
tool SHALL read the request as a `native-analyze/1` document. The tool SHALL
write the result as one `filament-outcome/1` document with `operation`
`analyze`. Any conforming tool then reads the same request and writes a result
another tool can read.

## Inputs

A `native-analyze/1` document `{"format": "native-analyze/1", "request": {...}}`.

## Outputs

One `filament-outcome/1` document and its exit status (FR-301).

## Document members

A `native-analyze/1` request document. Its `request` object has these
members:

| Member | Multiplicity | Meaning |
| --- | --- | --- |
| `program` | 0..1 | The program source selection `{source: {file, authority, identity, document, formal_revision}}`, the source selection the native run request uses (FR-323): `file` is resolved against the request document's directory, and `authority` and `identity` are the source labels the checked package records. Required unless `package` is present. |
| `package` | 0..1 | A selection `{directory}` naming the checked-package handoff directory (FSL FR-517) that holds an existing `filament.checked-package/v2` artifact (FR-322), resolved against the request document's directory. The analysis admits the directory's `checked-package.json` with the checked-package reader, with no source compiled, and attaches the identity rows of its required `identities.json` and the equality rows of its optional `equality.json`, each read by its strict reader; those rows are the identity witness a package read from its bytes needs, and they come from those files only, never from the package bytes (FSL FR-531). A directory, `checked-package.json` or `identities.json` that is absent refuses the request `invalid_package`, `missing-file`, naming the path; an identity file its reader refuses refuses `invalid_package` with `malformed-file`, `unknown-format`, `unjoined-row` or `native-witness`, and a present equality file its reader refuses with `refused-file`, each naming the file (FSL FR-850). |
| `models` | 0..* | Domain model selections `{format, source}`, each `source` a source selection as for `program`, as the native run request carries them (FR-323): `format` `semantic-ir/2.0.0` selects a domain package document, and `native-model-unit/1` an FSL model unit that FSL compiles to its domain package document before admission (FSL FR-710); any other format refuses `invalid-request`, exit `20`, naming `models` and the format. |
| `libraries` | 0..* | `{identity, source, refinement_internals?, integer_division?}` library selections: `identity` is the library's package identity, `source` a source selection as for `program`, `refinement_internals` that library's refinement internals (below), and `integer_division` that library's integer-division law name (below), as the native run request's dependency entries carry them (FR-323 `DependencyEntry`). |
| `refinement_internals` | 0..1 | The program's own refinement internals: one JSON value naming, for each refinement the program declares, the internal names its abstract side reads (FR-375), handed to compilation unchanged. Absent means the empty list. The same member on a `libraries` entry names that library's refinements only. |
| `integer_division` | 0..1 | The program's integer-division law name, `euclidean`, `floor` or `truncating`, as the native run request carries it (FR-301, FR-570): it selects the law for every program unit's `div` and `rem`, composes with a header law as FR-570 states, and never reaches a library, which takes only its own `integer_division`. An unknown name refuses `unknown_profile` `unsupported-selection` before any source is read; a member beside `package` refuses `invalid-request`. |
| `documents` | 0..* | `{file}` or `{file, identity}` selections of supplied documents: initial-state snapshots (`filament.state.snapshot/v1`), supplied finite protocol traces (`trace`, below) and FR-737 protocol recordings (`recording`, below). Each is provisioned under the `{authority, identity}` it declares for itself; a stated `identity` must equal it, and binds a recording that declares none (FR-821). A second document under one identity refuses `invalid-request`. |
| `memory` | 0..* | FR-331's `request.memory` weak-memory selections, one per weak `parallel` of the subject (FR-439). |
| `boundary` | 0..1 | The run's observe boundary, `model-state` or `observe-only` ([FR-822](../protocol/FR-822-declare-observe-only-protocol-boundaries-and-typed-recommendations.md) "Run-wide observe mode"). Absent means `model-state`: FR-182's run-wide boundary alone. `observe-only` treats every role and the whole `run` control of each protocol the request steps as observe-only and refuses the run at each offending action before any step. Another value refuses `invalid_runtime_input`/`invalid-value` at `/request/boundary`. The member is part of the request identity. |
| `subject` | 0..1 | The default model subject `{initial_states, universes, proof_bounds}` in FR-331's `temporal.subject` spelling; each reference names a provisioned document by `{authority, identity}`. It, and an item's `temporal.subject`, may carry `protocol`, a protocol binding `{protocol, over, roles, capacities}` in FR-433's `protocol-subject` spelling without `type`: the subject is then the protocol subject of that clause over the model subject (FR-425), its outermost `parallel` controls resolved by `memory` (FR-434). `protocol` names a protocol clause by qualified name; `over` is required and binds the clause's `over` parameter to exactly one object reference of its population universe (FR-425, FR-433); `roles` binds each static role on a model type to `{"type": "model-binding", "selection": <identity>}`; `capacities` is FR-785's capacity binding. The binding is closed, and the request writer carries it into the item's FR-331 `temporal.subject` as the member `protocol`. |
| `items` | 0..* | The analysis items below. |
| `limits` | 0..1 | `{accounting, observation, normalization}`, each optional: `accounting` is any subset of the ten `filament.value.accounting/v1` counters, each a JSON integer from 0 to 2^53 - 1, an omitted counter taking its published default (FR-461); `observation` and `normalization` are the observation and model-normalization limits FR-461 names, each with its published default. |
| `deadline_ms` | 0..1 | The run's deadline in milliseconds, a JSON integer from 0 to 2^53 - 1 (9007199254740991), the largest integer every JSON reader holds exactly. Absent means no deadline. |

An analysis item has these members:

| Member | Multiplicity | Meaning |
| --- | --- | --- |
| `claim` | 0..1 | The declared qualified name of the checked declaration the item decides: a temporal or state clause, a state-graph claim, a hyper clause, a refinement declaration, a schedulability or response claim, a probabilistic claim, a protocol clause, or, for a derived item, the declaration the derived item anchors at (FR-368 "Claim and anchor of a derived or refinement item"). Absent only for a `simulation` item. |
| `kind` | 0..1 | The FR-290 kind, or `simulation` for a simulation item (below). Absent means the one kind FR-290 admits for the claim's declaration family; absent on a family that admits several refuses the item `invalid-request` `absent-kind`. |
| `backend` | 0..1 | A `BackendId` the caller names. |
| `required` | 0..1 | Boolean, default `true`: whether the result's `aggregate` counts this item, so that complete aggregate success needs it settled `proved` or `measured`. |
| `temporal`, `probabilistic`, `hybrid`, `check`, `options`, `reductions`, `correspondence`, `premises`, `assessed_root` | 0..1 each | FR-331's members, with its spellings and its rule for which kinds carry each. `temporal.item_kind` is `formula` (the default), `deadlock-freedom`, `time-lock-freedom`, `race-freedom` or `refinement`, as FR-331 places it. `temporal.subject` is optional and defaults to the request's `subject`. `reductions` names a constraint by its declared name, which the reader writes as FR-382's node key. |
| `trace` | 0..1 | For a `finite-replay` item only, the `{authority, identity}` of a provisioned document holding a finite protocol trace in the protocol counterexample's trace spelling (FR-433). The trace is replayed step by step through the protocol system and the clause is evaluated on it; a step that is stale or not enabled declines the item `invalid-input` naming the step's position. An item of another kind that carries `trace`, or a `trace` naming no provisioned document, refuses `invalid-request`. |
| `recording` | 0..1 | For a `global-conformance` item only, the `{authority, identity}` of a provisioned FR-737 protocol recording, assessed in replay mode (FR-059); its binding and refusals are FR-821's. |
| `simulation` | 0..1 | For a `simulation` item only: `{mode, seed, samples, max_steps, max_states, max_state_key_bytes}`. `mode` is `enumerate` or `sample`; `seed` (a 64-bit unsigned integer) and `samples` (a positive count) belong to `sample` only; each bound is a positive count with FR-181's published default. A simulation item settles no claim and is not negotiated; its result carries traces, coverage and the unexplored frontier in FR-181's exploration contract. |

## Behavior

- **Request members.** The request carries the program or checked-package
  selection, domain packages, libraries and refinement internals in the
  native run request's spelling; supplied `documents` (snapshots and finite
  protocol traces) provisioned under their own identities; `memory`
  selections, a default `subject`, `items`, shared `limits` and an optional
  `deadline_ms`, spelled as the tables in "Document members" state.
- **Items.** An item carries the request-item members of FR-331 (`kind`,
  `temporal` with `item_kind`, `probabilistic`, `hybrid`, `check`, `options`,
  `reductions`, `correspondence`, `premises`, `assessed_root`, `backend`) with
  FR-331's spellings and rules, names its claim by declared name, and never
  carries `request_index`, `extent` or `candidates`, which the reading tool
  computes from the checked package and its registry. A `finite-replay` item
  names a supplied trace in `trace`; a `simulation` item carries its bounds in
  `simulation` and settles no claim; its result reports the invariant
  violations its traces meet as findings (FR-181), never as a settlement.
- **Reductions.** An item selects reductions in FR-382's `reductions` member,
  over what the unit declares (`symmetric` populations, `constraint`
  clauses); a selection the preservation table does not admit settles that
  item `inconclusive` with `reduction-not-preserving` (FR-385).
- **Result items.** Each result item carries its request index, the declared
  claim name, the admitted FR-331 request item with every default explicit,
  its category, its FR-331 terminal record, the certificate identity and
  checker verdict when there is one, the FR-331 counterexample and replay
  target for a refuted record, its FR-331 accounting record, its exploration counts (states, transitions and
  automaton states) for an item whose engine explores states, and the four
  outcome axes of FR-521 when the item has an activation scope. A
  `finite-replay` item also carries its FR-352 `disposition`, and a
  `global-conformance` item its per-obligation `obligations`, its `closure`,
  its `global` verdict, its `projections` and, when it assessed one, its
  `recording` identity (FSL FR-990, FR-1032). Its record also carries
  `settled_by`, `backend` exactly when a result settled it, and, where the
  run states them, `non_zeno`, `memory_bound` (FR-439) and, for a replayed
  trace, `trace` and `steps`. A `simulation` item carries, in place of a
  record, its admitted `simulation` settings, its traces, `coverage` and
  `frontier` (FSL FR-634, FR-181). The result also carries the root
  `aggregate` (`status` and `gaps`) and `registry` (FSL FR-621, FR-635).
- **Kinds.** Every category label in the result is one FR-301 lists, and
  every temporal disposition is FR-352's `disposition.temporal` with one of
  its sixteen kinds and exactly that row's `category`, `truth`, `basis` and
  `outcome` (FR-352-AC-10, AC-11, AC-16, AC-17); a writer never emits another
  kind. An item's `axes.settlement` equals its disposition's `basis`.
- **Values.** Every value in a counterexample, assignment, witness or
  simulation trace uses FR-181's typed canonical forms; the result has no
  value spelling of its own.
- **Engine roles.** An item settled by the bounded Kani engine is `refuted`
  or `incomplete` at its bound and never `proved`.
- **Several items.** `items` is ordered; each entry gives exactly one result
  item in that order, and one entry's refusal settles only that entry, as a
  multi-item run does.
- **Equivalence.** The same request through the library and through the CLI
  gives byte-equal documents (FR-300).
- **Schema.** The result SHALL validate against the published JSON Schema
  of `filament-outcome/1`, `$id` `urn:filament:outcome:1`.
- **Canonical bytes.** The tool SHALL write the result in its RFC 8785 (JCS)
  canonical encoding.

## Result schema

The analyze result is a `filament-outcome/1` document, so its schema is the
one published for that format: `urn:filament:outcome:1` (JSON Schema draft
2020-12), which the language implementation publishes with the outcome
document it describes (FSL FR-286 "Published schema"). That schema's
`analyze` branch, selected by `operation` `analyze`, requires the
`aggregate` and `registry` members and describes each result item, the
`simulation` item included, with every member "Result items" lists, in
FR-331's spellings. It closes every object whose member names the format
fixes, and describes a map keyed by names the program declares with
`additionalProperties`. The language implementation's completeness test
fails when a writer can emit a member the schema does not declare (FSL
FR-286-AC-31), so the schema and the writers cannot drift apart. A second analyze-only schema is not published: it would
describe the same bytes twice and could drift from the first. A tool that
writes an analyze result conforms only if every result it writes validates
against `urn:filament:outcome:1`; a tool that reads one may validate it
before reading.

The result's bytes are its RFC 8785 canonical encoding: at every depth an
object's members are in ascending order of their names' UTF-16 code units.
The member order of the request carries no meaning wherever the result
does not depend on it: an admitted request and any permutation of its object
members give byte-equal results, and so does a refusal whose named defect
does not depend on request order. Two refusals depend on it by their own
rule and legitimately change with a permutation: one that names the first
defect in request order where its requirement says so (FSL FR-1031
refuses a `refinement_transfer` request at its first defective member in
request order), and one that keeps several defects in the order the reader met
them. The key order is UTF-16 code-unit order, which differs from UTF-8
byte order for a key holding a supplementary character against one holding
a character from U+E000 to U+FFFF.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-520-AC-1 | A `native-analyze/1` document with one item naming a holding temporal clause gives one result item whose `request` is a valid FR-331 request item with `request_index` 0 and explicit limits, whose record is `proved`, and whose `accounting` is a valid FR-331 accounting record. | Test (TC-1080) |
| FR-520-AC-2 | An item carrying `request_index`, `extent` or `candidates` refuses the whole request with `invalid-request`, exit `20`. | Test (TC-1080) |
| FR-520-AC-3 | A refuted item carries an FR-331 counterexample entry and a replay target, and every category and record label in the result is registered by FR-352. | Test (TC-1080) |
| FR-520-AC-4 | An item selecting partial-order reduction for a bounded MLTL claim settles `inconclusive` `reduction-not-preserving` while the request's other items settle as they do alone. | Test (TC-1080) |
| FR-520-AC-5 | A `native-analyze/1` request whose program declares `a div 2` with no law header and carries `"integer_division": "floor"` compiles its program and settles its items; without the member it refuses the whole request `missing_declaration`/`missing-selection` at the `div` with no item settled; with `"integer_division": "ceiling"` it refuses `unknown_profile`/`unsupported-selection` before any source is read. | Test (TC-1654) |
| FR-520-AC-6 | A request whose `subject` carries `protocol` `{"protocol": "SB", "over": O, "roles": {"w": B, "c": B}}` (`O` the `core` object reference, `B` a `model-binding`) and whose `memory` selects `SB::Both` as `tso` gives an item whose FR-331 request item carries `temporal.subject.protocol` with its `over` binding and the effective model; the binding with an unknown member refuses the whole request `invalid-request` naming it; the same claim with no `protocol` member is a model-subject item. | Test (TC-2005) |
| FR-520-AC-7 | A `native-analyze/1` request whose `package` is `{"directory": "handoff"}`, naming a handoff directory holding `checked-package.json` and `identities.json`, is admitted; one whose `package` is `{"file": "package.json"}` refuses `invalid-request` naming `file`; the directory with `identities.json` removed refuses `invalid_package`, `missing-file`, naming `identities.json`; with `identities.json` replaced by `{}` it refuses `invalid_package`, `malformed-file`; with `equality.json` removed it is admitted, and with `equality.json` replaced by `not json` it refuses `invalid_package`, `refused-file`. | Test (TC-2009) |
| FR-520-AC-8 | FR-520-AC-1's result validates against `urn:filament:outcome:1` and equals its own RFC 8785 re-encoding; the same result with `/items/0/request_index` removed fails validation at `/items/0`, with `/items/0/category` set to `"proven"` fails at `/items/0/category`, and with `aggregate` removed fails at the root pointer; FR-520-AC-1's request, and FR-520-AC-2's request whose item carries only `request_index`, each written with the members of every object reversed, give byte-equal results; the result of an FSL FR-990-AC-1 `global-conformance` item (with `obligations`, `closure` and `global`), of an FSL FR-990-AC-3 item with `recording`, and of an FSL FR-634-AC-1 `simulation` request each validate. | Test (TC-2096) |

## Dependencies

- FR-331 owns the item, record, counterexample and accounting spellings;
  FR-352 owns the kind registry; FR-382 owns `reductions`.
- FSL FR-286 publishes the `filament-outcome/1` schema
  `urn:filament:outcome:1` the result validates against; RFC 8785 fixes its
  canonical bytes.
