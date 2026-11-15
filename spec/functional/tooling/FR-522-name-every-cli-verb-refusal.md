---
id: FR-522
title: "Name every CLI verb refusal and never exit silently"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/ADR-004
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-624
    type: traces_to
---
# FR-522: Name every CLI verb refusal and never exit silently

## Description

When a CLI verb ends, the CLI SHALL write one outcome document in machine
mode, or one diagnostic line in text mode, and exit with the status of its
category (FR-301). When the CLI accepts a verb whose library operation the
build does not route, it SHALL refuse that verb with category
`unsupported`, code `unsupported_operation` and cause `operation-not-routed`
naming the verb, exit `21`.

## Inputs

A CLI invocation.

## Outputs

One outcome document or diagnostic line, and an exit status.

## Behavior

- The catalog holds the code `unsupported_operation`, category
  `unsupported`, with the cause `operation-not-routed` whose payload is
  `{"verb": <verb>}`.
- The refusal is written after argument parsing and before any file is read;
  an unknown option still refuses with exit `20` (FR-301-AC-3).
- In text mode the CLI writes `<tool> <verb>: this build does not route the <verb> operation (unsupported_operation)`
  to stderr.
- An exit status is never produced without the document or line: an empty
  stdout in machine mode happens only when the output stream fails, which is
  a tool failure, exit `30`.
- The refusal describes a build that lacks a verb; it removes no verb from
  the lifecycle FR-301 requires.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-522-AC-1 | Each verb a build accepts but does not route writes an outcome document with that operation, category `unsupported`, code `unsupported_operation`, cause `operation-not-routed` naming the verb, and exits `21`; text mode writes the named line and exits `21`. | Driver test (TC-1082): verb routing and text rendering exist only in the CLI binary, so the driver code ticket QSL-1995 adds the test in the driver repository. |
| FR-522-AC-2 | No verb exits with a non-zero status and empty stdout in machine mode while its output stream is open. | Test (TC-1082) |

## Dependencies

- FR-301 owns the exit contract; ADR-004 records it.
