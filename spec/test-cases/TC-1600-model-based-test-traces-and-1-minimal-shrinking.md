---
id: TC-1600
title: "Model-based test traces and 1-minimal shrinking"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-190
    type: verifies
---

# TC-1600: Model-based test traces and 1-minimal shrinking

## Description

Verify what model-based testing generates, its oracle, its result and its shrinking guarantee. Scope: FR-190-AC-6 to FR-190-AC-10.

## Test Procedure

1. Over a counter model with operation `inc` on `Int[0, 5]` and the state clause `value <= 3`, run model-based testing with a fixed seed, `max_cases` 100 and `max_trace_length` 8.
2. Run it over a clause every case passes, and again with a work limit smaller than 100 cases.
3. Run it over a value clause with an unbounded `Integer` parameter, with and without `max_value_bits`.
4. Run it over an empty domain, with `max_cases` 0, `max_trace_length` 0 and an unknown clause identity.
5. Repeat step 1 with the same inputs and compare the result records byte for byte.

## Expected Results

- Step 1: every generated invocation is enabled where applied; the retained counterexample is four `inc` from `value = 0`, 1-minimal, and native replay fails `value <= 3`.
- Step 2: the passing run reports `tested` evidence and counts, never `proved`; the limited run is `incomplete` naming the limit.
- Step 3: the bound appears in provenance; without it the request refuses naming `max_value_bits`.
- Step 4: `invalid_model_binding`/`malformed-declaration` for the empty domain; each other request refused naming its member.
- Step 5: byte-equal records.
