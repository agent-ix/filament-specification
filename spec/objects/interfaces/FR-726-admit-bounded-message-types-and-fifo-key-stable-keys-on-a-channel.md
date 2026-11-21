---
id: FR-726
title: "Admit bounded message types and FIFO-key-stable keys on a protocol channel"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-465
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
---

# FR-726: Admit bounded message types and FIFO-key-stable keys on a protocol channel

## Description

The protocol reader SHALL admit a channel whose `carries` target is a
`bounded_domain` node, and SHALL admit a FIFO channel whose `key` type is
FIFO-key-stable. A *FIFO-key-stable* type is an FR-322 equality-stable type
with no `float32` or `float64` leaf at any depth. Every other equality-stable
type is FIFO-key-stable: a bounded scalar or bounded collection (a
`bounded_domain` node, of its base type's family), a `quantity`, `bytes`,
`json_object`, an enum, a reference, and a model-owned
`record_value_type`, `tuple_value_type` or `variant_type` whose leaves are
FIFO-key-stable.

### Decision: a bound is part of the message type, not a reason to refuse it

A source language lowers `Int[0, 9]`, `Text[0, 8; nfc]` and a bounded
`Sequence` to `bounded_domain` nodes, and a protocol author names such a type
as a message type as often as a native one. The bound is a premise every sent
message satisfies; it changes neither how a channel queues messages nor how
the queue is stepped (FR-170). The channel admits the type exactly as written,
so the package keeps the bound for every consumer that checks a send against
it.

### Decision: the FIFO key needs an equivalence, so IEEE floats are out

A FIFO channel orders the messages that carry equal key values (FR-465
"Channel"). That grouping is well defined only when key equality is an
equivalence relation. Every equality-stable type's `eq` identity is one, and a
bounded domain or a quantity compares by its base value, so the reader admits
them. IEEE `numeric_equal` is not reflexive on NaN and equates `+0` and `-0`,
so a NaN key would match no group, its own included; a key with a float leaf
is therefore refused. An `event_type` leaf already makes a type
non-equality-stable (FR-322). A producer's checker SHALL apply this same rule,
so that a key the checker accepts is never refused by the reader.

## Behavior

- FR-465 reader step 2 admits a `bounded_domain` `carries` target. The
  queue, delivery and capacity rules are unchanged.
- FR-465 reader step 3 checks the key type for FIFO-key-stability. It refuses
  a key with an IEEE leaf, or a type that is not equality-stable, with
  `ill_typed`/`operator-ineligible` at the key, retaining the operator `=`
  and the type.
- A key's type is decided over the lowered key type node by a structural walk
  through `option`, collection, record, tuple and variant members to every
  leaf. The walk follows FR-322's equality-stable rule and adds the IEEE
  exclusion. A model-owned `record_value_type`, `tuple_value_type` or
  `variant_type` has no member nodes in the package, so the walk follows the
  fields, positions and payloads the model document declares for it, and a
  float field declared there is a float leaf of the key.
- `proposals/checked-package-v2/fifo-key-type-vectors.json` publishes the
  key types of FR-726-AC-2 and FR-726-AC-3 as the cases `FK-01` to `FK-17`,
  each a lowered key type with, for a model-owned type, the members its
  model document declares.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-726-AC-1 | A channel whose `carries` is each of `Int[0, 5]`, `Text[0, 8; nfc]`, a bounded `Decimal`, a bounded `Rational`, `Float64[nearest-even]` and `Sequence<Boolean>[0, 3]`, lowered to `bounded_domain` nodes, is admitted `unordered`, and the channel body names the `bounded_domain` node as `carries`; a `carries` that names an `expression` node still refuses `invalid_model_binding`/`malformed-declaration` at the target. | Test (TC-1770) |
| FR-726-AC-2 | A FIFO key of each of these types is admitted (`FK-01` to `FK-10`): `Int[0, 99]`, a model value type that is a bounded integer, `Quantity<m>`, `Option<Int[0, 5]>`, `Sequence<Quantity<m>>`, a record with a bounded `Decimal` field, a model `variant_type` and a model `record_value_type` whose leaves are FIFO-key-stable, `Bytes` and `JsonObject`. | Test (TC-1770) |
| FR-726-AC-3 | A FIFO key with a `float32` or `float64` leaf at any depth, or with an `event_type` leaf, refuses `ill_typed`/`operator-ineligible` at the key, retaining `=` and the type (`FK-11` to `FK-17`): `float64`, `Option<float32>`, a record with a `float64` field, a type holding an `event_type` leaf, a record with a `Sequence<float32>` field, and the model-owned `record_value_type` `Reading`, whose field `value: Float64` only the model document declares, alone and inside `Sequence<Option<Reading>>[0, 4]`; the same channels `unordered` are admitted. | Test (TC-1770) |

## Dependencies

- [FR-465](FR-465-checked-package-protocol-declaration-bodies.md): the channel
  body and reader order this requirement extends.
- [FR-322](FR-322-checked-package-artifact.md): equality-stable types and the
  `bounded_domain` node.
- [FR-170](../../functional/protocol/FR-170-evaluate-channel-queue-semantics.md):
  queue semantics, which a bounded message type leaves unchanged.

## Status

Proposed complete-V1 boundary contract.
