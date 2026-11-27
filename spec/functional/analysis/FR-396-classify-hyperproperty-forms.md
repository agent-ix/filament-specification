---
id: FR-396
title: "Classify hyperproperty and step-relation forms"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-395
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-397
    type: references
  - target: ix://agent-ix/filament-specification/FR-398
    type: references
  - target: ix://agent-ix/filament-specification/FR-390
    type: references
  - target: ix://agent-ix/filament-specification/FR-391
    type: references
  - target: ix://agent-ix/filament-specification/FR-290
    type: references
  - target: ix://agent-ix/filament-specification/FR-363
    type: references
  - target: ix://agent-ix/filament-specification/FR-364
    type: references
  - target: ix://agent-ix/filament-specification/FR-361
    type: references
  - target: ix://agent-ix/filament-specification/FR-399
    type: references
  - target: ix://agent-ix/filament-specification/FR-354
    type: references
  - target: ix://agent-ix/filament-specification/FR-492
    type: references
  - target: ix://agent-ix/filament-specification/FR-393
    type: references
  - target: ix://agent-ix/filament-specification/FR-394
    type: references
  - target: ix://agent-ix/filament-specification/FR-400
    type: references
  - target: ix://agent-ix/filament-specification/FR-401
    type: references
  - target: ix://agent-ix/filament-specification/FR-243
    type: references
---

# FR-396: Classify hyperproperty and step-relation forms

## Description

When a `hyper` clause over `behaviours` or a `relation` clause over model
operations is admitted, the checker SHALL classify it into exactly one form
below from its quantifier prefix, body, fairness sets and alignment, SHALL
record the form beside its requested item, and SHALL route the item by its
form.

## Inputs

The checked clause from [FR-395](FR-395-analyze-hyperproperties-over-behaviours.md)
or [FR-397](FR-397-check-step-relations-over-model-operations.md).

## Outputs

The clause's form and its requested item or items.

## Forms

The safety fragment is the safety fragment of the infinite-trace profile,
interval operators included.

| Form | Shape | Settled by |
| --- | --- | --- |
| HP-1 step relation | a `relation` clause whose execution bindings name model operations | FR-397 |
| HP-2 universal | `hyper` over `behaviours`, every quantifier `forall` (at least one), lockstep, any infinite-trace body, any fairness sets | exhaustive exploration; bounded refutation; induction for a safety body |
| HP-3 `∀∃` safety | prefix of `n >= 0` `forall` then `m >= 1` `exists`, `n + m >= 2`, lockstep, body in the safety fragment, every existential variable with the empty fairness set | exhaustive exploration |
| HP-4 other | an `exists` before a `forall`; an existential variable beside another variable whose body is outside the safety fragment or whose fairness set is non-empty; `align skip` with an existential variable or a body outside the safety fragment | none: `unsupported` |
| HP-5 single existential | exactly one quantifier, `exists`, any infinite-trace body, any fairness set | the possibility family (below) |
| HP-6 projection-aligned universal | as HP-2, with `align skip` and a body in the safety fragment | FR-398 |

The admitted fragment is HP-1, HP-2, HP-3, HP-5 and HP-6. When a clause is
HP-4, negotiation SHALL settle its item `unsupported`, warned,
`unsupported_projection`/`unsupported-requested-capability`, and no engine
runs for it.

## Routing

A `hyper` clause, over all of its instances, and the model claim of a
`relation` clause are each one requested item that requires the FR-290 kind
`temporal-satisfaction` with an unbounded extent naming the clause; its form
is recorded beside the item as an obligation form. An HP-5 clause is one such
item too, as an FR-390
state-graph claim is. A candidate's negotiation
arm settles the item `supported` only when it discharges that form. A
`relation` clause's code claim is a separate item of kind
`operation-contract` (FR-397).

## Instances and the item's result

A `hyper` item has one result over the instances FR-395 gives its clause.

