---
id: TC-1911
title: "A symbolic channel capacity steps at its bound value or requires a bound"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-785
    type: verifies
---

# TC-1911: A symbolic channel capacity steps at its bound value or requires a bound

## Description

Catches a symbolic capacity that is refused as unsupported, read from a
default, truncated to a horizon, or claimed beyond its bound value. Scope:
FR-785-AC-1, FR-785-AC-2, FR-785-AC-3, FR-785-AC-4, FR-785-AC-5, FR-785-AC-6.

## Test Procedure

1. Build the FR-785-AC-1 subject with `capacities` `{"Bound": "1"}` and
   `{"Bound": "2"}`. Explore each, and the same clause written with
   `capacity 1` and `capacity 2`. Add a second channel with
   `symbolic(Bound)`.
2. Build the subject with no entry for `Bound`, then a clause with two
   capacity names and no binding.
3. Build the subject with each binding defect of FR-785-AC-3, then with
   `{"Bound": "0"}`.
4. Settle the deadlock-freedom item at `"1"` and at `"2"`.
5. Replay the counterexample of step 4 unchanged, with `capacities` changed to
   `{"Bound": "2"}`, without `capacities`, and with `capacities` added over a
   clause that has a literal capacity.
6. Settle a realizability item over the clause at `"2"`, at `"1"`, and with no
   binding.

## Expected Results

- Step 1: at `"1"` the second send waits, and the deadlock is `full-queue`
  naming the qualified channel name. At `"2"` the run finishes. The state keys
  and transition identities equal the literal clause's at each value. Both
  channels hold at most the bound value.
- Step 2: `requires-bound` before any expansion, naming `Bound` and each of
  its channels; with two names, both are named. No `unsupported` result and
  no `unsupported_construct`/`not-yet-implemented` diagnostic.
- Step 3: `unknown-member`, `invalid-value` for each bad value and
  `duplicate-member`, each `invalid_runtime_input` naming its entry, before
  any expansion. `"0"` is admitted.
- Step 4: `refuted` at `"1"` and `proved` at `"2"`, with different obligation
  and request identities. The `proved` result's subject carries `{"Bound": "2"}`.
- Step 5: `reproduced-with-evaluated-witness`; then `inconclusive`, cause
  `replay-parity`; then a refusal before replay in each of the last two
  cases.
- Step 6: decided at `"2"` with a channel premise holding `Bound` and `2`; a
  different witness identity at `"1"`; `inconclusive`, cause
  `premise-not-stated`, with no binding.
