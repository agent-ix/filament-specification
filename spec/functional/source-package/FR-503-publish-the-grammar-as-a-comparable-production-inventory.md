---
id: FR-503
title: "Publish the grammar as a production inventory an implementation can compare with"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-339
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-600
    type: references
---

# FR-503: Publish the grammar as a production inventory an implementation can compare with

## Description

The shared native grammar SHALL define every production of the `1-draft`
grammar in a fenced `ebnf` block, so that the set of production names is read
from the document alone and an implementation's parser can be compared with it
by a test that fails when the two disagree.

## Inputs

`proposals/filament-v1/shared-grammar.md`.

## Outputs

The ordered set of production names the document defines, each with the block
it is defined in.

## Behavior

- A production is defined by a line inside a fenced code block whose info
  string is `ebnf`, of the form `name = ...` (a definition) or `name =/ ...`
  (alternatives added to an earlier definition of the same name). Names are
  lowercase ASCII words joined by `-`. A form described only in prose, outside
  an `ebnf` block, is not a production; prose may restrict where a production
  applies but never introduces one.
- A complete-V1 definition of a name the composed grammar also defines
  replaces it when the package selects the complete facets, as the grammar
  states; the inventory holds the name once.
- Every name used on the right-hand side of a definition is either defined in
  some `ebnf` block of the document or is one of the lexical terminals `ident`,
  `member-name`, `string`, `uint`, `decimal-literal` and `EOF` that the lexical
  contract defines. `decimal-literal` is the token `[0-9]+ '.' [0-9]+`; it has
  no exponent form.
- Because the inventory is read from the document, a conforming implementation
  keeps a correspondence between its own grammar productions and these names,
  with each name that has no counterpart on either side listed with the
  reason. A change to either grammar that is not reflected in the
  correspondence is a detectable disagreement, not a silent drift.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-503-AC-1 | Reading every fenced `ebnf` block of the shared grammar yields a non-empty set of production names; each name is defined at least once with `=`, and each `=/` line names a production defined with `=` earlier in the document. | Test (TC-1010) |
| FR-503-AC-2 | Every name on the right-hand side of a definition in the shared grammar is defined in some `ebnf` block or is one of `ident`, `member-name`, `string`, `uint`, `decimal-literal` and `EOF`; `decimal-literal` matches `[0-9]+ '.' [0-9]+` with no exponent form. A name used but not defined is reported with the line that uses it. | Test (TC-1010) |

## Dependencies

- [FR-339](FR-339-define-complete-native-grammar.md) incorporates the grammar as the normative syntax contract.
