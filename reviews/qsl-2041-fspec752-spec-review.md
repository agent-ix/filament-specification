---
id: SR-2026111919022500000001
title: "spec-review of QSL-2041 FR-012 qualified operation parameter name"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #752 at 93da26bf; spec/functional/foundation/FR-012-bind-operation-observations.md (Behavior parameter paragraphs, FR-012-AC-7, FR-012-AC-9), spec/test-cases/TC-2083-qualified-operation-parameter-name.md"
review_set: subset
---

## Summary

Spec review of FSpec PR #752. Ticket: QSL-2041. The PR follows the QSL-2041 ruling: a pre/post body reads an operation parameter by its bare name or by `<op>::p`, where `<op>` is the clause's operation spelled as its `on` authors it. That qualified form is the only spelling of a reserved-word parameter such as `by`. The PR adds FR-012-AC-9 and TC-2083. The review checked the new text against the FSL grammar and checker on filament-spec-language main. `fsl-cst` `QualifiedName = ident {'::' member}`, and `Terminal::MemberName` accepts reserved words (`parser.rs:2685`). `Primary` takes `EnumValue` and `QualifiedName`, and `fsl-forms` lowers both to `Shape::Name`. `Terminal::Identifier` refuses reserved words, so bare `by` (`token.rs:172`) is no primary. Today `typing.rs` sends `ExprNode::Name` through `resolve_target` and `qualified_name_refusal`, which turn a qualified name that resolves to nothing into `MissingQualifiedName`. That maps to `missing_declaration`/`missing-name` (`refusal.rs:3757`, `3851`). The review also compared the PR with FR-012-AC-7, FR-042-AC-4 and the "similarly named object field" rule. `quire validate` passes on both changed files.

## Examined scope

- FR-012 Behavior, the new parameter paragraph (examined). The grammar facts match the code. The rule that the prefix must be spelled as the clause's `on` is sound. It is consistent with the existing rule that a parameter is never replaced by a similarly named field (FND-002 covers the binder edge).
- FR-012-AC-7 together with the new text (examined). See FND-001.
- FR-012-AC-9 (examined). Each clause can fail. The field `by = -1.00` against the parameter `by = 1.00` separates "reads the parameter" from "reads the field". The refusals on `::nope`, `reset::by` and the invariant can fail under an implementation that resolves by the last segment alone. The fixture is incomplete (FND-003).
- TC-2083 (examined). It matches AC-9 step for step.
- Ids (examined). FR-012-AC-9 and TC-2083 are free on main. The other open PRs, #640 and #695, add neither id.

## Verdict

Not mergeable as is. Two medium wording gaps and one low fixture gap need small edits. FND-001: the spec never says that `pre(<op>::p)` refuses, and FR-012-AC-7's "bare parameter" now clashes with the PR's own "bare name". FND-002: "both spellings denote the same value" does not hold when a `let` or quantifier binder shadows the parameter. FND-003: AC-9's invocation does not supply `step`. The grammar facts, the prefix rule and the refusal causes are correct. Merge this PR before FSL #3075.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new text does not say that the qualified form under `pre(...)` refuses `wrong_snapshot`/`forbidden-pre-read`. FR-012-AC-7 says "A bare parameter inside pre refuses", and this PR now uses "bare name" to mean the unqualified spelling, so a reader can take `pre(M::Meter::adjust::by)` as admitted. FSL FR-104-AC-18 pins the refusal; the FSpec upstream does not | spec/functional/foundation/FR-012-bind-operation-observations.md:31-43 |
| FND-002 | medium | "Both spellings denote the same invocation-supplied parameter value" fails when a same-named `let` or quantifier binder is in scope. The checker resolves a bare name innermost-first (fsl-semantics check.rs:3327), so there bare `step` is the binder and `M::Meter::adjust::step` is the parameter. State that the qualified name always denotes the parameter, whatever lexical binder is in scope | spec/functional/foundation/FR-012-bind-operation-observations.md:31-33 |
| FND-003 | low | AC-9's invocation supplies only `by = 1.00`. `step` is a declared parameter, so as written the invocation is declined at admission (`invalid_runtime_input`/`missing-member`) before `M::Meter::adjust::step = step` evaluates. Give `step` a value | spec/functional/foundation/FR-012-bind-operation-observations.md:66 |
