---
id: TC-1810
title: "A unit of only dimensions and units compiles"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-746
    type: verifies
---

# TC-1810: A unit of only dimensions and units compiles

## Description

Verify that a declaration that has a node is written when nothing names it. Scope: FR-746-AC-1.

## Test Procedure

Under the header `language "ix:native" edition "1-draft";` with `profile v = "filament.value.complete/v1";`, check and compile a unit whose only declaration is `dimension Length;`, then a unit of `dimension Length;` and `unit m : Length = rational(1, 1);`. Read each package with the strict reader.

## Expected Results

- Both units check and compile.
- The first package holds exactly one source `scalar_type`/`dimension` node `Length` with `base: true`.
- The second holds that dimension node and one `scalar_type`/`unit` node `m` whose `dimension_node_id` is the dimension node.
- The strict reader admits both packages.
