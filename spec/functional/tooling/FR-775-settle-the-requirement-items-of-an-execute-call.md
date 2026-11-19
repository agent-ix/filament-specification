---
id: FR-775
title: "Settle the requirement items of an execute call when the request asks"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-300
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-711
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-776
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-777
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-960
    type: traces_to
  - target: ix://agent-ix/filament-spec-language/FR-961
    type: traces_to
  - target: ix://agent-ix/filament-spec-language/FR-963
    type: traces_to
---

# FR-775: Settle the requirement items of an execute call when the request asks

## Description

An `execute` request that calls a function MAY ask for the called function's
requirement items to be settled: the items written from the requirement
records whose owner is that function. When it asks, the producer SHALL route,
negotiate and settle each of those items by the rules every provider-routing
operation uses, and SHALL return one FR-331 terminal record per item beside
the evaluation. When it does not ask, `execute` settles no item and its
outcome is the evaluation alone.

A function's requirement items say whether its body stays inside its declared
types for every input, which is a different question from the value one call
returns. A caller that runs a function and wants that evidence would
otherwise need a second `analyze` request naming the same items. Settling
them in the same request lets the run and its evidence share one compilation
and one package, so the records describe the package that was executed.

## Inputs

- The `execute` request's selection of requirement settlement. It names, for
  every item or for one item by its FR-331 claim `{node, occurrence}`, the
  `BackendId` of the backend that decides it and that backend's options
  (FR-196). Naming follows the request-item rule of every provider-routing
  operation: a named registered backend is the item's only candidate when it
  matches; a named unregistered backend carries the unknown-backend mark; an
  item naming none has every matching registered backend as a candidate,
  narrowed only as FR-776 permits.
- The provider registry, the caller's negotiation entry (FR-711) and the
  run's provider limits (FR-777).

## Outputs

The evaluation, as FR-300's `execute` row states, and, when settlement was
asked for, one FR-331 terminal record per item in package order, each with its
request item, its disposition or result, and its diagnostic where a
disposition settled it, and the registry the items were negotiated against.

## Behavior

- Each item SHALL be negotiated through FR-711's item entry; no item reaches
  a provider by any other route, and no provider is called before every item
  is negotiated.
- An item routed to a process provider SHALL be sent as an FR-331 request
  whose `package` is the emitted bytes of the package that was executed.
- When the executed package is a selected checked package rather than a
  compile, the items, their owner and their package order SHALL be read from
  the package's `requirement_items` rows and `claim`/`value_validity` nodes
  ([FR-825](../../objects/interfaces/FR-825-carry-the-requirement-records-of-a-checked-package.md)),
  and the items and their request indices SHALL equal those of a compile of
  the package's source; no item is rebuilt from the graph and none is
  dropped.
- The evaluation SHALL be the same with and without settlement.
- The outcome's category SHALL be the most severe of the evaluation's and the
  items' categories.
- A request that asks for settlement and names an item that is not an item of
  the called function SHALL be refused before any item is negotiated.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-775-AC-1 | An `execute` request calling a function with three requirement items and asking for settlement with no named backend, over a registry whose one `value-validity` backend matches each item, returns the evaluation and three terminal records in package order, each negotiated through FR-711's entry; the same request not asking returns the same evaluation and no record. | Test (TC-1880) |
| FR-775-AC-2 | Naming an unregistered backend for every item settles each `invalid-request` `invalid_capability`/`unknown-backend` naming it, with the evaluation unchanged and the outcome's category refusal; naming a registered process provider sends it one FR-331 request holding every item, after every item is negotiated, whose `package` is the executed package's emitted bytes. | Test (TC-1880) |
| FR-775-AC-3 | Negative: a settlement selection naming a claim that is not an item of the called function is refused before any item is negotiated, and no provider is called. | Test (TC-1880) |

## Dependencies

- FR-300: the `execute` operation. FR-290, FR-331, FR-711: candidates, request items and negotiation.
- FR-776: narrowing an unnamed tied item. FR-777: the run's provider limits.
