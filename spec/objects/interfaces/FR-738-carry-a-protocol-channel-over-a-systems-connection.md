---
id: FR-738
title: "Carry a protocol channel over a systems connection"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-465
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-152
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-151
    type: depends_on
---
# FR-738: Carry a protocol channel over a systems connection

## Description

A protocol channel between two roles on Ports MAY name the systems
Connection (FR-152) that its messages travel over. A role is never bound to
a connection: FR-465-AC-12 refuses that, because a connection has two ends
and no single participant. The channel is the protocol construct with two
ends, a direction and a message type, so the channel is what binds to the
connection. A checker and the checked-package reader SHALL apply the rules
below.

## Source form

The channel production gains an optional `via` clause after its `to` role:

```ebnf
channel = 'channel', ident, 'from', ident, 'to', ident, [ 'via', ident ],
          'carries', type-ref, 'ordering', ordering,
          'delivery', delivery-policy,
          'capacity', capacity,
          'overflow', overflow-policy, ';' ;
```

The `via` name resolves among the protocol's `relationship` members.

## Node form

A `protocol`/`channel` node (FR-465 "Channel") has an optional sixth
argument, `connection`: a local reference to the model relationship node
that the clause's `relationships` set holds. It is present exactly when the
source writes `via`. When it is present, the node's dependencies list it.

## Rules

The reader checks a channel's `connection` after FR-465's five channel
checks, in this order:

| Rule | Refusal |
| --- | --- |
| The target is a relationship of the clause's `relationships` set | `invalid_semantic_graph` at the argument |
| The relationship is a Connection | `invalid_model_binding`/`wrong-export` at the channel node, required `connection` |
| `from` and `to` are each roles on a Port (FR-465 "Member") | `invalid_model_binding`/`malformed-declaration` at the channel node, retaining the role |
| The two roles' Ports are the connection's ends in flow order: for `source-to-target`, `from` on the source Port; for `target-to-source`, `from` on the target Port; for `bidirectional`, either assignment | `invalid_model_binding`/`malformed-declaration`, retaining `expected` and `actual` ports |
| `carries` conforms under FR-151 to the effective type of the flow-source Port's interface type | `ill_typed`/`type-mismatch`, retaining `expected` and `actual` |

A connection adds no behaviour to the channel. Ordering, delivery,
capacity and overflow are the channel's own premises (FR-054). A `related
by` clause cannot name a connection, because a connection relates Ports,
not values. A checker refuses it `ill_typed`/`operator-ineligible` at the
relationship name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-738-AC-1 | A channel `via` a source-to-target connection, from the role on its source Port to the role on its target Port, with `carries` conforming to the source Port's interface type, is admitted, and its node holds the `connection` argument and the dependency. | Test (TC-1785) |
| FR-738-AC-2 | Each rule's violation refuses with its code: a target outside `relationships`; a navigation relationship; a role on a Part; the reversed role assignment; and a non-conforming `carries`. | Test (TC-1785) |
| FR-738-AC-3 | A channel with no `via` has no `connection` argument, and its node bytes equal those it had before this argument existed. A `related by` clause naming a connection is refused at the name. | Test (TC-1785) |

## Dependencies

- [FR-465](FR-465-checked-package-protocol-declaration-bodies.md): channel
  and role nodes.
- [FR-152](../../functional/type-model/FR-152-bind-systems-model-structures.md):
  Ports and Connections.
- [FR-151](../../functional/type-model/FR-151-resolve-conformance-redefinition-dispatch.md):
  conformance.
