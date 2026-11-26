---
id: FR-712
title: "Analyze state-graph claims over a timed model subject"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-390
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-391
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-393
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-415
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-418
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-852
    type: traces_to
---
# FR-712: Analyze state-graph claims over a timed model subject

## Description

When a `possible`, `always possible` or `unique path` clause is checked over
a timed model subject (FR-415), the analyzer SHALL evaluate it over the
subject's timed state graph, defined below, and SHALL NOT evaluate it over
the subject's untimed graph. FR-390 defines the forms over an untimed
subject; this requirement gives their meaning when the subject has clocks,
clock guards and time invariants.

### Decision: nodes are timed states, paths are discrete-step sequences

A timed subject's behaviour alternates delays, during which clocks advance
and every time invariant must keep holding, with discrete steps, whose clock
guards must hold. Dropping the clocks would count as reachable a node that
can only be reached by waiting past a deadline, so the graph keeps them: a
node is a timed state, a discrete state with a clock valuation. State-graph
predicates read model state, never clocks (FR-390), so they are evaluated on
a node's discrete state. A path is the sequence of discrete steps it takes;
two runs that take the same steps with different delays are one path, since
a delay is not a choice of the model's operations, and `unique path` asks
how many ways the operations lead from `X` to `Y`.

## Inputs

The checked clause, its form and predicates, its `over` binding, and the
timed model subject: initial states with their clock valuations, universes,
proof bounds, clock guards and time invariants.

## Outputs

One result per claim instance, settled by FR-391, with its FR-393 evidence in
timed form.

## Behavior

- **Timed state graph.** The nodes SHALL be the timed states reachable from an
  initial state of the subject. A delay edge `(s, v) -d-> (s, v + d)`
  exists for a non-negative exact rational `d` when every time invariant of
  `s` holds at `v + t` for every `t` in `[0, d]`; a step edge
  `(s, v) -t-> (s', v')` exists for each discrete step `t` FR-390 gives over
  `s` whose clock guards hold at `v`, `v'` being `v` with the step's clock
  resets applied and every time invariant of `s'` holding at `v'`.
- **Forms.** With `E(P)` the nodes from which some finite path of delay and
  step edges reaches a node whose discrete state satisfies `P`:
  `possible` holds when every initial node is in `E(P)`; `always possible`
  holds when every reachable node whose discrete state satisfies `Q` is in
  `E(P)`; `unique path` holds when, from every reachable node whose discrete
  state satisfies `X`, exactly one sequence of step edges, each preceded by
  some admissible delay, reaches a node satisfying `Y`, and a sequence that
  can repeat a cycle counts as more than one.
- **Symbolic decision.** A conforming analyzer SHALL decide these over zones
  (FR-416): `E(P)` as the least fixpoint of timed and discrete predecessors of
  the `P` zones, and the forward reachable zones by FR-416's zone search,
  with a result equal to the definition above over the concrete timed states.
  A zone holds many timed states, and an edge of a zone graph says only that
  some of them can take its step, so neither a whole zone nor a zone-graph
  edge stands for the timed states in it.
- **Definedness scope.** On the timed zone route, the analyzer SHALL apply
  FR-391's undefined-evaluation rule to claim predicates and to the subject's
  guard enabled-tested at an explored state, `terminal when` predicate at an
  explored state with no enabled step, and selected state constraint.
- **Undefined guard.** The timed zone route SHALL treat a guard that evaluates
  undefined at an explored state as undefined evaluation, not as a disabled
  step (FR-391).
- **Undefined terminal declaration.** The timed zone route SHALL classify an
  explored terminal state using FR-366; an undefined `terminal when`
  predicate is not a non-terminal classification.
- **Refuting-evidence order.** The timed zone route SHALL settle refuting
  evidence in FR-391's canonical discovery order, including FR-391's
  precedence for undefined evidence at the same node.
