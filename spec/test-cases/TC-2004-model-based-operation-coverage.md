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

Verify that a model-based testing run over a state model reports, for every model operation, whether the generated suite covered it, and why an uncovered one is uncovered. Scope: FR-190-AC-11 to FR-190-AC-13, FR-190-AC-17, FR-190-AC-43, FR-190-AC-44 and FR-190-AC-46.

Catches a generator that reports an operation it never applied as covered, one that calls an operation unreachable because no generated case happened to apply it, one that lets coverage change the clause's verdict or exit, and one that writes a coverage limit or a step prefix in a form other than the record's.

## Test Procedure

1. Over a counter model with `inc` (enabled when `value < 5`, `value` in `Int[0, 5]`), `reset` (enabled when `value = 5`) and `jam` (enabled when `value > 5`), run model-based testing with a fixed seed, `max_cases` 100 and `max_trace_length` 8 over a clause every case passes.
2. Repeat with `max_trace_length` 3, then with a work limit too small for the bounded exploration.
3. Repeat step 1 over the clause `value <= 3`, which fails.
4. Run over a value clause, and repeat step 1 with the same inputs.
5. Over FR-190-AC-17's counter (`value: Integer`, `add(by: Int[1, 100])`, `big()` enabled when `value >= 300`), run with seed 7, `max_cases` 100, `max_trace_length` 8 and `max_value_bits` 8, then with `max_value_bits` 10, then with `max_value_bits` 10 and `max_cases` 1.

## Expected Results

- Step 1: `operations` lists `inc`, `reset`, `jam` in declaration order; `inc` and `reset` `covered` with their first case index; `jam` `uncovered`, reason `unreachable`, naming its declaration identity; `coverage` `partial`; `tested` evidence, no verdict, the passing tested run's outcome category.
- Step 2: `reset` `uncovered`, reason `unreachable-within-bound`, `bounds` `["max_trace_length"]`; under the small work limit, `undetermined` with `stopped_by` a `limit-reached` cause naming the limit, its value and its setting, and no `limit`, `value` or `setting` member beside it, with case counts and verdict unchanged.
- Step 3: `refuted` with its counterexample, and `jam` still `uncovered`, `unreachable`.
- Step 4: no `operations` and no `coverage` member for the value clause, not even `null`; byte-equal `operations` entries for the repeated run.
- Step 5: with 8, `big` `uncovered`, reason `unreachable-within-bound`, `bounds` `["max_value_bits"]`, never `unreachable`; `value_bound_cuts.count` at least 1 and `first` `{initial_state: 0, prefix, transition}`, `prefix` FR-181 transition identities of `add` and `transition` an `add` application from a state where `value + by` exceeds 255; `tested`. With 10, `big` `covered` or `not-generated` with an `enabled_at` `{initial_state, prefix}` whose `prefix` is FR-181 transition identities of `add` whose arguments sum to at least 300. With `max_cases` 1, `big` `not-generated` with `enabled_at` exactly `{"initial_state": 0, "prefix": [A, A, A]}`, `A` the transition identity of `add` with argument 100.
