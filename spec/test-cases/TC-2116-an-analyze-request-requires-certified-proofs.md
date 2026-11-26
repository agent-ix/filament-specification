---
id: TC-2116
title: "An analyze request requires certified proofs"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-520
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-301
    type: verifies
---

# TC-2116: An analyze request requires certified proofs

## Description

A `native-analyze/1` request's `require_certified` member, and the
`--require-certified` flag that sets it, settle a proof that is not
`certified` `inconclusive`; the default settles it `proved`.

Scope: FR-520-AC-9, FR-301-AC-8.

## Test Procedure

1. Run FR-520-AC-9's request without `require_certified`, with `false`, with
   `true` and with `"yes"`.
2. Run the request without the member with `--require-certified`, and `check`
   with `--require-certified`.

## Expected Results

- Step 1: `proved` `uncertified` twice; `inconclusive`,
  `proof-not-certified`; the request refused `invalid-request`, exit `20`.
- Step 2: the same document as step 1's `true` run; `check` refuses the
  option, exit `20`.
