---
id: TC-1977
title: "Export golden text and optional target oracles"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-818
    type: verifies
---

# TC-1977: Export golden text and optional target oracles

## Description

Verify byte-exact golden comparisons and the optional TLC and Lean oracles. Scope: FR-818-AC-1 to FR-818-AC-4.

## Test Procedure

1. Export every corpus request and compare with its golden files.
2. Run TLC on the counter and the lock, or note its absence.
3. Elaborate the Lean golden files, when Lean is present.
4. Inspect the crate dependencies.

## Expected Results

- Step 1: byte-equal.
- Step 2: equal state counts and verdicts, or `tlc not found` with the golden comparison passing.
- Step 3: no error, no `sorry`.
- Step 4: no target library dependency.
