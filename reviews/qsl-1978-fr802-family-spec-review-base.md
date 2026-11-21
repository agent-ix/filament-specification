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

## Dispositions

Disposition pass 1, against fix head `dc3092b7a4994e3ab203b23f35dac450f959afd2`
("Correct plan candidate typing and scope adapter boundary"). Spec text only;
no implementation or runtime behavior was inspected or claimed.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The note now says: "On this route, the quantity boundary includes `Duration` and typed `elapsed` objectives; it makes no claim about other timed-planner entrypoints." The timed-objective question is answered explicitly, and the note no longer reads as marking all timed planning unsupported. The normative family list, FR-808, FR-810-AC-3 and FR-805-AC-7 are unchanged and still apply. |
| FND-002 | fixed | The note is now titled "Source-plan adapter implementation boundary". It says "the adapter wiring is being corrected to support ..." and "This note describes the wiring correction, not released or runtime-validated behavior." It no longer asserts current or released behavior. The admitted objective types and the arithmetic, `totalOrder`, NaN, unit-conversion and certificate requirements "remain applicable". |

## New findings (disposition pass 1)

Scope added in this pass: the authorized FR-322 "Synthesis-request nodes"
`candidate` clause and the plan-synthesis sentences of FR-322-AC-467, read
against FR-322 reader step 3 (Joins), FR-800 "The plan value", FR-650,
FR-819 and TC-2037. This is not a full FR-322 audit.

| ID      | Severity | Summary | Refs | Escape Cause |
| ------- | -------- | ------- | ---- | ------------ |
| FND-003 | medium | The changed `candidate` clause types a plan candidate by "the generated plan-record type node ... whose members FR-800 lists", but nothing in this repository defines that node. No node form, `node_id` path, owner, occurrence, emission rule or member schema is given (FR-800's `length`/`final`/`initial`/`steps`, plus the conditional `elapsed`/`probability` and the step record). "plan-record" appears nowhere else in `spec/`. Reader step 3 requires the candidate's `semantic_type` to be "the node this list requires", so for a plan synthesis the reader has no definition of the node to compare against. Two readers could admit different packages: one could accept any record type node with FR-800's fields, another only a node at a fixed path. AC-467's refusal oracle exercises only retyping to the `ConfigVersion` grammar node, so the exact-readback claim has no defined target. The clause should name how the plan-record type node is identified, using the existing record type node form and no new tag or key, or reference the owning FR that defines it. | spec/objects/interfaces/FR-322-checked-package-artifact.md:5916-5923, :5965-5967, :6409; spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md:63-81 | wrong-requirement |
| FND-004 | low | The `grammar` bullet separates "a plan synthesis (FR-650)" from "a hybrid plan synthesis (FR-819)". The changed `candidate` bullet types only "a value synthesis" and "a plan synthesis", and adds "The plan candidate is not typed by `G`". A hybrid plan synthesis's candidate type is now unstated: either the plan-record type node or the `hybrid`/`hybrid_model` `G` node. Reader step 3 gives no answer. | spec/objects/interfaces/FR-322-checked-package-artifact.md:5908-5923; spec/functional/analysis/FR-819-plan-over-hybrid-models.md | wrong-requirement |
| FND-005 | low | FR-322-AC-467's new plan case names its fixture ("plan synthesis `Reach` over `Config::ConfigVersion` with domain `Lengths`") without source text. `Lengths` is declared nowhere in `spec/`, unlike the value case, which spells out `type Digit = Int[0, 9];` and the declaration. The AC still cites TC-2037, and TC-2037's procedure and expected results cover only `Pick`/`Pick2` and the AC-468 edits: no step reads the plan synthesis, checks the plan-record candidate readback, or feeds the `ConfigVersion`-retyped candidate. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6409; spec/test-cases/TC-2037-a-synthesis-declaration-is-one-synthesis-request-node.md | wrong-requirement |

### Disposition pass 1 verdict

**FAIL (needs fix round 2).** FND-001 and FND-002 are fixed. The new FR-322
plan-candidate scope adds one medium finding (FND-003) and two low findings
(FND-004, FND-005).

What is right in the new FR-322 scope:

- The architectural direction is consistent. `grammar` still references the
  actual `G` node and `domain` still references `D`. The plan candidate is no
  longer typed by `G`, which matches FR-800, where the declaration's name
  denotes the plan value inside the block and the objective, not a model
  object. This removes the earlier `G`-typed contradiction.
- Body shape, binding order, literals, `dependencies` and the `claim_names`
  row are unchanged. The change adds no node tag, `semantic_form`, key, request
  member, source grammar or refusal code. The wrong-type refusal reuses the
  existing reader step 3 code and locus (`invalid_package`/`invalid-value` at
  the `candidate` binding).
- The value-synthesis case and FR-322-AC-468 are byte-identical to `main`.
- No implemented-reader evidence is claimed. The AC remains a Test criterion
  with no result attached.

The FR-1036 writer patch belongs to another repository and is not reviewed
here.
