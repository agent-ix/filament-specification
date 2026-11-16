---
id: FR-729
title: "Encode and step a post-test repeat and a repeat's back-edge reads"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-466
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-426
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
---

# FR-729: Encode and step a post-test repeat and a repeat's back-edge reads

## Description

The `repeat` control (FR-466 "Control layouts") SHALL have an optional `test`
argument, the text `post`, which follows `exhausted`.

- A `repeat` without `test` is **pre-test**. Its guard is read at entry and
  after each completed body iteration, and a false entry guard skips the body.
  Its bytes are the same as they were before the member existed.
- A `repeat` with `test` `post` is **post-test**. Its body runs once before
  the guard is first read, the guard is read after each completed iteration,
  and its `maximum`, when present, is at least 1.

In both forms, the `repeat`'s `visible` entries and its `proof` `invariant`
and `variant` MAY read binders that the body binds on every path to its
exit. These are *back-edge reads*. Each one reads the value from the most
recently completed iteration. In a pre-test `repeat`, a back-edge read has no
value at entry, because no iteration has completed. Only a post-test guard
reads body binders.

### Decision: give the package every repeat that a source language can write

A retry loop such as `repeat by c visible (loaded) while { not loaded } {
attempt Load by c as (loaded: Boolean) }` has a guard that reads the body's
binder, so it is post-test. A pre-test loop may also list a body binder as a
decision fact, or prove a variant over it, reading the previous iteration's
value (FSL FR-218-AC-18). Without a package form for both cases, a source
language could check such a loop but never write, explore or replay it. The
`test` member is optional and has one value, so every existing pre-test
package keeps its bytes and node ids.

## Behavior

- **Scope.**
  - The guard reads `In(repeat)`. Under `test` `post` it also reads
    `Out(body)`, the binders the body binds on every path to its exit.
  - The `visible` entries, the `invariant` and the `variant` read
    `In(repeat)` and `Out(body)` in both forms.
  - `Out(repeat)` is `In(repeat)`: iteration binders do not escape.
- **Entry of a pre-test `repeat`.** At entry, before any iteration has
  completed:
  - a `visible` entry that reads a body binder is not a decision fact, and
    the guard cannot read it in any case;
  - an `invariant` or `variant` that reads a body binder is not evaluated.

  At every later test, these slots are evaluated over the completed
  iteration's binders.
- **Decision facts.** A `visible` entry over a body binder follows FR-466
  "Decision facts", with that binder as an eligible record binder of the
  owner.
- **Layout refusals.**
  - A `test` value other than `post` refuses `malformed_wire`.
  - A post-test `repeat` with `maximum` 0 refuses `invalid_package`/
    `invalid-value` at the node.
  - A pre-test guard that reads a body binder refuses
    `missing_declaration`/`missing-name` as an out-of-scope read.
  - Any slot that reads a binder bound on only some paths to the body's exit
    refuses `missing_declaration`/`missing-name` as an out-of-scope read.
- **Moves (FR-426).**
  - At a post-test `repeat`, entry from outside sets the counter to 0 and
    enters the body without reading the guard. At the body's exit the guard
    is read. When it is true and the counter, advanced by one, is below the
    maximum (or there is no maximum), the body is entered again. When it is
    true at the maximum, `exhausted` is entered. When it is false, the
    `repeat` is left.
  - A pre-test `repeat` moves as FR-426 states.
  - Neither form's moves have a transition identity.
- **Progress.** The FR-464 "Nested `repeat` rule" and the progress rule apply
  unchanged. A post-test body is always entered once, so a post-test `repeat`
  has a step on every path through it exactly when its body does.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-729-AC-1 | The `repeat` of `attempt Load by c as (loaded: Boolean)` with `test` `post`, `visible (loaded)` and guard `not loaded` is admitted with no `maximum`, with `maximum` 3 and with `maximum` 1. With `maximum` 0 it refuses `invalid_package`/`invalid-value` at the node, and with `test` `pre` it refuses `malformed_wire`. Without `test`, the guard `not loaded` refuses `missing_declaration`/`missing-name` as an out-of-scope read. | Test (TC-1773) |
| FR-729-AC-2 | Over the admitted post-test `repeat`, the first `attempt(Load)` is enabled at the instance's first state with no guard read. After a `Load` with `loaded` false the body is entered again, and after one with `loaded` true the `repeat` is left. Under `maximum` 2, a second false `loaded` enters `exhausted`. | Test (TC-1773) |
| FR-729-AC-3 | A pre-test `repeat` written without `test` has the same node bytes and node id as before the member existed. A post-test `repeat` and the pre-test `repeat` of the same body and guard have different node ids. | Test (TC-1773) |
| FR-729-AC-4 | A post-test `repeat` whose `invariant` and `variant` read the body's binder is admitted. The same `repeat` whose `visible` entry reads a binder that only one branch of a body `choice` binds refuses `missing_declaration`/`missing-name` as an out-of-scope read. | Test (TC-1773) |
| FR-729-AC-5 | A pre-test `repeat` with guard `true` and `visible (loaded)`, and one whose `invariant` and `variant` read `loaded`, are admitted without `test`. At entry the body is entered with no fact for `loaded` and with neither proof slot evaluated. After the first iteration, `loaded` is the decision fact and the proof slots are evaluated over that iteration's value. | Test (TC-1773) |

## Dependencies

- [FR-466](FR-466-checked-package-protocol-control-bodies.md): the `repeat`
  layout, binder scope and decision facts.
- [FR-426](../../functional/protocol/FR-426-apply-protocol-steps.md): the
  structural moves.
- [FR-464](FR-464-checked-package-protocol-node-bodies.md): the progress and
  nested-`repeat` rules.

## Status

Proposed complete-V1 boundary contract.
