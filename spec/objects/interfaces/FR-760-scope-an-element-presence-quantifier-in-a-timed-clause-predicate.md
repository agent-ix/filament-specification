---
id: FR-760
title: "Scope an element-presence quantifier in a timed clause predicate"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
---

# FR-760: Scope an element-presence quantifier in a timed clause predicate

## Description

A timed clause predicate (a `clock_guard` part's `predicate`, a
`time_invariant`'s `predicate`, an `urgency` of kind `when`) MAY hold a
`filament.op.collection.forall` or `filament.op.collection.exists` application
over a clock collection place whose body reads only element presences
(`presence_read`) and data. Such a quantifier SHALL bind its own slot
`parameter`, typed by the selector's domain (FR-322 "Presence"), and that
parameter is scoped by the application alone. It SHALL NOT be a member of the
part's or invariant's `binders`, which name only parameters that a clock
constraint of that part, or a bound of that invariant, has as a slot term
(FR-322 "Quantified guards and invariants", FR-322-AC-295).

The two scopes have different meanings. A part's `binders` quantify the whole
part, its constraints included, over the live slots, so a constraint with no
slot term under such a binder would be vacuous on an empty collection. A
presence quantifier in the predicate is a Boolean value (FR-415-AC-20: `forall`
true on no live slot, `exists` false), and the clock constraints beside it
apply whatever the collection holds. Keeping the quantifier in the predicate
preserves the source meaning of `self.x >= 1 ms and forall(e in self.q: present(e))`.

The quantifier ranges over the slot domain of the collection, the live
slots written by their selectors. Its first operand is the
`filament.op.option.slots` application that FR-772 defines, together with
that node's identity and the reader rule that admits it.

**A presence quantifier beside a constraint that has a slot term.** A part may
hold both a constraint whose slot term makes its `binders` quantify the part
(`x[i] >= 1` over live slots `i`) and, in its predicate, a presence
quantifier over the same or another collection. The two scopes stay
separate: the part's `binders` hold exactly the slot parameters of its
constraints, the quantifier binds its own parameter inside the predicate,
and neither is a member of the other's scope, even when both are written with
the same source name (they are two `parameter` nodes). The predicate reads no
`binders` parameter of the part it belongs to as a presence-quantifier
operand except through the quantifier's own parameter, and the quantifier's
parameter is unreadable from a constraint. The same holds in a time
invariant's predicate with its bounds, and in an `urgency` of kind `when`,
which has no `binders` at all, so a presence quantifier there is scoped by
its application alone. A `forall` presence over an empty collection is true
and an `exists` false, while the part's `binders` still quantify its
constraints over the live slots.

A part or invariant none of whose constraints or bounds has a slot term
carries no `binders` binding. An `urgency` has no `binders` binding in any
case.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-760-AC-1 | A package whose `clock_guard` part has, as its predicate, a `filament.op.collection.forall` application over the element places of `q: Sequence<Option<Clock>>[0, 2]` binding the parameter `e` (typed by `q`'s selector domain) with body the `presence_read` of `q`'s element place at `self` and `e`, one constraint `x ge 1/1` with no slot term, and no `binders`, is admitted; so are a `time_invariant` with the same predicate and bound `x le 9/1` and no `binders`, and an `urgency` of kind `when` with that predicate. | Test (TC-1830) |
| FR-760-AC-2 | The AC-1 package with a `binders` binding naming `e` added to the guard part, or to the invariant, refuses `invalid_package`/`timed-node-reference` at the binder target, because no constraint or bound has `e` as a slot term. | Test (TC-1830) |
| FR-760-AC-3 | The AC-1 package whose `forall` application is replaced by the `exists` application over the same domain and body is admitted with one guard part, not one part for each slot. | Test (TC-1830) |
| FR-760-AC-4 | A `clock_guard` part with the constraint `q[i] ge 1/1` over `q: Sequence<Option<Clock>>[0, 2]` (so `binders` names `i`) and, in its predicate, a `forall` presence over `q` binding its own parameter `e` is admitted with `binders` holding `i` only; the same part whose `binders` also names `e`, or whose quantifier reuses the part's parameter node of `i` instead of its own, refuses `invalid_package`/`timed-node-reference` at the offending binding. In a time invariant with a bound over `q[i]` and a presence `exists` predicate, and in an `urgency` `when` with a presence quantifier and no `binders`, the quantifier is scoped by its application alone and the package is admitted. | Test (TC-1830) |
| FR-760-AC-5 | Over an empty `q`, the AC-4 guard's presence `forall` is true and an `exists` false while its slot-bound constraint holds vacuously over no live slot; over a `q` with one live slot the constraint is checked at that slot and the presence quantifier reads that slot's presence, each as FR-415-AC-20 states. | Test (TC-1830) |

## Dependencies

- FR-322 ("Presence", "Quantified guards and invariants", FR-322-AC-155, FR-322-AC-295).
- FR-415-AC-20 (the meaning of a presence quantifier).
