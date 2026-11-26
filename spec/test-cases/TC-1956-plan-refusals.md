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

Verify the check-time and request-time refusals and the outcomes that are not refusals. Scope: FR-806-AC-1 to FR-806-AC-5 and FR-806-AC-7.

## Test Procedure

1. Check an unbounded `domain` and an unbounded parameter.
2. Check a goal reading `elapsed` over an untimed model and a negative `domain`.
3. Request a value-type synthesis and a foreign policy.
4. Request with an inadmissible initial snapshot.
5. Request an unreachable goal.
6. Request the policies `["NotFour", "Nowhere"]` with `counters` listed
   `complete: false`, then with `counters` omitted from the snapshot.

## Expected Results

- Steps 1 to 4: each refusal with its code, cause and locus, exit 20, no plan.
- Step 5: `infeasible`, exit 10.
- Step 6: `incomplete_population`/`incomplete-scope` at `/initial`, then
  `missing_declaration`/`missing-name` at `/policy/1`; no plan outcome.
