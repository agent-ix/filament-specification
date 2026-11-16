---
id: TC-1750
title: "Replay reads its clock configurations from the package and refuses a clocks member"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-715
    type: verifies
---

# TC-1750: Replay reads its clock configurations from the package and refuses a clocks member

## Description

Check that a replay recompiles under the package's own clock bindings,
refuses a request that carries a `clocks` member, and refuses a package that
lacks a library clock selection instead of borrowing a program binding.

Scope: FR-715-AC-1, FR-715-AC-2.

## Test Procedure

1. Replay a request whose package was compiled with a `timed` `plant` clock in `ms`.
2. Replay the same request with a `clocks` member added.
3. Replay a request whose package lock omits a library's dependency clock selection, with a program binding of the same clock name kept.

## Expected Results

- Step 1: the proved verdict is reproduced.
- Step 2: stage `request`, `invalid-request`, exit 20, naming `clocks`, no result.
- Step 3: `missing-clock-selection` naming the clock and the library.
