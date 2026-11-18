---
id: TC-2029
title: "An export refuses only what the target cannot state"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-811
    type: verifies
---

# TC-2029: An export refuses only what the target cannot state

## Description

Verify that a construct the target can represent is lowered, and that `target-unrepresentable` is the only cause that makes an obligation `unrepresented`. Scope: FR-811-AC-6.

## Test Procedure

1. Export the counter's invariant `Bounded` (`value <= 5`) under `filament.output.tla2/v1` and `filament.output.lean4/v1`.
2. Export `SlowEnough` over the hybrid `Braking` under `minimum_fidelity` `any`.

## Expected Results

- Step 1: both records are `preserved` with no cause.
- Step 2: one `unrepresented` record with one `target-unrepresentable{target, construct, node, locus}` cause per flow-equation node in node order. Each cause names the family, the node's `node_tag`/`semantic_form`, a node id of the package and that node's source region. No other cause appears.
