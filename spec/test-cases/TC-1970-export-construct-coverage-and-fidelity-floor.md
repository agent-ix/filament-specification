---
id: TC-1970
title: "Export construct coverage and the fidelity floor"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-811
    type: verifies
---

# TC-1970: Export construct coverage and the fidelity floor

## Description

Verify located unrepresented causes, the fidelity floor and closure coverage. Scope: FR-811-AC-1 to FR-811-AC-5.

## Test Procedure

1. Export an invariant that normalizes text.
2. Export it with a preserved obligation under the default floor.
3. Repeat under `minimum_fidelity` `any`.
4. Export a dense-time clause under `preserved` and under `conditional`.
5. Export the whole conformance corpus and check closure coverage.

## Expected Results

- Step 1: `unrepresented` with `target-unrepresentable` naming the node and region.
- Step 2: `below-fidelity-floor`, no package or file.
- Step 3: a package; the failing record claims no region.
- Step 4: fails, then passes.
- Step 5: every closure node covered by a region or a cause.
