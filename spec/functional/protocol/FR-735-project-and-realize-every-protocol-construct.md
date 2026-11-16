---
id: FR-735
title: "Project and decide realizability for every protocol construct"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-174
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-175
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-171
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-427
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-428
    type: depends_on
---
# FR-735: Project and decide realizability for every protocol construct

## Description

Local projection (FR-174) and realizability (FR-175) SHALL apply to every
construct of a checked protocol. That covers replicated roles and their
lifetimes, per-trigger activation, symbolic queue capacity, every join
policy and `outstanding` rule, pre-test and post-test `repeat`, timed
`await`, compensation, and the `attempt`, `effect`, `event`, `commit`,
`check` and `fence` controls. FR-174-AC-3's non-conclusive answer applies
only to an exhausted bound, or to a limit that a request or the selected
system sets. It does not apply to a construct for want of a projection
rule.

## Decision points

A *decision point* is a point at which a protocol's run can continue in more
than one way. Each decision point has alternatives and an owner. A branch
merges for a role (FR-174 "Projection correctness") only when the role's next
visible actions in the alternatives are compatible, or when the role has
distinguishing evidence. The owner takes the decision as a local internal
choice, or as an external choice keyed by distinguishing receives. When the
owner is the environment, every role takes the external side.

| Decision point | Alternatives | Owner |
| --- | --- | --- |
| `choice` | its cases | its `by` role |
| `repeat` guard read | continue, exit, and exhausted when a maximum is declared | its `by` role |
| `await` | match then `then`; timeout then `timeout` | the role that owns the match event |
| `join` whose policy is not `all` | the disposition sets at which the policy first holds, with remainders cancelled or continuing per `outstanding` | the environment |
| compensation activation | activated, not activated | the role whose step binds the trigger record |
| commit cutoff of a compensation thread | retry, end | the role of the commit node |
| acting instance of a replicated role's event | the live instances | the environment |

## Behavior

- **Ownership.** `attempt`, `effect`, `event` and `commit` are local
  internal actions of their owning role. `check` and `fence` belong to no
  role: they are erased from every local graph, and the causal order
  through them is kept.
- **Replicated roles.** A replicated role class projects to one local
  behaviour, which each live instance runs over its own observation. An
  event of the class on a thread is taken by the instance that took the
  thread's previous event of the class (its session instance). The first
  such event on a thread is a receive, so that the instance learns from the
  message that it is the one to act. A protocol that violates this has no
  sound projection, and the counterexample names the event and two
  instances that could take it.
- **Per-trigger activation.** Each protocol instance has its own threads,
  captures and queues (FR-428). A role's local behaviour is the projection
  of the body, started by the instance's activation, which every role of the
  instance observes. Observations of two instances are never equal.
- **Symbolic capacity.** A projection does not depend on capacity. A
  realizability result takes each named capacity's value as a channel
  premise. When the value is absent, the result is inconclusive (FR-175-AC-3).
- **Realizability moves.** The environment chooses activations, spawns and
  retirements, the records and post-states of attempts, effects, events,
  commits and compensation attempts, when a join's policy fires, and
  timeouts. A role chooses its enabled actions from its local history, and
  a compensation template's role chooses at each retry point whether to
  retry or to end. A role's observation includes its own internal actions
  and their records, its timeouts, its instance identity and, for a
  replicated role, its own instance reference.
- **Counterexample.** A projection counterexample names the decision point,
  its kind and owner, the role, the two global paths from the decision
  point to the role's first divergent action, and the two actions
  (FR-174-AC-2).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-735-AC-1 | A projection request over a protocol holding each construct listed in the Description returns a local graph, a counterexample or an exhausted-bound result, never a non-conclusive result naming the construct. | Test (TC-1780) |
| FR-735-AC-2 | A `repeat` whose exit and continue alternatives give a non-owner role next actions of the same direction, peer and message type, with no distinguishing evidence, yields a counterexample whose decision point is the `repeat` and whose owner is its `by` role. | Test (TC-1780) |
| FR-735-AC-3 | A replicated role whose first event on a thread is a send yields a counterexample naming that event and two live instances. The same role whose first event on a thread is a receive projects. | Test (TC-1781) |
| FR-735-AC-4 | A realizability request over a symbolic capacity with no stated value is inconclusive, and with a stated value it is decided. A compensation whose recovery needs a retry is realizable, with a strategy that retries. | Test (TC-1781) |

## Dependencies

- [FR-174](FR-174-project-global-protocols.md) and
  [FR-175](FR-175-analyze-protocol-realizability.md): the analyses this
  requirement extends to every construct.
- [FR-171](FR-171-evaluate-dynamic-control.md),
  [FR-427](FR-427-check-protocol-compensation.md) and
  [FR-428](FR-428-check-replicated-roles-and-per-trigger-activation.md): the
  constructs' step semantics.
