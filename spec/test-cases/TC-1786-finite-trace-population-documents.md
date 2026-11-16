---
id: TC-1786
title: "A finite trace population is read, bounded and aligned in lockstep"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-739
    type: verifies
---

# TC-1786: A finite trace population is read, bounded and aligned in lockstep

## Description

Catches a past-end read compared as a value, or an open population that proves.

Scope: FR-739-AC-1, FR-739-AC-2.

## Test Procedure

1. Read a closed population of three traces, each malformed variant, and four traces under `bounded 3`; evaluate a read past a trace's end; evaluate an open population with and without a violating tuple.

## Expected Results

- Step 1: The population reads and each defect refuses at its pointer; `requires-bound` naming 3; the past-end tuple is incomplete; the open population is inconclusive, then refuted.
