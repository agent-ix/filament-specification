---
id: FR-828
title: "Read a protocol step's message, delivery and effect identity in an obligation"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-006
    type: implements
  - target: ix://agent-ix/filament-specification/FR-051
    type: implements
  - target: ix://agent-ix/filament-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-431
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-370
    type: references
  - target: ix://agent-ix/filament-specification/FR-093
    type: references
---

# FR-828: Read a protocol step's message, delivery and effect identity in an obligation

## Description

The checker SHALL accept three **step identity reads** in a temporal clause:
`filament::protocol::message(N)`, `filament::protocol::delivery(N)` and
`filament::protocol::effect(N)`, where `N` names one node of a protocol clause
of the same unit. At each position of a protocol behaviour a read is
`some(identity)` when the position's step is a step of `N` that carries that
identity, and `none` otherwise. The reads let an obligation state "each
message is received at most once" or "each charge is applied once" over
every behaviour, using FR-051's identities and never a payload.

## Use case

A payment protocol sends `Charge` on a channel with `delivery at-least-once`
and applies `Paid` after the receive. The verification operator wants two
claims: no delivery is received twice, and no `Paid` occurrence is
established twice. Equal payloads cannot answer either question, because a
redelivered message and a second charge can carry the same amount. The claims
therefore read the identities FR-425 keeps in the state key: the message
occurrence, the delivery index and the effect occurrence key.

## Inputs

A unit that selects `filament.protocol.complete/v1` and
`filament.temporal.complete/v1` and holds a step identity read in a temporal
clause; at evaluation, a protocol behaviour (FR-425, FR-426) or a recorded
protocol execution whose steps carry FR-051 identities.

## Outputs

A checked read carried as one FR-322 `protocol`/`step_read` node, typed
`Option<filament::protocol::Message>`, `Option<filament::protocol::Delivery>`
or `Option<filament::protocol::EffectOccurrence>`; its value at each
position; or a refusal.

## Surface syntax

The three reads are reserved exact qualified intrinsic identities of the
selected `filament.protocol.complete/v1` definition, written with the
ordinary qualified-call production, as the `filament::value::ieee::…`
intrinsics are. A user declaration cannot replace or shadow them.

```ebnf
step-identity-read = ( 'filament::protocol::message'
                     | 'filament::protocol::delivery'
                     | 'filament::protocol::effect' ),
                     '(', protocol-node-ref, ')' ;
protocol-node-ref  = ident, '::', node-ref ;
```

The first `ident` of `protocol-node-ref` names a protocol clause of the unit
and the `node-ref` after it names a node of that clause exactly as the
clause's own controls name it (FR-466), for example `Echo::Rcv` or
`Pay::Main::P::left::Charge`. The argument is a node reference, not a value:
it is never evaluated, and it resolves in the named clause's node scope, not
in the expression scope.

The three identity types are reserved type names usable wherever a
`type-ref` is admitted, so a capture can hold a read's value:
`filament::protocol::Message`, `filament::protocol::Delivery` and
`filament::protocol::EffectOccurrence`.

## Behavior

### Identities and types

- `filament::protocol::Message` is a message occurrence (FR-425): the
  protocol instance, the send node, the sending thread path and the send
  node's occurrence ordinal on that path. Its family is `step_identity`: it
  admits `=` and `!=` only, has no member, no order and no arithmetic, and
  two values are equal exactly when all four components are equal.
- `filament::protocol::EffectOccurrence` is an effect occurrence key
  (FR-425): the protocol instance, the effect node, the thread path and the
  effect node's occurrence ordinal on that path. Its family is
  `step_identity`, with the same rules.
- `filament::protocol::Delivery` is a record with two members, `message` of
  type `filament::protocol::Message` and `index` of type `Integer` (the
  delivery index, at least 0). Two deliveries are equal exactly when their
  messages and indices are equal. `d.index` reads the index and `d.message`
  the message.
- `message(N)` has type `Option<filament::protocol::Message>`,
  `delivery(N)` has type `Option<filament::protocol::Delivery>` and
  `effect(N)` has type `Option<filament::protocol::EffectOccurrence>`.
  `present`, `value` and the comparisons with `none` apply as to every
  `Option` (FR-143).

### Values at a position

At position 0 and at the terminal stutter step every read is `none`. At
position `i > 0`, with step `i` of the behaviour:

| Read | `some(...)` exactly when step `i` is | Value |
| --- | --- | --- |
| `message(N)`, `N` a send node | a `send` step of `N`, including each continuation step of a pending interval send (FR-426 "Interval sends") | the message the step dispatches copies of |
| `message(N)`, `N` a receive node | a `receive` step of `N` | the message of the delivery the step binds |
| `delivery(N)`, `N` a receive node | a `receive` step of `N` | the delivery the step binds: its message and delivery index |
| `effect(N)`, `N` an effect node | an `event` step of `N` (FR-426) | the step's effect occurrence key |

