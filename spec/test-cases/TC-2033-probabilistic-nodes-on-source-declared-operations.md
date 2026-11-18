---
id: TC-2033
title: "Probabilistic nodes on source-declared operations"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-640
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-610
    type: verifies
---

# TC-2033: Probabilistic nodes on source-declared operations

## Description

Verify that delay, random-parameter and workload nodes whose operation references name source-declared object types are emitted, admitted by the strict read and joined through the identity rows, and that the identity rows carry each source operation's parameters. Scope: FR-640-AC-6, FR-610-AC-9.

## Test Procedure

1. Emit FR-640-AC-6's unit (`O`, `Other`, `time O dense unit ms;`, the two delays and `random Other::poke(k)`), read it strictly and run the identity-row join.
2. Edit `Other::poke`'s delay `operation` to `nudge`; then the random parameter's `parameter` to `j`; join each.
3. Edit a delay's `declaring` to a source `relation`/`population` node; then read a package of the same unit with no `time` member; read each strictly.
4. Read the identity row of FR-610-AC-9's `O`; then the row with `clock_parameters` `[]`, and with `parameters` omitting `k`.

## Expected Results

- Step 1: admitted; the typed read gives both delays, typed by the unit `ms`, and the support `1`, `3` with probabilities `1/4`, `3/4`.
- Step 2: each `invalid_package`/`invalid-value` at the edited member, at the identity-row join.
- Step 3: each `invalid_package`/`invalid-value` at `declaring`, at the strict read.
- Step 4: `parameters` `x` (clock entry) then `k` (data entry) and `clock_parameters` `["x"]`; both edited rows refuse.
