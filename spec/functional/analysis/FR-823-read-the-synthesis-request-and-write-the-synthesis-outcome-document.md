---
id: FR-823
title: "Read the synthesis request, select its declaration and write the synthesis outcome document"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-194
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-805
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-461
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: references
  - target: ix://agent-ix/filament-specification/FR-323
    type: references
---

# FR-823: Read the synthesis request, select its declaration and write the synthesis outcome document

## Description

When a caller asks for a synthesis, the synthesizer SHALL read one
synthesis request with a strict reader, select exactly one `synthesis`
declaration of the checked package by its declared name, refuse a request
that selects none or several before any search, and write one outcome
document: FR-805's `filament.plan-outcome/v1` for a plan synthesis and the
`filament.synthesis-outcome/v1` document below for a value synthesis.

## Inputs

A synthesis request and the checked package whose
`claim`/`synthesis_request` nodes (FR-322 "Synthesis-request nodes") it
selects from.

## Outputs

One outcome document, or a refusal naming the first defect by its RFC 6901
pointer.

## Behavior

### Decision: one synthesis request for every synthesis kind

A synthesis request is one object whatever the selected declaration's
grammar. A native run carries it as its `synthesis` member, the member that
already carries FR-805's plan request (FR-805 "Request"). The kind is the
node tag of the selected node's `grammar` target, so the caller never states
it: a type node selects value synthesis (FR-194), a `model`/`object_type`
node plan synthesis (FR-650), a `hybrid`/`hybrid_model` node hybrid plan
synthesis (FR-819). Two members for one declaration form would let a request
name a plan declaration under the value member, and would make the caller
restate a fact the package already holds.

### Request members

| Member | Value synthesis | Plan synthesis | Hybrid plan synthesis |
| --- | --- | --- | --- |
| `declaration` | optional (Selection) | optional (Selection) | optional (Selection) |
| `limits` | optional, `synthesis.*` names below | optional, FR-801's names | optional, FR-801's names |
| `accounting` | optional, `ScalarLimitsV1` counters | as FR-805 | as FR-805 |
| `deadline_ms` | optional | as FR-805 | as FR-805 |
| `initial` | not admitted | required (FR-805) | not admitted |
| `policy` | not admitted | optional (FR-805) | optional (FR-805) |
| `dwell_grid` | not admitted | not admitted | required (FR-819) |

- The member list is closed, as FR-805's is. Admission runs in two steps. A
  member named in no column of the table refuses
  `invalid_runtime_input`/`unknown-member` at its pointer before any stage
  runs. Once the package is checked and the declaration selected, a member
  the selected kind does not admit refuses `unknown-member` at its pointer,
  as `initial` on a hybrid plan does today (FR-805-AC-8), and a required
  member the kind needs refuses `missing-member` at its pointer.
- A value synthesis's `limits` is a closed object of FR-461 limits with the
  engine name `synthesis`: `max_candidates` (setting
  `synthesis.max_candidates`, one per candidate value enumerated, default
  10000000) and `max_work` (setting `synthesis.max_work`, one per
  evaluation of the block or of the objective on one candidate, default
  100000000). Each is an unsigned integer of at least 1; `0` refuses
  `invalid-value`, and any other name refuses `unknown-member`, at
  `/limits/<member>`. The independent checker runs under the request's
  `accounting`, not under these limits, so a limit on the search never
  weakens the check.
- Every pointer is relative to the synthesis request object; a host
  surface prefixes it as FR-805 states, so the native runtime's member
  reports `/request/synthesis/declaration` and `/request/synthesis/limits/max_work`.

### Selection

`declaration` names the declaration to run by its declared name, resolved
through FR-322's `claim_names` lookup, as an export request names its
obligations by declared name (FR-817): a bare name matches the last segment
of the rows of exactly one declaration, a qualified `a::Name` the whole
`qualified_name`, byte for byte. The selected node is that declaration's own
node.

