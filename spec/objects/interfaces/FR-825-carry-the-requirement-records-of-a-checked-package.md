---
id: FR-825
title: "Carry the requirement records of a checked package"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-484
    type: references
  - target: ix://agent-ix/filament-specification/FR-775
    type: references
  - target: ix://agent-ix/filament-specification/FR-640
    type: references
---

# FR-825: Carry the requirement records of a checked package

## Description

When a producer checks a package, it records one requirement record per
claim the package asks a backend to settle: the claim's occurrence key, its
FR-290 capability kind and its extent, and for a `value-validity` claim of a
function body the facts of its claim site (the function that owns it, its
result bound, its direct conversion, its path condition, its extent roots
and the object types it reads). The producer SHALL write these records into
the `filament.checked-package/v2` package: one `requirement_items` row per
record, in package order, and one `claim`/`value_validity` node per
`value-validity` record. The reader SHALL admit them and expose them as a
typed read.

A consumer that holds only the package bytes (a run of a selected package,
FR-775) needs these records to answer which requirement items belong to a
called function, what each item's request index is, and what each item
requests. Without them it would have to rebuild ownership, guards, bounds and
extents from the graph, which re-derives what the producer's checker
decided, or answer with fewer items than the source route, which is a partial
map. With them, the source route and the package route read one value per
fact.

## Properties

### Requirement items

`requirement_items` is a top-level member of the package. It is absent
exactly when the package has no requirement record, and otherwise is a
non-empty array with one row per requirement record. Each row is exactly:

```json
{
  "claim": {"node": {"package": "<package>", "path": ["<segment>"], "ordinal": 0},
            "occurrence": {"role": "expression", "ordinal": 0}},
  "kind": "value-validity",
  "extent": {"classification": "bounded"},
  "record": {"package": "<package>", "path": ["<segment>"], "ordinal": 0}
}
```

- `claim` is the record's occurrence key: the local structural `NodeRef` of
  the node the record is keyed at and one of that node's occurrences
  `{role, ordinal}`. It is the FR-331 request item's `claim` for the record's
  item, so the key the package carries and the key a provider receives are
  one value.
- `kind` is the record's FR-290 capability kind, one of the closed
  vocabulary of FR-331's `kind` (`value-validity`, `operation-contract`,
  `finite-replay`, `temporal-satisfaction`, `global-conformance`,
  `monitorability`, `local-projection`, `refinement`, `realizability`,
  `composition`, `probabilistic-satisfaction`, `schedulability`). Two records
  of different kinds may share one occurrence, and each is its own row.
