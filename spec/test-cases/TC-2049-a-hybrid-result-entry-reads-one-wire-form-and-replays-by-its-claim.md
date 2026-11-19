---
id: TC-2049
title: "A hybrid result entry reads one wire form and replays by its claim"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-193
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-331
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-323
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-272
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-652
    type: verifies
---

# TC-2049: A hybrid result entry reads one wire form and replays by its claim

## Description

Verify that every exact number on the hybrid wire is the `Rational` object,
that a provider's claimed refutation that does not replay settles
`replay-parity`, that an undefined-evaluation counterexample's `where`
carries exactly `position`, `time` and `locus`, that a provider reports a
construct outside its class as a located `unsupported` diagnostic, and that
a hybrid counterexample replays through a `state_clause` target naming its
claim. Verify that a hybrid solver proof carries an FR-652 certificate in its closed wire form and that the record, not the entry, becomes `certified`. Scope: FR-193-AC-24, FR-331-AC-54, FR-323-AC-47, FR-323-AC-48, FR-272-AC-31 and FR-652-AC-6.

## Test Procedure

1. Over `Braking`, read a `proved` `bounded-solver` entry with `horizon`
   `{"numerator": "10", "denominator": "1"}`; then with `horizon` `"10"`
   and `10`; then a `solver-inconclusive` trace whose `start.time.lower`
   is `"0"`. Admit an item whose `options` are `{"step": "1/16"}`.
2. Read a `refuted` entry whose point trace exact replay does not
   reproduce.
3. Replay the FR-193 point trace from `(Moving, v = 20, a = 2)` against
   `always [0, 12] holds(1 / (h.v - 5) != 0)` and write its counterexample;
   read the same counterexample with `state_key` added to `where`, and with
   `time` removed.
4. Read an `unsupported` entry for the flow `v' = -a / (1 + v)` carrying the
   `expression-form` diagnostic and no `cause`; then the same entry with a
   `cause` `{"type": "expression-form"}`.
5. Build the replay request of the refuted `TooSlow` item; replay it with
   its reproducing trace, with a trace that does not reproduce, and with
   `work_units` 100 over the FR-450-AC-7 lasso; then with a `state_clause`
   naming no declaration.
6. Validate the FR-652-AC-6 entries against the schema; settle the
   `inductive-solver` entry with its accepted and with a rejected
   `inductive-enclosure` certificate.
7. Build the replay request of FR-652-AC-3's `Below` item refuted through
   its accepted `validated-witness`; replay it, then with the witness cut
   after its step at `[28, 32]`, then with no `certificate`.

## Expected Results

- Step 1: the object form is read; `"10"` and `10` are refused
  `invalid_result`/`wrong-value-kind` at `/basis/horizon`; the string
  `lower` is refused `wrong-value-kind` at its pointer; the option is
  admitted as a string.
- Step 2: `inconclusive`, `replay-parity`, carrying the provider's
  counterexample; never `solver-inconclusive`.
- Step 3: `kind` is `{"type": "undefined-evaluation", "where": {"position":
  0, "time": {"lower": {"numerator": "15", "denominator": "2"}, "upper":
  {"numerator": "15", "denominator": "2"}}, "locus": L}, "cause":
  "division-by-zero"}`, `L` the division's term node; both altered
  counterexamples fail the schema and are refused by the reader.
- Step 4: admitted and recorded `unsupported` with `detail` `form`,
  `declaration`, `mode` `Moving`, `variable` `v`, `backend` and `locus`, the
  flow's term node as an FR-331 `Locus`, and no profile; the `cause` form is refused `invalid_result`/`unknown-cause` at
  `/cause/type`.
- Step 5: the request carries `{"state_clause": "TooSlow"}` only and
  resolves to the hybrid claim; rows `arm` `hybrid` with settlement
  `reproduced-with-evaluated-witness` (category `success`, with `replay`
  and `counterexample`), `inconclusive` (with `disagreement` `hybrid-trace`
  `not-reproduced` and the complete counterexample) and `incomplete` (with
  `limit` `{"limit": "work_units", "value": "100", "setting":
  "accounting.work_units"}`); the unknown name refuses
  `missing_declaration`/`missing-name`.
- Step 6: the object-form certificates are admitted and each altered entry is refused by the schema; the accepted certificate gives a `certified` record over an `uncertified` entry, and the rejected one a `proved`, `uncertified` record with `certificate-rejected{rule, at}` beside it.
- Step 7: the request carries the counterexample and the witness as
  `certificate`; the row settles `reproduced-with-validated-witness`,
  category `success`, with `replay` and `counterexample`; then `inconclusive`
  with `disagreement` `hybrid-trace` `not-exact` twice.
