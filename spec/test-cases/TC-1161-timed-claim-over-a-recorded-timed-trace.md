---
id: TC-1161
title: "Timed claim over a recorded timed trace"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-541
    type: verifies
---

# TC-1161: Timed claim over a recorded timed trace

## Description

Settle timed claims over finite and lasso timed traces exactly on rational stamps, and refuse malformed timed traces (FR-541-AC-1 to AC-3).

## Test Procedure

Run each FR-541 acceptance vector through the timed evaluator: the request/acknowledge claim over the two finite traces and the two lassos, the `eventually[0, 1/2]` claim over stamps 0, 1/3 and 0, 2/3, and the three malformed inputs.

## Expected Results

- Finite: `pending` (basis `unsettled`), `violated`; lasso: `holds` at period 200, `violated` at period 400.
- Rational stamps compare exactly: `holds` at 1/3, `violated` at 2/3.
- Decreasing stamps, period 0 and an unstamped trace each refuse naming the defect.
