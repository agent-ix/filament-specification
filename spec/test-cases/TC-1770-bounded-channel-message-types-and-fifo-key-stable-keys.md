---
id: TC-1770
title: "A channel carries a bounded type and keys by a FIFO-key-stable type"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-726
    type: verifies
---

# TC-1770: A channel carries a bounded type and keys by a FIFO-key-stable type

## Description

Check that the protocol reader admits a `bounded_domain` message type and every FIFO-key-stable key, and refuses a key with an IEEE or event leaf at the key.

Scope: FR-726-AC-1, FR-726-AC-2, FR-726-AC-3.

## Test Procedure

1. Read packages whose `unordered` channel carries each of `Int[0, 5]`, `Text[0, 8; nfc]`, a bounded `Decimal`, a bounded `Rational`, `Float64[nearest-even]` and `Sequence<Boolean>[0, 3]`, and one whose `carries` names an `expression` node.
2. Read packages whose FIFO channel is keyed by each FR-726-AC-2 type (`FK-01` to `FK-10` of `proposals/checked-package-v2/fifo-key-type-vectors.json`).
3. Read packages whose FIFO channel is keyed by each FR-726-AC-3 type (`FK-11` to `FK-17`), the model-owned `Reading` with its fields read from the model document, and the same channels `unordered`.

## Expected Results

- Step 1: the six are admitted with the `bounded_domain` node as `carries`; the `expression` target refuses `invalid_model_binding`/`malformed-declaration` at the target.
- Step 2: every package is admitted.
- Step 3: each FIFO package refuses `ill_typed`/`operator-ineligible` at the key, retaining `=` and the type; each `unordered` one is admitted.
