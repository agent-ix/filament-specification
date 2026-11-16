---
id: FR-729
title: "Encode and step a post-test repeat"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-466
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-426
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
---

# FR-729: Encode and step a post-test repeat

## Description

The `repeat` control (FR-466 "Control layouts") SHALL have an optional `test`
argument, the text `post`, after `exhausted`. A `repeat` without `test` is
pre-test, as before, and its bytes are unchanged. A `repeat` with `test`
`post` is post-test:

- its body runs once before the guard is first read;
- its guard, its `visible` entries and its `proof` `invariant` and `variant`
  are read at the body's exit, over the binders of the iteration that has just
  completed;
- its `maximum`, when present, is at least 1.

### Decision: give the package a post-test form

A loop that runs an operation and repeats while the result says to retry,
such as `repeat by c visible (loaded) while { not loaded } { attempt Load by c
as (loaded: Boolean) }`, is the usual shape of a retry loop. Its guard reads
the body's binder, so it has no pre-test meaning. Without a package form the
loop could be written and checked in a source language and then never
written, explored or replayed. The checked package therefore carries the form.
The `test` member is optional and has one value, so every existing pre-test
package keeps its bytes and node ids.

## Behavior

- **Scope.** Under `test` `post`, the guard, the `visible` entries, the
  `invariant` and the `variant` read `In(repeat)` together with `Out(body)`,
  where `Out(body)` is the binders the body binds on every path to its exit.
  Under pre-test they read `In(repeat)` only (FR-466 "Binder scope").
  `Out(repeat)` is `In(repeat)` in both forms: iteration binders do not
  escape the loop.
- **Decision facts.** A `visible` entry over a body binder follows FR-466
  "Decision facts" with that binder as an eligible record binder of the
  owner.
- **Layout refusals.**
  - A `test` value other than `post` refuses `malformed_wire`.
  - A post-test `repeat` with `maximum` 0 refuses `invalid_package`/
    `invalid-value` at the node.
  - A pre-test `repeat` whose guard, `visible` entry, `invariant` or
    `variant` reads a body binder refuses `missing_declaration`/`missing-name`
    as an out-of-scope read, as FR-466 states.
- **Moves (FR-426).** At a post-test `repeat`, entry from outside sets the
  counter to 0 and enters the body without reading the guard. At the body's
  exit the guard is read:
  - when it is true and the counter, advanced by one, is below the maximum
    (or there is no maximum), the body is entered again;
  - when it is true at the maximum, `exhausted` is entered;
  - when it is false, the `repeat` is left.

  Like the pre-test moves, these moves have no transition identity.
- **Progress.** The FR-464 "Nested `repeat` rule" and the progress rule apply
  unchanged. A post-test body is always entered once, so a post-test `repeat`
  has a step on every path through it exactly when its body does.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-729-AC-1 | The `repeat` of `attempt Load by c as (loaded: Boolean)` with `test` `post`, `visible (loaded)` and guard `not loaded` is admitted with no `maximum`, with `maximum` 3 and with `maximum` 1. With `maximum` 0 it refuses `invalid_package`/`invalid-value` at the node. With `test` `pre` it refuses `malformed_wire`. Without `test` the same reads refuse `missing_declaration`/`missing-name` as out-of-scope reads. | Test (TC-1773) |
| FR-729-AC-2 | Over the admitted post-test `repeat`, the first `attempt(Load)` is enabled at the instance's first state with no guard read. After a `Load` with `loaded` false the body is entered again. After one with `loaded` true the `repeat` is left. Under `maximum` 2, a second false `loaded` enters `exhausted`. | Test (TC-1773) |
| FR-729-AC-3 | A pre-test `repeat` written without `test` has the same node bytes and node id as before the member existed. A post-test `repeat` and the pre-test `repeat` of the same body and guard have different node ids. | Test (TC-1773) |
| FR-729-AC-4 | A post-test `repeat` whose `invariant` and `variant` read the body's binder is admitted. The same `repeat` whose `visible` entry reads a binder bound on only one branch of a body `choice` refuses `missing_declaration`/`missing-name` as an out-of-scope read. | Test (TC-1773) |

## Dependencies

- [FR-466](FR-466-checked-package-protocol-control-bodies.md): the `repeat`
  layout, binder scope and decision facts.
- [FR-426](../../functional/protocol/FR-426-apply-protocol-steps.md): the
  structural moves.
- [FR-464](FR-464-checked-package-protocol-node-bodies.md): the progress and
  nested-`repeat` rules.

## Status

Proposed complete-V1 boundary contract.
