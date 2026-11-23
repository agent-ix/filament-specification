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
heading id that owns the fence. A fence with no owning heading uses
`<document identity>#unowned-<line>`, where `line` is the one-based line of
its opening fence. Every item and diagnostic names its unit as
the document's labels, its path, its clause id when a heading owns the fence,
and the body identity; an unowned fence has no `clause` member. It keeps
its span in the body, and gives the line and column in the Markdown file. A
heading owning more than one `ix:native` fence refuses each of them
`extraction-clause-count`; a fence no heading owns refuses
`missing_declaration`. One fence's refusal does not stop the others. A fence
of any other language is not checked.

A failure of the whole-document extraction or checking operation is an
operation-level failure, not a fence item. The result has operation `check`,
no items and one operation diagnostic. An extraction refusal has category
`refusal` and `last_stage: null`; a reached limit has category `incomplete`,
its catalog code and cause, and `last_stage` equal to the stage that reached
it; cancellation has category `incomplete`, its cancellation code and cause,
and no stage; an internal fault has category `internal_failure`, its catalog
code and no stage. The exit status follows FR-301.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-563-AC-1 | `check --markdown agent-ix doc spec.md --format json` over a document with a checking fence under `### Healthy` and an ill-typed fence under `### Broken` writes two items, the first success with body identity `doc#Healthy`, the second `ill_typed` naming document `doc`, path `spec.md`, clause `Broken`, body identity `doc#Broken` and the Markdown line and column of the offending token; exit `20`. | Test (TC-1243) |
| FR-563-AC-2 | `run` over a clause-run request selecting `Healthy` through `program.extraction` in the default build runs it and writes its outcome document; `check` with a Markdown file but without `--markdown` reads it as native source and refuses `invalid_syntax`. | Test (TC-1243) |
| FR-563-AC-3 | An `ix:native` fence opened on Markdown line 3 before any heading produces a `missing_declaration` refusal at line 3, has no `clause`, and uses body identity `doc#unowned-3`; other owned fences in the same document retain their own identities and results. | Test (TC-989) |
| FR-563-AC-4 | A whole-document extraction refusal, limit, cancellation and injected fault each produce an operation `check` outcome with no items and one operation-level diagnostic: respectively `refusal` with no stage, `incomplete` with the reached stage and the limit code/cause, `incomplete` with no stage and cancellation code/cause, and `internal_failure` with no stage and the fault catalog code; exits are 20, 22, 22 and 30. | Test (TC-989) |

## Dependencies

- [FR-301](FR-301-expose-complete-cli.md) owns the CLI.
- [FR-352](../foundation/FR-352-mint-native-run-result-witness-wire.md) owns the `extraction` member of a run result.
