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

### Shared path-condition frames

The package MAY carry the top-level `path_condition_frames` member. It SHALL
be absent exactly when every `value-validity` record has an empty path
condition; otherwise it SHALL be a non-empty array. Its array index is the
frame's package-local `frame_id`, a zero-based unsigned integer. The array is
in canonical order: owner function structural key ascending, then first visit
in that owner's body-reference order, depth first. A frame is emitted before
any child that names it. Exactly one row exists for each distinct
`(parent, guard occurrence, holds)` tuple; duplicate tuples are refused.

Each row is a closed object with exactly these members, in this order:

```json
{"parent": null, "guard": {"node": {"package": "<package>", "path": ["<segment>"], "ordinal": 0}, "occurrence": {"role": "expression", "ordinal": 0}}, "holds": true}
```

- `parent` is `null` for an outermost frame, or the `frame_id` of its
  immediately enclosing guard frame. It is a non-negative integer smaller
  than this row's array index.
- `guard` is the exact expression occurrence read as a Boolean guard: a
  local `NodeRef` and an `expression` source-map occurrence ordinal, in the
  closed occurrence object `{node, occurrence}`. The `occurrence` object is
  exactly `{role, ordinal}` with role `expression`.
- `holds` is the required Boolean outcome for this branch: `true` for an
  `if` then branch and for the right operand of `and` or `implies`; `false`
  for an `if` else branch and the right operand of `or`.

The tuple's `parent` links the row to the complete outer prefix. Frames with
the same tuple are shared by every record using that nested guard path. The
table is not a graph node and frame IDs are not `NodeRef`s; they are indexes
only in this package. Every table row SHALL occur in at least one record's
referenced chain. The reader SHALL refuse a frame whose guard is not a Boolean
expression/value condition in its owner's body, whose parent is not the
immediately enclosing guard, or whose `holds` differs from the branch
semantics above.

