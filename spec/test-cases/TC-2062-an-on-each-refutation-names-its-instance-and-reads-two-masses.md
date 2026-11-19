---
id: TC-2062
title: "An on-each refutation names its instance and reads two masses"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-413
    type: verifies
---
# TC-2062: An on-each refutation names its instance and reads two masses

## Description

A path set that refutes an `on each` instance names the instance and counts
its not-activated mass, as a quantile path set does.

Scope: FR-413-AC-10.

## Test Procedure

1. Under FR-407-AC-18's workload, replay a counterexample for
   `on each (x: T) when (g) { probability >= rational(2, 3) [E] }` with
   `instance` naming `x`, one violating path (`b`, then the event false,
   probability `1/4`) and one not-activated path (`a`, then the state from
   which `g` is unreachable, probability `3/8`).
2. Replay the same counterexample without the not-activated path.
3. Replay it with `instance` removed.

## Expected Results

- Step 1: `refuted`, `decisive-counterexample`, since `1/4 > (1/3) · (5/8)`.
- Step 2: `inconclusive`, `replay-parity`, since `1/4` is not above `1/3`.
- Step 3: `invalid_runtime_input`/`missing-member`.
