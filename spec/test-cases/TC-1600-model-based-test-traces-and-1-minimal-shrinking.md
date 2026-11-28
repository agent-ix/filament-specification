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

Verify what model-based testing generates, its oracle, its result and its shrinking guarantee. Scope: FR-190-AC-6 to FR-190-AC-10 and FR-190-AC-14 to FR-190-AC-16, FR-190-AC-18 to FR-190-AC-26, and the record side of FR-331-AC-55.

Catches a generator that narrows a declared finite domain by `max_value_bits`, emits a refusal partition for a size-bounded root, drops a precondition-rejected partition instead of counting it, charges discards outside `max_cases`, reports `tested` for a run that evaluated no case, or writes a record whose members, `null`s, causes or counterexample differ from the FR-190 "Result record".

## Test Procedure

1. Over a counter model with operation `inc` on `Int[0, 5]` and the state clause `value <= 3`, run model-based testing with a fixed seed, `max_cases` 100 and `max_trace_length` 8.
2. Run it over a clause every case passes, and again with a work limit smaller than 100 cases.
3. Run it over a value clause with an unbounded `Integer` parameter, with and without `max_value_bits`.
4. Run it over an empty domain, with `max_cases` 0, `max_trace_length` 0 and an unknown clause identity.
5. Repeat step 1 with the same inputs and compare the result records byte for byte.
6. Over FR-190-AC-14's value clause (`x: Integer`, `t: Text[0, 4]`), run with seed 7, `max_cases` 30 and `max_value_bits` 8, then 1; then send `max_value_bits` 0, 65 and none, and `max_value_bits` 8 over a clause whose one parameter is `q: Int[0, 3]`.
7. Over FR-190-AC-15's `inc(by: Int[1, 6])` model, run the postcondition clause of `inc` with seed 7, `max_cases` 100 and `max_trace_length` 8.
8. Add `jam()` (enabled when `value > 5`) and run the postcondition clause of `jam`, the precondition clause of `inc`, and step 7 with a test-double argument admission that accepts `by` 7.
9. Call `unrank` and `rank` on each domain and index of FR-190-AC-18 (`Text[0, 2; unicode-scalars]`, `Text[0, 3; binary-utf8]`, `Text[0, 1]` under `unicode-scalars`, `nfc`, `nfd` and `nfkc`); list the partitions of `t: Text[0, 4]` under each of the six profiles and of `Text[0, 1; nfd]` (FR-190-AC-19); run the value clause over `t: Text[0, 13; unicode-scalars]` with seed 7 and `max_cases` 30 twice and compare (FR-190-AC-20).
10. Run `Square` (`x: Integer`, `max_value_bits` 8) and the value clause over `t: Text[0, 2; unicode-scalars]`, each with seed 7, and record the first seeded case's draw value, its index and its value (FR-190-AC-21).
11. Run the value clause over `t: Text[0, 1099511627776; nfc]` (body `size(t) >= 0`) with seed 7, `max_cases` 30 and `accounting.work_units` 1000000; repeat under `unicode-scalars`; repeat over `Text[0, 4; nfc]`, then with `accounting.work_units` 2559 (FR-190-AC-22).
12. Run the value clause `Nonneg` (`q: Int[0, 3]`, body `q >= 0`) with seed 7 and `max_cases` 10 and read its record strictly; then submit that record with `cases` removed, with each of `status`, `evidence`, `verdict` and `domain` added, with `"operations": null`, `"coverage": null` and `"cause": null` added, and with `provenance.max_value_bits` removed. Read step 7's record (FR-190-AC-23).
13. Run `Nonneg` with body `q <= 2`; read step 1's counterexample; run a value clause over `t: Text[0, 4]` with body `size(t) < 4` (FR-190-AC-24).
14. Read the `cause` of step 8's `jam` and accepting-admission runs and of step 11's first run; run the `frame` clause of an operation with no implementation binding; submit a record whose `cause` is the bare string `"no-evaluated-case"`, and one whose `cause` is keyed by `kind` (FR-190-AC-25).
15. Read the `by` 6 and `value` -1 partition entries and `checks.skipped` of step 7, and the `size_bounded` entry and `t`'s `upper-endpoint` entry of step 6 (FR-190-AC-26).

