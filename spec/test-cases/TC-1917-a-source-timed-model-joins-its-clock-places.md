---
id: TC-1917
title: "A source timed model joins its clock places"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-789
    type: verifies
---

# TC-1917: A source timed model joins its clock places

## Description

Verify that the clock places of a source timed model join through source rows. Scope: FR-789-AC-1. Vectors: `proposals/checked-package-v2/source-timed-model-vectors.json` case `TM-S1-source-root`, with the identity rows of `O` and `W`.

## Test Procedure

1. Read the FR-789-AC-1 package and join its identity rows.

## Expected Results

- Step 1: the strict read admits the package and the identity-row join joins the places `d` and `w.c` and their rows.
