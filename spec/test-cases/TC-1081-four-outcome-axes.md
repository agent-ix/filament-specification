---
id: TC-1081
title: "Four independent outcome axes"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-521
    type: verifies
---

# TC-1081: Four independent outcome axes

## Description

Checks the `axes` member on clause runs and analyze items. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| AX-01 | Two clause runs differing only in scope closure | Both carry `axes` with the FR-521 spellings; only `settlement` differs | FR-521-AC-1 |
| AX-02 | A function run; analyze items settling `proved`, `refuted`, `incomplete`, `unsupported` | No `axes` on the function run; the mapped axes on each item | FR-521-AC-2 |

## Expected Results

The axes are independent of each other and of the category.
