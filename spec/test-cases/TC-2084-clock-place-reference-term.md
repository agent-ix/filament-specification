---
id: TC-2084
title: "A clock-place reference is a place-id term"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-829
    type: verifies
---

# TC-2084: A clock-place reference is a place-id term

## Description

Verify that `ref(p)` is carried as the `filament.op.clock_reference.ref`
application typed by the one `scalar_type`/`clock_reference` node, that it
and `present(p)` are evaluated per instance under a population binder and
through a reference, that `Reference<Clock>` is a data member, and that the
reader refuses the malformed shapes. Scope: FR-829-AC-1 to FR-829-AC-10.

## Test Procedure

1. Check and emit FR-829-AC-1's model; read the place-reference nodes of
   `ref(self.x)` (twice), `ref(other.x)` and `ref(self.q[1])`, and the type
   nodes of `r` and of a `Reference<A>` field; admit the package strictly.
2. Evaluate FR-829-AC-2's `only` and `count` rows at the two states named
   there and at a state that differs only in clock values.
3. Evaluate FR-829-AC-3's `count`, `sum`, `all` and `exists` forms over the
   two calls and read their `presence_read` and slot-domain nodes.
4. Check FR-829-AC-4's `ref(self.peer.x)`, `present(self.peer.v)` and
   `ref(self.v)` over an absent `v`, and evaluate them.
5. Emit FR-829-AC-5's `Tag` record and tuple, evaluate `eq` and `ne`, and
   read the catalog and the V2 schema `proposals/checked-package-v2/schema.json`.
6. Feed the strict reader each malformed variant of FR-829-AC-6.
7. Admit FR-154-AC-53's `acme/rpc` package, check and emit FR-829-AC-7's postcondition, read the type node of `r` and the postcondition's nodes, admit the package strictly, and read a post-state's key.
8. Compare clock references across states and universes, supply a set and a map in two entry orders with a repeated place id, and round-trip references through the state key and wire form (FR-829-AC-8 to FR-829-AC-10).

## Expected Results

- Steps 1 to 5: each value, node, path and enumeration is as FR-829-AC-1 to
  FR-829-AC-5 state.
- Step 6: each variant refuses with the code, cause and pointer FR-829-AC-6
  states.
- Step 7: `r` is typed by the `clock_reference` node with no node for `ClockRef`; the postcondition is the `eq` over the read of `r` and the place reference; the package is admitted; the key holds the place id.
- Step 8: equality, canonical order, duplicate and dangling refusals, and round-trip are as FR-829-AC-8 to FR-829-AC-10 state.
