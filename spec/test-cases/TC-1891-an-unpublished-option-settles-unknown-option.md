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

Check that an option a named backend does not publish settles the item `invalid-request` `invalid_capability`/`unknown-option`, and that the two cause tables list it. Scope: FR-779-AC-1.

## Test Procedure

1. Build items naming `b` with unpublished options, with `depth`, and naming an unregistered backend with options.
2. Compare the `invalid_capability` rows of native-diagnostics and FR-272.

## Expected Results

- Step 1: `unknown-option` with `request_index`, `backend` `b`, `option` `alpha`; `depth` reaches `b`; `unknown-backend`.
- Step 2: both rows list `unknown-option` with the three payload keys.
