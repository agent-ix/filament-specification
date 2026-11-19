---
id: TC-2085
title: "A model declaration node takes its selecting occurrence's region"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2085: A model declaration node takes its selecting occurrence's region

## Description

Verify that every model declaration node carries one `generated` occurrence
at ordinal 0 whose `source_map` regions are those of its identity's
`model_root` entry, and that the reader refuses each other shape.
Scope: FR-322-AC-485 and FR-322-AC-486.

## Test Procedure

1. For each published positive package, read every model declaration node's
   `occurrences`, its `source_map` entries and the `model_root` entry of its
   owner identity; run the reader's model declaration occurrence check.
2. Apply each `model-tuple-union-vectors.json` wire case `TW-29` to `TW-34`
   to `positive-model-tuple-union.json` and read the result strictly.

## Expected Results

- Step 1: each node has the occurrence and the one entry FR-322-AC-485
  states, with the root's regions, and the reader admits every package.
- Step 2: `TW-29` to `TW-33` refuse `invalid_package`/`invalid-value` at the
  locus FR-322-AC-486 states; `TW-34` is admitted.
