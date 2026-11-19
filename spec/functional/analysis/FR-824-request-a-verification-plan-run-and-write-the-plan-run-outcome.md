---
id: FR-824
title: "Request a verification plan run, select its plan and write the plan run outcome"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-192
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-823
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-190
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-409
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-290
    type: depends_on
---

# FR-824: Request a verification plan run, select its plan and write the plan run outcome

## Description

When a caller asks for a verification plan, the coordinator SHALL read one
verification plan request with a strict reader, select exactly one plan by
its declared name, supply the request's per-step inputs to the steps that
consume them, and write one `filament.verification-plan-outcome/v1`
document with one disposition per plan node and the exit FR-301 assigns.

## Inputs

A verification plan request, the checked package, and the plan documents
FR-192 "Package form" emits for that package.

## Outputs

One `filament.verification-plan-outcome/v1` document, or a refusal naming
the first defect by its RFC 6901 pointer.

## Behavior

### Request

```json
{"plan": "Release", "emit": false, "execute": true, "initial": "spine-snapshot.json", "seed": 7}
```

A native run carries the request as its one `verification_plan` member, and
every pointer below is reported with that member's prefix
(`/verification_plan/plan`), as FR-805 states for a host surface. The member
list is closed:

| Member | Type | Default | Consumed by |
| --- | --- | --- | --- |
| `plan` | string, a bare or qualified declared name | Selection | the selection |
| `emit` | boolean | `false` | the outcome's `document` |
| `execute` | boolean | `true` | the coordinator |
| `initial` | string, a `filament.state.snapshot/v1` file resolved against the request file's directory | none | every `model_check`, `model_based_test`, `simulate` and `contract_check` step whose claim's subject is a state model: the state each such step starts from |
| `seed` | unsigned 64-bit integer | `0` | every `model_based_test` and `probabilistic_statistical` step: the FR-190 generation seed, the same seed for each such step, so a rerun with one seed reproduces every step |
| `document` | `{"file": P}` | none | the plan document, on a package-route run only (below) |

- An unknown member refuses `invalid_runtime_input`/`unknown-member` at its
  pointer; a member of the wrong JSON kind `wrong-value-kind`; `emit` and
  `execute` both `false` `invalid-value` at `/execute`, since the request
  would ask for nothing.
- Once the plan is selected, `initial` on a plan with no step it is consumed
  by, and `seed` on a plan with no step it is consumed by, refuse
  `unknown-member` at their pointers: the member set is closed by the
  selected plan, as FR-823 closes it by the selected synthesis kind. A step
  that consumes `initial` and finds none starts from every state the step's
  `domain` admits (FR-192 "Source form").
- `initial` is read by the snapshot's own admission for the step's model; an
  inadmissible snapshot refuses with that admission's refusal at `/initial`
  and the snapshot's member, as FR-806 states for a plan request.
- No member sets a step's method, engine, bound or limits: those are the
  plan's (FR-192), and each step runs under its method's FR-461 settings, so
  a request cannot weaken a step the plan states.

### Selection

`plan` names one plan by its declared name. A plan is outside the checked
package (FR-192 "Package form"), so its name is not a `claim_names` row; it
is the `plan` member of the plan documents emitted for the package, matched
by FR-322's `claim_names` lookup rule over those names (a bare name the last
segment of exactly one plan's `qualified_name`, a qualified `a::Release` the
whole path), with FR-823's refusals:

| Request | Result | Code / cause | Locus |
| --- | --- | --- | --- |
| `plan` names one plan | that plan runs | | |
| `plan` names no plan and no declaration | refusal | `missing_declaration`/`missing-name` naming the name | `/plan` |
| `plan` names a declaration that is not a plan (a claim, a `synthesis`) | refusal | `missing_declaration`/`missing-name` naming the name and its form | `/plan` |
| a bare `plan` matches two plans | refusal | `ambiguous_declaration`/`ambiguous-name` naming both | `/plan` |
| `plan` absent, exactly one plan | that plan runs | | |
| `plan` absent, no plan | refusal | `missing_declaration`/`missing-name` | `/plan` |
| `plan` absent, two or more plans | refusal | `invalid_runtime_input`/`missing-member` naming every plan | `/plan` |

On a run over a selected checked package, no source is compiled and no plan
document is emitted, so the request SHALL carry `document`, the plan
document to run; its `plan` is the only candidate of the selection, and a
`package` that differs from the selected package's identity refuses
`invalid_runtime_input`/`invalid-value` at `/document`, naming both
identities (FR-192 "Package form"). On a source run `document` refuses
`unknown-member`.

### Outcome document

```json
{
  "format": "filament.verification-plan-outcome/v1",
  "package": "<checked package identity>",
  "plan": "Release",
  "document": null,
  "status": "failed",
  "inputs": {"initial": "spine-snapshot.json", "seed": 7},
  "dispositions": {
    "safe":   {"status": "refuted", "method": "model_check", "evidence": "refutation", "blocked_by": null, "result": {}},
    "tested": {"status": "tested",  "method": "model_based_test", "evidence": "tested",     "blocked_by": null, "result": {}},
    "timely": {"status": "blocked", "method": "model_check", "evidence": null,         "blocked_by": "safe", "result": null}
  },
  "failed_steps": ["safe"]
}
```

