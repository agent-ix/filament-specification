---
id: TC-1524
title: "Liveness under release/acquire"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-437
    type: verifies
---

# TC-1524: Liveness under release/acquire

## Description

Exercise the decision of a liveness claim over a protocol subject with an `ra` `parallel`. Scope: FR-437-AC-16.

## Test Procedure

Check a liveness claim that an eventual write is eventually loaded, over a subject whose `parallel` resolves to `ra`; inspect the fairness set and the stale-load lasso.

## Expected Results

The claim is decided, never refused as unsupported; it settles `proved`; the fairness set carries visibility fairness with a `memory` origin; the stale-load lasso is rejected by visibility fairness.
