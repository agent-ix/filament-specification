---
id: TC-1830
title: "An element-presence quantifier is scoped in its predicate"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-760
    type: verifies
---

# TC-1830: An element-presence quantifier is scoped in its predicate

## Description

An element-presence quantifier in a timed clause predicate binds its own
parameter, and its binder is never a part or invariant binder. Scope:
FR-760-AC-1, FR-760-AC-2, FR-760-AC-3.

## Test Procedure

Read a package holding the guard part, time invariant and urgency of FR-760-AC-1; then the variants adding `binders` naming `e` to the guard part and to the invariant; then the variant with the `exists` application.

## Expected Results

- The AC-1 package and the `exists` variant are admitted, the guard having one part.
- Each `binders` variant refuses `invalid_package`/`timed-node-reference` at the binder target.

Test: filament-spec-language `fsl-package/src/emit/tests/timed_clauses.rs` `an_element_presence_quantifier_is_scoped_in_its_predicate`, traced `FSpec-TC-1830`.
