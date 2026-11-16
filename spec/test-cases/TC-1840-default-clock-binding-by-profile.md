---
id: TC-1840
title: "A model claim with no clock member binds its profile's default"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-765
    type: verifies
---

# TC-1840: A model claim with no clock member binds its profile's default

## Description

Check that the default clock binding of a model claim follows its selected
profile, and that a timed-profile claim over an untimed model refuses at its
`using` alias.

Scope: FR-765-AC-1, FR-765-AC-2.

## Test Procedure

1. Over a timed model (`time dense unit ms`), check a claim with no `clock` member under the timed profile, and the same claim with `clock "model-time"`.
2. Check the same claim under the event-position profile with `eventually[0, 3]`, the same with `clock "model-steps"`, and with `eventually[0 ms, 3 ms]`.
3. Over an untimed model, check a claim with no `clock` member under the timed profile and under the infinite-trace profile.

## Expected Results

- Step 1: both bind `model-time` with equal bindings and keys.
- Step 2: the first two bind `model-steps` with equal keys; the third refuses `ill_typed`/`type-mismatch` at the interval.
- Step 3: the timed claim refuses `invalid_timed_model`/`model-time-on-untimed-model` at the `using` alias naming the model; the infinite-trace claim binds `model-steps`.
