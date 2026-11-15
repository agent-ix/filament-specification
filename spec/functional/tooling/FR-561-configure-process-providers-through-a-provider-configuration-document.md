---
id: FR-561
title: "Configure process providers through a provider-configuration document"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-305
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-462
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
---

# FR-561: Configure process providers through a provider-configuration document

## Description

A process plugin runs at the path the caller names, with no discovery and
no search path (FR-305). When a user names process providers to the FSL CLI,
the caller SHALL name them in one provider-configuration document given by
`--provider-config <file>`. The document lists the plugins only: run
budgets stay the caller's limits options, so one configuration serves runs
of any size.

## Inputs

`--provider-config <file>` on a verb that routes items to providers
(`analyze`, `prove`, `monitor`, `replay`, `generate`), naming one document:

```json
{"format": "filament.provider-config/v1",
 "providers": [
   {"path": "bin/my-prover", "arguments": ["--fast"]}
 ]}
```

- `format`: exactly `filament.provider-config/v1`.
- `providers`: required array of closed entries; an empty array names no plugin.
- `path`: required non-blank string, the plugin executable; a relative path resolves against the directory of the configuration document.
- `arguments`: optional array of strings passed to the plugin in order; absent is `[]`.

## Outputs

One process plugin per entry, started in entry order and registered through
its `hello` manifest beside the linked providers (FR-305); or a refusal
before any stage runs.

## Behavior

The CLI SHALL read the document before any stage runs and before any plugin
starts. A missing or unreadable file refuses naming the path; a document
that is not JSON, a missing `format` or `providers`, an unknown member, an
entry missing `path`, a blank `path` or an `arguments` member that is not
an array of strings refuses `invalid-request` naming the file and the
member; a `format` other than `filament.provider-config/v1` refuses
`unknown_wire`/`unsupported-wire`. Each refusal exits `20` with empty
stdout. A verb that routes no items refuses the option as unknown
(FR-301-AC-3). A plugin that fails after it starts settles its items as
FR-305 states.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-561-AC-1 | `prove --provider-config providers.json` whose one entry names a fixture plugin by a path relative to the document starts it, registers its manifest and routes to it only the items negotiation settles on it; `providers` `[]` runs with the linked providers only. | Test (TC-1241) |
| FR-561-AC-2 | A document with no `providers` member refuses `invalid-request` naming the file and `providers`; an entry with an unknown member `env`, an entry with no `path`, and `format` `filament.provider-config/v2` refuse as Behavior states; a path that does not exist refuses naming it; each exits `20` with empty stdout and starts no plugin. | Test (TC-1241) |
| FR-561-AC-3 | `parse --provider-config providers.json` refuses the option as unknown, exit `20`. | Test (TC-1241) |

## Dependencies

- [FR-305](FR-305-load-explicit-rust-plugins.md) and [FR-462](FR-462-speak-the-plugin-wire.md) own the plugin and its wire.
- [FR-301](FR-301-expose-complete-cli.md) owns the CLI options and their refusal.
