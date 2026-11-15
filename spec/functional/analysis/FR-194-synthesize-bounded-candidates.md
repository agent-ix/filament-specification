---
id: FR-194
title: "Synthesize and independently check bounded candidates"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/AD-009
    type: references
---

# FR-194: Synthesize and independently check bounded candidates

## Description

When bounded synthesis is requested, the synthesizer SHALL search only the
declared candidate grammar/domain and keep candidate discovery separate from
independent correctness checking.

## Inputs

Checked specification, candidate grammar/domain, examples/constraints, search
bounds, synthesizer and independent checker profiles.

## Outputs

Candidate plus unchecked/checked status and proof/counterexample, unsatisfiable
within scope, incomplete, unsupported or failed result.

## Behavior

Search success yields an identified candidate, not a proof. The independent
checker consumes the candidate as a new subject under exact bounds. Search
exhaustion distinguishes proved no-candidate-in-domain from incomplete search.

## Search and validation contract

The request defines a finite candidate grammar, typed parameter domains,
symmetry rule, cost ordering and obligation. Search enumerates canonical
equivalence representatives in nondecreasing cost. A candidate becomes
`validated` only after an independently identified checker re-parses it and
discharges the obligation under the same subject and bounds. `no-candidate`
requires complete exhaustion of the declared canonical domain; any timeout,
provider loss or undisclosed pruning is incomplete.

## Source form

A synthesis request is written with the shared grammar's `synthesis-decl`:

```
synthesis Tariff using p grammar M::Tariff domain M::SmallTariff
  minimize Tariff.monthly
  satisfies { Tariff.monthly >= 10 and Covers(Tariff, M::Peak) };
```

- `grammar G` SHALL resolve to a type declaration (record, enum, union,
  tuple or alias, recursive or not), whose values are the candidate terms, or
  to a state model object type, which selects plan synthesis
  ([FR-650](FR-650-plan-and-optimize-over-a-state-model.md)). A name that
  resolves to neither refuses `missing_declaration`/`missing-name`.
- `domain D` SHALL resolve to a type declaration whose every value converts
  to a `G` value (`D` may be `G` itself). `D`'s value set is the search space.
  A `D` whose values do not convert to `G` refuses `ill_typed`/`type-mismatch`.
  For plan synthesis `D` is an integer range type whose values are the
  admitted plan lengths.
- **The bound.** `D`'s value set SHALL be finite: every numeric component has
  both bounds, every collection a maximum cardinality, every text a maximum
  length, and no value of `D` contains a value of `D`'s own declaration
  (recursion is unfolded only through a type with its own finite bound). A
  `synthesis` whose `D` is not finite SHALL refuse
  `unsupported_projection`/`unbounded-extent` at the `domain` name, naming the
  first component with no finite bound. Bounded synthesis searches only a
  finite space, and the source states that space.
- Inside the `satisfies` block and the objective, the declaration's own name
  SHALL denote the candidate, a value of type `D`. The block SHALL be a total
  Boolean expression (FR-004 rules); a non-Boolean root refuses
  `ill_typed`/`non-boolean-root`.
- The optional objective `minimize e` or `maximize e` SHALL be a numeric
  expression over the candidate. Without an objective the cost ordering is
  the canonical value order of `D`; with one, candidates are enumerated in
  nondecreasing (for `maximize`, nonincreasing) objective value, ties broken
  by the canonical value order. A non-numeric objective refuses
  `ill_typed`/`type-mismatch`. `minimize` and `maximize` are keywords only in
  this position.
- The symmetry rule of a source-written synthesis SHALL be the identity: no
  two distinct values are merged.

## Result

The synthesizer SHALL return exactly one of:

- `candidate`: the first satisfying candidate in the cost ordering, labelled
  `unchecked`, with the number of candidates visited; then the independent
  checker re-evaluates the `satisfies` block on the candidate as a
  `value-validity` item over the candidate value and labels it `validated`
  (the item settles `proved`) or `rejected` with the checker's counterexample
  and the synthesis lineage. With an objective, a `validated` candidate is
  also *optimal*: every candidate earlier in the cost ordering was visited and
  failed the block, and the result records that count;
- `no-candidate`: the whole finite space was visited and no value satisfies
  the block, a proof of absence within `D`;
- `incomplete`: a work, deadline or cancellation limit stopped the search,
  naming the FR-461 limit and the visited count, and, when one was found, the
  best candidate so far with no optimality claim;
- `unsupported` or `failed`, from a selected external synthesizer, as FR-331
  records them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-194-AC-1 | A found candidate is labeled unchecked until an independent checker proves the requested bounded obligation. | Test (TC-216) |
| FR-194-AC-2 | A failing candidate retains a replayable checker counterexample and synthesis lineage. | Test (TC-216) |
| FR-194-AC-3 | Complete no-candidate proof and resource-incomplete search remain distinct. | Test (TC-216) |
| FR-194-AC-4 | Canonical enumeration records the candidate grammar, domains, symmetry rule and cost ordering. | Test (TC-216) |
| FR-194-AC-5 | Validation uses an independently identified checker and the exact synthesis subject, assumptions and bounds. | Test (TC-216) |
| FR-194-AC-6 | `synthesis Pick using p grammar Digit domain Digit satisfies { Pick * Pick = 49 };` over `type Digit = Int[0, 9];` returns `candidate` 7, `unchecked` and then `validated`; with `satisfies { Pick * Pick = 50 }` it returns `no-candidate` after visiting all ten values. | Test (TC-1602) |
| FR-194-AC-7 | With `minimize Pick` and `satisfies { Pick >= 3 }` the result is the validated, optimal candidate 3 with two earlier candidates visited and failed; with `maximize Pick` it is 9. | Test (TC-1602) |
| FR-194-AC-8 | A `domain` naming `Integer` with no bounds, a record with an unbounded `Text` field, and a recursive union with no finite bound each refuse `unsupported_projection`/`unbounded-extent` at the `domain` name naming the unbounded component. | Test (TC-1602) |
| FR-194-AC-9 | A `grammar` naming nothing refuses `missing_declaration`/`missing-name`; a `domain` whose values do not convert to the grammar refuses `ill_typed`/`type-mismatch`; a `satisfies` block of type `Integer` refuses `ill_typed`/`non-boolean-root`; a `Text` objective refuses `ill_typed`/`type-mismatch`. | Test (TC-1602) |
| FR-194-AC-10 | A search stopped by its work limit returns `incomplete` naming the limit and the visited count, never `no-candidate`, and any best candidate it carries makes no optimality claim. | Test (TC-1602) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/filament-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
