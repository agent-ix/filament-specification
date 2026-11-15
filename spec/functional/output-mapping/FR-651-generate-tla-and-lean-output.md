---
id: FR-651
title: "Generate TLA+ and Lean 4 output with per-obligation fidelity"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-120
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-121
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-297
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-269
    type: references
---
## Description

When `filament.output.tla2/v1` or `filament.output.lean4/v1` is selected,
the mapper SHALL translate the requested state models, operations and clauses
of a checked package into TLA+ or Lean 4 text under the common output-mapping
contract, and SHALL classify every requested obligation's fidelity with an
FR-269 disposition, never claiming more than the translation preserves.

## Inputs

An admitted common request (FR-120) selecting one of the two profiles; the
checked package; the requested obligation identities; and the common
resource limits.

## Outputs

Deterministic text regions of one TLA+ module or one Lean 4 file, and one
FR-298 record per requested obligation, in one FR-299 generated package.

## Behavior

### Target versions

- `filament.output.tla2/v1` SHALL emit TLA+ version 2 as the TLA+ Tools
  distribution 1.8.0 parser (SANY) accepts it, using only the standard
  modules `Naturals`, `Integers`, `Sequences`, `FiniteSets` and `TLC`, and,
  for a fairness obligation, the TLA+ `WF_vars` and `SF_vars` operators.
- `filament.output.lean4/v1` SHALL emit Lean 4 source for the Lean 4
  toolchain version the profile's FR-297 `target_standard_refs` names,
  using only the Lean core library; no Mathlib dependency is required.

### What is exported

| Source declaration | TLA+ | Lean 4 |
| --- | --- | --- |
| State model object type and population | `VARIABLES` with a type invariant over finite sets of keys and record values | a `structure State` and its well-typedness predicate |
| Initial-state snapshot or initial constraint | `Init` | `def Init : State → Prop` |
| Operation with precondition and postcondition | one action per operation, `Next` their disjunction with existential argument choice | `def Step : State → State → Prop` as the disjunction of operation relations |
| State clause and invariant | a state predicate and `THEOREM Spec => []Inv` | `def Inv : State → Prop` and `theorem inv_holds : ∀ s, Reachable s → Inv s` stated without proof |
| Temporal clause, future operators with no interval | the LTL formula over `Spec`, fairness as `WF`/`SF` | a predicate over `Nat → State` behaviours |
| Temporal clause with intervals under a step-counting profile | the formula with an auxiliary step counter | a predicate over indexed behaviours |
| Dense-time clock, timed model | an explicit real-time encoding with a `now` variable | a predicate over timed behaviours |
| Hyperproperty over behaviours | self-composition of `Spec` | a predicate over pairs of behaviours |
| Hybrid model and claim | the mode and discrete-jump skeleton with flows as uninterpreted relations | a predicate over hybrid behaviours with flows as `Prop` relations |
| Probabilistic model and claim | the nondeterministic support abstraction | the support relation |

- Every exported definition SHALL carry, as a comment, the source
  declaration's identity and span, so each text region maps back to its
  source obligation.
- Lean theorem statements are exported as statements; the mapper writes no
  proof and no `sorry`-free claim. A proof a user writes against the export
  is external evidence and enters Filament only as `trusted` evidence.

### Fidelity

Each requested obligation SHALL receive exactly one FR-269 disposition:

- `preserved`: the target text has the source meaning: finite-domain state
  models, operations, state clauses, invariants, unbounded future temporal
  clauses with their fairness, and step-counting intervals through the
  auxiliary counter;
- `conditional`: the target text has the source meaning only under the
  retained condition the record names: the real-time encoding of a timed
  clause (condition: non-Zeno behaviours), self-composition of a
  hyperproperty with two trace quantifiers of one kind (condition: the
  composition's fairness), and a safety claim over a probabilistic support
  abstraction (condition: the claim holds with probability 1 exactly when it
  holds on the support);
- `unrepresented`: the target cannot state the fact and the record retains
  it: flow equations of a hybrid model (exported as uninterpreted
  relations), probabilistic thresholds other than probability 1, and
  hyperproperties with quantifier alternation;
- `refused`: a request member that is not an obligation of the package.

The mapper SHALL NOT classify an obligation as `preserved` because an external
tool accepted the text. An external parser or checker (SANY, TLC, Apalache,
Lean) is an observer: its result is recorded in the package's observer
section and never changes a disposition or settles a claim.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-651-AC-1 | A counter state model with operation `inc` and a state clause `value <= 5` exports under `filament.output.tla2/v1` as a module with `VARIABLES`, `Init`, an `inc` action, `Next` and an invariant theorem, each region carrying its source identity and span, every obligation `preserved`; the module parses under SANY 1.8.0 as an observer. | Test (TC-1604) |
| FR-651-AC-2 | The same package under `filament.output.lean4/v1` exports `structure State`, `Init`, `Step`, `Inv` and the theorem statement with no proof, every obligation `preserved`; the file elaborates under the named Lean 4 toolchain as an observer. | Test (TC-1604) |
| FR-651-AC-3 | A timed clause exports `conditional` naming non-Zeno behaviours; a hybrid claim's flows export `unrepresented`; a probabilistic threshold 0.9 exports `unrepresented` and a probability-1 safety claim `conditional`; no record is `preserved` for these. | Test (TC-1604) |
| FR-651-AC-4 | A request for an identity that is no obligation of the package refuses that member; an observer that rejects the text changes no disposition; exporting the same request twice gives byte-equal packages. | Test (TC-1604) |
| FR-651-AC-5 | On the export conformance corpus of finite models, TLC's reachable state count and invariant verdict over the TLA+ export equal FSL's explicit-state exploration over the same model for every `preserved` model obligation. | Test (TC-1604) |

## Dependencies

- [Common output-mapping contract](../../../proposals/filament-v1/output-mappings/common.md).
- [FR-120](FR-120-select-output-mapping-profile.md), [FR-121](FR-121-account-for-every-output-obligation.md), FR-297, FR-298, FR-299 and FR-269.
