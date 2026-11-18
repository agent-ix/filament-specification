---
id: FR-822
title: "Declare observe-only protocol boundaries and typed recommendations"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-012
    type: implements
  - target: ix://agent-ix/filament-specification/FR-182
    type: references
  - target: ix://agent-ix/filament-specification/FR-052
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
  - target: ix://agent-ix/filament-specification/FR-465
    type: references
  - target: ix://agent-ix/filament-specification/FR-466
    type: references
  - target: ix://agent-ix/filament-specification/FR-433
    type: references
  - target: ix://agent-ix/filament-specification/FR-520
    type: references
  - target: ix://agent-ix/filament-specification/FR-181
    type: references
---

# FR-822: Declare observe-only protocol boundaries and typed recommendations

## Description

When a protocol declares a role or a control observe-only, the protocol
implementation SHALL keep every action behind that boundary free of state
change: the checker refuses each `attempt`, `effect`, `send`, `commit`,
compensation-triggering `event` and compensation template behind it at the
offending action and admits reads, captures and `recommend` controls there, the
checked package records the boundary, and every engine takes each step behind
it without changing the model state or a compensation registration. A
`recommend` control produces a typed recommendation record naming the
operation application it recommends and changes no state wherever it appears.

## Use case

A verification operator models an advisory agent that watches an order
workflow and suggests refunds but must never issue one. They declare the
agent's role observe-only and write its suggestions as `recommend` controls.
Any `attempt` they write for that role by mistake is refused at the attempt,
and every simulation, counterexample and replay shows each suggestion as a
typed record naming the refund operation, its receiver and its arguments.

## Inputs

A protocol clause (shared grammar `protocol-clause`) with its roles and
controls, and, for a run, the run request's `boundary` member (FR-520).

## Outputs

A checked clause whose observe-only roles and controls are recorded, or a
located typed refusal at each offending action; for a run, steps whose
observe-only members leave the model state unchanged and `recommend` steps
that carry their recommendation records.

## Behavior

### Source form

A role member or a control written with the leading word `observe` is
**observe-only** (shared grammar `role` and `control`):
`observe role auditor on Orders::Order;`,
`observe role w each M::Worker from workers max 2 lifetime workflow;` and
`observe sequence Review { ... }`. `observe` is matched as a keyword
spelling only as the first word of a role member or of a control, and remains
an identifier elsewhere, so no existing name changes meaning. `observe` is
written once: `observe observe` is not in the grammar.

- An observe-only **role** marks every action that role performs: an
  `attempt`, `event`, `commit` or `recommend` written `by` it, a `send` on a
  channel whose `from` end it is, a `receive` on a channel whose `to` end it is,
  and a compensation template whose role it is.
- An observe-only **control** marks itself and every control nested in it, at
  any depth, including the `match` event and the `then` and `timeout`
  controls of an `await` and every case, branch and body.

An action is **behind an observe-only boundary** when an observe-only role
performs it or an observe-only control holds it. A control written `observe`
inside another observe-only control, or performed by an observe-only role, is
admitted and changes nothing: the boundary is a property of the action, not a
count of marks.

The boundary is a source construct because the author states it per role or
per region, the checker proves it before any run, and the package carries it
to every consumer. The run-wide mode below composes with it.

### Check

Behind an observe-only boundary the checker SHALL admit `receive`, a domain
`event` with no `for` target, `check`, `choice`, `repeat`, `sequence`,
`parallel`, `await`, `fence` and `recommend`, and the protocol captures and
guards they read. It SHALL refuse each of these with
`ill_typed`/`operator-ineligible`, located at the offending action's own
declaration (its keyword through its terminating `;`), retaining `action` (the
offending form: `attempt`, `effect`, `send`, `commit`, `event-for` or
`compensate`) and `boundary` (the observe-only role or control that places the
action behind the boundary; for an action behind several, the innermost
control, else the role):

