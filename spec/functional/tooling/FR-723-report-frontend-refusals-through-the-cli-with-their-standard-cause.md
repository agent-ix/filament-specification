---
id: FR-723
title: "Report frontend refusals through the CLI with their standard cause"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-271
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-272
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-407
    type: references
  - target: ix://agent-ix/filament-specification/FR-252
    type: references
---

# FR-723: Report frontend refusals through the CLI with their standard cause

## Description

When a CLI verb reports a refusal that the language frontend produced, the
CLI SHALL report it with the frontend's code, cause variant, payload and
locus unchanged, and SHALL NOT refuse a unit that the frontend admits. Every
cause in the CLI's output SHALL be a variant of FR-272's closed set for its
code. A CLI does not add, rename or substitute causes of its own: a reader of
the outcome document must be able to act on the cause from the standard
alone, and a cause only one tool knows cannot be read that way.

## Inputs

A CLI invocation of a verb that runs the frontend (`check`, `compile`,
`analyze`, FR-301) on a unit.

## Outputs

The verb's outcome document or diagnostic line (FR-301, FR-522).

## Behavior

- The CLI SHALL pass the frontend's diagnostics to its outcome document in
  the frontend's order, each with its FR-271 code, its FR-272 cause variant
  and payload, and its source locus.
- The CLI SHALL NOT refuse a construct the frontend admits. In particular, a
  probabilistic claim under the timed profile over a subject whose model
  declares a time binding is admitted (FR-407-AC-11), and the verb continues
  with it.
- A probabilistic claim under the timed profile over a subject whose model
  declares no time binding is refused by the frontend with
  `invalid_timed_model`/`model-time-on-untimed-model` naming the model, at
  the profile alias (FR-252); the CLI reports that refusal and no other.
- A cause outside FR-272's closed set for its code in a CLI outcome document
  is a defect of the CLI, which a conforming reader refuses as FR-272 states.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-723-AC-1 | `compile` of a unit whose probabilistic claim uses the timed profile over a timed subject, with the event `eventually[0 ms, 4 ms] holds(P)`, exits `0` with no diagnostic. | Driver test (TC-1765): the CLI binary exists only in the driver repository, so the driver code ticket adds the test there. |
| FR-723-AC-2 | `compile` of the same claim over a subject whose model declares no time binding exits with the `invalid_timed_model` category status and reports exactly one diagnostic, `invalid_timed_model`/`model-time-on-untimed-model` naming the model at the profile alias; no diagnostic carries a cause outside FR-272's closed set, and none names a timed event profile as the cause. | Driver test (TC-1765) |
| FR-723-AC-3 | For each frontend refusal of TC-1765's corpus, the CLI outcome document carries the same code, cause variant, payload and locus as the frontend's library result for the same unit. | Driver test (TC-1765) |

## Dependencies

- FR-301 owns the verbs and exit contract; FR-271 and FR-272 own codes and
  causes; FR-407 and FR-252 own the timed-profile admission and refusal.
