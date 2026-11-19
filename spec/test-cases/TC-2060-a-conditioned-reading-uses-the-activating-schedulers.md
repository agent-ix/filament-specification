---
id: TC-2060
title: "A conditioned reading uses the activating schedulers"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-407
    type: verifies
---
# TC-2060: A conditioned reading uses the activating schedulers

## Description

A conditioned probabilistic reading takes its value over the schedulers that
activate with positive probability, and under a workload as the ratio of two
exact probabilities.

Scope: FR-407-AC-18.

## Test Procedure

1. Build the model of FR-407-AC-18: initial state `s0` with action `a`
   (probability `1/4` to a state where `g` first holds and the event then
   holds with probability 1, `3/4` to a state from which `g` is unreachable)
   and action `b` (probability 1 to a state where `g` first holds and the
   event then holds with probability `1/2`).
2. Settle `on each (x: T) when (g) { probability >= θ [E] }` under every
   scheduler at `θ = rational(1, 2)` and `rational(3, 5)`, and the `<=`
   form at `rational(9, 10)`.
3. Settle the `>=` form under a workload choosing `a` and `b` with `1/2`
   each, at `rational(3, 5)` and `rational(2, 3)`.
4. Repeat steps 2 and 3 over the variant in which neither action reaches a
   state where `g` holds.

## Expected Results

- Step 2: minimum `1/2` and maximum `1`; `>= 1/2` holds, `>= 3/5` fails,
  `<= 9/10` fails. A result reading `1/4` or `1/2` as the extremum fails the
  case.
- Step 3: `Pr(act) = 5/8`, `Pr(E and act) = 3/8`, value `3/5`; `>= 3/5`
  holds and `>= 2/3` fails.
- Step 4: each claim holds vacuously, under every scheduler and under the
  workload.
