---
id: TC-2045
title: "A plan step settles by its own negotiation and its prerequisites"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-824
    type: verifies
---

# TC-2045: A plan step settles by its own negotiation and its prerequisites

## Description

Verify the closed step disposition vocabulary of the plan run outcome: a
method with no registered engine settles `unsupported` without stopping
independent steps, a prerequisite blocks its dependents unless it is
satisfied, and a `measured` step settles by its FR-409 decision. Scope:
FR-824-AC-4 and FR-824-AC-5.

## Test Procedure

1. Execute `Release` with `max_trace_length` 8 and an engine set that registers no
   `temporal-satisfaction` engine.
2. Execute a plan of a `probabilistic_statistical` step `s` and a
   `model_check` step `t depends [s]` three times, with `s` measured
   `accepted`, `rejected` and `undecided`.

## Expected Results

- Step 1: `safe` `unsupported` with
  `unsupported_projection`/`unsupported-requested-capability` naming
  `temporal-satisfaction` and `bounded`; `tested` `tested`; `timely`
  `blocked` by `safe`, not negotiated; `status` `incomplete`, no failed
  steps, exit `21`.
- Step 2: `accepted` runs `t`; `rejected` blocks `t`, lists `s` in
  `failed_steps`, `status` `failed`, exit `22`; `undecided` blocks `t`, no
  failed steps, `status` `incomplete`, exit `22`.
