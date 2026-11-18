---
id: TC-2025
title: "Source clock parameters join the identity rows"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-789
    type: verifies
---

# TC-2025: Source clock parameters join the identity rows

## Description

Verify that a source timed model's `parameter` constraints and resets name `Clock` parameters listed by the identity rows, and that a `parameter` reset's frame names a clock. Scope: FR-789-AC-5. Vectors: `proposals/checked-package-v2/source-timed-model-vectors.json` cases `TM-S3`, `TM-SR6`, `TM-SJ9` and `TM-SJ10`.

## Test Procedure

1. Read and join the FR-789-AC-5 package.
2. Read the package with the `touch` reset added.
3. Join the package with the constraint naming `k`, and with `arm`'s `clock_parameters` empty.

## Expected Results

- Step 1: the package is joined.
- Step 2: the strict read refuses `invalid_package`/`timed-node-reference` at the reset's `/body/members/1/value/target`.
- Step 3: each refuses `invalid_package`/`timed-node-reference` at the constraint's `/body/members/2` at the identity-row join.
