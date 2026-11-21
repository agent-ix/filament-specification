---
id: SR-2026112106460400000001
title: "Base specification review — FR-802 step-sum decimal literal typing"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/functional/analysis/FR-802-compute-plan-costs-exactly.md (Evaluation order, new step-sum literal sentence; Classification; FR-802-AC-1) at ef26b98ca429bb0d0822ec595298c6b52551ac9e, against spec/functional/type-model/FR-140-evaluate-exact-decimals.md, spec/functional/foundation/FR-662-read-a-bare-decimal-literal-as-an-exact-decimal.md and spec/functional/analysis/FR-194-synthesize-bounded-candidates.md"
review_set: base
---

## Summary

PR agent-ix/filament-specification#780 at frozen head
`ef26b98ca429bb0d0822ec595298c6b52551ac9e` adds one sentence to FR-802 "Evaluation
order": inside a step sum, a `decimal(c,s)` literal's unique expected type under
FR-140 is "the summand family at the objective's scale", and otherwise it refuses
`ill_typed`/`ambiguous-literal`. The refusal half matches FR-140 exactly. The typing
half names a type that no FR-140, FR-194 or FR-802 rule defines, and for the one
criterion it is meant to support (FR-802-AC-1, whose summand leaves are all literals)
it either refuses, which contradicts AC-1, or infers a scale from literal spellings,
which FR-140 forbids. The new rule has no acceptance criterion. Classification, the
sum-plus-constant rule and the certificate rules are untouched by the diff.

## Verdict

**FAIL** — one high and one medium finding. Not mergeable as written.

What is right: the refusal spelling `refused { code: ill_typed, cause: ambiguous-literal }`
is FR-140's exact code and cause; the link path resolves to FR-140; the sentence sits
in Evaluation order and does not alter the per-step-sum classification, the
`sum + constant` rule, non-negativity, or the `potential`/`closure` certificate split;
the diff adds no hash, pin, digest or tracking record. A literal constant added to a
step sum (`Reach.steps.sum(s => c) + 0.5`) lies outside "within a step sum" and is
already typed by FR-140's "other operand's static type" rule, so that case stays as it
was. `quire validate` of FR-802 at the frozen head exits 0 with no diagnostics.

## Review notes

No `AssuranceProfile` exists under `spec/`, so no review policy applies; `base` was
selected because the review is scoped to this one sentence. The changed text is
a Behavior paragraph, not the FR-802 Description statement, so `spec-ears-analysis`
does not apply; no other sub-analysis scope (relationships, objects, structure)
changed.

FR-140's unique-expected-type sources are a closed list: the other operand's static
type when that operand is not a literal; a declared field, parameter, result or
annotated type of the position; a `convert` target; or the unique expected type of an
enclosing `+`, `-`, `*`, `/` or unary `-`. FR-140 ends: "no coefficient range, scale,
rounding mode or target scale is inferred from the literal spellings". A FR-140 type is
a full `Decimal[lo, hi; smin, smax; mode]`; operands of distinct decimal types are
`type-mismatch`. FR-194's grammar (`synthesis-objective = ('minimize' | 'maximize'),
expr`) gives the objective no declared type, so the objective's type and scale are
derived from its expression.

Two readings of the new sentence follow, and they diverge:

1. "Summand family at the objective's scale" means the static type of the summand's
   non-literal parts. Then FR-140 already gives every literal next to a typed operand
   its type, the sentence adds nothing, and FR-802-AC-1
   (`if s.operation = M::Op::charge then 0.10 else 0.05`, no non-literal decimal
   anywhere) refuses `ambiguous-literal`, which contradicts AC-1's expected `0.25`
   `Decimal` with a `potential` certificate.
2. It means a `Decimal` type whose scale is the scale the objective "has". With every
   leaf a literal, the only scale available is the one spelled by `0.10`/`0.05`, which
   is the inference FR-140 forbids, and it still fixes no `lo`, `hi`, `smin` or `mode`,
   so it is not a unique FR-140 type.

Possible fixes, for the author and architecture owner to choose from: (a) name a
concrete source, for example an expected type that `if` branches and the step-sum term
pass down from a non-literal sibling, and make FR-802-AC-1 carry a typed operand,
field or `convert` target so its literals have one; or (b) give the objective a declared
type and state that it is the literals' expected type. Either way, add an AC with the
positive case and the `ambiguous-literal` refusal.

## Scope

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-802 Evaluation order (new sentence) | examined | `Within a step sum, the type checker SHALL use the summand family at the objective's scale as a decimal(c,s) literal's unique expected type under FR-140, refusing refused { code: ill_typed, cause: ambiguous-literal } when no unique expected type exists.` |
| FR-802 Classification | examined | `An objective is a per-step sum when it is length, elapsed under tick time (FR-808), or an expression Reach.steps.sum(s => c) (optionally plus a constant) whose term c reads only the step record s, and the objective's type is an exact type.` |
| FR-802-AC-1 | examined | `minimize Reach.steps.sum(s => if s.operation = M::Op::charge then 0.10 else 0.05) over three steps evaluates to exactly 0.25 (Decimal), with no binary rounding, and the certificate is potential.` |
| FR-140 Decimal typing paragraph | context_only | `A decimal(c,s) literal takes exactly its unique expected type: the other operand's static type when that operand is not a literal, a declared field, parameter, result or annotated type of its position, a convert target, or the unique expected type of the enclosing decimal operation ... no coefficient range, scale, rounding mode or target scale is inferred from the literal spellings.` |
| FR-140-AC-7 | context_only | `a decimal(c,s) literal without a unique expected type is ill_typed with cause ambiguous-literal.` |
| FR-662 Behavior | context_only | `The decimal primary takes its Decimal[..] type from its position as the constructor does ... a position that gives no type, or expects a type that is not a Decimal[..], refuses exactly as the constructor refuses there.` |
| FR-194 objective rule | context_only | `The optional objective minimize e or maximize e SHALL be a numeric expression over the candidate.` |

