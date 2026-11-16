---
id: FR-661
title: "Hold a consumer-fact inventory and bind each fact to a producer's public accessor"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-002
    type: implements
  - target: ix://agent-ix/filament-specification/FR-660
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-419
    type: references
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
  - target: ix://agent-ix/filament-specification/FR-110
    type: references
---

# FR-661: Hold a consumer-fact inventory and bind each fact to a producer's public accessor

## Description

The standard SHALL publish an inventory that lists, for each consumer of a
checked package, the facts that consumer reads from a producer, each tied to the
requirement that defines it. Each producer that implements the standard SHALL
bind every fact to a public accessor of one public module per consumer, checked
by a contract test that fails on any fact without a resolving accessor.

## Rationale

Consumers may not copy a producer's logic, so every fact they need must be
reachable through a public API. Without a list of those facts, each missing
accessor is found by a consumer one ticket at a time. With the list in the
standard and a contract test in the producer, a missing accessor is a failing
producer gate or a specification review finding before any consumer meets it.

## Inputs

The inventory document `proposals/consumer-facades/consumer-facts.json`, and a
producer's binding document.

## Outputs

A contract-check report that names each fact with its resolved public path, or
names each fact that has no binding, a binding that names no fact, and each
accessor that does not resolve.

## Behavior

### The inventory

The inventory is the published file
`proposals/consumer-facades/consumer-facts.json` (a published file under
FR-660, read through `published_file`). It is a closed JSON object:

- `inventory`: exactly `filament.consumer-facts/v1`.
- `consumers`: an array of closed objects `{consumer, role, facts}`, one per
  consumer, in this order: `ir` (the Contract IR reader and lowering), `protocol`
  (the protocol link layer), `code-generation` (function replay and generated
  code), `runtime` (the runtime monitor), `driver` (the run driver and its result
  settlement) and `observation` (the observation binder). `role` is one sentence
  naming what the consumer does with the facts.
- `facts`: a non-empty array of closed objects `{id, fact, requirement}`. `id` is
  `CF-<code>-<nn>`, where `<code>` is `IR`, `PR`, `CG`, `RT`, `DR` or `OB` for
  the six consumers in order and `<nn>` is a two-digit ordinal that starts at
  `01` and increases within the consumer; the id of a removed fact is retired
  and never reused. `fact` is one sentence naming the fact the consumer reads. `requirement` is the `ix://` URI
  of the requirement that defines the fact, in this standard or in a producer
  repository that implements it.

The inventory is versioned with the standard (FR-660): adding a fact is a
compatible release, and removing a fact or changing its meaning is a breaking
one. An id is never reassigned to another fact.

### A producer's bindings

A producer that implements the standard publishes a binding document in its own
repository. For each consumer it names one public module of its facades, and for
each inventory fact of that consumer one or more accessors in that module. An
accessor is an item name (`channel_endpoints`, `CHECKED_PACKAGE_V2`) or a member
of a public type (`ImportedFunctionSite::library`,
`SchedulabilityRoot::obligation_identity`), where the member is a public method
or a public field. An accessor resolves when a crate outside the producer can
name it at `<module>::<accessor>` through public paths alone.

The producer's contract check reads the inventory from its pinned standard
release and its own binding document. It passes only when every inventory fact
has a binding, every binding names an inventory fact, and every bound accessor
resolves. It fails on each other case and names every failing fact and path in
one report. The binding document has no pending, ignored or deferred state: a
fact that a requirement defines is bound to an accessor that exists, or the
check fails.

### Requirements that name a consumer fact

A requirement, in this standard or in a producer, that defines a fact a consumer
reads through a producer's public API SHALL name the fact's inventory id. The
change that adds such a requirement adds the inventory fact and the producer's
binding in the same release. A requirement that names a consumer fact without an
inventory id, or an inventory fact whose `requirement` does not define it, is a
specification review finding.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-661-AC-1 | The published inventory is admitted by its closed shape: `inventory` is `filament.consumer-facts/v1`, the consumers are exactly `ir`, `protocol`, `code-generation`, `runtime`, `driver` and `observation` in that order, every id has its consumer's code and strictly increasing ordinals from `01`, no id repeats, and every `requirement` names an existing requirement artifact; an inventory with an extra member, a missing consumer, a consumer out of order, a repeated id or a requirement that names no artifact is refused, naming the defect. | Test (TC-1645) |
| FR-661-AC-2 | The inventory lists at least: for `ir`, the document type, the import view of an admitted dependency package, the schedulability claim roots and the obligation identity of a schedulability claim; for `protocol`, the protocol view, the causal relation, channel endpoints (sender, receiver, message type), the await anchor, choice visibility and the compensation attempts maximum; for `code-generation`, the call site of a function, operation or clause, the imported-function site with its declaring package and function occurrence, the obligation identity of a site, and the closed-form verdict check; for `runtime`, the monitor plan root and the progress view admitted from a request or event input; for `driver`, the bounded run-result reader, the clause run-result writer and the typed run result; for `observation`, the subject kinds and relationship declarations taken from the checked package's domain declarations. | Test (TC-1645) |
| FR-661-AC-3 | A producer's contract check over its pinned inventory and its binding document passes and lists every fact with its resolved public path when every fact is bound and resolves; it fails, naming the fact and the path, when a bound accessor is not public or does not exist; it fails, naming the fact, when an inventory fact has no binding; and it fails, naming the binding, when a binding names an id the inventory does not hold. | Test (TC-1646) |
| FR-661-AC-4 | Every requirement that defines a fact a consumer reads through a producer's public API names that fact's inventory id, and every inventory fact's `requirement` defines that fact. | Inspection (TC-1647) |

## Dependencies

- [FR-660](FR-660-version-the-wire-the-published-files-and-the-consumer-facades.md)
  owns the versioning and pinning of the inventory and of the bound facades.
- The requirements each inventory row names own the facts.
