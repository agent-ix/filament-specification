---
id: TC-1520
title: "Loop invariant and termination discharge"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-171
    type: verifies
---

# TC-1520: Loop invariant and termination discharge

## Description

Exercise the discharge of a loop's invariant and termination obligations over the explored protocol subject. Scope: FR-171-AC-6.

## Test Procedure

Over a loop with no maximum whose variant decreases each iteration, run the protocol check; repeat with a body that leaves the variant unchanged, with an invariant false at one decision, with a state limit below the reachable state count, and with a variant of Boolean type.

## Expected Results

Both obligations settle `proved` over the complete exploration; the unchanged variant settles termination `refuted` with a replayable counterexample ending at the back edge; the false invariant settles the invariant `refuted`; the limited run settles both `inconclusive` naming the limit; the Boolean variant refuses `ill_typed`/`type-mismatch` at the variant.
