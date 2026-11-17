---
id: FR-501
title: "Settle the grammar forms the standard left ambiguous or contradictory"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-339
    type: references
  - target: ix://agent-ix/filament-specification/FR-140
    type: references
  - target: ix://agent-ix/filament-specification/FR-143
    type: references
  - target: ix://agent-ix/filament-specification/FR-149
    type: references
  - target: ix://agent-ix/filament-specification/FR-252
    type: references
---

# FR-501: Settle the grammar forms the standard left ambiguous or contradictory

## Description

When the parser reads a window-aggregate atom, an indexed read, a `let`, a
`Some(...)` call, a `Decimal[..]` type, a complete temporal clause, an
operation name, a `join` list or an `align skip` list, it SHALL recognize the
one form the complete-V1 grammar states for it and give every other spelling
the located refusal stated below.

## Inputs

UTF-8 source bytes of a `1-draft` unit.

## Outputs

A lossless source tree with the productions below, or a located refusal.

## Behavior

1. **Window-aggregate threshold.** The threshold after an `aggregate-atom`'s
   comparison operator is a `sum`, the same operand a `comparison` takes. A
   following `and`, `or` or `implies` therefore joins the aggregate atom to the
   next temporal operand: `count where holds(p) over past[0,1] >= 1 and
   holds(q)` is the temporal conjunction of the atom and `holds(q)`. A
   threshold is a number, and letting it be a full expression silently read
   `>= s.k and s.ok` as the Boolean threshold `(s.k and s.ok)`. A threshold
   that needs a lower-precedence operator is written in parentheses. FR-784 states the same delimitation for `or` and `implies`, where an atom may stand, and how a parenthesized threshold is typed.
2. **Indexed read admitted only in a hyper body.** The `expr` of a
   `holds(...)` leaf and of an `on each ... when` guard is recognized with the
   same expression grammar in every temporal clause, `indexed-read`
   (`postfix @ ident`), `step-label` and `stutter` included. Only the body of a
   `hyper` clause over `behaviours` gives them meaning, because only its
   `forall trace` and `exists trace` quantifiers bind the trace a read is
   indexed by. In any other temporal clause the first such form, in source
   order, refuses `unsupported_construct`/`expression-form` located at the
   form, with a message saying that indexed reads and step labels are admitted
   in `hyper` clauses only. One expression grammar for every `holds(...)` and
   guard keeps both placements refusing with the same code, cause and message
   and keeps the refusal naming what the author must change, which a syntax
   error at `@` would not. Outside temporal and hyper clauses (a function body, a state clause,
   a refinement row) the `@` stays `invalid_syntax`/`unexpected-token` at the
   `@`, because no trace position exists there at all.
3. **Annotated `let`.** `let x: T = e in b` declares the binder `x` with the
   type `T`. `e` is checked with `T` as its expected type, exactly as an
   argument is checked against a declared parameter type, and `x` has type `T`
   in `b`. An `e` that does not conform to `T` refuses
   `ill_typed`/`type-mismatch` at `e`; an `Int[lo,hi]` annotation over an
   arithmetic `e` carries the range obligation FR-149 states. An unannotated
   `let` types `x` by `e`. FR-143 and FR-149 already rely on the annotation as
   a declared-type position.
4. **`Some(e)` everywhere.** `Some(e)` is the present-option constructor in
   every expression position, not only in a refinement row. At a position with
   no expected option type it has type `Option<E>` for `e` of type `E`;
   otherwise, at a position whose expected type is `Option<P>`, it has that
   type and the payload `e` is checked against the expected element type `P`
   (FR-143, which owns how an operand checks at an expected type). A call spelled `Some` always reads as
   the constructor, so a function declared with that name cannot be called by
   it, and `Some` with other than one argument refuses
   `ill_typed`/`type-mismatch` at the call. `absent` and `none` already denote
   the empty option in every expression position; without a general present
   constructor a function could not return a present option.
5. **`Decimal[..]` names its rounding mode.** The source type form is
   `Decimal[lo, hi; smin, smax; mode]`; the `; mode` part is required, and
   `Decimal[lo, hi; smin, smax]` in source is `invalid_syntax`/`unexpected-token`
   at the closing `]`. FR-140 states this and the grammar now agrees: a type
   position never selects a rounding behaviour the author did not write. The
   mode-less spelling stays the domain notation of diagnostics and of wire
   forms, where it means `exact`.
6. **`clock` is optional in a temporal clause.** In both the composed and the
   complete-V1 `temporal-clause`, `clock <string>` is optional. A clause with no
   `clock` binds the default clock FR-252 states (over a model subject,
   `model-time` for a timed model and `model-steps` for an untimed one; over a
   trace subject, the binding the observation interface supplies), and writing
   that default explicitly gives the same binding and obligation identity.
