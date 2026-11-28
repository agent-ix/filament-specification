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
{"plan": "Release", "emit": false, "execute": true, "universe_snapshot": "universe.json", "initial": "spine-snapshot.json", "seed": 7, "max_trace_length": 8}
```

A native run carries the request as its one `verification_plan` member, and
every pointer below is relative to the plan request object. A host surface
reports it prefixed by the pointer of the `verification_plan` member in the
document the caller sent, as FR-805 states for a host surface: in a
`native-run/1` document that member sits in its `request` object, so `/plan`
is reported as `/request/verification_plan/plan`, the same form as FR-817's
`/request/export/obligations`. A pointer then resolves in the bytes the
caller wrote, and every member of one run request is located the same way.
The member list is closed:

| Member | Type | Default | Consumed by |
| --- | --- | --- | --- |
| `plan` | string, a bare or qualified declared name | Selection | the selection |
| `emit` | boolean | `false` | the outcome's `document` |
| `execute` | boolean | `true` | the coordinator |
| `universe_snapshot` | string, a `filament.state.snapshot/v1` file resolved against the request file's directory | none | each consuming `model_based_test` step: its admitted complete population memberships supply the population universes; snapshot field values are not initial states |
| `initial` | string, a `filament.state.snapshot/v1` file resolved against the request file's directory | none | every `model_check`, `model_based_test`, `simulate` and `contract_check` step whose claim's subject is a state model: the state each such step starts from |
| `seed` | unsigned 64-bit integer | `0` | every `model_based_test` and `probabilistic_statistical` step: the FR-190 generation seed, the same seed for each such step, so a rerun with one seed reproduces every step |
| `max_trace_length` | unsigned 64-bit integer, at least 1 | none | every `model_based_test` step whose claim's subject is a state model: the FR-190 trace bound, the same for each such step |
| `max_value_bits` | integer from 1 to 64 | none | every `model_based_test` step with a generation root or an operation result of no finite bound (FR-190 "Value domains"): the FR-190 size bound, the same for each such step |
| `document` | `{"file": P}` | none | the plan document, on a package-route run only (below) |

- An unknown member refuses `invalid_runtime_input`/`unknown-member` at its
  pointer; a member of the wrong JSON kind `wrong-value-kind`; `emit` and
  `execute` both `false` `invalid-value` at `/execute`, since the request
  would ask for nothing.
- A member whose value is JSON `null` is read as absent: it takes its
  default, or for `plan`, `initial` and `document` the absent behaviour
  above. FR-817's export request reads `null` the same way (its example
  carries `"observer": null` for no observer), so one rule holds for every
  member of a run request. The `verification_plan` member itself is the
  request and has no default; `"verification_plan": null` is not an object
  and refuses `wrong-value-kind` at the member's own pointer.
- `seed` SHALL be a JSON number written as an integer, with no fraction or
  exponent part, from 0 to 2^64 − 1. A JSON number outside that domain (a
  negative number such as `-1`, a fraction such as `7.5`, an exponent form
  such as `7e0`, or a value above 2^64 − 1) refuses `invalid-value` at
  `/seed`, because its JSON kind is right and its value is outside the
  member's domain, as FR-331's reader table and FR-805's negative deadline
  read it. A value of another JSON kind (a string such as `"7"`, a boolean,
  an array or an object) refuses `wrong-value-kind`.
- `max_trace_length` and `max_value_bits` are read as `seed` is: a JSON
  number written as an integer, with no fraction or exponent part. A
  `max_trace_length` outside 1 to 2^64 − 1, or a `max_value_bits` outside 1
  to 64, refuses `invalid-value` at its pointer, and a value of another JSON
  kind `wrong-value-kind`.
- Once the plan is selected, `universe_snapshot`, `initial`, `seed`, `max_trace_length` or
  `max_value_bits` on a plan with no step it is consumed by refuses
  `unknown-member` at its pointer: the member set is closed by the
  selected plan, as FR-823 closes it by the selected synthesis kind. The
  coordinator supplies each of them only to the steps that consume it, so a
  step never receives a member FR-190 would refuse as unknown.
- When `execute` is `true`, a selected plan with a step that consumes
  `max_trace_length` or `max_value_bits` and a request without that member
  refuses `invalid_runtime_input`/`missing-member` at the member's pointer,
  naming the first consuming step in source order, and no step runs. Neither
  member has a default, because FR-190 forbids inventing a bound the request
  did not state. These plan-dependent checks run after the selection (its
  `document` refusals included), member by member in the table's order, and
  the first defect is the refusal: a request missing both bound members refuses
  `missing-member` at `/max_trace_length`. A step
  that consumes `initial` and finds none starts from every state the step's
  `domain` admits (FR-192 "Source form").
- `universe_snapshot` is admitted for the selected model by FR-106 before
  any `initial` snapshot is admitted. Each `complete: true` population row
  supplies exactly its listed object keys as that population's finite
  universe; a missing row or `complete: false` supplies no finite universe.
  Snapshot field values are validated by FR-106 but do not become initial
  states. If both members are present, `universe_snapshot` is authoritative:
  `initial` snapshots must be admitted against it and cannot add keys. If it
  is absent, initial snapshots supply universes as before. An invalid
  `universe_snapshot` retains FR-106's exact refusal at `/universe_snapshot`;
  missing path is `io-error`, wrong JSON kind is `wrong-value-kind`. It is
  accepted only when the selected plan has a `model_based_test` state-subject
  step; otherwise it refuses `unknown-member` at `/universe_snapshot`.
- `initial` is read by the snapshot's own admission for the step's model; an
  inadmissible snapshot refuses with that admission's refusal at `/initial`
  and the snapshot's member, as FR-806 states for a plan request.
- No member sets a step's method, engine, `bound` or limits: those are the
  plan's (FR-192), and each step runs under its method's FR-461 settings, so
  a request cannot weaken a step the plan states. `max_trace_length` and
  `max_value_bits` are FR-190 inputs that a plan step does not state (its
  `bound` is `max_cases`), so the request supplies them as it supplies
  `seed`, and they change nothing the plan states.
- No request member changes a step's limits or deadline: a step runs under
  its own `bound`, its method's FR-461 settings at their published defaults,
  and cancellation.

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
  "inputs": {"initial": "spine-snapshot.json", "seed": "7", "max_trace_length": "8", "max_value_bits": null},
  "dispositions": {
    "safe":   {"status": "refuted", "method": "model_check", "evidence": "refutation", "blocked_by": null, "result": {}},
    "tested": {"status": "tested",  "method": "model_based_test", "evidence": "tested",     "blocked_by": null, "result": {}},
    "timely": {"status": "blocked", "method": "model_check", "evidence": null,         "blocked_by": "safe", "result": null}
  },
  "failed_steps": ["safe"]
}
```

