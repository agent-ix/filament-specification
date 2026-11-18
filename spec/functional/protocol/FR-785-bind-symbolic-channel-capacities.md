---
id: FR-785
title: "Bind symbolic channel capacities in a protocol subject"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-170
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-465
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-433
    type: references
  - target: ix://agent-ix/filament-specification/FR-175
    type: references
  - target: ix://agent-ix/filament-specification/FR-735
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-272
    type: references
  - target: ix://agent-ix/filament-specification/FR-326
    type: references
  - target: ix://agent-ix/filament-specification/FR-059
    type: references
  - target: ix://agent-ix/filament-specification/FR-737
    type: references
---

# FR-785: Bind symbolic channel capacities in a protocol subject

## Description

When a protocol subject's clause declares a channel whose queue capacity is
`symbolic(<name>)`, the analyzer SHALL take the capacity's value only from the
subject's capacity binding and step the channel at that value. When the name
has no binding, an item whose subject FR-425 classifies SHALL return
`requires-bound`, and a realizability item SHALL settle `inconclusive` with
cause `premise-not-stated`.

## Inputs

A protocol subject ([FR-425](FR-425-define-protocol-subjects-and-state-keys.md))
whose clause has one or more queues with a `symbolic` capacity
([FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md)
"queue"), and the subject's capacity binding.

## Outputs

A protocol system whose symbolic capacities each have their bound value; or,
before any expansion, a typed refusal of the binding, a `requires-bound`
naming every unbound capacity (an item FR-425 classifies), or an
`inconclusive` `premise-not-stated` result (a realizability item).

## Behavior

### The binding

A clause's **capacity names** are the `name` members of its queues' `symbolic`
capacities. Two queues that name the same capacity share one name and one
value. The **capacity binding** `capacities` is a map from capacity name to a
natural number, written as canonical decimal text (`"0"`, `"2"`, no sign and
no leading zero). It is a member of the protocol subject, like the static role
bindings, and is supplied by the request with the subject's other bindings
(the in-process subject binding of FR-331). It is not package content: the
package carries the name and no value (FR-465), and analysis bounds are
request members ([FR-326](../../objects/interfaces/FR-326-checked-protocol-artifact.md)).
The binding is not an FR-331 `ProofBound`. It does not enter the item's
`extent`, and it does not change the extent's classification.

A request that replays a supplied finite trace over the protocol subject, or
that assesses a recorded execution against it
([FR-059](FR-059-assess-finite-global-conformance.md), the
[FR-737](../../objects/interfaces/FR-737-protocol-execution-recording.md) recording),
supplies the binding the same way, with that subject's other bindings. The
trace and the recording carry no capacity value. The recording's channel
obligations read each symbolic capacity at its bound value, and a name with
no binding is an unbounded root of that request, as above.

### Admission

Before any expansion, and before FR-425's classification of the subject's
roots, the analyzer SHALL refuse the following, for every item over the
subject, a realizability item included:

- an entry whose name is not a capacity name of the clause, with
  `invalid_runtime_input`/`unknown-member` naming the entry;
- an entry whose value is not canonical decimal text of a natural number,
  with `invalid_runtime_input`/`invalid-value` naming the entry;
- a name bound twice, with `invalid_runtime_input`/`duplicate-member` naming
  the second entry.

So a binding defect refuses even when another capacity name is unbound: it
is never reported as `requires-bound` or `premise-not-stated`.

A capacity name with no entry is an unbounded root of the subject. For an item
whose subject FR-425 classifies (a simulation, model-check, temporal,
refinement or derived item over the protocol subject), FR-425's classification
SHALL return the FR-181 `requires-bound` disposition before any expansion,
naming each unbound capacity by its name and the qualified name of each
channel whose queue names it.

A realizability item is the one exception. FR-425's root classification does
not run for it. Its unbound capacity is a premise recorded as not stated
("Realizability" below), and the item settles `inconclusive` with cause
`premise-not-stated`, never `requires-bound`.

No analyzer SHALL substitute a default value, a literal of another channel, a
horizon or a search limit for an unbound capacity.

### Stepping

A queue whose capacity names a bound name SHALL step exactly as the same queue
with a `literal` capacity of the bound value: the FR-170 queue transitions,
the [FR-426](FR-426-apply-protocol-steps.md) steps, the FR-425 state key and
the [FR-429](FR-429-report-protocol-deadlocks.md) wait causes are those of the
literal. A bound value of 0 is admitted, as a literal capacity 0 is.

A symbolic capacity is never an `unsupported` result. A simulation, model
check, finite replay or derived item over the subject SHALL NOT settle it
`unsupported` with any FR-331 cause or with the FR-272
`unsupported_construct`/`not-yet-implemented` diagnostic. The binding
discharges it, and without one the item returns `requires-bound`. The
binding adds no decision method. Each method that FR-181, FR-363 or FR-391
defines decides the subject as it decides the same subject with literal
capacities.

### Identity

The capacity binding is part of the protocol subject, and so of every
obligation identity that reads the subject (FR-425), as the static role
bindings are. Two subjects that differ only in one bound value are different
subjects with different obligation identities. A result recorded for one is
not reused for the other.

### What a result claims

A result over the subject holds at the bound values only.

