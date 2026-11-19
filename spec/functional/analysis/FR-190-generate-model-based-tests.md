---
id: FR-190
title: "Generate and shrink model-based tests"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/AD-009
    type: references
---

# FR-190: Generate and shrink model-based tests

## Description

When model-based testing is selected, the generator SHALL derive valid inputs
and shrink candidates from the exact model domains while using native execution
as the semantic oracle.

## Inputs

Checked package, model domains, preconditions, generator/shrinker profile, seed,
case limits and native runtime selection.

## Outputs

Case results, operation coverage, minimal retained counterexample and complete generation/replay
provenance.

## Behavior

Generation covers declared boundaries and relationships, including empty,
minimum, maximum and just-outside refusal inputs. Shrinking preserves domain
validity and the failing verdict. A rejected precondition is not a passing case.
Every retained counterexample serializes for native replay.

## Generation and shrinking contract

Generators are derived from checked domains ("Value domains") and
relationship constraints, not from implementation types. Partition
construction includes empty, singleton, each inclusive endpoint, adjacent
just-outside inputs and cross-field boundary combinations ("Partitions").
Seeded generation is rejection-free after domain construction: it draws only
domain members and applies only enabled invocations. Partition cases
deliberately include refusal and boundary values, and each one is a counted
case ("Case accounting"). An unsatisfiable domain is a model refusal. Shrinking follows a
pinned well-founded ordering over typed values and accepts a candidate only when
it remains in-domain and native replay preserves the same failing clause and
disposition.

## What is generated, and from which model

Model-based testing is a method the language runs over a checked package. It
SHALL be requested by a `verify` plan step whose method is `model_based_test`
([FR-192](FR-192-execute-linked-verification-plans.md) "Source form"), or by a
direct request carrying the same members. The request names one clause by its
identity, the generator profile, a seed, `max_cases`, for a clause over a
state model `max_trace_length`, and, when a generation root has no finite
bound ("Value domains"), `max_value_bits`.

- For a value clause or a pure Boolean predicate, a *case* SHALL be one typed
  assignment of the clause's parameters drawn from their checked domains.
- For a state clause, an operation contract, or a temporal clause over a state
  model, a case SHALL be one *test trace*: an initial state drawn from the
  subject's initial-state domain and a sequence of at most `max_trace_length`
  operation invocations of the model, each with arguments drawn from its
  parameter domains and each enabled (its precondition holds) at the state it
  is applied to. The trace's states are computed by the native reference
  runtime, never by the generator.
- The oracle SHALL be native evaluation of the named clause over the case: a
  value clause at its assignment; a state clause or operation contract at
  every state and step of the trace; a temporal clause over the finite trace
  under the clause's selected profile. A case passes, fails with the failing
  clause and disposition, or is discarded because a precondition or domain
  rule rejected it; a discarded case is counted and is never a pass. "Case
  accounting" states the check points, premises and discard reasons.
- A generation root SHALL be classified as "Value domains" states, and only
  a root with no finite bound takes `max_value_bits`. The generator SHALL
  NOT narrow a declared finite domain, and SHALL NOT invent a bound the
  request did not state.

## Value domains

A *generation root* is a value-clause parameter, an operation parameter
(path `[1, o, p]`), a state field (`[0, f]`) or a population, after the
composite decomposition of FSL's model transition system. An operation
result (`[2, o]`) is computed by the native runtime, never generated: it has
no partitions and no draws, but its domain for candidate results is chosen
by the same classes, so a result of a type with no finite bound takes the
size-bounded domain below. Each root falls into exactly one class:

| Class | Types | Domain used | `max_value_bits` |
| --- | --- | --- | --- |
| E, enumerated finite | `Boolean`, `Int[lo, hi]`, enums, `Reference<T>`, `Option` of these, and records, tuples, unions, collections and maps with a declared maximum whose components are all of class E | the declared domain, in the model transition system's enumeration order | not applied |
| D, declared finite | `Rational[..]`, `Decimal[..]`, `Text[..]`, `Float32`, `Float64`, `Uuid`, `Timestamp` and `Quantity<U>[min, max]` | the declared domain, indexed in its canonical value order (FR-804) and never materialized | not applied: a declared bound is never narrowed |
| S, no finite bound | `Integer`; a quantity over an unbounded exact carrier; `String`; `Bytes`; `JsonObject`; a collection or map with no declared maximum; a recursive value type | the size-bounded domain `S_b` below, `b` the request's `max_value_bits` | required |
| P, population with no maximum | `Population(None)` | the universe the `initial` snapshot admits; identities are never invented | not applied |

