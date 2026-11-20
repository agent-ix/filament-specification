---
id: FR-192
title: "Execute linked verification plans"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/AD-009
    type: references
---

# FR-192: Execute linked verification plans

## Description

When executing a verification plan, the planner SHALL resolve every referenced
clause, method, domain, assumption, dependency, bound and evidence output before
dispatching a provider.

## Inputs

Verification plan, checked package, method-provider manifests and resource
policy.

## Outputs

Dependency-ordered method requests/results and evidence obligations, or typed
plan/link/cycle refusal.

## Behavior

Plans are linked artifacts outside the clause AST. Method selection cannot alter
source admission. Dependencies form an explicit graph; unsupported methods and
failed prerequisites preserve independent sibling results. Each output names the
requirement, assumption, method, provider and semantic subject it addresses.

## Plan execution graph

Each plan node binds one requirement/claim identity, semantic-subject identity,
method profile, provider identity/options, assumptions, limits and expected
evidence kind. Edges are typed `requires`, `validates`, `replays` or `aggregates`.
The coordinator topologically dispatches ready nodes, records a result for every
node and never substitutes one evidence kind for another. A failed `requires`
edge blocks only descendants; independent nodes still run. Aggregation applies
an authored decision rule after all required inputs have terminal dispositions.

## Source form

A plan is written with the shared grammar's `verification-plan`:

```
verify Release using p {
  check safe   claim Orders::NoDoubleRefund method model_check domain M::Order bound 20;
  check tested claim Orders::NoDoubleRefund method model_based_test domain M::Order bound 500;
  check timely claim Orders::RefundWithin   method model_check domain M::Order depends [safe];
}
```

- `verify P using p` SHALL declare plan `P` under profile alias `p`. The
  plan is a declaration of its unit, but it is not a clause and has no
  truth value.
- Each `check s ...` SHALL declare one plan node named `s`. Step names SHALL
  be unique within the plan; a repeated name refuses
  `conflicting_declaration`/`duplicate-declaration` at the second step,
  naming both.
- `claim C` SHALL resolve, by the unit's ordinary name resolution, to one
  clause declaration (state, temporal, hyper, hybrid, probabilistic, relation,
  state-graph or refinement claim). The node binds that clause's identity,
  never its text. A name that resolves to nothing, or to a declaration that is
  not a clause, refuses `missing_declaration`/`missing-name` at its span.
- `method M` SHALL name one method of the closed method vocabulary below.
  Another name refuses `missing_declaration`/`missing-name` at its span. A
  method whose FR-290 kind does not apply to the claim's declaration family
  refuses `invalid_capability`/`inapplicable-kind` at the step.
- `domain D` SHALL resolve to a type declaration or a model object type whose
  values bound the method's inputs: the population, parameter or initial-state
  domain the method draws from. A name that resolves to no such declaration
  refuses `missing_declaration`/`missing-name`; a type whose values do not
  convert to the claim subject's input type refuses `ill_typed`/`type-mismatch`.
- `depends [a, b]` SHALL add one `requires` edge from the step to each named
  step of the same plan. A name that is no step of the plan refuses
  `missing_declaration`/`missing-name`. A cycle of `requires` edges refuses
  `invalid_package`/`definition-cycle`, retaining the cycle's edges in path
  order (the fixed-point rule of FR-192-AC-3 is never selected by source).
- `bound n` SHALL be the method's bound in the unit the method counts
  (steps for a model checker, cases for `model_based_test`, plan length for
  `synthesis`). `bound 0` refuses `invalid_model_binding`/`malformed-declaration`
  at the bound. A step with no `bound` requests the method's unbounded extent,
  which FR-290 negotiation settles (a bounded-only candidate gives
  `requires-bound` or `unbounded-extent`). A step whose method is
  `model_based_test` and that has no `bound` SHALL settle `unsupported` with
  `unsupported_projection`/`unbounded-extent`, FR-290's pairing for an
  unbounded extent. The method is a language method (FR-190) and is never
  negotiated, so no candidate can supply the unbounded extent the step asks
  for.

