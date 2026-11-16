---
id: TC-1841
title: "A timed interval with a lower bound and no upper bound"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-766
    type: verifies
---

# TC-1841: A timed interval with a lower bound and no upper bound

## Description

Check that the timed profile admits `[q, *]` on past and future operators,
gives it the stated meaning and key, and that the bounded profiles still
refuse it.

Scope: FR-766-AC-1, FR-766-AC-2, FR-766-AC-3.

## Test Procedure

1. Under the timed profile in `ms`, check `once[5 ms, *] holds(p)`, `eventually[5 ms, *] holds(p)`, `once(5 ms, *) holds(p)`, `once[5 ms, *)`, `once[0 ms, *] holds(p)`, `once holds(p)` and `once[5, *] holds(p)`.
2. Evaluate `once[5 ms, *] holds(p)` and `once(6 ms, *) holds(p)` over the timed trace with positions at 0, 4 and 6 ms and `p` true only at 0 ms.
3. Check `once[5 ms, *] holds(p)` under the timestamped-event finite-window profile.

## Expected Results

- Step 1: the first two check with lower bound 5, closed, no upper bound; the third has an open lower end; `[5 ms, *]` and `[5 ms, *)` have equal keys; `[0 ms, *]` and the bare operator have equal keys; `[5, *]` refuses `ill_typed`/`type-mismatch`.
- Step 2: false at 4 ms and true at 6 ms; `(6 ms, *)` is false at 6 ms.
- Step 3: refused as an unbounded interval under a bounded profile.
