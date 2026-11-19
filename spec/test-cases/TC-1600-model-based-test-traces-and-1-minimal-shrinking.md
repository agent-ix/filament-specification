---
id: TC-1600
title: "Model-based test traces and 1-minimal shrinking"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-190
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-331
    type: verifies
---

# TC-1600: Model-based test traces and 1-minimal shrinking

## Description

Verify what model-based testing generates, its oracle, its result and its shrinking guarantee. Scope: FR-190-AC-6 to FR-190-AC-10 and FR-190-AC-14 to FR-190-AC-16, and the record side of FR-331-AC-55.

Catches a generator that narrows a declared finite domain by `max_value_bits`, emits a refusal partition for a size-bounded root, drops a precondition-rejected partition instead of counting it, charges discards outside `max_cases`, or reports `tested` for a run that evaluated no case.

## Test Procedure

1. Over a counter model with operation `inc` on `Int[0, 5]` and the state clause `value <= 3`, run model-based testing with a fixed seed, `max_cases` 100 and `max_trace_length` 8.
2. Run it over a clause every case passes, and again with a work limit smaller than 100 cases.
3. Run it over a value clause with an unbounded `Integer` parameter, with and without `max_value_bits`.
4. Run it over an empty domain, with `max_cases` 0, `max_trace_length` 0 and an unknown clause identity.
5. Repeat step 1 with the same inputs and compare the result records byte for byte.
6. Over FR-190-AC-14's value clause (`x: Integer`, `t: Text[0, 4]`), run with seed 7, `max_cases` 30 and `max_value_bits` 8, then 1; then send `max_value_bits` 0, 65 and none, and `max_value_bits` 8 over a clause whose one parameter is `q: Int[0, 3]`.
7. Over FR-190-AC-15's `inc(by: Int[1, 6])` model, run the postcondition clause of `inc` with seed 7, `max_cases` 100 and `max_trace_length` 8.
8. Add `jam()` (enabled when `value > 5`) and run the postcondition clause of `jam`, the precondition clause of `inc`, and step 7 with a test-double argument admission that accepts `by` 7.

## Expected Results

- Step 1: every generated invocation is enabled where applied; the retained counterexample is four `inc` from `value = 0`, 1-minimal, and native replay fails `value <= 3`.
- Step 2: the passing run reports `tested` evidence and counts, never `proved`; the limited run is `incomplete` naming the limit.
- Step 3: the bound appears in provenance; without it the request refuses naming `max_value_bits`.
- Step 4: `invalid_model_binding`/`malformed-declaration` for the empty domain; each other request refused naming its member.
- Step 5: byte-equal records.
- Step 6: with 8, every `x` in `[-255, 255]`; `x`'s partitions 0, -1, 1, -255, 255 with no refusal kind; `t`'s `empty`, `singleton`, `upper-endpoint`, `upper-refusal`; one `size_bounded` entry with `values` `"511"`. With 1, `x` in `[-1, 1]` and `t`'s partitions unchanged. Then `invalid-value` twice, `missing-member` and `unknown-member`, each naming `max_value_bits`, with no case run.
- Step 7: `domain-refusal` `{count: 4, first_case: 0}`; `precondition-false` `{count: 1, first_case: 6}` with the `by` 6 partition `uncovered`, reason `precondition-false`, `bounds` `[]`; the `value` 0 case `no-check-point`; `cases.generated` 100 = `passed + failed + discarded`; `tested`.
- Step 8: `jam`'s clause discards all 100 cases (`domain-refusal` 4, `precondition-false` 1 at case 6, `invariant-false` 0, `no-check-point` 95), `cases.passed` 0, `inconclusive`, cause `no-evaluated-case`, exit 22; `inc`'s precondition clause with no `precondition-false` discard and the `by` 6 partition `covered`; the accepting admission `failed` with `tool-failure` `oracle-disagreement`, exit 30.
