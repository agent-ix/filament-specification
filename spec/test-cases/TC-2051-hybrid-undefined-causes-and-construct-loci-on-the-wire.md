---
id: TC-2051
title: "Hybrid undefined causes and construct loci on the wire"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-193
    type: verifies
---
# TC-2051: Hybrid undefined causes and construct loci on the wire

## Description

Verify that a reader resolves an unsupported construct's `locus` through the
checked package it holds and refuses a locus, mode or variable the
declaration lacks or a node outside the mode's entries and the
functions they call, and that a hybrid `undefined-evaluation` counterexample
carries exactly one of the four catalogued hybrid undefined reasons. Scope:
FR-193-AC-25, FR-193-AC-26 and FR-193-AC-27.

## Test Procedure

1. Over `Braking`, read an `unsupported` `expression-form` entry whose
   `locus` names the term node of `v`'s rate in `Moving`; repeat with the
   `locus` naming a guard node of `Moving`, with a node of no hybrid
   declaration, and with `mode` `Halted`.
2. Validate and read hybrid counterexamples whose `undefined-evaluation`
   `cause` is each of `division-by-zero`, `outside-parameter-range`,
   `outside-result-range` and `outside-capture-range`, then `absent-key` and
   `undefined`; read the native diagnostics catalog's "Undefined reasons".
3. Over `BrakingH` (FR-193-AC-27), read an `unsupported` `expression-form`
   entry, `mode` `Moving` and no `variable`, whose `locus` names the `/` in
   `half`'s body; repeat with the `locus` naming the `/` in `spare`'s body,
   with `mode` `Stopped`, and with `variable` `v`.

## Expected Results

- Step 1: the construct is read; then `invalid_result`/`invalid-value` at
  `/diagnostic/detail/locus` twice and at `/diagnostic/detail/mode`.
- Step 2: the four are admitted by the schema and the reader; `absent-key`
  and `undefined` fail the schema and are refused `invalid_result`/
  `invalid-value` at the kind's `cause`; the catalog lists the four.
- Step 3: the construct is read, located in `half`'s declaration; then
  `invalid_result`/`invalid-value` at `/diagnostic/detail/locus` three times.
