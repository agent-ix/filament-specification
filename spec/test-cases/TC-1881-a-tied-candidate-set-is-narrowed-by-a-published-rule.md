---
id: TC-1881
title: "A tied candidate set is narrowed by a producer's published claim-form rule"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-776
    type: verifies
---

# TC-1881: A tied candidate set is narrowed by a producer's published claim-form rule

## Description

Check that a published claim-form rule narrows an unnamed tied item to one candidate before negotiation and leaves any other multi-candidate set to settle `ambiguous-backend`. Scope: FR-776-AC-1, FR-776-AC-2.

## Test Procedure

1. Negotiate an unnamed item the rule assigns to A over A and B.
2. Register C matching the same item; repeat unnamed and naming B.

## Expected Results

- Step 1: candidates `[A]`, no `backend`, settled through A's arm.
- Step 2: `ambiguous-backend` naming A, B and C; named, candidates `[B]`.