- `document` is the plan document when `emit` is `true`, else `null`.
- `inputs` echoes `initial` (or `null`), the `seed` in force, and `max_trace_length` and `max_value_bits` (each `null` when absent). The request's `seed` and `max_trace_length` are JSON integers; `inputs.seed` and `inputs.max_trace_length` write them as canonical unsigned decimal strings (no sign, no leading zero), as the FR-190 record's `provenance.seed` and `limits.max_trace_length` do, because a 64-bit value above 2^53 has no exact RFC 8785 number form; a `null` `max_trace_length` stays `null`.
- `dispositions` holds one entry per plan node, keyed by step name, or is
  `null` when `execute` is `false`. Its `status` is the step's FR-331
  result value (`proved`, `refuted`, `tested`, `measured`, `inconclusive`,
  `unsupported`, `declined`, `incomplete` or `failed`), or `blocked` for a
  step a `requires` prerequisite stopped, with `blocked_by` the first such
  prerequisite in source order. The vocabulary is closed and is the FR-331
  result vocabulary with one addition, so a step settles exactly as the same
  item settles outside a plan. A `measured` step's FR-409 decision is read
  from its `result` only, at `result.verdict.decision.type` (`accepted`,
  `rejected` or `undecided`), the FR-410 `StatisticalVerdict` the FR-331
  `measured` entry carries, with its cause beside it in that object. A
  disposition has exactly the members `status`, `method`, `evidence`,
  `blocked_by` and `result`; it carries no `decision` member of its own,
  because a second copy of the decision could disagree with the verdict
  the engine wrote, and the gate, `failed_steps`, `status` and the exit all
  read that one value. `evidence` is the evidence kind the method produced
  (FR-192 method table), never another, and `null` on a step that produced
  none; `result` is the step's FR-331 result entry, `null` on a blocked
  step. A `model_based_test` step is not negotiated and has no provider
  entry, so its `result` is its FR-190 result record, whose `result` and
  `cause` members carry the same FR-331 settlement; a run that evaluated no
  case is `inconclusive` with cause `no-evaluated-case`, and so is not a
  satisfied prerequisite.
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
  otherwise `incomplete`. When `execute` is `false`, `status` is `null`:
  the run settled no node, so it neither passed nor failed, and
  `incomplete` would name a category (exit `22`) the run does not have.

