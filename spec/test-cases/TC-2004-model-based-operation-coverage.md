---
id: TC-2004
title: "Model-based operation coverage"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-190
    type: verifies
---

# TC-2004: Model-based operation coverage

## Description

Verify that a model-based testing run over a state model reports, for every model operation, whether the generated suite covered it, and why an uncovered one is uncovered. Scope: FR-190-AC-11 to FR-190-AC-13.

Catches a generator that reports an operation it never applied as covered, one that calls an operation unreachable because no generated case happened to apply it, and one that lets coverage change the clause's verdict or exit.

## Test Procedure

1. Over a counter model with `inc` (enabled when `value < 5`, `value` in `Int[0, 5]`), `reset` (enabled when `value = 5`) and `jam` (enabled when `value > 5`), run model-based testing with a fixed seed, `max_cases` 100 and `max_trace_length` 8 over a clause every case passes.
2. Repeat with `max_trace_length` 3, then with a work limit too small for the bounded exploration.
3. Repeat step 1 over the clause `value <= 3`, which fails.
4. Run over a value clause, and repeat step 1 with the same inputs.

## Expected Results

- Step 1: `operations` lists `inc`, `reset`, `jam` in declaration order; `inc` and `reset` `covered` with their first case index; `jam` `uncovered`, reason `unreachable`, naming its declaration identity; `coverage` `partial`; `tested` evidence, no verdict, the passing tested run's outcome category.
- Step 2: `reset` `uncovered`, reason `unreachable-within-bound`; under the small work limit, `undetermined` naming the limit, with case counts and verdict unchanged.
- Step 3: `refuted` with its counterexample, and `jam` still `uncovered`, `unreachable`.
- Step 4: no `operations` member for the value clause; byte-equal `operations` entries for the repeated run.
