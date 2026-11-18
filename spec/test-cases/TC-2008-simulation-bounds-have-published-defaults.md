---
id: TC-2008
title: "Simulation bounds have published defaults"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-181
    type: verifies
---

# TC-2008: Simulation bounds have published defaults

## Description

Verify the simulation bounds' published defaults, their settings, their
overrides and their refusals. Scope: FR-181-AC-32.

## Test Procedure

1. Run a `sample` simulation with no bounds and no `seed`; read the reported
   bounds.
2. Run an enumeration with `max_states` 2.
3. Run with `max_steps` 0, with `samples` in `enumerate` mode and with an
   unknown bound `bogus`.

## Expected Results

- Step 1: the defaults and settings of AC-32, `seed` 0.
- Step 2: `incomplete` `limit-reached` naming `explore.states`, value 2.
- Step 3: `invalid-value`; `unknown-member`; `unknown-member`; nothing
  explored.
