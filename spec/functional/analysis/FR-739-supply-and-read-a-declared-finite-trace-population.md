---
id: FR-739
title: "Supply and read a declared finite trace population"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-191
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-402
    type: depends_on
---
# FR-739: Supply and read a declared finite trace population

## Description

In a `hyper` clause over a finite trace domain, `t in Q bounded N`, `Q` SHALL
name the record type of one trace state, and `N` SHALL be a positive bound on
the population's size. The population itself SHALL be supplied with the
request as one `filament.trace-population/1` document, never written in the
clause. An analyzer SHALL read it as follows.

| Member | Value |
| --- | --- |
| `format` | the text `filament.trace-population/1` |
| `record_type` | the qualified name of `Q` |
| `closure` | `closed` or `open` |
| `traces` | an array of `{identity, states}`, `identity` a non-empty text unique in the array, and `states` a non-empty array of records of `Q` in canonical value form |

## Rules

- A record that does not conform to `Q`, two traces sharing an identity, or
  an empty `states` array is refused `invalid_runtime_input`/`invalid-value`
  at its JSON Pointer.
- A population with more than `N` traces is not assessed. The result is
  `requires-bound` naming `N` (FR-191-AC-3).
- Each quantified variable ranges over every trace identity, left to right
  in the prefix (FR-191-AC-4). One trace may fill several variables.
- The traces are aligned in lockstep: joint position `j` is position `j` of
  each trace. A read past a trace's last state is an absent alignment. It
  leaves the tuple incomplete and is never compared (FR-191-AC-5).
- An `open` population can refute the claim and can never prove it, so a
  non-refuted result over an open population is inconclusive.
- The population's identity is part of the result's identity: its
  `record_type`, `closure` and the ordered trace identities with their
  states (FR-402).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-739-AC-1 | A well-formed closed population of three traces is read. A record outside `Q`, a repeated identity and an empty trace are each refused at their pointer. Four traces under `bounded 3` give `requires-bound` naming 3. | Test (TC-1786) |
| FR-739-AC-2 | A body that reads a trace past its last state leaves the tuple incomplete. An open population whose tuples all hold gives an inconclusive result, and one with a violating tuple gives a refutation. | Test (TC-1786) |

## Dependencies

- [FR-191](FR-191-analyze-relational-hyperproperties.md): finite-trace-set
  hyperproperties.
- [FR-402](../../objects/interfaces/FR-402-hyperproperty-obligation-identity.md):
  hyperproperty identity.