- `document` is the plan document when `emit` is `true`, else `null`.
- `inputs` echoes `initial` (or `null`) and the `seed` in force.
- `dispositions` holds one entry per plan node, keyed by step name, or is
  `null` when `execute` is `false`. Its `status` is the step's FR-331
  result value (`proved`, `refuted`, `tested`, `measured`, `inconclusive`,
  `unsupported`, `declined`, `incomplete` or `failed`), or `blocked` for a
  step a `requires` prerequisite stopped, with `blocked_by` the first such
  prerequisite in source order. The vocabulary is closed and is the FR-331
  result vocabulary with one addition, so a step settles exactly as the same
  item settles outside a plan; a `measured` step's `result` carries its
  FR-409 decision. `evidence` is the evidence kind the method produced
  (FR-192 method table), never another, and `null` on a step that produced
  none; `result` is the step's FR-331 result entry, `null` on a blocked
  step.
- A method with no registered engine is not a plan refusal. The step's item
  is negotiated as every item is (FR-290): when no registered backend
  matches its kind, extent and evidence kind, it settles `unsupported`
  with `unsupported_projection`/`unsupported-requested-capability` naming
  the kind and the extent, no engine runs, and every step that does not
  require it still runs (FR-192-AC-2). Every other negotiated disposition
  that runs no engine (a bounded-only candidate's `requires-bound` or
  `unbounded-extent`, an ambiguous or unknown backend) settles the step with
  the record and diagnostic the same item gets outside a plan, and never
  stops a sibling step.
- A `requires` prerequisite is satisfied when it settles `proved`, `tested`,
  or `measured` with decision `accepted`. A prerequisite with any other
  status, `blocked` included, blocks each step that requires it, because the
  dependent step's premise is the prerequisite's evidence; a step that does
  not require it is never blocked by it. A blocked step is not negotiated
  and runs no engine.
- `failed_steps` lists, in source order, every step whose status is
  `refuted` or `failed`, or `measured` with decision `rejected`, so the plan
  result names each failing step.
- `status` is `passed` when every node is `proved`, `tested`, or `measured`
  with decision `accepted`; `failed` when any node is in `failed_steps`; and
  otherwise `incomplete`.

The run is multi-item output, so its exit is FR-301's: each node's
disposition has its FR-301 category (`proved` and `tested` success `0`,
`refuted` violation `10`, `incomplete` and `blocked` incomplete `22`,
`inconclusive` `22`, `unsupported` `21`, `declined` refusal `20`, `failed`
internal-failure `30`, and `measured` by its FR-409 decision: `accepted`
success `0`, `rejected` violation `10`, `undecided` inconclusive `22`), and the process exit is
the highest-severity code present under FR-301's ordering (`30`, `20`,
`21`, `22`, `10`, `11`, `0`). There is no plan-specific ranking. A request
with `execute` `false` settles no node and exits `0`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-824-AC-1 | Executing FR-192's `Release` plan where `safe` is refuted writes `status` `failed`, `safe` `refuted` with `evidence` `refutation`, `tested` `tested`, `timely` `blocked` with `blocked_by` `safe` and `result` `null`, three dispositions, `failed_steps` `["safe"]`, `inputs.seed` 7, and exits `22`, FR-301's highest-severity code of the refuted `10` and the blocked `22`; `{"plan": "Release", "emit": true, "execute": false}` writes the plan document as `document`, `dispositions` `null`, exit `0`. | Test (TC-2039) |
| FR-824-AC-2 | Over one plan, `{}` runs it; over two plans, `{}` refuses `invalid_runtime_input`/`missing-member` at `/plan` naming both; `plan` `Nope` and `plan` naming the claim `Never` each refuse `missing_declaration`/`missing-name` at `/plan`; a bare `Release` matching `A::Release` and `B::Release` refuses `ambiguous_declaration`/`ambiguous-name` at `/plan`; each exits `20` before any step runs. | Test (TC-2039) |
| FR-824-AC-3 | `seed` `"7"` refuses `wrong-value-kind` at `/seed`; `emit` and `execute` both `false` refuse `invalid-value` at `/execute`; `engine` refuses `unknown-member` at `/engine`; `seed` on a plan with only `model_check` steps refuses `unknown-member` at `/seed`; a plan run over a selected package without `document` refuses `missing-member` at `/document`, and with a plan document of another package `invalid-value` at `/document` naming both identities. | Test (TC-2039) |
| FR-824-AC-4 | `Release` executed with no registered engine advertising `temporal-satisfaction`: `safe` settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming `temporal-satisfaction` and the extent `bounded`, and no engine runs for it; `tested` still runs and settles `tested`; `timely`, whose prerequisite is not satisfied, is `blocked` by `safe` and is not negotiated; `status` is `incomplete`, `failed_steps` is empty and the exit is `21`, FR-301's highest-severity code of `21`, `22` and `0`. | Test (TC-2045) |
| FR-824-AC-5 | A plan of a `probabilistic_statistical` step `s` and a `model_check` step `t depends [s]`: with `s` `measured` `accepted`, `t` runs and `s`'s disposition has category success `0`; with `s` `measured` `rejected`, `t` is `blocked` by `s`, `failed_steps` is `["s"]`, `status` `failed` and the exit `22`; with `s` `measured` `undecided`, `t` is `blocked`, `failed_steps` is empty, `status` `incomplete` and the exit `22`. | Test (TC-2045) |

## Dependencies

- [FR-192](FR-192-execute-linked-verification-plans.md) owns the plan, its
  graph and its document; [FR-823](FR-823-read-the-synthesis-request-and-write-the-synthesis-outcome-document.md)
  the selection rule this request shares; FR-190 the seed; FR-331 the step
  settlements; FR-301 the categories and exits.
