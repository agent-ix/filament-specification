---
id: TC-1680
title: "A hyper clause carries one clock row per quantified trace"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1680: A hyper clause carries one clock row per quantified trace

## Description

Check the `temporal_bindings` rows of a hyper clause's body wrapper: one row
per quantified trace, keyed by the trace variable, each with its own model's
derived configuration, and the reader's refusals of a missing, unknown,
repeated or misplaced `trace`.

Scope: FR-322-AC-453.

## Test Procedure

1. Compile, under event-position, a hyper clause with trace `s` over a model alias selecting `example/device-saving` and trace `n` over one selecting `example/device-normal`; read its `temporal_bindings`.
2. Compile a hyper clause with two traces of one model; read its rows.
3. Read the package of step 1 with the `n` row removed, with the second row's `trace` set to `x`, with a third row repeating `s`, and with a `trace` member added to a single-trace clause's row.

## Expected Results

- Step 1: two rows for the wrapper occurrence, `trace` `s` then `n`, each `clock` an `event_position` configuration whose `sequence_authority` is its model's declaration identity text, both with `clock_name` `model-steps`.
- Step 2: two rows with equal configurations.
- Step 3: `invalid_package`/`timed-node-reference` at the wrapper's node body; at `/temporal_bindings/1/trace`; `invalid_package`/`conflicting-definition` at the repeated row; `timed-node-reference` at the single-trace row's `trace`.
