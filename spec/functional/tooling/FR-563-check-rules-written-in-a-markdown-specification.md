---
id: FR-563
title: "Check rules written in a Markdown specification"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-352
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-662
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-031
    type: references
---

# FR-563: Check rules written in a Markdown specification

## Description

Rules may be written in a Markdown specification as `ix:native` fences, each
under a `### <clause id>` heading. When a user invokes `check --markdown`
with the document's two source labels and its path, the FSL CLI SHALL check
every `ix:native` fence of the document as its own unit and report one item
per fence. `run` runs one selected clause of such a document through a
request carrying `program.extraction` (FSL FR-031). Both are in the default
build. The operand form is explicit: the CLI never decides from a file name
whether bytes are Markdown.

## Inputs

`check --markdown <authority> <identity> <file>`, `--format json|text` and
the optional `--extensions <file>` (FR-562).

## Outputs

One `filament-outcome/1` document with operation `check` and one item per
`ix:native` fence in document order; the exit status is that of the most
severe item (FR-301).

## Behavior

Each fence body is a unit whose identity is the document's authority and
the identity `<document identity>#<clause id>`, the clause id being the
heading id that owns the fence. Every item and diagnostic names its unit as
the document's labels, its path, the clause id and the body identity, keeps
its span in the body, and gives the line and column in the Markdown file. A
heading owning more than one `ix:native` fence refuses each of them
`extraction-clause-count`; a fence no heading owns refuses
`missing_declaration`. One fence's refusal does not stop the others. A fence
of any other language is not checked.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-563-AC-1 | `check --markdown agent-ix doc spec.md --format json` over a document with a checking fence under `### Healthy` and an ill-typed fence under `### Broken` writes two items, the first success with body identity `doc#Healthy`, the second `ill_typed` naming document `doc`, path `spec.md`, clause `Broken`, body identity `doc#Broken` and the Markdown line and column of the offending token; exit `20`. | Test (TC-1243) |
| FR-563-AC-2 | `run` over a clause-run request selecting `Healthy` through `program.extraction` in the default build runs it and writes its outcome document; `check` with a Markdown file but without `--markdown` reads it as native source and refuses `invalid_syntax`. | Test (TC-1243) |

## Dependencies

- [FR-301](FR-301-expose-complete-cli.md) owns the CLI.
- [FR-352](../foundation/FR-352-mint-native-run-result-witness-wire.md) owns the `extraction` member of a run result.
