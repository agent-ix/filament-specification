---
id: TC-2024
title: "Source tuple and union rows join their identity rows"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-789
    type: verifies
---

# TC-2024: Source tuple and union rows join their identity rows

## Description

Verify that the `tuple` and `union` rows of a source timed model join the identity rows of the native tuple and union. Scope: FR-789-AC-4. Vectors: `proposals/checked-package-v2/source-timed-model-vectors.json` cases `TM-S2` and `TM-SJ6` to `TM-SJ8`.

## Test Procedure

1. Read and join the FR-789-AC-4 package.
2. Read and join it with each forged place.

## Expected Results

- Step 1: the places `p`/`tuple(0)` and `u`/`union(armed)` are joined with the stated rows.
- Step 2: each forged place passes the strict read and refuses `invalid_package`/`malformed-clock-place` at row `1` at the identity-row join.
