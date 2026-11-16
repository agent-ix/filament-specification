---
id: FR-718
title: "Write a readable text outcome by default"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-spec-language/FR-865
    type: references
---

# FR-718: Write a readable text outcome by default

## Description

When a user invokes `parse`, `check`, `compile` or `run` without naming an
output format, the FSL CLI SHALL write the text form of the same outcome it
writes in machine mode, so the first command a reader types prints a result.
Text is a display of the machine outcome, built from it alone; the machine
outcome stays the authoritative form. `analyze` and `monitor` keep the text forms their own
requirements give them. The output format changes the bytes
written and never the exit status.

## Inputs

A verb, its operands and the optional `--format json|text` (default
`text`).

## Outputs

The text outcome on stdout, or the machine outcome under `--format json`,
and FR-301's exit status of the outcome's category in both modes.

## Behavior

The text outcome names the operation, the outcome category, the exit
status, the last stage reached when one is known, every diagnostic's code,
cause and source position, every item's result, category and cause, and the
evaluation result. None of these verbs refuses the text format as unsupported. An
output format other than `json` or `text` is refused before any input is
read, exit `20`, naming the option and the value. A missing or malformed
input is reported with the same code in both modes.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-718-AC-1 | `parse`, `check`, `compile` and `run`, each over an accepted and a refused input with no `--format`, write a text outcome naming the category, exit status, each item's result and each diagnostic's code, cause and position, and exit with the status the same invocation has under `--format json`. | Test (TC-1755) |
| FR-718-AC-2 | `run` and `compile` with no `--format` over a request path that does not exist exit `20` with the I/O failure, as under `--format json`, never `21`; `--format bogus` exits `20` naming `--format` and `bogus` before any file is opened. | Test (TC-1755) |

## Dependencies

- [FR-301](FR-301-expose-complete-cli.md) owns the CLI and its exit table.
- FSL FR-865 implements the text form and the driver's default.
