---
id: TC-2098
title: "A type argument written as an alias carries the alias's declaration key"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2098: A type argument written as an alias carries the alias's declaration key

## Description

Verify that the `type_argument` member of an application whose source type
argument names a type alias carries that alias's original declaration key as
`alias`, beside the resolved node it names as before; that the strict reader
admits and refuses `alias` by the five rules of FR-322 "Named type
arguments", each at its locus; and that `alias` changes no check and no
value. Scope: FR-322-AC-491 to FR-322-AC-493.

## Test Procedure

1. Emit the unit of FR-322-AC-491 (`type Small = Int[0, 4]`, a field
   `xs: Sequence<Int[0, 9]>[0, 4]`) with the leaves `count<Small>`,
   `count<Int[0, 4]>`, `sum<Small>`, `fold<Small>`, `reduce<Small>`,
   `convert<Small>` and a population `count`; read each application's
   `type_argument` member and `node_id`; emit the `Small = Tiny`,
   `Tiny = Int[0, 4]` variant.
2. Validate and admit each case TA-01 to TA-09 of
   `proposals/checked-package-v2/type-argument-alias-vectors.json`.
3. Admit the domain-package case of FR-322-AC-492: an alias `Small` of
   `Int[0, 4]` keyed on a member whose `declaration` is the `Int[0, 9]` node.
4. Evaluate every application of the TA-01 package and of its base package;
   strip every `alias` from step 1's package and evaluate both; repeat with
   each source key replaced by another admitted key, and with an
   operator-ineligible `result_type` variant of each.

Test: `tests/checked_package_v2.rs` `tc_2098_named_type_argument_alias`
(step 2: the schema verdict, outcome and locus of each case) and FSL
`fsl-package/src/emit/tests` (step 1). Steps 3 and 4 need a reader and an
evaluator, which this repository does not hold; they have no executable test
here.

## Expected Results

- Step 1: `count<Small>` and every `<Small>` form carry
  `alias: {package: "I", node: "ix://I/Small"}` beside the `Int[0, 4]`
  node; `count<Int[0, 4]>` and the population `count` carry none; the
  `count<Small>` and `count<Int[0, 4]>` applications have distinct
  `node_id`s; the chain variant's key is `ix://I/Small`; no byte spells
  `ix://source/`.
- Step 2: TA-01 and TA-02 admitted; TA-03 and TA-04 `malformed_wire` at the
  member's `alias`; TA-05 `missing_declaration`/`missing-selection` at
  `.../alias/package`; TA-06 and TA-07 `invalid_package`/`invalid-value` at
  `.../alias/node`; TA-08 `missing_declaration`/`missing-name` at
  `.../alias/node`; TA-09 `ill_typed`/`type-mismatch` at node 7's `alias`,
  naming nodes 5 and 7.
- Step 3: `ill_typed`/`type-mismatch` at `.../member/alias`.
- Step 4: equal admission, equal values, equal charges and equal refusals of
  the defective variants with and without `alias`.
