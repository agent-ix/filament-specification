---
id: TC-2037
title: "A synthesis declaration is one synthesis-request node"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322-AC-467
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-322-AC-468
    type: verifies
---

# TC-2037: A synthesis declaration is one synthesis-request node

## Description

Verify the `claim`/`synthesis_request` node a `synthesis` declaration
checks to, its `claim_names` row, and the reader's refusals of edited
bodies. Scope: FR-322-AC-467 and FR-322-AC-468.

## Test Procedure

1. Read the package of a unit declaring `type Digit = Int[0, 9];` and
   `synthesis Pick using p grammar Digit domain Digit satisfies { Pick * Pick = 49 };`.
2. Read the same unit with `minimize Pick`, and with a second declaration
   `Pick2` of the same grammar, domain and block.
3. Feed the reader each edit FR-322-AC-468 lists.

## Expected Results

- Step 1: one node, path `["Pick"]`, one `declaration` occurrence, body
  bindings `grammar`, `domain`, `candidate`, `objective` (empty),
  `satisfies`, `symmetry` `identity`; `dependencies` `Digit`, the candidate
  and the block root; one `claim_names` row named `Pick`.
- Step 2: `objective` is `minimize`, the candidate node and the text `Pick`;
  `Pick2` is a second node.
- Step 3: each refusal at the code and locus FR-322-AC-468 states.