- **Evidence.** The analyzer SHALL concretize a witness or counterexample
  path with exact rational delays chosen by FR-416's canonical rule. The
  analyzer SHALL carry the path in FR-393 form with each step's delay before
  it. The replay component SHALL replay it with no provider present. A
  `unique path` refutation carries the two step sequences, or the cycle and
  the node it returns to, or, when a reachable node whose discrete state
  satisfies `X` has no step sequence to a `Y` node, FR-393's `trap` arm, whose
  `stem` ends at that node (FR-391 "Trap").
  A `ModelPath` records a delay only ahead of a step, so a `trap` `stem` SHALL
  end at a trapped timed state. The analyzer SHALL search, in FR-391's
  canonical discovery order, the zone-graph nodes holding trapped timed states
  where the form requires reaching the target, and each convex piece of a
  node's trapped timed states, for a piece that the node's canonical path can
  end in. The first piece found is the trap and decides the instance. Where
  there is none, `unique path` SHALL still search for two step sequences. With
  neither, the analyzer SHALL settle the instance `inconclusive`,
  `lasso-not-concretized`, never `refuted`.
- **Proof basis and certificate.** A zone proof settles with basis
  `zone-certified` and an FR-418 certificate covering the reachable zones.
  For `always possible` the certificate also carries a `state_graph` part
  `{form, ef_nodes}`: `ef_nodes` lists, in ascending order, the positions in
  `nodes` of the nodes whose whole zone lies in `E(P)`; a node only part of
  whose zone lies in `E(P)` is not listed. The checker accepts it only after
  recomputing `E(P)` over the timed states itself, as the least fixpoint of
  timed and discrete predecessors over its own exploration, finding the
  listing equal to the one that follows from it, and finding every reachable
  timed state whose discrete state satisfies `Q` in `E(P)`. A `possible` proof by digitization of an
  all-closed subject settles with basis `exhaustive` (FR-416).
- **A subject with no time-divergent behaviour.** A state-graph item reads
  finite paths of delay and step edges and quantifies over no infinite
  behaviour, so it needs no time-divergence witness (FR-418) and never settles
  `no-admitted-behaviour` (FR-416 "Outcomes decided before a run is read"). Over a subject
  whose every infinite path is zero-delay (every state's invariant forbids a
  positive delay), the item is decided on zones exactly as over any other
  timed subject: a `possible` claim holds with a witness path or does not, an
  `always possible` claim settles with its `state_graph` part, and the proof
  certificate carries no `admitted` member (FR-418-AC-16). Candidate
  selection SHALL NOT refuse such a request as inconsistent because no
  divergence witness exists.
- **Routing.** A timed state-graph item is routed by its `options.method`,
  as a timed temporal item is (FR-416): without `method`, or with `zones`,
  only a backend that decides the subject on zones and lists the form in
  `state_graph_forms` matches it; with `digital-clocks`, only a backend
  that publishes that method and lists the form matches it, and decides the
  closed subject's integer-clock reading for a `possible` item; a provider
  may decline the other two forms (FR-416). A backend that decides only
  untimed graphs, and offers neither method, never matches. One item reaches
  one backend.

## Acceptance Criteria

