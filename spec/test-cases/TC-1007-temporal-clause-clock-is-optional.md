---
id: TC-1007
title: "Temporal clause clock is optional"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1007: Temporal clause clock is optional

## Description

Verify that a complete-V1 temporal clause may omit `clock` and then binds the FR-252 default (FR-501 item 6). Scope: FR-501-AC-6.

## Test Procedure

Compile the clause of FR-501-AC-6 over an untimed model with no `clock`, then with `clock "model-steps"`.

## Expected Results

Both parse; both bind `model-steps`; the binding and the obligation identity are equal.
