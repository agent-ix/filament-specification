---
id: TC-2081
title: "Document refusal locus on the native-run-result/2 wire"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: verifies
---

# TC-2081: Document refusal locus on the native-run-result/2 wire

## Description

Verify that the strict `native-run-result/2` reader reads a refusal whose
locus is a `document` locus and keeps the locus kind set closed. Scope:
FR-352-AC-23.

## Test Procedure

1. Read a stage `admit` clause-run document refused
   `invalid_runtime_input`/`missing-member` whose `disposition.refusal.locus`
   is the `document` locus FR-352-AC-23 states.
2. Read the same document with `kind` set to `snapshot`.
3. Read it with `member` removed.
4. Read it with an added locus member `line`.

## Expected Results

- Step 1: the document reads and the refusal retains the equal locus.
- Steps 2-4: each is refused naming `disposition.refusal.locus`.
