---
id: TC-1915
title: "A source dispatch selection key is admitted"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-788
    type: verifies
---

# TC-1915: A source dispatch selection key is admitted

## Description

Verify that a source dispatch table whose candidate keys are the source declaration keys is admitted. Scope: FR-788-AC-1.

## Test Procedure

1. Read the FR-788-AC-1 package.

## Expected Results

- Step 1: the dispatch table is admitted with candidates `Order` then `Sub`.
