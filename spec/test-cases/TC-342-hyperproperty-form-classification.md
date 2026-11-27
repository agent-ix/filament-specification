---
id: TC-342
title: "Hyperproperty form classification, unsupported forms and single-existential claims"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-396
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-399
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-290
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-394
    type: verifies
---

# TC-342: Hyperproperty form classification, unsupported forms and single-existential claims

## Description

Classify clauses into HP-1 to HP-6, settle HP-4 unsupported without running an engine, route single-existential clauses to the possibility family, and form one result for a `hyper` clause from its instances.

## Test Procedure

Admit each vector through the real local Rust boundary and record its form, its requested items and kinds, and its result; replay every witness and trap. The vault subject: model `V` with object type `Vault`, fields `h: Int[0, 1]` (secret) and `l: Int[0, 1]` (public output), population `vaults` with universe `{v}`, one operation `step(i: Int[0, 1])` with frame `modifies [l]` and no precondition, and initial snapshots `(h, l) = (0, 0)` and `(1, 0)`. The leaky model's post clause is `self.l = (i + self.h) mod 2`; the secure model's is `self.l = i`.

The counter subject: model `M` with object type `Counter`, field `value: Int[0, 5]`, one population with universe `[p, q]` unless a vector states another, one operation `inc` that raises its receiver's `value` by 1 and is enabled while it is below 3, and one initial state holding both counters. A vector marked "injected" hands the stated per-instance outcomes of a clause with instances `p` then `q` to the item's settlement in place of an engine run.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| HF-01 | `exists trace b of V forall trace a of V` with a safety body | HP-4; `unsupported`, `unsupported_projection`/`unsupported-requested-capability`; no engine runs |
| HF-02 | `forall trace a of V exists trace b of V` with body `always eventually holds(v.l @ a = v.l @ b)` | HP-4; `unsupported` |
| HF-03 | `Opaque` with `b` carrying `fair weak V::Vault::step;` | HP-4; `unsupported` |
| HF-04 | `Opaque` with `align skip { V::Vault::step }` | HP-4; `unsupported` |
| HF-05 | `hyper E using inf over behaviours (v: V::Vault) exists trace b of V { eventually holds(v.l @ b = 1) }` on the secure model | HP-5; `proved`, `decisive-witness`, with a lasso witness per initial state carrying `loop_entry`; replay checks fairness and evaluates the body true |
| HF-06 | `hyper E2 using inf over behaviours (v: V::Vault) exists trace b of V { always holds(v.l @ b = 1) }` on a model where `l` starts at 0 and `step` is disabled | HP-5; `refuted`, `closed-scope`, with a trap at the initial state and an empty stem; replay re-explores the product part |
| HF-07 | HF-05 with a `match` block | `unsupported_construct`/`expression-form` at the block |
| HF-08 | HB-01, `Opaque`, HB-11 and an `align skip` universal safety clause | HP-2, HP-3, HP-1 and HP-6, each item of kind `temporal-satisfaction` with its form recorded |
| HF-09 | HB-01 against a single registered candidate whose arm discharges HP-1 only | `unsupported`, warned, `unsupported_projection`/`unsupported-requested-capability` naming the HP-2 form |
| HF-10 | HF-05 with `max_states` set so the product exploration stops after a witness lasso was found for every initial state | `inconclusive`, cause `well-definedness-unchecked`, with the witnesses; never `proved` |
| HF-11 | HF-05 with body `eventually holds((lookup<V::Vault>(vaults, rw) absent undefined).l @ b = 1)`, `rw` valued a key not in `vaults` | `refuted`, `decisive-counterexample`, cause `undefined-evaluation` with `absent-key`, although a witness lasso exists for the defined part |
| HF-12 | HF-05's witness with its lasso made unfair under a `fair weak V::Vault::step;` constraint on `b`, and HF-06's trap replayed on a model where `step` is enabled | `inconclusive`, `replay-parity` in both cases |
| HF-13 | Over the counter subject, `hyper N using inf over behaviours (v: M::Counter) exists trace a of M { always holds(v.value @ a <= 3) }`, maximum depth 1, `witness_samples` 0, `p` at 0 and `q` at 4. Precondition: within depth 1 instance `p`, first in order, is `bound-reached`, and instance `q` has a trap | one item, `refuted`, `closed-scope`, at `q` |
| HF-14 | HF-13's clause with both counters at 4, over the universe `[p, q]` and over `[q, p]` | one item, `refuted`, at `p` for `[p, q]` and at `q` for `[q, p]` |
| HF-15 | `hyper N using inf over behaviours (v: M::Counter, w: M::Counter) exists trace a of M { always holds(v.value @ a = w.value @ a) }`, universe `[p, q]`, `p` at 0 and `q` at 4 | one item over 4 instances; `(p, q)` and `(q, p)` are refuted; the item reports `v = p`, `w = q` |
| HF-16 | Injected: (a) `p` `inconclusive` `induction-not-closed`, `q` `inconclusive` `vacuous-match`, then the two outcomes exchanged; (b) `p` `proved`, `q` `inconclusive` `bound-reached`; (c) `p` `inconclusive` `bound-reached`, with the caller cancelling before `q` is refuted | (a) `p`'s result each time; (b) `inconclusive`, `bound-reached`, never `proved`; (c) `incomplete`, `cancelled` |
| HF-17 | `hyper N using inf over behaviours (v: M::Counter) forall trace a of M { always holds(v.value @ a <= 1) }`, `p` at 0 and `q` at 2 | one item, `refuted`, `decisive-counterexample`, a `lockstep` counterexample whose `instance` names `v = p` and on which `p` reaches 2, although `q` is violated at position 0 |
| HF-18 | Injected, both `proved`: (a) `p` `certified` `exhaustive`, `q` `uncertified` `inductive{2}`; (b) `p` `certified` `exhaustive`, `q` `certified` `reduced`; (c) both `uncertified`, `inductive{1}` and `inductive{3}`; (d) both `uncertified`, `p` `inductive{1}`, `q` `bounded-complete{5}`; (e) both `uncertified`, `bounded-complete{5}` and `bounded-complete{3}`; (f) `p` `uncertified` `exhaustive`, `q` `trusted` `exhaustive`; (g) both `uncertified` `exhaustive`, `q`'s run under a state constraint; (h) both `uncertified`, `p` `bounded-complete{5}`, `q` a `bounded-solver` horizon of 3 time units; (i) both `uncertified`, `p` `exhaustive`, `q` `bounded-complete{5}` | (a) `uncertified`, `q`'s; (b) `certified`, `q`'s; (c) `p`'s; (d) `q`'s; (e) `q`'s; (f) `trusted`, `q`'s; (g) `q`'s; (h) `p`'s, bounds in different units not being compared; (i) `q`'s; each with both instances' evidence |
| HF-19 | Injected: (a) `p`'s refutation fails replay with `replay-parity` and `q`'s replays, then the two outcomes exchanged; (b) `p`'s refutation fails replay and `q` is `proved`, then exchanged; (c) HF-17's clause under an engine returning one outcome over all instances, its counterexample for `p` altered so that replay disagrees | (a) `refuted` with the confirmed instance's evidence each time; (b) `inconclusive`, `replay-parity` each time; (c) `inconclusive`, `replay-parity`, although `q` is violated |
| HF-20 | HF-13's clause and starts, `witness_samples` 0, with `max_states` 1 and no maximum depth, over the universe `[p, q]` and over `[q, p]` | `[p, q]`: `incomplete`, `limit-reached` naming `max_states` 1, since `p` spends the item's one state; `[q, p]`: `refuted`, `closed-scope`, at `q`, whose part is closed because its unexpanded states have no run of the body's automaton, and the trap replays |
| HF-21 | HF-14's clause over `[p, q]`, on a variant whose `inc` precondition has no value for a receiver at value 4 | never `closed-scope`; `refuted`, `undefined-evaluation` |
| HF-22 | HF-15's clause with both counters at 3 | one item, `proved`, `decisive-witness`, with four `witness` arms in the order `(p, p)`, `(p, q)`, `(q, p)`, `(q, q)`, each naming its two-entry `instance`, and each replays |
| HF-23 | Injected, over HF-15's clause shape with instances `p` then `q`, the caller cancelling before any instance is refuted: (a) every initial state of both instances has a witness and `p`'s exploration is incomplete; (b) `p` has a witness for every initial state and `q` has none | (a) `inconclusive`, `well-definedness-unchecked`, with `p`'s witnesses; (b) `incomplete`, `cancelled` (FR-396-AC-10) |
| HF-24 | Over the counter subject, `hyper N using inf over behaviours (v: M::Counter) exists trace a of M { always holds(v.value @ a <= 3) }`, `p` at 0 and `q` at 1, universe `[p, q]` | one item, `proved`, `decisive-witness`, one terminal record whose `evidence` is an array of exactly two `witness` arms, the first naming `v = p` and the second `v = q`, each replaying (FR-396-AC-12) |
| HF-25 | HF-24's clause with `p` at 0 and `q` at 4, then with both at 4 | `refuted`, `closed-scope`, with exactly one `counterexample` and no `evidence` array: the `trap` arm naming `v = q` the first time, and the `trap` arm naming `v = p` the second (FR-396-AC-13) |
| HF-26 | HF-24's clause over a population with an empty universe, so the clause has zero instances | `inconclusive`, `no-instance`, never `proved`, with no `evidence` array (FR-396-AC-14) |

## Expected Results

Each clause has exactly one form, HP-4 never reaches an engine, a single-existential claim is settled by the possibility family's rows, and a `hyper` clause has one result over its instances, never `proved` unless every instance is.