- `extent` is the record's extent classification as FR-331 spells an
  item's extent before any proof bound is substituted:
  `{"classification": "bounded"}` for a record with no unbounded domain, or
  `{"classification": "unbounded", "domains": [...]}` with one entry
  `{"domain": <DomainKey>, "kind": <domain kind>}` per unbounded domain.
  `DomainKey` is FR-331's `{"type": "node", "node": <NodeRef>, "path":
  [<u32>...]}` or `{"type": "population", "member_type": <NodeRef>,
  "ordinal": <u32>}`, and the domain kind is one of FR-331's seven
  (`collection`, `population`, `integer`, `recursive`, `quantity`, `loop`,
  `infinite-trace`). Domains are in ascending domain-key order: every `node`
  key before every `population` key; `node` keys by `node` structural key,
  then `path` element by element numerically, a proper prefix first;
  `population` keys by `member_type` structural key, then `ordinal`. No key
  repeats. `finite_bound_available` is not written: it is a function of the
  domain kinds (FR-331), so the request writer computes it.
- `record` is present exactly on a `value-validity` row and names that
  record's `claim`/`value_validity` node ("Value-validity record node").
  Every other row has no `record`.

**Package order.** Rows are in package order: ascending `claim.node`
structural key (FR-322 "Structural node identity", the order of structural
keys, never graph order), then `claim.occurrence.role` in UTF-8 byte order,
then `claim.occurrence.ordinal` numerically, then `kind` in UTF-8 byte
order. No two rows share a (`claim`, `kind`) pair. A row's position in the
array is the record's *request index*: the index FR-075's request writer and
FSL FR-395's call association give the record's item. The package order is
the order of every record of the package, not of one function's records, so
a request index is the same whichever function a caller selects.

The member is not a graph node and changes no `node_id`. What a record reads
is carried by graph nodes whose identity covers it (the record's `claim` node
for every kind, and the `claim`/`value_validity` node for a `value-validity`
record), so the rows add no identity of their own. Like `claim_names`, the
member is no digest or hash input (FR-201).

### Value-validity record node

A `value-validity` record is the claim that one occurrence of a scalar
operation application in a function body is defined and lies in its result
bound under its path condition (FR-290's claim-form table, FR-484). The
producer SHALL write each such record as one node with `node_tag: "claim"`
and `semantic_form: "value_validity"`. It is a compiler-derived node (FR-322
`source_map`): it has no `declaration` member and exactly one occurrence,
with role `generated`, whose source region is the region of the application
occurrence the record is keyed at. Its owner is the function whose body holds
the claim site, and its `generated` ordinal is the record's position among
that owner's `value-validity` records in package order, so a function's
records take ordinals 0, 1, 2, … in the order of their rows. Its
`semantic_type` is the `scalar_type`/`boolean` node, and its `dependencies`
are the unique, ascending set of the targets of its body's references.

Its body is an aggregate of exactly these eight bindings, in this order:

1. `owner`: a `reference` to the `function` node whose body holds the claim
   site. It is a declared value function, or a function the producer derives
   from an operation `body` clause or a `precondition` clause (FR-322 "Owner
   joins"), which owns records but is not callable by name. A site in a
   function's termination measure is not in its body and has no record.
2. `application`: a `reference` to the scalar operation application's
   `expression` node. Its `operation.identity` is in one of the families
   `filament.op.integer`, `filament.op.rational`, `filament.op.decimal`,
   `filament.op.ieee`, `filament.op.quantity`, `filament.op.numeric`,
   `filament.op.text` and `filament.op.enum`, and is not
   `filament.op.numeric.narrow`.
3. `occurrence`: a `literal` integer term, the ordinal of the application's
   `expression` occurrence the record is keyed at. Two occurrences of one
   application node, in one function or in two, are two records with two
   nodes, each with its own bound, guards and roots.
4. `result_bound`: a `reference` to the type node of the record's result
   bound: the `type_argument` node of the narrow that directly wraps the
   application when there is one, and otherwise the type node of the
   application's checked type (for an integer `div`, `rem` or `mod`, the
   exact interval type node, which need not be the application's
   `result_type` node).
5. `conversion`: the empty aggregate when no narrow directly wraps the
   application at this occurrence, and otherwise an aggregate of exactly two
   bindings, `narrow` (a `reference` to the innermost
   `filament.op.numeric.narrow` application whose operand is the
   application) and `occurrence` (a `literal` integer, the ordinal of that
   narrow's `expression` occurrence that wraps this one). A narrow is never a
   record of its own.
6. `path_condition`: an aggregate with one member per guard the occurrence
   stands under, outermost first, each an aggregate of exactly three
   bindings: `guard` (a `reference` to the guard's Boolean `expression`
   node), `occurrence` (a `literal` integer, the ordinal of that guard's
   `expression` occurrence) and `holds` (a `literal` Boolean, the outcome the
   guard is required to have). A guard is an enclosing `if`'s condition,
   required true in its `then` branch and false in its `else` branch, or the
   left operand of an enclosing `and`, `or` or `implies` whose right operand
   holds the occurrence, required true for `and` and `implies` and false for
   `or` (FR-484 "premises"). An unguarded occurrence has the empty aggregate.
7. `extent_roots`: an aggregate of `reference`s to the `value`/`parameter`
   nodes of every function parameter and every query, `count`, `sum`,
   `fold` or `reduce` binder that the application's argument subtrees or a
   guard of its path condition read, in ascending node-id order, with no
   repeat. It holds every root, a root whose type adds no unbounded domain
   included, so a `bounded` record still names each parameter it reads. A
   `let` binder contributes the roots its bound value reads, a binder bound
   inside the application's own arguments is not a root, and a literal
   contributes none.
8. `objects`: an aggregate of `reference`s to the `model`/`object_type`
   node, of a selected domain package or source-declared, of the static
   object type of every model member the claim reads, in ascending node-id
   order, with no repeat; empty when the claim reads no model member.

A `value-validity` record ranges over no population of its own and is about
no operation, so these bindings are all of its references.

The node's `node_id` path is

`["claim", "value_validity", {owner}, {application}, K, "bound",
{result_bound}, "conversion", ({narrow}, M)?, "guards", ({guard}, J,
H)…, "roots", {r}…, "objects", {o}…]`

where `{x}` is the operand segment naming node `x`; `K`, `M` and `J` are the
decimal texts of the `occurrence` ordinals of the application, the narrow
and each guard; `H` is `"true"` or `"false"`; and the operands are in body
order. The conversion group is absent for the empty `conversion`. Each
variable-length group has a fixed arity and the binding-name strings
separate the groups, so two different bodies never give one path. The
`ordinal` is `0`: the owner, the application and its occurrence ordinal
identify one claim site, so no two records share a path.

The record's identity therefore covers what the claim reads. A change to the
owning function, to the application or its occurrence, to the bound's type,
to the wrapping narrow, to a guard or its required outcome, to the roots or
to the object types read changes the node's `node_id`, and a change to the
unit that changes none of these leaves it unchanged. A sidecar row cannot
carry these facts with identity, because a row never alters a node or its id
(FR-322 `checked_term_data`); the `requirement_items` row therefore names
the node and repeats none of its bindings.

A `claim`/`value_validity` node is not a claim-bearing node for
`claim_names` (FR-322 "Claim declaration names"): no source declaration
introduces it, it has no `declaration` or `claim` occurrence and no name, and
a request never names it. It is reached only through its `requirement_items`
row.

## Behavior

### Emission

- The producer SHALL write one `requirement_items` row for every requirement
  record its checker produced, of every kind, and none for anything else.
  It SHALL NOT omit a record because a consumer is not expected to read it,
  because the package order counts every record.
- The producer SHALL write one `claim`/`value_validity` node for every
  `value-validity` record, after the owner function, the application, the
  bound's type node, the narrow, every guard, every root and every object
  type node it references, with the body and path above, and SHALL name it
  from the record's row.
- The producer SHALL write a record's `extent` from its own classification
  of the record's extent (FSL ADR-014 §4), never from a backend's bounds.
- The producer SHALL NOT carry a record's facts in any other table or
  sidecar row.

### Reader

The reader SHALL run this step after FR-640's `probabilistic` and
`probabilistic_claim` checks and before the `checked_term_data` records
(FR-322 "Check order"), and refuse the first failure with the code and cause
given, at the JSON pointer named. First, for each `claim`/`value_validity`
node in ascending node-id order, at `/semantic_graph/nodes/{n}`, `n` the
node's graph index:

1. a node with a `declaration` member, or whose occurrences are not exactly
   one `generated` occurrence: `invalid_package`/`invalid-value` at
   `/semantic_graph/nodes/{n}/occurrences`;
2. a body that is not an aggregate of exactly the eight bindings in order:
   `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body`;
3. an `owner` that is not a `reference` to a local `function` node:
   `invalid_package`/`invalid-value` at `body/members/0`;
4. an `application` that is not a `reference` to a local `expression` node
   applying an operation of the scalar families above, or that is not
   reached from the owner's body root through body references (the
   termination measure is not searched): `invalid_package`/`invalid-value` at
   `body/members/1`;
5. an `occurrence` that is not a non-negative integer literal naming an
   `expression` occurrence of the application node in `source_map`:
   `invalid_package`/`invalid-value` at `body/members/2`;
6. a `result_bound` that is not a `reference` to a local or dependency type
   node (tag `scalar_type`, `composite_type` or `bounded_domain`):
   `invalid_package`/`invalid-value` at `body/members/3`;
7. a `conversion` that is neither the empty aggregate nor the two bindings
   above, whose `narrow` is not a `filament.op.numeric.narrow` application
   whose operand references the `application` node, whose `occurrence` names
   no `expression` occurrence of that narrow, or whose narrow's
   `type_argument` node is not the `result_bound` node:
   `invalid_package`/`invalid-value` at `body/members/4`;
8. a `path_condition` member that is not the three bindings above, whose
   `guard` is not a `reference` to a local `expression` node of Boolean
   `semantic_type` reached from the owner's body root, whose `occurrence`
   names no `expression` occurrence of that node, or whose `holds` is not a
   Boolean literal: `invalid_package`/`invalid-value` at
   `body/members/5/value/members/{g}`;
9. an `extent_roots` member that is not a `reference` to a `value`/`parameter`
   node, or that is repeated or out of ascending node-id order:
   `invalid_package`/`invalid-value` at `body/members/6/value/members/{r}`;
10. an `objects` member that is not a `reference` to a `model`/`object_type`
    node, or that is repeated or out of order: `invalid_package`/`invalid-value`
    at `body/members/7/value/members/{o}`.

Then, for `requirement_items`:

11. the member absent while the graph holds a `claim`/`value_validity` node:
    `invalid_package`/`invalid-value` at `/requirement_items`. A present
    empty array, a row with a missing or extra member, a `kind` or domain
    kind outside its vocabulary, an `unbounded` extent without `domains` or
    with an empty `domains`, and a `bounded` extent with `domains` refuse the
    closed schema, `malformed_wire` at the member;
12. for each row in array order, at `/requirement_items/{i}`: a `claim.node`
    that names no local node, or a `claim.occurrence` that is not an
    occurrence of that node in `source_map`: `invalid-value` at
    `/requirement_items/{i}/claim`; a `node` domain key whose `node`, or a
    `population` key whose `member_type`, names no node of the graph, or a
    domain repeated or out of domain-key order: `invalid-value` at
    `/requirement_items/{i}/extent/domains/{d}`; a `value-validity` row with
    no `record`: `invalid-value` at `/requirement_items/{i}`; a row of any
    other kind with a `record`: `invalid-value` at
    `/requirement_items/{i}/record`; a `record` that names no local
    `claim`/`value_validity` node, or one whose `application` is not
    `claim.node` or whose `occurrence` is not the ordinal of a `claim.occurrence`
    of role `expression`: `invalid-value` at `/requirement_items/{i}/record`;
    a `value-validity` row with a `population` domain key, or a `node` key
    whose `node` is not one of its record's `extent_roots`: `invalid-value` at
    `/requirement_items/{i}/extent/domains/{d}`; all `invalid_package`;
13. a row whose (`claim`, `kind`) equals an earlier row's:
    `invalid_package`/`conflicting-definition` at the later row; otherwise a
    row that is not after its predecessor in package order:
    `invalid_package`/`invalid-semantic-graph` at the later row;
14. after every row, a `claim`/`value_validity` node that no row's `record`
    names: `invalid_package`/`invalid-value` at that node's
    `/semantic_graph/nodes/{n}/body`; and, for each owner in ascending
    node-id order, a record node whose `generated` ordinal is not its
    position among that owner's rows: `invalid_package`/`invalid-value` at
    `/semantic_graph/nodes/{n}/occurrences`.

The reader does not compare a record node's path with its body (FR-322
"Structural node identity"), recomputes no extent, re-derives no owner or
guard, and does not decide whether the producer's checker should have
produced a record it did not write: each of those is the producer's duty,
held by its own criteria.

The reader SHALL charge one validation visit of the caller's reader budget per
`claim`/`value_validity` node and per `path_condition`, `extent_roots` and
`objects` member, per `requirement_items` row and per extent domain, before
inspecting it, and one visit per node it reaches while it walks an owner's
body, each owner's body walked once whatever the number of its records. On
exhaustion it SHALL return its `incomplete` result at the first uncharged
member, with no partial view.

### Typed read

An admitted package SHALL expose its requirement records in package order,
each with its claim node and occurrence, its kind, its extent and, for a
`value-validity` record, its record node, owner function node, application
node and occurrence ordinal, result-bound type node, conversion narrow and
occurrence, path condition (guard node, occurrence ordinal and required
outcome, outermost first), extent-root nodes and object-type nodes. A
record's request index is its position in that read. The read SHALL return,
for a function node, its `value-validity` records in package order without a
scan of the graph, and returns no record for a function that owns none.

### Consumers

- A run of a selected package (FR-775, FSL FR-1039) takes a called
  function's items from the `value-validity` rows whose record names that
  function as `owner`, each with its row's request index, and writes each
  item's FR-331 request item from its row and record node.
- A check arm that decides a `value-validity` item (FR-484) MAY read the
  item's record node in place of locating the occurrence by its source region
  and walking its guards. For a package that passes this reader, both give
  the same owner, guards, bound and conversion, because both describe one
  claim site.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-825-AC-1 | A unit holding `function g using v(p: Int[0, 10]): Boolean pure { true }`, `function h using v(q: Int[0, 20]): Boolean pure { true }` and `function f using v(x: Int[0, 9]): Boolean pure { g(x + 1) and h(x + 1) }` is emitted with exactly two `requirement_items` rows, both `value-validity` and `{"classification": "bounded"}`, keyed at the one `x + 1` application node's `expression` occurrences 0 and 1 in that order, and two `claim`/`value_validity` nodes each with one `generated` occurrence (ordinals 0 and 1) and the eight bindings: `owner` `f`'s node; `application` the `+` node; `occurrence` 0 and 1; `result_bound` the `Int[0, 10]` and `Int[0, 20]` type nodes; `conversion` the narrow to `Int[0, 10]` at its occurrence and the narrow to `Int[0, 20]` at its occurrence; `path_condition` empty for the first, and for the second one member whose `guard` is the `g(x + 1)` call node at its occurrence with `holds` true; `extent_roots` `x`'s parameter node for both; `objects` empty. Each node's path is the path of its body, `["claim", "value_validity", {f}, {+}, "0", "bound", {Int[0, 10]}, "conversion", {narrow}, "0", "guards", "roots", {x}, "objects"]` for the first, and the reader admits the package; `g` and `h` own no record. | Test (TC-2053) |
| FR-825-AC-2 | A unit holding `function g using v(p: Int[0, 10]): Boolean pure { true }` and `function f using v(n: Integer): Boolean pure { if n >= 0 and n < 10 then g(n + 1) else true }` is emitted with three `value-validity` rows (the `>=`, the `<` and the `+`), each `{"classification": "unbounded", "domains": [{"domain": {"type": "node", "node": <n's parameter node>, "path": []}, "kind": "integer"}]}`, in package order, which the test first establishes by comparing the three application nodes' structural keys; the `<` record's `path_condition` is one member (`n >= 0`, `holds` true), the `+` record's one member (the `if` condition `n >= 0 and n < 10`, `holds` true), and the `>=` record's empty; their record nodes have `generated` ordinals equal to their positions among `f`'s rows. Rewriting `n < 10` as `n < 11` changes the `<` record node's `node_id` and the `+` record node's (its guard changed), and not the `>=` record node's. | Test (TC-2053) |
| FR-825-AC-3 | FSL FR-620's Counter unit (claims `Bounded` and `BelowThree`), extended with `function inc using v(x: Int[0, 9]): Int[0, 10] pure { x + 1 }`, is emitted with one row per requirement record its checker produced, including `Bounded`'s `operation-contract` row at its `state_clause` node's `claim` occurrence and `BelowThree`'s `temporal-satisfaction` row at its clause node's `claim` occurrence, each without `record`, and `inc`'s one `value-validity` row with `record`; the rows are in package order, which the test establishes from the rows' structural keys, occurrences and kinds; the reader admits the package; and the typed read returns, for `inc`'s node, exactly its one record at the request index of its row. A package of a unit with no requirement record has no `requirement_items` member and is admitted. | Test (TC-2053) |
| FR-825-AC-4 | Over FR-825-AC-1's package, each mutation refuses with the code, cause and pointer the reader rule gives: a record node with a `declaration` member, and with a second `generated` occurrence (rule 1, at `occurrences`); `conversion` and `result_bound` swapped (rule 2, at `body`); `owner` referencing the `+` node (rule 3); `owner` referencing `g`'s node, which does not reach `+` (rule 4, at `body/members/1`); `application` referencing the narrow node (rule 4); `occurrence` 2 (rule 5); `result_bound` referencing `x`'s parameter node (rule 6); the first record's `conversion` naming the `Int[0, 20]` narrow (rule 7, at `body/members/4`); the second record's guard with `holds` the text `"true"` (rule 8, at `body/members/5/value/members/0`); `extent_roots` holding `x` twice (rule 9, at `body/members/6/value/members/1`); `objects` referencing the Boolean type node (rule 10). Each is `invalid_package`/`invalid-value`. | Test (TC-2053) |
| FR-825-AC-5 | Over FR-825-AC-2's and FR-825-AC-3's packages, each mutation refuses with the code, cause and pointer the reader rule gives: `requirement_items` removed from AC-1's package (`invalid-value` at `/requirement_items`); `requirement_items` `[]`, a row without `extent`, and a row with `kind` `value-validty` (`malformed_wire`); a row whose `claim.occurrence` ordinal is 7 (at `/requirement_items/{i}/claim`); the `>=` row's one domain written twice (at `/requirement_items/{i}/extent/domains/1`); `inc`'s row without `record` (at `/requirement_items/{i}`); `Bounded`'s row with `inc`'s `record` (at its `record`); the `<` row's `record` naming the `+` record node (at its `record`); a `value-validity` row whose domain names `g`'s parameter `p` (at `extent/domains/0`); the `<` row repeated after itself (`conflicting-definition` at the later row); the first two rows of AC-2's package swapped (`invalid-semantic-graph` at `/requirement_items/1`); the `+` row removed (`invalid-value` at the `+` record node's `body`); and the `>=` and `<` record nodes' `generated` ordinals swapped (`invalid-value` at the first such node's `occurrences` in owner, then node-id, order). Over AC-1's package a reader budget one visit short of the package's count returns `incomplete` at the first uncharged member, and a package whose `claim_names` is absent and whose only `claim` nodes are `value_validity` nodes is admitted. | Test (TC-2053) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the graph, node identity,
  `source_map`, the check order and the reader budget.
- [FR-290](../protocol/FR-290-protocol-claim-kind.md) owns the capability
  kinds and the `value-validity` claim form; [FR-331](FR-331-backend-provider-envelope.md)
  owns the request item's `claim`, `kind` and `extent` spellings that the rows
  reuse.
- [FR-484](../../functional/ir-proof/FR-484-check-value-validity-items.md)
  decides a `value-validity` item; [FR-775](../../functional/tooling/FR-775-settle-the-requirement-items-of-an-execute-call.md)
  settles the items of a called function.
- [FR-640](FR-640-carry-probabilistic-model-declarations-in-the-checked-package.md)
  is the pattern for a derived claim node that carries what it reads.
