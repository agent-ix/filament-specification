---
id: SR-2026111922284000000001
title: "spec-review of QSL-2629 FSpec FR-149 integer widening at declared parameter and payload positions"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #762 at d9ec4dcf; spec/functional/type-model/FR-149-apply-complete-equality-matrix.md (reference upcast sentence, new section Integer widening at declared parameter and payload positions, FR-149-AC-30), spec/functional/foundation/FR-044-check-composed-numeric-definedness.md (Behavior exception list, FR-044-AC-9), spec/test-cases/TC-2097-integer-widening-at-declared-parameter-and-payload-positions.md; context: FSpec FR-007, FR-033, FR-143, FR-146 (FR-146-AC-21), FR-322-AC-70 at origin/main; Contract IR FR-347; FSL #3117 at ef9ffdd2"
review_set: subset
---

## Summary

Spec review of FSpec PR #762. Ticket: QSL-2629. The PR adds an FR-149 section that admits an `Int[..]` argument at a declared function parameter or union-member payload of type `Integer` or a containing `Int[..]`. The package records it as one exact `filament.op.numeric.convert` with no obligation. A narrowing argument keeps `unproved-range`, and `Some` keeps FR-143-AC-21. The section lists the positions it does not cover. The PR amends FR-149's reference-upcast sentence, FR-044's exception list and FR-044-AC-9, and adds FR-149-AC-30 and TC-2097.

## Examined scope

- FR-149's earlier text (examined). The "only implicit subsumption admitted for arguments and bindings" sentence is amended. No other FR-149 sentence still says that. The "Call arguments supply none either" sentence is about the expected type of arithmetic operands, so it does not conflict.
- Narrowing at arguments (examined). It is consistent with FR-146, which already refuses a narrowing argument `unproved-range` at the argument (FR-146-AC-21).
- FR-143 (examined). The `Some` exact rule is kept, and FR-143's union construction text states no payload typing rule that conflicts.
- FR-044 (examined). The exception list and AC-9 are consistent with the new section.
- Contract IR FR-347 (examined). Its exact non-record payload match, and FR-322-AC-70's typing of a call argument against the parameter node, are met by the explicit conversion.
- Grep of FSpec spec text for "implicit", "widen" and "subsum" (examined; FND-001, FND-003).
- FR-149-AC-30 (examined). It can fail: FSL's union path adds no conversion until QSL-2632 lands.
- Ids (examined). FR-149-AC-30 and TC-2097 are free on main and in open FSpec PRs #761, #760, #695 and #640. #760 holds TC-2090.
- `quire validate --scope .` passes on all three files.

## Verdict

Not mergeable as is. FND-001 and FND-002 are medium and each needs a short text fix in this PR. The rule itself is sound, and it is consistent with FR-146, FR-143, FR-044 and Contract IR FR-347.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-033 is not amended. Its Behavior says "Calls resolve by exact declaration identity and parameter order/type ... There is no implicit numeric, collection or truthiness coercion", and FR-033-AC-2 refuses a "wrong type". Under the new FR-149 section, an `Int[0, 1000]` argument at an `Integer` parameter is admitted, so FR-033 now contradicts FR-149. FR-033 needs a sentence citing FR-149's integer widening, and FR-146's argument range obligation | spec/functional/foundation/FR-033-admit-reusable-predicates.md:23-25 |
| FND-002 | medium | The last bullet says a `let` binding, an annotated `let`, a declared result, a field write and an operand of `=`/`!=` "take no widening", but does not say whether such a value is refused or admitted without a conversion. FR-146-AC-21 admits `function narrow using V(x: Int[0, 9]): Integer pure { x }`, an `Int[0, 9]` body at a declared `Integer` result. Read as a refusal, the bullet contradicts that AC. Read as an admission, it contradicts the claim that these positions keep the exactness the new section relaxes. The tuple bullet ("not covered by this rule") has the same gap, and FSL admits tuple widening today without a conversion (QSL-2636). State the outcome for each listed position | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:355-357 |
| FND-003 | low | FR-007 still says "No host overflow, inferred machine-width domain or implicit widening is admitted". It means value-domain widening of integer arithmetic, but it now reads against FR-149's admitted argument widening. Qualify it, for example "implicit widening of a value's domain", or cite FR-149 | spec/functional/foundation/FR-007-prove-integer-definedness.md:29 |
| FND-004 | low | FR-149-AC-30 and TC-2097 do not cover the stated exception: an argument the package already carries as `Integer` (an integer division, or an integer-lowered local) at an `Integer` parameter records no conversion. Nor do they cover any excluded position other than `Some`. A producer that adds a conversion there, or widens at a tuple or result position, passes the AC | spec/functional/type-model/FR-149-apply-complete-equality-matrix.md:488 |
