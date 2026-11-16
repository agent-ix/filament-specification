---
id: FR-719
title: "Check a unit against supplied models and libraries"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-154
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-866
    type: references
---

# FR-719: Check a unit against supplied models and libraries

## Description

When a user invokes `check` on a unit that selects a domain package or
imports a library, the FSL CLI SHALL accept those inputs as options,
`--model <identity>=<file>` and `--library <identity>=<file>`, each
repeatable, and check the unit against them exactly as a compile request
that supplies the same models and libraries checks it. A rule written over a
model is checked with one command and no request file.

## Inputs

The source path, the `--model` and `--library` operands, and the existing
`check` options of FR-301.

## Outputs

The `check` outcome and FR-301's exit status.

## Behavior

A model is supplied under its identity alone, with no version or digest
(FR-154). An operand with no `=`, an empty identity or path, or an identity
given twice to one option is refused before any file is read, exit `20`,
naming the option. A supplied model or library that no declaration of the
unit selects is refused naming the option and the identity, because an
input that changes nothing is a mistake the user should see. A selected
model that is not supplied is refused `missing_import`, naming the identity
and the option that supplies it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-719-AC-1 | `check` over a unit with `model Config = "example/config-version"` and an invariant over `Config::ConfigVersion`, given `--model example/config-version=<file>`, exits `0`; without the option it exits `20` with `missing_import` naming the identity and `--model`. | Test (TC-1756) |
| FR-719-AC-2 | The same `--model` operand given twice, an operand with no `=`, and a `--model` for a unit with no `model` declaration each exit `20` naming `--model`; a `--library` for a library the unit does not import exits `20` naming `--library`. | Test (TC-1756) |

## Dependencies

- [FR-301](FR-301-expose-complete-cli.md) owns the CLI.
- [FR-154](../type-model/FR-154-admit-domain-package-model.md) owns model
  selection by identity.
- FSL FR-866 implements the options and their library entry.
