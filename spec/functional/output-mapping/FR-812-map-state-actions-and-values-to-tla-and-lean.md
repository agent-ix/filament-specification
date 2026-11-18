---
id: FR-812
title: "Map state, actions, values and arithmetic to TLA+ and Lean 4"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-811
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: references
---

# FR-812: Map state, actions, values and arithmetic to TLA+ and Lean 4

## Description

When the TLA+ or Lean 4 mapper exports a state model, its operations and
its values, it SHALL use the encodings below, each of which gives the target
text the FSL meaning exactly, including FSL's undefined evaluations, exact
numbers and division law, so that an obligation built only from these
constructs is `preserved`.

## Inputs

The obligation's dependency closure (FR-811).

## Outputs

The state, action and value regions of the TLA+ module or Lean file.

## Behavior

### State and actions

| FSL | TLA+ | Lean 4 |
| --- | --- | --- |
| population of object type `T` | one variable `v_T`, a function from the population's present keys to field records; its domain is the present key set | one `State` field `v_T : List (Key_T × Rec_T)`, sorted by key, keys distinct |
| population bound | `TypeOK` conjunct `Cardinality(DOMAIN v_T) <= n` | the well-typedness predicate `WF` conjunct `v_T.length <= n` |
| field type | `TypeOK` conjunct over every present object | `WF` conjunct |
| initial snapshot or constraint | `Init` | `def Init (s : State) : Prop` |
| operation `op(p1, …)` | `op_op(p1, …) == pre /\ post /\ frame /\ wf'` with `UNCHANGED` for every variable the frame does not write | `def op_op (s s' : State) (p1 …) : Prop` with the same conjuncts |
| the step relation | `Next == \E p1 \in Dom1, … : op_a(…) \/ …`, one disjunct per operation in declaration order | `def Step (s s' : State) : Prop`, one disjunct per operation |
| a behaviour | `Spec == Init /\ [][Next]_vars /\ WF_vars(Next) /\ fairness` | `structure Behaviour` over `Nat → State` with `Init (b 0)` and, at each `n`, `Step (b n) (b (n+1))` or `b (n+1) = b n` exactly when no step is enabled at `b n` |

- A population's bound `n` SHALL be the authored maximum of a
  `bound L on M::p [n]` declaration (FR-502), the one place source gives a
  model population a maximum. A population that no `bound` names has no
  maximum, because a domain-package population declaration (FR-153) and a
  source `population` member (FR-610) declare none. The identity rows of
  FR-610 list a type's populations by `{name, symmetric}` only and are not a
  source of bounds. An unbounded population's variable carries no
  `Cardinality` or `length` conjunct, and its meaning is still exact. The
  mapper reads the bound from the checked package it exports. It never
  reads it from a limit, a snapshot or a tool setting, because the bound is
  part of the population's meaning (FR-502) and not a resource limit.
- `Init` SHALL state the request's `initial` snapshot when one is named
  (FR-817). Otherwise it states the model's declared initial constraints.
  For a hybrid, these are its `hybrid-initial` entries, one disjunct per
  entry, or the implicit `mode M when { true }` (shared grammar) when it
  declares none. A state model of object populations declares no initial
  constraint in source, so its `Init` is its well-typedness: `Init ==
  TypeOK` (TLA+) and `def Init (s : State) : Prop := WF s` (Lean), and every
  well-typed state within its population bounds is initial. A `constraint`
  clause cuts a search (FR-382) and an invariant is an obligation, so
  neither is read into `Init`. The export then states the model as declared
  and adds nothing that the source does not say. A caller who wants the
  behaviours from one start state names that state's snapshot.
- `wf'` SHALL be the post-state well-typedness that native execution checks
  (FR-181); an operation whose post-state is not well-typed is not enabled
  in either target, as in FSL.
- `WF_vars(Next)` makes a TLA+ behaviour take a step whenever one is
  enabled, which is FSL's reading of a behaviour: it stutters only at a
  terminal state. Finite stuttering, which every TLA+ formula ignores, is
  handled for step-sensitive clauses by FR-813's step counter.
- An argument domain is the FSL parameter type's value set, written as a
  finite set when it is finite and as `Int` (TLA+) or `Int` with a range
  conjunct (Lean) otherwise; the meaning is exact either way, and only a
  finite domain is enumerable by an explicit-state observer.

### Values

| FSL value | TLA+ | Lean 4 |
| --- | --- | --- |
| `Boolean` | `BOOLEAN` | `Bool` |
| `Integer`, integer range | `Int`, range as a `TypeOK` conjunct | `Int`, range as a `WF` conjunct |
| `Rational`, `Decimal`, decimal range | a record `[num \|-> n, den \|-> d]` with `d > 0` and `gcd(n, d) = 1`, through the generated operators `Q_add`, `Q_sub`, `Q_mul`, `Q_div`, `Q_lt`, `Q_le` | `structure Q` with `num : Int`, `den : Nat`, the same normal form and generated operations |
| quantity with a unit | its exact rational value in the dimension's canonical unit, the unit kept in a comment | the same |
| binary floating point | a tagged record (`finite` with sign, integer significand and exponent; `inf` with sign; `nan` with its exact bit pattern `bits`, below) and generated operators that round an exact rational result by the package's rounding rule | an `inductive F` of the same tags and the same generated operators |
| `Text` | the sequence of its Unicode scalar values as integers, `<<104, 105>>` for `"hi"`, with the text in a comment | the sequence as `List Nat` of scalar values |
| enum | the set of strings of its mangled member names (FR-814) | an `inductive` with one constructor per member, `deriving DecidableEq` |
| record, tuple | a TLA+ record, a tuple | a `structure`, a `structure` with fields `_1`, `_2`, … |
| option | `[tag \|-> "none"]` or `[tag \|-> "some", value \|-> v]` | `Option` |
| union, variant, recursive type | a tagged record per variant, recursive operators declared `RECURSIVE` | an `inductive` |
| sequence | a TLA+ sequence | `List` |
| set, ordered set | a finite set; an ordered set as a sequence of distinct elements | a `List` sorted in the canonical value order with no duplicates |
| bag | a function from elements to positive counts | a sorted `List (Elem × Nat)` |
| map | a function on its key set | a sorted `List (Key × Val)`, keys distinct |
| object reference | the key | the key |