- A `proved` result claims its property for the protocol with each capacity
  name equal to its bound value, and for no other value. Its label is the
  capacity binding held in the subject of its obligation identity; no FR-331
  result entry carries a subject, so the label lives there and nowhere else.
  The result is never stated without that label.
- A `refuted` result's counterexample is a behaviour of the protocol at the
  bound values. The counterexample stands on its own as that behaviour.
- No result is extended to another value, to every value, or by any
  monotonicity in capacity.

### Counterexamples and replay

An [FR-433](FR-433-replay-protocol-counterexamples.md) counterexample over the
subject carries the binding as the `capacities` member of its `subject`,
present exactly when the clause has a capacity name. Replay re-admits the
binding with the subject's other bindings and re-executes the steps at those
values, so FR-433's rules settle a counterexample whose values were changed.

### Realizability

A realizability item ([FR-175](FR-175-analyze-protocol-realizability.md),
[FR-735](FR-735-project-and-realize-every-protocol-construct.md)) takes each
symbolic capacity's channel premise from the subject's binding. The premise
carries the name and the bound value, so the value enters the realizability
key and the witness identity (FR-175-AC-6). The game steps the queue at that
value, and its result holds at that value only, as above.

With no binding, the premise is recorded as not stated: it carries the name
and the marker that its value is not stated. So the key member is present,
and FR-175's `invalid_runtime_input`/`missing-member` refusal of an absent
key member does not apply. The item then settles `inconclusive` with cause
`premise-not-stated` (FR-175-AC-3, FR-735-AC-4), never `requires-bound` and
never a default capacity. A projection reads no capacity (FR-735).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-785-AC-1 | Over a clause whose channel `Order` has capacity `symbolic(Bound)` and overflow `block`, and whose run is `send P via Order; send Q via Order; receive RP of P; receive RQ of Q`, the subject with `capacities` `{"Bound": "1"}` waits at the second send. Its deadlock is reported `full-queue` naming the channel's qualified name. With `{"Bound": "2"}` both sends queue and the run finishes. Each subject's state keys and transition identities equal those of the same clause written with `capacity 1` and `capacity 2`. A second channel whose capacity is also `symbolic(Bound)` takes the same value: at `"2"` it queues two messages and its queue's state keys equal those of a `capacity 2` literal. | Test (TC-1911) |
| FR-785-AC-2 | The AC-1 subject with no `capacities` entry for `Bound` returns `requires-bound` before any state is expanded. It names `Bound` and the qualified name of each channel whose queue names it, and settles no `unsupported` result and no `unsupported_construct`/`not-yet-implemented` diagnostic. A clause with two unbound capacity names names both. | Test (TC-1911) |
| FR-785-AC-3 | Before any expansion and before root classification: an entry `{"Other": "1"}` that names no capacity of the clause refuses `invalid_runtime_input`/`unknown-member`, and so does `{"Other": "1"}` with `Bound` unbound, which never returns `requires-bound`; the values `"-1"`, `"01"`, `"1.0"` and `"x"` each refuse `invalid_runtime_input`/`invalid-value`; and a name bound twice refuses `invalid_runtime_input`/`duplicate-member`. Each refusal names its entry. The value `"0"` is admitted. | Test (TC-1911) |
| FR-785-AC-4 | The AC-1 subjects at `"1"` and `"2"` have different obligation identities. The deadlock-freedom item settles `refuted` at `"1"` and `proved` at `"2"`. The subject of the `proved` result's obligation identity holds the capacity binding `{"Bound": "2"}`, and no result at one value is reused for the other. | Test (TC-1911) |
| FR-785-AC-5 | The `refuted` counterexample at `"1"` carries `subject.capacities` `{"Bound": "1"}` and replays to `reproduced-with-evaluated-witness`. The same counterexample with `capacities` `{"Bound": "2"}` settles `inconclusive`, cause `replay-parity`, because its last state enables a step. The counterexample without `capacities` refuses before replay `invalid_runtime_input`/`missing-member` at `subject.capacities`, and a counterexample with `capacities` over a clause that has no capacity name refuses before replay `invalid_runtime_input`/`unknown-member` at `subject.capacities`. | Test (TC-1911) |
| FR-785-AC-6 | A realizability item over the AC-1 clause with `{"Bound": "2"}` is decided, and its channel premise carries `Bound` and `2`. The same item at `"1"` has a different witness identity. With no binding the item's key holds the channel premise with `Bound` recorded as not stated, is not refused `missing-member`, and settles `inconclusive`, cause `premise-not-stated`, never `requires-bound`. | Test (TC-1911) |
| FR-785-AC-7 | A finite replay of a supplied trace over the AC-1 clause, and the assessment of a recorded execution of it, take the binding from the request's subject: with `{"Bound": "1"}` the trace that sends `P` then `Q` declines at the second send, as over `capacity 1`, and the recording that shows both sends queued violates the channel obligation; with no binding each returns `requires-bound`, never `unsupported`. | Test (TC-1911) |

## Dependencies

- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) owns the protocol
  subject, its bindings and the classification of its roots.
- [FR-170](FR-170-evaluate-channel-queue-semantics.md) owns the queue
  transitions that a bound capacity steps.
- [FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md)
  owns the `symbolic` capacity member and its `name`.
- [FR-433](FR-433-replay-protocol-counterexamples.md) owns the counterexample
  wire and replay.