- an `attempt`, which applies an operation to the model state;
- an `effect`, which establishes a business effect;
- a `send`, which places a message where another role takes it;
- a `commit`, which closes a compensation's window;
- an `event` with a `for` target, which triggers a compensation
  registration (`action` `event-for`);
- a compensation template whose role is observe-only (`action` `compensate`),
  located at the template declaration, because its attempts apply its
  operation.

Each offending action is refused once, independently, in source order, and a
refusal never moves to the role or the observe-only control. A clause with no
offending action checks exactly as it would with every `observe` removed.

### Recommend

A `recommend N by R on T::op contracts [c1, ...] as (r: B) { constraint };`
control (shared grammar `event-node`) recommends one application of the
operation `T::op` and applies none. It is admitted anywhere an event node is,
behind an observe-only boundary or not. `R` is a declared role, `T::op` and
its `contracts` resolve and check as an `attempt`'s do, and the binder `r` is
bound to the step's recommendation record. Any declared binder type `B` is
admitted; because a recommendation has no operation result, every read of `r`
refuses `ill_typed`/`type-mismatch` at the read, retaining `expected` (no
result) and `actual` (`B`), as a read of an attempt binder over an operation
with no result refuses. A recommend binder is never a decision fact.

A `recommend` step is enabled for one application of the operation (receiver
and argument vector from the finite domains, as FR-426's `attempt`) whose
effective precondition over the contract clauses the control selects is true
over the thread's observation, where the gate holds and the record satisfies
the constraint. Its effect binds the binder and settles the thread; it
computes no post-state, checks no frame, writes no model field, registers no
compensation and changes no queue. One step per application. A precondition
that evaluates undecided gives FR-426's `ContractUndetermined` finding at the
control, as for an `attempt`.

The **recommendation record** is the typed canonical object

`{"type": "protocol-recommendation", "node": <the control's declaration path>,
"role": <the role's qualified name>, "actor": <the acting instance, present
for a replicated role only>, "instance": <the protocol instance ordinal>,
"operation": <the operation's qualified name>, "receiver": <the receiver
reference>, "arguments": <the argument vector in parameter order>}`.

It names the action it recommends and holds no post-state, result or effect.
The step's FR-426 transition identity has `step` `recommend` and carries the
record as its `recommendation` member. The record therefore appears wherever a
protocol step appears: in each step of a simulated behaviour (FR-181), in the
steps of every counterexample and witness trace an analysis returns (FR-520,
FR-331, in FR-433's trace spelling) and in each enabled-step listing. Replay
(FR-433) re-derives each recorded `recommend` step and refuses a trace whose
recorded record differs from the derived one, as for any transition identity.
Every result of a run is a recommendation in FR-182's sense; a `recommend`
record is the one that names a specific action, and no adapter consumes it
except through FR-182's decision boundary.

### Engines

Every engine that steps a protocol (evaluation, simulation, exploration,
analysis and replay) SHALL take a step of an action behind an observe-only
boundary, and every `recommend` step, from a protocol state to a successor
with the same model state (FR-181 `semantic` key member) and the same
compensation registrations. Such a step changes only its own thread's rest
point and binder environment, a `receive` additionally taking the one entry it
delivers from its channel queue (the queue between its two roles, whose
delivery to the receiving role is the observation itself), and a `fork` or
`join` its own parallel instance. An engine that reads a package whose
observe-only boundary holds an offending action refuses it at the reader, by
FR-465 and FR-466, and never steps it.

### Run-wide observe mode

FR-182's observe-or-recommend boundary is the run mode every model run has:
the run applies steps only to the model state it holds and performs no
external effect. A run request MAY in addition select `boundary`
`observe-only` (FR-520). A run in observe-only mode treats every role and the
whole `run` control of each protocol it steps as observe-only: before the
first step it applies the check above to the checked clause as though each
role were written `observe`, and refuses the run with the same typed refusals
at the same actions, each keeping its own code, cause, locus and members,
when any offending action exists; a clause with none runs with every step leaving the model state
unchanged. The default, `model-state`, is FR-182's mode alone. The mode is
part of the run's request identity, so the two modes never share a cached
result.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-822-AC-1 | A clause with `observe role Auditor on Orders::Order;` whose `run` holds `receive Seen via Ch of Placed ...` and `recommend Advise by Auditor on Orders::Order::refund contracts [] as (r: Boolean) { true };` by `Auditor`, and a clause whose `run` holds `observe sequence Review { check Ok using v { true }; recommend Advise by Buyer ...; }`, each check; the same clauses with `observe` removed check to equal clauses except for the recorded boundary; `observe observe sequence ...` fails at the parser with `invalid_syntax`/`unexpected-token` at the second `observe`; a capture named `observe` still checks. | Test (TC-2016) |
| FR-822-AC-2 | Behind `observe sequence Review`, each of an `attempt`, an `effect`, a `send`, a `commit` and an `event Ev by Buyer for Main::Pay` refuses `ill_typed`/`operator-ineligible` located at exactly that action's declaration text, retaining `action` (`attempt`, `effect`, `send`, `commit`, `event-for`) and `boundary` `Review`; the same five actions outside `Review` check. With `observe role Auditor`, an `attempt by Auditor`, a `send` on a channel `from Auditor` and a compensation template `by Auditor` each refuse at that action with `boundary` `Auditor` (`action` `compensate` at the template); a `receive` on a channel `to Auditor`, a `choice by Auditor` and a `check` check. Two offending actions give two refusals in source order and none at the role or the control. | Test (TC-2016) |
| FR-822-AC-3 | Exploring the AC-1 clause from an order with `total = 5` gives `recommend` steps whose transition identity has `step` `recommend` and a `recommendation` record with `type` `protocol-recommendation`, the control's `node`, `role` `Auditor`, `operation` `Orders::Order::refund`, the receiver and the arguments, one step per application whose selected precondition holds; every successor of a `recommend` step has the predecessor's `semantic` key member and compensation registrations; a counterexample through it carries the record in its step, and a replay of that counterexample with the record's `arguments` altered refuses. A read of the recommend binder `r` refuses `ill_typed`/`type-mismatch` at the read. | Test (TC-2017) |
| FR-822-AC-4 | The checked package of the AC-1 clauses carries, on the `Auditor` role node and on the `Review` control node, the trailing argument `boundary` with the text `observe-only`, and on no other role or control node; the role and control nodes of the same clauses without `observe` have no `boundary` argument, so a clause written without `observe` has the same node bodies and node ids as one checked by a reader that knows no boundary; the `recommend` control is a `protocol`/`control` node of operator `recommend` with the FR-466 layout. A reader given that package with an `attempt` control added as a child of `Review` refuses `ill_typed`/`operator-ineligible` at the attempt node, and one given a `boundary` text other than `observe-only` refuses `malformed_wire`. | Test (TC-2018) |
| FR-822-AC-5 | A run of the AC-1 clause takes no step behind `Auditor` or `Review`, and no `recommend` step, whose successor differs from its predecessor in the model state or the compensation registrations, while the same run's `attempt` outside the boundary changes the model field it writes. A run with `boundary` `observe-only` over a clause whose `run` holds an `attempt` refuses before the first step with the AC-2 refusal at the attempt; the same run over a clause with only `receive`, `check` and `recommend` controls runs and every step keeps the model state; the two modes have different request identities. | Test (TC-2019) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns
  the `observe` and `recommend` productions.
- [FR-182](../workflow/FR-182-drive-authorized-effects.md) owns the run-wide
  observe-or-recommend boundary and the decision boundary a recommendation
  crosses before any effect.
- [FR-426](FR-426-apply-protocol-steps.md) owns the step table and the
  transition identity the `recommend` step joins.
- [FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md)
  and [FR-466](../../objects/interfaces/FR-466-checked-package-protocol-control-bodies.md)
  own the role and control layouts that carry the boundary and the
  `recommend` operator.
- [FR-520](../tooling/FR-520-carry-analyze-requests-and-results-as-documents.md)
  owns the run request's `boundary` member.
