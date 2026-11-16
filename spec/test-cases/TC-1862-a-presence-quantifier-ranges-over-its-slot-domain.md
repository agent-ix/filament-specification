---
id: TC-1862
title: "A presence quantifier ranges over its collection's slot domain"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-772
    type: verifies
---

# TC-1862: A presence quantifier ranges over its collection's slot domain

## Description

The first operand of a presence-only quantifier is the slot-domain
application, with a derived path that names the clock place by an operand
segment. Scope: FR-772-AC-1, FR-772-AC-2.

## Test Procedure

Read FR-772-AC-1's package and its `exists` variant with a second quantifier
over `self.q`; then the variants with an extra slot term, no owner, a
`scalar_type` operand 0, a `Boolean` binder, the application used as a
`count` operand, and a path embedding the `clock_place` `node_id`.

## Expected Results

- The package and the variant are admitted, sharing one slot-domain node.
- Each negative variant refuses as FR-772-AC-2 states.

Test: filament-spec-language `fsl-package/src/emit/tests/timed_clauses.rs` `a_presence_quantifier_ranges_over_its_slot_domain`, traced `FSpec-TC-1862`.
