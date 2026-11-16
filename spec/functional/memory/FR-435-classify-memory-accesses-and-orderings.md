---
id: FR-435
title: "Classify memory accesses, orderings and fences"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-013
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-434
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-439
    type: references
---

# FR-435: Classify memory accesses, orderings and fences

## Description

When a protocol clause holds a `parallel` control, the checker SHALL classify
every attempt inside it as a load, store, read-modify-write or local access of
at most one shared location, SHALL admit only the orderings each class takes,
and SHALL apply each access under a weak model through the thread observation
stated below.

## Inputs

A checked protocol clause with its attempts, their optional `ordering`
members and `fence` controls; the operations' read and write footprints and
frames ([FR-013](../foundation/FR-013-validate-operation-frames.md)); and the
resolved memory model of each `parallel`
([FR-434](FR-434-select-memory-models-for-parallel.md)).

## Outputs

The access class of every instantiated attempt and every fence, with the
ordering restrictions applied ([FR-741](FR-741-step-every-weak-memory-access-shape.md)); or a located refusal of an ordering.

## Behavior

### Orderings and fences

The [shared grammar](../../../proposals/filament-v1/shared-grammar.md)'s `event-node`, `memory-order`, `fence` and
`fence-order` productions carry the forms below.

An `attempt` inside a `parallel` may carry `ordering o`. An attempt with no
`ordering` is **non-atomic**. The control `fence Name ordering o;` issues a
fence in its thread and is a thread-owned step of kind
`fence`, with the transition identity members `thread`, `node` and
`ordering`. A fence step is uncounted for intervals and has no operation.

### Locations and classification

A **location** is (object, field), (object type, field, any object), or
(population, membership). Each attempt's read and write footprints are
instantiated per transition identity over the subject's universes and
argument domains. A location is **shared** in a weak `parallel` when the
instantiated footprints of attempts in two or more of its branches contain it
and at least one of them writes it. Every other location an attempt touches
is **branch-local**: a register of its thread.

An `event`, `effect`, `commit`, `send` or `receive` node inside a branch of a
weak `parallel` whose constraint reads a model location (an event constraint
may read `over`, [FR-464](../../objects/interfaces/FR-464-checked-package-protocol-node-bodies.md))
is a **reader** of that location, and the location counts toward sharing as an
attempt's read does. A reader has no access class or ordering. A reader's read of a shared
location is a `relaxed` load in the reader's own step (FR-741). A `finish` and a node outside every weak `parallel` read
the joined memory and are not readers; under `sc` a reader is admitted.

A location encodes as `{"type":"location","object":<reference>,"field":"<name>"}`,
`{"type":"location","object_type":"<qualified-name>","field":"<name>"}` for
any object, or `{"type":"location","population":"<qualified-name>"}` for a
membership.

Each instantiated attempt is one of:

| Class | Shared footprint | Admitted orderings |
| --- | --- | --- |
| load of `ℓ` | `{ℓ}`, read only | `relaxed`, `acquire`, `seq_cst` or none |
| store to `ℓ` | `{ℓ}`, written only, with no pre-state read of `ℓ` | `relaxed`, `release`, `seq_cst` or none |
| read-modify-write (RMW) of `ℓ` | `{ℓ}`, read in the pre-state and written | one of the five, never none |
| local | empty | none |

A load or store with an ordering is **atomic**; one without is non-atomic.
The checker classifies every attempt in every `parallel` symbolically,
whatever its source default. Under `sc` orderings change no behaviour.

### Admitted orderings

The checker sees which fields an attempt's operation reads and writes, not
whether they are shared, so the class is known only when the pre-check
instantiates the footprint. At check time the checker SHALL refuse an
ordering only when no class the footprint leaves possible takes it, with
`ill_typed`/`operator-ineligible` naming the attempt and the ordering:

- any ordering on an attempt that touches no location, shared or branch-local;
- `release` or `acq_rel` on an attempt that writes nothing;
- `acquire` or `acq_rel` on an attempt that reads nothing.

The checker SHALL admit every other ordering, and an absent ordering, on an
attempt that touches a location, shared or branch-local: an RMW with no ordering and a `release`
attempt that reads a field and also writes a register field are admitted at
check time.

Under a resolved weak model, the evaluator SHALL restrict the ordering to
the instantiated kind and list the restriction in the run record
(FR-741), for:

- an instantiated load that carries `release` or `acq_rel`;
- an instantiated store that carries `acquire` or `acq_rel`;
- an instantiated RMW that carries no ordering;
- an instantiated local access, whose shared footprint is empty, that carries
  any ordering.

Each single-location access of a multi-location attempt takes the
attempt's ordering restricted the same way. Under `sc`, an attempt whose instantiated class does not
take its ordering SHALL be admitted, because orderings change no behaviour
under `sc`.

### Multi-location accesses

