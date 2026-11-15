---
id: TC-1320
title: "Index a sequence or ordered set under the index obligation"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-144-AC-31 FR-144-AC-32
    type: verifies
---

# TC-1320: Index a sequence or ordered set under the index obligation

## Description

Verify the FR-144 indexing form over sequences and ordered sets: the element-typed result, the FR-146 index obligation, the refusals for unordered and non-collection operands, and the `filament.op.collection.index` producer node.

## Test Procedure

Select `filament.value.complete/v1`. Check and evaluate the admitted reads of FR-144-AC-31 (`s[0]`, `s[i]`, `o[1]`, `r.xs[0]`) under their guards, recording each value, its static type and its charges, and read the emitted checked package. Then check each refused form of FR-144-AC-32 with no guard.

## Expected Results

The admitted reads return `7`, `7`, `8` and the first element, each typed by the element type, each charging one `collection.element` and one `collection.result-retain` after both operands, and each one `filament.op.collection.index` application. Every unguarded index is `undefined_expression`/`unproved-presence` at the index; a set, bag, integer or record operand is `ill_typed`/`operator-ineligible`; a Boolean index is `ill_typed`/`type-mismatch`; `s[0..1]` is `invalid_syntax`.
