---
id: FR-789
title: "Join the clock places of a source timed model"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-610
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-788
    type: depends_on
---
# FR-789: Join the clock places of a source timed model

## Description

When a checked-package reader validates a `timed_model` root whose `model`
is a source-declared `model`/`object_type` node (a **source timed model**),
it SHALL join every `resolved_path` row of the root's `clock_place` nodes to
source declarations in two stages. The **strict read** of the package SHALL
check every fact of a row that the package alone shows and refuse a row that
fails one `invalid_package`/`malformed-clock-place` at the row. The
**identity-row join**, which the reader runs after the strict read over the
producer's identity rows (FR-610 "Identity rows"), SHALL check every fact of
a row that only a declaration's member list shows, and refuse a row that
fails one `invalid_package`/`malformed-clock-place` at the row. A source
timed model's places are joined, and a consumer SHALL use them, only when
both stages admit them.

## Rationale

FR-322-AC-124 joins every row to the declaration of its path step so that a
forged path refuses without reopening source. A source timed model has no
selected document. Its data fields are bindings of the declared nodes, so
the package shows them; its clock members are not, because no node stands
for a clock (FR-322 "Member type", the `Clock` row) and an object type body
or a `record_value_type` body lists data fields only. A clock member's
existence, presence, type and bounds therefore exist only in the producer's
identity rows, which a reader receives beside the package and joins after it
has read the package. Checking each fact at the first stage that holds it
keeps the strict read independent of any side input and still refuses every
forged row: a row that the package contradicts refuses at the read, and a
row that only the identity rows contradict refuses at the join. A source
root never resolves a row through a selected document, which keeps a source
model's clocks apart from a selected model's.

## Behavior

### Rendering

A row's `declaring_type` and `member_type` are type renderings, and a
member identity is a rendering followed by a member segment:

- A source-declared object type, native record, native tuple or native union
  `T` of the `SourceOwner` with `identity` `I` renders as the `node` of its
  source declaration key, `ix://I/T` (FR-610 "Declaration keys", FR-788).
- A native scalar renders as `ix://filament/native/<Name>` (`Clock`,
  `Integer`, `Text`, ...). An alias renders as its target.
- A member whose presence is `optional` renders as `Option<R>` around the
  rendering `R` of its type, and an optional collection element renders as
  `Option<R>` inside the collection: `Sequence<Option<ix://filament/native/Clock>>`.
  The wrapper adds no path step, and the terminal row is the one whose type
  is `ix://filament/native/Clock` once one `Option` wrapper is removed.
- A collection renders as `Sequence<R>`, `Set<R>`, `Bag<R>` or
  `OrderedSet<R>` by its kind, and a map as `Map<K, R>` with `K` the bare name
  of its scalar key type (`Text`), as a selected map renders. A rendering never
  carries bounds: a collection's kind and bounds are the row's
  `collection_kind`, `minimum` and `maximum`.
- The payload of a union case with one payload position renders as that
  position's type; a payload of several positions renders as the product
  `(R0, ..., Rn-1)` of its positions' renderings.

A `field(name)` row's member identity is `<declaring_type>/<name>`, a
`tuple(i)` row's `<declaring_type>/p<i>`, a `union(case)` row's
`<declaring_type>/<case>` and a `slot` row's `<C>[slot]` for `C` the previous row's `member_type`, the collection, whose element rendering is the slot row's `declaring_type`, as for a selected place.

### What the strict read checks

From the package alone, for each `clock_place` of a source timed model with
root `SourceOwner` `identity` `I`, the strict read SHALL refuse
`invalid_package`/`malformed-clock-place` at
`/semantic_graph/nodes/{i}/body/members/2/value/members/{j}` a row `j` that
fails one of these, and SHALL keep FR-322's path-step refusals at
`/body/members/1/value/members/{j}`:

1. Row count, `path_step` index, the closed row members and the rule that a
   path does not start with a `slot` step, as for a selected place.
2. Row `0`'s `declaring_type` is the rendering of the root's `model` node,
   `ix://I/<root type>`; row `j > 0`'s is row `j - 1`'s `member_type`, or for
   a `slot` row the element rendering of that collection, as for a selected
   place.
