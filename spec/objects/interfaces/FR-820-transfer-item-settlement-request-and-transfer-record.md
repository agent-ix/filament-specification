---
id: FR-820
title: "Transfer item settlement request and transfer record"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-380
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-492
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-375
    type: references
  - target: ix://agent-ix/filament-specification/FR-377
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-360
    type: references
  - target: ix://agent-ix/filament-specification/FR-243
    type: references
---

# FR-820: Transfer item settlement request and transfer record

## Description

When the verification planner settles an FR-380 transfer item, it SHALL read
one FR-492 `filament.planner-settlement/v1` request whose `item` has role
`transfer` and whose two slots carry the refinement premise and the abstract
claim premise, SHALL refuse a malformed, forged or mismatched request before
settlement, and SHALL write one FR-492 `filament.planner-result/v1` result
that carries a `transfer` record of every fact FR-380 checked. A transfer
item is planner-local (FR-380 "Kind and reading"), so this request is its only
wire form; a tool that settles both premises in the same process passes its
own records instead of bytes, as FR-492 allows.

## Inputs

- The checked package that carries the refinement declaration (FR-375) and
  the clause `P`.
- A `filament.planner-settlement/v1` request (below).

## Outputs

A planner result with a `transfer` record (below), or an
`invalid_runtime_input` refusal naming the first defect's member by its RFC
6901 pointer.

## Behavior

### Request

```json
{
  "format": "filament.planner-settlement/v1",
  "package": "<checked package identity>",
  "item": {"role": "transfer", "node": "<refinement NodeId>",
           "clause": "<clause NodeId>", "subject": "<concrete subject>"},
  "slots": [
    {"role": "refinement", "identity": {}, "result": {}},
    {"role": "abstract-claim", "identity": {}, "result": {}}
  ],
  "limits": {"max_work": 100000}
}
```

- `item` — `role` `transfer`; `node` the refinement declaration's node id;
  `clause` the node id of the infinite-trace clause `P`, whose fairness set
  is `F_P`; `subject` the concrete subject, written as FR-331 writes a
  temporal item's subject (the concrete model's package and its initial
  states).
- `slots` — exactly two, in this order. Slot 0, role `refinement`, is the
  FR-377 refinement item over `subject` for `node`; its slot key is that
  item's FR-492 slot key. Slot 1, role `abstract-claim`, is the claim item
  for `P` under `F_P` over the refinement's abstract subject; its slot key is
  that item's FR-492 slot key without `stutter_closure`, which the planner
  reads from the result to decide the premise form. Each slot carries its
  complete FR-492 premise result.
- A transfer request carries no `guarantees`, `discharges` or
  `environment` slot; `limits` is FR-492's.

Every member is part of the request identity, so the transfer item's
obligation identity binds `P`, `F_P`, the concrete subject, the refinement
node and both premise identities (FR-380 "Kind and reading").

### Admission

The planner SHALL apply FR-492 "Admission" steps 1, 5 and 6 unchanged, and
in place of its steps 2 to 4 and 7 SHALL refuse
`invalid_runtime_input`/`invalid-value` at the first of these defects:

1. `package` not the checked package's identity; `item.node` not a
   refinement declaration node; `item.clause` not an infinite-trace clause
   node of that package (a state clause, a bounded-profile clause or a
   protocol clause): at the member.
2. `slots` not exactly the two slots above, in order, by `role`: at the
   first differing slot.
3. A slot with no `result`: at the slot. A premise identity that names no
   settled item is this defect.
4. A refinement result whose request item is not the refinement item for
   `item.node` over `item.subject`: at `/slots/0/result`.
5. An abstract-claim result whose clause or fairness set is not `P`'s, or
   whose subject is not the refinement's abstract subject (another checked
   package, other initial states, or universes not equal to the concrete
   subject's universes of the populations they are mapped from): at
   `/slots/1/result`, naming the mismatching member (FR-380 "Subject match").

A refusal settles no slot and writes no planner result.

### Settlement

The planner SHALL settle the item by FR-380's settlement table, taking the
first rule that applies in this order, and SHALL NOT use FR-492's decision
order for a transfer item:

1. **Fragment and step-for-step.** The planner decides step-for-step from the
   refinement node's step rows alone (FR-380 "Step-for-step refinements").
   When `P` uses a previous or interval operator and the refinement is not
   step-for-step, the item settles `inconclusive`, `unsettled`,
   `transfer-not-preserving{reason: fragment, operator}`, naming the first
   such operator in clause order.
2. **Fairness implication.** A constraint of `F_P` that no constraint of the
   refinement's `F_A` implies settles `inconclusive`, `unsettled`,
   `transfer-not-preserving{reason: fairness, constraint}`, naming the first
   such constraint in `F_P` order.
3. **Premise form.** A liveness `P` whose abstract-claim result was checked
   without `stutter_closure` settles `inconclusive`, `unsettled`,
   `transfer-not-preserving{reason: stutter-closure}`.
