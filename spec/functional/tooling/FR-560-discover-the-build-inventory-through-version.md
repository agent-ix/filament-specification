---
id: FR-560
title: "Discover the build inventory through version"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-131
    type: references
  - target: ix://agent-ix/filament-specification/FR-132
    type: references
  - target: ix://agent-ix/filament-specification/FR-305
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-661
    type: references
---

# FR-560: Discover the build inventory through version

## Description

When a user invokes `version`, the FSL CLI SHALL report, besides the
application version, the exact editions, profiles, declaration and semantic
families, atomic features and consumers the running build supports, so that
a user can write a header the build admits and a consumer can compare a
package's required-feature set with what the build knows (FR-132-AC-3).

`version` is the verb because the inventory is a fact about the build, like
its version: it takes no input and reads no file.

## Inputs

`version`, `--format json|text`, and an optional `--kind` naming one of
`editions`, `profiles`, `families`, `features` and `consumers`.

## Outputs

In machine mode, one `filament.build-inventory/v1` document on stdout:

```json
{"format": "filament.build-inventory/v1",
 "application": {"name": "fsl", "version": "0.2.0"},
 "editions": ["1-draft"],
 "profiles": [{"identity": "filament.value.complete/v1", "requires": []}],
 "families": [{"family": "SRC", "features": ["V1-SRC-001", "V1-SRC-002"]}],
 "features": ["V1-SRC-001", "V1-SRC-002"],
 "consumers": [{"backend": "kani", "capabilities": [{"kind": "...", "mode": "..."}]}]}
```

- `editions`: every edition the build's `language ... edition` header admits, in edition order.
- `profiles`: every header-selectable profile identity with the identities it `requires`, ascending by identity.
- `families`: every capability family in normative order, each with its feature ids in ordinal order.
- `features`: every atomic capability id the build knows, in normative family and ordinal order.
- `consumers`: every provider linked into the binary, with its backend identity and advertised (kind, mode) pairs, ascending by backend identity.

With `--kind K` the document holds `format`, `application` and the one list
`K`. In text mode each list is a heading followed by one entry per line, in
the same order. The exit status is `0`.

## Behavior

The inventory is read from the tables the compiler and the registry use,
so an entry appears exactly when the build admits it. Two invocations of
one binary write equal bytes. A `--kind` value that names no list refuses
before any output, exit `20`, naming the value and the five list names.
A profile listed is accepted when a header selects it; an identity not
listed is refused `unknown_profile` (FR-301-AC-3).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-560-AC-1 | `version --format json` writes one `filament.build-inventory/v1` document holding the application version and the five lists, whose `features` equal the build's complete capability inventory in order and whose `families` group exactly those ids; two invocations write equal bytes; exit `0`. | Test (TC-1240) |
| FR-560-AC-2 | Every identity in `profiles` is accepted by `check` when a unit's header selects it with its `requires` profiles, and no listed identity begins with `quire.`. | Test (TC-1240) |
| FR-560-AC-3 | `version --kind profiles` writes only `format`, `application` and `profiles`; `version --kind widgets` exits `20` naming `widgets` and the five list names, with empty stdout. | Test (TC-1240) |

## Dependencies

- [FR-301](FR-301-expose-complete-cli.md) owns the CLI and its `version` verb.
- [FR-131](../source-package/FR-131-compose-complete-profile-bundle.md) and [FR-132](../source-package/FR-132-declare-complete-package-capabilities.md) own profiles and capabilities.
- [FR-305](FR-305-load-explicit-rust-plugins.md) owns linked providers.
