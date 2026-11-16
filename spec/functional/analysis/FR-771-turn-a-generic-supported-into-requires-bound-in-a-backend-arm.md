---
id: FR-771
title: "Turn a generic supported into requires-bound in a backend-specific arm"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-711
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-196
    type: depends_on
---

# FR-771: Turn a generic supported into requires-bound in a backend-specific arm

## Description

The generic arm (FR-711) reads only the manifest and the item's extent. It
settles `supported` when the backend advertises the item's kind and mode
over its domains. A backend may still be able to decide an item's IR form
only over finite domains: a Kani harness needs a finite bound for every
value it enumerates or assumes (FR-196 "Kani mapping contract"). The
generic arm cannot see that, because the extent classifies the claim's
parameters and not every type the form reads. The type could be a binder's
collection or an operand inside a callee.

A backend-specific arm SHALL therefore settle `requires-bound` naming the
candidate, in place of the generic `supported`, exactly when all of these
hold:

1. the generic arm settled `supported`;
2. the backend's own form check, run on the item's IR form, finds a type
   node the form reads whose domain is unbounded (for example an unbounded
   `Integer`, an unbounded collection or an unbounded population), or a
   bounded type whose finite bound the form needs and the item's
   `extent.bounds` do not supply;
3. a finite `ProofBound` exists that would make the form decidable, so the
   caller can supply it and negotiate the item again.

When no finite bound could make the form decidable, the arm settles
`unsupported` with its warning, not `requires-bound`. When the form is
malformed, it settles `invalid-request`. `requires-bound` here means what
it means in FR-290's advertised-mode table: the item becomes decidable by
this backend once the caller substitutes a finite bound. It is never a
refusal, and an item settled so is never run.

The Kani arm of `filament-contract-codegen` is such an arm: its scalar,
collection and graph form checks report a required bound for an unbounded
or bound-missing type, and the arm settles `requires-bound`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-771-AC-1 | Over a Kani backend advertising (`value-validity`, `bounded`) and an item of `bounded` extent whose claim parameters carry `integer-range` bounds, but whose form reads a binder over an unbounded `Sequence<Integer>`, the generic arm settles `supported` and the item entry returns `requires-bound` naming the Kani backend; the same item with a `collection-size` bound for that sequence settles `supported`. | Test (TC-1861) |
| FR-771-AC-2 | Negative: the same item over a linked or process candidate with the same manifest settles `supported` (no backend-specific arm adds the bound requirement); an item the generic arm settles `unsupported` `unbounded-extent` stays `unsupported` on the Kani candidate; an item whose form the Kani arm cannot lower under any bound settles `unsupported` with the arm's warning, never `requires-bound`. | Test (TC-1861) |

## Dependencies

- FR-711: the generic and backend-specific arms.
- FR-290: the meaning of `requires-bound`.
- FR-196: the Kani mapping contract's finite domains.
