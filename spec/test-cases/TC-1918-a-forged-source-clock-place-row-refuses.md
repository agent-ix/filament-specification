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

Verify that a source clock-place row joining no source row refuses. Scope: FR-789-AC-2. Vectors: `proposals/checked-package-v2/source-timed-model-vectors.json` cases `TM-SR1` to `TM-SR5`.

## Test Procedure

1. Read the FR-789-AC-1 package with each FR-789-AC-2 edit, with no identity row supplied.

## Expected Results

- Step 1: the strict read refuses each edit `invalid_package`/`malformed-clock-place` at the edited row.