## Expected Results

- Step 1: every generated invocation is enabled where applied; the retained counterexample is four `inc` from `value = 0`, 1-minimal, and native replay fails `value <= 3`.
- Step 2: the passing run reports `tested` evidence and counts, never `proved`; the limited run is `incomplete` naming the limit.
- Step 3: the bound appears in provenance; without it the request refuses naming `max_value_bits`.
- Step 4: `invalid_model_binding`/`malformed-declaration` for the empty domain; each other request refused naming its member.
- Step 5: byte-equal records.
- Step 6: with 8, every `x` in `[-255, 255]`; `x`'s partitions 0, -1, 1, -255, 255 with no refusal kind; `t`'s `empty`, `singleton`, `upper-endpoint`, `upper-refusal`, then its `normalization` partitions as FR-190-AC-19 gives them for its profile; one `size_bounded` entry with `values` `"511"`. With 1, `x` in `[-1, 1]` and `t`'s partitions unchanged. Then `invalid-value` twice, `missing-member` and `unknown-member`, each naming `max_value_bits`, with no case run.
- Step 7: `domain-refusal` `{count: 4, first_case: 0}`; `precondition-false` `{count: 1, first_case: 6}` with the `by` 6 partition `uncovered`, reason `precondition-false`, `bounds` `[]`; the `value` 0 case `no-check-point`; `cases.generated` 100 = `passed + failed + discarded`; `tested`.
- Step 8: `jam`'s clause discards all 100 cases (`domain-refusal` 4, `precondition-false` 1 at case 6, `invariant-false` 0, `no-check-point` 95), `cases.passed` 0, `inconclusive`, cause `no-evaluated-case`, exit 22; `inc`'s precondition clause with no `precondition-false` discard and the `by` 6 partition `covered`; the accepting admission `failed` with `tool-failure` `oracle-disagreement`, exit 30.
- Step 9: the domain sizes, values and ranks FR-190-AC-18 states; the partition values, in order, FR-190-AC-19 states; for `Text[0, 13]`, 30 cases, `tested`, no `total-above-draw-range`, and equal `t` values in both runs.
- Step 10: the draw values, indices and values FR-190-AC-21 states: `x` = 53 at case 5 and `t` = U+D51A9 U+87AC9 at case 10.
- Step 11: `incomplete`, cause `{"type": "limit-reached", "limit": "work_units", "setting": "accounting.work_units", "value": 1000000}`, `cases.generated` 0, no abort and not `unsupported`, under both profiles; then `tested` with `cases.generated` 30; with 2559, `incomplete` naming `accounting.work_units`, value 2559, `cases.generated` 0.
- Step 12: exactly the example record of FR-190 "Result record", with no `cause`, `diagnostic`, `counterexample`, `retained`, `limits.max_trace_length`, `value_bound_cuts`, `operations` or `coverage` member; each altered record refused naming the altered member. Step 7's record has `limits` `{max_cases: 100, max_trace_length: 8}`, `value_bound_cuts` `{count: 0, first: null}` and `operations`; no record has `status`, `evidence` or `verdict`.
- Step 13: `refuted`, no `cause`, `retained` `{minimality: "1-minimal", replay: "reproduced", disposition: "predicate-false"}`, and `counterexample` the assignments of `q` = 3 keyed by `q`'s `NodeId`; step 1's counterexample an FR-364 `TemporalCounterexample`, `trace` `model`, `initial_state` 0, four `inc` steps, `loop` `[]`, disposition `invariant-violated`; the Text clause's `t` a `text` typed value of length 4. No counterexample carries `minimality`, `replay` or `disposition`.
- Step 14: `{"type": "no-evaluated-case"}`, `{"type": "tool-failure", "defect": "oracle-disagreement"}` and the `limit-reached` object of step 11; the `frame` clause `unsupported` with FR-485's diagnostic and no `cause`; both submitted records refused naming `cause`.
- Step 15: the entries, `skipped` members, `size_bounded` entry (`measures` `["value"]`, `values` `"511"`) and `text` value FR-190-AC-26 states, with no text member beside `value`.
