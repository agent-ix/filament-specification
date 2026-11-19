---
id: FR-770
title: "Carry each function node's check mode"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-323
    type: references
---

# FR-770: Carry each function node's check mode

## Description

Every `function` node of a `filament.checked-package/v2` package SHALL carry
the closed member `check_mode`, whose value is exactly one of the strings
`linked` and `kernel`. It records how the producer's checker treated the
body's definedness obligations:

- `linked`: every definedness obligation of the body (range, nonzero,
  presence, rational-range, nonempty-reduction) was proved at check, so the
  body is defined for every argument in its domain.
- `kernel`: those obligations were left to evaluation, so the body may
  evaluate undefined (for example `division-by-zero`) at an argument where
  an obligation fails.

The member is identity-bearing. It is part of the node's content, compared
structurally as every other member is ("Identity and validation"). Two
packages that differ only in one function node's `check_mode` are two
packages, and the same `{package, path}` reached in two places with unequal
`check_mode` refuses `invalid_package`/`conflicting-definition`. It does not
enter a node's `path`: a declared function keeps its declaration path, and
a declaration-less function keeps the derived path of "Declaration-less
function nodes".

A replay of a claim minted from a `kernel` body has to meet the body in the
same mode to reproduce an undefined evaluation. Without the member, a
consumer could not tell a `kernel` package from a `linked` one. With it,
replay recompiles each body under its recorded mode (FSL FR-098).

A declared clause is always checked as `linked`, whatever mode its unit's
function bodies were checked under. An `invariant` or `precondition` clause
function ("clause kind", FR-322) therefore carries `linked`, and so does a
`reward` clause function (FR-640): a reward is a declared expression of the
model, a provider evaluates it on every transition it rewards, and its
definedness must not depend on the mode the unit's function bodies chose. A `body` clause
function, a declared helper `function` and a declaration-less function carry
the mode their body was checked under.

## Behavior

- A producer SHALL write `check_mode` on every `function` node and SHALL
  write `linked` on every `invariant`, `precondition` or `reward` clause
  function.
- A reader SHALL refuse a `function` node with no `check_mode`, or with a
  value other than `linked` or `kernel`, as `malformed_wire` at
  `/semantic_graph/nodes/{i}/check_mode`, `i` the node's graph index.
- A reader SHALL refuse `check_mode` on a node whose `node_tag` is not
  `function` as an unknown member (`malformed_wire`).
- After the owner join, a reader SHALL refuse `kernel` on an `invariant`,
  `precondition` or `reward` clause function as `invalid_package`/`invalid-value` at
  that pointer.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-770-AC-1 | A package whose every `function` node carries `"check_mode": "linked"` is admitted, and so is the same package with a declared function's and a `body` clause function's `check_mode` `"kernel"`; the two admitted packages are unequal, and a dependency diamond reaching one `{package, path}` with `linked` on one side and `kernel` on the other refuses `invalid_package`/`conflicting-definition`. | Test (TC-1860) |
| FR-770-AC-2 | Negative: a `function` node with `check_mode` removed, or set to `"strict"`, `""` or `true`, refuses `malformed_wire` at `/semantic_graph/nodes/{i}/check_mode`; `check_mode` on an `expression` node refuses `malformed_wire`; `"kernel"` on an `invariant` clause function owned by `Order` refuses `invalid_package`/`invalid-value` at that pointer, and on a `precondition` clause function the same; FR-640-AC-1's `duration` reward function carries `"linked"`, and with `"kernel"` it refuses `invalid_package`/`invalid-value` at its `check_mode`. | Test (TC-1860) |

## Dependencies

- FR-322: the function node, clause kinds and identity rules.
- FSL FR-098, FR-495 and FR-952: the producer's check modes and the replay
  that reads them.