Every other step, including `duplicate`, `lose`, `fork`, `join`, an `attempt`
and a step of another node, gives `none`. The values are the identities the
step's transition identity carries (FR-426), so a read never derives an
identity from a payload, a queue position or a timestamp (FR-051). Over a
recorded execution a read takes the same identities from the observation's
bound FR-051 references: the delivery reference and its related send
occurrence for `message` and `delivery`, and the business-effect binding's
effect occurrence for `effect`.

A read is a value at its position, like the observation a `holds` atom reads
(FR-431 "Positions and anchors"). A capture fixes it at the activation
position (FR-093), so a clause with activation `on each` and a `when` guard
that reads `present(filament::protocol::delivery(Echo::Rcv))` activates at
each receive step of `Rcv` and can capture that delivery and compare it with
the reads at later positions.

### Where a read is legal

A step identity read is legal only inside a `temporal-clause`: in a `holds`
atom, in a `capture` initialiser and in the `when` guard of an `on each`
activation. A read anywhere else (a predicate, a state clause, an operation
body, a protocol control's guard, `check`, `related`, retry or recover
block, a graph clause, a hyper clause, a relation clause, a refinement step
row, a synthesis declaration or a verification plan) refuses
`ill_typed`/`operator-ineligible` at the read. A protocol step exists only at
a position of a protocol behaviour, and those positions are read only by a
temporal clause.

A temporal clause holding a read is checked over a protocol subject of the
protocol the read names. A model-check item, replay or recorded-execution
assessment whose subject is a model subject, or a protocol subject of
another protocol, refuses `invalid_runtime_input`/`invalid-value` naming the
read, before any state is explored, because no position of that subject has
a step of `N` and every read there would be `none`.

### Refusals at check time

- The first `ident` naming no protocol clause of the unit, or a protocol
  offered only by an import, and a `node-ref` naming no node of that clause:
  `missing_declaration`/`missing-name` at the argument.