3. A `field`, `tuple` or `union` row's `declaring_type` is `ix://I/<T>` for a
   source-declared object type, native record, tuple or union node `T` of a
   `SourceOwner` with `identity` `I`, or, for a `tuple` row after a `union`
   row, that row's product rendering. Any other spelling, including another
   owner's identity and a selected model's IR node identity, refuses.
4. The row's member identity is the spelling the Rendering section gives
   for its step.
5. A `field(name)` row whose declaring node's body has a binding `name` (a
   data field) has `member_type` the rendering of the binding's type node,
   wrapped in `Option` exactly when the binding carries the `optional` group,
   and, when the next step is a `slot`, the `collection_kind` and bounds of
   that type node's collection and `collection_bounds` nodes.
6. A `field(name)` row whose declaring node's body has no binding `name`, and
   every `tuple` and `union` row, has a `member_type` that is a well-formed
   rendering; its member is joined at the identity-row join.
7. A `slot` row's selector, kind and bounds against the next step, and the
   terminal `Clock` row, as for a selected place.

### What the identity-row join checks

The identity-row join SHALL run after the strict read admits the package
and after the identity rows themselves are admitted (FR-610 "Identity
rows"), and SHALL visit the `clock_place` nodes in ascending node-id order
and their rows in path order. For each row the strict read left to it, it
SHALL find the member entry the row names, in the identity row of the type
whose source declaration key `node` is the row's `declaring_type`, and SHALL
refuse `invalid_package`/`malformed-clock-place` at the row, the first such
row reported, when:

1. A `field(name)` row with no binding: the declaring type's identity row has
   no member entry `name`, or that entry is a data entry (it carries a type
   node), or the row's `member_type` is not the entry's `member_type`, or a
   following `slot` row's `collection_kind`, `minimum` or `maximum` is not the
   entry's collection layer at that depth.
2. A `tuple(i)` row: the tuple's identity row has no position `i`, or the
   row's `member_type` is not that position's rendering (the rendering of its
   type node for a data position, its `member_type` for a clock position), or
   the bounds of a following `slot` row are not the position's.
3. A `union(case)` row: the union's identity row has no member `case`, the
   member has no payload, or the row's `member_type` is not the rendering of
   its payload; a `tuple(i)` row after it joins the payload's position `i` by
   rule 2.

A `slot` row and the terminal `Clock` row hold nothing the identity rows add
and are complete at the strict read. A data field's row is complete at the
strict read; the identity-row join adds nothing to it.

### Clock parameters and reset frames

For a `clock_constraint` or `clock_reset` of kind `parameter` under a source
timed model, the strict read SHALL check that `operation` is an operation
anchor of the root's model and `parameter` a `text` literal, and the
identity-row join SHALL check that `parameter` names a parameter whose
declared type is `Clock` of the operation the anchor names: the
`clock_parameters` of that operation's entry in the identity row of the
root type, or, for an operation the root type only inherits, of the nearest
supertype in `supertypes` order whose row lists it. A name that is no
`Clock` parameter refuses `invalid_package`/`timed-node-reference` at
`/semantic_graph/nodes/{i}/body/members/2`, as for a selected model. For a
`parameter` reset, the strict read SHALL refuse
`invalid_package`/`timed-node-reference` at the reset's
`/semantic_graph/nodes/{i}/body/members/1/value/target` when the anchor's
frame names no field of the root's model that has no binding in the root
node's body, because a frame that names only data fields writes no clock;
the identity-row join SHALL refuse it at the same locus when none of those
fields is a clock member of the root's identity row whose `member_type` is
`ix://filament/native/Clock`.

### Order and precedence

The strict read's checks keep FR-322's order: the aggregate order check
first, then the row join of each place in ascending node-id order, then the
repeated-path determination. The identity-row join runs only over a package
the strict read admitted, so a row that both stages would refuse is refused
by the strict read.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-789-AC-1 | A source timed model with `SourceOwner` `identity` `I`, `record W { c: Clock; }`, `object O { w: W; d: Clock; }` and `time O dense unit ms;`, whose root's `model` is `O`'s node, `O`'s identity row listing the data entry `w` (`W`'s node) then the clock entry `d` (`member_type` `ix://filament/native/Clock`) and `W`'s identity row listing the clock entry `c`, is admitted by the strict read and joined by the identity-row join with two places: `d`, row `{ix://I/O, ix://I/O/d, ix://filament/native/Clock}`, and `w.c`, rows `{ix://I/O, ix://I/O/w, ix://I/W}` then `{ix://I/W, ix://I/W/c, ix://filament/native/Clock}`. | Test (TC-1917) |
| FR-789-AC-2 | Over the AC-1 package, the strict read alone, with no identity row supplied, refuses each `invalid_package`/`malformed-clock-place` at `/semantic_graph/nodes/{i}/body/members/2/value/members/{j}` of the edited row: `d`'s `member_identity` `ix://I/O/e`; `d`'s `declaring_type` `ix://J/O` for an identity `J` of no `SourceOwner` of the root; `d`'s `member_type` `Integer`, which is no `Clock` terminal; `w`'s `member_type` `ix://I/V`, which is not the rendering of `w`'s binding; and `w.c`'s second row's `declaring_type` `ix://acme/orders/Order`, a selected model's type. | Test (TC-1918) |
| FR-789-AC-3 | Over the AC-1 package, each edit is admitted by the strict read and refused by the identity-row join `invalid_package`/`malformed-clock-place` at row `0` of the edited place: a third place with the path `field(e)` and the row `{ix://I/O, ix://I/O/e, ix://filament/native/Clock}`, `O` declaring no `e`; `d`'s `member_type` `Option<ix://filament/native/Clock>` against the required entry; the clock entry `d` removed from `O`'s identity row; and, with `O` declaring `q: Sequence<Clock>[0, 3]` and its place `field(q)`, `slot`, `q`'s row `maximum` `4` with the next step's `maximum` `4`. With `c` removed from `W`'s identity row, `w.c` refuses at row `1`. | Test (TC-2023) |
| FR-789-AC-4 | A source timed model with `tuple P (Clock, Integer);`, `union U { armed(Clock), idle }` and `object O { p: P; u: U; }` is joined with the places `p` then `tuple(0)`, rows `{ix://I/O, ix://I/O/p, ix://I/P}` then `{ix://I/P, ix://I/P/p0, ix://filament/native/Clock}`, and `u` then `union(armed)`, rows `{ix://I/O, ix://I/O/u, ix://I/U}` then `{ix://I/U, ix://I/U/armed, ix://filament/native/Clock}`. A place `p` then `tuple(1)` whose second row has `member_type` `ix://filament/native/Clock`, a place `u` then `union(idle)`, and a place `p` then `tuple(2)` each pass the strict read and refuse at the identity-row join `invalid_package`/`malformed-clock-place` at row `1`. | Test (TC-2024) |
| FR-789-AC-5 | A source timed model with `object O { d: Clock; n: Integer; operation arm(x: Clock, k: Integer) modifies self.d; operation touch(x: Clock) modifies self.n; }`, `O`'s identity row listing `arm` with `clock_parameters` `["x"]` and `touch` with `["x"]`, is joined with a `parameter` constraint and a `parameter` reset of `arm`'s `x`. The strict read refuses `invalid_package`/`timed-node-reference` at the reset's `/body/members/1/value/target` a `parameter` reset of `touch`, whose frame names only the data field `n`. The identity-row join refuses `invalid_package`/`timed-node-reference` at `/body/members/2` a `parameter` constraint of `arm` naming `k`, and with `arm`'s `clock_parameters` empty, the `x` constraint. | Test (TC-2025) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) "Timed-model nodes",
  "Member type" and "Source-declared object types" item 8.
- [FR-610](../../functional/type-model/FR-610-declare-object-types-in-native-source.md)
  "Declaration keys" and "Identity rows".
- [FR-788](FR-788-join-a-source-dispatch-selection-key.md): the source key
  rendering.
- [FR-415](../../functional/temporal/FR-415-declare-clocks-and-timed-models.md):
  clock places, clock parameters and resets.
