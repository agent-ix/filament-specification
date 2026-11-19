---
id: TC-2039
title: "A verification plan request selects one plan and writes its outcome"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-824
    type: verifies
---

# TC-2039: A verification plan request selects one plan and writes its outcome

## Description

Verify the verification plan request's members, its plan selection, its
per-step inputs and the `filament.verification-plan-outcome/v1` document.
Scope: FR-824-AC-1 to FR-824-AC-3.

## Test Procedure

1. Execute `Release` with `seed` 7 where `safe` is refuted; then request it
   with `emit` `true` and `execute` `false`.
2. Send `{}` over one plan and over two; name `Nope`, the claim `Never`, and
   a bare `Release` declared as `A::Release` and `B::Release`.
3. Send `seed` `"7"`, both flags `false`, an `engine` member, `seed` on a
   plan of `model_check` steps only, and, over a selected package, no
   `document` and a plan document of another package.

## Expected Results

- Step 1: `failed`, the three dispositions, `failed_steps` `["safe"]`, exit
  10; then `document` present, `dispositions` `null`, exit 0.
- Step 2: the plan runs; `missing-member`, `missing-name` twice and
  `ambiguous-name`, each at `/plan`, exit 20.
- Step 3: `wrong-value-kind` at `/seed`, `invalid-value` at `/execute`,
  `unknown-member` at `/engine` and `/seed`, `missing-member` and
  `invalid-value` at `/document`.
