---
id: TC-1001
title: "Profile-less units and functions without using"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-500
    type: verifies
---

# TC-1001: Profile-less units and functions without using

## Description

Verify that a unit may declare no profile and a function or predicate may omit `using`, that both are admitted under the fixed no-layer default, and that other declarations keep `using` required (FR-500). Scope: FR-500-AC-2, FR-500-AC-3 and FR-500-AC-4.

## Test Procedure

1. Parse, check and evaluate the profile-less unit of FR-500-AC-2.
2. Check the profile-less unit whose function has a `Clock` parameter, then the same function in a unit whose only profile is `filament.value.complete/v1` (FR-500-AC-3).
3. Parse and check the declarations of FR-500-AC-4.

## Expected Results

- Step 1: no diagnostic; no profile selection, no layer, the edition as the only definition selection; `f()` is `true`.
- Step 2: both refuse `unsupported_construct` at the `Clock` type with the same code and cause; the profile-less refusal names no selected layer and `filament.temporal.complete/v1` as required.
- Step 3: `g` and `p` check with no profile selection; `h` refuses at the alias `w`; the `invariant` without `using` refuses `invalid_syntax`/`unexpected-token` at `on`.
