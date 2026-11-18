---
id: TC-2018
title: "Checked package records the observe-only boundary"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-822
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-465
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-466
    type: verifies
---

# TC-2018: Checked package records the observe-only boundary

## Description

Verify the `boundary` member of role and control nodes, the `recommend`
control layout and the reader's boundary check. Scope: FR-822-AC-4,
FR-322-AC-462, FR-465-AC-14 and FR-466-AC-19.

## Test Procedure

1. Emit the package of a clause with `observe role Auditor`, `observe sequence
   Review` and a `recommend`, and of the same clause without `observe`.
2. Read the package with an `attempt` added under `Review`, with an `attempt`
   by `Auditor`, with a `boundary` text other than `observe-only`, with
   `boundary` not last, and with a `boundary` member on a non-protocol node.

## Expected Results

- Step 1: exactly the `Auditor` role and `Review` control nodes carry the last
  argument `boundary` `observe-only`; the unmarked package's nodes carry none
  and differ from the marked one only at those two nodes; `recommend` is a
  `protocol`/`control` node of operator `recommend` with the arguments `role`,
  `operation`, `contracts`, `binder`, `related`, `constraint`.
- Step 2: `ill_typed`/`operator-ineligible` at the attempt node with
  `boundary` the `Review` node, then the `Auditor` role node; `malformed_wire`
  for the wrong text, the misplaced member and the non-protocol node.