The vector is TC-369's `Rpc` model with `T = 4 ms`.

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-712-AC-1 | `possible` of `phase = Replied` holds with a witness of `send` then `reply` whose delay before `reply` is an exact rational in `[1, 3]` and replays; `possible` of `phase = TimedOut` does not hold, because every run takes `reply` by `x = 3` and `timeout` needs `x >= 4` while `phase` is `Waiting`. | Test (TC-1742) |
| FR-712-AC-2 | `always possible` of `phase = Idle` holds, and its proof's certificate carries the `state_graph` part with the `E(P)` zones in `ef_nodes`; with `reset` removed it is refuted by a path ending at `phase` `Replied`. | Test (TC-1742) |
| FR-712-AC-3 | `unique path` from `phase = Idle` to `phase = Replied` is refuted with the cycle through `reset`; without `reset` it holds, the runs with different delays before `reply` counting as one path. | Test (TC-1742) |
| FR-712-AC-4 | A backend that lists the three forms but offers neither timed method is not a candidate for these items, and an item naming it settles `unsupported` `unsupported-requested-capability`; with `options.method` `digital-clocks`, AC-1's items settle the same verdicts on the backend that publishes that method, the proof with basis `exhaustive` (FR-416). | Test (TC-1742) |
| FR-712-AC-5 | Over the Zeno-only variant of `Rpc` (invariant `x <= 0 ms` in every state), `possible` of `phase = Replied` settles `proved` with basis `zone-certified` and a zone certificate carrying no `admitted` member, and the item is not refused `invalid_capability`/`inconsistent-candidates` or any refusal that names a missing divergence witness; `possible` of `phase = TimedOut` settles `refuted` with no `admitted` requested; supplying an `admitted` member to either certificate settles `inconclusive`, `certificate-rejected`, `malformed` (FR-418-AC-16). | Test (TC-1742) |
| FR-712-AC-6 | On the `zones` route with the timed overlay of TC-1742 step 6, TC-349 SG-06 has a trap at depth 1 and undefined predicate at depth 3 and settles from the earlier trap as `refuted`, `closed-scope`; SG-07 has undefined predicate at depth 1 and trap at depth 2 and settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation`. | Test (TC-1742) |
| FR-712-AC-7 | On the `zones` route with the timed overlay of TC-1742 step 6, TC-349 SG-09's `audit(a)` guard evaluates `lookup` of absent `rz` at the initial state; `possible` settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, with an FR-393 `undefined` arm whose `where` names the timed state key, guard locus and FR-181 identity of `audit(a)`, and whose cause is `absent-key`; the guard is not treated as disabled. | Test (TC-1742) |
| FR-712-AC-8 | On the `zones` route with the timed overlay of TC-1742 step 6, TC-349 SG-11's `Lost` terminal state evaluates its `terminal when` predicate by `lookup` of an absent key; `possible` settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, with an FR-393 `undefined` arm whose `where` names the timed state key and terminal-predicate locus, with no predicate or transition member, and whose cause is `absent-key`. | Test (TC-1742) |
| FR-712-AC-9 | On the `zones` route with the timed overlay of TC-1742 step 6, TC-349 SG-12's selected state constraint evaluates `lookup` of absent `rz` at the initial state; `possible` settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, with an FR-393 `undefined` arm whose `where` names the timed state key and constraint locus, with no predicate or transition member, and whose cause is `absent-key`. | Test (TC-1742) |
| FR-712-AC-10 | Over a subject with dense clocks `x` and `y` in which `first` runs by `y = 1 ms` and resets `x`, and from `phase` 1, bounded by `y <= 2 ms`, `zero` needs `y >= 2 ms` and `x >= 2 ms` and `one` needs `y >= 2 ms` and `x <= 1 ms`, on the `zones` route `always possible` of `phase >= 2` and `unique path` from `phase = 1` to `phase >= 2` each settle `refuted`: with `first` strictly between 0 and 1 ms neither step is ever taken. With `one`'s guard `y >= 2 ms and x < 2 ms` both settle `proved`. With `one` never enabled, the checker rejects an `always possible` certificate whose `ef_nodes` lists the zones of `phase` 0, 1 and 2. | Test (TC-1742) |
| FR-712-AC-11 | Over a subject with a dense clock `x` in which `first` takes `phase` 0 to 1 and resets `x`, `escape` takes `phase` 1 to 2 with guard `x <= 0 ms`, and `phase` 1 is bounded by `x <= 2 ms`, `always possible` of `phase = 2` from `phase = 1` and `unique path` from `phase = 1` to `phase = 2` settle `inconclusive`, `lasso-not-concretized`, with no counterexample. With a further step from `phase` 1 to a `phase` 3 with guard `x > 1 ms`, and `phase = 3` added to each claim's source states, both settle `refuted` with `trap` evidence whose `stem` ends in `phase` 3. With instead a further step from `phase` 0 to 2, `unique path` from `phase` 0 or 1 settles `refuted` with a path pair. | Test (TC-1742) |

## Dependencies

- FR-390 and FR-391 own the forms and their settlement; FR-393 owns the
  evidence.
- FR-415 and FR-416 own timed models, zones and concretization; FR-418 owns
  the zone certificate.
