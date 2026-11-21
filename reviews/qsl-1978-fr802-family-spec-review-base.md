---
id: SR-20261121082015
title: "Base specification review — FR-802 current planner objective family boundary"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/functional/analysis/FR-802-compute-plan-costs-exactly.md (Objective types, new current-behavior note) at 39331123035b8e5738f48816bc4390d86233ecb2, against FR-802 Exact arithmetic, Classification and AC-1..AC-8, spec/functional/analysis/FR-650-plan-and-optimize-over-a-state-model.md, FR-805, FR-808, FR-810, FR-193 and spec/functional/type-model/FR-149-apply-complete-equality-matrix.md"
review_set: base
---

## Summary

Ticket: QSL-1978. PR agent-ix/filament-specification#783 at frozen head
`39331123035b8e5738f48816bc4390d86233ecb2` adds one non-normative paragraph,
"Current FSL behavior", under FR-802 "Objective types". It says three things. The
planner supports `Integer`, `Rational` and `Decimal` objectives. A `Float32`,
`Float64` or unit-bearing `Quantity` objective returns a typed `unsupported` result
before search, naming the numeric family, and is never an undefined objective or an
infeasible result. And this boundary leaves the admitted objective types and the
arithmetic, `totalOrder`, NaN, unit-conversion and certificate requirements unchanged.
No other line of FR-802 changes.

## Verdict

**FAIL (needs one fix round)**: two medium findings, no high. Not mergeable as written.

What is right:

- The note uses no SHALL and says outright that it does not change the admitted
  objective types or the Exact arithmetic, `totalOrder`/NaN, unit-conversion or
  certificate rules. The float and quantity families stay required scope. They are
  not cut or waived.
- It invents no request member, wire code, cause, type or default. `unsupported` is
  the existing FR-805 plan-outcome status (exit `21`, `plan` `null`, zero `explored`
  counts when no search ran). Naming the family matches the generic idiom, in which an
  `unsupported` warning names the missing capability (FR-193).
- "Before search" and "never an undefined objective or an infeasible result" agree
  with FR-650 and FR-802. `infeasible` needs a complete exploration and an accepted
  closure certificate. `explored.undefined_objectives` counts plans that a search
  evaluated.
- `Float32` and `Float64` are the type model's binary floating-point types (FR-142,
  FR-149, FR-190). No other binary float type exists, so the list is complete.
- Literal typing (Evaluation order), FR-802-AC-1, AC-7 and AC-8 are byte-identical to
  `main`.
- The diff adds no hash, digest, pin, ledger or tracking record.

The note is not evidence that any implementation was tested. This review checked
spec text only.

## Review notes

No `AssuranceProfile` exists under `spec/`, so no review policy applies. `base` was
selected because the change is one informative paragraph. It does not change the FR
statement (no `spec-ears-analysis`), relationships (no `spec-dependency-analysis`),
domain objects or document structure. No other sub-analysis scope applies. Code
review and gap analysis do not apply because the diff touches only `spec/`.

## Scope

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-802 Objective types (new note) | examined | `Current FSL behavior: the planner supports Integer, Rational and Decimal objectives. A typed floating-point (Float32 or Float64) or unit-bearing Quantity objective returns a typed unsupported result before search, naming the numeric family; it does not produce an undefined objective or an infeasible result. This implementation boundary does not change the admitted objective types or the arithmetic, totalOrder, NaN, unit-conversion and certificate requirements below.` |
| FR-802 Objective types (admitted types) | examined | `The objective SHALL be numeric: Integer or an integer range, Rational, Decimal or a decimal range, a quantity with a unit (including Duration), or a binary floating-point type.` |
| FR-802 Exact arithmetic | examined | `A quantity objective SHALL be computed in the objective expression's unit ... A floating-point objective SHALL be computed under the package's selected floating-point profile ... Plans are compared by the profile's totalOrder predicate; a plan whose objective is a NaN is not admissible ...` |
| FR-802 Classification | examined | `An objective is a per-step sum when it is length, elapsed under tick time (FR-808), or an expression Reach.steps.sum(s => c) ...` |
| FR-802-AC-1, AC-7, AC-8 | examined | Unchanged from main: the Decimal literal-typing criteria. |
| FR-802-AC-3, AC-5 | examined | `An objective adding 30 s and 1 min per step compares in seconds ...`; `A Float64 objective that is NaN on one plan excludes that plan ...` |
| FR-650 Outputs/Result | context_only | `One planning result: optimal, feasible, infeasible or incomplete ...`; `infeasible: no admissible plan of any length in D: a closure certificate the core accepted` |
| FR-805 status/cause | context_only | `status is FR-650's optimal, feasible, infeasible or incomplete, or unsupported when no engine can decide the plan ...`; `cause ... on unsupported (FR-193's unsupported_projection/unsupported-requested-capability)` |
| FR-808 Tick time / Dense time | context_only | `elapsed is the number of O steps times p, an exact quantity of u.`; `elapsed SHALL be the exact sum of the plan's delays.` |
| FR-810-AC-3 | context_only | `O3 returns optimal with elapsed exactly 60 min ...` |
| FR-149 Duration row | context_only | `a Duration is a quantity in the unit s` |

## Findings

| ID      | Severity | Summary | Refs | Escape Cause |
| ------- | -------- | ------- | ---- | ------------ |
| FND-001 | medium | The note refuses every "unit-bearing `Quantity`" objective, which also covers `elapsed` and `Duration`. FR-808 makes `elapsed` an exact quantity of the model's unit, FR-149 makes `Duration` a quantity in `s`, and FR-802 Classification, FR-808-AC-2, FR-810-AC-3 (O3 makespan `60 min`) and FR-805-AC-7 all use a timed `elapsed` objective. Read literally, the note says timed makespan planning currently returns `unsupported`. If the planner handles `elapsed`, the note overclaims a refusal. If it does not, it silently marks the timed planning criteria unsupported. The note should say explicitly whether `elapsed`/`Duration` objectives fall inside the refused family. | spec/functional/analysis/FR-802-compute-plan-costs-exactly.md:45-51, :95-96; spec/functional/analysis/FR-808-plan-over-timed-models.md:40-52; spec/functional/analysis/FR-810-plan-and-optimize-the-six-reference-cases.md:74, :85; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:80 | wrong-requirement |
| FND-002 | medium | The note states present-tense current behavior ("the planner supports ...", "returns a typed `unsupported` result before search"). The PR description says this change does not claim runtime validation of the `unsupported` mapping and that the corresponding wiring correction is separate. If this paragraph merges before that implementation lands, the spec describes a current behavior that does not yet exist, for both the exact families and the float/quantity refusal. Merge it after the implementation that produces the behavior has merged, or narrow the wording so that it does not assert behavior not yet in place. Either way, do not change the normative family list. | spec/functional/analysis/FR-802-compute-plan-costs-exactly.md:45-49 | wrong-requirement |
