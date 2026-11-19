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
  - target: ix://agent-ix/filament-specification/FR-370
    type: references
  - target: ix://agent-ix/filament-specification/FR-475
    type: references
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

The body is an aggregate of exactly these nine bindings, in this order. The
first five carry the nodes the claim reads; the last four carry what the claim
states (FR-407 "Obligation identity"):

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
6. `form`: the claim's form, event or measure and threshold (FR-407 "Forms"),
   an aggregate whose first binding is `kind`, a `text` literal naming the
   form, followed by exactly the bindings of that kind, in this order:

   | `kind` | Bindings after `kind` | Source form |
   | --- | --- | --- |
   | `probability` | `event`, `bound` | `probability >= θ [E]`, `probability <= θ [E]` over a bounded or timed event |
   | `quantile` | `level`, `measure`, `bound` | `quantile q of M <= c`, `>= c` |
   | `mean_of_fraction` | `measure`, `bound` | `mean of F >= θ`, `<= θ` |
   | `long_run_fraction` | `predicate`, `weighting`, `bound` | `long_run_fraction holds(P) >= θ`, `<= θ`, optionally weighted |
   | `reach` | `target`, `bound` | the reachability forms under the infinite-trace profile |
   | `expected_reward` | `until`, `bound` | `expected accumulate R until holds(B) <= c`, `>= c` |
   | `expected_elapsed` | `until`, `bound` | `expected elapsed until holds(B) <= c`, `>= c` |

   The nested members are:

   - `event`: `kind` `formula` followed by `formula`, a `reference` to the
     `temporal`/`formula` node whose body root is the event's temporal term,
     built as [FR-370](FR-370-checked-package-temporal-clause-body.md) builds
     a clause's formula, its intervals in its operators' `temporal_interval`
     members. FR-370 "Placement and profile fit" admits a formula reference
     at this binding and nowhere else in the claim body; or `kind` `compare` followed by `measure` and `bound`, for
     `M <= c` and `M >= c`. A formula event's horizon is not a member: it is
     the sum of the formula's interval bounds, which the formula node already
     holds, so no second number can disagree with it.
   - `measure`: `kind` followed by, for `accumulate`, `steps` and `elapsed`,
     `from`, `until` and `within` (the horizon `h`); for `fraction`,
     `predicate`, `window` (the `h` of `over [0, h]`) and `weighting`. A
     quantile's measure is an `accumulate`, `steps` or `elapsed` measure and a
     mean's is a `fraction` measure (FR-407 "Forms").
   - `weighting`: `kind` alone, `none` for an unweighted fraction, `reward`
     for `weighted by R` and `delay` for `weighted by delay`. The reward that
     an `accumulate` measure, a `reward` weighting or an `expected_reward`
     form reads is named by the nodes of binding 3, so the form holds no
     reward name and no second carrier of that name can disagree with it.
   - `target`: `kind` `eventually` or `always` followed by `predicate`, or
     `kind` `until` followed by `left` and `right`, for `[eventually
     holds(P)]`, `[always holds(P)]` and `[holds(A) until holds(B)]`.
   - `bound`: `direction`, a `text` literal, `at_least` for `>=` and
     `at_most` for `<=`, followed by `value`, an exact value. The strict
     reading of `quantile q of M >= c` (FR-407 "Thresholds and units") is
     the meaning of `at_least` in a quantile, not a member of its own.
   - `from`, `until`, `predicate`, `left` and `right`: a `reference` to the
     predicate node of that state predicate (the `P` of `holds(P)`), an
     `expression` node or a Boolean `value`/`literal` node, as
     [FR-475](FR-475-checked-package-state-graph-claim-body.md) "predicate
     nodes" defines one.
   - `level`, `within` and `window`: exact values.

   An **exact value** is an inline `literal` term of `value_kind` `integer`,
   `rational` or `quantity` whose `value` has the one canonical spelling
   FR-322 "Scalar literal values" gives it (for an `integer`, the canonical
   decimal integer: no sign but a leading `-`, no leading zero), so one
   number has one spelling. A dimensionless threshold written `0.5`,
   `rational(1, 2)`, `rational(2, 4)` or `decimal(50, 2)` is the `rational`
   `1/2` in each case, and a quantity threshold is converted exactly
   (FR-142) into the unit of its position, so `5 ms` and `0.005 s` over a
   `Millis` reward are both the `quantity` `5` typed at the unit node of
   `ms`. Each position takes one literal kind and one `type` node, the node the
   table names; a literal of the right kind typed at another node (an
   `integer` typed at an `integer_range` node, which FR-322's family check
   would admit elsewhere) is refused by rule 7.

   The subject is **timed** when a `timed_model` root whose `model` belongs
   to the subject's model exists, whatever its time source: `time M dense
   unit u` and `time M tick O period p unit u` (FR-415) each give the model a
   model-time binding, and FR-407 asks only for that binding. The model's
   **time unit** is that root's source unit `u`. A `delay` weighting needs a
   `dense` source as well, because delay distributions exist only on a
   `dense` model (rule 5).

   Each position takes:

   | Position | Exact value |
   | --- | --- |
   | `bound` of `probability`, `mean_of_fraction`, `long_run_fraction` and `reach`; `level`; `bound` of a `fraction` measure's comparison | `rational`, typed at the builtin `scalar_type`/`rational` node |
   | `bound` of a `steps` measure's comparison or quantile | `integer`, typed at the builtin Integer node |
   | `bound` of an `accumulate` measure's comparison or quantile, and of `expected_reward` | `quantity` typed at the unit node of the reward's `value_type` (the `scalar_type`/`unit` or `compound_unit` node of its quantity) for a quantity reward; `rational` for an integer reward |
   | `bound` of an `elapsed` measure's comparison or quantile, and of `expected_elapsed` | `quantity` typed at the unit node of the subject model's time unit (FR-415) |
   | `within` and `window` | `integer` over an untimed subject; over a timed subject a `quantity` typed at the unit node of the model's time unit |
   | each confidence parameter | `rational`, typed at the builtin `scalar_type`/`rational` node |

7. `over`: a `reference` to the `value`/`parameter` node of the claim's
   `over (p: T)` parameter, or the empty aggregate for a claim written
   without one, as FR-475 encodes a state-graph claim's `over`.
8. `activation`: an aggregate of `kind` `origin` alone for `on origin`, or of
   `kind` `each` followed by `trigger`, a `reference` to the `value`/`parameter`
   node of the `on each (x: T)` binder, and `guard`, a `reference` to the
   predicate node of `g`. A claim written `on each (x: T)` with no `when`
   references the Boolean `value`/`literal` node of `true`: FR-407
   "Activation" reads no `when` as every position, so `on each (x: T)` and
   `on each (x: T) when (true)` are one activation and encode identically.
9. `confidence`: the empty aggregate for a claim that states no confidence
   parameters, or exactly the bindings `alpha`, `beta` and `indifference`, in
   that order whatever order the source writes them in, each an exact value.

The claim's `dependencies` are those FR-322's `dependencies` rule 1 gives its
body: the unique node-id-ascending targets of its `reference`s, so every
predicate, formula and parameter node of bindings 6 to 9 is one. A literal's
`type` is a type annotation and is not listed.

These are not members, because FR-407 places them outside what the claim
states: the evidence kind, the statistical method and every budget, which are
request settings (FR-407 "Obligation identity"). The temporal profile is not a
member either: a package carries one `temporal_profile` selection in its lock
(`lock.profile_selections`, FR-322 "Temporal clock bindings"), every claim of the package is read under it, and so
no two claims of one package can differ in it.

The node's `node_id` path is

`["claim", "probabilistic_claim", N, "subject", {subject}, "scheduler", S,
"reward", {r}…, "distributions", {p}…, "delays", {d}…, "form", F, "over", O,
"activation", A, "confidence", C]`

where `N` is the claim's declared name, `{x}` is the operand segment naming
node `x`, and the operands are in the body's member order. `S` is
`{workload}` for `under W`. For every scheduler, `S` is the string `"every"`
followed, per fairness member in body order, by four items: its `kind`
string, its `granularity` string, its `{declaring}` operand and its
`operation` string. `F`, `O`, `A` and `C` are the **spellings** of bindings
6 to 9: an aggregate spells as, for each of its bindings in order, the
binding's name followed by the spelling of its value; a `text` literal spells
as its text; a `reference` as the operand segment naming its target; an exact
value as its `value_kind`, then, for a `quantity`, the operand segment naming
its unit node, then its canonical `value` text; the empty aggregate spells as
nothing. FR-640-AC-10 spells `P95`'s.

Two different bodies never give one path. The binding-name strings separate
the variable-length groups of bindings 3 to 5. In bindings 6 to 9, the
`kind` of each aggregate fixes the names of the bindings that follow it, an
exact value's `value_kind` fixes how many items it spells, an exact value's
`type` is fixed by its position (rule 7 refuses any other), so a spelling
that leaves out an `integer` or `rational` literal's `type` loses nothing,
and a reference
spells as an operand segment, never as a string. So the only spellings of
varying length are the empty `over` and `confidence`, and each of them is
followed by a fixed binding name or the path's end. The node's `ordinal`
ranks the claims of one package and one path by their written content (FSL
FR-498), so claims that share a path take consecutive ordinals.

The claim's identity therefore covers every node it reads and everything it
states. Each node it reads holds its content in its own id: a workload its
sorted weights, a random parameter its support, a delay its family and
parameters, and a reward its content operand ("Reward identity"). Each
predicate and formula node holds its content in its own id too, because
FR-322 keys an expression or `temporal`/`formula` node by its content. A
claim's `node_id` changes whenever:

- a node it reads changes;
- its subject, scheduler or name changes;
- any member that FR-407 "Obligation identity" names changes: its form; its
  event or measure (a predicate, a formula and its intervals, a horizon, a
  window, a weighting or the reward read); a threshold, its direction or a
  quantile level; its `over` parameter or activation; or a confidence
  parameter, stated or removed.

A claim's body and path stay unchanged when only these change, and so does
its `node_id` when no other claim shares its path; among claims that share a
path, such a change alters the claim's written content and can re-rank its
ordinal (FSL FR-498):

- the order in which a workload's weight lines, a fairness set's
  constraints, the confidence parameters or the unit's declarations are
  written;
- whitespace and comments;
- the decimal or rational spelling of a dimensionless exact value;
- the unit a quantity threshold, horizon or window is written in;
- the evidence kind, the method and the budgets of a request.

Equal `node_id`s therefore mean one obligation of FR-407. The node identity
is finer than the obligation identity in two respects only, both spellings of
names the source writes: the claim's declared name, and the names of the
`over` and `on each` binders, which FR-322 keys a `value`/`parameter` node
by (its `name` and `level`), as FR-370 and FR-475 do for a clause's or a
state-graph claim's parameter. FSL identifiers are case-sensitive and never
case-folded, so a name that differs only in letter case is another name.
Each of these splits one obligation into two identities. None merges two
obligations.

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
  node, every `probabilistic` node its body references, and every predicate,
  `temporal`/`formula`, `value`/`parameter` and type node its bindings 6 to 9
  reference or type a literal at, with the body and path of "Probabilistic
  claim node". Its scheduler `under W` SHALL resolve to
  the `workload` node whose `model` belongs to the claim's subject model and
  whose `name` is `W`, held in its `scheduler` binding and as an operand of
  its path. Because the workload node is part of the claim's identity, it is
  part of the claim's obligation identity (FR-407). The producer SHALL write
  the claim's form, event or measure, thresholds, `over` parameter,
  activation and stated confidence parameters as bindings 6 to 9, each from
  the value the frontend checked (FR-407), with every exact value in its
  canonical spelling and every quantity in its position's unit. The producer
  SHALL NOT hold what a claim reads or states in any other table or sidecar
  row.

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

6. a `probabilistic_claim` node whose body is not exactly the nine bindings
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
   for a body that is not the nine bindings). As for every derived node, the
   reader does not compare the claim's path with its body (FR-322
   "Structural node identity").
7. for the same node, right after rule 6 and before the next claim node, the
   checks below in this order, each over the bindings 6 to 9 in body order,
   a binding before the bindings nested in it, refusing the first failure:
   1. **Shape.** A `form`, `event`, `measure`, `weighting`, `target`,
      `bound`, `activation` or non-empty `confidence` aggregate whose `kind`
      (for `bound` and `confidence`, whose fixed bindings) is not one this
      requirement lists for its position, or whose bindings are not exactly
      that kind's, in order; a quantile `measure` that is a `fraction`, or a
      `mean_of_fraction` `measure` that is not one; a `direction` other than
      `at_least` or `at_most`: `invalid-value` at that aggregate, or at the
      `kind` or `direction` literal when that literal is the failure.
   2. **Time.** An `elapsed` measure or an `expected_elapsed` form over a
      subject that is not timed (no `timed_model` root of either source,
      "Probabilistic claim node"), or a `delay` weighting over a subject whose
      model has no `dense` `timed_model` root (the root rule 5 names):
      `invalid-value` at that measure's, form's or weighting's `kind`
      (FR-407 "Bounded measures" and "Forms").
   3. **Exact values.** An exact value of another `value_kind` than its
      position's, or typed at another node than the one its position names:
      for a `rational` the builtin `scalar_type`/`rational` node, for an
      `integer` the builtin Integer node, and for a `quantity` the reward's
      unit node or the unit node of the subject's time unit (the source unit
      of its `timed_model` root, `dense` or `tick`); a `within` or `window`
      is an `integer` over an untimed subject and a time `quantity` over a
      timed one: `invalid-value` at the literal. A `value` in a non-canonical
      spelling (`2/4`, `+1`, `05`) has already refused `invalid-value` at the
      literal's `value` by FR-322 "Scalar literal values".
   4. **Ranges.** A `rational` `bound` value of `probability`,
      `mean_of_fraction`, `long_run_fraction` or `reach`, a `level`, or a
      `fraction` comparison's `bound` value that is not strictly between 0
      and 1 (FR-407 `qualitative-threshold` and
      `fraction-bound-out-of-range`); a `steps` `bound`, a `within` or a
      `window` that is negative; an `alpha` or `beta` not strictly between 0
      and 1, an `indifference` not above 0, or, for a form whose `bound` is a
      probability (`probability`, `mean_of_fraction`, `long_run_fraction`,
      `reach`) or a quantile (whose probability is its `level`), a
      probability `θ` with `θ − ι <= 0` or `θ + ι >= 1` (FR-407 "Confidence
      parameters"): `invalid-value` at the literal. An `expected_reward` or
      `expected_elapsed` form has no probability, so only the bounds of each
      parameter on its own apply to its confidence.
   5. **Targets.** A `from`, `until`, `predicate`, `left`, `right` or `guard`
      whose target is neither an `expression` node nor a `value`/`literal`
      node, or whose `semantic_type` is not the `scalar_type`/`boolean` node,
      or a `formula` whose target is not a `temporal`/`formula` node:
      `ill_typed`/`operator-ineligible` at the reference, as FR-475 refuses a
      predicate. An `over` or `trigger` whose target is not a
      `value`/`parameter` node: `invalid_model_binding`/`malformed-declaration`
      at the reference, as FR-475 and FR-370 refuse an `over`. A target that
      is a `temporal`/`formula` node anywhere but `form`'s `event` `formula`,
      or a `temporal`/`fairness` node anywhere, has already refused
      `ill_typed`/`operator-ineligible` at the claim node by FR-370
      "Placement and profile fit", before this reader runs.
   6. **Reward.** A `reward` binding that is empty when the form reads a
      reward (an `accumulate` measure anywhere in it, a `reward` weighting or
      an `expected_reward` form), or non-empty when it does not:
      `invalid-value` at `reward`. Rule 6 has already checked which reward
      nodes a non-empty binding holds.

   Each failure is `invalid_package` with the cause given, unless a code is
   given. The reader checks the form of each target node only: that a
   formula's operators are admitted by the package's temporal profile, that
   an event has a finite window and that `T` is an object type some
   population admits are the frontend's checks (FR-407), as the sign of a
   reward is.

The reader SHALL charge one validation visit of the caller's reader budget per
`probabilistic` node and per support, weight or delay member, and per
`probabilistic_claim` node and per `reward`, `distributions`, `delays` and
fairness member, and per binding of `form`, `over`, `activation` and
`confidence` and per binding nested in them, a binding before the bindings
nested in it, before inspecting it, and on exhaustion return its `incomplete` result at the first uncharged
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
parameter nodes and its delay nodes, and its form with its event or measure,
predicate and formula nodes, exact thresholds, level, horizon or window and
weighting, its `over` parameter node, its activation and its confidence
parameters as exact rationals, read from the claim's body and keyed by the
claim's node identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-640-AC-1 | A package with FSL ADR-024 §7.2's `Service` (two random parameters on `Server::attempt`, the reward `duration` and the workload `Steady`) is emitted with two `random_parameter` nodes, one `reward` node with its clause function and one `workload` node, each with one `declaration` occurrence; the reader admits it and its typed read gives the supports `1`, `3`, `12` with probabilities `9/10`, `7/100`, `3/100`, `duration` typed `Millis`, and weight 1 for each operation of `Steady`. | Test (TC-1563) |
| FR-640-AC-2 | A `time dense` model with `delay Call::reply ~ discrete { 1 ms: 90, 3 ms: 10 };` is emitted with one `delay_distribution` node of family `discrete` whose delays are `1` and `3` in the model's unit `ms` with weights 90 and 10, and the reader admits it; `uniform`, `uniform[1 ms, 4 ms]` and `exponential(rational(1, 2))` are emitted with families `uniform`, `uniform_interval` and `exponential` and admitted. | Test (TC-1563) |
| FR-640-AC-3 | Writing `Steady`'s weights in another order leaves the `workload` node and every claim node `under Steady` byte-identical. A workload with weight 0 on one operation is emitted and admitted. Over FR-640-AC-8's unit, each change below changes exactly the stated `node_id`s and `probabilistic_claims` rows and no other claim's: one weight of `Steady` changes `Steady`'s node and `P95`, `Avail` and `Cost`, and not `Fair`; one support value, and separately one support weight, of `d` changes that `random_parameter` node and all four claims; `duration` on `Server::request` rewritten from `1 ms` to `2 ms` changes that `reward` node and `P95` only; `cost` rewritten from `1` to `2` changes the `cost` node and `Cost` only; `Fair`'s constraint rewritten `fair strong Server::reset` changes `Fair` only, and rewritten `fair weak each Server::reset` changes `Fair` only. Writing the four claims, the rewards and `Steady`'s weight lines in another order changes no byte. For a unit with rewards `cost` and `load` on `Server::attempt` and `stable` on `Server::reset`, a random parameter, a workload and a claim, the declaration orders [`stable`, workload, `load`, `cost`], [`load`, `stable`, `cost`, workload] and [workload, `load`, `stable`, `cost`] emit byte-identical packages, the reward functions ranked `cost`, `load`, `stable` by (operation key, name); changing `load`'s expression from `d` to `d + 1` changes the `load` node's id (its content operand) and the ids of the claims that read `load`, keeps the id of `load`'s function node, and changes no other node id except the new expression nodes of that function's body. | Test (TC-1563) |
| FR-640-AC-7 | In `Service`, the `duration` reward's function carries `clause` `"reward"`; the reader admits the package, gives `Server::attempt` no precondition read from that function, and keeps its `operation_bodies` row `no_body` (FR-322-AC-471). The reward rewritten to name a function whose `clause` is `"precondition"`, and one owned by another operation member `Server/reset`, each refuse `invalid_package`/`invalid-value` at the `reward` node's `/body/expression` member (rule 4). FR-640-AC-6's unit with a reward `cost` on `Other::poke` returning `k` emits the function with the unit's `SourceOwner`, `declaration` `["Other", "poke", "reward", "cost"]`, ordinal 0 and `clause` `"reward"`, and the reader admits it; the same function with `declaration` `["Other", "poke", "reward", "load"]` refuses rule 4 at `/body/expression`. | Test (TC-1563) |
| FR-640-AC-8 | `Service` with `reward duration on Server::request = 1 ms;` and `reward cost on Server::reset = 1;` added, and four claims: `P95` (FSL ADR-024 §7.2, `quantile 0.95 of (accumulate duration from holds(s.phase = Busy) until holds(s.phase = Done) within 10) <= 5 ms` under `Steady`, `on origin`, `with alpha 0.01, beta 0.01, indifference 0.01`), `Avail` (`probability >= 0.5 [always[0,2] holds(true)]` under `Steady`), `Fair` (the same event `under every scheduler fair weak Server::reset`) and `Cost` (`expected accumulate cost until holds(s.phase = Done) <= 3` under `Steady`). Each claim node has `node_tag` `claim`, no `declaration`, one `claim` occurrence and the nine bindings in order, with `subject` the `{package: "example/service", path: ["model", "model_root"], ordinal: 0}` node. `P95`: `scheduler` `Steady`'s node, `reward` the two `duration` nodes (on `Server::attempt` and `Server::request`) in ascending node-id order, `distributions` the two `Server::attempt` nodes, `delays` empty. `Avail`: `reward` empty. `Fair`: `scheduler` an aggregate whose `fairness` has one member (`kind` `weak`, `granularity` `whole`, `Server`'s node, `reset`), `reward` empty. `Cost`: `reward` the one `cost` node. Each path is the "Probabilistic claim node" path of its body; `Fair`'s is `["claim", "probabilistic_claim", "Fair", "subject", {root}, "scheduler", "every", "weak", "whole", {Server}, "reset", "reward", "distributions", {d}, {outcome}, "delays", "form", "kind", "probability", "event", "kind", "formula", "formula", {f}, "bound", "direction", "at_least", "value", "rational", "1/2", "over", "activation", "kind", "origin", "confidence"]` with `{d}` and `{outcome}` in ascending node-id order and `{f}` the `temporal`/`formula` node of `always[0,2] holds(true)`, which `Avail`'s `event` references too. The reader admits the package and its typed read gives each claim's scheduler, rewards and distributions. With `Fair` written `under every scheduler fair weak whole Server::reset fair weak each Server::reset`, its `fairness` has two members that differ only in `granularity`, `each` before `whole`, and its path holds two four-item groups `"weak", "each", {Server}, "reset", "weak", "whole", {Server}, "reset"`; the reader admits it. | Test (TC-1563) |
| FR-640-AC-9 | FR-640-AC-8's unit is extended with `model Avail = "example/avail"` (FSL ADR-024 §7.3) and `workload Calm on Avail` with weight 1 on each `Avail` operation, so the package holds `Avail`'s `model_root` and `Calm`'s `workload` node, and the reader admits it. Over that package, each mutation refuses `invalid_package`/`invalid-value` at the named member of the claim node: `P95`'s body without `delays` (at `body`); `P95`'s `subject` referencing `Server`'s `object_type` node (at `subject`); `P95`'s `scheduler` referencing the `duration` reward node, and referencing `Calm`'s node, whose `model` belongs to `Avail` and not to `Service` (at `scheduler`); `Fair`'s fairness member with `kind` `medium`, with `granularity` `some`, and with `operation` `nope` (at the member); `Fair`'s fairness set holding one member twice (at the second); `P95`'s `reward` without the `Server::request` `duration` node, with the `cost` node added, and with its two members swapped (at `reward`); `Avail`'s `distributions` without the `outcome` node, and with its two members swapped (at `distributions`); `Avail`'s `delays` holding the `d` node (at `delays`). Rewriting `Avail`'s path to hold `Calm` in place of `Steady` while its body still references `Steady` is admitted, because the reader does not compare a path with a body. Over FR-640-AC-8's package itself, the claims are checked in node-id order `Avail`, `Cost`, `Fair`, `P95`, each node charged before its members in body order, and each binding of `form`, `over`, `activation` and `confidence` charged before the bindings nested in it, so `P95`'s last charged member is its `confidence`'s `indifference`. FR-825's requirement-record step runs after this reader and charges the same budget for the package's `requirement_items` rows and their domains, so the count here is the visits charged up to the end of this reader, every step before it included and the requirement-record step excluded: a reader budget one visit short of that count returns `incomplete` at that `indifference` member. | Test (TC-1563) |
| FR-640-AC-10 | Over FR-640-AC-8's package, `P95`'s bindings 6 to 9 are: `form` with `kind` `quantile`, `level` the `rational` `19/20`, `measure` with `kind` `accumulate`, `from` and `until` the predicate nodes of `s.phase = Busy` and `s.phase = Done`, and `within` the `integer` `10`, and `bound` with `direction` `at_most` and `value` the `quantity` `5` typed at the unit node of `ms`; `over` the empty aggregate; `activation` `kind` `origin`; and `confidence` `alpha`, `beta` and `indifference`, each the `rational` `1/100`. Its path ends `"delays", "form", "kind", "quantile", "level", "rational", "19/20", "measure", "kind", "accumulate", "from", {Busy}, "until", {Done}, "within", "integer", "10", "bound", "direction", "at_most", "value", "quantity", {ms}, "5", "over", "activation", "kind", "origin", "confidence", "alpha", "rational", "1/100", "beta", "rational", "1/100", "indifference", "rational", "1/100"]`. `Avail`'s `form` is `kind` `probability`, `event` `kind` `formula` referencing the `temporal`/`formula` node of `always[0,2] holds(true)`, and `bound` `at_least` the `rational` `1/2`, with `confidence` empty. `Cost`'s `form` is `kind` `expected_reward`, `until` the predicate node of `s.phase = Done` and `bound` `at_most` the `rational` `3` (`cost` is an integer reward). The typed read gives each of these. Each change below, applied alone, changes exactly that claim's `node_id` and `probabilistic_claims` row and no other claim's: `P95`'s threshold `5 ms` to `6 ms`, its `<=` to `>=`, its level `0.95` to `0.9`, its horizon `within 10` to `within 11`, its `from` to `holds(s.phase = Idle)`, its `alpha` `0.01` to `0.02`, its three confidence parameters removed, and `on origin` rewritten `on each (m: Service::Server)`; `Avail`'s `0.5` to `0.6`, its `>=` to `<=`, its event to `always[0,3] holds(true)`, and its form to `long_run_fraction holds(true) >= 0.5`; `Cost`'s `until holds(s.phase = Done)` to `until holds(s.phase = Idle)` and its `3` to `4`. Each spelling below leaves every claim's `node_id` and body unchanged: `P95`'s threshold written `0.005 s`; its level written `rational(19, 20)` and `rational(38, 40)`; its confidence parameters written in the order `beta`, `indifference`, `alpha`; `Avail`'s `0.5` written `rational(1, 2)`, `rational(2, 4)` and `decimal(50, 2)`; and whitespace and a comment inserted in each claim. `on each (m: Service::Server)` and `on each (m: Service::Server) when (true)` give `P95` one `node_id`, its `guard` the `true` literal node. Two claims both named `A` over `Service` under `Steady`, `probability >= 0.99 [always[0,2] holds(true)]` and the same with `0.999`, have paths that differ only in the `bound` `value` segment (`99/100` and `999/1000`) and each take ordinal 0; the same pair written `0.5` and `rational(2, 4)` share one path and take ordinals 0 and 1. | Test (TC-1563) |
| FR-640-AC-11 | Over FR-640-AC-9's package, each mutation of a claim node's bindings 6 to 9 refuses at the member stated (rule 7), and a mutation that also breaks rule 6 refuses at rule 6's member: `P95`'s `form` `kind` `median` (`invalid-value` at that `kind`); `P95`'s `level` and `measure` in swapped order (`invalid-value` at `form`); `P95`'s `measure` `kind` `fraction` with its bindings (`invalid-value` at that `measure`); `P95`'s `direction` `below` (`invalid-value` at `direction`); `P95`'s `bound` `value` the `rational` `5`, and the `quantity` `5` typed at the unit node of `s` (`invalid-value` at the literal); `P95`'s `level` the `rational` `1` (`invalid-value` at the literal); `P95`'s `level` `value` spelled `38/40` (`invalid-value` at the literal's `value`, FR-322 "Scalar literal values"); `P95`'s `within` the `integer` `-1` (`invalid-value` at the literal); `P95`'s `within` the `integer` `10` typed at the `Int[0, 10]` `integer_range` node instead of the builtin Integer node (`invalid-value` at the literal, so `within 10` has one body and one path); `P95`'s `from` referencing `Steady`'s `workload` node (`ill_typed`/`operator-ineligible` at `from`); `P95`'s `confidence` without `indifference` (`invalid-value` at `confidence`), and with `indifference` `1/20`, so that `19/20 + 1/20 = 1` (`invalid-value` at that literal); `Avail`'s `event` `formula` referencing the `true` literal node (`ill_typed`/`operator-ineligible` at `formula`); `Avail`'s `reward` holding the two `duration` nodes, its form reading no reward (`invalid-value` at `reward`); `Cost`'s `reward` empty (`invalid-value` at `reward`); `Avail`'s `activation` `kind` `each` whose `trigger` references the `true` literal node (`invalid_model_binding`/`malformed-declaration` at `trigger`); `Avail`'s `over` referencing `Steady`'s node (`invalid_model_binding`/`malformed-declaration` at `over`); `P95`'s `measure` `kind` `elapsed`, over the untimed `Service` (`invalid-value` at that `kind`); and `P95` with both its body missing `delays` and its `level` the `rational` `1` (at `body`, rule 6). Over FSL FR-233-AC-3's `Rpc` model with `time tick send period 1 ms unit ms` and state predicates `P` and `Q` of it, a claim `expected elapsed until holds(P) <= 3 ms` is admitted with `form` `expected_elapsed` and `bound` the `quantity` `3` typed at the unit node of `ms`, and `quantile 0.5 of (elapsed from holds(P) until holds(Q) within 5 ms) <= 3 ms` is admitted with `within` the `quantity` `5` at `ms`, because a tick source is a model-time binding; over that model `long_run_fraction holds(P) weighted by delay >= 0.5` refuses `invalid-value` at the weighting's `kind`, because a tick model has no `dense` root; over the untimed `Service`, `expected elapsed until holds(s.phase = Done) <= 3 ms` refuses `invalid-value` at the form's `kind`. In a package of FR-640-AC-8's unit with only `P95`, its path's `"5"` rewritten `"6"` while its body is unchanged is admitted, because the reader does not compare a path with a body. | Test (TC-1563) |
| FR-640-AC-4 | Reader mutations each refuse `invalid_package` at the failing member: a support with a repeated value, a support weight 0, a workload missing an operation, a workload with a negative weight, workload members out of order, a reward whose function reads a state field, a reward typed `Boolean`, a delay on an untimed model and a `discrete` delay with weight 0 (`invalid-value`); a second `random_parameter` of one parameter, a second `workload` of one name, a second `reward` of one name on one operation and a second `delay_distribution` of one operation (`conflicting-definition` at the later node). A reader budget one visit short of the package's count returns `incomplete` at the first uncharged member. | Test (TC-1563) |
| FR-640-AC-5 | A claim `under Steady` over `Service` resolves its scheduler to `Steady`'s `workload` node, and a provider run over the emitted package gives the same step probabilities as FSL FR-187-AC-1. | Test (TC-1563) |
| FR-640-AC-6 | A package of a unit that declares `object O { d: Clock; operation step() modifies self.d; }`, `object Other { n: Integer; operation poke(k: Int[1, 3]) modifies self.n; }` and `time O dense unit ms;`, with `delay O::step ~ uniform;`, `delay Other::poke ~ discrete { 1 ms: 9, 3 ms: 1 };`, `random Other::poke(k) ~ { 1: 1, 3: 3 };` and `workload W on O { weight O::step = 1; }`, is emitted with operation references whose `declaring` is the source node of `O` or `Other` and a `workload` node whose `model` is the source node of `O`; the strict read and the identity-row join admit it and the typed read gives the delays, the support with probabilities `1/4` and `3/4`, and `W` with weight 1 on `O::step`, its weight rows joined to the `operations` entries of `O`'s identity row. Each refuses `invalid_package`/`invalid-value` at the member: at the identity-row join, an `operation` `nudge` that no `operations` entry of `Other`'s row carries a `parameter` `j` that no `parameters` entry of `Other::poke` carries, and a `W` whose weights omit `O::step` or name `O::nudge`; at the strict read, a `declaring` that is a source-declared `relation`/`population` node, and a delay on a source node of a unit whose source model has no `dense` time member. | Test (TC-2033) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the graph, node identity,
  owner joins and the reader budget these nodes use.
- [FR-407](../../functional/analysis/FR-407-state-probabilistic-claims.md)
  "Obligation identity" names what the claim's identity covers; bindings 6
  to 9 of "Probabilistic claim node" carry each member it names.
- [FR-370](FR-370-checked-package-temporal-clause-body.md) builds an event's
  `temporal`/`formula` node and [FR-475](FR-475-checked-package-state-graph-claim-body.md)
  defines a predicate node.
- [FR-405](../../functional/analysis/FR-405-declare-probabilistic-models.md)
  admits the declarations; [FR-420](../../functional/analysis/FR-420-sample-stochastic-timed-models.md)
  defines the delay families.
- [FR-205](../foundation/FR-205-exact-rational-value.md) types weights and
  rates; [FR-142](../../functional/type-model/FR-142-evaluate-quantities-and-units.md) types
  quantities.
