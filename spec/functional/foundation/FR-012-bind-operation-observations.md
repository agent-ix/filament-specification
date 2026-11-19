---
id: FR-012
title: "Bind operation observations to one invocation"
type: FR
relationships:
  - target: "ix://agent-ix/filament-specification/US-003"
    type: traces_to
---
# FR-012: Bind operation observations to one invocation

## Description

When an operation clause is evaluated, the runtime validator shall select observations from the same declared invocation.

## Inputs

Operation identity, parameters, result, pre/post references, context identity and universe.

## Outputs

Validated current/pre/post context or correspondence diagnostics.

## Behavior

Preconditions select pre; postconditions select post. result is post-only. Proposed pre(expression) selects pre for context-dependent reads while preserving already captured lexical values. A deleted required post self is unavailable as a post context even if deletion is permitted.

Operation parameters are immutable values from the selected invocation, checked
against that operation's declared parameter types. Selecting another observation
does not replace a parameter with a similarly named object field.

A precondition or postcondition reads a parameter by its bare name or by the
qualified name `<op>::p`, where `<op>` is the clause's operation name spelled
exactly as the clause's `on` authors it (for example `M::T::op::p`). The
qualified name always denotes the parameter's invocation value, whatever `let`
or quantifier binder is in scope; the bare name resolves the innermost binder
of that name first and denotes the parameter only when no binder of that name
is in scope. A parameter inside `pre(...)`, bare or qualified, refuses
`wrong_snapshot`/`forbidden-pre-read` (FR-012-AC-7). The qualified
name is the one spelling of a parameter whose name is a reserved word of the
edition, such as `by`: domain models declare parameters and fields with one
identifier class, the shared grammar's `member-name` admits a reserved word
after `::`, and the grammar has no escape form, while a bare reserved word is
refused `invalid_syntax`/`unexpected-token`. The qualified name never denotes
an object field of the same name, another operation's parameter, or anything
in an invariant; a last segment naming no parameter of the clause's operation,
another operation's prefix, and the form in an invariant each refuse
`missing_declaration`/`missing-name` at the name. The proposed
initial anchor rule admits pre(expression) only inside a postcondition.

A pre or post snapshot MAY carry one optional member, `invocation`, holding the
four identity labels `authority`, `identity`, `revision_namespace` and
`revision` of the one invocation document it was taken for. When the member is
present and the four labels differ from the identity labels of the invocation
document the snapshot is read with, the snapshot is refused
`wrong_snapshot`/`wrong-invocation`. When it is absent, the snapshot is unbound
and is read as before, with no check. 

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-012-AC-1 | Different pre/post invocation identities receive wrong_snapshot. A snapshot whose `invocation` member names the four identity labels of another invocation document receives `wrong_snapshot`/`wrong-invocation`; one whose `invocation` member equals the document's four labels, and one with no `invocation` member, are read unchanged. | Test |
| FR-012-AC-2 | A missing required pre observation receives unavailable_observation. | Test |
| FR-012-AC-3 | A post-only result used in current context receives wrong_snapshot. | Test |
| FR-012-AC-4 | A captured post reference retains its observation inside pre. | Test |
| FR-012-AC-5 | A precondition reading self.versionNumber evaluates against pre even when post contains a different value for the same object. | Test |
| FR-012-AC-6 | An operation parameter retains the invocation-supplied value in both precondition and postcondition evaluation. | Test |
| FR-012-AC-7 | Applying pre to an expression that reads state beside an immutable parameter, in a postcondition, preserves that parameter's value. A parameter inside pre, written by its bare or its qualified name, refuses `wrong_snapshot`/`forbidden-pre-read` (FR-042-AC-4). | Test |
| FR-012-AC-8 | Using pre in an invariant or precondition receives wrong_snapshot at linking. | Test |
| FR-012-AC-9 | For an object type `Meter` with field `by: P` and operations `adjust(by: P, step: P)` and `reset(by: P)`, `P` a decimal type of scale 2, and one invocation of `adjust` supplying `by = 1.00` and `step = 2.00` whose pre and post `Meter` has field `by = -1.00`: the precondition and the postcondition `M::Meter::adjust::by >= 0.00` each complete `true`, reading the parameter and not the field. `M::Meter::adjust::step = step` completes `true`. `let step = M::Meter::adjust::by in M::Meter::adjust::step > step` completes `true`, the qualified name reading the parameter `2.00` and the bare `step` the binder `1.00`. In a postcondition of `adjust`, `pre(M::Meter::adjust::by) >= 0.00` refuses `wrong_snapshot`/`forbidden-pre-read` at the `pre`. `M::Meter::adjust::nope >= 0.00` and `M::Meter::reset::by >= 0.00` in a precondition of `adjust`, and `M::Meter::adjust::by >= 0.00` in an invariant on `M::Meter`, each refuse `missing_declaration`/`missing-name` at the name. The bare precondition body `by >= 0.00` refuses `invalid_syntax`/`unexpected-token` at `by`. | Test (TC-2083) |

## Examples

These are specification hypotheses for the criteria above, not domain
packages or observed evaluator output. Assume one resolved operation declares
an expectedVersion parameter with the same bounded integer type as
self.versionNumber. One invocation supplies expectedVersion = 1. Its pre and
post snapshots contain the same object with versionNumber = 1 and 2 respectively.
Both snapshots satisfy the declared population and frame validation contracts.

| Clause context | Expression | Expected completed value | Criteria |
| --- | --- | --- | --- |
| Precondition | self.versionNumber = expectedVersion | true | FR-012-AC-5, FR-012-AC-6 |
| Postcondition | self.versionNumber = expectedVersion | false | FR-012-AC-6 |
| Postcondition | pre(self.versionNumber = expectedVersion) | true | FR-012-AC-7 |
| Postcondition | pre(expectedVersion) | refused `wrong_snapshot`/`forbidden-pre-read` | FR-012-AC-7, FR-042-AC-4 |

The corresponding precondition containing pre(expectedVersion) refuses at
linking under FR-012-AC-8, so it has no completed Boolean.

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
