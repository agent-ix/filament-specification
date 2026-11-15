---
id: FR-053
title: "Enforce choice ownership and information visibility"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-006
    type: implements
  - target: ix://agent-ix/filament-specification/FR-052
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-227
    type: references
---
## Description

When admitting a labeled protocol choice, the protocol linker SHALL require one owning role and evidence that the owner can distinguish every selected branch from facts available at its decision point.

## Inputs

A finite choice node, owning role, labeled guards, predecessor graph and declared
role-visible facts/messages.

## Outputs

An admitted owned choice with explicit visibility premises, or a located typed
unobservable-choice refusal.

## Behavior

The linker SHALL reject ownerless, multiply owned, unlabeled and overlapping
choices. The linker SHALL reject a choice when its distinguishing fact is
available only to another role and no causal message exposes it to the owner.
Rejecting an unobservable choice SHALL NOT imply that every global trace is
invalid or that a general projection theorem has been attempted.

The protocol input `over` denotes the model state at each step, which no role
observes at its decision. The linker SHALL refuse a choice or repeat guard, a
`visible` entry or a `check` condition that reads `over` with
`ill_typed`/`operator-ineligible`, located at the smallest term that reads
`over` (the field-read path rooted at it, or the bare name) and naming the
slot (FR-464). A decision that depends on the input reads a protocol capture of
it instead: the capture fixes the value at activation and every role holds it,
so the owner knows it and the choice projects to the owner's local behaviour.
A guard may read a protocol capture without a `visible` entry.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-053-AC-1 | A fulfillment-owned ship/cancel choice is admitted when an earlier message supplies the distinguishing payment outcome. | Test (TC-053) |
| FR-053-AC-2 | The same choice is refused when the payment outcome remains visible only to the external provider. | Test (TC-053) |
| FR-053-AC-3 | Ownerless, multiply owned, duplicate-label and guard-overlap mutants are refused independently. | Test (TC-053) |
| FR-053-AC-4 | The refusal identifies the exact choice, owner and unavailable distinguishing fact without reporting global conformance success. | Test (TC-053) |
| FR-053-AC-5 | A choice whose guards read a protocol capture of the input (`capture ready: Boolean = input.ready`) is admitted with no `visible` entry; the same guards reading `input.ready` directly each refuse `ill_typed`/`operator-ineligible` located at the term `input.ready`, naming the guard slot, and never at the guard block, the choice or the protocol declaration. | Test (TC-1521) |

## Dependencies

- [FR-052](FR-052-represent-bounded-control.md).
- Agent A's typed predicate and scope rules.
