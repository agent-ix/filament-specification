---
id: TC-2028
title: "A finite-domain hyper clause carries one clock row per trace"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-191
    type: verifies
---

# TC-2028: A finite-domain hyper clause carries one clock row per trace

## Description

Check the `temporal_bindings` rows of a hyper clause over a declared finite
trace population: one row per quantified trace, keyed by its
`hyperproperty` node, each carrying the domain binder as its clock role and
the one configuration bound to it. Check that the row configuration is
validated against the finite hyper clause's `filament.op.claim.clause`
`temporal_profile` law even when the lock contains multiple temporal profile
selections.

Scope: FR-322-AC-464.

## Test Procedure

1. Read `TCB-14` from `temporal-clock-binding-vectors.json`: its package lock has temporal `profile_selections` `[T, D]` in that order, where `D` is `event-position.false-extension/v1`; the hyper clause is `hyper Det using ep ...` and its application law selects `D`.
2. Read the same package with its event-position row replaced by the `fixed_sample` row in TCB-14; also exercise the existing `b`-row removal, trace `c`, and second-row `clock_name` `u` mutations.

## Expected Results

- Step 1: the package is admitted with two event-position rows keyed by the `hyperproperty` node and its one `claim` occurrence, `trace` `a` then `b`, each with `clock_name` `t`. Its row kind is checked against the application law selecting `D`, not lock-array entry `T`.
- Step 2: the fixed-sample mutation refuses `invalid_package`/`timed-node-reference` at `/temporal_bindings/0/clock/kind`; the existing missing-row, wrong-trace, and wrong-clock-name mutations refuse at the node's body, that row's `trace`, and `/temporal_bindings/1/clock_name` respectively.
