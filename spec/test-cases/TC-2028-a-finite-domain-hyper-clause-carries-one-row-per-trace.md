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
the one configuration bound to it, checked against the lock's
`temporal_profile` selection.

Scope: FR-322-AC-464.

## Test Procedure

1. Compile `hyper Det using ep over t in Obs bounded 3 forall trace a forall trace b { (a.in = b.in) -> (a.out = b.out) }` with role `t` bound to `{"kind": "event_position", "sequence_authority": "obs"}`; read its rows and read the package.
2. Read the package with the `b` row removed, with a row's `trace` set to `c`, with the second row's `clock_name` set to `u`, and with a row's `clock` replaced by a `fixed_sample` configuration.

## Expected Results

- Step 1: two rows keyed by the `hyperproperty` node and its one `claim` occurrence, `trace` `a` then `b`, each with `clock_name` `t` and the bound configuration; the package is admitted.
- Step 2: `invalid_package`/`timed-node-reference` at the node's body; at that row's `trace`; at `/temporal_bindings/1/clock_name`; at that row's `clock/kind`.
