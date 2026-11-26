---
id: TC-297
title: "Deadlock-freedom vectors"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-366
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-391
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-364
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-365
    type: verifies
---

# TC-297: Deadlock-freedom vectors

## Description

The deadlock vectors: a refuted deadlock-freedom item with a prefix that
replays, the same model with `terminal when` covering the deadlocked state, and
with `terminal any`, plus state-graph settlement over mixed terminal
classifications and the rules that keep authored claims unchanged.

## Test Procedure

The **Job** subject: object type `Job` with `status` of an enum
`{queued, running, done, stuck}`; operation `start` (`queued` to `running`),
`finish` (`running` to `done`) and `fail` (`running` to `stuck`). Universe
`{j}`, initial `status = queued`. `done` and `stuck` are reachable and
successor-free.

1. Request `always eventually holds(j.status = done)` under
   `filament.temporal.infinite-trace/v1` with no `terminal` declaration.
2. Add `terminal Finished using v on M when { … }` whose predicate holds at
   `done` and at `stuck`, and repeat.
3. Change the predicate to hold at `done` only, and repeat.
4. Replace the declaration with `terminal Halts using v on M any`, and repeat.
5. Add a second temporal item over the same subject, then a third item over a
   subject with universe `{j, k}`.
6. Add a second `terminal` declaration for the same model alias.
7. Change the step 3 predicate to hold at `running` and `done`.
8. Replay a deadlock counterexample whose last state is `running`.
9. With universe `{j, k}`, declare `terminal Finished using v on M when {
   self.status = done }`; classify the terminal state with both objects
   `done` and the one with `j` `done` and `k` `stuck`; repeat with universe
   `{}`; then add a second object type with its own population to `M`;
   then, instead, add a second population of `Job` to `M`.
10. Over FR-366-AC-12's unit (`M` and `N` selecting one identity; `q(x)` is
    `6 mod x`), classify the terminal state and settle the deadlock-freedom
    item under each declaration pair of that criterion, in both placements;
    then write the request for the same declarations over a unit whose
    aliases select `test/vault` and `test/vault-other`.
11. On the same selected model and intended terminal declaration pair from
    step 10 (`Done` undefined and `Stop` true), add a `possible true`
    state-graph item and complete the exploration.
12. On that same model, set `Stop`'s terminal predicate evaluation to the
    existing typed `refused` outcome, leave `Done` undefined, and add a
    `possible` state-graph item whose predicate is false at every state in the
    unit. Its terminal node remains open as an undecided successor, and the
    graph has no other refuting evidence.
13. On the same unknown-terminal model from step 12, add a `possible true`
    state-graph item, which has a witness at every initial state; keep the
    unknown terminal open.
14. On that same model and terminal state at `l = 1`, set both aliases'
    terminal predicates to `q(1 - self.l) = 0`, add the same state-graph item,
    and complete the exploration with no other refuting evidence.

## Expected Results

1. The request holds one deadlock-freedom item, settled `refuted`, basis
   `decisive-counterexample`, with a `kind: deadlock` finite prefix to the first
   deadlocked state in canonical breadth-first order, which replays
   (FR-366-AC-1, FR-364-AC-1, FR-365-AC-5). The authored claim settles
   `refuted` with a terminal stutter lasso at `stuck` (FR-366-AC-5).
2. The deadlock-freedom item settles `proved`, proof basis `exhaustive`; the
   authored claim's verdict is unchanged (FR-366-AC-2, FR-366-AC-5).
3. The item settles `refuted` with a prefix ending at `stuck` (FR-366-AC-2).
4. The request holds no deadlock-freedom item; the authored claim's verdict is
   unchanged (FR-366-AC-3).
5. Two items over one subject share one deadlock-freedom item; the
   two-object subject receives its own, with identity (subject,
   `deadlock-freedom`) (FR-366-AC-4).
6. Refuses `ambiguous_declaration`/`ambiguous-name` naming both declarations
   (FR-366-AC-6).
7. `running` is not terminal and is never reported as intended or as a
   deadlock; the item still settles `refuted` at `stuck` (FR-366-AC-7).
8. `inconclusive`, cause `replay-parity`: `finish` and `fail` are enabled at
   `running` (FR-364-AC-6).
9. Both `done`: intended; `j` `done`, `k` `stuck`: deadlocked; universe
   `{}`: the initial terminal state is intended; two object types: `self`
   refuses `missing_declaration`/`missing-name` at `self`; two populations of
   `Job`: `self` refuses `ambiguous_declaration`/`ambiguous-name` at `self`,
   naming both (FR-366-AC-9).
10. One identity under `M` and `N`: undefined `Done` with `when { true }`,
    in both placements: intended, `proved`. Undefined `Done` with
    `when { false }`: `refuted`, `undefined-evaluation`, naming `Done`'s
    terminal key in both placements (label `M`, then `N`). `when { false }`
    on both: deadlocked, `kind: deadlock`. `Done` with `any`: no
    deadlock-freedom item. The two-identity unit: `unsupported`, no request
    item (FR-366-AC-12).
11. The terminal state is `intended` under FR-366; the state-graph item is
    `proved`, basis `decisive-witness`, with no `Undefined` evidence arm
    (FR-391-AC-15).
12. The terminal state is `unknown`; the no-witness state-graph item is
    `inconclusive`, cause `undecided-successor`, naming the first unknown
    declaration in terminal-row order (FR-391-AC-16).
13. The terminal state remains open and `unknown`; the state-graph item is
    `inconclusive`, cause `well-definedness-unchecked`, carrying the witness
    (FR-391-AC-13).
14. The terminal state is `undefined`; the state-graph item is `refuted`,
    basis `decisive-counterexample`, cause `undefined-evaluation`, with the
    `Undefined` evidence arm naming the first declaration in terminal-row
    order and replaying to its undefined value (FR-391-AC-17).
