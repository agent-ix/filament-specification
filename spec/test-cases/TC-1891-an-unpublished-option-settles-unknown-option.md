---
id: TC-1891
title: "An unpublished option settles unknown-option"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-779
    type: verifies
---

# TC-1891: An unpublished option settles unknown-option

## Description

Check that an option a named backend does not publish settles the item `invalid-request` `invalid_capability`/`unknown-option`, and that a published option whose value the backend refuses settles `invalid_capability`/`invalid-option-value`; the two cause tables list both. Scope: FR-779-AC-1 and FR-779-AC-2.

## Test Procedure

1. Build items naming `b` with unpublished options, with `depth`, and naming an unregistered backend with options.
3. Build items naming a backend `h` that publishes `step` and holds its domain, with `{"step": "0"}`, `{"step": "1/4"}` and `{"colour": "blue", "step": "0"}`.
4. Compare the `invalid_capability` rows of native-diagnostics and FR-272 for `invalid-option-value`.

## Expected Results

- Step 1: `unknown-option` with `request_index`, `backend` `b`, `option` `alpha`; `depth` reaches `b`; `unknown-backend`.
- Step 2: both rows list `unknown-option` with the three payload keys.
- Step 3: `invalid-option-value` with `option` `step` and `value` `0` and no solving step; `1/4` reaches the backend; `unknown-option` with `option` `colour`.
- Step 4: both rows list `invalid-option-value` with the four payload keys.
