---
id: TC-1975
title: "Target syntax versioning through the profile"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-816
    type: verifies
---

# TC-1975: Target syntax versioning through the profile

## Description

Verify the emitted syntax subsets, target references and observer independence. Scope: FR-816-AC-1 to FR-816-AC-3.

## Test Procedure

1. Scan the corpus outputs' constructs.
2. Change a target reference.
3. Observe one package with two Lean releases.

## Expected Results

- Step 1: every construct within the profile's subset.
- Step 2: profile equality changes.
- Step 3: one package key, two observer records.
