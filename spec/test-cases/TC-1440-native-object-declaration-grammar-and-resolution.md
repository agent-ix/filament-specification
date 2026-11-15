---
id: TC-1440
title: "Native object declaration grammar and name resolution"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-610
    type: verifies
---

# TC-1440: Native object declaration grammar and name resolution

## Description

Verify that the complete-V1 `object-decl` productions parse every part of an
object declaration in source order, refuse malformed declarations at the
offending token, and that one-segment names resolve only to source
declarations and two-segment names only to model declarations. Scope:
FR-610-AC-1 and FR-610-AC-2.

## Test Procedure

1. Parse the three declarations of FR-610-AC-1 and compare each part
   (abstract flag, name, supertypes, identity list, fields, populations,
   operations with parameters, result, `redefines` and frame) in source order.
2. Parse `operation step()` with no `;`, `modifies self.` with no field name
   and an operation parameter named `modifies`; record each diagnostic.
3. In a unit with `model M = "test/orders";` selecting a document that
   declares `Order`, check `invariant I using P on Order at current { ... }`
   and the same clause `on M::Order`, and a protocol with `role w on M`.
4. Check `object M { }` in a unit that also declares `model M`.

## Expected Results

Step 1 yields the parts in source order. Each step 2 input is one
`invalid_syntax`/`unexpected-token` at the offending token. In step 3 `Order`
resolves to the source type, `M::Order` to the model type and `role w on M` to
the whole model. Step 4 refuses `ambiguous_declaration`/`ambiguous-name` at
both declarations.
