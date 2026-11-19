---
id: TC-2054
title: "Optional operation parameters"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-154
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-151
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2054: Optional operation parameters

## Description

Verify that an operation parameter declared `optional` in a domain package is
read by intake, typed `Option<X>`, checked under the option rules, may be
omitted by a trailing call position, is checked contravariantly on
redefinition, and binds a checked-package parameter node whose identity reads
its presence. Scope: FR-154-AC-52, FR-151-AC-20, FR-151-AC-21, FR-322-AC-476.

## Golden fixture

Domain package `test/refunds` (semantic IR 2.0.0). Object types `Staff`
(field `badge`, `ix://filament/native/Integer`, `required`, `[1, 1]`), `Order`
(field `total`, `ix://filament/native/Integer`, `required`, `[1, 1]`) and `Desk`
(fields `order` typed `ix://test/refunds/Order` and `staff` typed
`ix://test/refunds/Staff`, each `required`, `[1, 1]`). `Order` declares the
query operation `note` (empty frame, result `ix://filament/native/Boolean`
`[1, 1]`) whose `params` are, in order:

```json
[
  {"identity": "ix://test/refunds/Order/note/firstApprover",
   "name": "firstApprover", "typeRef": "ix://test/refunds/Staff",
   "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": false},
   "presence": "required", "nullable": false, "defaultKind": "none",
   "origin": {"source": {"sourceIdentity": "ix://test/refunds/source",
     "path": "Order.md", "startLine": 10, "startColumn": 3}}},
  {"identity": "ix://test/refunds/Order/note/secondApprover",
   "name": "secondApprover", "typeRef": "ix://test/refunds/Staff",
   "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": false},
   "presence": "optional", "nullable": false, "defaultKind": "none",
   "origin": {"source": {"sourceIdentity": "ix://test/refunds/source",
     "path": "Order.md", "startLine": 11, "startColumn": 3}}}
]
```

Variants: (V1) `secondApprover`'s `presence` absent, `null` or `"maybe"`;
(V2) an option value type `OptStaff` (`kind` `option`, `inner`
`ix://test/refunds/Staff`) typing `secondApprover`, presence `optional`; (V3)
the same with presence `required`; (V4) `secondApprover` `required`; (V5) an
object type `RushOrder` with supertype `Order` declaring `note` with
`redefines` `ix://test/refunds/Order/note` and the same parameters, under the
three presence pairs of step 5.

Unit (model alias `R`):

```text
model R = "test/refunds";
pre NoteBy using v on R::Order::note { present(secondApprover) }
pre NoteVal using v on R::Order::note {
  present(secondApprover) implies deref(value(secondApprover)).badge > 0
}
pre NoteSame using v on R::Order::note { secondApprover = secondApprover }
```

## Test Procedure

1. Admit the fixture, then each V1 variant, V2 and V3. Tag
   `#[trace("TC-2054", "FR-154-AC-52")]`.
2. Check the unit; then check it with each added clause body on
   `R::Order::note`: `value(secondApprover) = firstApprover` and
   `deref(secondApprover).badge > 0`. Tag `FR-151-AC-20`.
3. Add, one at a time, a `Desk` invariant whose body is each call of
   FR-151-AC-20 compared `= true`. Tag `FR-151-AC-20`.
4. Emit the checked package of the unit, of V4 with `NoteSame` only, and of V3;
   emit the `Desk` invariant package with the omitted and with the explicit
   `absent` call; hand-edit the first to drop the call's third operand and read
   it. Tag `FR-322-AC-476`.
5. Admit V5 with (base, redefining) presences (`optional`, `optional`),
   (`required`, `optional`) and (`optional`, `required`). Tag `FR-151-AC-21`.

## Expected Results

- Step 1: the fixture is admitted with presences `required` and `optional`;
  `firstApprover` is `Reference<Staff>` and `secondApprover`
  `Option<Reference<Staff>>`. Each V1 variant and V2 refuses
  `invalid_model_binding`/`malformed-declaration` at the `Order/note` node, path
  `Operation.params[1].presence`. V3 is admitted with `secondApprover`
  `Option<Reference<Staff>>`.
- Step 2: the unit checks. `value(secondApprover) = firstApprover` refuses
  `undefined_expression`/`unproved-presence` at `value(secondApprover)`;
  `deref(secondApprover).badge > 0` refuses `ill_typed`/`type-mismatch` at
  `deref(secondApprover)`.
- Step 3: `note(self.staff)`, `note(self.staff, absent)` and
  `note(self.staff, Some(self.staff))` check; `note()` refuses
  `ill_typed`/`type-mismatch` at the call naming `note` and position 0;
  `note(self.staff, self.staff)` refuses `ill_typed`/`type-mismatch` at the
  second argument; `note(self.staff, absent, self.staff)` refuses
  `ill_typed`/`type-mismatch` at the call naming `note` and position 2.
- Step 4: the `secondApprover` node is typed by the option node over `Staff`'s
  reference node; under V4 its `node_id` differs while the `Order` declaration
  node and the `NoteSame` clause node keep theirs; under V3 its `node_id` and
  `semantic_type` equal the fixture's. The omitted-argument call is one
  `dispatch_call` with three operands, the third the `none` literal typed by the
  option node, and the two call packages are byte-equal; the hand-edited package
  refuses `ill_typed`/`operator-ineligible` at the call.
- Step 5: the first two pairs are admitted; the third refuses
  `ill_typed`/`variance-parameter` on the parameter presence axis at position 1,
  with no parameter type axis refusal at that position.
