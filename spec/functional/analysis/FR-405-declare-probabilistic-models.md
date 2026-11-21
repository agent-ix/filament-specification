---
id: FR-405
title: "Declare random parameters, workloads and rewards"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-013
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-205
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-406
    type: references
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
  - target: ix://agent-ix/filament-specification/FR-640
    type: references
---

# FR-405: Declare random parameters, workloads and rewards

## Description

When a package declares a random parameter, a workload or a reward over a
model's operations, the frontend SHALL admit the declaration only in the exact
form below and SHALL bind it to the model subject before any probabilistic
claim over that subject is evaluated.

## Inputs

A checked package with a state model and its operations (FR-013, FR-181); the
native `random`, `reward` and `workload` declarations of the shared grammar's
probabilistic forms ([shared grammar](../../../proposals/filament-v1/shared-grammar.md));
exact rational literals (FR-205) and quantities (FR-142).

## Outputs

The model subject with each operation parameter classified as random or
non-random, each random parameter's distribution, each operation's rewards and
each declared workload, or a located typed refusal naming the declaration and
the violated rule.

## Behavior

### Random parameters

A random parameter declaration names one parameter of one operation of the
model and gives a distribution: a finite list of (value, weight) pairs. Each
weight is a positive exact rational. Each value is a literal of the
parameter's declared type, and the values are pairwise distinct under the
parameter type's equality. The probability of a value is its weight divided by
the sum of the distribution's weights. The frontend refuses a repeated value
with `invalid_probabilistic_model`/`repeated-support-value`, a weight that is
not a positive exact rational with `invalid_probabilistic_model`/`invalid-weight`,
and a value outside the parameter's type with
`invalid_probabilistic_model`/`value-outside-type`, each at the value's locus.

The set of a random parameter's values is its **support**. The support is the
parameter's finite domain for every purpose of FR-181: successor enumeration,
the requires-bound pre-check and the transition identity. A random parameter
has exactly one distribution: the frontend SHALL refuse a second `random`
declaration of a parameter that already has one with
`invalid_probabilistic_model`/`duplicate-distribution` at the second
declaration's locus, naming the operation and the parameter.

An operation's precondition and guard read no random parameter. Its
postcondition and its rewards may read random parameters. The frontend SHALL
refuse a precondition that reads a random parameter with
`invalid_probabilistic_model`/`random-read-in-precondition` at the read's source locus.

### Rewards