Under a weak model, an attempt instance whose shared footprint holds two or
more locations is a sequence of single-location accesses in program order,
and a multi-location read-modify-write is a locked transaction (FR-741). An
any-object location is instantiated to its object, and a population
membership is one location. The checker refuses no access shape. Under `sc`
every attempt is one step.

### Guards and checks

A choice guard, repeat guard or `check` reads binders and constants only
([FR-426](../protocol/FR-426-apply-protocol-steps.md)), under every memory
model. A guard or check that reads a model location refuses at check time
`ill_typed`/`operator-ineligible` at the read, naming the location; the
author writes a load attempt and guards on its binder.

### Applying an access under a weak model

An access applies its operation by the FR-181 successor rule over the
**thread observation**: the model state with `ℓ` holding the value the memory
model gives the reading thread, and branch-local locations as they are. The
precondition reads that value, so a load whose precondition constrains `ℓ` is
enabled only for values that satisfy it. The postcondition's delta at `ℓ`
goes to the memory model; branch-local writes apply in place. An RMW
application whose post-state leaves `ℓ` unchanged is a load with the read
part of its ordering.

Under `ra`, the value the model gives the reading thread at a shared location
is the value of the message the access chose, at the location a load or the
read part of an RMW reads, and otherwise the value of the message the thread's
current view points at. A store's precondition, and a local access's, read the
current-view message.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-435-AC-1 | In the store-buffering protocol, `Sx` and `Sy` classify as atomic stores, `Ly` and `Lx` as atomic loads, `(x, v)` and `(y, v)` as shared and `(c1, r)` and `(c2, r)` as branch-local. | Test (TC-389) |
| FR-435-AC-2 | At check time, an ordering on an attempt that touches no location, shared or branch-local, `release` on an attempt that only reads and `acquire` on an attempt that only writes each refuse `ill_typed`/`operator-ineligible` naming the attempt and the ordering; an RMW with no ordering and a `release` attempt that reads a field and writes a branch-local register are admitted at check time; each class with each admitted ordering is admitted. | Test (TC-389) |
| FR-435-AC-3 | An attempt writing two shared fields of its receiver under a weak default checks with no refusal; under a `tso` selection it explores as FR-741's load steps and apply step, with no `weak-access-shape` result; under `sc` it is one step. | Test (TC-389) |
| FR-435-AC-4 | A choice guard that reads a model field refuses `ill_typed`/`operator-ineligible` at the read, under `sc`, `tso` and `ra` alike; the same guard over the binder of a load attempt of that field is admitted. | Test (TC-389) |
| FR-435-AC-5 | A load whose precondition requires the value 1 is enabled under `tso` only when its thread observation of the location is 1, and an RMW whose post-state leaves the location unchanged takes no write step. | Test (TC-389) |
| FR-435-AC-6 | `fence F ordering relaxed;` is `refused { code: invalid_syntax, cause: unexpected-token }` at the `relaxed` token, because `fence-order` admits only `acquire`, `release`, `acq_rel` and `seq_cst`; an admitted fence is a step with members `thread`, `node` and `ordering`, uncounted for intervals. | Test (TC-389) |
| FR-435-AC-7 | A branch holding an `event` whose constraint reads `x.v`, beside a branch whose attempt stores `x.v`, explores under a `tso` selection with the read a `relaxed` load in the event's step, one successor per visible value, and under `sc` reads memory; an `event` reading a field of an object that no attempt accesses, and a `finish` reading `x.v` after the join, explore under `tso`. | Test (TC-389) |
| FR-435-AC-8 | Under `ra`, a store whose precondition requires `x.v = 0` is enabled only while its thread's current view points at a message of `x` with value 0, whatever the mo-last message holds; a load of `x` whose precondition requires `x.v = 1` is enabled for each message at or after the current view whose value is 1. | Test (TC-389) |
| FR-435-AC-9 | Under a `tso` selection and under an `ra` selection, each of these explores with its ordering restricted and listed in `ordering-restrictions` as `{site, access, written, effective}`: a load with `release` (`relaxed`), a load with `acq_rel` (`acquire`), a store with `acquire` (`relaxed`), a store with `acq_rel` (`release`), an RMW with no ordering (`relaxed`), and an attempt with `release` that touches no shared location (access `local`, no memory effect). Under `sc` each is admitted unchanged. An attempt with `acquire` that writes two shared fields explores as FR-741's multi-location access, each sub-access taking the restricted ordering. | Test (TC-389) |

## Dependencies

- [FR-013](../foundation/FR-013-validate-operation-frames.md) owns frames and
  footprints.
- [FR-434](FR-434-select-memory-models-for-parallel.md) owns the resolved
  model and the observation and write contributions.
- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns the
  `event-node`, `control`, `fence`, `memory-order` and `fence-order`
  productions.

## References

- Design: filament-spec-language ADR-025 MM-6, MA-1 to MA-6, MV-1; FSL FR-220
  states the same check-time and pre-check ordering rules (FR-220-AC-2,
  FR-220-AC-14).
- Tickets: Linear STD-138; paired QSL-372.
