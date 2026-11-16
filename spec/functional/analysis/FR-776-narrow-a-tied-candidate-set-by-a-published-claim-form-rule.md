---
id: FR-776
title: "Narrow a tied candidate set by a producer's published claim-form rule"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-290
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-711
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-489
    type: traces_to
  - target: ix://agent-ix/filament-spec-language/FR-962
    type: traces_to
---

# FR-776: Narrow a tied candidate set by a producer's published claim-form rule

## Description

`negotiate_*` never selects among several candidates (FR-290). A producer
whose own linked backends overlap on one capability kind by design SHALL be
allowed to resolve that overlap while it computes an item's candidates, by a
published rule, so that the item reaches negotiation with one candidate. Such
an item SHALL NOT settle `ambiguous-backend`.

Some overlaps are deliberate: two linked backends of one producer can each
decide a different form of claim within one kind, such as a reachability
search for an invariant over a model subject and a bounded checker for an
operation's contract. Making every caller name a backend for every such item
would put the producer's own routing knowledge into every request, and an
unnamed item would always be refused although the producer knows which
backend decides it. A published rule keeps the choice reproducible and keeps
negotiation the one place that settles an item.

## Inputs

An item that names no backend, its claim form, whether its request names a
model subject, and the matching set FR-290's candidate-set rule computes.

## Outputs

The item's candidate set: the matching set, or the one backend a row of the
producer's published rule selects.

## Behavior

- A rule SHALL read only the item's kind, its claim's form and whether the
  request names a model subject; never registration order, display names, a
  caller preference or ambient state.
- A row SHALL decide an item only when the matching set is exactly the
  backends the row names; a matching set holding any other backend, such as a
  process provider advertising the same kind, SHALL stay whole and settle
  under FR-290's candidate table.
- An item that names a backend SHALL never be narrowed.
- The narrowed item's FR-331 request item SHALL carry no `backend` member and
  SHALL carry the narrowed set as its `candidates`, so the request item is
  consistent under FR-290's candidate table and its identity is a function of
  the item and the registry.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-776-AC-1 | Over a producer whose published rule ties backends A and B on one kind, an unnamed item of a form the rule assigns to A reaches `negotiate_*` with candidates exactly `[A]`, no `backend` member, and settles through A's arm, never `ambiguous-backend`. | Test (TC-1881) |
| FR-776-AC-2 | Negative: with a third backend C matching the same item registered, the item keeps candidates `[A, B, C]` and settles `invalid-request` `invalid_capability`/`ambiguous-backend` naming all three; naming B gives candidates `[B]`. | Test (TC-1881) |

## Dependencies

- FR-290: the candidate set and table this rule feeds. FR-331: the request item. FR-711: the negotiation entry.
