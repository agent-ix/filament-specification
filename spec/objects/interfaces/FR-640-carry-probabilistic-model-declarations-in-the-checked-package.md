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
  `function` node). The function node is a clause function of clause kind
  `reward` (its `clause` binding is the text `"reward"`, FR-322 "Owner
  joins") whose owner is the
  `ModelOwner` naming the operation member (for an operation of a
  source-declared type, the unit's `SourceOwner` with the `declaration`
  `[<type>, <operation>, "reward", <reward name>]`, FR-322 "Source-declared
  object types" item 5), whose `parameters` binding
  references the operation's parameters in declared order, random parameters
  included, whose result type is `value_type`, and whose body reads no state;
  it is the function a provider evaluates on a transition's arguments. It
  has its own clause kind, not `precondition` or `body`: a consumer reads an
  operation's guard from its `precondition` clause functions and its body
  from its `body` clause function (FR-322 `operation_bodies`), and a reward
  is neither, so a reward never adds a guard to its operation and never
  makes an operation without a body read as having one. A consumer keys a
  reward's function on the clause kind `reward` or reaches it through the
  `reward` node's `expression`.

  **Reward identity.** The `reward` node's operand (FR-322 "Structural node
  identity") is, in this order: the `declaring` node's id, the operation's
  name, the reward's name, and a **content operand** that spells what the
  function computes: the node id of the expression node its `body` binding
  references, which FR-322 keys by the expression's content, or, for a body
  that is a literal, the literal's operand text. The function node of a reward on a source-declared
  type is a declared node keyed by its `declaration` path at ordinal 0, so
  the lowering-order rule below does not apply to it and its id is fixed by
  the type, operation and reward names. The function node of a reward on a
  selected model's type
  shares its derived path with the other model-owned clause functions of its
  owner and takes its ordinal in **lowering order**: the producer lowers
  every other clause function first and then the package's rewards in
  ascending order of (operation key, reward name), so a reward's function
  ordinal depends on the clause functions and rewards that precede it in
  that order and never on the source order of any declaration. The
  function's key carries no content, so the content operand is what makes a
  changed expression change the reward's identity. Writing the rewards, the
  random parameters, the workloads or the claims of a unit in another order
  therefore changes no node; changing a reward's expression changes that
  reward node's id and the id of every claim that reads the reward (through
  the claim's `reward` binding and path, "Probabilistic claim node"), and
  leaves every other node unchanged; adding a reward, or a clause function
  that precedes it in lowering order, may shift the ordinals of the reward
  functions after it.
- **`delay_distribution`** — body bindings, in this order: `declaring`,
  `operation`, `family` (a `literal` text term, one of `uniform`,
  `uniform_interval`, `exponential` and `discrete`) and `parameters`: empty for
  `uniform`; `lower` and `upper` time quantities for `uniform_interval`; `rate`,
  an exact rational per time unit greater than 0, for `exponential`; and for
  `discrete` an ordered aggregate of `delay` (a time quantity) and `weight` (an
  exact weight, greater than 0) members in declared order. Time quantities are
  in the timed model's time unit (FR-415), converted exactly by FR-142.

**Probabilistic claim node.** A probabilistic claim (FR-407) is one node with
`node_tag: "claim"` and `semantic_form: "probabilistic_claim"`, not a
`probabilistic` node. It has no `declaration` member and exactly one
occurrence, with role `claim`. It carries what the claim reads: its body binds
the nodes the claim reads, and its `node_id` path holds the same nodes as
operand segments. The body lets the reader check and expose what a claim
reads. The path makes the claim's identity cover it, because FR-322
"Structural node identity" keys a node by `{package, path, ordinal}` alone and
never by its body. A sidecar row cannot carry either, because a
`checked_term_data` record never alters a node or its id.

A node *belongs to the subject's model* when it is a node of the package of
the subject's `model_root`, that is, when its `node_id.package` is that root's
selected identity. A probabilistic node belongs to the subject's model when its
`declaring` node does (for a `workload`, its `model` node).

The body is an aggregate of exactly these five bindings, in this order:

1. `subject`: a `reference` to the `model`/`model_root` node of the model
   selection the claim's subject reads, `{package: <selected identity>, path:
   ["model", "model_root"], ordinal: 0}`. It does not depend on the subject's
   alias. A probabilistic claim's subject is always a model selection,
   because a subject that is not a model alias is a trace subject and the
   frontend refuses it (FR-407, FSL FR-186).
2. `scheduler`: for `under W`, a `reference` to the `workload` node whose
   `name` is `W`. For `under every scheduler`, an aggregate with exactly one
   binding, `fairness`. `fairness` is an aggregate with one member per
   fairness constraint. Each member is an aggregate of `kind` (a `literal`
   text term, `weak` or `strong`), `granularity` (a `literal` text term,
   `whole` or `each`; a constraint written with no granularity is `whole`,
   FSL FR-123), `declaring` and `operation` (the operation reference of the
   constrained operation). Granularity is part of the constraint: `fair weak
   whole X` and `fair weak each X` are two distinct members, because the
   two granularities constrain the scheduler differently (FR-362). Members are in
   ascending order of (`declaring` node id, `operation` bytes, `kind` bytes,
   `granularity` bytes), with no repeats, so the order in which constraints
   are written changes nothing. An empty
   fairness set is an empty aggregate.
3. `reward`: an aggregate of `reference`s to every `reward` node that belongs
   to the subject's model and whose `name` is the one reward the claim's form
   reads, in ascending node-id order. The forms that read a reward `R` are
   `accumulate R` (in a probability comparison or a quantile), a fraction or
   long-run fraction `weighted by R`, and `expected accumulate R` (FR-407
   "Forms"). A form holds at most one measure, so it reads at most one reward
   name. An operation that declares no `R` contributes 0 to the measure, so it
   has no node and no member. For every other form (an event formula, `steps`,
   `elapsed`, an unweighted fraction, `weighted by delay`, a reachability form
   and `expected elapsed`) the binding is an empty aggregate.
4. `distributions`: an aggregate of `reference`s to every `random_parameter`
   node that belongs to the subject's model, in ascending node-id order, empty
   when there is none. These are the distributions the subject's transition
   probabilities read whatever the scheduler: a weight of 0 under `W` does
   not remove a parameter. The binding may name more than one claim's
   behaviour reaches, never less.
5. `delays`: an aggregate of `reference`s to every `delay_distribution` node
   that belongs to the subject's model, in ascending node-id order. It is
   empty for an untimed subject. A timed claim's step distribution reads
   these delays (the race of FR-420), and so do `elapsed`, `expected elapsed`
   and `weighted by delay`.

The node's `node_id` path is

`["claim", "probabilistic_claim", N, "subject", {subject}, "scheduler", S,
"reward", {r}…, "distributions", {p}…, "delays", {d}…]`

where `N` is the claim's declared name, `{x}` is the operand segment naming
node `x`, and the operands are in the body's member order. `S` is
`{workload}` for `under W`. For every scheduler, `S` is the string `"every"`
followed, per fairness member in body order, by four items: its `kind`
string, its `granularity` string, its `{declaring}` operand and its
`operation` string. The binding-name strings
separate the variable-length groups, so two different bodies never give one
path. The node's `ordinal` ranks the claims of one package and one path by
their written content (FSL FR-498). The claim's form, event or measure,
threshold, `over` binding, activation and confidence parameters are not body
members or path operands yet; claims that share a path are told apart by that
ordinal. When they are added, they follow `delays`.

The claim's identity therefore covers every node it reads. Each node it reads
holds its content in its own id: a workload its sorted weights, a random
parameter its support, a delay its family and parameters, and a reward its
content operand ("Reward identity"). So a claim's `node_id` changes whenever
a node it reads changes, or its subject, scheduler or name changes, and
stays unchanged when a workload's weight lines, a fairness set's constraints
or the unit's declarations are written in another order.

FR-407 "Obligation identity" also includes the claim's form, event or
measure, threshold and confidence parameters. The node identity does not
cover those yet: a change to one of them changes the claim's written content,
which can change its ordinal among claims that share its path, but a claim
alone on its path keeps ordinal 0 and its `node_id`. Until the form members
are body members and path operands (Dependencies, "Open"), this requirement
settles only that the `node_id` covers everything the claim reads.

## Behavior

### Emission

- The producer SHALL emit one node for each admitted declaration of the
  package, and none for a refused one, after the `model` node and the
  operation's owner nodes it references.
- Two declarations of one form whose bodies are equal cannot both be
  admitted: FR-405 refuses a second distribution, a second reward of one name
  on one operation and a second delay of one operation, and a workload name is
  unique in the package. Equal bodies therefore never merge two declarations.
- The producer SHALL emit a probabilistic claim's node after the `model_root`
  node and every `probabilistic` node its body references, with the body and
  path of "Probabilistic claim node". Its scheduler `under W` SHALL resolve to
  the `workload` node whose `model` belongs to the claim's subject model and
  whose `name` is `W`, held in its `scheduler` binding and as an operand of
  its path. Because the workload node is part of the claim's identity, it is
  part of the claim's obligation identity (FR-407). The producer SHALL NOT
  hold what a claim reads in any other table or sidecar row.

### Reader

The reader SHALL check, for each `probabilistic` node in ascending node-id
order and after FR-322's owner join and its `operation_effects` and
`operation_bodies` carriers (FR-322 `operation_bodies` "Check order"),
and refuse the first failure with
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
   whose `expression` is not a clause function of clause kind `reward` owned
   by that operation member (for a source-declared `declaring` node, carrying
   the unit's `SourceOwner` and the `declaration` `[<type>, <operation>,
   "reward", <name>]` of that type, operation and reward) with exactly its
   parameters, or whose function
   body reads state:
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

After every `probabilistic` node, the reader SHALL check each
`claim`/`probabilistic_claim` node in ascending node-id order, and refuse the
first failure in the same way:

6. a `probabilistic_claim` node whose body is not exactly the five bindings
   of "Probabilistic claim node" in order; whose `subject` does not reference
   a `model`/`model_root` node; whose `scheduler` is neither a `reference` to
   a `workload` node whose `model` belongs to the subject's model nor an
   aggregate of exactly one `fairness` binding; whose fairness member has a
   `kind` other than `weak` or `strong` or a `granularity` other than `whole`
   or `each`, names an operation the subject's
   model does not declare (rule 1's operation-reference check), or is
   repeated or out of order; whose `reward` members are not `reward` nodes,
   carry two reward names, do not belong to the subject's model, are out of
   order or, when there is at least one, omit a `reward` node of that name
   that belongs to the subject's model; or whose `distributions`, respectively
   `delays`, is not exactly the set of `random_parameter`, respectively
   `delay_distribution`, nodes that belong to the subject's model in
   ascending node-id order: `invalid-value` at the failing member (at `body`
   for a body that is not the five bindings). The form is not a body member,
   so the reader does not check whether the form reads a reward, and an empty
   `reward` is admitted. As for every derived node, the reader does not
   compare the claim's path with its body (FR-322 "Structural node
   identity").

The reader SHALL charge one validation visit of the caller's reader budget per
`probabilistic` node and per support, weight or delay member, and per
`probabilistic_claim` node and per `reward`, `distributions`, `delays` and
fairness member, before inspecting it, and on exhaustion return its `incomplete` result at the first uncharged
member with no partial view. The reader does not evaluate a reward over the
operation's domains: the sign of a reward is the frontend's check (FR-405),
and a provider that reads a package from an unknown producer runs that check
itself before it admits the model (FR-411).

### Typed read

An admitted package SHALL expose, per model operation, its random parameters
with their supports and exact probabilities and its rewards with their
functions, and per model its workloads and its delay distributions, each
keyed by its node identity. It SHALL expose, per probabilistic claim node,
its subject root, its workload or fairness set, its reward nodes, its random
parameter nodes and its delay nodes, read from the claim's body and keyed by
the claim's node identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-640-AC-1 | A package with FSL ADR-024 §7.2's `Service` (two random parameters on `Server::attempt`, the reward `duration` and the workload `Steady`) is emitted with two `random_parameter` nodes, one `reward` node with its clause function and one `workload` node, each with one `declaration` occurrence; the reader admits it and its typed read gives the supports `1`, `3`, `12` with probabilities `9/10`, `7/100`, `3/100`, `duration` typed `Millis`, and weight 1 for each operation of `Steady`. | Test (TC-1563) |
| FR-640-AC-2 | A `time dense` model with `delay Call::reply ~ discrete { 1 ms: 90, 3 ms: 10 };` is emitted with one `delay_distribution` node of family `discrete` whose delays are `1` and `3` in the model's unit `ms` with weights 90 and 10, and the reader admits it; `uniform`, `uniform[1 ms, 4 ms]` and `exponential(rational(1, 2))` are emitted with families `uniform`, `uniform_interval` and `exponential` and admitted. | Test (TC-1563) |
| FR-640-AC-3 | Writing `Steady`'s weights in another order leaves the `workload` node and every claim node `under Steady` byte-identical. A workload with weight 0 on one operation is emitted and admitted. Over FR-640-AC-8's unit, each change below changes exactly the stated `node_id`s and `probabilistic_claims` rows and no other claim's: one weight of `Steady` changes `Steady`'s node and `P95`, `Avail` and `Cost`, and not `Fair`; one support value, and separately one support weight, of `d` changes that `random_parameter` node and all four claims; `duration` on `Server::request` rewritten from `1 ms` to `2 ms` changes that `reward` node and `P95` only; `cost` rewritten from `1` to `2` changes the `cost` node and `Cost` only; `Fair`'s constraint rewritten `fair strong Server::reset` changes `Fair` only, and rewritten `fair weak each Server::reset` changes `Fair` only. Writing the four claims, the rewards and `Steady`'s weight lines in another order changes no byte. For a unit with rewards `cost` and `load` on `Server::attempt` and `stable` on `Server::reset`, a random parameter, a workload and a claim, the declaration orders [`stable`, workload, `load`, `cost`], [`load`, `stable`, `cost`, workload] and [workload, `load`, `stable`, `cost`] emit byte-identical packages, the reward functions ranked `cost`, `load`, `stable` by (operation key, name); changing `load`'s expression from `d` to `d + 1` changes the `load` node's id (its content operand) and the ids of the claims that read `load`, keeps the id of `load`'s function node, and changes no other node id except the new expression nodes of that function's body. | Test (TC-1563) |
| FR-640-AC-7 | In `Service`, the `duration` reward's function carries `clause` `"reward"`; the reader admits the package, gives `Server::attempt` no precondition read from that function, and keeps its `operation_bodies` row `no_body` (FR-322-AC-471). The reward rewritten to name a function whose `clause` is `"precondition"`, and one owned by another operation member `Server/reset`, each refuse `invalid_package`/`invalid-value` at the `reward` node's `/body/expression` member (rule 4). FR-640-AC-6's unit with a reward `cost` on `Other::poke` returning `k` emits the function with the unit's `SourceOwner`, `declaration` `["Other", "poke", "reward", "cost"]`, ordinal 0 and `clause` `"reward"`, and the reader admits it; the same function with `declaration` `["Other", "poke", "reward", "load"]` refuses rule 4 at `/body/expression`. | Test (TC-1563) |
| FR-640-AC-8 | `Service` with `reward duration on Server::request = 1 ms;` and `reward cost on Server::reset = 1;` added, and four claims: `P95` (FSL ADR-024 §7.2, `quantile 0.95 of (accumulate duration …) <= 5 ms under Steady`), `Avail` (`probability >= 0.5 [always[0,2] holds(true)]` under `Steady`), `Fair` (the same event `under every scheduler fair weak Server::reset`) and `Cost` (`expected accumulate cost until holds(s.phase = Done) <= 3` under `Steady`). Each claim node has `node_tag` `claim`, no `declaration`, one `claim` occurrence and the five bindings in order, with `subject` the `{package: "example/service", path: ["model", "model_root"], ordinal: 0}` node. `P95`: `scheduler` `Steady`'s node, `reward` the two `duration` nodes (on `Server::attempt` and `Server::request`) in ascending node-id order, `distributions` the two `Server::attempt` nodes, `delays` empty. `Avail`: `reward` empty. `Fair`: `scheduler` an aggregate whose `fairness` has one member (`kind` `weak`, `granularity` `whole`, `Server`'s node, `reset`), `reward` empty. `Cost`: `reward` the one `cost` node. Each path is the "Probabilistic claim node" path of its body; `Fair`'s is `["claim", "probabilistic_claim", "Fair", "subject", {root}, "scheduler", "every", "weak", "whole", {Server}, "reset", "reward", "distributions", {d}, {outcome}, "delays"]` with `{d}` and `{outcome}` in ascending node-id order. The reader admits the package and its typed read gives each claim's scheduler, rewards and distributions. With `Fair` written `under every scheduler fair weak whole Server::reset fair weak each Server::reset`, its `fairness` has two members that differ only in `granularity`, `each` before `whole`, and its path holds two four-item groups `"weak", "each", {Server}, "reset", "weak", "whole", {Server}, "reset"`; the reader admits it. | Test (TC-1563) |
| FR-640-AC-9 | FR-640-AC-8's unit is extended with `model Avail = "example/avail"` (FSL ADR-024 §7.3) and `workload Calm on Avail` with weight 1 on each `Avail` operation, so the package holds `Avail`'s `model_root` and `Calm`'s `workload` node, and the reader admits it. Over that package, each mutation refuses `invalid_package`/`invalid-value` at the named member of the claim node: `P95`'s body without `delays` (at `body`); `P95`'s `subject` referencing `Server`'s `object_type` node (at `subject`); `P95`'s `scheduler` referencing the `duration` reward node, and referencing `Calm`'s node, whose `model` belongs to `Avail` and not to `Service` (at `scheduler`); `Fair`'s fairness member with `kind` `medium`, with `granularity` `some`, and with `operation` `nope` (at the member); `Fair`'s fairness set holding one member twice (at the second); `P95`'s `reward` without the `Server::request` `duration` node, with the `cost` node added, and with its two members swapped (at `reward`); `Avail`'s `distributions` without the `outcome` node, and with its two members swapped (at `distributions`); `Avail`'s `delays` holding the `d` node (at `delays`). Rewriting `Avail`'s path to hold `Calm` in place of `Steady` while its body still references `Steady` is admitted, because the reader does not compare a path with a body. Over FR-640-AC-8's package itself, the claims are checked in node-id order `Avail`, `Cost`, `Fair`, `P95`, each node charged before its members in body order, so a reader budget one visit short of the package's count returns `incomplete` at `P95`'s second `distributions` member. | Test (TC-1563) |
| FR-640-AC-4 | Reader mutations each refuse `invalid_package` at the failing member: a support with a repeated value, a support weight 0, a workload missing an operation, a workload with a negative weight, workload members out of order, a reward whose function reads a state field, a reward typed `Boolean`, a delay on an untimed model and a `discrete` delay with weight 0 (`invalid-value`); a second `random_parameter` of one parameter, a second `workload` of one name, a second `reward` of one name on one operation and a second `delay_distribution` of one operation (`conflicting-definition` at the later node). A reader budget one visit short of the package's count returns `incomplete` at the first uncharged member. | Test (TC-1563) |
| FR-640-AC-5 | A claim `under Steady` over `Service` resolves its scheduler to `Steady`'s `workload` node, and a provider run over the emitted package gives the same step probabilities as FSL FR-187-AC-1. | Test (TC-1563) |
| FR-640-AC-6 | A package of a unit that declares `object O { d: Clock; operation step() modifies self.d; }`, `object Other { n: Integer; operation poke(k: Int[1, 3]) modifies self.n; }` and `time O dense unit ms;`, with `delay O::step ~ uniform;`, `delay Other::poke ~ discrete { 1 ms: 9, 3 ms: 1 };`, `random Other::poke(k) ~ { 1: 1, 3: 3 };` and `workload W on O { weight O::step = 1; }`, is emitted with operation references whose `declaring` is the source node of `O` or `Other` and a `workload` node whose `model` is the source node of `O`; the strict read and the identity-row join admit it and the typed read gives the delays, the support with probabilities `1/4` and `3/4`, and `W` with weight 1 on `O::step`, its weight rows joined to the `operations` entries of `O`'s identity row. Each refuses `invalid_package`/`invalid-value` at the member: at the identity-row join, an `operation` `nudge` that no `operations` entry of `Other`'s row carries a `parameter` `j` that no `parameters` entry of `Other::poke` carries, and a `W` whose weights omit `O::step` or name `O::nudge`; at the strict read, a `declaring` that is a source-declared `relation`/`population` node, and a delay on a source node of a unit whose source model has no `dense` time member. | Test (TC-2033) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the graph, node identity,
  owner joins and the reader budget these nodes use.
- **Open:** the form, event or measure, threshold, `over` binding, activation
  and confidence members of the `probabilistic_claim` body and path, which
  FR-407 "Obligation identity" requires the identity to cover; until they
  are defined, the claim's `node_id` covers what it reads and its subject,
  scheduler and name, and not those members.
- [FR-405](../../functional/analysis/FR-405-declare-probabilistic-models.md)
  admits the declarations; [FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md)
  defines the delay families.
- [FR-205](../foundation/FR-205-exact-rational-value.md) types weights and
  rates; [FR-142](../../functional/type-model/FR-142-evaluate-quantities-and-units.md) types
  quantities.