A class P root with no universe keeps the model transition system's
requires-bound refusal, naming the root. A class D root whose domain is too
large for per-state candidate enumeration stops the expansion on
`max_candidates`, an FR-461 limit stop: the run settles `incomplete` naming
that limit and the generator never shrinks the domain to fit.

**Size measure.** The bit measure is value accounting's `bits(x)`, the
magnitude bit length of a mathematical integer with `bits(0) = 1`. A value of
a class S type lies in `S_b` when every unbounded integer measure of the value
has `bits ≤ b`:

| Type | Unbounded measures |
| --- | --- |
| `Integer` | the value `n`, so `S_b` is `[-(2^b - 1), 2^b - 1]` |
| unbounded exact rational carrier | the reduced numerator and the positive denominator |
| unbounded decimal carrier | the coefficient `c` and the scale `k` of its canonical `(c, k)` form |
| `String` | the length in Unicode scalars |
| `Bytes` | the length in bytes |
| `JsonObject` | the length in bytes of its RFC 8785 form, which bounds every value inside |
| collection or map with no declared maximum | the element or entry count, and the measures of each element, key and value of class S |
| recursive value type | the occurrence count `occ(v)` of value accounting, and the measures of each class S component |

- `b` SHALL be an integer from 1 to 64. Any other value refuses the request
  `invalid_runtime_input`/`invalid-value` naming `max_value_bits`. A request
  with a class S root and no `max_value_bits` refuses
  `invalid_runtime_input`/`missing-member` naming it, and a request with
  `max_value_bits` and no class S root refuses
  `invalid_runtime_input`/`unknown-member` naming it, so the member is
  present exactly when it bounds something.
- When `S_b` is empty for a root, such as a `JsonObject` with `b` too small
  for its shortest RFC 8785 form, the request refuses
  `invalid_runtime_input`/`invalid-value` naming `max_value_bits` and that
  root, before any case runs. That is a defect of the bound, not of the model,
  so it is not the empty-domain `invalid_model_binding` refusal.
- *Size order* is the order of `S_b`: by the largest measure's `bits`
  ascending, then by canonical value order (FR-804). For `Integer` it is
  `0, -1, 1, -3, -2, 2, 3, -7, ..., 7, ...`. Its least element is the zero,
  empty or `none` value. Seeded draws index into `S_b` in size order, and for
  a class S root the shrinker's least element and one-step decrease
  ("Shrinking guarantee") are taken in size order; classes E and D keep the
  canonical value order.
- For a class S state field or operation result, `S_b` is its domain for
  candidate post-states and results. An enabled application whose
  postcondition has no candidate inside `S_b` is a *value-bound cut*: it
  yields no successor, it is counted in the provenance, and it is never read
  as a deadlock of the model.

## Partitions

Partition cases are generated before any seeded case. Roots are visited in
FR-331 rooted-domain order (populations, then fields `[0, f]`, then
parameters `[1, o, p]`), and each root's partitions are listed in the order
below. A boundary partition is generated only when the source type can
represent its value.

