---
id: TC-1660
title: "A remainder is typed by its exact interval under a bounded search"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-147
    type: verifies
---

# TC-1660: A remainder is typed by its exact interval under a bounded search

## Description

Check that `mod` is typed by the exact smallest interval of its remainders,
including singleton dividends whose lower endpoint depends on their factors,
and that a search cut short by the checker's work limit is reported as a
stage limit instead of a wider type.

Scope: FR-147-AC-14.

## Test Procedure

1. Check each `mod` of FR-147-AC-14 into its stated interval, into that
   interval with the lower endpoint raised by one and into it with the upper
   endpoint lowered by one.
2. Check `a mod b` with `a: Int[1000000007, 1000000007]` and
   `b: Int[2, 1000000006]` into `Int[1, 500000003]` under the default work
   limit, then under a work limit of 10000.

## Expected Results

- Step 1: each stated interval is admitted; each narrowed interval is refused
  `undefined_expression`/`unproved-range` at the operator.
- Step 2: admitted under the default limit; under the limit of 10000, one
  stage limit naming the work limit, its value 10000 and its setting, at the
  operator, and no `unproved-range` refusal.
