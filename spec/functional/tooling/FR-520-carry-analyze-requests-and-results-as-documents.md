---
id: FR-520
title: "Carry an analyze request and its result as one document pair"
type: FR
relationships:
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

When a tool decides claims over every behaviour of a subject from a file, it
SHALL read the request as a `native-analyze/1` document and write the result
as one `filament-outcome/1` document with `operation` `analyze`, so that any
conforming tool reads the same request and writes a result another tool can
read.

## Inputs

A `native-analyze/1` document `{"format": "native-analyze/1", "request": {...}}`.

## Outputs

One `filament-outcome/1` document and its exit status (FR-301).

## Behavior

- **Request members.** The request carries the program or checked-package
  selection, domain packages, libraries and refinement internals in the
  native run request's spelling; supplied `documents` (snapshots and finite
  protocol traces) provisioned under their own identities; `memory`
  selections, a default `subject`, `items`, shared `limits` and an optional
  `deadline_ms`. FSL FR-620 states each member's spelling and defaults.
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

## Dependencies

- FR-331 owns the item, record, counterexample and accounting spellings;
  FR-352 owns the kind registry; FR-382 owns `reductions`.
