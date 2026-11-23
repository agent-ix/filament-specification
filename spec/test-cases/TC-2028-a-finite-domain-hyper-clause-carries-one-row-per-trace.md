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
the one configuration bound to it. Check that the finite hyper node carries
the profile definition in its typed body member and that the row
configuration is validated against that definition.

Scope: FR-322-AC-464.

## Test Procedure

1. Generate the complete base package from `finite_hyper_profile_fixture` in `temporal-clock-binding-vectors.json` using its source, named FSL emitter test, and generation procedure; check that it has a `claim`/`hyperproperty` node whose body aggregate carries `temporal_profile` and a row for each quantified trace.
2. Apply TCB-14's lock-selection mutation to the base package; then independently apply TCB-15's lock mutation and TCB-16's row-clock mutation to the same base package. Also exercise the existing `b`-row removal, trace `c`, and second-row `clock_name` `u` mutations.

## Expected Results

- Step 1: TCB-14 is admitted with two event-position rows keyed by the `hyperproperty` node and its one `claim` occurrence, `trace` `a` then `b`, each with `clock_name` `t`.
- Step 2: TCB-15 refuses `invalid_package`/`operation-law-unselected` at `/semantic_graph/nodes/{N}/body/temporal_profile`; TCB-16 refuses `invalid_package`/`timed-node-reference` at `/temporal_bindings/0/clock/kind`. The existing missing-row, wrong-trace, and wrong-clock-name mutations refuse at the node's body, that row's `trace`, and `/temporal_bindings/1/clock_name` respectively.
