---
id: FR-562
title: "Name extension definitions to a command"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-467
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-133
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-660
    type: references
---

# FR-562: Name extension definitions to a command

## Description

A unit selects an extension definition by identity (FR-133), and the
definition's document carries its grammar and typed-node schemas (FR-467).
When a caller supplies extension definitions to a command, the caller SHALL
name each one by an entry that gives its identity, its document file and its
dependency edges. The caller supplies them explicitly: no definition is
found by search, installation or configuration, so a result depends only on
the inputs the caller names.

## Inputs

An *extension entry* is a closed JSON record:

- `authority`, `identity`: the definition's identity, both non-blank;
- `file`: the path of the definition document holding FR-467's members;
- `depends_on`: optional array of `{authority, identity}` records; absent is `[]`;
- `capabilities`: optional array of capability ids; absent is `[]`.

Entries reach a command in two ways:

- a request file's top-level `extensions` member (`compile`, `run`), an
  array of entries whose `file` resolves against the request file's directory;
- `--extensions <file>` on `parse` and `check`, naming one document
  `{"format": "filament.extension-set/v1", "extensions": [ ... ]}` whose
  entries' `file` resolve against that document's directory.

## Outputs

The definition catalog the command compiles every unit against, or a
refusal before any unit is compiled.

## Behavior

Two entries with one identity refuse `invalid_package`/`conflicting-definition`.
An unknown member, a missing `authority`, `identity` or `file`, or a blank
label refuses `invalid-request` naming the member. A unit selecting an
identity no entry supplies refuses `unknown_profile`/`unsupported-selection`
at the identity literal. Definition validation follows FR-467. Each refusal
exits `20`. A unit that selects no extension has the same result with or
without entries (FR-133-AC-1).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-562-AC-1 | `check --extensions set.json unit.native`, with `set.json` naming FR-467-AC-1's `retention` definition and the unit selecting it and declaring `retention Audit { subject: "audit", days: 30 }`, exits `0`; the same unit checked without `--extensions` refuses `unknown_profile`/`unsupported-selection` at the identity literal. | Test (TC-1242) |
| FR-562-AC-2 | A set document entry with an unknown member, one with no `file`, two entries with one identity, and a definition document violating FR-467's `missing-member` rule each refuse with the code Behavior states, exit `20`; a unit selecting no extension checks to the same result with and without the set. | Test (TC-1242) |

## Dependencies

- [FR-467](../source-package/FR-467-define-extension-definition-members.md) owns the definition document.
- [FR-133](../source-package/FR-133-extend-language-uniformly.md) owns extension selection.
- [FR-301](FR-301-expose-complete-cli.md) owns the CLI options.
