---
id: FR-640
title: "Carry probabilistic model declarations in the checked package"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-405
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-420
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-205
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-407
    type: references
  - target: ix://agent-ix/filament-specification/FR-411
    type: references
  - target: ix://agent-ix/filament-specification/FR-610
    type: depends_on
---

# FR-640: Carry probabilistic model declarations in the checked package

## Description

When a checked package admits a random parameter, a workload, a reward or a
delay declaration ([FR-405](../../functional/analysis/FR-405-declare-probabilistic-models.md)),
the producer SHALL emit it into the `filament.checked-semantic-graph/v2`
graph as one node with `node_tag: "probabilistic"` and the semantic form
below, and the reader SHALL admit such a package and expose each declaration
as a typed read. A probabilistic model is therefore a checked package like
any other: a provider, a replay and a certificate checker read its
distributions, weights, rewards and delays from the package, never from the
source text, and a package that declares them is never refused for holding
them.

## Properties

Every node below has `node_tag: "probabilistic"`, no `declaration` member and
the derived-node path shape `[node_tag, semantic_form, operand]` of FR-322
"Structural node identity", its identity being its complete body. An
**operation reference** is the pair FR-322 uses for a model operation member
in its operation carriers: `declaring`, a `reference` to the `model` node that
declares the operation, and `operation`, a `literal` text term holding the
operation's name. The declaring node is a `model`/`object_type`, `process` or
`state_machine` node of a selected model, or a source-declared
`model`/`object_type` node of the package (FR-322 "Source-declared object
types", FR-610), whatever origin its type has; a workload's `model` is either
kind of node too. A declaration on an operation of a source-declared type is
emitted like one on a model-owned type's, because a source model is a model:
its operations draw, weigh and wait as a selected model's do. An **exact weight** is an FR-205 exact rational literal in
lowest terms. Each node has exactly one occurrence, with role `declaration`,
whose source region is the declaration's label span.

- **`random_parameter`** — body bindings, in this order: `declaring`,
  `operation` (the operation reference), `parameter` (a `literal` text term,
  the parameter's name) and `support`, an ordered aggregate with one member
  per support value in declared order, each an aggregate of `value` (a literal
  of the parameter's declared type; a quantity literal in the parameter's unit
  for a `Quantity<u>` parameter, FR-142) and `weight` (an exact weight, greater
  than 0). The node carries the weights, not the probabilities: the
  probability of a value is its weight over the sum, which every reader
  derives exactly, so no second number can disagree with the first. The
  declared order is kept because it is the enumeration order of the support
  (FR-181).
- **`workload`** — body bindings, in this order: `model` (a `reference` to the
  `model` node the workload is declared on), `name` (a `literal` text term)
  and `weights`, an aggregate with one member per operation of the model, each
  an aggregate of `declaring`, `operation` and `weight` (an exact weight,
  greater than or equal to 0). Members are in ascending order of
  (`declaring` node id, `operation` name), so the order in which weights are
  written changes neither the body nor the identity.
- **`reward`** — body bindings, in this order: `declaring`, `operation`,
  `name` (a `literal` text term), `value_type` (a `reference` to the integer
  or quantity type node of the reward) and `expression` (a `reference` to a
  `function` node). The function node is a clause function whose owner is the
  `ModelOwner` naming the operation member, whose `parameters` binding
  references the operation's parameters in declared order, random parameters
  included, whose result type is `value_type`, and whose body reads no state;
  it is the function a provider evaluates on a transition's arguments.
- **`delay_distribution`** — body bindings, in this order: `declaring`,
  `operation`, `family` (a `literal` text term, one of `uniform`,
  `uniform_interval`, `exponential` and `discrete`) and `parameters`: empty for
  `uniform`; `lower` and `upper` time quantities for `uniform_interval`; `rate`,
  an exact rational per time unit greater than 0, for `exponential`; and for
  `discrete` an ordered aggregate of `delay` (a time quantity) and `weight` (an
  exact weight, greater than 0) members in declared order. Time quantities are
  in the timed model's time unit (FR-415), converted exactly by FR-142.

## Behavior

### Emission

- The producer SHALL emit one node for each admitted declaration of the
  package, and none for a refused one, after the `model` node and the
  operation's owner nodes it references.
- Two declarations of one form whose bodies are equal cannot both be
  admitted: FR-405 refuses a second distribution, a second reward of one name
  on one operation and a second delay of one operation, and a workload name is
  unique in the package. Equal bodies therefore never merge two declarations.
- A probabilistic claim's scheduler `under W` SHALL resolve to the `workload`
  node whose `model` is the claim's subject model and whose `name` is `W`; the
  workload node is part of the claim's obligation identity (FR-407).

### Reader

The reader SHALL check, for each `probabilistic` node in ascending node-id
order and after FR-322's owner join, and refuse the first failure with
`invalid_package` and the cause given, at the JSON pointer of the failing
member:

1. an operation reference whose `declaring` node is absent, foreign or not a
   `model` node of either origin, or which names no operation of it:
   `invalid-value`. For a source-declared declaring node the strict read
   checks only that the node is a source-declared `model`/`object_type`
   node of the package; the identity-row join, run after the strict read
   over the producer's identity rows (FR-610 "Identity rows"), resolves the
   operation among the `operations` entries of that node's identity row,
   and for a workload among those of the node and of its `supertypes`,
   followed transitively, and refuses a name no entry carries at the same
   member;
2. a `random_parameter` whose `parameter` names no parameter of the
   operation (for a source-declared operation, no entry of its identity
   row's `parameters`, checked at the identity-row join), whose support is empty, repeats a value, or holds a value
   outside the parameter's type or a weight that is not greater than 0:
   `invalid-value`; two `random_parameter` nodes of one operation parameter:
   `conflicting-definition` at the later node;
3. a `workload` whose weights omit, repeat or misname an operation of its
   model, or hold a negative weight, or whose members are out of order:
   `invalid-value`; two `workload` nodes of one name in the package:
   `conflicting-definition` at the later node;
4. a `reward` whose `value_type` is neither an integer nor a quantity type,
   whose `expression` is not a clause function owned by that operation member
   with exactly its parameters, or whose function body reads state:
   `invalid-value`; two `reward` nodes of one name on one operation:
   `conflicting-definition` at the later node;
5. a `delay_distribution` on an operation of a model that is not `time
   dense` (for a selected model, its `declaring` node is not the `model` of a
   `timed_model` root whose source is `dense`; for a source-declared
   `declaring` node, no `timed_model` root whose `model` is a
   source-declared node of the same `SourceOwner` has a `dense` source,
   since one source time member times every source object type of the unit,
   FR-415), an unknown `family`, parameters that do not match the family, or a
   weight or rate not greater than 0: `invalid-value`; two
   `delay_distribution` nodes of one operation: `conflicting-definition` at
   the later node.

The reader SHALL charge one validation visit of the caller's reader budget per
`probabilistic` node and per support, weight or delay member before inspecting
it, and on exhaustion return its `incomplete` result at the first uncharged
member with no partial view. The reader does not evaluate a reward over the
operation's domains: the sign of a reward is the frontend's check (FR-405),
and a provider that reads a package from an unknown producer runs that check
itself before it admits the model (FR-411).

### Typed read

An admitted package SHALL expose, per model operation, its random parameters
with their supports and exact probabilities and its rewards with their
functions, and per model its workloads and its delay distributions, each
keyed by its node identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-640-AC-1 | A package with FSL ADR-024 §7.2's `Service` (two random parameters on `Server::attempt`, the reward `duration` and the workload `Steady`) is emitted with two `random_parameter` nodes, one `reward` node with its clause function and one `workload` node, each with one `declaration` occurrence; the reader admits it and its typed read gives the supports `1`, `3`, `12` with probabilities `9/10`, `7/100`, `3/100`, `duration` typed `Millis`, and weight 1 for each operation of `Steady`. | Test (TC-1563) |
| FR-640-AC-2 | A `time dense` model with `delay Call::reply ~ discrete { 1 ms: 90, 3 ms: 10 };` is emitted with one `delay_distribution` node of family `discrete` whose delays are `1` and `3` in the model's unit `ms` with weights 90 and 10, and the reader admits it; `uniform`, `uniform[1 ms, 4 ms]` and `exponential(rational(1, 2))` are emitted with families `uniform`, `uniform_interval` and `exponential` and admitted. | Test (TC-1563) |
| FR-640-AC-3 | Writing `Steady`'s weights in another order leaves the `workload` node byte-identical; changing one weight, one support value or one reward expression changes that node's identity and the identity of every claim that reads it. A workload with weight 0 on one operation is emitted and admitted. | Test (TC-1563) |
| FR-640-AC-4 | Reader mutations each refuse `invalid_package` at the failing member: a support with a repeated value, a support weight 0, a workload missing an operation, a workload with a negative weight, workload members out of order, a reward whose function reads a state field, a reward typed `Boolean`, a delay on an untimed model and a `discrete` delay with weight 0 (`invalid-value`); a second `random_parameter` of one parameter, a second `workload` of one name, a second `reward` of one name on one operation and a second `delay_distribution` of one operation (`conflicting-definition` at the later node). A reader budget one visit short of the package's count returns `incomplete` at the first uncharged member. | Test (TC-1563) |
| FR-640-AC-5 | A claim `under Steady` over `Service` resolves its scheduler to `Steady`'s `workload` node, and a provider run over the emitted package gives the same step probabilities as FSL FR-187-AC-1. | Test (TC-1563) |
| FR-640-AC-6 | A package of a unit that declares `object O { d: Clock; operation step() modifies self.d; }`, `object Other { n: Integer; operation poke(k: Int[1, 3]) modifies self.n; }` and `time O dense unit ms;`, with `delay O::step ~ uniform;`, `delay Other::poke ~ discrete { 1 ms: 9, 3 ms: 1 };`, `random Other::poke(k) ~ { 1: 1, 3: 3 };` and `workload W on O { weight O::step = 1; }`, is emitted with operation references whose `declaring` is the source node of `O` or `Other` and a `workload` node whose `model` is the source node of `O`; the strict read and the identity-row join admit it and the typed read gives the delays, the support with probabilities `1/4` and `3/4`, and `W` with weight 1 on `O::step`, its weight rows joined to the `operations` entries of `O`'s identity row. Each refuses `invalid_package`/`invalid-value` at the member: at the identity-row join, an `operation` `nudge` that no `operations` entry of `Other`'s row carries a `parameter` `j` that no `parameters` entry of `Other::poke` carries, and a `W` whose weights omit `O::step` or name `O::nudge`; at the strict read, a `declaring` that is a source-declared `relation`/`population` node, and a delay on a source node of a unit whose source model has no `dense` time member. | Test (TC-2033) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the graph, node identity,
  owner joins and the reader budget these nodes use.
- [FR-405](../../functional/analysis/FR-405-declare-probabilistic-models.md)
  admits the declarations; [FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md)
  defines the delay families.
- [FR-205](../foundation/FR-205-exact-rational-value.md) types weights and
  rates; [FR-142](../../functional/type-model/FR-142-evaluate-quantities-and-units.md) types
  quantities.
