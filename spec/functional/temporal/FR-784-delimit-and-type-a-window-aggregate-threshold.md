---
id: FR-784
title: "Delimit and type a window-aggregate threshold"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-501
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-414
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-339
    type: references
---

# FR-784: Delimit and type a window-aggregate threshold

## Description

When a temporal formula holds an `aggregate-atom`, the recognizer SHALL end
the atom's threshold at the first `and`, `or` or `implies` outside
parentheses and join the atom to the following temporal operand with that
connective, and the checker SHALL refuse a threshold that is not of the
aggregate's value type.

## Rationale

The threshold of an `aggregate-atom` is a `sum`, the operand a comparison
takes (FR-501 item 1). That rule fixes where the threshold ends for every
connective, not only `and`, and it fixes where an aggregate atom may stand:
it is a `temporal-primary`, so it stands wherever another temporal operand
stands, first, last or between others, but it is not an `expr` and so it is
not an operand of `holds(...)`. A threshold that needs a lower-precedence
operator is parenthesized, and the parenthesized expression then has to be
a number of the aggregate's type, because the atom compares the aggregate
with it: a Boolean in parentheses would otherwise reintroduce the silent
reading the `sum` rule removed.

## Inputs

A temporal formula holding one or more aggregate atoms.

## Outputs

A temporal formula tree in which each aggregate atom is one operand, or a
located refusal.

## Behavior

1. **Connectives after the threshold.** After the threshold `sum`, a
   following `and`, `or` or `implies` is the temporal connective of the
   enclosing formula, with its usual precedence and associativity
   (`implies` right associative, then `or`, then `and`). So
   `A >= 1 and holds(q)`, `A >= 1 or holds(q)` and `A >= 1 implies holds(q)`,
   for an aggregate term `A`, are the temporal conjunction, disjunction and
   implication of the atom and `holds(q)`.
2. **No value operand after a connective.** The operand after the
   connective is a temporal operand. A bare value expression there, such as
   `s.ok` in `A >= s.k or s.ok`, is not one, and the recognizer refuses
   `invalid_syntax`/`unexpected-token` at its first token. No parse reads
   `s.k or s.ok` or `s.k implies s.ok` as the threshold.
3. **Parenthesized threshold.** `A >= (e)` reads `e` as a full `expr`. The
   checker SHALL type the threshold against the aggregate's value type as
   FR-414 states: a threshold of another dimension refuses
   `ill_typed`/`unit-mismatch`, and a threshold whose type is not numeric at
   all (a `Boolean`, a `Text`, an `Option`) refuses `ill_typed`/`type-mismatch`
   at the threshold.
4. **Position.** An aggregate atom may be any operand of a temporal
   connective and the operand of a temporal operator (`always`, `eventually`
   and the others FR-414 admits it under), in any position. Inside
   `holds(...)` the text `count where ..` is not an aggregate atom: the
   typed `count<N>(..)` and the refinement population form `count(..)` are
   the only `count` expressions, so `holds(count where ..)` refuses
   `invalid_syntax`/`unexpected-token` at `where`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-784-AC-1 | In a temporal clause over a record trace subject `s`, `count where holds(s.ok) over past[0,1] >= 1 or holds(s.done)` parses as one temporal disjunction and `count where holds(s.ok) over past[0,1] >= 1 implies holds(s.done)` as one temporal implication, each with the aggregate atom (threshold `1`) as its left operand; `count where holds(s.ok) over past[0,1] >= s.k or s.ok` and `count where holds(s.ok) over past[0,1] >= s.k implies s.ok` each refuse `invalid_syntax`/`unexpected-token` at the `s` after the connective. | Test (TC-1901) |
| FR-784-AC-2 | `holds(s.done) and count where holds(s.ok) over past[0,1] >= 1 and holds(s.ok)` parses as a conjunction whose middle operand is the aggregate atom; `always (count where holds(s.ok) over past[0,1] >= 1)` parses with the atom as the operand of `always`. | Test (TC-1901) |
| FR-784-AC-3 | `count where holds(s.ok) over past[0,1] >= (s.k + 1)` checks with threshold `s.k + 1` over an integer field `k`; `count where holds(s.ok) over past[0,1] >= (s.ok and s.done)` parses with the parenthesized threshold and refuses `ill_typed`/`type-mismatch` at `(s.ok and s.done)`, `ok` and `done` being Boolean fields. | Test (TC-1901) |
| FR-784-AC-4 | `always holds(count where holds(s.ok) over past[0,1] >= 1)` refuses `invalid_syntax`/`unexpected-token` at `where`. | Test (TC-1901) |

## Dependencies

- [FR-501](../source-package/FR-501-settle-ambiguous-and-contradictory-grammar-forms.md) item 1 makes the threshold a `sum`.
- [FR-414](FR-414-evaluate-window-aggregates.md) owns aggregate atoms, their profiles and their threshold dimension.
- [FR-339](../source-package/FR-339-define-complete-native-grammar.md) incorporates the grammar.
