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
| `package` | 0..1 | A selection `{file}` of an existing `filament.checked-package/v2` artifact (FR-322), admitted by the checked-package reader with no source compiled, as the native run request selects one (FR-323). |
| `models` | 0..* | `semantic-ir/2.0.0` domain package selections `{format, source}`, each `source` a source selection as for `program`, as the native run request carries them (FR-323). |
| `libraries` | 0..* | `{identity, source, refinement_internals?, integer_division?}` library selections: `identity` is the library's package identity, `source` a source selection as for `program`, `refinement_internals` that library's refinement internals (below), and `integer_division` that library's integer-division law name (below), as the native run request's dependency entries carry them (FR-323 `DependencyEntry`). |
| `refinement_internals` | 0..1 | The program's own refinement internals: one JSON value naming, for each refinement the program declares, the internal names its abstract side reads (FR-375), handed to compilation unchanged. Absent means the empty list. The same member on a `libraries` entry names that library's refinements only. |
| `integer_division` | 0..1 | The program's integer-division law name, `euclidean`, `floor` or `truncating`, as the native run request carries it (FR-301, FR-570): it selects the law for every program unit's `div` and `rem`, composes with a header law as FR-570 states, and never reaches a library, which takes only its own `integer_division`. An unknown name refuses `unknown_profile` `unsupported-selection` before any source is read; a member beside `package` refuses `invalid-request`. |
| `documents` | 0..* | `{file}` or `{file, identity}` selections of supplied documents: initial-state snapshots (`filament.state.snapshot/v1`), supplied finite protocol traces (`trace`, below) and FR-737 protocol recordings (`recording`, below). Each is provisioned under the `{authority, identity}` it declares for itself; a stated `identity` must equal it, and binds a recording that declares none (FR-821). A second document under one identity refuses `invalid-request`. |
| `memory` | 0..* | FR-331's `request.memory` weak-memory selections, one per weak `parallel` of the subject (FR-439). |
| `subject` | 0..1 | The default model subject `{initial_states, universes, proof_bounds}` in FR-331's `temporal.subject` spelling; each reference names a provisioned document by `{authority, identity}`. |
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
  `simulation` and settles no claim.
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
  outcome axes of FR-521 when the item has an activation scope.
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

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-520-AC-1 | A `native-analyze/1` document with one item naming a holding temporal clause gives one result item whose `request` is a valid FR-331 request item with `request_index` 0 and explicit limits, whose record is `proved`, and whose `accounting` is a valid FR-331 accounting record. | Test (TC-1080) |
| FR-520-AC-2 | An item carrying `request_index`, `extent` or `candidates` refuses the whole request with `invalid-request`, exit `20`. | Test (TC-1080) |
| FR-520-AC-3 | A refuted item carries an FR-331 counterexample entry and a replay target, and every category and record label in the result is registered by FR-352. | Test (TC-1080) |
| FR-520-AC-4 | An item selecting partial-order reduction for a bounded MLTL claim settles `inconclusive` `reduction-not-preserving` while the request's other items settle as they do alone. | Test (TC-1080) |
| FR-520-AC-5 | A `native-analyze/1` request whose program declares `a div 2` with no law header and carries `"integer_division": "floor"` compiles its program and settles its items; without the member it refuses the whole request `missing_declaration`/`missing-selection` at the `div` with no item settled; with `"integer_division": "ceiling"` it refuses `unknown_profile`/`unsupported-selection` before any source is read. | Test (TC-1654) |

## Dependencies

- FR-331 owns the item, record, counterexample and accounting spellings;
  FR-352 owns the kind registry; FR-382 owns `reductions`.