## Findings

| ID      | Severity | Summary | Refs | Escape Cause |
| ------- | -------- | ------- | ---- | ------------ |
| FND-001 | high | "Summand family at the objective's scale" is not a type that FR-140, FR-194 or FR-802 defines. The objective carries no declared type, so its scale comes from the summand itself. For FR-802-AC-1, whose summand leaves are all literals, the sentence either refuses `ambiguous-literal`, which contradicts AC-1, or infers a scale from literal spellings, which FR-140 forbids. It also fixes no `lo`, `hi`, `smin` or `mode`. | spec/functional/analysis/FR-802-compute-plan-costs-exactly.md:71-75, :106; spec/functional/type-model/FR-140-evaluate-exact-decimals.md:130-137 | wrong-requirement |
| FND-002 | medium | The new SHALL has no acceptance criterion. No AC states the expected type a step-sum literal takes, or the `ambiguous-literal` refusal inside a step sum, so neither half can fail a test. | spec/functional/analysis/FR-802-compute-plan-costs-exactly.md:71-75, :104-111 | correct-requirement-no-evidence |

## Dispositions

Disposition pass 1, checked against the fix commit `dcad029b6bb6fe8279433fe103cc4249bd311428`
(FR-802 "Evaluation order" and FR-802-AC-1, AC-7, AC-8), read against FR-140 "Decimal typing",
FR-140 membership, FR-501 item 3 (annotated `let`), FR-146 "Case result type" item 1, FR-149 and
FR-322 "Integer containment" (expected type passes through `if` branches), FR-145 `sum`, FR-662 and
TC-185 D25 (`type P = Decimal[..];` and `convert<P>(..)` source forms).

| FND | Outcome | Reason |
| --- | ------- | ------ |
| FND-001 | fixed | The undefined "summand family at the objective's scale" is gone. The summand type now comes from the summand expression without reference to the objective, a literal's expected type comes only from FR-140's own sources (typed operand, declared or annotated position, `convert` target, enclosing typed decimal operation), passed through `if` branches as FR-146, FR-149 and FR-322 already pass an expected type, and neither literal spelling nor the objective supplies the type, its bounds, scales or mode. AC-1 now types its summand by the FR-501 annotated `let cost: Decimal[0,10000;2,2;exact]`: `0.10` and `0.05` are members (`c*` 10 and 5 at `s*` 2), the total `0.25` (`c*` 25) has that complete type, and `lo = 0` keeps the per-step sum non-negative, so `potential` holds. No FR-140 exception or objective type is introduced. |
| FND-002 | fixed | FR-802-AC-7 is the positive case: with `type Cost = Decimal[0,10000;2,2;exact];`, `convert<Cost>(0.10)` summed over three steps is exactly `0.30` with the complete `Cost` type and a `potential` certificate. FR-802-AC-8 is the refusal: AC-1's original all-literal summand, in both the bare `0.10`/`0.05` and the `decimal(10,2)`/`decimal(5,2)` constructor spellings, refuses `ill_typed`/`ambiguous-literal`, since neither the step sum, the objective (FR-194 gives it no declared type) nor the spellings supply an FR-140 source. Each half can now fail a test. |
