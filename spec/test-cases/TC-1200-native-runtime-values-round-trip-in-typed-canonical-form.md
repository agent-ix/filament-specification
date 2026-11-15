---
id: TC-1200
title: "Native-runtime values round-trip in typed canonical form"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-550
    type: verifies
---

# TC-1200: Native-runtime values round-trip in typed canonical form

## Description

Native-runtime values round-trip in typed canonical form. Scope: FR-550-AC-1.

## Test Procedure

Build one FR-323 request per FR-181 kind listed in FR-550-AC-1, selecting an identity function with an argument in that kind's typed canonical form, and run it on the reference runtime.

## Expected Results

Each item completes with the same typed canonical form, a rational normalized and a set's elements in canonical order.
