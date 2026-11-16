---
id: TC-1773
title: "A post-test repeat is encoded and stepped"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-729
    type: verifies
---

# TC-1773: A post-test repeat is encoded and stepped

## Description

Check the `repeat` `test` member, its scope and refusals, and the post-test moves.

Scope: FR-729-AC-1, FR-729-AC-2, FR-729-AC-3, FR-729-AC-4.

## Test Procedure

1. Read the `Load` retry `repeat` with `test` `post` and no `maximum`, `maximum` 3, 1 and 0; with `test` `pre`; with no `test`.
2. Step the admitted `repeat` through `loaded` false then true, and under `maximum` 2 through two false results.
3. Compare a pre-test `repeat`'s node with its pre-change bytes, and with the post-test form of the same body and guard.
4. Read a post-test `repeat` whose `invariant` and `variant` read the body binder, and one whose `visible` reads a binder bound on one `choice` branch only.

## Expected Results

- Step 1: admitted for no maximum, 3 and 1; `invalid_package`/`invalid-value` for 0; `malformed_wire` for `pre`; `missing-name` without `test`.
- Step 2: the first `attempt(Load)` is enabled with no guard read; false re-enters, true leaves; two false under `maximum` 2 enter `exhausted`.
- Step 3: the bytes are equal; the two forms have different node ids.
- Step 4: the first is admitted; the second refuses `missing-name` as an out-of-scope read.
