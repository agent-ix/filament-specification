---
id: FR-816
title: "Version the TLA+ and Lean 4 target syntax through the profile identity"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-297
    type: depends_on
---

# FR-816: Version the TLA+ and Lean 4 target syntax through the profile identity

## Description

When a TLA+ or Lean 4 profile is selected, its identity SHALL fix the
target language revision, the emitted syntax subset and every mapping rule,
so that a change to any of them is a new profile identity and no package
written under an earlier identity changes meaning.

## Inputs

An FR-297 profile selection.

## Outputs

The fixed target revision and syntax subset the mapper emits.

## Behavior

- `filament.output.tla2/v1` SHALL have `target_standard_refs` `["TLA+
  Version 2", "Specifying Systems (2002)"]` and emit exactly these
  constructs, which are every construct FR-812 to FR-814 use:
  - structure: the module header `---- MODULE <name> ----` and closing
    `====`, `EXTENDS` of `Naturals`, `Integers`, `Sequences`, `FiniteSets`
    and `TLC`, `CONSTANTS`, `VARIABLES`, operator definitions `==` with
    parameters, `RECURSIVE`, `THEOREM` without proof, and `\*` line comments;
  - logic: `TRUE`, `FALSE`, `BOOLEAN`, `/\`, `\/`, `~`, `=>`, `<=>`, `=`,
    `/=`, `\in`, `\notin`, `\A x \in S :`, `\E x \in S :`, `LET`/`IN`,
    `IF`/`THEN`/`ELSE`, `CASE`/`[]`/`OTHER`;
  - arithmetic: integer literals, `Int`, `Nat`, `+`, binary and unary `-`,
    `*`, `<`, `<=`, `>`, `>=`, `..`, and `\div` and `%` only with a positive
    divisor inside the `D_` helpers;
  - data: `{…}`, `{x \in S : p}`, `{e : x \in S}`, `\cup`, `\cap`, `\`,
    `\subseteq`, `Cardinality`, `[x \in S |-> e]`, `f[x]`, `DOMAIN`,
    `[f EXCEPT ![k] = v]`, `@`, records `[a |-> e]` and `r.a`, tuples and
    sequences `<<…>>`, `Len`, `Append`, `Head`, `Tail`, `SubSeq`, `\o`,
    `Seq`, and string literals for enum members and variant tags;
  - actions and time: priming `'`, `UNCHANGED`, `[][Next]_vars`, `[]`,
    `<>`, `WF_vars(A)` and `SF_vars(A)`.
- `filament.output.lean4/v1` SHALL have `target_standard_refs` `["Lean 4
  core, release v4.0.0 or later"]` and emit exactly these constructs, all
  in ASCII (FR-814):
  - structure: `namespace`/`end`, `structure … where`, `inductive … where`
    with `|` constructors and `deriving DecidableEq`, `def`, `abbrev`, and
    `--` line comments;
  - logic: `Prop`, `True`, `False`, `/\`, `\/`, `Not`, `->`, `<->`, `=`,
    `Ne`, `forall x, p`, `exists x, p`, `decide`, `match … with`,
    `if`/`then`/`else`, `fun x =>`;
  - arithmetic: `Nat` and `Int` literals, `+`, `-`, `*`, `<`, `<=`, `>`,
    `>=`, `Int.ofNat`, `Int.natAbs`, and `Nat` `/` and `%` inside the
    generated helpers;
  - data: `Bool`, `true`, `false`, `Nat`, `Int`, `List` with `[]`, `::`,
    `List.length`, `List.map`, `List.foldl` and `List.get?`, `Option` with
    `some` and `none`, `Prod A B` with `(a, b)`, `.1` and `.2`, structure
    field access, and function types `Nat -> State`.
  Generated helpers use only these, so their meaning is the same in every
  Lean 4 release. Mathematical notation in FR-812 and FR-813 (`∀`, `∧`,
  `×`, `≤`) is written in these ASCII forms.
- Changing the target revision, adding an emitted construct, changing a
  mapping rule of FR-812 to FR-814, or changing a disposition rule of FR-651
  SHALL require a new profile identity (`/v2`); the `v1` mapper keeps
  emitting `v1` text, and no stored package is rewritten.
- The release of a target tool that observes a package is part of the
  observer selection and of the observation record, never of the profile.
- A target release that no longer accepts `v1` text SHALL be met with a new
  profile identity; the `v1` profile is never silently re-pointed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-816-AC-1 | Every token class in the TLA+ and Lean outputs of the export conformance corpus (FR-818) belongs to its profile's emitted subset; a generated file using a construct outside it fails the corpus test. | Test (TC-1975) |
| FR-816-AC-2 | The profile records `target_standard_refs` as stated, and changing a reference changes profile equality (FR-297-AC-3). | Test (TC-1975) |
| FR-816-AC-3 | Two observations of one package by two Lean releases reference the same package key and differ only in their observer record. | Test (TC-1975) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md) and
  [FR-297](../../objects/output-mapping/FR-297-output-mapping-consumer-profile.md).
