---
id: TC-2003
title: "Plan search limit defaults and overrides"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-801
    type: verifies
---

# TC-2003: Plan search limit defaults and overrides

## Description

Verify that each `plan_search` limit has its published default, that a request overrides one member without changing the others, and that an invalid value refuses before any search. Scope: FR-801-AC-7.

Catches a planner that leaves a default unpublished or engine-specific, one that resets unset members when one is set, and one that admits 0 or an out-of-range value as unlimited.

## Test Procedure

1. Run FR-650-AC-1's plan request with no `limits` member.
2. Run it with `limits` `{max_states: 500}`.
3. Run it with `limits.max_work` 0, `-1`, `1.5`, 18446744073709551616 and `"10"`, and with `limits.max_steps`.

## Expected Results

- Step 1: the outcome's `limits` lists the five settings with 10000000, 100000000, 16777216, 100000000 and 1048576.
- Step 2: `plan_search.max_states` 500 and the four other defaults.
- Step 3: `invalid-value`, `invalid-value`, `invalid-value`, `invalid-value` and `wrong-value-kind` at `/limits/max_work`, and `unknown-member` at `/limits/max_steps`, each `invalid_runtime_input`, exit `20`, before any search.
