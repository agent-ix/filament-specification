---
id: FR-038
title: "Resolve predicate names under explicit lexical scopes"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-005
    type: implements
---
## Description

When resolving native predicate declarations and local bindings, the linker SHALL apply the selected grammar's explicit alias, declaration, parameter, let and quantifier scopes without ambiguous name selection.

## Inputs

Collected source-unit imports/profile aliases, linked-package declarations, predicate signatures and located call/local-name uses.

## Outputs

Each use resolved to one declaration and scope, or a located duplicate, shadowing, unresolved or ambiguous-name refusal.

## Behavior

Aliases are source-unit scoped; native predicate declaration names are unique in the linked package. A `let` or quantifier binder may have the same name as a visible parameter, and bare reads in that binder's body resolve to the local binder. That rule does not allow a local binder to shadow an alias, native declaration or another active local binder; duplicate parameters refuse. Different predicates may reuse a parameter name. A let initializer cannot refer to the binding it is introducing; a quantifier variable is not visible in its domain. Forward calls resolve after collecting declarations and remain subject to FR-033's cycle/type checks. Named predicate bodies use explicit parameters: implicit self/result/pre-state reads refuse. Callers may pass correctly anchored values. No overload or case-folded fallback selects a declaration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-038-AC-1 | Two units may use the same local alias spelling for different exact imports without cross-binding; a duplicate native predicate name in the package refuses. | Test (TC-038) |
| FR-038-AC-2 | Duplicate parameters and nested local-binder name collisions refuse, while distinct predicates may each use a parameter named x. | Test (TC-038) |
| FR-038-AC-3 | A let initializer and quantifier domain cannot read their newly introduced binder; uses in the respective body resolve correctly. | Test (TC-038) |
| FR-038-AC-4 | A forward acyclic call resolves to one declared predicate; unresolved, ambiguous and case-mismatched references refuse. | Test (TC-038) |
| FR-038-AC-5 | Implicit self/result/pre reads inside a named predicate refuse; explicitly passed caller-anchored values retain their binding. | Test (TC-038) |
| FR-038-AC-6 | In a named predicate with parameter `x`, a `let` binder named `x` is admitted and bare `x` reads in its body resolve to that local binder; the let initializer resolves `x` to the predicate parameter. | Test (TC-038) |
| FR-038-AC-7 | In a named predicate with parameter `x`, a quantifier binder named `x` is admitted and bare `x` reads in its predicate resolve to that local binder; the quantifier domain resolves `x` to the predicate parameter. | Test (TC-038) |
| FR-038-AC-8 | A local binder whose name is an in-scope alias refuses at that binder's source location. | Test (TC-038) |
| FR-038-AC-9 | A local binder whose name is a native declaration refuses at that binder's source location. | Test (TC-038) |
| FR-038-AC-10 | A nested local binder whose name is already active in an enclosing local scope refuses at that binder's source location. | Test (TC-038) |

## Dependencies

- [Proposed shared grammar](../../../proposals/filament-v1/shared-grammar.md).
- [Shared drafting foundation](../../../proposals/filament-v1/shared-foundation.md).

Accepted by FS03 for the composed state query/graph closure. It does not change
historical definition bytes or claim complete compiler/backend coverage.
