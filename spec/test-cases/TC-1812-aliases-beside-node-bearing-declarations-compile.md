---
id: TC-1812
title: "Aliases beside node-bearing declarations compile"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-746
    type: verifies
---

# TC-1812: Aliases beside node-bearing declarations compile

## Description

Verify that an alias in a unit or package that has a node compiles, introduces no node and resolves across units. Scope: FR-746-AC-3, FR-746-AC-4.

## Test Procedure

Under the TC-1810 header, compile a unit of `type I = Integer;` and `dimension Length;`, then a unit of `type I = Integer;` and `function f(x: I): I pure { x }`. Then compile a package of two units, the first holding only `type Port = Integer;` and the second only `function f(p: Port): Port pure { p }`, in both unit orders, and check the first unit alone.

## Expected Results

- The first package holds the `Length` dimension node and no node for `I`.
- The second package's function node `f` has parameter and result type the node of `Integer`, and the package holds no node for `I`.
- The two-unit package checks and compiles in either order, and `f`'s parameter and result type is the node of `Integer`.
- The alias-only unit alone refuses as TC-1811 states.
