---
id: FR-521
title: "Report activation, execution, truth and settlement as four independent outcome axes"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-352
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-622
    type: traces_to
---
# FR-521: Report activation, execution, truth and settlement as four independent outcome axes

## Description

When a temporal outcome is written, the producer SHALL write its activation
state, execution disposition, truth value and settlement as the four members
of one `axes` object beside the outcome's single category: on every
native-run-result/2 clause-run document, every `monitor` item and every
`analyze` item that has an activation scope.

## Inputs

A temporal outcome's four derived values.

## Outputs

`{"activation", "execution", "truth", "settlement"}`.

## Behavior

- `activation`: `inactive`, `unknown`, `active` or `unavailable`; never
  `null`.
- `execution`: `completed`, `failed`, or `null` when no execution ran.
- `truth`: the string `true`, `false` or `pending`, or `null`.
- `settlement`: `closed-scope`, `decisive-witness`,
  `decisive-counterexample`, `unsettled`, `unavailable` or `null`.
- The category and its exit status are unchanged by the axes; one input
  that decides one axis changes only that axis.
- A function run outcome and a non-temporal analyze item (value validity,
  operation contract, schedulability, probabilistic, simulation) carry no
  `axes` member, because they have no activation scope.
- For an `analyze` item, the axes follow from the terminal record: `proved`
  gives `active`, `completed`, `true`, `closed-scope`; `refuted` gives
  `active`, `completed`, `false`, `decisive-counterexample`;
  `inconclusive` gives `active`, `completed`, `null`, `unsettled`;
  `incomplete` gives `active`, `failed`, `null`, `unsettled`; any refusal or
  disposition record gives `unavailable`, `null`, `null`, `unavailable`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-521-AC-1 | A clause-run document carries `axes` with the four spellings above, and two runs that differ only in scope closure differ only in `settlement`. | Test (TC-1081) |
| FR-521-AC-2 | A function run outcome carries no `axes`; analyze items settling `proved`, `refuted`, `incomplete` and `unsupported` carry the axes the mapping gives. | Test (TC-1081) |

## Dependencies

- FR-352 owns the clause-run document that carries the member.
