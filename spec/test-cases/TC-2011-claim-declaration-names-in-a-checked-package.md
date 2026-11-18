---
id: TC-2011
title: "A checked package names every non-temporal claim in claim_names"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2011: A checked package names every non-temporal claim in claim_names

## Description

Verify that a checked package carries one `claim_names` row per declaration
of every state-clause, refinement and non-temporal claim node, outside every
node identity, and that the reader refuses dangling, duplicate and uncovered
rows. Scope: FR-322-AC-457 and FR-322-AC-458.

## Test Procedure

1. Compile a unit declaring the invariant `NotAboveThree` and the
   state-graph claim `ReachThree`; read `claim_names`.
2. Compile two structurally equal state clauses `A` and `B`; rename
   `NotAboveThree` and compare node ids; compile a unit with no claim node.
3. Mutate the step 1 package: a dangling `declaration`, a `temporal_clause`
   `declaration`, an occurrence the node lacks, a `qualified_name` ending in
   another segment, two rows with one `qualified_name`, a repeated key, a
   removed row, the member removed, and a row with an extra member.

## Expected Results

- Step 1: two rows with `qualified_name` `["NotAboveThree"]` and
  `["ReachThree"]`, in `NodeRef` then occurrence order.
- Step 2: one node, two rows; equal node ids after the rename; no member.
- Step 3: `invalid-value` at `declaration`, `declaration`, `occurrence` and
  `qualified_name`; `ambiguous_declaration`/`ambiguous-name`;
  `conflicting-definition`; `invalid-value` at `/claim_names` twice;
  `malformed_wire`.
