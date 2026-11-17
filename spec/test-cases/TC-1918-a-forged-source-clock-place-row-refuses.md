---
id: TC-1918
title: "A forged source clock-place row refuses"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-789
    type: verifies
---

# TC-1918: A forged source clock-place row refuses

## Description

Verify that a source clock-place row joining no source row refuses. Scope: FR-789-AC-2.

## Test Procedure

1. Read the FR-789-AC-1 package with each FR-789-AC-2 edit.

## Expected Results

- Step 1: each edit refuses `invalid_package`/`malformed-clock-place` at the edited row.