The run is multi-item output, so its exit is FR-301's: each node's
disposition has its FR-301 category (`proved` and `tested` success `0`,
`refuted` violation `10`, `incomplete` and `blocked` incomplete `22`,
`inconclusive` `22`, `unsupported` `21`, `declined` refusal `20`, `failed`
internal-failure `30`, and `measured` by its FR-409 decision: `accepted`
success `0`, `rejected` violation `10`, `undecided` inconclusive `22`), and the process exit is
the highest-severity code present under FR-301's ordering (`30`, `20`,
`21`, `22`, `10`, `11`, `0`). There is no plan-specific ranking. A request
with `execute` `false` settles no node and exits `0`.

## Certified proofs

- The `native-run/1` request member `require_certified`, a JSON boolean,
  absent meaning `false`, requires certified proofs of every step (FSL
  FR-1052); it is a request member, not a member of `verification_plan`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-824-AC-1 | Executing FR-192's `Release` plan, over a model `M::Order` whose generation roots all have finite bounds, with `{"plan": "Release", "seed": 7, "max_trace_length": 8}` where `safe` is refuted writes `status` `failed`, `safe` `refuted` with `evidence` `refutation`, `tested` `tested`, `timely` `blocked` with `blocked_by` `safe` and `result` `null`, three dispositions, `failed_steps` `["safe"]`, `inputs` `{"initial": null, "seed": "7", "max_trace_length": "8", "max_value_bits": null}`, and exits `22`, FR-301's highest-severity code of the refuted `10` and the blocked `22`; `{"plan": "Release", "emit": true, "execute": false}` writes the plan document as `document`, `dispositions` `null`, exit `0`. | Test (TC-2039) |
