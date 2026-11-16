---
id: TC-1722
title: "Timed clause forms admit an optional point anchor"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-1722: Timed clause forms admit an optional point anchor

## Description

Timed clause forms admit an optional point anchor. Scope: FR-322-AC-454.

## Test Procedure

Validate `clock_guard` and both `clock_reset` kinds with and without a final `point_anchor` against the V2 schema and the reader, compare the node ids of the two spellings of one guard, and read a `point_anchor` placed early or on another clause form.

## Expected Results

- Both layouts of each form are admitted and the two guard spellings have different node ids.
- A misplaced `point_anchor` or one on another form refuses `invalid_package`/`timed-node-reference` at the binding.