- `message(N)` whose `N` is not a `send` or `receive` node, `delivery(N)`
  whose `N` is not a `receive` node, and `effect(N)` whose `N` is not an
  `effect` node: `ill_typed`/`operator-ineligible` at the argument, naming
  the node's kind. `delivery` of a send node refuses because one `send` step
  can dispatch several copies (an interval send's `count` step), so it binds
  no single delivery.
- An argument that is not a `protocol-node-ref` (a literal, an arithmetic
  expression, a parenthesised name) and a call with zero or two or more
  arguments: `ill_typed`/`operator-ineligible` at the call.
- A read in a position the previous section excludes:
  `ill_typed`/`operator-ineligible` at the read.
- An ordering, arithmetic or member read on a `step_identity` value (`<`,
  `+`, `m.ordinal`): `ill_typed`/`operator-ineligible` at the application.

No refusal produces a package.

### Carrier and identity

The checked package carries each distinct (read kind, node) pair as one
FR-322 `protocol`/`step_read` node whose body is exactly
`{term: "step-read", field: "message"|"delivery"|"effect", target}`, with
`target` the node id of the named protocol node, and whose `semantic_type` is
the `composite_type`/`option` node over the identity type's
`protocol`/`step_identity_type` node (FR-322 "Protocol step identity
reads"). The field and the target are part of the node's identity; its
occurrences are not. So two reads of `delivery(Echo::Rcv)` in one package
share one node with two `expression` occurrences, and `message(Echo::Rcv)`
and `delivery(Echo::Rcv)` are two nodes. The temporal clause that holds a
read depends on its node, so changing a read's field or target, adding a read
or removing one changes the clause's node, its obligation identity and the
package identity. The protocol clause's own nodes do not depend on the read,
so a read leaves the protocol's identity unchanged.

### Decision: an intrinsic value read, not a new atom or quantifier

The identities already exist in the protocol system: FR-425 keeps them in
the state key and FR-426 puts them in every transition identity. What an
obligation lacked was a way to name them. An `Option` value at each position
fits the existing temporal language with no new operator: `holds` atoms
compare it, `on each` with a `when` guard activates on the steps that carry
it, and a capture freezes it, which is the binder an "at most once" claim
needs. A separate atom kind or an identity quantifier would duplicate that
machinery. The qualified intrinsic spelling follows the value intrinsics, so
no new reserved word enters the expression grammar and a user name never
collides with a read. The argument names a node, not a channel, because the
send or receive node fixes which identity is meant and the channel follows
from it; restricting `delivery` to receive nodes keeps every read
single-valued.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-828-AC-1 | Over protocol `Echo` (channel `ch` `fifo`, capacity 2, `delivery unknown`; `run sequence { send Snd via ch; receive Rcv via ch of Snd }`), the three reads parse as qualified calls with a `protocol-node-ref` argument and check: `filament::protocol::message(Echo::Snd)` and `filament::protocol::message(Echo::Rcv)` to `Option<filament::protocol::Message>`, `filament::protocol::delivery(Echo::Rcv)` to `Option<filament::protocol::Delivery>`, with `value(filament::protocol::delivery(Echo::Rcv)).index` typed `Integer`; over a protocol with `effect Paid of Pay`, `filament::protocol::effect(P::Paid)` checks to `Option<filament::protocol::EffectOccurrence>`. A capture `capture d: filament::protocol::Delivery = value(filament::protocol::delivery(Echo::Rcv));` checks. | Test (TC-2068) |
| FR-828-AC-2 | Refusals, each with no package: `filament::protocol::delivery(Echo::Snd)` (a send node) and `filament::protocol::effect(Echo::Rcv)` refuse `ill_typed`/`operator-ineligible` at the argument naming the node's kind; `filament::protocol::message(Echo::Missing)`, `filament::protocol::message(Nope::Snd)` and a protocol name offered only by an import refuse `missing_declaration`/`missing-name` at the argument; `filament::protocol::message(1)`, `filament::protocol::message()` and `filament::protocol::message(Echo::Snd, Echo::Rcv)` refuse `ill_typed`/`operator-ineligible` at the call; the same read in a predicate, a state clause and a protocol `check` block each refuse `ill_typed`/`operator-ineligible` at the read; `value(filament::protocol::message(Echo::Snd)) < value(filament::protocol::message(Echo::Rcv))` refuses `ill_typed`/`operator-ineligible`. A checker that admits any of these fails this criterion. | Test (TC-2068) |
| FR-828-AC-3 | Values along `Echo`'s behaviour `send(Snd)`, `duplicate`, `receive(Rcv)` of delivery 1: at position 0 every read is `none`; at the `send` position `message(Echo::Snd)` is the message (instance 0, `Snd`, path `(0)`, ordinal 0) and `message(Echo::Rcv)` and `delivery(Echo::Rcv)` are `none`; at the `duplicate` position all three are `none`; at the `receive` position `message(Echo::Rcv)` equals the `send` position's `message(Echo::Snd)` and `delivery(Echo::Rcv)` is that message with index 1. A read that returns the payload or index 0 at the `receive` position fails this criterion. | Test (TC-2068) |
| FR-828-AC-4 | Obligation over `Echo` with a `fifo` channel of `delivery at-least-once`, activation `on each` with `when (present(filament::protocol::delivery(Echo::Rcv)))`, capture `d` of that delivery, and formula `not eventually[1, 8] holds(filament::protocol::delivery(Echo::Rcv) = some(d))`: it holds on every behaviour, since a receive removes the copy it binds. In a variant with `run sequence { send Snd via ch; receive Rcv via ch of Snd; receive Rcv2 via ch of Snd }` and the formula over `message` instead, `not eventually[1, 8] holds(filament::protocol::message(Echo::Rcv2) = some(m))` with `m` captured at `Rcv`, the behaviour that duplicates and then receives both copies refutes it, with the counterexample's two receive identities carrying one message and deliveries 0 and 1. | Test (TC-2068) |
| FR-828-AC-5 | Two `Paid` effect steps in two iterations of a `repeat` with no maximum around `attempt Pay; effect Paid of Pay` give `effect(P::Paid)` values with ordinals 0 and 1, which are not equal; the obligation "no `Paid` occurrence is established twice" (`on each` on `present(filament::protocol::effect(P::Paid))`, capture `e`, `not eventually[1, 8] holds(filament::protocol::effect(P::Paid) = some(e))`) holds. A read whose ordinal does not advance per iteration makes the two values equal and fails this criterion. | Test (TC-2068) |
| FR-828-AC-6 | A model-check item that reads a clause holding `message(Echo::Snd)` over a model subject, and one over a protocol subject of another protocol `Other`, each refuse `invalid_runtime_input`/`invalid-value` naming the read, before any state is explored. | Test (TC-2068) |
| FR-828-AC-7 | Identity: a package holding `delivery(Echo::Rcv)` twice carries one `protocol`/`step_read` node with two `expression` occurrences; adding `message(Echo::Rcv)` adds a second node; changing a read from `delivery` to `message`, or its target from `Rcv` to another receive node, changes the reading clause's node id and its obligation identity, and leaves `Echo`'s `protocol`/`protocol_clause` node id unchanged (FR-322-AC-480). | Test (TC-2068) |

## Dependencies

- [FR-051](FR-051-preserve-communication-identities.md) owns the identities a
  read returns.
- [FR-425](FR-425-define-protocol-subjects-and-state-keys.md) owns the
  message occurrence and effect occurrence key, and
  [FR-426](FR-426-apply-protocol-steps.md) the transition identities that
  carry them.
- [FR-431](FR-431-count-protocol-steps-for-intervals.md) owns positions.
- [FR-093](../temporal/FR-093-bind-temporal-activation-and-captures.md) owns
  activation and captures.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)
  "Protocol step identity reads" owns the carrier.
- FSL FR-894 (evaluation) and FSL FR-205 (the read sets the cycle key keeps).

## References

- Ticket: Linear QSL-2568.
