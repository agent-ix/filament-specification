---
id: TC-1861
title: "A backend arm requires a bound the generic arm cannot see"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-771
    type: verifies
---

# TC-1861: A backend arm requires a bound the generic arm cannot see

## Description

A backend-specific arm turns the generic `supported` into `requires-bound`
only when its form check finds a domain that needs a finite bound the item
lacks. Scope: FR-771-AC-1, FR-771-AC-2.

## Test Procedure

Negotiate FR-771-AC-1's item on the Kani candidate without and with the
`collection-size` bound; then on a linked and a process candidate with the
same manifest; then an `unbounded-extent` item and an item the Kani arm
cannot lower under any bound.

## Expected Results

- Kani: `requires-bound` without the bound, `supported` with it.
- Linked and process: `supported`. The `unbounded-extent` item stays `unsupported`; the unlowerable form settles `unsupported` with the arm's warning.

Test: filament-contract-codegen `tests/negotiate_item.rs` `a_backend_arm_requires_a_bound_the_generic_arm_cannot_see`, traced `FSpec-TC-1861`.
