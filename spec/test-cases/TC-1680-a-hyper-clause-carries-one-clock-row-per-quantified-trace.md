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

Check the `temporal_bindings` rows of a hyper clause: one row per quantified
trace, keyed by its `hyperproperty` node and the trace variable, each with
its own model's derived configuration, no row for the body wrapper, and the
reader's admission of distinct configurations within one occurrence and its
refusals of a missing, unknown, repeated or misplaced `trace`.

Scope: FR-322-AC-453.

## Test Procedure

1. Compile, under event-position, a hyper clause with trace `s` over a model alias selecting `example/device-saving` and trace `n` over one selecting `example/device-normal`; read its `temporal_bindings`.
2. Compile a hyper clause with two traces of one model; read its rows.
3. Read the package of step 2 with the `n` row's `sequence_authority` replaced by a text different from the `s` row's.
4. Read the package of step 2 with the `n` row removed, with the second row's `trace` set to `x`, with a third row repeating `s`, with the two rows written `n` then `s`, with a `trace` member added to a single-trace clause's row, and with a row whose `declaration` is the body wrapper.

## Expected Results

- Step 1: two rows whose `declaration` is the `hyperproperty` node and whose `occurrence` is its `claim` occurrence, `trace` `s` then `n`, each `clock` an `event_position` configuration whose `sequence_authority` is its model's declaration identity text, both with `clock_name` `model-steps`; no row names the body wrapper.
- Step 2: two rows with equal configurations.
- Step 3: admitted; the reader compares no configuration with a model.
- Step 4: `invalid_package`/`timed-node-reference` at the `hyperproperty` node's body; at `/temporal_bindings/1/trace`; `invalid_package`/`conflicting-definition` at the repeated row; `timed-node-reference` at the reordered `/temporal_bindings/1/trace`; `timed-node-reference` at the single-trace row's `trace`; `timed-node-reference` at the wrapper row's `declaration`.