| FR-824-AC-2 | Over the one plan `Release`, `{"max_trace_length": 8}` runs it; over two plans, `{"max_trace_length": 8}` refuses `invalid_runtime_input`/`missing-member` at `/plan` naming both; `plan` `Nope` and `plan` naming the claim `Never` each refuse `missing_declaration`/`missing-name` at `/plan`; a bare `Release` matching `A::Release` and `B::Release` refuses `ambiguous_declaration`/`ambiguous-name` at `/plan`; each exits `20` before any step runs. | Test (TC-2039) |
| FR-824-AC-3 | `seed` `"7"` refuses `wrong-value-kind` at `/seed`; `emit` and `execute` both `false` refuse `invalid-value` at `/execute`; `engine` refuses `unknown-member` at `/engine`; `seed` on a plan with only `model_check` steps refuses `unknown-member` at `/seed`; a `Release` run carrying `"max_trace_length": 8` over a selected package without `document` refuses `missing-member` at `/document`, and with a plan document of another package `invalid-value` at `/document` naming both identities; a `Sized` run (FR-824-AC-7) with neither `max_trace_length` nor `max_value_bits` refuses `missing-member` at `/max_trace_length` only. | Test (TC-2039) |
| FR-824-AC-4 | `Release` executed with `{"plan": "Release", "max_trace_length": 8}` and no registered engine advertising `temporal-satisfaction`: `safe` settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming `temporal-satisfaction` and the extent `bounded`, and no engine runs for it; `tested` still runs and settles `tested`; `timely`, whose prerequisite is not satisfied, is `blocked` by `safe` and is not negotiated; `status` is `incomplete`, `failed_steps` is empty and the exit is `21`, FR-301's highest-severity code of `21`, `22` and `0`. | Test (TC-2045) |
| FR-824-AC-5 | A plan of a `probabilistic_statistical` step `s` and a `model_check` step `t depends [s]`: with `s` `measured` `accepted`, `t` runs and `s`'s disposition has category success `0`; with `s` `measured` `rejected`, `t` is `blocked` by `s`, `failed_steps` is `["s"]`, `status` `failed` and the exit `22`; with `s` `measured` `undecided`, `t` is `blocked`, `failed_steps` is empty, `status` `incomplete` and the exit `22`. | Test (TC-2045) |
| FR-824-AC-6 | Carried in a `native-run/1` request, `seed` `"7"` refuses `wrong-value-kind` at `/request/verification_plan/seed`; `seed` `-1`, `7.5` and `18446744073709551616` each refuse `invalid-value` there; `{"plan": "Release", "seed": null, "emit": null, "initial": null, "max_value_bits": null, "max_trace_length": 8}` runs `Release` with `inputs` `{"initial": null, "seed": "0", "max_trace_length": "8", "max_value_bits": null}` as if the four `null` members were absent; `"verification_plan": null` refuses `wrong-value-kind` at `/request/verification_plan`; each refusal exits `20` before any step runs. In FR-824-AC-5's runs, `s`'s disposition has exactly the members `status`, `method`, `evidence`, `blocked_by` and `result`, and its decision is `result.verdict.decision.type` (`accepted`, `rejected`, `undecided`); `{"plan": "Release", "emit": true, "execute": false}` writes `status` `null`. | Test (TC-2046) |
| FR-824-AC-7 | `{"plan": "Release"}` refuses `invalid_runtime_input`/`missing-member` at `/max_trace_length` naming the step `tested`, and carried in a `native-run/1` request at `/request/verification_plan/max_trace_length`; `max_trace_length` 0, `-1` and `8.5` each refuse `invalid-value` there and `"8"` `wrong-value-kind`; `max_value_bits` 8 on `Release`, whose roots all have finite bounds, refuses `unknown-member` at `/max_value_bits`, and `max_trace_length` 8 on a plan of `model_check` steps only refuses `unknown-member` at `/max_trace_length`; over a plan `Sized` whose one step `s` is a `model_based_test` of a state clause over a model with a field `n: Integer`, `{"plan": "Sized", "max_trace_length": 8}` refuses `missing-member` at `/max_value_bits` naming `s`, `max_value_bits` 0 and 65 refuse `invalid-value` there, and with `max_value_bits` 8 `s` runs with `max_trace_length` 8 and `max_value_bits` 8 `inputs.max_value_bits` is 8 and `inputs.max_trace_length` is `"8"`; each refusal exits `20` before any step runs. A step of a plan whose FR-190 run discards every case is `inconclusive` with `result.cause` `{"type": "no-evaluated-case"}`, and a step that `depends` on it is `blocked` by it, exit `22`. | Test (TC-2046) |
| FR-824-AC-8 | The checked package `test/no-initial` declares record value type `ix://test/no-initial/Cell` with `value: Int[0,1]`, model `NoInitial` with only population `cells: Population<Cell>`, and state clause `NoInitial::CheckCells` whose predicate is `true` over that model. Its plan source is `verify Release using p { check tested claim NoInitial::CheckCells method model_based_test domain M::NoInitial bound 100; }`. Executing that plan with `{"plan":"Release","universe_snapshot":"universe.json","seed":7,"max_trace_length":8}` and no `initial` admits the `universe_snapshot` before `initial`; its `cells` row `complete: true`, with objects `c1` and `c2`, is the exact object universe `{c1,c2}`. Each generated initial state and the reachability domain is the product of those keys and `Cell.value` in `{0,1}`; no other key or population is present, and snapshot field values do not constrain the field-value domain or become initial states. Equal requests produce byte-equal model-based-test records and `inputs.initial` is `null`. | Test (TC-2039) |
| FR-824-AC-9 | For a refusal fixture whose selected model declares populations `cells` then `unbounded_cells`, both of `record value type ix://test/no-initial/Cell`, a numeric `universe_snapshot` refuses `wrong-value-kind` at `/universe_snapshot`, a missing `universe.json` refuses `io-error` there, and a snapshot whose `cells` row is complete with objects `c1`,`c2` and whose `unbounded_cells` row has `complete: false` admits but leaves `unbounded_cells` unbounded. The selected plan result is `declined` with refusal `{member:"subject",code:"requires_bound",cause:"requires-bound",details:{domains:[{key:DomainKey::Population{member_type:{package:"test/no-initial",path:["model","record_value_type","ix://test/no-initial/Cell"],ordinal:0},ordinal:1},kind:Population(None)}]}}`, exit `20`, before any case runs. A malformed or model-inadmissible snapshot retains FR-106's exact code and cause at `/universe_snapshot`. Supplying `universe_snapshot` to a selected plan with no state-subject `model_based_test` step refuses `unknown-member` there. | Test (TC-2039) |

## Dependencies

- [FR-192](FR-192-execute-linked-verification-plans.md) owns the plan, its
  graph and its document; [FR-823](FR-823-read-the-synthesis-request-and-write-the-synthesis-outcome-document.md)
  the selection rule this request shares; FR-190 the seed; FR-331 the step
  settlements; FR-301 the categories and exits.
