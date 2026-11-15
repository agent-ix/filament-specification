---
id: FR-502
title: "Recognize population bounds, quantified clock bounds, delivery intervals, qualified protocol requirements, clock references and the Text constructor"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-339
    type: references
  - target: ix://agent-ix/filament-specification/FR-153
    type: references
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
  - target: ix://agent-ix/filament-specification/FR-465
    type: references
  - target: ix://agent-ix/filament-specification/FR-141
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-380
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-509
    type: references
---

# FR-502: Recognize population bounds, quantified clock bounds, delivery intervals, qualified protocol requirements, clock references and the Text constructor

## Description

When a `1-draft` unit writes a population bound, a quantified clock bound in a
time invariant, a channel delivery count interval, a qualified name after
`requires temporal`, a `ref(e)` clock reference or a `Text(...)` value, the
parser SHALL recognize it with the complete-V1 production stated in the shared
grammar, and the frontend SHALL give it the meaning stated below.

## Inputs

UTF-8 source bytes of a `1-draft` unit.

## Outputs

A lossless source tree with the productions below, or a located refusal.

## Behavior

Each form below is admitted by the language and given meaning by an existing
requirement. The grammar records it so that a conforming parser and a
conformance vector can be written from the standard alone.

1. **Population bound.**
   `population-bound = 'bound', ident, 'on', population-name, '[', uint, ']'`
   is a declaration, ended by its `]` with no `;`, whose `population-name` is
   `ident, [ '::', member-name ]`, a model alias and a population declaration of
   that model's domain package. It states the authored finite maximum `N` of the
   population (FR-153 declares a population with no maximum). The maximum is part
   of the population's meaning for every clause of the package, never a resource
   limit; an observation whose population holds more than `N` objects refuses
   `cardinality_out_of_bound`/`above-maximum`. `bound` is a complete-V1 reserved
   word and stays a `member-name` in member position.
2. **Quantified clock bound.** A `time-invariant` body conjunct is a
   `clock-bound` or a
   `quantified-clock-bound = ('forall' | 'exists'), '(', ident, 'in', expr, ':', clock-bound, ')'`.
   `forall(e in c: b)` is the conjunction of the bound `b` over the live
   elements `e` binds and `exists` their disjunction, read as FR-415 reads a
   quantifier over a clock collection.
3. **Delivery count interval.** `delivery-policy` additionally admits
   `'[', uint, ',', uint, ']'`, the per-send delivery count interval `[lo, hi]`
   (the number of deliveries one send produces, not elapsed time) whose
   channel-node text FR-465 "Delivery count interval" owns. `lo > hi` refuses
   `ill_typed`/`type-mismatch` at the interval.
4. **Qualified protocol requirement.**
   `protocol-requirement = 'requires', 'temporal', ident, { '::', ident }, ';'`.
   The name resolves once in the unit's own declaration scope and binds a
   temporal clause declared in the same unit. A requirement binds a local clause
   only, so a name that resolves through an import, as every qualified name
   does, refuses `missing_declaration`/`missing-name` at the name, and a name
   that resolves to a declaration that is not a temporal clause refuses
   `ill_typed`/`operator-ineligible` at it. The qualified spelling is recognized
   so that it receives this located semantic refusal instead of an
   `invalid_syntax` that would not say what is wrong.
5. **Clock reference.** `primary` additionally admits `'ref', '(', expr, ')'`,
   and `ref` is a complete-V1 reserved word. For a place expression `p`,
   `ref(p)` is the `Reference<Clock>` data value naming the clock place `p`
   denotes (FR-415); `ref` of a place under a `Map` value, which has no place
   identity, refuses `ill_typed`/`type-mismatch` at the `ref`.
6. **Text constructor.** `primary` additionally admits
   `'Text', '(', [ expr, { ',', expr } ], ')'`, because `Text` is a reserved
   type word and cannot start the ordinary qualified call. `Text("...")` with
   one quoted text argument is the `Text[min, max; profile]` value its position
   expects, holding the argument's retained sequence under that profile
   (FR-141); a position with no expected `Text[..]` type, a non-literal
   argument or another argument count refuses `ill_typed`/`type-mismatch` at
   the call, and a retained length outside the bounds refuses
   `ill_typed`/`type-mismatch` naming the bounds and the retained length.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-502-AC-1 | `bound HistoryBound on Config::config_history [8]` parses as one `population-bound` with label `HistoryBound`, population name `Config::config_history` and maximum `8`; `[0]` parses; `[-1]` and `[x]` refuse `invalid_syntax`/`unexpected-token` at `-` and `x`; `b.bound` parses as member navigation. | Test (TC-1009) |
| FR-502-AC-2 | `time invariant I when { true } { forall(e in self.q: e < 9 ms) and self.x <= 3 ms }` parses with one `quantified-clock-bound` and one `clock-bound`; `exists(e in self.q: e < 9 ms)` parses; `forall(e in self.q: e > 9 ms)` refuses `invalid_syntax`/`unexpected-token` at `>`. | Test (TC-1009) |
| FR-502-AC-3 | `channel ch from a to b carries Boolean ordering unordered delivery [0,3] capacity 4 overflow block;` parses with the delivery count interval `[0,3]`; `delivery [4,3]` refuses `ill_typed`/`type-mismatch` at the interval; `delivery [-1,3]` refuses `invalid_syntax`/`unexpected-token` at `-`. | Test (TC-1009) |
| FR-502-AC-4 | `requires temporal Due;` naming a temporal clause of the unit binds it; `requires temporal Lib::Due;` parses and refuses `missing_declaration`/`missing-name` at `Lib::Due`; `requires temporal f;` naming a function refuses `ill_typed`/`operator-ineligible` at `f`. | Test (TC-1009) |
| FR-502-AC-5 | In a timed model, `self.r = ref(self.x)` parses with one `ref` primary and checks as a write of the place `x` of `self`; `ref(self.m["a"].x)` over a `Map` value refuses `ill_typed`/`type-mismatch` at the `ref`; `function ref using v(): Integer pure { 1 }` refuses `invalid_syntax`/`unexpected-token` at `ref`. | Test (TC-1009) |
| FR-502-AC-6 | `function t using v(): Text[0, 8; nfc] pure { Text("abc") }` parses with one `Text` constructor primary and returns the text `abc`; `Text("abc")` with result `Text[0, 2; nfc]` refuses `ill_typed`/`type-mismatch` naming the bounds `0` and `2` and retained length `3`; `Text("a", "b")`, `Text()` and `Text("abc")` in a function whose result is `Integer` each refuse `ill_typed`/`type-mismatch` at the call. | Test (TC-1009) |

## Dependencies

- [FR-339](FR-339-define-complete-native-grammar.md) incorporates the grammar this requirement extends.
- [FR-153](../type-model/FR-153-query-closed-model-environments.md) owns domain-package populations.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) owns clock places, time invariants and `Reference<Clock>`.
- [FR-465](../../objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md) owns the channel's delivery count interval.
- [FR-141](../type-model/FR-141-evaluate-text-and-enumerations.md) owns `Text[..]` values.
