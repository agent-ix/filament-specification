---
id: TC-2006
title: "A schedulability miss is a closed counterexample"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-419
    type: verifies
---

# TC-2006: A schedulability miss is a closed counterexample

## Description

Verify the `schedulability-miss` counterexample spelling of a refuted
schedulability item and its reader refusals. Scope: FR-419-AC-13.

## Test Procedure

1. Analyze `Ctl` with the third WCET 6 under `fixed_priority` and under `edf`.
2. Read the fixed-priority counterexample with each defect of AC-13.

## Expected Results

- Step 1: the `response` miss for task `c`, job 0, iterates 6, 10, 13,
  response 13, bound 12, unit `ms`; the `utilization` miss 13/12.
- Step 2: each defect refused `invalid_result` with its cause, naming the
  member.
