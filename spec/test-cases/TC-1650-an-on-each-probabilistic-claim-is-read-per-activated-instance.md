---
id: TC-1650
title: "An on-each probabilistic claim is read per activated instance"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-407
    type: verifies
---
# TC-1650: An on-each probabilistic claim is read per activated instance

## Description

An on-each probabilistic claim is read per activated instance.

Scope: FR-407-AC-17.

## Test Procedure

On a vector with two objects of `T`, `g` first true for the first at position 1 and never for the second, settle `on each (x: T) when (g)`; then the same claim over an empty population; then `on each (k: Integer)`.

## Expected Results

- One instance read from position 1, conditioned on activation; one instance that never activates and holds vacuously.
- Empty population: `inconclusive`, `no-instance`.
- `over-not-object-type`.