| Root | Partitions, in order |
| --- | --- |
| class E or D ordered scalar (`Int`, `Rational`, `Decimal`, `Float32`, `Float64`, `Timestamp`, bounded quantity) | `lower-refusal`, `lower-endpoint`, `upper-endpoint`, `upper-refusal`, with `singleton` in place of both endpoints when they are equal. A declared bound over several integer components (a rational's numerator and denominator, a decimal's coefficient and scale) adds one `component-refusal` per component bound, one past it with every other component at its least member. A float type adds `-0`, `+0`, `-inf`, `+inf` and `NaN` as `special` where it admits them, and has no refusal |
| class E or D length or count (`Text[min, max]`, a collection or map `[min, max]`) | `empty` when `min` is 0, else `lower-endpoint` (shortest, least content), `singleton` (length 1, when admitted), `upper-endpoint` (longest, least content), `lower-refusal` (length `min - 1`, when `min ≥ 1`), `upper-refusal` (length `max + 1`) |
| class E unordered (`Boolean`, enum, `Reference<T>`) | every member, as `member`, in enumeration order |
| `Option<T>` | `none`, then the partitions of `T` |
| class S | `size-least` (the least element), `size-unit` (each value whose every measure has `bits` 1, in size order, so `-1` then `1` for `Integer`), `size-extreme-low` and `size-extreme-high` (the least and greatest members of `S_b` in canonical value order, `-(2^b - 1)` and `2^b - 1` for `Integer`). There is no refusal partition: a value beyond `S_b` is a member of the type, so it is never generated and never reported as a refusal |
| cross-field | for each vector of `k ≥ 2` ordered components (an operation's parameters, the initial state's fields, a value clause's parameters), the `2^k` combinations of each component's least and greatest member, in binary order with the first component most significant, as `cross-field`. A combination that a relationship constraint excludes (an invariant over the fields, or a declared refinement relating the parameters) is a `relationship-refusal` |

A partition becomes a case as follows, and every partition case counts
toward `max_cases`:

- **Value clause.** The assignment holding the partition value at its root
  and every other root at its least member.
- **Field partition in the domain.** The first initial state in canonical
  order that holds the value; otherwise the shortest enabled trace, in
  canonical step order (FR-804), that reaches a state holding it within
  `max_trace_length`, found by the coverage exploration ("Operation
  coverage"). When no state within the bound holds the value, no trace can
  carry it, so no case is generated: the partition is `uncovered` with
  reason `unreachable`, or `unreachable-within-bound` with its `bounds`, as
  an operation would be.
- **Parameter partition `v` of operation `o` in the domain.** The shortest
  enabled prefix, in canonical step order, to a state at which an argument
  vector of `o` holding `v` is enabled, then that invocation (the first such
  vector in canonical order), the whole trace at most `max_trace_length`
  invocations long. When the exploration finishes without such a state, the
  case is still generated: the first initial state with the attempted,
  disabled invocation of `o` holding `v`, which is never applied. It is
  discarded `precondition-false`. When the exploration stops on a limit first,
  no case is generated and the partition is `uncovered` with reason
  `undetermined` naming the limit; that stop belongs to the coverage
  exploration and never turns the run `incomplete`. For a `precondition`
  clause of `o` the check point needs no enabled invocation ("Case
  accounting"), so the case is the first initial state with the argument
  vector holding `v` (every other parameter at its least member) and no
  invocation.
- **Refusal partition** (`lower-refusal`, `upper-refusal`,
  `component-refusal`, `relationship-refusal`). A *refusal case*: the
  generator submits the value to the native admission that guards the root
  (snapshot admission for a field or initial-state value, argument admission
  for an operation argument, parameter admission for a value clause), and
  the oracle requires that admission to refuse it. The required refusal
  discards the case `domain-refusal`. An admission that accepts the value
  disagrees with the declared domain, and the run settles `failed` with
  `tool-failure{defect: oracle-disagreement}` (FR-331).

## Case accounting

A *case* is a partition case, a seeded case or a refusal case: anything
charged to `max_cases`. Filtering disabled argument vectors out of a state's
expansion, and choosing among enabled successors, is domain construction: it
builds the enabled set a seeded trace draws from, it is not a case, and it is
not counted.

A case is evaluated at *check points*. A check point whose premise is false
makes no check: it is skipped and counted, and it never passes a case, as an
empty premise set makes no check of an operation-contract item (FR-485
"Common premises"). Within model-based testing the premises range over the
trace's states, not over every state of the declared types; that is the
difference between `tested` evidence and FR-485's proof.

| Clause kind | Check points of one case | Premises (the point is skipped when one is false) | Goal at a point |
| --- | --- | --- | --- |
| value clause or pure predicate | the one assignment | the clause's declared premise (a guard or refinement relating its parameters), if any | the clause is true. An undefined evaluation fails the case with `undefined-evaluation{where, cause}`; it is never a discard |
| state clause | every state of the trace, the initial state included | none | the clause is true at that state |
| temporal clause | the whole finite trace | none | the clause's verdict under its profile |
| `postcondition` of `o` | every applied step `(s_i, a_i, s_{i+1})` of `o` | FR-485 premises 1 to 4 at `(s_i, a_i)`. Premise 4 holds by construction, because only enabled invocations are applied; premise 3, the invariants at `s_i`, can be false at a reached state | with no implementation binding, the clause is defined at `(s_i, s_{i+1}, a_i, r_i)` (FR-485 "Clause"); with a binding, the post premise holds over the implementation's outcome |
| `precondition` of `o` | every state `s_i` of the trace with every argument vector `a` of `o`'s parameter domains | FR-485 premises 1 to 3 | the precondition is defined at `(s_i, a)`. A false precondition is a passing point; an undefined or undecided one fails the case |
| `frame` of `o` | as `postcondition` | as `postcondition` | FR-485 "Frame". With no implementation binding the clause has no contract-level reading, and the run settles `unsupported` `no-contract-level-reading` before any case, as the same item does outside model-based testing |

A case *fails* when any check point fails, *passes* when at least one check
point is evaluated and none fails, and is otherwise *discarded*. A discarded
case has exactly one reason, the first of these that applies:

| Reason | When |
| --- | --- |
| `domain-refusal` | a refusal case whose value native admission refused, as required |
| `precondition-false` | a parameter partition case whose invocation no explored state within the bound enables, or a case whose check points all had a false FR-485 premise 4 |
| `invariant-false` | a case whose check points all had a false FR-485 premise 3 at the reached state |
| `no-check-point` | a case in which the named clause has no check point, such as a contract clause of `o` over a trace that applies no `o` |

When a case's check points are skipped for different reasons, its reason is
the one of the first skipped point in trace order.

- `max_cases` SHALL count every generated case, discarded ones included, so
  `cases.generated = cases.passed + cases.failed + cases.discarded ≤ max_cases`
  and discards never make a run longer.
- `cases.discarded_by` SHALL list all four reasons in the order above, each
  `{count, first_case}`, with `count` 0 and `first_case` `null` for a reason
  no case has.
- `checks` SHALL be `{evaluated, skipped}`, where `skipped` maps
  `precondition-false` and `invariant-false` to the number of skipped check
  points.
- Each `partitions` entry SHALL be `{root, kind, value, status, ...}`, `root`
  `{node, path}`, in generation order. `status` is `covered` with
  `first_case` when an evaluated (passing or failing) case holds the value
  at its root; `refused` with `first_case` when its refusal case was refused
  as required; or `uncovered` with `reason` and `bounds`, the reason one of
  `precondition-false`, `unreachable`, `unreachable-within-bound`,
  `not-generated` or `undetermined` (the last naming the FR-461 limit).
  `partition_coverage` is `{status, uncovered}`, summarized as the operation
  `coverage` member is.
- A discarded case never covers an operation or a partition.
- The provenance SHALL record `max_value_bits` (`null` with no class S root);
  `size_bounded`, one `{root, measures, values}` per class S root in root
  order, `values` being `|S_b|` as a decimal string; and `value_bound_cuts`
  `{count, first}`, where `first` is the shortest enabled prefix in canonical
  step order to the first cut application together with that application's
  transition identity, or `null` when `count` is 0. No truncation of a domain
  is left out of the record.

## Result

The method SHALL return one result record holding the clause identity, the
generator profile, seed, every limit, the number of passing, failing and
discarded cases, the partitions covered, the operation coverage ("Operation coverage") and either no failure or one retained
counterexample. Its settlement is the member `result`, an FR-331 result
value (`tested`, `refuted`, `inconclusive`, `incomplete` or `failed`), with
`cause` where FR-331's result-entry table gives that value one. The method is
not negotiated and no provider writes it, so in a verification plan this
record is the `model_based_test` step's `result` (FR-824).

- A retained counterexample SHALL be a shrunk failing case that serializes as
  an FR-364 `TemporalCounterexample`, an FR-485 contract counterexample or a
  value assignment, and that native replay reproduces; only then does the
  clause settle `refuted`.
- With no failure, the clause SHALL NOT settle `proved`: testing establishes no
  proof, and the result is evidence of the kind `tested` with its case counts.
- Exhausting `max_cases` before any failure is a complete tested result, not
  an incomplete proof; a run stopped by a work or deadline limit before
  `max_cases` is `incomplete` with the FR-461 limit that stopped it.
- The record SHALL hold `cases` `{generated, passed, failed, discarded,
  discarded_by}`, `checks` `{evaluated, skipped}`, `partitions` and
  `partition_coverage`, and the provenance members, as "Case accounting"
  states.
- A completed run with no failing and no passing case, every generated case
  discarded, SHALL settle `inconclusive` with the cause `no-evaluated-case`
  (FR-331), never `tested` and never `proved`; its category is
  inconclusive, exit `22` (FR-301). `tested` evidence requires at least one
  passing case.
- A refusal case that native admission accepts, and a shrink step that
  leaves the domain or changes the failing clause, SHALL settle the run
  `failed` with `tool-failure{defect: oracle-disagreement}` and
  `tool-failure{defect: invalid-shrink}` respectively (FR-331), category
  internal failure, exit `30`, with no `tested` record.
- Discards, skipped check points and uncovered partitions never change a
  `tested` or `refuted` result: like operation coverage, they are evidence
  about the suite.

## Operation coverage

For a clause over a state model, the result record SHALL hold one
`operations` entry for each operation the subject's model declares, in the
model's canonical declaration order, so that a reader can tell which
operations the generated suite exercised. A value clause or a pure Boolean
predicate has no test trace, and its record has no `operations` member.

- **Covered.** An operation is *covered* when at least one evaluated case
  (passing or failing, never a discarded one) applies an invocation of it,
  with arguments in its parameter domains, at a state where it is enabled,
  within `max_trace_length` invocations of the initial state. The entry is
  `{operation, status: covered, first_case}`: `operation` is
  `{identity, name}`, the operation's checked declaration identity and its
  declared name (the name is for reading; entries are keyed by identity), and `first_case` is the index
  of the first case that applies it. A refused or discarded attempt to
  invoke an operation never covers it.
- **Uncovered.** Every other operation is *uncovered*. The entry is
  `{operation, status: uncovered, reason}`, and `reason` is exactly one of:

  | `reason` | Meaning | How it is established |
  | --- | --- | --- |
  | `unreachable` | no state reachable from the initial-state domain, at any trace length, enables the operation | exploration of the model's reachable state set (FSL's model transition system) finished without reaching a state that enables it |
  | `unreachable-within-bound` | no state reachable within the request's bounds enables it: `max_trace_length` invocations, and `S_b` when a root is of class S ("Value domains"); a longer trace or a larger `max_value_bits` may | exploration of every trace of at most `max_trace_length` invocations within those domains finished without such a state, and the reachable state set beyond the bounds was not exhausted |
  | `not-generated` | some state reachable within the bound enables it, but no evaluated case applied it before `max_cases` | the exploration reached an enabling state within the bound; the entry carries `enabled_at`, the shortest invocation prefix that reaches one, which native replay reproduces |
  | `undetermined` | exploration stopped before it decided reachability within the bound | the entry names the FR-461 limit that stopped it and the value used |

  `unreachable` and `unreachable-within-bound` are proved facts about the
  model under the request's bound; `not-generated` is a fact about the
  suite. A generator SHALL NOT report `unreachable` or
  `unreachable-within-bound` from the absence of the operation in the
  generated cases alone.
- **Bounds.** Every `unreachable-within-bound` entry, of an operation or a
  partition, SHALL carry `bounds`, the request members whose bound limited
  the exploration, in the order `max_trace_length`, `max_value_bits`:
  `max_trace_length` when the exploration reached the trace bound with a
  successor not yet visited, and `max_value_bits` whenever a root is of
  class S. With a class S root or a value-bound cut the exploration has
  searched only `S_b`, so `unreachable` SHALL NOT be reported: an operation
  it never found enabled is `unreachable-within-bound`. An `uncovered`
  partition with another reason carries the same `bounds`, which is `[]`
  when neither applies.
- **Exploration.** The coverage exploration runs from the same
  initial-state domain and parameter domains as generation, after the cases,
  under the request's work and deadline limits, and is part of the
  deterministic result: the same package, clause, profile, seed and limits
  give byte-equal `operations` entries.
- **Summary.** The record's `coverage` member is `{status, uncovered}`:
  `status` is `complete` when every entry is `covered` and `partial`
  otherwise, and `uncovered` maps each `reason` that occurs to its count of
  entries. A model with no operation has an empty `operations` array and
  `coverage` `{status: complete, uncovered: {}}`.
- **Verdict and exit.** Coverage is evidence about the generated suite, not
  about the clause, so it changes no verdict. A retained counterexample
  still settles the clause `refuted`; a run with no failure is still
  `tested` evidence with its case counts and settles no verdict, with
  `coverage` `partial` when any operation is uncovered. An uncovered or
  `undetermined` entry never turns a complete tested result into
  `incomplete` and never settles the clause, and the method's outcome
  category, and so its exit code, is the one FR-301 assigns to that
  verdict or evidence. A verification plan that requires complete coverage
  states it as a gate over `coverage` (FR-192).

## Shrinking guarantee

Shrinking SHALL reduce a failing case by the pinned order: first the trace
length (removing one invocation, or one maximal run of invocations), then
each argument and initial-state value toward the least element of its
domain in the canonical value order, or in size order for a class S root
("Value domains"). The guarantee is:

1. every accepted shrink step is strictly smaller in that order, so
   shrinking terminates;
2. every accepted candidate is domain-valid and, for a trace, every
   invocation is enabled where it is applied;
3. native replay of every accepted candidate fails the same clause with the
   same disposition;
4. the retained counterexample is *1-minimal*: no single removal of an
   invocation and no single one-step decrease of one value yields a
   candidate that satisfies 2 and 3.

A candidate that leaves the domain, disables an invocation or changes the
failing clause is rejected and never retained.

## Refusals

- A clause whose subject domain is empty SHALL refuse
  `invalid_model_binding`/`malformed-declaration` naming the empty domain,
  before any case runs.
- A request naming no clause, an unknown clause identity, a `max_cases` or
  `max_trace_length` of 0, or a seed that is not a 64-bit unsigned integer
  SHALL refuse the whole request before any case runs, naming the member.
  A request over a root whose domain has no finite bound and that
  carries no `max_value_bits` refuses the same way, naming that member, and
  so do the other `max_value_bits` refusals of "Value domains".
- A trace clause over a model that has no operation, and so has only
  traces of length 0, SHALL still run: its cases are initial states only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-190-AC-1 | Generated cases inhabit the exact declared domains and include required boundary partitions. | Test (TC-212) |
| FR-190-AC-2 | Shrinking terminates at a domain-valid counterexample with the same clause verdict. | Test (TC-212) |
| FR-190-AC-3 | Same package/generator/seed/options reproduce cases; invalid shrink or oracle disagreement is a method failure. | Test (TC-212) |
| FR-190-AC-4 | Generated partitions include every inclusive endpoint and adjacent refusal value representable by the source type. | Test (TC-212) |
| FR-190-AC-5 | Every shrink step is strictly smaller, domain-valid and replay-equivalent for the failing disposition. | Test (TC-212) |
| FR-190-AC-6 | Over a state model whose operation `inc` raises a counter in `Int[0, 5]` and a state clause `value <= 3`, a generated test trace applies only enabled invocations, its states are the native runtime's, and the retained counterexample is the 1-minimal trace of four `inc` invocations from `value = 0`, which native replay fails on the same clause. | Test (TC-1600) |
| FR-190-AC-7 | A run with no failing case reports its passing and discarded counts and the `tested` evidence kind, and the clause is never settled `proved`; a run stopped by its work limit before `max_cases` is `incomplete` naming that limit. | Test (TC-1600) |
| FR-190-AC-8 | A parameter typed `Integer` with no bound is generated by size-bounded enumeration up to the request's `max_value_bits`, and the bound appears in the provenance; a request without `max_value_bits` for such a parameter refuses naming the member. | Test (TC-1600) |
| FR-190-AC-9 | A clause over an empty domain refuses `invalid_model_binding`/`malformed-declaration` before any case; `max_cases` 0, `max_trace_length` 0 and an unknown clause identity each refuse the request naming the member. | Test (TC-1600) |
| FR-190-AC-10 | Two runs with the same package, clause, generator profile, seed and limits produce byte-equal result records, including the retained counterexample. | Test (TC-1600) |
| FR-190-AC-11 | Over a state model with `inc` raising `value` in `Int[0, 5]` (enabled when `value < 5`), `reset` (enabled when `value = 5`) and `jam` (enabled when `value > 5`), a run with `max_trace_length` 8 reports `operations` in declaration order: `inc` and `reset` `covered` with the index of the first case applying each, and `jam` `uncovered`, reason `unreachable`, naming `jam`'s declaration identity; `coverage` is `{status: partial, uncovered: {unreachable: 1}}`. `jam` is never reported `covered`. | Test (TC-2004) |
| FR-190-AC-12 | The same model with `max_trace_length` 3 reports `reset` `uncovered`, reason `unreachable-within-bound` with `bounds` `["max_trace_length"]`, and never `unreachable`; with a work limit too small to finish the bounded exploration, the entry is `undetermined` naming that limit, and the run's case counts and verdict are those of the unlimited run. | Test (TC-2004) |
| FR-190-AC-13 | A run whose suite passes with `jam` uncovered is `tested` evidence, settles no verdict and has the outcome category of a passing tested run; with a clause that fails, the same uncovered entry is reported beside the `refuted` counterexample. A value clause's record has no `operations` member, and two runs with equal inputs give byte-equal `operations` entries. | Test (TC-2004) |
| FR-190-AC-14 | Over a value clause with parameters `x: Integer` and `t: Text[0, 4]`, a clause every case passes, seed 7, `max_cases` 30 and `max_value_bits` 8: every case's `x` lies in `[-255, 255]`; `x`'s partitions are, in order, `size-least` 0, `size-unit` -1 and 1, `size-extreme-low` -255 and `size-extreme-high` 255, and no partition of `x` has a kind ending in `-refusal`; `t`'s partitions are `empty`, `singleton`, `upper-endpoint` (length 4) and `upper-refusal` (length 5); `provenance.max_value_bits` is 8 and `provenance.size_bounded` has exactly one entry, for `x`, with `values` `"511"`. With `max_value_bits` 1, `x` lies in `[-1, 1]` and `t`'s partitions are unchanged. `max_value_bits` 0 and 65 each refuse `invalid_runtime_input`/`invalid-value`, its absence `missing-member`, and `max_value_bits` 8 over a clause whose one parameter is `q: Int[0, 3]` `unknown-member`; each names `max_value_bits` and runs no case. | Test (TC-1600) |
| FR-190-AC-15 | Over a state model with `value` in `Int[0, 5]` (initial 0) and `inc(by: Int[1, 6])` (enabled when `value + by <= 5`), the postcondition clause of `inc`, seed 7, `max_cases` 100 and `max_trace_length` 8: cases 0 to 7 are the partitions of `value` (-1, 0, 5, 6) followed by those of `by` (0, 1, 6, 7); the cases for `value` -1 and 6 and `by` 0 and 7 are refusal cases, their partitions `refused`, and `cases.discarded_by.domain-refusal` is `{count: 4, first_case: 0}`; the `by` 6 case is discarded `precondition-false`, `discarded_by.precondition-false` is `{count: 1, first_case: 6}`, and its partition is `uncovered` with reason `precondition-false` and `bounds` `[]`; the `value` 0 case (the initial state, no `inc`) is discarded `no-check-point`; `cases.generated` is 100 and equals `passed + failed + discarded`; the run is `tested`. | Test (TC-1600) |
| FR-190-AC-16 | AC-15's model with an added operation `jam()` (enabled when `value > 5`): the postcondition clause of `jam` gives every case reason `no-check-point` or `domain-refusal`, `cases.passed` 0, and settles `inconclusive` with cause `no-evaluated-case`, exit `22`, never `tested` or `proved`; the precondition clause of `inc` has `discarded_by.precondition-false.count` 0 and the `by` 6 partition `covered`, because a false precondition is a defined, passing point; with a test-double argument admission that accepts `by` 7, the run settles `failed` with `{"type": "tool-failure", "defect": "oracle-disagreement"}`, exit `30`. | Test (TC-1600) |
| FR-190-AC-17 | Over a state model with `value: Integer` (initial 0), `add(by: Int[1, 100])` (`value' = value + by`) and `big()` (enabled when `value >= 300`), seed 7, `max_cases` 100, `max_trace_length` 8 and `max_value_bits` 8: `big` is `uncovered` with reason `unreachable-within-bound` and `bounds` `["max_value_bits"]`, never `unreachable`; `provenance.value_bound_cuts.count` is at least 1 and `first` names an `add` application from a state where `value + by` exceeds 255; the run is `tested`. With `max_value_bits` 10, `big` is `covered`, or `not-generated` with an `enabled_at` prefix of `add` invocations whose arguments sum to at least 300. | Test (TC-2004) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
