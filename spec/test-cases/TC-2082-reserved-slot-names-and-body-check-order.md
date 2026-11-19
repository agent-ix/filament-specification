---
id: TC-2082
title: "Reserved slot names and the hyperproperty body check order"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-402
    type: verifies
---

# TC-2082: Reserved slot names and the hyperproperty body check order

## Description

Verify that a source identifier in the reserved slot form is refused where it
would name a `value`/`parameter` node, that a reader classifies a parameter
as a slot parameter by its name alone, and that the hyperproperty step's Body
check runs its checks in the stated order. Scope: FR-402-AC-12,
FR-402-AC-13.

## Test Procedure

1. Check `function f(op_sa: Integer): Integer pure { op_sa }`.
2. Check `pre F using v on Config::ConfigVersion::attemptUpdate
   { forall(slot_sx_sy in self.us: slot_sx_sy.x < 5 ms) }` over a
   `us: Sequence<Timer>` field.
3. Check the function of step 1 with its parameter renamed, in turn, to
   `op_s_x41`, `slot_sv`, `op_sa_sb`, `rcv_s`, `op_sa_x4`, `op_s_xC3`,
   `op_s_xc3` and `slotted`.
4. Read `Whole`'s package (FR-402-AC-11) with its body wrapper formula
   edited to read an Integer `value`/`parameter` node named, in turn,
   `op_sx`, `op_s_x41`, `arg_sa_sVault_sstep_si`, `op_sa_sb`, `slot_sv`,
   `rcv_s`, `op_s_xC3`, `op_s_xc3` and `slotted`.
5. Read `Whole`'s package with the wrapper's `fairness` argument made
   non-empty and its `over` set to the `model_root` node of a selected
   package no `subjects` binding names.
6. Read the package of step 5 with the `fairness` argument emptied.
7. Read `Whole`'s package with the hyperproperty node's `body` retargeted to
   a Boolean `expression` node of the package, which FR-370's placement
   check admits in a hyperproperty body.

## Expected Results

- Steps 1 and 2: each refused `ambiguous_declaration`/`ambiguous-name` at
  the identifier's declaring source region, naming `op_sa` with tag `op`
  and `slot_sx_sy` with tag `slot`; no package is emitted.
- Step 3: `op_s_x41` refused as in step 1, naming the tag `op`; the other
  seven each admitted.
- Step 4: `op_sx`, `op_s_x41` and `arg_sa_sVault_sstep_si` each refused
  `invalid_model_binding`/`malformed-declaration` at
  `/semantic_graph/nodes/{w}/body/arguments/0`; the other six admitted at
  the temporal and hyperproperty steps.
- Step 5: refused `invalid_semantic_graph` at
  `/semantic_graph/nodes/{w}/body/arguments/4`.
- Step 6: refused `invalid_model_binding`/`malformed-declaration` at
  `/semantic_graph/nodes/{w}/body/arguments/0`.
- Step 7: refused `ill_typed`/`operator-ineligible` at the hyperproperty
  node by the Body check, with no wrapper check reported.
