---
id: TC-2019
title: "Engines never advance state through an observe-only step"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-822
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-182
    type: verifies
---

# TC-2019: Engines never advance state through an observe-only step

## Description

Verify that every engine keeps the model state through observe-only and
`recommend` steps, and the run-wide observe mode. Scope: FR-822-AC-5 and
FR-182-AC-7.

## Test Procedure

1. Simulate and explore a clause with an observe-only role that receives and
   recommends an operation writing a model field, beside an `attempt` of the
   same operation by another role, under the default run mode.
2. Run, with `boundary` `observe-only`, the same clause, then a clause holding
   only `receive`, `check` and `recommend` controls.

## Expected Results

- Step 1: no step behind the boundary and no `recommend` step changes the
  model state or the compensation registrations; the `attempt` changes the
  field; no adapter request is issued.
- Step 2: the first run refuses before any step with the FR-822-AC-2 refusal
  at the `attempt`; the second runs and every step keeps the model state; the
  two modes have different request identities.
