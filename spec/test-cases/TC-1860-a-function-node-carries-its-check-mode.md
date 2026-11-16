---
id: TC-1860
title: "A function node carries its check mode"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-770
    type: verifies
---

# TC-1860: A function node carries its check mode

## Description

Every function node carries a closed, identity-bearing `check_mode`, and a
declared clause's function carries `linked`. Scope: FR-770-AC-1,
FR-770-AC-2.

## Test Procedure

Read a package with `linked` on every function node, then the same package
with `kernel` on a declared function and a `body` clause function, then a
dependency diamond that disagrees on one node's mode; then the variants with
the member removed, set to `"strict"`, `""` and `true`, placed on an
`expression` node, and set to `"kernel"` on an `invariant` and on a
`precondition` clause function.

## Expected Results

- The first two packages are admitted and unequal; the diamond refuses `invalid_package`/`conflicting-definition`.
- The missing, malformed and misplaced members refuse `malformed_wire`; `kernel` on the clause functions refuses `invalid_package`/`invalid-value`.

Test: `tests/checked_package_v2.rs` `tc_1860_a_function_node_carries_its_check_mode` (schema and vectors); filament-spec-language `fsl-analyze/tests/it/check_mode_record.rs`, traced `FSpec-TC-1860` (producer).