| Method | FR-290 kind and extent | Evidence |
| --- | --- | --- |
| `model_check` | `temporal-satisfaction`, `bounded` with a bound, else `unbounded` | proof or refutation |
| `contract_check` | `operation-contract` | proof or refutation |
| `value_check` | `value-validity` | proof or refutation |
| `probabilistic_exact` | `probabilistic-satisfaction`, evidence `exact` | exact value or bound |
| `probabilistic_statistical` | `probabilistic-satisfaction`, evidence `statistical` | statistical estimate |
| `hybrid_solver` | `temporal-satisfaction` carrying the hybrid-dynamics requirement (FR-193) | solver result |
| `schedulability` | `schedulability` | proof or refutation |
| `model_based_test` | language method of [FR-190](FR-190-generate-model-based-tests.md) | `tested` evidence or refutation |
| `simulate` | language method of FR-181 | simulation runs |
| `replay` | `finite-replay` | replay disposition |

## Package form

A plan SHALL NOT be part of the checked package: compiling a unit with or
without its plans gives the same checked package identity. The compiler SHALL
emit each plan as one linked document of format `filament.verification-plan/v1`
holding:

- `plan`: the plan name, its package-local `qualified_name` (the declaring scopes, then the name) and its source location, which a plan request selects by (FR-824);
- `package`: the checked package identity the plan was checked against;
- `nodes`: one entry per step, in source order, with its step name, the
  bound clause's node identity and claim occurrence in that package, the
  method, its FR-290 kind, extent and evidence kind, the resolved domain's
  declaration key, and the bound or its absence;
- `edges`: one `{from, to, type: "requires"}` entry per `depends` name.

A reader SHALL refuse a plan document whose `package` differs from the package
it is executed against, naming both identities, so a plan result is never
bound to a changed clause.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-192-AC-1 | A complete acyclic plan dispatches methods in dependency order and accounts for every requested evidence output. | Test (TC-214) |
| FR-192-AC-2 | A missing reference/provider or unsupported method is reported per request without deleting independent work. | Test (TC-214) |
| FR-192-AC-3 | A circular dependency without a selected sound fixed-point rule refuses before provider execution. | Test (TC-214) |
| FR-192-AC-4 | Every plan node yields exactly one accounted disposition, including blocked, unsupported and not-run nodes. | Test (TC-214) |
| FR-192-AC-5 | Failure blocks only typed descendants and cannot erase or relabel independent sibling evidence. | Test (TC-214) |
| FR-192-AC-6 | An authored plan selects its subjects by requirement, clause and claim identity, and selects methods, bounds and evidence obligations against them; a plan that restates a clause body rather than referencing its identity refuses admission. | Test (TC-214) |
| FR-192-AC-7 | Editing a plan changes no clause meaning and no checked package identity, and editing a clause invalidates every plan result bound to its previous identity. | Test (TC-214) |
| FR-192-AC-8 | The `Release` plan checks to three nodes bound to the clause identities of `NoDoubleRefund` and `RefundWithin`, with methods `model_check`, `model_based_test` and `model_check`, bounds 20, 500 and none, and one `requires` edge from `timely` to `safe`; its emitted `filament.verification-plan/v1` document carries those nodes and that edge. | Test (TC-1601) |
| FR-192-AC-9 | A repeated step name refuses `conflicting_declaration`/`duplicate-declaration` naming both steps; a `claim` naming a type, a `method` named `fuzz`, a `domain` naming nothing and a `depends` naming no step each refuse `missing_declaration`/`missing-name` at the name. | Test (TC-1601) |
| FR-192-AC-10 | Steps `a depends [b]` and `b depends [a]` refuse `invalid_package`/`definition-cycle` retaining the edges `a -> b -> a`; `bound 0` refuses `invalid_model_binding`/`malformed-declaration`; `method schedulability` on a temporal claim refuses `invalid_capability`/`inapplicable-kind`. | Test (TC-1601) |
| FR-192-AC-11 | The same unit compiled with and without its `verify` declaration has the same checked package identity; a plan document executed against a package with another identity refuses naming both identities. | Test (TC-1601) |
| FR-192-AC-12 | The `Release` plan with its `tested` step written `check tested claim Orders::NoDoubleRefund method model_based_test domain M::Order;`, with no `bound`, settles `tested` `unsupported` with `unsupported_projection`/`unbounded-extent`, runs no case and writes no negotiation or provider entry for it; the step is never negotiated, and `safe` and `timely` run as before. | Test (TC-1601) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
