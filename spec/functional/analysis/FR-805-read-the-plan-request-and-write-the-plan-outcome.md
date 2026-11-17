---
id: FR-805
title: "Read the plan request and write the plan outcome document"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-650
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-323
    type: references
  - target: ix://agent-ix/filament-specification/FR-272
    type: references
---

# FR-805: Read the plan request and write the plan outcome document

## Description

When a caller asks for a plan, the planner SHALL read one plan request with
a strict reader, refuse a malformed one before any search, and write one
`filament.plan-outcome/v1` document whose members carry the result, the plan,
its replay, the certificate, the explored counts and the limits, and whose
status maps to exactly one FR-301 outcome category and exit code.

## Inputs

A plan request (below) and the checked package it names.

## Outputs

One `filament.plan-outcome/v1` document, or an `invalid_runtime_input`
refusal naming the first defect by its RFC 6901 pointer.

## Behavior

### Request

```json
{
  "format": "filament.plan-request/v1",
  "package": "<checked package identity>",
  "declaration": "Reach",
  "initial": {"format": "filament.state.snapshot/v1"},
  "policy": ["NotFour"],
  "limits": {"max_states": 100000, "max_work": 1000000},
  "accounting": {"work_units": 1000000},
  "deadline_ms": 60000
}
```

- `declaration` names a `synthesis` declaration of the package whose
  `grammar` is a state model object type; `initial` is the initial-state
  snapshot; `policy` (optional, default empty) the ordered policy clause
  names (FR-803), each at most once; `limits` (optional) any subset of
  FR-801's planner limits; `accounting` (optional) any subset of the
  `ScalarLimitsV1` counters; `deadline_ms` (optional) the wall-clock
  deadline; `dwell_grid` (required for, and only admitted on, a hybrid plan,
  FR-819) the exact positive dwell grid.
- A host request surface (a run request of the native runtime, FR-323) MAY
  carry the same members under its own envelope; their meaning is this
  requirement's.
- Admission refuses `invalid_runtime_input` at the first defect: not JSON or
  not an object (`malformed-json`); a missing, unknown or duplicate member
  (`missing-member`, `unknown-member`, `duplicate-member`); a member of the
  wrong JSON kind (`wrong-value-kind`); a duplicate policy name, a limit of
  zero or a negative deadline (`invalid-value`). An unknown limit name
  refuses as FR-461-AC-4 states.

### Outcome document

```json
{
  "format": "filament.plan-outcome/v1",
  "package": "<checked package identity>",
  "declaration": "Reach",
  "status": "optimal",
  "objective": {"direction": "minimize", "expression": "length", "value": 3},
  "plan": {
    "length": 3,
    "steps": [ {"index": 0, "operation": "inc", "arguments": []} ],
    "states": [ {} ]
  },
  "replay": {"outcome": "reproduced", "steps": 3},
  "certificate": {"type": "potential", "entries": 9, "outcome": "accepted"},
  "certification": "certified",
  "bound": null,
  "explored": {"states": 12, "work": 57, "nan_objectives": 0, "undefined_objectives": 0},
  "limits": {"plan_search.max_states": 100000},
  "cause": null
}
```

- `status` is FR-650's `optimal`, `feasible`, `infeasible` or `incomplete`,
  or `unsupported` when no engine can decide the plan (a hybrid plan with no
  hybrid candidate, FR-819).
- `objective.value` and every argument and state value are written in the
  native runtime's run value form (FR-323); an exact rational or decimal is
  never written as a JSON float.
- `plan.steps[i]` carries `index`, `operation`, `arguments` as
  `{parameter, value}` pairs in parameter order and, for a timed model,
  `delay`; `plan.states` holds the `length + 1` states from the initial
  state, each as a `filament.state.snapshot/v1` object world.
- `replay` records FR-650's native replay: `reproduced` with the step
  count, the only value a reported plan carries.
- `certificate` carries `type` (`potential` or `closure`), `entries`, and
  `outcome` `accepted` or `rejected`; a rejected certificate adds `rule` and
  `entry`, the first failing rule and the entry it failed at.
- `bound` is present on `feasible`: `{side: lower|upper, value, attained}`,
  or `null` when no certified bound exists.
- `explored` carries the FR-801 counts and the FR-802 exclusion counts.
- `limits` lists every limit in force by setting name, with defaults filled.
- `cause` is present on `incomplete` (FR-801's `limit-reached` or
  `cancelled`, or FR-819's `grid-exhausted{dwell_grid}` or `timed-out`) and
  on `unsupported` (FR-193's `unsupported_projection`/
  `unsupported-requested-capability`), and `null` otherwise.
- `plan` is `null` on `infeasible`, on `unsupported` and on an `incomplete`
  result that found no admissible plan.

### Categories and exits

| Status | FR-301 category | Exit |
| --- | --- | --- |
| `optimal` | success | `0` |
| `feasible` | success | `0` |
| `infeasible` | violation | `10` |
| `incomplete` | incomplete | `22` |
| `unsupported` | unsupported | `21` |
| refusal (FR-806) | refusal | `20` |
| method failure (FR-650 replay failure) | internal-failure | `30` |

`infeasible` is a violation: the goal cannot be met within the stated bound,
which is a definite negative answer and is never success.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-805-AC-1 | FR-650-AC-1's request writes a `filament.plan-outcome/v1` document with `status` `optimal`, `objective.value` 3, three steps with their arguments, four states, `replay.outcome` `reproduced`, certificate `potential` `accepted`, `certification` `certified`, and exits `0`. | Test (TC-1955) |
| FR-805-AC-2 | An `infeasible` result writes `plan` `null`, a `closure` certificate and exits `10`; an `incomplete` result writes its `cause` and exits `22`. | Test (TC-1955) |
| FR-805-AC-3 | A `Decimal` objective `0.30` is written in the run value form, never as the JSON number `0.3`. | Test (TC-1955) |
| FR-805-AC-4 | A request with an unknown member, a duplicate policy name, a `policy` given as a string and a limit of 0 each refuse `invalid_runtime_input` (`unknown-member`, `invalid-value`, `wrong-value-kind`, `invalid-value`) at the member's pointer, exit `20`, before any search. | Test (TC-1955) |
| FR-805-AC-5 | A `feasible` result whose certificate was rejected carries `certificate.outcome` `rejected` with `rule` and `entry`, and `bound` `null` when no certified bound exists. | Test (TC-1955) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the result
  kinds; [FR-801](FR-801-bound-and-meter-the-plan-search.md) the limits.
- [FR-301](../tooling/FR-301-expose-complete-cli.md) owns the categories and
  exits; FR-323 the run value form; FR-272 the refusal causes.
