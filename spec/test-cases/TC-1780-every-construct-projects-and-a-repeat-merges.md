---
id: TC-1780
title: "Every protocol construct projects, and a repeat decision point merges or yields a counterexample"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-735
    type: verifies
---

# TC-1780: Every protocol construct projects, and a repeat decision point merges or yields a counterexample

## Description

Catches a projector that answers a construct non-conclusively for want of a rule.

Scope: FR-735-AC-1, FR-735-AC-2.

## Test Procedure

1. Request projection over one protocol per construct of FR-735's list; then over a `repeat` whose continue and exit give a non-owner role the same next action with no evidence.

## Expected Results

- Step 1: Every request returns a local graph, a counterexample or an exhausted-bound result, none naming the construct; the `repeat` yields a counterexample whose decision point is the `repeat`, owned by its `by` role.
