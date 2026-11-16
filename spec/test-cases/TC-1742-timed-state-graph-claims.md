---
id: TC-1742
title: "State-graph claims over a timed model subject"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-712
    type: verifies
---

# TC-1742: State-graph claims over a timed model subject

## Description

Check the three state-graph forms over TC-369's timed `Rpc` vector: timed
reachability, the `E(P)` fixpoint, path counting that ignores delays, exact
rational evidence, and routing that excludes untimed backends.
Scope: FR-712-AC-1 to FR-712-AC-4.

## Test Procedure

1. Check `possible` of `phase = Replied` and of `phase = TimedOut` at `T = 4 ms`.
2. Check `always possible` of `phase = Idle`, then over the model without `reset`.
3. Check `unique path` from `phase = Idle` to `phase = Replied` with and without `reset`.
4. Name a backend listing the forms that offers neither timed method; repeat step 1 with `options.method` `digital-clocks`.

## Expected Results

- Step 1: holds with a replayed `send`, `reply` witness and a delay in `[1, 3]`; then does not hold.
- Step 2: holds with the `state_graph` part's `ef_nodes` in the certificate; then refuted ending at `phase` `Replied`.
- Step 3: refuted by the `reset` cycle; holds without `reset`.
- Step 4: `unsupported` `unsupported-requested-capability`; with `digital-clocks`, the same verdicts with basis `exhaustive`.
