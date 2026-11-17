---
id: TC-1916
title: "A foreign source selection key refuses"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-788
    type: verifies
---

# TC-1916: A foreign source selection key refuses

## Description

Verify that every other spelling of a source candidate's selection key refuses. Scope: FR-788-AC-2.

## Test Procedure

1. Read the FR-788-AC-1 package with each FR-788-AC-2 edit of the first candidate's key.

## Expected Results

- Step 1: each edit refuses `invalid_package`/`invalid-value` at the candidate's `selection_key`.
