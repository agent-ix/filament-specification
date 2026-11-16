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

A part or invariant none of whose constraints or bounds has a slot term
carries no `binders` binding. An `urgency` has no `binders` binding in any
case.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-760-AC-1 | A package whose `clock_guard` part has, as its predicate, a `filament.op.collection.forall` application over the element places of `q: Sequence<Option<Clock>>[0, 2]` binding the parameter `e` (typed by `q`'s selector domain) with body the `presence_read` of `q`'s element place at `self` and `e`, one constraint `x ge 1/1` with no slot term, and no `binders`, is admitted; so are a `time_invariant` with the same predicate and bound `x le 9/1` and no `binders`, and an `urgency` of kind `when` with that predicate. | Test (TC-1830) |
| FR-760-AC-2 | The AC-1 package with a `binders` binding naming `e` added to the guard part, or to the invariant, refuses `invalid_package`/`timed-node-reference` at the binder target, because no constraint or bound has `e` as a slot term. | Test (TC-1830) |
| FR-760-AC-3 | The AC-1 package whose `forall` application is replaced by the `exists` application over the same domain and body is admitted with one guard part, not one part for each slot. | Test (TC-1830) |

## Dependencies

- FR-322 ("Presence", "Quantified guards and invariants", FR-322-AC-155, FR-322-AC-295).
- FR-415-AC-20 (the meaning of a presence quantifier).
