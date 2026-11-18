---
id: TC-2023
title: "A clock member row refuses at the identity-row join"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-789
    type: verifies
---

# TC-2023: A clock member row refuses at the identity-row join

## Description

Verify that a clock-place row that only the identity rows contradict passes the strict read and refuses at the identity-row join. Scope: FR-789-AC-3. Vectors: `proposals/checked-package-v2/source-timed-model-vectors.json` cases `TM-SJ1` to `TM-SJ5`.

## Test Procedure

1. Read the FR-789-AC-1 package, with each FR-789-AC-3 edit, with no identity row supplied.
2. Join the identity rows of each edited package.

## Expected Results

- Step 1: each edited package is admitted by the strict read.
- Step 2: each refuses `invalid_package`/`malformed-clock-place` at the stated row.
