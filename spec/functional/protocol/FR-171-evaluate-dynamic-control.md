---
id: FR-171
title: "Evaluate joins, proved loops and dynamic roles"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-052
    type: references
  - target: ix://agent-ix/filament-specification/AD-007
    type: references
  - target: ix://agent-ix/filament-specification/FR-368
    type: references
---

# FR-171: Evaluate joins, proved loops and dynamic roles

## Description

When checking dynamic protocol control, the checker SHALL validate each join,
loop and role-spawn against a closed control and population contract.

## Inputs

Control graph, join policy, branch states, loop invariant/variant, dynamic-role
population/lifetime/closure and resource bounds.

## Outputs

Checked control semantics or typed ambiguity, termination, closure or bound
refusal/incomplete outcome.

## Behavior

Join policies are `all`, `any`, `quorum(n)` or a named pure predicate and define
outstanding-branch cancellation/continuation. Loops require invariant preservation
and a well-founded variant for termination claims. Dynamic roles bind unique
instances to declared populations and lifetimes; finite execution requires a
discharged population bound.

## Control and population rules

`all` settles after every branch settles; `any` settles after the first branch;
`quorum(n)` settles after `n` distinct branches; and a predicate join settles
only when its pure predicate is true over one Boolean per joined label, in
ascending label order, `true` for a `completed` branch and `false` for a
`running` one ([FR-466](../../objects/interfaces/FR-466-checked-package-protocol-control-bodies.md)). Each
join declares whether unsettled branches continue or receive cancellation.
A loop invariant holds before the first iteration and after every body step; a
lexicographic well-founded variant decreases on every back edge.

A loop's invariant and variant are discharged over the explored protocol
subject, as two obligations of the loop: the invariant holds at every decision
of the loop (on entry of a pre-test loop and after every completed body
iteration), and on every back edge the variant is strictly smaller in its
well-founded order (an integer at least 0, or a lexicographic tuple of such
integers) than at the decision that entered the iteration. An obligation is
`proved` only when exploration expanded every reachable state with no limit,
truncating method bound or cancellation; `refuted` with a replayable finite
counterexample to the violating decision or back edge, including an undefined
invariant or variant; and `incomplete`, naming what stopped exploration,
otherwise. A proved result claims the invariant or termination for the
explored subject and nothing beyond it. A loop with a maximum terminates by
that maximum and has no termination obligation. Each obligation is one derived
request item over the protocol subject, of `item_kind` `loop-invariant` or
`loop-termination`, anchored at the loop's `repeat` node
([FR-368](../temporal/FR-368-negotiate-temporal-model-check-items.md) "Item
kinds").

A spawn allocates a fresh `(protocol_instance, role_declaration, ordinal)`
identity within its closed population. Lifetime is lexical, explicit-until or
protocol-instance; retirement forbids later actions but preserves history.
Population closure and a finite upper bound are separate premises.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-171-AC-1 | Every join policy settles only under its exact branch condition and applies its declared outstanding-branch behavior. | Test (TC-202) |
| FR-171-AC-2 | A loop with proved invariant/variant is admitted; an unproved termination claim is inconclusive/refused rather than silently bounded. | Test (TC-202) |
| FR-171-AC-3 | Spawn/lifetime/population closure creates and retires exact role instances; missing symbolic bounds return requires-bound for finite execution. | Test (TC-202) |
| FR-171-AC-4 | Each join policy is tested at zero, below-threshold, threshold and post-threshold branch states. | Test (TC-202) |
| FR-171-AC-5 | A retired dynamic role cannot act or be silently replaced by a new role with the same display name. | Test (TC-202) |
| FR-171-AC-6 | A loop with no maximum whose variant decreases on every iteration settles its invariant and termination obligations `proved` over a complete exploration; a body that leaves the variant unchanged settles termination `refuted` with a counterexample ending at that back edge; a false invariant at a decision settles the invariant `refuted`; and exploration stopped by a state limit settles both `incomplete`, naming the limit. A variant whose type has no well-founded order (not an integer or a tuple of integers) refuses `ill_typed`/`type-mismatch` at the variant. | Test (TC-1520) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
