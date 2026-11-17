---
id: FR-818
title: "Qualify TLA+ and Lean exports against reviewed golden text and optional target oracles"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-815
    type: references
---

# FR-818: Qualify TLA+ and Lean exports against reviewed golden text and optional target oracles

## Description

When the TLA+ and Lean mappers are qualified, the qualification SHALL
compare each export of a fixed conformance corpus byte for byte with
reviewed golden text, and MAY additionally run the target tools as separate
processes to check that the text parses, elaborates and agrees with FSL's
own verdict; a target tool is never linked into an FSL build, and its
absence skips only the oracle check, with a printed reason.

## Inputs

The export conformance corpus and its golden files.

## Outputs

Per-model pass or fail for the golden comparison, and per-model oracle
observations when a tool is present.

## Behavior

### The corpus

The corpus SHALL hold at least: a counter with `inc` and `value <= 5`; a
two-process lock with mutual exclusion and weak fairness; FR-810's O1
rollout model; a two-role `fifo` protocol; a tick-timed model and a
dense-time model; a model exercising decimals, rationals, a floating-point
sum, text, sets, bags, maps and options; a clause with `until` and one with
a past operator; and one obligation per `unrepresented` construct of FR-811.
Each model carries its golden `.tla`, `.lean` and records files.

### Golden comparison

The export of each corpus request SHALL equal its golden target text and
records byte for byte. A golden file changes only by review, together with
the mapping rule or profile change that explains it (FR-816).

### Oracles

- When `java` with `tla2tools.jar` is present, the qualification SHALL run
  SANY on every TLA+ golden file and TLC on every finite model, comparing
  TLC's distinct-state count and verdict for each `preserved` obligation
  with FSL's explicit-state exploration of the same model (FR-651-AC-5).
- When `lean` is present, the qualification SHALL elaborate every Lean
  golden file and require no error and no `sorry`.
- A TLC counterexample for a seeded violating model SHALL read back through
  the name map and replay natively (FR-815-AC-4): the round trip.
- Each oracle SHALL run as a child process on a copy of the golden file. An
  absent tool skips that oracle with the printed reason `<tool> not found`;
  the golden comparison still runs. An oracle result never edits a golden
  file or a disposition.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-818-AC-1 | Every corpus export equals its golden TLA+ text, Lean text and records byte for byte; changing one mapping rule without updating the golden files fails the comparison. | Test (TC-1977) |
| FR-818-AC-2 | With TLC present, for the counter and the two-process lock TLC's distinct-state count and invariant verdict equal FSL's; with TLC absent the test prints `tlc not found` and the golden comparison still passes. | Test (TC-1977) |
| FR-818-AC-3 | With Lean present every Lean golden file elaborates with no error and no `sorry`. | Test (TC-1977) |
| FR-818-AC-4 | No FSL crate depends on a TLA+ or Lean library, and the oracles run only as child processes. | Inspection |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-815](FR-815-state-what-an-exported-proof-certifies.md),
  [FR-816](FR-816-version-the-tla-and-lean-target-syntax.md).