The Lean set, bag and map encodings keep one canonical list per value, so
list equality is value equality.

A `nan` value SHALL carry `bits`, its exact IEEE 754-2019 bit pattern (sign,
quiet bit and payload) as FR-181 writes it: 8 lowercase hexadecimal digits for
`Float32` and 16 for `Float64`, big-endian, as a TLA+ string and a Lean
`String`. The export is therefore lossless: two NaNs export equal exactly when
their bits are equal, which is when FR-181 gives them equal state keys, so the
exported state space neither merges states FSL keeps apart nor splits states FSL
merges. The tag carries no other spelling of the NaN, and no generated operator
reads a NaN through a target's own floating point.

### Arithmetic and undefinedness

- Integer and exact arithmetic SHALL be exact in both targets; no operation
  wraps or saturates.
- Integer division and remainder SHALL be emitted through generated
  operators defined from first principles for the package's selected law
  (Euclidean, floor or truncating), never through a target's own division
  operator, whose meaning at negative or zero operands differs between
  targets and releases.
- Every partial operation (division or remainder by zero, a conversion out
  of range, an absent option read, an index out of bounds) SHALL carry a
  definedness condition. A clause exports as the conjunction of the
  definedness conditions of its operations, in evaluation order, with its
  value, so an undefined evaluation makes the exported clause false, as it
  refutes the FSL clause (FR-301 `UndefinedEvaluation`). Neither target's
  totalization (TLA+'s unspecified values, Lean's `x / 0 = 0`) is relied on.
- Collection queries (`sum`, `count`, `filter`, `map`, `forall`, `exists`)
  SHALL be emitted as generated recursive definitions that fold in the
  collection's canonical order.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-812-AC-1 | A model with a population of `Account` keyed by text, a field `balance: Int[0, 100]` and an operation `deposit(a, n: Int[1, 10])` exports `v_Account`, `TypeOK`, `Init`, `op_deposit`, `Next` and `Spec` with `WF_vars(Next)` to TLA+, and the matching `State`, `WF`, `Init`, `op_deposit` and `Step` to Lean; every record is `preserved`. | Test (TC-1971) |
| FR-812-AC-2 | An invariant `self.total / self.count >= 1` exports with the definedness conjunct `count /= 0` (TLA+) and `count ≠ 0` (Lean) before the comparison, so a state with `count` 0 violates the exported invariant. | Test (TC-1971) |
| FR-812-AC-3 | `(-7) div 2` under the floor law exports through the generated floor operator and evaluates to `-4` in both targets; under the truncating law, to `-3`. | Test (TC-1971) |
| FR-812-AC-4 | The decimal `0.10 + 0.20` exports as the normalized rational `3/10`, and a `Float64` sum `0.1 + 0.2` exports through the generated rounding operator and compares unequal to the rational `3/10`, as in FSL. | Test (TC-1971) |
| FR-812-AC-5 | A set `{3, 1, 2}` exports to Lean as `[1, 2, 3]`, and two sets built in different orders export to equal Lean lists; the text `"hé"` exports as the scalar sequence `<<104, 233>>`. | Test (TC-1971) |
| FR-812-AC-6 | With `bound B on M::accounts [3]`, the TLA+ `TypeOK` holds the conjunct `Cardinality(DOMAIN v_Account) <= 3` and the Lean `WF` holds `v_Account.length <= 3`. Without the `bound`, neither conjunct appears. | Test (TC-2031) |
| FR-812-AC-7 | Exported without `initial`, a state model writes `Init == TypeOK` and `def Init (s : State) : Prop := WF s`, and its `constraint` clause appears in neither. `Braking` with no `hybrid-initial` writes an `Init` whose one disjunct is the mode `Moving` with `TRUE`. With `initial` naming a snapshot, `Init` states that snapshot's state. | Test (TC-2031) |
| FR-812-AC-8 | The `Float64` NaNs `7ff8000000000000`, `7ff8000000000001` and `fff8000000000000` and the `Float32` NaN `7fc00001` export to TLA+ as `[tag \|-> "nan", bits \|-> "7ff8000000000000"]`, `[tag \|-> "nan", bits \|-> "7ff8000000000001"]`, `[tag \|-> "nan", bits \|-> "fff8000000000000"]` and `[tag \|-> "nan", bits \|-> "7fc00001"]`, and to Lean as `F.nan` applied to the same strings; the first three are pairwise unequal in both targets, and two states differing only in a NaN payload export as two distinct states. | Test (TC-1971) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-811](FR-811-classify-every-construct-an-export-reads.md); FR-181 owns
  native execution.
