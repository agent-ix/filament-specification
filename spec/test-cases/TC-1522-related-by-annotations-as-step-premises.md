---
id: TC-1522
title: "Related-by annotations as step premises"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-051
    type: verifies
---

# TC-1522: Related-by annotations as step premises

## Description

Exercise a `related by` annotation as a premise of its event step and of observed-event binding. Scope: FR-051-AC-9.

## Test Procedure

Over orders `o1`, `o2` and payments settling each, explore a protocol over `o1` whose event node is annotated `related by settles (paid, input)`; add an annotation no payment satisfies; swap two annotations; bind an observed event whose payment settles `o2`.

## Expected Results

Steps bind only the payment settling `o1`; the unsatisfiable annotation leaves no step and the thread waits; swapped annotations give the same steps; the foreign payment refuses `invalid_runtime_input`/`invalid-value` naming the node and the annotation.
