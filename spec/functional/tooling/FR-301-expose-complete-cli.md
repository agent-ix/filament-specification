---
id: FR-301
title: "Expose the complete native CLI lifecycle"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-010
    type: implements
  - target: ix://agent-ix/filament-specification/FR-300
    type: references
  - target: ix://agent-ix/filament-specification/FR-560
    type: references
  - target: ix://agent-ix/filament-specification/FR-561
    type: references
  - target: ix://agent-ix/filament-specification/FR-562
    type: references
  - target: ix://agent-ix/filament-specification/FR-563
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-010
    type: references
---

# FR-301: Expose the complete native CLI lifecycle

## Description

When a user invokes the FSL CLI, the command SHALL map parse, check, lower,
execute, replay, monitor, inspect, format and version requests to the exact Rust
lifecycle contracts.

## Inputs

Command, files/stdin, explicit profile/provider/options and output format.

## Outputs

Deterministic stdout artifact/result, structured stderr diagnostics and stable
exit status.

## Behavior

Commands never infer a wider profile or fallback target. Machine output is
separate from human diagnostics. Exit statuses distinguish success, logical
violation, undefined evaluation, invalid/refused input, unsupported capability,
incomplete execution, inconclusive results and tool failure. Version output reports the app version. No verb exits
without its outcome: an accepted verb the build does not route is refused
`unsupported` with cause `operation-not-routed` naming the verb (FR-522), and
`analyze` reads `native-analyze/1` requests and writes its result as FR-520
states.

## Command and exit contract

CLI verbs mirror the lifecycle API and accept an explicit profile plus
explicit input/output formats. One total function maps each outcome
category to its exit code:

| Outcome category | Exit code |
| --- | --- |
| success: completed without violation | `0` |
| violation: a logical violation, including a claim whose evaluation is undefined | `10` |
| undefined: a value run (`run` of a function) whose value is undefined | `10` |
| pending: a temporal clause that declared progress could still resolve, including a supplied-trace clause pending when the trace ends | `0` |
| indeterminate: a temporal clause whose closed evidence leaves its verdict unresolved ([FR-091](../temporal/FR-091-evaluate-bounded-future.md)) | `11` |
| refusal: invalid or refused input | `20` |
| unsupported | `21` |
| incomplete: a limit, cancellation, deadline or budget | `22` |
| inconclusive | `22` |
| internal-failure, including tool failure | `30` |

A claim whose evaluation is undefined settles violation (refuted) with cause
`UndefinedEvaluation{where, cause}` and exits `10`. A value run whose value is
undefined exits `10` and its machine output keeps the label `undefined`. Multi-item machine output retains
per-item dispositions; the process code is the highest-severity present code
under this ordering: `30`, `20`, `21`, `22`, `10`, `11`, `0`.

Exit `11` is the next free code in the evaluation-result decade beside `10`: an
indeterminate item is neither a violation nor a success, so it takes its own code
rather than sharing `10` or `22`. It ranks below `10` so that an indeterminate item
never hides a violation, and above `0` so that it is never reported as success.
A `pending` result is its own category and exits `0`: nothing has failed and the
clause awaits progress. `inconclusive` is the proof and analysis category and
keeps `22`; `pending` is not inconclusive, and neither is indeterminate.

The table holds ten categories: success, violation, undefined, refusal,
unsupported, incomplete, pending, indeterminate, inconclusive and internal-failure,
with the exits `0`, `10`, `10`, `20`, `21`, `22`, `0`, `11`, `22` and `30`. The
label and the exit of a category are one pair: an implementation reads both from
one shared catalog of the ten categories and keeps no second mapping from an
outcome (a finite-window or other trace outcome included) to a label or an exit,
so two outcomes of one category exit alike whichever path produced them.

## Request surface

The CLI reaches these request-surface features through named commands, all
in the default build:

- `version` also reports the build inventory ([FR-560](FR-560-discover-the-build-inventory-through-version.md)).
- `analyze`, `prove`, `monitor`, `replay` and `generate` take
  `--provider-config <file>`, the process providers to start
  ([FR-561](FR-561-configure-process-providers-through-a-provider-configuration-document.md)).
- `compile` and `run` requests carry extension definitions in their
  `extensions` member, and `parse` and `check` take `--extensions <file>`
  ([FR-562](FR-562-name-extension-definitions-to-a-command.md)).
- `check --markdown` checks every native fence of a Markdown specification,
  and `run` runs one of its clauses through `program.extraction`
  ([FR-563](FR-563-check-rules-written-in-a-markdown-specification.md)).
- A `compile` request carrying a `handoff` member also writes the
  checked-package handoff directory, and `inspect --handoff <directory>`
  reads one back and checks it against its selection sidecar
  (FSL FR-665).
- `parse` reports the source labels, path, byte count and S1 usage of the
  unit it parsed, and every refusal record carries a `locus` (FSL FR-663).

Each of these refuses a malformed input document before any stage runs,
exit `20`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-301-AC-1 | Every lifecycle command round-trips its typed API request/result in machine-output mode. | Test (TC-221) |
| FR-301-AC-2 | Each outcome category produces its assigned stable exit status without parsing message text; a claim whose evaluation is undefined exits `10` as a violation with cause `UndefinedEvaluation`, a value run whose value is undefined exits `10` with the label `undefined`, a pending clause exits `0` with the category label `pending`, an indeterminate clause exits `11` with the category label `indeterminate`, a finite-window `Pending`, `Indeterminate`, success, violation and incomplete outcome carry the label and exit `pending`/`0`, `indeterminate`/`11`, `success`/`0`, `violation`/`10` and `incomplete`/`22`, the label and exit of a category are the same on the single-report path and in the multi-item ranking, a multi-item run with item codes {10, 11} exits `10`, {11, 22} exits `22` and {0, 11} exits `11`, and `version` reports the application version. | Test (TC-221) |
| FR-301-AC-3 | Unknown options/profiles/providers refuse before execution and identify the command argument locus. | Test (TC-221) |
| FR-301-AC-4 | A verb the build accepts but does not route exits `21` with an outcome document naming the verb under `unsupported_operation`/`operation-not-routed` (FR-522), and no verb exits with empty machine output while its stream is open. | Test (TC-1082) |
| FR-301-AC-5 | `version --format json` writes the build inventory document; `check --markdown` over a document of two native fences writes one item per fence; `compile` with a `handoff` member writes the handoff directory and `inspect --handoff` over it exits `0`; `--provider-config` naming a document with no `providers` member, and `--extensions` naming a set document with an unknown entry member, each exit `20` before any stage runs. | Test (TC-1244) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
