---
id: SR-2026111920504600000001
title: "spec-review of QSL-2587 FR-814 name prefixes for parameters, receiver and fields"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #757 at 1622b3d8; spec/functional/output-mapping/FR-814-write-canonical-tla-and-lean-text.md (Names bullets lines 34-61, FR-814-AC-7), spec/test-cases/TC-1973-export-canonical-text-and-names.md (scope, step 6); context: FR-814 Mangling and keyword-safety bullets, FR-812 State and actions and Values tables and FR-812-AC-1/AC-2, FR-817 name_map, FR-012 parameter naming, FR-207 receiver binding, FSL FR-104 and FR-035-AC-7, FSL PR #3084 at its head"
review_set: subset
---

## Summary

Spec review of FSpec PR #757. Ticket: QSL-2587. The PR adds `p_` (operation parameters) and `fld_` (object-type and record fields) to FR-814's prefix list, a bullet saying every name class has its own prefix and none share one, the receiver binder `p_self`, and FR-814-AC-7 with TC-1973 step 6 (field, parameter and function all named `total`). `quire validate` passes on both files. FR-814-AC-7 is not on origin/main and no open FSpec or FSL PR uses it. TC-1973 is changed only here.

## Examined scope

- Prefix distinctness. `p_` and `fld_` differ from every other listed word at the first or second character (`f_` against `fl`, `p` starts no other word), and the existing "proper prefix" argument covers `f`/`fld`. With the `_`-doubling mangling, a user name that itself begins with a prefix word (a parameter `self_x` gives `p_self__x`, a field `p_x` gives `fld_p__x`) never equals a name of another class. Within the listed classes the naming is injective. (examined)
- The receiver binder `p_self`. The justification "`self` is an FSL keyword and never a parameter name" is not supported: FR-012 and FSL FR-104 say domain models declare parameters and fields with one identifier class that admits reserved words (the `by` parameter), and FR-207 keeps the receiver binding `self` distinct from a parameter binding. A declared parameter `self` gives `p_self` too (FND-001). (examined)
- Unlisted name classes. Quantifier, `let` and lambda binders, FSL function parameters, FR-812's Lean `Key_T`/`Rec_T` types and the generated record fields `num`, `den`, `tag`, `value`, `bits` are not in the list. The new last sentence gives them no prefix, which contradicts both line 34 and the bullet's own first sentence, and leaves the keyword and structural-name safety argument unproven for them (FND-002). (examined)
- FR-812 consistency. FR-812-AC-2 still expects the field read `count /= 0`, unprefixed (FND-003). FR-812's operation and `Next` rows bind `p1, …` with no receiver, while FR-814 now names a receiver binder of the step (FND-005). (examined)
- FR-814-AC-7 and TC-1973 step 6. The explicit name strings make the AC fail for `f_total` as a field. The parse clause does not add failing power, and no row covers a parameter spelled `self` (FND-004). (examined)
- Ids and merge order. No collision on main or in open PRs. This PR has no dependency on FSL #3091 and should merge first, before FSL #3084 renames `f_` fields to `fld_`. (examined)

## Verdict

Not mergeable. Two high findings must be fixed in this PR: the receiver binder must have a name no declared parameter can produce, and the unlisted classes must be given prefixes (or the keyword-safety claim must be argued for them) without contradicting line 34. FR-812-AC-2 should be updated in the same PR. The `p_`/`fld_`/`f_` split itself is sound and injective among the listed classes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `p_self` is not reserved to the receiver. FR-012 and FSL FR-104 admit reserved words as domain-model parameter names (one identifier class, read through the qualified name, e.g. `by`), and no spec text makes `self` unusable as a parameter. A parameter `self` mangles to `p_self` and collides with the receiver binder, so the naming is not injective. Give the receiver its own class (e.g. a prefix no other class uses, or an unprefixed structural name) | spec/functional/output-mapping/FR-814-write-canonical-tla-and-lean-text.md:47-57 |
| FND-002 | high | The new sentence "A name class the list does not name has no prefix of its own" contradicts line 34 ("Every generated name SHALL be a kind prefix followed by the mangled FSL name") and the same bullet's "Every name class the mapper writes SHALL have its own prefix". It also exempts binders, FSL function parameters, Lean `Key_T`/`Rec_T` and generated record fields from the keyword-safety and structural-name arguments, which rest on "every mangled name carries a prefix". A `let` binder named `Next` or `instance`, or one named `DOMAIN`, then breaks SANY or Lean | spec/functional/output-mapping/FR-814-write-canonical-tla-and-lean-text.md:34-61 |
| FND-003 | medium | FR-812-AC-2 still expects the definedness conjunct `count /= 0` (TLA+) and `count ≠ 0` (Lean) for the field `count`, which under the new `fld_` rule is written `fld_count`. The two ACs contradict each other, and the PR does not touch FR-812 | spec/functional/output-mapping/FR-812-map-state-actions-and-values-to-tla-and-lean.md:138 |
| FND-004 | low | FR-814-AC-7's parse clause ("no duplicate-definition or shadowing error") cannot fail for the old naming: a TLA+ record field is a string and a Lean structure field is namespaced, so a field `f_total` next to a function `f_total` parses. Only the name strings make the AC fail. No row exercises the receiver against a parameter spelled `self` | spec/functional/output-mapping/FR-814-write-canonical-tla-and-lean-text.md:118 |
| FND-005 | low | FR-814 now says the receiver is "the binder `p_self`" of the operation's step, but FR-812's rows `op_op(p1, …)` and `Next == \E p1 \in Dom1, …` bind no receiver and give it no domain (the present key set). Where it is bound is unstated | spec/functional/output-mapping/FR-812-map-state-actions-and-values-to-tla-and-lean.md:42-43 |
