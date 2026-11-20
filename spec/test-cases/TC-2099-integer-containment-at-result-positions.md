---
id: TC-2099
title: "Integer containment at result positions"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---

# TC-2099: Integer containment at result positions

## Description

Verify that the strict reader admits an integer term whose type node a
containing integer type node integer-contains, with no convert node, at the
result positions of FR-322 "Integer containment" (a function result,
each branch of an `if`, a `let` body, a `case` arm body and a `let`
binder), and that it widens nothing else: a range that is not contained,
`Integer` at an `Int[..]`, a constructor over a contained element and a bare
call argument still refuse. Scope: FR-322-AC-495 to FR-322-AC-502.

## Test Procedure

1. Validate and read each case IC-01 to IC-20 of
   `proposals/checked-package-v2/integer-containment-vectors.json`.
2. For each admitted case, confirm that the graph holds no `convert` node
   over the contained term.

Test: `tests/checked_package_v2.rs` `tc_2099_integer_containment`. The
vector file and the test are delivered with the strict-reader change under
QSL-2709 (contract-ir strict reader and vectors); until then this procedure
has no executable test.

## Expected Results

- IC-01 (`Int[0, 9]` returned as `Int[0, 10]`) and IC-02 (returned as
  `Integer`) admitted with no `convert` node.
- IC-03 (returned as `Int[0, 8]`) and IC-04 (`Integer` returned as
  `Int[0, 10]`) `ill_typed`/`type-mismatch` at the function node.
- IC-05 (call argument converted to `Int[0, 10]`) admitted; IC-06 (bare
  argument at an `Int[0, 10]` parameter) and IC-07 (bare argument at an
  `Int[0, 8]` parameter) `ill_typed`/`operator-ineligible` at the call
  application.
- IC-08 (`Sequence<Int[0, 9]>[0, 3]` returned as
  `Sequence<Int[0, 10]>[0, 3]`) `ill_typed`/`type-mismatch` at the function
  node.
- IC-09 (`if`), IC-10 (`case` of result `Integer`) and IC-11 (`let` body)
  admitted with no `convert` node; IC-12 (`if` of result `Int[0, 8]`)
  `ill_typed`/`operator-ineligible` at the `if` application; IC-13 (`case` of
  result `Int[0, 9]` with an `Int[0, 20]` arm) `ill_typed`/`operator-ineligible`
  at the `case` node.
- IC-14 (`let y: Integer = x in g(y)`) admitted with no `convert` node; IC-15
  (binder typed `Int[0, 999]` over an `Int[0, 1000]` value)
  `ill_typed`/`operator-ineligible` at the `let` application; IC-19 (binder
  typed `Int[0, 1000]`, passed bare to the `Integer` parameter)
  `ill_typed`/`operator-ineligible` at the call application; IC-20 (binder
  typed `Money` over a `TaxedMoney` value) admitted.
- IC-16 (contained else-branch) and IC-17 (`result_type` the hull
  `Int[0, 20]` of `Int[0, 9]` and `Int[5, 20]` branches) admitted; IC-18
  (`Int[0, 1000]` else-branch under an `Int[0, 10]` result, no narrow)
  `ill_typed`/`operator-ineligible` at the `if` application.
