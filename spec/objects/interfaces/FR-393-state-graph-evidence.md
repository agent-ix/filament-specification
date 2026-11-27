---
id: FR-393
title: "State-graph evidence wire"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-391
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-394
    type: references
  - target: ix://agent-ix/filament-specification/FR-197
    type: references
  - target: ix://agent-ix/filament-specification/FR-363
    type: references
  - target: ix://agent-ix/filament-specification/FR-047
    type: references
  - target: ix://agent-ix/filament-specification/FR-364
    type: references
  - target: ix://agent-ix/filament-specification/FR-396
    type: references
  - target: ix://agent-ix/filament-specification/FR-400
    type: references
---

# FR-393: State-graph evidence wire

## Description

When a state-graph item settles from evidence, the producer SHALL encode that
evidence as one `GraphEvidence` value below, serialized with RFC 8785 JCS, and
the reader SHALL refuse a value that breaks a rule below before any replay
runs.

## Properties

- **ModelPath** — `{"type":"model-path","initial":"<decimal>","steps":[<step>, ...]}`:
  - `initial` — the index of the path's first state in the subject's initial
    states, in their canonical order, as a decimal string;
  - `steps` — the path's steps in order, possibly none. A step is the model
    trace step of a model-subject counterexample:
    `{"type":"step","transition":<transition>,"post_state":{"key":"<state key>"}}`,
    with `transition` the FR-181 typed canonical form of the step's
    transition identity and `post_state` the post-state's State locus
    `{"key":"<state key>"}` (FR-331), `<state key>` the state key (FR-181)
    in the unpadded base64url spelling of its canonical JCS bytes, with no
    hash. A step of a timed state-graph path
    ([FR-712](../../functional/analysis/FR-712-analyze-state-graph-claims-over-a-timed-model-subject.md))
    also carries `delay`, the FR-181 delay before the step as a
    `{"numerator","denominator"}` exact rational in the model's unit.
- **Instance** — the `over` binding of the instance the evidence is for:
  `{"type":"instance","parameter":"<ident>","key":<encoded>}`, with `key`
  the bound object's universe key in FR-181 typed canonical form, or `null`
  for a clause with no `over` parameter. For a single-existential claim
  (FR-396) with two or more object parameters, `instance` is instead the
  FR-400 `Instance` list, one entry per object parameter in declaration
  order.
- **WitnessSource** — exactly one of
  `{"type":"sampled","seed":"<decimal>","trace_index":"<decimal>"}`,
  `{"type":"explored"}` or `{"type":"unrolled","depth":"<decimal>"}`.
- **GraphEvidence** — exactly one of four arms, each carrying `instance`:
  - `{"type":"graph-evidence","arm":"witness","instance":<Instance>,"paths":[{"path":<ModelPath>,"source":<WitnessSource>}, ...]}`
    — one entry per initial state of the subject, ordered by `initial`, each
    path ending at a node where `P` holds. For a single-existential claim
    with a temporal body ([FR-396](../../functional/analysis/FR-396-classify-hyperproperty-forms.md))
    each entry also carries `"loop_entry":"<decimal>"`, the position at which
    the path's last step closes its loop, so the path is a lasso; no other
    form carries `loop_entry`;
  - `{"type":"graph-evidence","arm":"trap","instance":<Instance>,"stem":<ModelPath>}`
    — `stem` ends at the trap node;
  - `{"type":"graph-evidence","arm":"path-pair","instance":<Instance>,"stem":<ModelPath>,"first":[<step>, ...],"second":[<step>, ...]}`
    — `stem` ends at a node where `X` holds, and `first` and `second` are two
    step sequences from that node, each ending at its first `Y` node;
  - `{"type":"graph-evidence","arm":"undefined","instance":<Instance>,"stem":<ModelPath>,"undefined":<UndefinedEvaluation>}`
    — `stem` ends at the node where a predicate of the claim, or a guard,
    the `terminal when` predicate or a state constraint of the subject,
    evaluates undefined ([FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md));
    for a guard that node is the transition's pre-state. `undefined` carries
    that evaluation.
- **UndefinedEvaluation** — the
  [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  undefined-evaluation envelope,
  `{"type":"undefined-evaluation","where":{...},"cause":<reason>}`, with the
  model-arm keys of FR-364: `position`, the stem's number of steps;
  `state_key`, the state key of the stem's last
  state; and `locus`, the source locus of the expression that has no value.
  For a predicate of the claim, a state-graph `where` also carries
  `predicate`, one of `"P"`, `"Q"`, `"X"` or `"Y"`, naming the predicate that
  has no value. For a guard it carries FR-364's `transition`, the FR-181
  transition identity whose guard it is. For the `terminal when` predicate or
  a state constraint it carries neither, and `locus` names the expression. It
  is the `where` and
  `cause` of the item's
  [FR-363](../../functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md)
  `undefined-evaluation`, so the evidence replays on its own.
- **ItemEvidence** — the `evidence` member of an item's one terminal record
  (FR-390, FR-331), by the item's result:
  - a `proved` item over the instances of a `hyper` clause (FR-396) whose
    proof carries graph evidence, which is a single-existential claim, carries
    `evidence` as a non-empty ordered array of `witness`-arm `GraphEvidence`
    values, one per instance of the clause in the instance order of
    [FR-361](../../functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md),
    each carrying that instance in its own `instance` member and, with it, that
    instance's own paths and sources. A `hyper` clause whose proof carries no
    graph evidence (HP-2, HP-3, HP-6) omits `evidence`;
  - a `refuted` item carries no `evidence` member and exactly one
    `GraphEvidence` value as its `counterexample`, that of the first refuted
    instance in the instance order;
  - an `inconclusive` item with cause `well-definedness-unchecked` carries
    `evidence` as exactly one `witness`-arm value, the one of the first
    instance that is not `proved`; any other `inconclusive` item carries none;
  - an item with zero instances settles `inconclusive`, `no-instance`
    ([FR-399](../../functional/analysis/FR-399-settle-hyperproperty-results.md))
    and carries no `evidence` array.

  The array and the single value are one type, `GraphEvidence`: an element of
  the array is encoded exactly as the single value is, and the array adds only
  the container and its order. A refuted or not-proved item never carries an
  array.

Every decimal is `0` or `[1-9][0-9]*`.

## Envelope and identity

`GraphEvidence` travels in the witness envelope of the replay boundary with
its own model-graph replay-source arm, distinct from the backend-transcript
and input arms of [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md).
The envelope's obligation identity binds the model subject (initial states,
universes and proof bounds) and the checked claim, including its form. A
state-graph item and a single-existential item (FR-396) each have one identity
over all their instances: it binds no instance, and the subject's universe
lists fix them. An `ItemEvidence` array does not change that identity, and it
is one value of the item's one terminal record ([FR-390](../../functional/analysis/FR-390-analyze-state-graph-claims.md)),
never one record for each instance. The evidence carries no canonical state, permutation or reduction:
evidence found by a reduced search is concretised before it is encoded.

