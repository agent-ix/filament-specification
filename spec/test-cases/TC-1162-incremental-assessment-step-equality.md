---
id: TC-1162
title: "Incremental assessment step equality"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-114
    type: verifies
---

# TC-1162: Incremental assessment step equality

## Description

Check that an incremental assessment equals batch assessment after every observation, keeps settled results, leaves state unchanged on a refused observation, and resumes from a checkpoint (FR-114-AC-5).

## Test Procedure

Feed a bounded, an infinite-trace and a safety obligation five observations one at a time; after each, run the batch assessment over the prefix. Insert an out-of-order observation. Checkpoint after observation 2, resume, and resume under another package.

## Expected Results

- Every step equals the prefix batch result; the settled violation is unchanged afterwards.
- The out-of-order observation is refused and the next observation gives the uninterrupted result.
- The resumed assessment gives the same later results; the foreign checkpoint is refused.
