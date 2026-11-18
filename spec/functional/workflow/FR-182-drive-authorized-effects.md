---
id: FR-182
title: "Drive authorized idempotent workflow effects"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-012
    type: implements
  - target: ix://agent-ix/filament-specification/AD-008
    type: references
  - target: ix://agent-ix/filament-specification/FR-822
    type: references
---

# FR-182: Drive authorized idempotent workflow effects

## Description

When an application accepts a typed formal decision, the workflow boundary SHALL require explicit authority and an idempotency identity before requesting an external effect.

## Inputs

Decision/result identity, action parameters, authority grant, idempotency key,
adapter manifest and current receipt state.

## Outputs

Rejected recommendation, effect request plus attributed receipt, or typed
adapter failure/incomplete outcome.

## Behavior

Evaluators emit recommendations only. The adapter validates authority scope,
binds one decision to one idempotency key and records request/receipt identities.
Retries reconcile the same request. A transport receipt is not a business effect;
effect observations re-enter through the observation authority contract.

## Effect-adapter protocol

The evaluator emits an immutable recommendation. A human or policy decision
binds its identity, actor, authority grant, decision time and approved effect
parameters. The adapter derives one idempotency key from adapter identity,
decision identity and effect identity, persists the pending request before
transport, and reconciles retries against that record. A reused key is valid
only for byte-identical canonical request content. Transport receipts update
delivery state; separately observed domain events establish business effect and
re-enter evaluation as new evidence. No evaluator or analyzer invokes an
adapter directly.

This is the observe-or-recommend boundary of every model run: running a model (evaluation, simulation, exploration, analysis and replay)
applies a protocol `attempt`, `effect` or `commit` only to the model state the
run holds, and every result it returns (a verdict, a counterexample, a
simulated behaviour, an enabled step) is a recommendation. The language
implementation that runs models SHALL perform no external business side
effect, start no process, open no network connection and hold no adapter.

The run-wide boundary composes with the source-level observe-only boundary of
[FR-822](../protocol/FR-822-declare-observe-only-protocol-boundaries-and-typed-recommendations.md):
a role or control written `observe` additionally leaves the model state the
run holds unchanged, its `recommend` controls emit typed recommendation
records naming the action they recommend, and a run that selects `boundary`
`observe-only` treats every step of the protocols it runs as observe-only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-182-AC-1 | A valid authority and fresh idempotency key issue one typed request and retain an attributed receipt. | Test (TC-211) |
| FR-182-AC-2 | Duplicate delivery with the same decision/key does not request a second external effect. | Test (TC-211) |
| FR-182-AC-3 | Missing/mismatched authority, reused key with different inputs or receipt/effect substitution refuses. | Test (TC-211) |
| FR-182-AC-4 | Crash-and-retry after pending-request persistence reconciles the same request without issuing a distinct effect identity. | Test (TC-211) |
| FR-182-AC-5 | Only an attributed domain observation can establish the business effect; recommendation, decision and transport receipt cannot. | Test (TC-211) |
| FR-182-AC-6 | Exploring and simulating a protocol whose `attempt` applies an operation that writes a model field changes only the returned model states; the run issues no adapter request, starts no process and opens no connection, and the language implementation's evaluation and analysis components have no dependency on an adapter, process or network facility. | Test (TC-1523) and Inspection |
| FR-182-AC-7 | Simulating a protocol whose observe-only role `Auditor` holds a `recommend` of an operation that writes a model field, under the default run mode, returns behaviours whose `recommend` steps carry recommendation records and keep the model state, while an `attempt` by another role of the same operation changes the returned model states; the run issues no adapter request for either; under `boundary` `observe-only` the same protocol refuses at the `attempt` before any step. | Test (TC-2019) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
