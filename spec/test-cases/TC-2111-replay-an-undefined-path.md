---
id: TC-2111
title: "Replay an undefined path"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-413
    type: verifies
---
# TC-2111: Replay an undefined path

## Description

An undefined path names the first undefined read on a path of positive
probability and the undefined evaluation it produced; replay reproduces it
whatever the bound, and disagrees when the read is elsewhere or defined.

Scope: FR-413-AC-11.

## Test Procedure

1. Over the `Coin` model (`drop(b)`, `b = 0` deletes the coin, each `b`
   with `1/2`), refute `probability >= 0.5 [ always[0,3] holds(c.v = 0) ]`
   and `probability >= 0.01 [ … ]` with the undefined path `drop` with
   `b = 0` and `undefined` naming `position` 1, the locus of `c.v`, the
   pair, the pre-read product key and `absent-key`.
2. Replay the same evidence with `position` 0, with the path `drop` with
   `b = 1`, and with another catalogued `cause`.
3. Validate the counterexample without `undefined` against the
   backend-provider schema.

## Expected Results

- Step 1: both replay `refuted`, `decisive-counterexample`.
- Step 2: each settles `inconclusive`, `replay-parity`.
- Step 3: schema-invalid.
