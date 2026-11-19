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
  - target: ix://agent-ix/filament-specification/FR-181
    type: references
  - target: ix://agent-ix/filament-specification/FR-823
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
  `grammar` is a state model object type or a `hybrid` declaration;
  `initial` is the initial-state snapshot, required for a plan over a state
  model and not admitted on a hybrid plan, whose initial set the `hybrid`
  declaration states (FR-819); `policy` (optional, default empty) the ordered policy clause
  names (FR-803), each at most once; `limits` (optional) any subset of
  FR-801's planner limits; `accounting` (optional) any subset of the
  `ScalarLimitsV1` counters; `deadline_ms` (optional) the wall-clock
  deadline; `dwell_grid` (required for, and only admitted on, a hybrid plan,
  FR-819) the exact positive dwell grid.
- The member list is closed: these seven members and `format` and
  `package` are the only ones. Test seams (FR-650 "Test seams") and engine
  selection are library parameters, never request members.
- `dwell_grid` SHALL be written in FR-181's typed value form of an exact
  rational: `{"type": "rational", "numerator": N, "denominator": D}` or
  `{"type": "decimal", "coefficient": C, "scale": S}`, read as the exact
  normalized rational and meaning that many units of the hybrid
  declaration's time unit. A JSON number, a string or the `integer` form
  refuses `wrong-value-kind`; zero or a negative value refuses
  `invalid-value`. Wherever the outcome echoes it (`grid-exhausted`), it is
  written as FR-181's normalized `rational` form. A grid is a rational
  because a dwell grid such as one half of a time unit must be exact, and a
  JSON number is not.
- A host request surface (a run request of the native runtime, FR-323) MAY
  carry the same members under its own envelope; their meaning is this
  requirement's. Every pointer this requirement and FR-801, FR-806 and
  FR-819 state (`/limits/<member>`, `/policy/<i>`, `/initial`,
  `/dwell_grid`) is relative to the plan request object; a host surface
  SHALL report it prefixed by the pointer of the member that carries the
  plan request in its own request object, so the native runtime's
  `synthesis` member reports `/synthesis/limits/max_work` and
  `/synthesis/dwell_grid`. One rule for every member keeps a pointer
  resolvable in the document the caller actually sent.
- The native runtime's `synthesis` member is FR-823's synthesis request,
  one member for every synthesis kind: when its selected declaration is a
  plan synthesis it carries exactly this requirement's members, and when it
  is a value synthesis it carries FR-823's value members and runs FR-194.
  Under that member, `declaration` is selected by FR-823 "Selection": it may
  be omitted when the package holds exactly one `synthesis` declaration, and
  a value-type declaration it names runs as a value synthesis rather than
  refusing. In a `filament.plan-request/v1` document, which asks for a plan
  by its format, `declaration` is required and FR-806's refusal of a
  value-type declaration stands.
- Admission refuses `invalid_runtime_input` at the first defect: not JSON or
  not an object (`malformed-json`); a missing, unknown or duplicate member
  (`missing-member`, `unknown-member`, `duplicate-member`); a member of the
  wrong JSON kind (`wrong-value-kind`); a duplicate policy name, a limit of
  zero or a negative deadline (`invalid-value`). An unknown member of
  `limits` or `accounting` is an unknown member of a closed object and
  refuses `unknown-member` at its pointer like any other; FR-461-AC-4
  governs only an unknown setting name given to `--limit` on the CLI, which
  names no request member.

### Outcome document

