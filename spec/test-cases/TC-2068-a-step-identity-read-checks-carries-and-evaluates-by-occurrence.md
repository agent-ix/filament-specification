---
id: TC-2068
title: "A step identity read checks, is carried by node and evaluates by occurrence"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-828
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-480
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-481
    type: verifies
---
# TC-2068: A step identity read checks, is carried by node and evaluates by occurrence

## Description

Verify that `filament::protocol::message`, `filament::protocol::delivery` and
`filament::protocol::effect` check to their `Option` identity types, refuse
every ill-formed or misplaced use, are carried as FR-322 `protocol`/`step_read`
nodes keyed by field and target, and evaluate at each position to the
identity the position's step carries. Scope: FR-828-AC-1 to FR-828-AC-7,
FR-322-AC-480 and FR-322-AC-481.

## Test Procedure

Use protocol `Echo` (channel `ch` `fifo`, capacity 2; `run sequence { send
Snd via ch; receive Rcv via ch of Snd }`) over the `Fill` model, and protocol
`P` with `run repeat { attempt Pay; effect Paid of Pay }`.

1. Check a temporal clause holding each read of FR-828-AC-1 and the capture
   `d`; read the checked types.
2. Check each refused form of FR-828-AC-2, one unit per form.
3. With `delivery unknown`, evaluate the three reads along `send(Snd)`,
   `duplicate`, `receive(Rcv)` of delivery 1.
4. With `delivery at-least-once`, model-check FR-828-AC-4's two obligations
   (the delivery claim over `Echo`, the message claim over the two-receive
   variant) under a bounded profile.
5. Model-check FR-828-AC-5's `Paid` obligation over `P` with `max_depth` 8
   and read the two `effect(P::Paid)` values of the first two iterations.
6. Request FR-828-AC-6's two items.
7. Build the packages of FR-322-AC-480 (two occurrences, then a second field,
   a changed field and a changed target) and compare node ids; mutate the
   carrier as FR-322-AC-481 lists and read each package.

## Expected Results

- Step 1: the types FR-828-AC-1 lists; `.index` is `Integer`.
- Step 2: each form's refusal kind and location; no package. A checker that
  admits any form fails the step.
- Step 3: `none` at positions 0 and 2; at position 1 `message(Echo::Snd)` is
  (0, `Snd`, `(0)`, 0); at position 3 `message(Echo::Rcv)` equals it and
  `delivery(Echo::Rcv)` has index 1, never index 0 or the payload.
- Step 4: the delivery claim holds; the message claim is refuted with
  receive identities of one message and deliveries 0 and 1.
- Step 5: ordinals 0 and 1, unequal; the obligation holds within the bound.
- Step 6: both refuse `invalid_runtime_input`/`invalid-value` naming the read,
  with no state explored.
- Step 7: one carrier with two occurrences and the derived id; a second
  carrier for the second field; the field or target change moves the carrier
  and clause ids and leaves `Echo`'s id unchanged; each mutation refuses with
  the kind and pointer FR-322-AC-481 states; `step_identity.eq` and
  `structural.eq` over `Option<Delivery>` are admitted.

Assertions compare typed fields, causes, identities and keys, never message
text.
