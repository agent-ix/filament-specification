---
id: FR-730
title: "Bind the recovery view at the end of a compensation thread"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-427
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-466
    type: references
  - target: ix://agent-ix/filament-specification/FR-035
    type: references
---

# FR-730: Bind the recovery view at the end of a compensation thread

## Description

When a compensation thread ends (`cend`, FR-427) and its template's recovery
predicate reads the template's recovery binder (`recover (r: M::Recovery)`,
FR-466 "Compensation"), the protocol evaluator SHALL bind the binder to a
*recovery record* of the binder's record type and evaluate the predicate over
that record and the registration and activation captures. In exploration,
`cend` SHALL have one successor for each record of the recovery type's finite
domain, and each successor carries the status the predicate gives for its
record: `recovered` when the predicate is true and `unrecovered` when it is
false. A predicate that does not read the binder binds no record, so `cend`
has one successor.

When the evaluator checks a supplied or observed execution, the recovery
record is the one that execution reports for the registration (FR-035). A
recovery that the execution requires and that is not observed stays
incomplete.

### Decision: the recovery view is what the environment reports

A recovery record describes what the compensation achieved, such as the
balance after a refund. The protocol's model does not compute it. The record
comes from the system that ran the compensation. Exploration therefore treats
it as an environment choice, in the same way an `event` binds any record that
satisfies its constraint: every record is possible, so a claim about recovery
holds only when it holds for every report the type allows. Deriving the
record from model state would be wrong, because the model holds no
compensation result. Refusing the predicate would remove the capability that
the `recover` binder exists to provide. The recovery type is a finite record
type, as the checker requires of every step binder, so the branching is
bounded.

## Behavior

- The binder's record is bound only at `cend`. No other stage reads it
  (FR-464 stage table), and it does not outlive the thread.
- The `cend` successors are a fan-out of records, in the record type's
  canonical value order. They form one step class with one transition per
  record, and each transition's identity includes the record.
- An undefined predicate value over a record settles by the
  undefined-protocol-expression rule (FR-426), as FR-427-AC-5 states.
- A counterexample that passes through `cend` carries the bound record, so
  replay (FR-433) reproduces the same status.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-730-AC-1 | Over TC-380's `Pay` with `recover (r: M::Recovery) { r.bal = 0 }`, where `bal` is `Int[0, 1]`, `cend` has two successors: `recovered` with `r.bal` 0 and `unrecovered` with `r.bal` 1. A claim that every finished instance is `recovered` is refuted, and its counterexample carries `r.bal` 1. | Test (TC-1774) |
| FR-730-AC-2 | With the predicate `triggered`, which does not read `r`, `cend` has one successor and the claim's verdict equals the one before the binder existed. | Test (TC-1774) |
| FR-730-AC-3 | Replaying the AC-1 counterexample reproduces `unrecovered`. The same counterexample with its recovery record changed to `bal` 0 does not reproduce. | Test (TC-1774) |
| FR-730-AC-4 | Checked against a supplied execution whose recovery observation for the registration reports `bal` 0, the registration is `recovered`. With the observation absent, the status stays incomplete and is never `unrecovered`. | Test (TC-1774) |

## Dependencies

- [FR-427](FR-427-check-protocol-compensation.md): compensation steps and
  `cend`.
- [FR-426](FR-426-apply-protocol-steps.md): step identity and the undefined
  rule.
- [FR-466](../../objects/interfaces/FR-466-checked-package-protocol-control-bodies.md):
  the `recovery` Group of a template.
- [FR-035](../foundation/FR-035-bind-ecosystem-subjects.md): recovery
  observations.