4. **Premises.** The first slot, in slot order, whose result is not `proved`
   settles `inconclusive`, `unsettled`, `premise-unproved{premise, result}`,
   naming the slot index and its FR-331 result value; `unsupported`,
   `incomplete`, `refuted`, `inconclusive` and `failed` premises all settle
   this way (FR-380-AC-5).
5. Otherwise the item settles `proved`, `decisive-witness`, with FR-492's
   certification rule over the two premises, so a premise read from request
   bytes never makes the item `certified`.

The planner's own `max_work` budget, exhausted before a rule decides,
settles `incomplete`, `unavailable`, `limit-reached` naming `max_work`
(FR-492). A transfer item is never `refuted`.

### Transfer record

The planner result of a transfer item SHALL carry, beside FR-492's `item`,
`value`, `basis`, `cause`, `certification` and `slots`, a `transfer`
member:

```json
{
  "refinement": "<refinement NodeId>",
  "clause": "<clause NodeId>",
  "subject": "<concrete subject>",
  "premises": [{"slot": 0, "identity": {}}, {"slot": 1, "identity": {}}],
  "step_for_step": {"holds": false, "row": {"operation": "<concrete operation>", "arm": "stutter"}},
  "fragment": {"class": "position-counting", "operator": "once[1,1]"},
  "fairness": {"implied": true},
  "premise_form": "safety"
}
```

- `step_for_step` — `holds` `true`, or `false` with the first reason in
  step-row order: `row` naming the first concrete operation whose arm is
  `stutter` or `any`, or `{"time": "dense"}` for a `time dense` concrete
  subject, which is reported before any row.
- `fragment` — `class` `stutter-invariant` when `P` is built only from the
  FR-380 fragment's operators, otherwise `position-counting` with the first
  previous or interval `operator` in clause order.
- `fairness` — `implied` `true`, or `false` with the first unimplied
  `constraint`.
- `premise_form` — `safety` for a safety `P`; `stutter-closure` or
  `maximal` for a liveness `P`, by the abstract-claim result's
  `stutter_closure`.
- Every fact is recorded whichever rule decided the item, so a reader sees
  each check FR-380 made.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-820-AC-1 | A transfer request over a step-for-step refinement whose two slots hold proved premise results for FR-380-AC-9's `once[1,1]` clause settles `proved`, `decisive-witness`, with a `transfer` record of `step_for_step.holds` `true`, `fragment.class` `position-counting`, operator `once[1,1]`, `fairness.implied` `true`, `premise_form` `safety` and both premise identities; with both entries read from request bytes as `certified` its certification is `uncertified`. | Test (TC-2000) |
| FR-820-AC-2 | The same request over the compare-and-set refinement settles `inconclusive`, `unsettled`, `transfer-not-preserving{reason: fragment, operator: once[1,1]}`, with `step_for_step.holds` `false` and `row` naming its first `stutter` arm; a refinement with an `any` arm names that row, and one over a `time dense` concrete subject names `{"time": "dense"}`. | Test (TC-2000) |
| FR-820-AC-3 | A third slot, the two slots swapped, a slot with no `result`, a refinement result over another concrete subject, an abstract-claim result over another abstract package, initial state or universe, and an `item.clause` naming a bounded-profile clause each refuse `invalid_runtime_input`/`invalid-value` at the member; no planner result is written. | Test (TC-2000) |
| FR-820-AC-4 | With rule 2's unimplied constraint, the item settles `transfer-not-preserving{reason: fairness}` naming it; a liveness clause whose abstract-claim result lacks `stutter_closure` settles `transfer-not-preserving{reason: stutter-closure}`; with a fragment clause and abstract-claim results `unsupported`, `incomplete` and `refuted` seeded one per run, each settles `inconclusive`, `premise-unproved` naming slot 1 and that result; no transfer item settles `refuted`. | Test (TC-2000) |
| FR-820-AC-5 | A `max_work` budget of zero settles the item `incomplete`, `limit-reached` naming `max_work`; every settled transfer result carries all four facts of the `transfer` record whichever rule decided it. | Test (TC-2000) |

## Dependencies

- [FR-380](../../functional/analysis/FR-380-transfer-claims-through-refinement.md):
  the transfer rule, its fragment, step-for-step, subject-match, fairness and
  premise-form conditions and its settlement table.
- [FR-492](FR-492-planner-settlement-request.md): the request and result
  formats, slot keys, premise results and certification rule this
  requirement extends with the `transfer` role.
- [FR-375](../../functional/analysis/FR-375-declare-model-refinement.md) and
  [FR-377](../../functional/analysis/FR-377-decide-model-refinement.md): the
  step map and the refinement item.
- [FR-331](FR-331-backend-provider-envelope.md),
  [FR-360](../temporal/FR-360-infinite-trace-result-disposition.md) and
  [FR-243](../protocol/FR-243-settlement-basis.md): request items, result
  entries, causes, labels and bases.
