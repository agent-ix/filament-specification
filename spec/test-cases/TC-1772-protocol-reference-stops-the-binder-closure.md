---
id: TC-1772
title: "A protocol reference in a protocol slot reads nothing of the protocol it names"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-728
    type: verifies
---

# TC-1772: A protocol reference in a protocol slot reads nothing of the protocol it names

## Description

Check that the binder-scope closure stops at a `protocol_reference` carrier.

Scope: FR-728-AC-1, FR-728-AC-2.

## Test Procedure

1. Read `Drain` with `protocol Fill` in its `check` constraint, `each` guard, `terminal when` condition, a capture initializer and its `finish` constraint.
2. Replace each reference with a read of `Fill`'s finish binder, then add a read of `over` beside the reference in a guard.

## Expected Results

- Step 1: admitted, with no `BinderScope` or `ModelStateRead` refusal.
- Step 2: `missing_declaration`/`missing-name` at the read; `ill_typed`/`operator-ineligible` at the `over` read.
