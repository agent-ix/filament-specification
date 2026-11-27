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
  - target: ix://agent-ix/filament-specification/FR-817
    type: references
  - target: ix://agent-ix/filament-spec-language/FR-010
    type: references
---

# FR-301: Expose the complete native CLI lifecycle

## Description

When a user invokes the FSL CLI, the command SHALL map parse, check, lower,
execute, replay, monitor, export, inspect, format and version requests to the exact Rust
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
| unsupported (codes `unsupported_construct` and `unsupported_projection` among them) | `21` |
| incomplete: a limit, cancellation, deadline or budget, or a population whose membership is not established (code `incomplete_population`) | `22` |
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
- `analyze`, `run` and `monitor` take `--require-certified`, which sets the
  request's `require_certified` member `true` (FR-520; FR-824 and FR-820 for
  `native-run/1`). Without the flag the request is read as written; the flag
  never sets the member `false`. No other verb takes it.
- `compile` and `run` requests carry extension definitions in their
  `extensions` member, and `parse` and `check` take `--extensions <file>`
  ([FR-562](FR-562-name-extension-definitions-to-a-command.md)).
- Every request that compiles a program (`compile`, `run`, `analyze`, `prove`
  and `monitor` requests) carries the integer-division law in its
  `integer_division` member (top level for the program, and on each library
  entry for that library), and `check` takes `--integer-division <law>`, where
  `<law>` is `euclidean`, `floor` or `truncating`. There is no default law;
  the law composes with a unit's header law as
  [FR-570](../expressions/FR-570-select-the-integer-division-law-from-a-unit-header.md)
  states, and an unknown law name refuses `unknown_profile`, cause
  `unsupported-selection`, exit `20`, before any source is read.
- `check --markdown` checks every native fence of a Markdown specification,
  and `run` runs one of its clauses through `program.extraction`
  ([FR-563](FR-563-check-rules-written-in-a-markdown-specification.md)).
- A `compile` request carrying a `handoff` member also writes the
  checked-package handoff directory for the compiled unit: the package
  bytes, the sources and domain package documents, the selection sidecar,
  the identity and equality files and, when asked, the adverse mutation
  corpus. The directory must not exist beforehand; an existing one is
  refused by path and left untouched. `inspect --handoff <directory>` reads
  the package back with the strict reader, using the evidence files beside
  it, and checks it against the selection sidecar: it exits `0` when they
  agree and `20` with the reader's or the sidecar's typed refusal
  otherwise.
- `export <request>` runs the TLA+ or Lean export of a `native-run/1`
  request's `export` member and writes the outcome document with operation
  `export`; `--output <file>` names the target file, which the language
  implementation writes only for an assembled package and never over an
  existing path, and `export --evidence <request>` takes in a target tool's
  result over an assembled export as FR-815 states, both as
  [FR-817](../output-mapping/FR-817-request-and-write-a-tla-or-lean-export.md)
  states. `--output` and `--evidence` are accepted on `export` only (`--output`
  also on `compile`); `--evidence` with `--output` refuses `invalid-request`
  naming both, exit `20`.
- `parse` reports, for a parse that completes, the source's two labels as
  the caller gave them, its path, its exact byte count and, for each S1
  limit, the bound and the count reached; it reports no other identity of
  the bytes. Every refusal record, a document diagnostic or an item's, carries
  a `locus`: a source position, a JSON document position (pointer, object
  and member), or `null` when the refusal has no position.

Each of these refuses a malformed input document before any stage runs,
exit `20`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-301-AC-1 | Every lifecycle command round-trips its typed API request/result in machine-output mode. | Test (TC-221) |
| FR-301-AC-2 | Each outcome category produces its assigned stable exit status without parsing message text; a claim whose evaluation is undefined exits `10` as a violation with cause `UndefinedEvaluation`, a value run whose value is undefined exits `10` with the label `undefined`, a pending clause exits `0` with the category label `pending`, an indeterminate clause exits `11` with the category label `indeterminate`, a finite-window `Pending`, `Indeterminate`, success, violation and incomplete outcome carry the label and exit `pending`/`0`, `indeterminate`/`11`, `success`/`0`, `violation`/`10` and `incomplete`/`22`, the label and exit of a category are the same on the single-report path and in the multi-item ranking, a multi-item run with item codes {10, 11} exits `10`, {11, 22} exits `22` and {0, 11} exits `11`, and `version` reports the application version. | Test (TC-221) |
| FR-301-AC-3 | Unknown options/profiles/providers refuse before execution and identify the command argument locus. | Test (TC-221) |
| FR-301-AC-4 | A verb the build accepts but does not route exits `21` with an outcome document naming the verb under `unsupported_operation`/`operation-not-routed` (FR-522), and no verb exits with empty machine output while its stream is open. | Driver test (TC-1082): only the CLI binary decides verb routing, so the driver code ticket QSL-1995 adds the test in the driver repository. |
| FR-301-AC-5 | `version --format json` writes the build inventory document; `check --markdown` over a document of two native fences writes one item per fence; `compile` with a `handoff` member writes the handoff directory and `inspect --handoff` over it exits `0`; `--provider-config` naming a document with no `providers` member, and `--extensions` naming a set document with an unknown entry member, each exit `20` before any stage runs. | Test (TC-1244) |
| FR-301-AC-6 | `check --integer-division floor` over a unit with `div` and no law header exits `0`; without the option it exits `20` with `missing_declaration`/`missing-selection`; `--integer-division ceiling` exits `20` with `unknown_profile`/`unsupported-selection` naming the option and the value before reading the source; a `run` request whose `integer_division` member is `floor` evaluates `-7 div 3` to `-3`; and `--integer-division` on `compile`, `run`, `analyze`, `prove` or `monitor` refuses as an unknown option, exit `20`. | Driver test (TC-1651): the CLI binary parses the option and renders the outcome, so the code ticket adds the test in the driver repository. |
| FR-301-AC-7 | `export <request> --output m.tla` over the counter request exits `0`, writes the target file and writes an outcome document with operation `export`; `export <request> --evidence e.json --output m.tla` and `export --evidence e.json --output m.tla` each refuse `invalid-request` naming the options, exit `20`, with no file written; `--evidence` on `compile` and `--output` on `run` each refuse as an unknown option, exit `20`. | Driver test (TC-2021): the CLI binary parses the options and routes the verb, so the code ticket adds the test in the driver repository. |
| FR-301-AC-8 | `analyze --require-certified` over FR-520-AC-9's request without the member writes the same outcome document as the request with `"require_certified": true` and no flag; `--require-certified` on `check` refuses as an unknown option, exit `20`. | Driver test (TC-2116): the CLI binary parses the option, so the code ticket adds the test in the driver repository. |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
- FSL FR-665 implements the handoff write and `inspect --handoff`, and FSL
  FR-663 the parse report and refusal locus, against the behaviour stated
  under "Request surface".
