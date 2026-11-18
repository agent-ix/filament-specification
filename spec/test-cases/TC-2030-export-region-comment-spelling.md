---
id: TC-2030
title: "The fsl region comment has one exact spelling"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-814
    type: verifies
---

# TC-2030: The fsl region comment has one exact spelling

## Description

Verify the exact spelling of the `fsl:` comment that precedes each exported definition. Scope: FR-814-AC-6.

## Test Procedure

1. Export the counter to TLA+ and to Lean.
2. Match every `fsl:` line against the FR-814-AC-6 pattern, and resolve its node id and region.

## Expected Results

- Step 2: every line matches. Each node id is a package node, and each region's source is the unit's `<authority>/<identity>` with `start < end`, covering the definition's source declaration. A node with no region writes `none`.
