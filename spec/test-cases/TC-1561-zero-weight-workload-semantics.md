---
id: TC-1561
title: "Zero-weight workload semantics and provider package intake"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-406 FR-420 FR-411
    type: verifies
---

# TC-1561: Zero-weight workload semantics and provider package intake

## Description

Verify the step probabilities of a workload with weight-0 operations, the tie order of timed races with weight-0 operations, and the exact provider's reward check on a package from another producer.

Scope: FR-406-AC-7, FR-420-AC-8, FR-411-AC-17.

## Test Procedure

1. On `Service` under the workload with `Server::reset: 0`, compute step probabilities at every reachable state and compare them with FR-406-AC-7; repeat with weight 0 on every operation and compare with weight 1 on every operation.
2. On a `time dense` model with `Call::reply` (weight 1) and `Call::retry` (weight 0) racing to a tie, and a variant with two weight-0 operations tying, read the tie order distribution; read the delay families from the package's `delay_distribution` nodes.
3. Give the exact provider a hand-built package whose `reward` node computes `n - 3` over `n: Int[0, 5]`, then one computing `n`.

## Expected Results

- Step 1: `Server::reset` has probability 0 wherever another operation is enabled and 1 where it alone is enabled; the all-zero workload equals the all-one workload; every non-terminal state sums to 1.
- Step 2: `reply` before `retry` with probability 1; two weight-0 operations tie each way with probability `1/2`.
- Step 3: `invalid_probabilistic_model`/`negative-reward` naming the operation, the reward and `n = 0` before any product state; the second package settles.