- **Instance order.** The keys of one object parameter are in the instance
  order of FR-361, over the populations of its alias's model that overlap its
  type. An instance of a clause with several parameters takes one key for
  each, and instances are ordered by the first declared parameter's key, then
  the next one's: the first parameter varies slowest.
- **Independence.** The analyzer SHALL decide each instance from its own
  findings. It SHALL NOT rank findings of different instances by joint
  position, kind or discovery order.
- **Replay first.** An instance is `refuted` only when replay confirms its
  refutation. An instance whose refutation does not replay takes the result
  its replay settles (`replay-parity`, `replay-refused` or a stopped
  replay), and it SHALL NOT replace or hide a confirmed refutation of another
  instance, earlier or later in the order. The item SHALL NOT settle
  `replay-parity` or `replay-refused` when any instance's refutation is
  confirmed. An engine that returns one outcome over all instances replays
  only the refutation it returns, so when that replay disagrees the item has
  no confirmed refutation and settles `replay-parity`.
- **Refuted.** The item is `refuted` exactly when some instance is refuted,
  even when an instance earlier in the order is not proved. Its refuting
  evidence (a counterexample or graph evidence), basis, cause and settlement
  method are those of the first refuted instance in the instance order. The
  canonical order of refuting evidence of FR-399 chooses the evidence within
  that instance; it does not choose between instances. The analyzer need not
  decide an instance after the first refuted one.
- **Not proved.** With no instance refuted and some instance not proved, the
  item's result is that of the first instance in the order that is not proved,
  with its label, basis, cause and evidence.
- **Proved.** The item is `proved` only when every instance is `proved` and
  every certificate an instance carries was checked. It carries every
  instance's evidence in instance order, each with that instance's own
  certification and basis; for an HP-5 clause that is one `witness` arm
  (FR-393) per instance, and the item's `evidence` member is the FR-393
  `ItemEvidence` array of those arms, one for each instance in the FR-361
  instance order, each naming its own instance. A refuted item carries exactly
  one `GraphEvidence` value as its `counterexample`, that of its first refuted instance, and a not-proved item
  exactly one, that of its first instance that is not proved; neither carries
  the array.
- **Limits and cancellation.** `max_states`, `max_transitions`, the work
  meter, the time budget and cancellation are the item's, shared by all its
  instances: a later instance is decided with what remains of them.
  `max_depth` is each instance's own search horizon, `max_automaton_states`
  bounds one instance's automaton and `max_witness_set` one witness set. An
  instance that is neither refuted nor proved does not end the item: the
  analyzer SHALL decide each later instance with what remains, and a later
  refuted instance gives the item's result. A cancellation ends the run. A
  run cancelled before it refutes an instance settles `cancelled`, whatever an
  earlier instance's result, except that when every initial state of every
  instance already has a witness it settles `inconclusive` with
  `well-definedness-unchecked`, as the FR-391 rows adopted for HP-5 state,
  taking the first instance that is not proved.

A clause with no instance settles as FR-399 states, `inconclusive`,
`no-instance`, and carries no `evidence` array.

### Certification and basis of a proved item

The `certification` of a `proved` item SHALL follow the rule FR-492 states for
`certification` (FR-354), each instance's proof a deciding premise: `trusted`
when any instance is `trusted`, else `uncertified` when any is, else
`certified`.

The item's proof basis with its payload, its FR-243 basis, its certificate and
its settlement method SHALL be those of one instance, chosen by these steps in
turn, each keeping some of the instances the step before it kept:

1. the instances whose certification is the item's;
2. if any has a depth-bounded proof basis, one stating a depth or horizon that
   bounds the behaviours it covers (`bounded-complete{depth}`, or a
   `bounded-solver` horizon), only those, so a depth-bounded basis is
   preferred over an `exhaustive` one;
3. of depth-bounded ones whose bounds are all in one unit (a step depth, or a
   horizon in one time unit), those with the smallest bound; where their units
   differ, all of them;
