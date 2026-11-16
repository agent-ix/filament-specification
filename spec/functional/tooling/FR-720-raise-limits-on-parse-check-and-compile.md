---
id: FR-720
title: "Raise limits on parse, check and compile"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-301
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-461
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-146
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-869
    type: references
---

# FR-720: Raise limits on parse, check and compile

## Description

When a user invokes `parse`, `check` or `compile`, the FSL CLI SHALL accept
`--limit <setting>=<value>`, repeatable, for every limit setting of every
stage the verb runs, and `check` and `compile` SHALL accept the byte
ceilings of the files they read, so a limit any of these verbs reports can
be raised on the same verb. Checking depth is never a limit (FR-146); a
deeper source needs only raised budgets, and FR-461 names the setting that
raises each one.

## Inputs

`--limit` operands; the byte-ceiling options of `check` and `compile`.

## Outputs

The verb's outcome under the configured limits.

## Behavior

A setting whose stage the verb does not run is refused before any input is
read, exit `20`, naming the setting and the settings the verb accepts:
accepting it would tell the user a limit was applied when it bounds nothing.
Each such setting is accepted by the verb that runs its stage. Unknown
settings, malformed values and repeated settings are refused as FR-461
states. With no limit configured each stage runs at its published default.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-720-AC-1 | A 6,000-deep parenthesis nest stops `parse` with exit `22` naming its node setting at the default limits, and `parse`, `check` and `compile` each succeed over it with that setting raised by `--limit`. | Test (TC-1757) |
| FR-720-AC-2 | An evaluation-budget setting given to `check`, and a stage setting of a stage `parse` does not run given to `parse`, each exit `20` naming the setting and the verb's accepted settings, before any file is read; a `compile` request over its request-file ceiling refuses naming the option that raises the ceiling, and succeeds with it raised. | Test (TC-1757) |

## Dependencies

- [FR-301](FR-301-expose-complete-cli.md) owns the CLI.
- [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) owns the
  setting names and their refusals.
- FSL FR-869 implements the options per verb.