7. **Operation names are a type name and a member.** An `operation-name` is
   `type-name, '::', member-name`, where `type-name` is a model-owned
   `alias::Type` or the bare name of an object type the unit declares. So
   `M::T::op` names operation `op` of the domain-package type `M::T`, and the
   two-segment `T::op` names operation `op` of the source object type `T`; both
   parse. Which one a name denotes is decided by resolution, not by the
   grammar: a name that resolves to no operation refuses
   `missing_declaration`/`missing-name` at the name. The parser's two-segment
   form is therefore the source-object form, not an error.
8. **Lists that name members are never empty.** The `join` list of a
   `parallel` names every branch exactly once and a `parallel` has at least two
   branches, so `join all []` is `invalid_syntax`/`unexpected-token` at `]`.
   An `align skip` list names at least one operation, so `align skip {}` is
   `invalid_syntax`/`unexpected-token` at `}`; a clause that skips nothing
   writes no `align skip`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-501-AC-1 | In a temporal clause, `count where holds(s.ok) over past[0,1] >= 1 and holds(s.done)` parses as one `temporal-conjunction` whose operands are the aggregate atom (threshold `1`) and `holds(s.done)`; `count where holds(s.ok) over past[0,1] >= s.k + 1` parses with the threshold `s.k + 1`; `count where holds(s.ok) over past[0,1] >= s.k and s.ok` refuses `invalid_syntax`/`unexpected-token` at the `s` after `and`, and no parse yields the threshold `s.k and s.ok`. | Test (TC-1002) |
| FR-501-AC-2 | `temporal T using c over (v: M::V) on origin { always holds(v.l @ a) }` parses with no syntax diagnostic and refuses `unsupported_construct`/`expression-form` at `v.l @ a` with a message containing `hyper`, and the same read in an `on each (w: M::V) when (w.l @ a = 0)` guard refuses with the same code, cause and message at `w.l @ a`; `hyper NI using inf over behaviours (v: V::Vault) forall trace a of V forall trace b of V { always holds(v.l @ a = v.l @ b) }` parses with two indexed reads. | Test (TC-1003) |
| FR-501-AC-3 | `function f using v(y: Int[0,2]): Integer pure { let x: Int[0,3] = y + 1 in x }` parses with the annotation `Int[0,3]` on the binder and checks; with `y: Int[0,3]` it refuses `undefined_expression`/`unproved-range` at the `+`; `let x: Boolean = 1 in x` refuses `ill_typed`/`type-mismatch` at `1`; `let x = y + 1 in x` parses with no annotation. | Test (TC-1004) |
| FR-501-AC-4 | Under `filament.value.complete/v1`, `function s using v(b: Int[0,9]): Option<Int[0,9]> pure { Some(b) }` checks and returns the present option holding `b`; `function n using v(b: Integer): Boolean pure { present(Some(b)) }` checks with `Some(b)` typed `Option<Integer>`; `Some(b, b)` and `Some()` refuse `ill_typed`/`type-mismatch` at the call; inside a refinement row `Some(e)` keeps its meaning. | Test (TC-1005) |
| FR-501-AC-5 | `Decimal[0, 9; 0, 2; exact]` and `Decimal[0, 9; 0, 2; nearest-even]` parse as a type reference; `Decimal[0, 9; 0, 2]` as a parameter type, a `convert` target and a `Quantity` carrier each refuse `invalid_syntax`/`unexpected-token` at the closing `]`. | Test (TC-1006) |
| FR-501-AC-6 | Under the complete facets, `temporal T using v over (o: M::O) on origin { always holds(true) }` parses with no `clock` member and binds the FR-252 default; the same clause with that default written as `clock "model-steps"` over an untimed model gives an equal binding and obligation identity. | Test (TC-1007) |
| FR-501-AC-7 | `pre P using v on M::T::op { true }` and `pre P using v on T::op { true }` both parse with one `operation-name`; in a unit declaring `object T { operation op(); }` the second binds the source object's operation, and in a unit declaring no `T` it refuses `missing_declaration`/`missing-name` at `T::op`; `pre P using v on T { true }` refuses `invalid_syntax`/`unexpected-token` at the `{`. | Test (TC-1008) |
| FR-501-AC-8 | `parallel p { branch a sequence s {} branch b sequence t {} } join all [a, b];` parses; the same with `join all []` refuses `invalid_syntax`/`unexpected-token` at `]`; a hyper clause with `align skip { V::Vault::mix }` parses and with `align skip {}` refuses `invalid_syntax`/`unexpected-token` at `}`. | Test (TC-1008) |

## Dependencies

- [FR-339](FR-339-define-complete-native-grammar.md) incorporates the grammar this requirement constrains.
- [FR-140](../type-model/FR-140-evaluate-exact-decimals.md) owns the `Decimal[..]` type.
- [FR-143](../type-model/FR-143-evaluate-record-tuple-recursive-values.md) owns option values and checking at an expected type.
- [FR-149](../type-model/FR-149-apply-complete-equality-matrix.md) owns the range obligation of an arithmetic operand at an `Int[..]` type.
- [FR-252](../../objects/temporal/FR-252-temporal-clock-binding.md) owns the default clock binding.