| Request | Result | Code / cause | Locus |
| --- | --- | --- | --- |
| `declaration` resolves to a `claim`/`synthesis_request` node | that declaration runs | | |
| `declaration` matches no `claim_names` row | refusal | `missing_declaration`/`missing-name` naming the name | `/declaration` |
| `declaration` resolves to a claim of another form (an invariant, a temporal clause, a `verify` plan) | refusal | `missing_declaration`/`missing-name` naming the name and the node's `semantic_form` | `/declaration` |
| a bare `declaration` matches the rows of two declarations | refusal | `ambiguous_declaration`/`ambiguous-name` naming both qualified names | `/declaration` |
| `declaration` absent, the package holds exactly one `synthesis_request` node | that declaration runs | | |
| `declaration` absent, the package holds none | refusal | `missing_declaration`/`missing-name` | `/declaration` |
| `declaration` absent, the package holds two or more | refusal | `invalid_runtime_input`/`missing-member`, naming every candidate declaration | `/declaration` |

The `missing-name` code for a non-synthesis name is FR-806's ("`declaration`
names no `synthesis` declaration"): a request asks for a synthesis, and a
name that is not one is a missing synthesis. An omitted name runs the only
declaration because the selection is then not a choice; with several the
reader never picks one by source or graph order, as an export never picks an
obligation set the request did not state.

### Outcome document of a value synthesis

```json
{
  "format": "filament.synthesis-outcome/v1",
  "package": "<checked package identity>",
  "declaration": "Pick",
  "status": "candidate",
  "check": "validated",
  "candidate": {"type": "integer", "value": "7"},
  "objective": null,
  "optimal": null,
  "explored": {"visited": 8, "work": 8},
  "lineage": {"node": {"package": "<package>", "path": ["Pick"], "ordinal": 0}, "index": 7, "symmetry": "identity", "synthesizer": "filament.synthesis.enumerate/v1", "checker": "filament.native.value-validity/v1"},
  "counterexample": null,
  "limits": {"synthesis.max_candidates": 10000000, "synthesis.max_work": 100000000},
  "cause": null
}
```

- `status` is FR-194's result: `candidate`, `no-candidate`, `incomplete`,
  `unsupported` or `failed`.
- `check` is `validated` or `rejected` once the independent checker has
  settled the candidate, `unchecked` on an `incomplete` result's best
  candidate, and `null` when no candidate is reported.
- `candidate` is the candidate value in the native runtime's value form
  (FR-323), FR-181's typed value form of `D`: never a bare JSON number.
- `objective` is `null` when the declaration has no objective, and otherwise
  `{direction, expression, value}`: `direction` and `expression` the
  `synthesis_request` node's `objective.direction` and `objective.text`,
  and `value` the objective on the reported candidate in the run value form.
- `optimal` is `true` on a `validated` candidate of a declaration with an
  objective, `false` on an `incomplete` result's best candidate (FR-194: no
  optimality claim), and `null` otherwise.
- `explored.visited` is the number of candidates evaluated, the reported one
  included, so with an objective `visited - 1` earlier candidates were
  evaluated and failed the block; `explored.work` the granted
  `synthesis.max_work` units.
- `lineage` identifies the candidate's origin (FR-194-AC-2, AC-5): the
  `synthesis_request` node, the candidate's zero-based `index` in the cost
  ordering, the symmetry rule, the synthesizer's identity and the
  independently identified checker's identity. It is `null` when no
  candidate is reported.
- `counterexample` is the checker's replayable counterexample on a
  `rejected` candidate, as the `value-validity` item records it, and `null`
  otherwise.
- `limits` lists both `synthesis.*` limits in force with defaults filled.
- `cause` is FR-801's `limit-reached{limit, value, setting}` (the setting
  `synthesis.max_candidates`, `synthesis.max_work`, an `accounting` counter
  or `deadline_ms`) or `cancelled` on `incomplete`; FR-193's
  `unsupported_projection`/`unsupported-requested-capability` on
  `unsupported`; the provider's FR-331 failure on `failed`; and `null`
  otherwise.

### Members by status

Every member is present in every outcome, `null` where the status gives it
no value, so one strict reader checks one key set, as FR-805's outcome
document does.