4. if any is reduced or constrained (proof basis `reduced`, or a run under a
   state constraint), only those;
5. the first in instance order.

An `inductive{depth}` basis is not depth-bounded: its depth is the induction
length, and the proof covers every behaviour. These steps only choose which
instance's basis the item carries; they rank no proof basis in general.

## Single-existential claims

An HP-5 clause is a possibility claim and joins the state-graph family
([FR-390](FR-390-analyze-state-graph-claims.md)) as a form with a temporal
body.

- **Meaning.** It holds when, from every initial state of the subject, some
  fair behaviour satisfies the body.
- **Admission.** A `match` block on an HP-5 clause SHALL be refused as
  `unsupported_construct`/`expression-form`, located at the block, since the
  clause has one trace. Its fairness set is admitted: fairness decides which
  infinite behaviours count as a witness.
- **Verdicts.** The rows of [FR-391](FR-391-settle-state-graph-claims.md)
  apply, read over the product of the subject with the body's automaton and
  the clause's fairness, including FR-391's rule that well-definedness is
  global. **Witnessed:** every initial state has a witness, a fair lasso on
  which the body is true, each replays, and the exploration of the product
  completed with no open node and no undefined evaluation of the body:
  `proved`, `decisive-witness`, with each source `sampled`, `explored` or
  `unrolled`. **Witness found, well-definedness unchecked:** every initial
  state has a witness and the exploration ends with open nodes, or a limit
  stops it, before refuting evidence is found: `inconclusive`, cause
  `well-definedness-unchecked`, with the witnesses. **Trap:** an initial state
  whose part of the product is closed and holds no fair accepting cycle:
  `refuted`, `closed-scope`. **Undefined:** the body evaluates undefined at a
  reachable position: `refuted`, cause `undefined-evaluation`, with the
  `undefined` arm. Refuting evidence is ordered as FR-391 orders it.
  **Otherwise** the "No decisive evidence" row of FR-391. Witness sampling
  ([FR-392](FR-392-sample-possible-witnesses.md)) runs first: a sampled walk
  that revisits a state gives a candidate lasso. The verdict does not depend
  on `witness_samples`.
- **Closed parts.** A product state from which the body's automaton has no
  run lies on no accepting cycle, and no state after it does. One that a run
  limit or the maximum depth left unexpanded SHALL NOT keep its initial
  state's part from being closed for a trap when every contract conjunction of
  its model state has a value; it stays open for a witness. An undefined
  contract conjunction is never read as a disabled step, so such a state does
  not close a part.
- **Evidence.** Each arm names the instance it is for. A witness is the
  `witness` arm of
  [FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md) with one
  model lasso per initial state and its `loop_entry`. A trap is the `trap` arm
  naming the initial state with an empty stem. Each replays by the
  single-existential rows of
  [FR-394](FR-394-replay-state-graph-evidence.md).
- **Basis.** An exhaustive refutation of a possibility claim carries no trace,
  because it shows that no trace exists. Its basis is `closed-scope`, since
  the trap's closure, the whole decision scope, was examined completely, and
  its settlement method is exhaustive exploration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-396-AC-1 | Each clause of the classification vectors is assigned exactly its listed form, and the form is recorded beside its item as an obligation form. | Test (TC-342) |
