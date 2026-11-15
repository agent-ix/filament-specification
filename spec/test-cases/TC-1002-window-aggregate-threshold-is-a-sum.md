---
id: TC-1002
title: "Window-aggregate threshold is a sum"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-501
    type: verifies
---

# TC-1002: Window-aggregate threshold is a sum

## Description

Verify that the threshold of a window-aggregate atom is a `sum`, so a following Boolean connective joins temporal operands (FR-501 item 1). Scope: FR-501-AC-1.

## Test Procedure

Parse the three temporal bodies of FR-501-AC-1 inside a temporal clause over `(s: M::S)`.

## Expected Results

The first parses as one `temporal-conjunction` of the atom (threshold `1`) and `holds(s.done)`; the second parses with threshold `s.k + 1`; the third refuses `invalid_syntax`/`unexpected-token` at the `s` after `and`.
