---
id: TC-1402
title: "Quantifier on a concrete unbounded value"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-346
    type: verifies
---

# TC-1402: Quantifier on a concrete unbounded value

## Description

Verify FR-346-AC-7: evaluating a quantifier on one concrete value of an unbounded collection type visits that value and never settles the claim.

## Test Procedure

Evaluate the quantifiers of FR-346-AC-7 on the two supplied values, recording visits, and request the claim over the declared domain with no quantifier-capable backend registered.

## Expected Results

The values and visit counts match FR-346-AC-7, and the claim settles `unsupported` with a warning naming the capability.
