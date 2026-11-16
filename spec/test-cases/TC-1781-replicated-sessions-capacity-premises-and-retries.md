---
id: TC-1781
title: "Replicated roles pin a session instance, symbolic capacity is a premise, and a retry is a role choice"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-735
    type: verifies
---

# TC-1781: Replicated roles pin a session instance, symbolic capacity is a premise, and a retry is a role choice

## Description

Catches a capacity read from a default or a retry treated as the environment's move.

Scope: FR-735-AC-3, FR-735-AC-4.

## Test Procedure

1. Project a replicated role whose first event on a thread is a send, then a receive; decide realizability over a symbolic capacity with and without a value; decide a compensation whose recovery needs a retry.

## Expected Results

- Step 1: The send-first role yields a counterexample naming the event and two instances, and the receive-first role projects; no value is inconclusive and a value decides; the compensation is realizable with a retrying strategy.
