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
- **Undefined evaluation.** When the analyzer explores a reachable timed
  state, it SHALL apply FR-391's undefined-evaluation rule to the claim
  predicates and the subject's guard enabled-tested there, `terminal when`
  predicate at a state with no enabled step, and selected state constraint.
  When one of those evaluations is undefined, the analyzer SHALL settle each
  affected item `refuted`, `decisive-counterexample`, with cause
  `undefined-evaluation`. When it settles an item this way, the analyzer SHALL
  carry FR-393's undefined-evaluation evidence naming the timed state's state
  key and expression locus, the FR-181 transition identity for a guard, and
  the catalogued undefined-value reason. The analyzer SHALL NOT read an
  undefined guard as a disabled step or an undefined `terminal when` predicate
  as a non-terminal state. This applies to `possible`, `always possible` and
  `unique path`, including an item for which `possible` already has a witness.
- **Evidence.** A witness or counterexample path SHALL be concretized with
  exact rational delays by FR-416's canonical choice, carried in FR-393's
  path form with each step's delay before it, and replay with no provider
  present. A `unique path` refutation carries the two step sequences, or the
  cycle and the node it returns to.
- **Proof basis and certificate.** A zone proof settles with basis
  `zone-certified` and an FR-418 certificate covering the reachable zones.
  For `always possible` the certificate also carries a `state_graph` part
  `{form, ef_nodes}`: `ef_nodes` lists, in ascending order, the positions in
  `nodes` of the zones of `E(P)`, and the checker accepts it only after
  re-deriving one predecessor pass that adds no zone and finding every
  reachable `Q` node among them. A proof by digitization of an all-closed
  subject settles with basis `exhaustive` (FR-416).
- **A subject with no time-divergent behaviour.** A state-graph item reads
  finite paths of delay and step edges and quantifies over no infinite
  behaviour, so it needs no time-divergence witness (FR-418). Over a subject
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
  closed subject's integer-clock reading. A backend that decides only
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
| FR-712-AC-6 | On the timed zone graph, if any reachable state has an undefined claim predicate, enabled-tested guard, terminal predicate or selected state constraint, every affected `possible`, `always possible` and `unique path` item settles `refuted`, `decisive-counterexample`, cause `undefined-evaluation`, with FR-393 evidence whose path reaches that state and whose `where` identifies its state key and expression locus, plus the transition identity for a guard and the catalogued reason; an undefined guard is not treated as disabled, an undefined terminal predicate is not treated as non-terminal, and a `possible` witness does not suppress the result. | Test |
| FR-712-AC-5 | Over the Zeno-only variant of `Rpc` (invariant `x <= 0 ms` in every state), `possible` of `phase = Replied` settles `proved` with basis `zone-certified` and a zone certificate carrying no `admitted` member, and the item is not refused `invalid_capability`/`inconsistent-candidates` or any refusal that names a missing divergence witness; `possible` of `phase = TimedOut` settles `refuted` with no `admitted` requested; supplying an `admitted` member to either certificate settles `inconclusive`, `certificate-rejected`, `malformed` (FR-418-AC-16). | Test (TC-1742) |

## Dependencies

- FR-390 and FR-391 own the forms and their settlement; FR-393 owns the
  evidence.
- FR-415 and FR-416 own timed models, zones and concretization; FR-418 owns
  the zone certificate.