| FR-396-AC-2 | An `exists` before a `forall`, a `∀∃` clause with a liveness body, a `∀∃` clause whose existential variable has a fairness constraint, and `align skip` with an existential variable each settle `unsupported` with `unsupported_projection`/`unsupported-requested-capability`, and no engine runs. | Test (TC-342) |
| FR-396-AC-3 | Every hyper item and every relation model claim requires `temporal-satisfaction`; a single candidate whose arm does not discharge the form settles `unsupported`. | Test (TC-342) |
| FR-396-AC-4 | An HP-5 claim with a fair lasso witness from every initial state and a completed product exploration with no undefined evaluation settles `proved`, `decisive-witness`, and the witness replays by FR-394; an HP-5 claim whose initial state's closed product part holds no fair accepting cycle settles `refuted`, `closed-scope`, and the trap replays by re-exploration. | Test (TC-342) |
| FR-396-AC-5 | A `match` block on an HP-5 clause refuses as `unsupported_construct`/`expression-form`; a fairness set on it is admitted. | Test (TC-342) |
| FR-396-AC-6 | An HP-5 claim with a witness for every initial state whose product exploration a limit stops before refuting evidence is found settles `inconclusive`, cause `well-definedness-unchecked`; one whose body evaluates undefined at a reachable position settles `refuted`, cause `undefined-evaluation`, also when a witness exists. | Test (TC-342) |
| FR-396-AC-7 | A `hyper` clause with several instances is one requested item with one result. It is `refuted` when any instance is refuted, even after an instance that is not proved, and takes its refuting evidence (counterexample or graph evidence), basis and cause from the first refuted instance in the instance order, the first declared parameter varying slowest; with none refuted it takes the result of the first instance that is not proved; it is `proved` only when every instance is. | Test (TC-342) |
| FR-396-AC-8 | A `proved` item over several instances takes its `certification` by FR-492's rule over its instances, and its proof basis, FR-243 basis, certificate and settlement method from one instance chosen by the five steps of "Certification and basis of a proved item"; it carries every instance's evidence in instance order, for HP-5 its `evidence` member as the FR-393 `ItemEvidence` array of one `witness` arm per instance, each naming its instance (FR-396-AC-12). | Test (TC-342) |
| FR-396-AC-9 | An instance is refuted only once its refutation replays. A refutation that does not replay never replaces or hides a confirmed refutation of another instance; the item settles `replay-parity` or `replay-refused` only when no instance's refutation is confirmed; and an engine returning one outcome over all instances whose refutation fails replay settles `replay-parity`. | Test (TC-342) |
| FR-396-AC-10 | `max_states`, `max_transitions`, the work meter, the time budget and cancellation are shared by the item's instances, while `max_depth` is each instance's own horizon; an undecided, bounded or stopped instance does not hide a later refuted one that can be decided with what remains; and a run cancelled before it refutes an instance settles `cancelled`, or `well-definedness-unchecked` when every initial state of every instance already has a witness. | Test (TC-342) |
| FR-396-AC-11 | A product state with no run of the body's automaton that a limit or the maximum depth left unexpanded does not keep a part from being a trap when every contract conjunction of its model state has a value, and the trap replays; with a contract conjunction undefined there the item does not settle `closed-scope`. | Test (TC-342) |
| FR-396-AC-12 | A `proved` HP-5 item whose clause has instances `p` then `q`, both proved, carries `evidence` as an array of two `witness` arms in that order, the first naming `v = p` and the second `v = q`, and one terminal record. | Test (TC-342) |
| FR-396-AC-13 | An HP-5 item whose instance `p` is proved and whose instance `q` is refuted settles `refuted` with exactly one `counterexample`, the `trap` arm naming `v = q`, and no `evidence` array; with both instances refuted its one `counterexample` is the `trap` arm naming `v = p`. | Test (TC-342) |
| FR-396-AC-14 | An HP-5 clause with zero instances settles `inconclusive`, `no-instance`, never `proved`, and carries no `evidence` array. | Test (TC-342) |

## Dependencies

- [FR-395](FR-395-analyze-hyperproperties-over-behaviours.md) and
  [FR-397](FR-397-check-step-relations-over-model-operations.md) supply the
  checked clauses.
- [FR-390](FR-390-analyze-state-graph-claims.md),
  [FR-391](FR-391-settle-state-graph-claims.md),
  [FR-392](FR-392-sample-possible-witnesses.md) and
  [FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md) settle
  HP-5.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) assigns the
  capability kinds.
- [FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md) and [FR-364](../temporal/FR-364-replay-temporal-counterexamples.md)
  own the safety fragment of the infinite-trace profile and model lassos.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
