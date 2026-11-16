---
id: FR-741
title: "Step every weak-memory access shape"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-435
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-434
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-439
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-436
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-437
    type: depends_on
---
# FR-741: Step every weak-memory access shape

## Description

Under a resolved weak memory model (`tso` or `ra`, FR-434), the evaluator
SHALL give every access shape the meaning below and SHALL NOT settle an item
`unsupported` with cause `weak-access-shape`. The checker SHALL refuse no
access shape.

| Shape | Meaning |
| --- | --- |
| An attempt whose shared footprint holds two or more locations, with an any-object location instantiated to its object and a population membership one location | A sequence of single-location accesses in program order. First, one load step per shared location read, in canonical location order, each reading the value the model gives the thread and holding it in the thread's pending-access record, which is part of the state key. Then one apply step that evaluates the operation over the loaded values and writes one store per changed shared location in canonical location order: appended to the thread's store buffer under `tso`, or one message per location under `ra`, each later message's view including the earlier ones. A precondition false over the loaded values leaves the thread waiting at the apply step. |
| A multi-location read-modify-write | A locked transaction in one step. Under `tso`, it is enabled only with an empty store buffer, reads and writes memory, and is a full fence. Under `ra`, it reads the modification-order-latest message at each location and writes each new message immediately after it, with the attempt's ordering, or `acq_rel` when it has none. |
| A reader (FR-435) of a shared location | A `relaxed` load in the reader's own step, with one successor per value the model admits for the reader's thread. |
| An ordering the instantiated kind does not take | The ordering restricted to the kind: on a load, `acq_rel` is `acquire` and `release` is `relaxed`; on a store, `acq_rel` is `release` and `acquire` is `relaxed`; an RMW with none is `relaxed`. The run record carries `ordering_restrictions`, a list of `{site, access, written, effective}`. |
| A `join` whose policy is not `all` | The join merges the completed branches. A cancelled `tso` thread's buffer still drains by memory steps under that thread's identity. A cancelled `ra` thread's messages stay, and its view is discarded. |

Under `sc` every attempt is one step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-741-AC-1 | An attempt writing two shared fields explores under `tso` and `ra`. Its stores flush in canonical location order under `tso`, and a two-field load in another thread can read the new first field with the old second field. | Test (TC-1788) |
| FR-741-AC-2 | A two-location compare-and-swap is one step, enabled under `tso` only with an empty buffer. No reachable state shows it applied at one location only. | Test (TC-1788) |
| FR-741-AC-3 | A reader of a shared location explores with one successor per visible value. A load carrying `release` explores as `relaxed`, and the record lists the restriction. | Test (TC-1789) |
| FR-741-AC-4 | A `join any … outstanding cancel` under `tso` explores, and the cancelled branch's buffered store flushes after the join. No item in TC-389's corpus settles `weak-access-shape`. | Test (TC-1789) |

## Dependencies

- [FR-434](FR-434-select-memory-models-for-parallel.md),
  [FR-435](FR-435-classify-memory-accesses-and-orderings.md),
  [FR-436](FR-436-explore-total-store-order.md),
  [FR-437](FR-437-explore-release-acquire-memory.md) and
  [FR-439](FR-439-bound-replay-and-report-weak-memory-runs.md).
