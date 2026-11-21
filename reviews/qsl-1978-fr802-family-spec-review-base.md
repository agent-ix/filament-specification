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
| FND-003 | fixed | Disposition pass 2, fix head `4739bf3bc942b765d90fbd6fbb0f9100d02bc9af`. The new "Plan-record type" rule says the producer emits the plan record and its state, step and arguments records "using the existing `composite_type`/`record` form" (the declared-record form that FR-322 already lists in its dependency and family rules), as generated declared records with the source unit's `SourceOwner` and `declaration` occurrences. The ordinary structural-identity, owner, dependency and source-map rules apply, and the rule reserves no name, path or literal. The candidate's `semantic_type` "is the exact reference identifying the plan record"; the reader resolves it and checks the field names, presence and types against FR-800, and "never finds a record by a display name or substitutes another record with equal fields". The comparison target of reader step 3 is now defined. No new tag, form or key is added. |
| FND-004 | fixed | Disposition pass 2. The `candidate` bullet now reads "the generated plan-record type node for a state-model or hybrid plan synthesis", and `grammar` stays "`model`/`object_type` or `hybrid`/`hybrid_model`, respectively". "Plan-record type" says a hybrid plan uses the same record forms with FR-819's extensions. A hybrid candidate's type is now stated: it is the plan record, not `G`. The residual hybrid step-field typing is recorded separately as FND-007. |
| FND-005 | fixed | Disposition pass 2. The new "FR-322-AC-467" subsection gives the complete source unit (`model Config = "example/config-version"; type Lengths = Int[0, 3]; synthesis Reach using p grammar Config::ConfigVersion domain Lengths satisfies { true };`), so `Lengths` is declared. TC-2037 adds step 4 (the plan-synthesis read-back of `grammar`, `domain`, `candidate` and the referenced plan record's node, body, declaration and owner) and step 5 (the `ConfigVersion`-retyped candidate with a rebuilt parameter key and references, so that only synthesis-reader step 3 can refuse, `invalid_package`/`invalid-value` at `candidate`), with expected results for each. |

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

## New findings (disposition pass 2)

Scope added in this pass: only the fix-round-2 text, which is the FR-322
"Plan-record type" rule, the changed `candidate` bullet, the
"FR-322-AC-467" subsection, and TC-2037 steps 4 and 5 with their expected
results. These were read against the FR-322 declared-record and object-type
field-binding rules, FR-800 "The plan value", FR-808, FR-809 and FR-819. FR-802
is byte-identical to fix head `dc3092b7`. This is not a full FR-322 audit.

| ID      | Severity | Summary | Refs | Escape Cause |
| ------- | -------- | ------- | ---- | ------------ |
| FND-006 | medium | "Plan-record type" introduces one shared arguments record for all operations: "The arguments record's fields carry the declared operation parameter names and types; a field absent for an operation is optional in the shared arguments record." It does not cover two operations that declare a parameter with the same name and different types or presence, for example `drain(s: Reference<Server>)` and `resize(s: Integer)`. One record field `s` cannot carry both types. One reader could refuse such a package, another could accept either type, and a producer has no rule to emit. FR-800 says only "a record with one field per parameter", so the collision comes from the shared-record choice. The rule should state the collision case: a per-operation arguments record (for example a union keyed by `operation`), a defined merged type, or an explicit refusal or `unsupported` outcome. | spec/objects/interfaces/FR-322-checked-package-artifact.md:5974-5980; spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md:76-80 | wrong-requirement |
| FND-007 | low | The step-record base fields (`index`, `operation` of the model's operation enum, `arguments`, `before`/`after`) are stated only "For a state-model plan". For a hybrid plan, the rule says the step's `operation` carries "its transition target mode" and that "Extension field types are those the owning requirement specifies". FR-819 gives no type for that `operation` field (a mode value), no `arguments` shape, and no `index` or `before`/`after` for hybrid steps. Under "no field is typed by guessing its name", a reader cannot check a hybrid step record's `operation` type or its base field set. AC-467 and TC-2037 exercise only the state-model case. | spec/objects/interfaces/FR-322-checked-package-artifact.md:5974-5998; spec/functional/analysis/FR-819-plan-over-hybrid-models.md:44-55 | wrong-requirement |

### Disposition pass 2 verdict

**FAIL as written (one medium, one low new finding); FND-001 to FND-005 are
fixed.** This was the last fix round allowed, so no third review cycle is
opened here. The outcome of FND-006 and FND-007 is left to the dispatching
lead: either another fix, or `deferred` to a named follow-up ticket. This
review does not assign that outcome.

What is sound and can stand on its own:

- The plan candidate is an exact reference to a generated `composite_type`/`record`
  node under the ordinary declared-record identity, owner, dependency and
  source-map rules, with no reserved name or path and no new tag, form, key,
  request member or refusal code. `grammar` references the actual `G` node,
  and `domain` references `D`.
- The plan fields `length`, `initial`, `final` and `steps`, and the
  state-model step fields `index`, `operation`, `before` and `after`, match FR-800,
  as do the timed and probabilistic extensions.
- The value-synthesis case is unchanged.
- The AC-467 plan fixture uses the existing `example/config-version`
  package. Its source is complete, and its true goal and absent objective
  keep the refusal oracle away from field-projection typing.
- TC-2037 step 5 isolates the synthesis reader's candidate-type refusal and
  claims no source-compilation or runtime evidence.

The finding-free path is the state-model plan record over a model where no
two operations share a parameter name with different types. The AC-467
fixture is on that path. A follow-up must specify the arguments-record
collision rule (FND-006) and the hybrid step-record field types (FND-007).
