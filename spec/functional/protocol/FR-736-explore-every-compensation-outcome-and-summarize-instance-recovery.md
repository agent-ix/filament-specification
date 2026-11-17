---
id: FR-736
title: "Explore every compensation outcome and summarize an instance's recovery"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-427
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-236
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-058
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-173
    type: depends_on
---
# FR-736: Explore every compensation outcome and summarize an instance's recovery

## Description

The protocol evaluator SHALL reach, in exploration, every per-attempt outcome
and every recovery-relation aggregate fact of FR-236's partition. It SHALL
also report, for each protocol instance, a recovery summary over its
registrations. FR-427's step table gains three compensation steps:

| Step | Enabled when | Effect |
| --- | --- | --- |
| `cfail` | as `cattempt`, without a post-state: the attempt is dispatched under the authored attempt count, before the commit, and its record satisfies the retry relation at a retry | the model state is unchanged; the attempt's outcome is `failed`; the attempt count advances |
| `ctimeout` | as `cfail` | as `cfail`, with outcome `timed-out` |
| `cexpire` | the compensation thread rests at its attempt or retry point | the thread ends without evaluating the recovery predicate, and the registration's aggregate fact is `timed-out` |

Under an untimed reading, which reads no clock, `ctimeout` and `cexpire` are
enabled whenever their thread waits. This admits every deadline's
behaviours, as an `await` timeout does. A `cattempt` has outcome
`successful`.

The aggregate fact of a registration is:

- `pending` while its thread runs;
- `successful` after `cend` with a true recovery predicate;
- `failed` after `cend` with a false one;
- `timed-out` after `cexpire`;
- absent for a registration that never activates.

A retried attempt carries its predecessor's index. Retry is a relation, and
the retried attempt keeps its own outcome label.

## Instance recovery summary

The recovery summary of a protocol instance is a fact of its own subject,
keyed by the instance. It is not a label of any attempt or registration,
and it never overwrites one. Its value is:

| Value | When |
| --- | --- |
| `untriggered` | no registration of the instance activated |
| `pending` | some activated registration's fact is `pending` |
| `recovered` | every activated registration's fact is `successful` |
| `partially-recovered` | at least one activated registration is `successful`, at least one is `failed` or `timed-out`, and none is `pending` |
| `unrecovered` | at least one registration activated, none is `successful`, and none is `pending` |

The five rows partition the instances: exactly one row holds for each
instance. `pending` takes precedence, so an instance with a successful, a
failed and a pending registration is `pending` until the pending one settles,
and only then `partially-recovered`: a registration that has failed keeps it
from `recovered`.

The summary is derived only from the registrations' aggregate facts, never
from an attempt label, arrival order or a label of another subject (FR-236).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-736-AC-1 | Over TC-380's `Pay` with `attempts 2`, exploration reaches an attempt with each of `failed`, `timed-out` and `successful`, and a successful retry whose predecessor keeps `failed`; no registration exceeds two attempts. | Test (TC-1782) |
| FR-736-AC-2 | Exploration reaches registrations whose aggregate facts are `pending`, `successful`, `failed` and `timed-out`, and one that closes with no fact. | Test (TC-1782) |
| FR-736-AC-3 | A protocol with two templates reaches instance summaries `partially-recovered`, `recovered`, `unrecovered` and `untriggered`, and `pending` in a state with a running thread, including a state where one registration is `successful`, one `failed` and one `pending`, which is `pending` and not `partially-recovered`. No summary value is admitted as an attempt or aggregate label. | Test (TC-1783) |

## Dependencies

- [FR-427](FR-427-check-protocol-compensation.md): the compensation steps.
- [FR-236](../../objects/protocol/FR-236-compensation-attempt-outcome.md):
  the outcome partition.
- [FR-058](FR-058-preserve-retry-and-partial-recovery.md) and
  [FR-173](FR-173-preserve-compensation-lifecycle.md): retries and the
  lifecycle.