A reward declaration names one operation and gives a reward name and an
expression. The expression's type is an integer type or a quantity type
(FR-142), and it reads only the operation's parameters, random parameters
included. It reads no state (`invalid_probabilistic_model`/`state-read-in-reward`
at the read's locus), and a type other than integer or quantity refuses with
`invalid_probabilistic_model`/`reward-type`. The reward's value is non-negative for every
argument vector in the operation's domains; the frontend checks this over the
finite domains and refuses a negative value with
`invalid_probabilistic_model`/`negative-reward`,
naming the operation, the reward and the argument vector, the first negative
value in canonical argument order (a random parameter ranging over its
support). The check belongs to the frontend, not to a provider: the reward
reads only the operation's parameters and constants, so its sign is decided
by the declaration alone, and refusing it before the package is checked means
every checked package, and every provider and certificate checker reading
one, holds only non-negative rewards. A reward that reads a parameter whose
type has no finite domain (an unbounded integer, bounded only by a subject's
proof bounds) is checked by the same rule and refusal when the provider
admits the subject with those bounds, the first point at which its domain is
finite. Reward names
are unique per operation: a second reward of one name on one operation refuses
with `invalid_probabilistic_model`/`duplicate-reward` at the second
declaration, naming the operation and the reward. One reward name declared on
several operations has one type across them. A transition carries the reward values of its operation
under its arguments.

### Workloads

A workload declaration names one model and gives every operation of that model
exactly one non-negative exact rational weight. The frontend SHALL refuse a
workload that omits an operation (`invalid_probabilistic_model`/`incomplete-workload`),
weights an operation twice (`invalid_probabilistic_model`/`duplicate-weight`), names an
operation of another model (`invalid_probabilistic_model`/`unknown-operation`) or gives a
negative weight (`invalid_probabilistic_model`/`invalid-weight`).

A weight of 0 is admitted. It states that the workload does not choose that
operation while an operation of positive weight is enabled; at a state where
every enabled operation has weight 0, the workload chooses among them
uniformly ([FR-406](FR-406-interpret-probabilistic-models.md)). A workload
whose weights are all 0 is therefore admitted and chooses uniformly among the
enabled operations at every state. The reason for this reading is that a
workload is a scheduler of the model: it may prefer operations, but it never
invents a deadlock the model does not have, so every behaviour of positive
probability under a workload remains a behaviour of the model. Omitting an
operation stays refused `incomplete-workload`, so that a misspelt or forgotten
operation is never read as weight 0; a workload that means to exclude an
operation writes its weight as 0.

### Delay distributions

Over a `time dense` model ([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md))
an operation may be the subject of one model-level `delay M::o ~ D;`
declaration (`delay-decl` of the shared grammar), `D` one of the families of
[FR-420](FR-420-sample-stochastic-timed-models.md). An operation with no
`delay` declaration has a nondeterministic delay. A `delay` declaration on an
operation of an untimed or digital model refuses with
`invalid_probabilistic_model`/`delay-on-untimed-model`, and a second `delay`
declaration of one operation refuses with
`invalid_model_binding`/`malformed-declaration` at the second declaration.

A **scheduled identity** is a transition identity without its random
arguments: (operation, receiver, non-random argument vector). On the wire it
has the FR-181 `TransitionIdentity` shape with only the non-random
arguments in `arguments`; a sampled witness step carries it as `scheduled`
and its random arguments as `random` (FR-410).

### Checked-package form

An admitted random parameter, workload, reward and delay declaration is
carried in the checked package as a probabilistic-model node
([FR-640](../../objects/interfaces/FR-640-carry-probabilistic-model-declarations-in-the-checked-package.md)),
so a package that declares them is emitted and read like any other checked
package, and a provider reads them from the package instead of from the
source.

### Identity

Random-parameter distributions, delay distributions, rewards and workloads are part of the model
subject's identity. A probabilistic claim's obligation identity includes the
workload it names ([FR-407](FR-407-state-probabilistic-claims.md)). Changing a
weight, a support value or a reward expression changes the identity of every
obligation that reads it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-405-AC-1 | A random parameter with distinct typed values and positive exact rational weights is admitted, each value's probability is its weight over the weight sum, and its support is its FR-181 enumeration domain. | Test (TC-350) |
| FR-405-AC-2 | A precondition or guard that reads a random parameter refuses `invalid_probabilistic_model`/`random-read-in-precondition` at the read's locus. | Test (TC-350) |
| FR-405-AC-3 | A distribution with a repeated value, a non-positive weight, or a value outside the parameter's type refuses `repeated-support-value`, `invalid-weight` or `value-outside-type` under `invalid_probabilistic_model` at the value's locus. | Test (TC-350) |
| FR-405-AC-4 | A reward reading state, typed other than integer or quantity, or negative for some argument vector refuses with its typed cause; an admitted reward is carried by every transition of its operation. | Test (TC-350) |
| FR-405-AC-5 | A workload that omits, duplicates or misnames an operation, or gives a negative weight, refuses with its typed cause; a complete workload is admitted. | Test (TC-350) |
| FR-405-AC-10 | `workload Steady on Service` with `Server::reset: 0` and weight 1 on the other two operations is admitted and records weight 0 for `Server::reset`; a workload with weight 0 on every operation is admitted; `Server::reset: rational(-1, 1)` refuses `invalid_probabilistic_model`/`invalid-weight` naming the workload and `Server::reset`; the workload with `Server::reset` omitted still refuses `incomplete-workload`. | Test (TC-1560) |
| FR-405-AC-11 | `reward refund on Shop::buy = n - 3` with `n: Int[0, 5]` refuses `invalid_probabilistic_model`/`negative-reward` naming `Shop::buy`, `refund` and `n = 0` when the package is checked, so no checked package is produced and no provider is reached; `reward refund on Shop::buy = n` is admitted. | Test (TC-1560) |
| FR-405-AC-6 | A `delay` declaration is admitted on an operation of a `time dense` model and refuses `invalid_probabilistic_model`/`delay-on-untimed-model` on an untimed or digital model; a second `delay` declaration of one operation refuses `invalid_model_binding`/`malformed-declaration`; an operation with no `delay` declaration has a nondeterministic delay. | Test (TC-350) |
| FR-405-AC-7 | Changing one weight, support value or reward expression changes the identity of every obligation that reads it. | Test (TC-350) |
| FR-405-AC-8 | A second `random` declaration of one operation parameter refuses `invalid_probabilistic_model`/`duplicate-distribution` at the second declaration's locus, naming the operation and the parameter. | Test (TC-350) |
| FR-405-AC-9 | A second `reward duration on Server::attempt = d;` after the first refuses `invalid_probabilistic_model`/`duplicate-reward` at the second declaration's locus, naming `Server::attempt` and `duration`; `reward duration` on `Server::attempt` and on `Server::request` with one value type is admitted. | Test (TC-350) |

## Dependencies

- [FR-013](../foundation/FR-013-validate-operation-frames.md) and
  [FR-181](../runtime/FR-181-simulate-finite-executions.md) define the
  operations, their finite domains and the successor relation the support
  enters.
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) types
  quantity rewards; [FR-205](../../objects/foundation/FR-205-exact-rational-value.md)
  types weights.
- [FR-406](FR-406-interpret-probabilistic-models.md) gives these declarations
  their probabilistic meaning.

## References

- FSL ADR-024 PM-1, PM-2, PM-5; FSL ADR-026 SD-1.
- Ticket: STD-137 (paired with QSL-371).
