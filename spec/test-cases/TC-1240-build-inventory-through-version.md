---
id: TC-1240
title: "Build inventory through version"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-560
    type: verifies
---

# TC-1240: Build inventory through version

## Description

Verify the build inventory document `version` writes. Scope: FR-560-AC-1 to FR-560-AC-3.

## Test Procedure

1. Run `version --format json` twice and compare with the build's capability inventory.
2. Check a unit selecting each listed profile with its `requires` profiles.
3. Run `version --kind profiles` and `version --kind widgets`.

## Expected Results

- Step 1: one `filament.build-inventory/v1` document with the five lists, `features` equal to the inventory, equal bytes, exit `0`.
- Step 2: every listed profile is accepted; none begins with `quire.`.
- Step 3: only `profiles` is listed; `widgets` exits `20` naming it and the five list names, empty stdout.