## Validation

The reader SHALL refuse each of the following as
`invalid_runtime_input`/`invalid-value`, before replay, naming the offending
member:

- an `arm` outside the four above, a missing member, a duplicate key or a
  non-canonical encoding;
- a `witness` arm whose entries do not number exactly the subject's initial
  states, or whose `initial` values are not `0` to `n - 1` in order;
- an `initial` index outside the subject's initial states;
- a `loop_entry` on an entry of a form that carries none, an absent
  `loop_entry` on an entry of a form that requires one, or a `loop_entry`
  greater than the path's number of steps;
- an `instance` whose parameter or key does not match the clause, or is not
  one of the item's instances (FR-390, FR-396); and an `instance` list where the clause has fewer than two object
  parameters, a single-parameter form where it has two or more, or a list
  that does not hold one entry per object parameter in declaration order;
- an `undefined` arm with no `undefined` member, or whose `cause` is not a
  catalogued undefined reason, or an `undefined` member on any other arm;
- an `undefined` arm whose `where` carries both `predicate` and `transition`,
  or a `transition` that is not a transition of the subject enabled-tested at
  the stem's last state;
- an `ItemEvidence` `evidence` on a `proved` item that is not an array, an
  empty array, an array holding an arm other than `witness`, an array whose
  length is not the item's number of instances, one whose `instance` values
  are not the item's instances in FR-361 instance order (a repeated, missing,
  swapped or unknown instance), and an array on a `refuted` item or on an
  `inconclusive` item;
- a step of a path over a timed subject with no `delay`, or a step of a path
  over an untimed subject that carries one.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-393-AC-1 | Each of the four arms serializes and reads back byte-identically under RFC 8785 JCS with exactly the members above. | Test (TC-335, TC-349) |
| FR-393-AC-2 | A `witness` arm carries one entry per initial state in index order, each with its source; a `sampled` source names its seed and trace index. | Test (TC-331, TC-335) |
| FR-393-AC-3 | An unknown arm, a missing or duplicate member, a witness arm with the wrong number or order of entries, an out-of-range initial index, a misplaced or out-of-range `loop_entry` and a mismatched instance each refuse as `invalid_runtime_input`/`invalid-value` before replay. | Test (TC-335) |
| FR-393-AC-5 | An `undefined` arm carries its `undefined-evaluation` value (state key, locus and catalogued reason, plus the predicate for a claim predicate or the FR-181 transition identity for a guard) and reads back with it; an `undefined` arm without it, with an uncatalogued reason, with both `predicate` and `transition`, with a `transition` not enabled-tested at the stem's last state, or an `undefined` member on another arm refuses before replay. | Test (TC-335, TC-349) |
| FR-393-AC-4 | Two envelopes whose evidence is equal but whose subject or claim differs have different obligation identities. | Test (TC-335) |
| FR-393-AC-6 | The evidence of a single-existential claim with two object parameters carries `instance` as a two-entry list in declaration order and reads back byte-identically; a one-entry list, the single-parameter form, a swapped order and a tuple that is not an instance of the item each refuse `invalid_runtime_input`/`invalid-value` before replay. | Test (TC-335) |
| FR-393-AC-7 | A `proved` single-existential item with two instances `p` then `q` carries `evidence` as an array of exactly two `witness` arms, the first with `instance` `{"type":"instance","parameter":"v","key":<p's key>}` and the second with `<q's key>`, and the array reads back byte-identically; an empty array, a bare object, an array of one, an array in the order `q`, `p`, an array repeating `p`, an array holding a `trap` arm, and an array on a `refuted` item each refuse `invalid_runtime_input`/`invalid-value` before replay. | Test (TC-335, TC-431) |

## Dependencies

- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md) owns
  transition identities, state keys and their typed canonical form.
- [FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md)
  decides which arm settles an item.
- [FR-394](../../functional/analysis/FR-394-replay-state-graph-evidence.md)
  replays each arm.
- [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  owns the replay boundary and its other source arms.
- [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  owns the model-trace step that a `ModelPath` step reuses.

## References

- STD-135 (this requirement); QSL-369 and its ADR-022 (design).
