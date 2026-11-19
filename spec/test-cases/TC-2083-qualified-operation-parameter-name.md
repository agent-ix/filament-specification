---
id: TC-2083
title: "Qualified operation parameter name reads the invocation's parameter"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-012
    type: verifies
---

# TC-2083: Qualified operation parameter name reads the invocation's parameter

## Description

Verify that a precondition or postcondition reads an operation parameter
through the qualified name `<clause operation>::p` as the same
invocation-supplied value as the bare name, never a same-named field, and that
the qualified form refuses for an unknown parameter, another operation's
parameter and an invariant. Scope: FR-012-AC-9.

## Test Procedure

1. Use FR-012-AC-9's model (`Meter` with field `by: P`, operations
   `adjust(by: P, step: P)` and `reset(by: P)`) and one invocation of `adjust`
   supplying `by = 1.00`, with pre and post snapshots whose `Meter` has field
   `by = -1.00`.
2. Evaluate the precondition and the postcondition
   `M::Meter::adjust::by >= 0.00`, and `M::Meter::adjust::step = step`.
3. Check `M::Meter::adjust::nope >= 0.00` and `M::Meter::reset::by >= 0.00`
   in a precondition of `adjust`, and `M::Meter::adjust::by >= 0.00` in an
   invariant on `M::Meter`.
4. Parse the precondition body `by >= 0.00`.

## Expected Results

- Step 2: each completes `true`.
- Step 3: each refuses `missing_declaration`/`missing-name` at the name.
- Step 4: `invalid_syntax`/`unexpected-token` at `by`.