The table's only semantic authority is the checked package body: the reader
derives the ordered guard chain for every scalar-operation occurrence by
walking each owner body from its root, excluding the termination measure.
It SHALL compare each record's referenced frame chain with that independently
derived chain, including guard occurrence, order and `holds`; it SHALL NOT
trust a producer-supplied chain as proof of body context.

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
4. `result_bound`: a `reference` to the local type node, or a
   `dependency_reference` to the dependency's type node, of the record's
   result bound: the `type_argument` node of the narrow that directly wraps the
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
6. `path_condition`: an aggregate that is empty when the occurrence has no
   guards, and otherwise has exactly one member: a non-negative integer
   literal, the `frame_id` of the terminal row in `path_condition_frames`.
   The parent links of that row and its ancestors encode the complete ordered
   condition. The reader derives the condition independently from the owner
   body's structure and requires the referenced chain to match it. The frame
   row's `guard` is the exact Boolean expression occurrence, including its
   source-map ordinal; its `holds` gives the required branch outcome. Thus the
   frame chain preserves the guard semantics of FR-484 without repeating each
   prefix in every record.
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
{result_bound}, "conversion", ({narrow}, M)?, "path_condition", (F)?,
"roots", {r}…, "objects", {o}…]`

where `{x}` is the operand segment naming node `x`; `K` and `M` are the
decimal texts of the application and narrow occurrence ordinals; `F`, when
present, is the decimal text of the terminal frame's package-local `frame_id`.
The conversion group is absent for the empty `conversion`, and the
`path_condition` group is absent for the empty path condition. The identity
path cites only the terminal frame id; it never expands the parent chain.
Each variable-length group has a fixed arity and the binding-name strings
separate the groups, so two different bodies never give one path. The
`ordinal` is `0`: the owner, application and occurrence identify one claim
site, so no two records share a path.

The record's identity therefore covers the claim site and its terminal frame
reference. A change to the owning function, application or occurrence, bound,
narrow, terminal frame id, roots or object types changes the node's `node_id`.
The frame rows are package contents, so changing a guard, parent or required
outcome changes the package's structural JSON identity even if a frame id is
unchanged. A `requirement_items` row never alters a node id and names the
record node without repeating its bindings.

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
  bound's type node, the narrow, every frame guard, every root and every object
  type node it references, with the body and identity path above, and SHALL name it
  from the record's row.
- The producer SHALL write `path_condition_frames` exactly when at least one
  value-validity record has a non-empty path condition. It SHALL intern each
  distinct `(parent, guard occurrence, holds)` tuple once, assign frame ids by
  the canonical order above, and make each guarded record cite only its
  terminal frame id in its body and identity path. It SHALL NOT serialize an
  expanded guard list in a record.
- The producer SHALL write a record's `extent` from its own classification
  of the record's extent (FSL ADR-014 §4), never from a backend's bounds.
- The producer SHALL NOT duplicate a record's facts in any table or sidecar
  row. `path_condition_frames` is the sole shared representation of guards.

### Reader

The reader SHALL run this step after FR-640's `probabilistic` and
`probabilistic_claim` checks and before the `checked_term_data` records
(FR-322 "Check order"), and refuse the first failure with the code and cause
given, at the JSON pointer named. It SHALL first validate `path_condition_frames`:

- a present member that is not a non-empty array of closed rows whose members
   are exactly `parent`, `guard`, `holds` in that order, or a row whose
   `guard` is not exactly `{node, occurrence}` with an `expression` occurrence,
   or whose primitive JSON types are wrong: `malformed_wire` at
   `/path_condition_frames` or the malformed row;
- a `parent` that is neither `null` nor an earlier array index, a missing
   parent row, or a parent from a different owner body: `invalid_package`/
   `invalid-value` at `/path_condition_frames/{f}/parent`. Requiring every
   parent index to be less than `f` excludes cycles;
- a guard node that is not a local Boolean `expression`, `value`/`literal`,
   or `value`/`parameter` in the owner's reachable body, or whose source-map
   expression occurrence does not identify that read: `invalid_package`/
   `invalid-value` at `/path_condition_frames/{f}/guard`;
- a row that repeats an earlier `(parent, guard, occurrence, holds)` tuple:
   `invalid_package`/`conflicting-definition` at the repeated row;
- a frame row not used by any record's referenced chain:
   `invalid_package`/`invalid-value` at `/path_condition_frames/{f}`.

The reader SHALL then walk each owner body once, from its body root and in
body-reference order, excluding its termination measure. It SHALL derive for
each scalar-operation occurrence the exact outermost-first chain of condition
occurrences and required outcomes under which it is evaluated. In this walk,
`if` then/else branches contribute their condition with `holds` true/false;
the right operand of `and`, `implies` and `or` contributes the left operand
with `holds` true/true/false respectively. Shared prefixes use the same
canonical frame id. This traversal both validates each frame's parent and
branch polarity and produces the expected terminal frame id for each
occurrence. A record whose reference differs from that derived id refuses as
`invalid_package`/`invalid-value` at its `body/members/5` binding. For an
unguarded occurrence, only the empty aggregate is valid. A non-empty frame
table when all record chains are empty, or a missing table when any derived
chain is non-empty, refuses `invalid_package`/`invalid-value` at
`/path_condition_frames`.

Next, for each `claim`/`value_validity` node in ascending node-id order, at
`/semantic_graph/nodes/{n}`, `n` the node's graph index:

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
6. a `result_bound` that is neither a `reference` to a local type node nor
   a `dependency_reference` resolving to a type node of a direct dependency
   (tag `scalar_type`, `composite_type` or `bounded_domain` in either case):
   `invalid_package`/`invalid-value` at `body/members/3`;
7. a `conversion` that is neither the empty aggregate nor the two bindings
   above, whose `narrow` is not a `filament.op.numeric.narrow` application
   whose operand references the `application` node, whose `occurrence` names
   no `expression` occurrence of that narrow, or whose narrow's
   `type_argument` node is not the `result_bound` node:
   `invalid_package`/`invalid-value` at `body/members/4`;
8. a `path_condition` member that is not an empty aggregate or an aggregate
   of exactly one non-negative integer literal naming a row in
   `path_condition_frames`, or whose frame chain differs from the independently
   derived chain for this owner's application occurrence:
   `invalid_package`/`invalid-value` at `body/members/5`;
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

The reader SHALL verify that a record node's structural identity path contains
the same terminal frame id as its body binding. A mismatch refuses
`invalid_package`/`invalid-semantic-graph` at the node's `node_id`. The reader
does not recompute an extent, re-derive an owner, or decide whether the
producer's checker should have produced a record it did not write; those
remain the producer's duty.

The reader SHALL charge one validation visit of the caller's reader budget per
`path_condition_frames` row, `claim`/`value_validity` node,
`extent_roots` and `objects` member, `requirement_items` row, and extent
domain, before inspecting it, and one visit per node it reaches while walking
an owner's body. Each owner body SHALL be walked once for all of its records.
It SHALL retain at most one path frame per distinct guard edge and one record
index per value-validity record; it SHALL NOT retain an expanded path-condition
vector per record. Retained storage for the requirement records, shared frames
and typed views SHALL be O(G + R), where G is the number of unique path frames
and R the number of records. The owner's body traversal may use O(D)
temporary stack storage, D the maximum body depth, and SHALL release it after
admission. On exhaustion the reader SHALL return its `incomplete` result at
the first uncharged member, with no partial view.

### Typed read

An admitted package SHALL expose its requirement records in package order,
each with its claim node and occurrence, kind and extent and, for a
`value-validity` record, its record node, owner function node, application
node and occurrence ordinal, result-bound type node, conversion narrow and
occurrence, and a borrowed shared-chain view of the path condition (outermost
first), plus extent-root nodes and object-type nodes. The view SHALL follow
parent links without allocating or materializing a per-record vector. A
record's request index is its position in that read. The read SHALL return,
for a function node, its `value-validity` records in package order without a
scan of the graph, and returns no record for a function that owns none.

### Consumers

- A run of a selected package (FR-775, FSL FR-1039) takes a called
  function's items from the `value-validity` rows whose record names that
  function as `owner`, each with its row's request index, and writes each
  item's FR-331 request item from its row and record node.
- A check arm that decides a `value-validity` item (FR-484) MAY read the
  item's owner, bound, conversion and shared path-condition view from the
  admitted record. The reader has already derived the occurrence's guard
  chain from the owner body and verified every frame and required outcome,
  including `holds` false for the right operand of `or`; the arm SHALL NOT
  replace, omit or expand that validated chain into retained per-record
  vectors.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-825-AC-1 | A unit holding `function g using v(p: Int[0, 10]): Boolean pure { true }`, `function h using v(q: Int[0, 20]): Boolean pure { true }` and `function f using v(x: Int[0, 9]): Boolean pure { g(x + 1) and h(x + 1) }` emits two ordered value-validity rows and two records for the shared `+` node's expression occurrences 0 and 1. The first record has the empty path-condition aggregate; the second cites the frame for the `g(x + 1)` call occurrence with holds true. The frame row has parent null, the exact guard occurrence and holds true. The second record's body and node_id path both contain only that terminal frame_id, not the expanded guard data. Both records retain their own result bounds, conversions, roots and generated ordinals, and the reader admits the package. | Test |
| FR-825-AC-2 | A unit holding `function g using v(p: Int[0, 10]): Boolean pure { true }` and `function f using v(n: Integer): Boolean pure { if n >= 0 and n < 10 then g(n + 1) else true }` emits three value-validity rows (the `>=`, `<` and `+`), with `>=` unguarded and the other two citing the terminal ids of their exact chains. The `<` chain contains `n >= 0`; the `+` chain contains the whole `if` condition. Every shared prefix appears once in the frame table; each `+` and `<` record body and identity path cite only their terminal frame id. Rewriting `n < 10` as `n < 11` changes the `<` frame and record identity and the `+` frame chain, but not the `>=` record. | Test |
| FR-825-AC-3 | FSL FR-620's Counter unit (claims `Bounded` and `BelowThree`), extended with `function inc using v(x: Int[0, 9]): Int[0, 10] pure { x + 1 }`, is emitted with one row per requirement record its checker produced, including `Bounded`'s `operation-contract` row at its `state_clause` node's `claim` occurrence and `BelowThree`'s `temporal-satisfaction` row at its clause node's `claim` occurrence, each without `record`, and `inc`'s one `value-validity` row with `record`; the rows are in package order, which the test establishes from the rows' structural keys, occurrences and kinds; the reader admits the package; and the typed read returns, for `inc`'s node, exactly its one record at the request index of its row. A package of a unit with no requirement record has no `requirement_items` member and is admitted. | Test (TC-2053) |
| FR-825-AC-4 | Over FR-825-AC-1's package, each mutation refuses with the code, cause and pointer the reader rule gives: a record node with a `declaration` member, and with a second `generated` occurrence; `conversion` and `result_bound` swapped; invalid `owner`, `application`, `occurrence` or `result_bound`; the first record's `conversion` naming the other narrow; a frame row whose `holds` is the text `"true"` (`malformed_wire` at `/path_condition_frames/{f}`); `extent_roots` holding `x` twice; and `objects` referencing the Boolean type node. Record-shape/reference defects are `invalid_package`/`invalid-value` at the existing rule's record pointer; the malformed frame is `malformed_wire`. | Test (TC-2053) |
| FR-825-AC-5 | Over AC-2 and AC-3 packages, malformed or reordered `requirement_items` rows refuse with the existing rule's exact codes and pointers; a reader budget one validation visit short returns `incomplete` at the first uncharged item and no partial view. The reader charges the frame rows, each owner's body walk once, each record node, roots, objects, rows and domains; it does not charge or retain a repeated guard vector per record. A package whose only `claim` nodes are `value_validity` nodes and which has no `claim_names` member is admitted. | Test (TC-2053) |
| FR-825-AC-6 | A unit with `function f using v(p: Boolean, x: Int[0, 9]): Int[0, 10] pure { if p then x + 1 else x + 1 }` has two records for the shared `+` node's occurrences. One frame for `p` with parent null and holds true and one frame for the same occurrence with parent null and holds false are present. The records cite their respective frame ids; each record body and node_id path contains only its terminal id. The reader admits both and the roots include `p` and `x` once in node order. | Test (TC-2053) |
| FR-825-AC-7 | Starting from AC-6, replace a frame's guard occurrence with the owner's integer parameter `x`, retaining a valid expression occurrence. The reader refuses `invalid_package`/`invalid-value` at `/path_condition_frames/{f}/guard` because the guard is not Boolean. | Test (TC-2053) |
| FR-825-AC-8 | For a package with nested guards and at least two scalar records sharing a prefix, each malformed frame mutation refuses at the stated first defect: a non-array, empty array, row with missing/extra/reordered members, or non-Boolean JSON `holds` is `malformed_wire` at the table/row; a `parent` that is forward, self, missing, or from another owner is `invalid_package`/`invalid-value` at `/path_condition_frames/{f}/parent`; an unknown guard node or occurrence is `invalid_package`/`invalid-value` at `/path_condition_frames/{f}/guard`; a duplicate `(parent, guard occurrence, holds)` row is `invalid_package`/`conflicting-definition` at the duplicate row. | Test (TC-2053) |
| FR-825-AC-9 | For the valid nested-guard package of AC-8, a frame chain with the wrong parent, guard order or `holds`, a missing table, a terminal frame id outside the table, a record id whose chain differs from the owner-body-derived chain, or an unreferenced frame row refuses `invalid_package`/`invalid-value` at the corresponding parent, `/path_condition_frames`, record `body/members/5`, or table row; changing only a record node's identity path to cite a different terminal id refuses `invalid_package`/`invalid-semantic-graph` at its `node_id`. | Test (TC-2053) |

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
