---
id: TC-1882
title: "One provider limit set applies to every provider of a run"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-777
    type: verifies
---

# TC-1882: One provider limit set applies to every provider of a run

## Description

Check that `analyze` and `execute` with settlement take one FR-462 limit set and apply it to every provider. Scope: FR-777-AC-1.

## Test Procedure

1. Run each operation with `deadline_ms` 250 and two process providers.
2. Run each with no set.

## Expected Results

- Step 1: every request carries `deadline_ms` 250; both providers held to the frame limit.
- Step 2: FR-462's default deadline.
