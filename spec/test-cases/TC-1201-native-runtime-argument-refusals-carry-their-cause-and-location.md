---
id: TC-1201
title: "Native-runtime argument refusals carry their cause and location"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-550
    type: verifies
---

# TC-1201: Native-runtime argument refusals carry their cause and location

## Description

Native-runtime argument refusals carry their cause and location. Scope: FR-550-AC-2.

## Test Procedure

Supply each defective argument FR-550-AC-2 lists, a dangling reference, a population argument naming a supplied binding, and a function whose result type has no FR-181 form.

## Expected Results

Each defect refuses with the code and cause FR-550-AC-2 gives, naming the parameter and the location in the value; the population argument binds; the unsupported result refuses naming the function.
