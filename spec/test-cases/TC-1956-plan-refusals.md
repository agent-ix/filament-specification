---
id: TC-1956
title: "Located plan refusals"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-806
    type: verifies
---

# TC-1956: Located plan refusals

## Description

Verify the check-time and request-time refusals and the outcomes that are not refusals. Scope: FR-806-AC-1 to FR-806-AC-5.

## Test Procedure

1. Check an unbounded `domain` and an unbounded parameter.
2. Check a goal reading `elapsed` over an untimed model and a negative `domain`.
3. Request a value-type synthesis and a foreign policy.
4. Request with an inadmissible initial snapshot.
5. Request an unreachable goal.

## Expected Results

- Steps 1 to 4: each refusal with its code, cause and locus, exit 20, no plan.
- Step 5: `infeasible`, exit 10.
