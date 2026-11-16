---
id: TC-1723
title: "A non-timed plan names a position-counter unit"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-487
    type: verifies
---

# TC-1723: A non-timed plan names a position-counter unit

## Description

A non-timed plan names a position-counter unit. Scope: FR-487-AC-15.

## Test Procedure

Read a plan root over an event-position `model-steps` clause whose `tick_unit` is the `step` position-counter unit; then read it with a `Time` unit, and a timed clause's root with the `step` unit.

## Expected Results

- The first is admitted with both unit nodes written once and the tick interval `[0, 3]`.
- The other two refuse `invalid_semantic_graph` at the `tick_unit` binding.
