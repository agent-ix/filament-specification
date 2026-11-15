---
id: FR-500
title: "Select the edition and the profiles of a source unit"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-339
    type: references
  - target: ix://agent-ix/filament-specification/FR-001
    type: references
  - target: ix://agent-ix/filament-specification/AD-003
    type: references
---

# FR-500: Select the edition and the profiles of a source unit

## Description

When a source unit is read, the frontend SHALL admit exactly the edition `1-draft`, zero or more `profile` declarations and a `function` or `predicate` with or without `using`, applying the fixed default stated below to a unit or declaration that selects no profile.

## Inputs

UTF-8 source bytes whose header names the language `ix:native` and an edition
string, followed by the unit's `profile`, `import` and `model` declarations and
its declarations.

## Outputs

The selected edition, the unit's ordered profile selections (possibly none) and
each declaration's `using` alias (possibly none), or a located refusal.

## Behavior

### One edition

The standard defines one edition, `1-draft`. A header whose edition string is
any other string, `0-draft` included, refuses
`unknown_edition`/`unsupported-selection`, located at the edition string literal
(the half-open byte range of the quoted token, quotes included), and the unit
reaches no later stage. The earlier `0-draft` edition and the
`state-finite/0-draft` profile defined under it are retired: no frontend admits
source under them, so a requirement promising that their rules and bytes stay
admissible would describe a conformance no implementation can meet. Their
documents remain as the design record the composed `1-draft` definitions were
derived from. The edition rule that a later edition adds grammar only under a
new exact edition identity, and never reinterprets bytes an earlier edition
admitted, governs every edition defined after `1-draft`.

### Zero or more profiles

A unit declares zero or more `profile` declarations after its header. A unit
that declares none selects no layer. Its declarations are admitted under the
admitted-form set of `filament.value.complete/v1`, the forms that need no
other layer, and a form that needs another layer refuses
`unsupported_construct` exactly as it does in a unit that selects only
`filament.value.complete/v1`. Its definition selection is the one FR-001 gives
a unit whose selected layers do not include `filament.value.complete/v1`:
exactly the edition. A construct whose meaning needs a selected law or
definition (for example the integer-division law of `div`) refuses as it does
in any unit that selects no such definition. This default is fixed by the
standard, never derived from an installed backend. A unit of value
declarations only needs no selection, and requiring a `profile` line there
would add a declaration that changes nothing.

### Optional `using` on functions and predicates

A `function` or `predicate` declaration may omit `using <alias>`. A declaration
without it selects no profile: the declaration, its `decreases` measure and its
body are admitted as the declarations of a unit that selects no profile are,
whatever the unit's header selects. A `using` alias that names no `profile`
declaration of the unit still refuses as before. Every other declaration that
the grammar writes with `using` keeps it required, because its family needs a
selected profile to give it meaning. A pure function over value types needs no
family profile, and the optional alias states that directly instead of making
the author name the value layer.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-500-AC-1 | `language "ix:native" edition "1-draft";` followed by a declaration parses; the same unit with edition `"0-draft"`, `"2-draft"` or `""` refuses `unknown_edition`/`unsupported-selection` at byte range 29..38, 29..38 and 29..31 respectively (the edition string literal), with no other diagnostic and no checked package. | Test (TC-1000) |
| FR-500-AC-2 | `language "ix:native" edition "1-draft";` followed only by `function f(): Boolean pure { true }` parses with no diagnostic and checks with no profile selection, no selected layer and the edition as its only definition selection; `f()` evaluates to `true`. | Test (TC-1001) |
| FR-500-AC-3 | In a unit with no `profile` declaration, a function whose body uses a form of `filament.model.complete/v1` (a `lookup`) refuses `unsupported_construct` at that form, naming an empty list of selected layers and `filament.model.complete/v1` as the required layer; the same function in a unit whose only profile is `filament.value.complete/v1` refuses with the same code and cause at the same form. | Test (TC-1001) |
| FR-500-AC-4 | In a unit that declares `profile v = "filament.value.complete/v1";`, `function g(x: Integer): Integer pure { x + 1 }` and `predicate p(x: Integer): Boolean { x > 0 }` parse and check with no profile selection for either declaration; `function h using w(x: Integer): Integer pure { x }` with no `profile w` refuses at the alias `w` as an undeclared alias does today; `invariant I on M::T at current { true }` with no `using` refuses `invalid_syntax`/`unexpected-token` at `on`. | Test (TC-1001) |

## Dependencies

- [FR-339](FR-339-define-complete-native-grammar.md) incorporates the grammar this requirement constrains.
- [FR-001](../foundation/FR-001-select-native-profile.md) owns layer and definition selection.
- [AD-003](../../assurance/AD-003-native-filament-language-charter.md) fixes the profile hierarchy and the edition-evolution rule.
