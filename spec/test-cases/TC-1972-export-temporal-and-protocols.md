---
id: TC-1972
title: "Export encodings of temporal clauses, fairness and protocols"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-813
    type: verifies
---

# TC-1972: Export encodings of temporal clauses, fairness and protocols

## Description

Verify direct and monitor-based temporal encodings, step intervals under stuttering, fairness, protocols and proof-free Lean output. Scope: FR-813-AC-1 to FR-813-AC-5.

## Test Procedure

1. Export `always holds(c.value <= 5)`.
2. Export an `until` clause.
3. Export `always[0, 3] holds(c.ok)` and insert stuttering steps into a behaviour.
4. Export weak fairness and a two-role `fifo` protocol.
5. Inspect the Lean files.

## Expected Results

- Steps 1 to 4: the stated encodings, all `preserved`; the verdict unchanged by stuttering.
- Step 5: no `theorem`, proof or `sorry`.