```json
{
  "format": "filament.plan-outcome/v1",
  "package": "<checked package identity>",
  "declaration": "Reach",
  "status": "optimal",
  "objective": {"direction": "minimize", "expression": "length", "value": {"type": "integer", "value": "3"}},
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
- `objective` is always the object `{direction, expression, value}`:
  `direction` `minimize` or `maximize`, `expression` the objective's source
  text (`length` when the declaration has none), and `value` the plan's
  objective value or `null` when no plan is reported. `objective.value`,
  every argument and state value and every `delay` are written in the
  native runtime's value form (FR-323), which is FR-181's typed value form
  for their type: an integer objective such as `length` as `{"type":
  "integer", "value": "3"}`, a Decimal as `{"type": "decimal",
  "coefficient": "30", "scale": "2"}` with its retained scale for `0.30`, a
  Rational as its normalized `{"type": "rational", "numerator": "3",
  "denominator": "4"}`, and a quantity such as a timed `elapsed` as FR-181's
  unit-bearing quantity, the normalized `rational` form with its `unit`
  (`60 min` as `{"type": "rational", "numerator": "60", "denominator": "1",
  "unit": "min"}`). An exact rational or decimal is never written as a JSON
  float, and an integer never as a bare JSON number, because the outcome
  must read back as the value it reports.
- `plan.steps[i]` carries `index`, `operation`, `arguments` as
  `{parameter, value}` pairs in parameter order and, for a timed model,
  `delay`; `plan.states` holds the `length + 1` states from the initial
  state, each as a `filament.state.snapshot/v1` object world.
- `replay` records FR-650's native replay: `reproduced` with the step
  count, the only value a reported plan carries.
- `certificate` carries `type` (`potential` or `closure`, or `enclosure`
  for a hybrid plan's FR-652 certificate, FR-819), `entries`, and
  `outcome` `accepted` or `rejected`; a rejected certificate adds `rule` and
  `entry`, the first failing rule by FR-650's rule names and the entry's
  index.
- `certification` is the FR-331 value standing behind the result:
  `certified` when the core accepted the result's certificate; for a hybrid
  `feasible` plan, its item's certification (FR-819); otherwise
  `uncertified` when a plan is reported without an accepted certificate,
  and `null` when no plan is reported and no certificate was accepted.
- `cause` additionally takes `certificate-rejected{rule, entry}` on an
  `incomplete` result whose closure certificate for `infeasible` the core
  rejected (FR-650).
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

### Members by status

Every member of the outcome document SHALL be present in every outcome; a
member that has no value for the status is `null`, never absent. The
document is a closed shape with one member set, so a strict reader checks
one key set for every status and a missing member is always a defect, as
`bound`, `cause` and `plan` already are when they do not apply.

| Status | `objective.value` | `plan` | `replay` | `certificate` | `certification` | `bound` | `cause` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `optimal` | the value | the plan | `reproduced` | accepted | `certified` | `null` | `null` |
| `feasible` | the value | the plan | `reproduced` | rejected, or `null` when none was written (or the hybrid `enclosure`) | `uncertified` (hybrid: the item's) | the bound or `null` | `null` |
| `infeasible` | `null` | `null` | `null` | accepted `closure` or `enclosure` | `certified` | `null` | `null` |
| `incomplete` with a best plan | the value | the best plan | `reproduced` | `null` | `uncertified` | `null` | the stop cause |
| `incomplete` with no plan | `null` | `null` | `null` | `null`, or the rejected certificate under `certificate-rejected` | `null` | `null` | the stop cause |
| `unsupported` | `null` | `null` | `null` | `null` | `null` | `null` | the `unsupported` cause |

`objective`'s `direction` and `expression`, `explored` (zero counts when no
search ran) and `limits` are written for every status.

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
| FR-805-AC-6 | Over the counter, the goal-21 `infeasible` outcome, a `plan_search.max_states` 2 `incomplete` outcome with no plan, and a hybrid `unsupported` outcome (FR-819-AC-3) each carry every member of the outcome document: `objective` with `direction`, `expression` and `value` `null`, and `plan`, `replay` and `bound` `null`; `certificate` is the accepted closure certificate with `certification` `certified` on `infeasible`, and `null` with `certification` `null` on the other two; no member is absent in any of the three. | Test (TC-1955) |
| FR-805-AC-7 | FR-650-AC-1's outcome writes `objective.value` `{"type": "integer", "value": "3"}`; FR-810-AC-3's writes it as FR-181's unit-bearing quantity `{"type": "rational", "numerator": "60", "denominator": "1", "unit": "min"}`; FR-809-AC-2's writes `3/4` as `{"type": "rational", "numerator": "3", "denominator": "4"}`; none writes a bare JSON number. | Test (TC-1955) |
| FR-805-AC-8 | A plan request with `limits` `{"max_steps": 1}` refuses `unknown-member` at `/limits/max_steps` and `accounting` `{"steps": 1}` at `/accounting/steps`, not FR-461-AC-4's setting refusal; carried under a host surface's `synthesis` member the same requests refuse at `/synthesis/limits/max_steps` and `/synthesis/accounting/steps`; a hybrid request's `dwell_grid` `{"type": "rational", "numerator": "1", "denominator": "2"}` is admitted as one half time unit, `dwell_grid` `1` (a JSON number) refuses `wrong-value-kind` and `{"type": "rational", "numerator": "0", "denominator": "1"}` refuses `invalid-value`, each at `/dwell_grid`; a hybrid request carrying `initial` refuses `unknown-member` at `/initial`. | Test (TC-1955) |

## Dependencies

- [FR-650](FR-650-plan-and-optimize-over-a-state-model.md) owns the result
  kinds; [FR-801](FR-801-bound-and-meter-the-plan-search.md) the limits.
- [FR-301](../tooling/FR-301-expose-complete-cli.md) owns the categories and
  exits; FR-323 the run value form; FR-272 the refusal causes.
