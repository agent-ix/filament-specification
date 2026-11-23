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

1. Generate the complete base package from `finite_hyper_profile_fixture` in `temporal-clock-binding-vectors.json` using its source, named FSL emitter test, and generation procedure; assert that the `claim`/`hyperproperty` node's body aggregate omits `temporal_profile` and that the package has a row for each quantified trace.
2. Apply TCB-14's mutations to that base: add the typed `temporal_profile` definition and set the lock selections to `[T, D]`. This accepted package is the common starting point for TCB-15 and TCB-16.
3. Independently derive TCB-15 from accepted TCB-14 by changing only the lock selections to `[T]`; derive TCB-16 from accepted TCB-14 by changing only trace `a`'s row clock to `fixed_sample`. Also exercise the existing `b`-row removal, trace `c`, and second-row `clock_name` `u` mutations.
4. Apply TCB-17 to TCB-24 from `finite_hyper_profile_cases`: remove the member (TCB-17); select `D` twice in the lock (TCB-18); add the member to a `behaviours`-domain hyperproperty (TCB-19) and to a non-hyperproperty node (TCB-20); and set the member to a non-object, an object with an extra field, an empty `authority` and an empty `identity` (TCB-21 to TCB-24).

## Expected Results

- Step 1: The generated base omits `temporal_profile`. TCB-14 is admitted after adding the typed definition and `[T, D]` selections; its two event-position rows are keyed by the `hyperproperty` node and its one `claim` occurrence, `trace` `a` then `b`, each with `clock_name` `t`.
- Step 2: TCB-15 refuses `invalid_package`/`operation-law-unselected` at `/semantic_graph/nodes/{N}/body/temporal_profile`; TCB-16 refuses `invalid_package`/`timed-node-reference` at `/temporal_bindings/0/clock/kind`. The existing missing-row, wrong-trace, and wrong-clock-name mutations refuse at the node's body, that row's `trace`, and `/temporal_bindings/1/clock_name` respectively.
- Step 4: TCB-17 and TCB-18 refuse `invalid_package`/`operation-law-unselected` at `/semantic_graph/nodes/{N}/body/temporal_profile`; TCB-19 and TCB-20 refuse `invalid_package`/`invalid-value` at the member's pointer; TCB-21 to TCB-24 refuse `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{N}/body/temporal_profile`.