| Status | `check` | `candidate` | `objective.value` | `optimal` | `lineage` | `counterexample` | `cause` | Category | Exit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `candidate`, validated | `validated` | the value | the value | `true` with an objective | the lineage | `null` | `null` | success | `0` |
| `candidate`, rejected | `rejected` | the value | the value | `null` | the lineage | the counterexample | `null` | internal-failure | `30` |
| `no-candidate` | `null` | `null` | `null` | `null` | `null` | `null` | `null` | violation | `10` |
| `incomplete` with a best candidate | `unchecked` | the value | the value | `false` | the lineage | `null` | the stop cause | incomplete | `22` |
| `incomplete` with none | `null` | `null` | `null` | `null` | `null` | `null` | the stop cause | incomplete | `22` |
| `unsupported` | `null` | `null` | `null` | `null` | `null` | `null` | the cause | unsupported | `21` |
| `failed` | `null` | `null` | `null` | `null` | `null` | `null` | the failure | internal-failure | `30` |

A refusal is FR-301's refusal, exit `20`, with no outcome document. A
`rejected` candidate is a method failure: the search's own evaluation and
the independent check disagree about one value, so the synthesizer is wrong
and no answer is given. `no-candidate` is a violation, like an `infeasible`
plan: a proof that no value of `D` meets the block.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-823-AC-1 | Over FR-194-AC-6's `Pick`, a request `{"declaration": "Pick"}` writes `filament.synthesis-outcome/v1` with `status` `candidate`, `check` `validated`, `candidate` `{"type": "integer", "value": "7"}`, `objective` and `optimal` `null`, `explored.visited` 8, `lineage.index` 7 and `lineage.symmetry` `identity`, `counterexample` and `cause` `null`, and every member present, exit `0`; `Pick * Pick = 50` writes `no-candidate` with `explored.visited` 10 and every other result member `null`, exit `10`; `minimize Pick` with `Pick >= 3` writes candidate 3, `optimal` `true`, `objective` `{"direction": "minimize", "expression": "Pick", "value": {"type": "integer", "value": "3"}}` and `explored.visited` 4. | Test (TC-2038) |
| FR-823-AC-2 | `Pick * Pick = 50` with `limits` `{"max_candidates": 3}` writes `incomplete` with `limit-reached` naming `synthesis.max_candidates` and `explored.visited` 3, exit `22`, never `no-candidate`; `limits` `{"work": 1}` refuses `invalid_runtime_input`/`unknown-member` at `/limits/work` and `{"max_work": 0}` `invalid-value` at `/limits/max_work`, exit `20`. | Test (TC-2038) |
| FR-823-AC-3 | A package holding only `Pick` runs it for a request with no `declaration`; a package holding `Pick` and `Reach` refuses that request `invalid_runtime_input`/`missing-member` at `/declaration` naming both; `declaration` `Nope` refuses `missing_declaration`/`missing-name` at `/declaration`; `declaration` naming the invariant `Positive` refuses `missing_declaration`/`missing-name` at `/declaration` naming `state_clause`; a bare `Pick` matching `A::Pick` and `B::Pick` refuses `ambiguous_declaration`/`ambiguous-name` at `/declaration`, and `A::Pick` runs; each refusal exits `20` before any search. | Test (TC-2038) |
| FR-823-AC-4 | A request selecting the value synthesis `Pick` with `initial`, `policy` or `dwell_grid` refuses `invalid_runtime_input`/`unknown-member` at `/initial`, `/policy` or `/dwell_grid`; a request selecting the plan synthesis `Reach` without `initial` refuses `missing-member` at `/initial`; a member `engines` refuses `unknown-member` at `/engines` before any stage runs; under a native run's `synthesis` member each pointer carries the `/request/synthesis` prefix. | Test (TC-2038) |
| FR-823-AC-5 | A candidate the independent checker rejects (a checker fault injected through a library test seam, never a request member) writes `check` `rejected`, the checker's `counterexample` and the `lineage`, exit `30`, never exit `0`. | Test (TC-2038) |

## Dependencies

- [FR-194](FR-194-synthesize-bounded-candidates.md) owns the source form and
  the result kinds; [FR-805](FR-805-read-the-plan-request-and-write-the-plan-outcome.md)
  the plan request members and the plan outcome;
  [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) the
  `synthesis_request` node and the `claim_names` lookup;
  [FR-301](../tooling/FR-301-expose-complete-cli.md) the categories and exits;
  FR-461 the limit settings.
