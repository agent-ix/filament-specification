---
id: FR-728
title: "Stop a binder closure at a protocol reference carrier"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-464
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-463
    type: references
---

# FR-728: Stop a binder closure at a protocol reference carrier

## Description

When the reader finds what a protocol expression reads (FR-464 "Binders"),
the dependency closure SHALL stop at a `protocol_reference` carrier, as it
stops at a `function` node. A `protocol Name` reference (FR-463) written in
any protocol slot SHALL therefore be admitted wherever a value of the
reference's type is admitted. This covers a `check` or `send` constraint, an
`each` guard, a `terminal when` condition, a capture initializer and a
`finish` constraint. The reference reads no binder, capture or model state
of the protocol it names.

### Decision: a reference names a protocol, it does not read one

A `protocol_reference` denotes the referenced protocol as a whole, in the
same way a function node denotes a function. The referring expression gets
that identity and never steps into the referenced clause. Its finish binder,
its captures and its `over` input are not in the referring slot's scope.
Walking through the carrier would charge the referring slot with reads that
it does not make, and every protocol body that names a protocol would then be
refused as a model-state read or an out-of-scope read. Stopping the walk at
the carrier also leaves recursion-group handling unchanged when the
reference closes a cycle (FR-464-AC-15).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-728-AC-1 | A protocol `Drain` whose `check` constraint, `each` guard, `terminal when` condition, capture initializer and `finish` constraint each hold `protocol Fill` is admitted, although `Fill`'s clause has a finish binder and reads `over`. No `BinderScope` or `ModelStateRead` refusal is raised for those slots. | Test (TC-1772) |
| FR-728-AC-2 | In the same slots of `Drain`, an expression that reads `Fill`'s finish binder by name, without the carrier, still refuses `missing_declaration`/`missing-name` at the read. A read of `over` beside the reference in a slot that bars model state still refuses `ill_typed`/`operator-ineligible`. | Test (TC-1772) |

## Dependencies

- [FR-464](FR-464-checked-package-protocol-node-bodies.md): binder scope and
  the dependency closure.
- [FR-463](../../functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md):
  the protocol reference.

## Status

Proposed complete-V1 boundary contract.
