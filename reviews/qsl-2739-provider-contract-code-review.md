---
id: SR-2026112114454020075413600
title: "Code review: PR 792 backend-provider-v1 schema delta (assumptions, InterfaceMismatch, provider-answer-unconfirmed)"
type: SpecReview
analysis: code-review
date: 2026-11-21
scope: "agent-ix/filament-specification; PR 792 git diff 9f5b30a5...35b3b0e0; proposals/backend-provider-v1/schema.json; QSL-2739"
review_set: subset
---

## Summary

Ticket: QSL-2739. Method: code-review. Reviewer run 4b0eb3f0-7e32-4a5e-ad42-0e821771b4c3, model claude-opus-5-5, quire 0.52.0 (engine 0.62.2), quoin 0.36.8. Frozen head 35b3b0e0a133f8288b5d252f371c5e43b7437788 over base 9f5b30a525a8729ddb836c8eb9b3821d30d0de4f.

JSON Schema delta only (no Rust, TS or Python changed, so no rust-review or review-react lane). The changed schema was checked for duplication or vendoring, closed shapes (`additionalProperties: false`), oneOf disjointness of the new `InterfaceMismatch` arm (a distinct `type` const), that the `premises`/`assumptions` coupling matches FR-331 "Items", and that the new cause sits in the same arm as the other producer-only causes (`premise-unproved`).

## Verdict

**PASS**: the schema delta is closed, faithful to the FR-331 prose and duplicates nothing. `jq`/`json.tool` parses it (exit 0). The schema has no qualification vectors; that coverage gap is recorded under gap-analysis (SR-2026112114454020075413601 FND-001) and not repeated here. Consumer note: FSL #3287 (merged) writes `premises` without `assumptions`, so its composition items are refused by this schema until PR2 lands. That follows from contract-first sequencing and is not a defect in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

- schema-assumptions (examined; clean)
- schema-allOf-assumptions (examined; clean)
- schema-InterfaceMismatch (examined; clean)
- schema-provider-answer-unconfirmed (examined; clean)
- FR-331-assumptions (context only)
- FR-331-counterexample (context only)

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | Rust lane (rust-style testing, "every assertion must be able to fail"). The adverse vector `assumptions-on-another-kind` cannot fail through the rule it names. Its base still carries `premises`, so the pre-existing rule "premises belongs to a composition item only" (schema allOf, FR-491) refuses the patched document at /request/items/0/kind even if the new assumptions rule is deleted. Because `assumptions` already requires `premises` and `premises` already requires `composition`, the `kind` clause of the new assumptions rule is unreachable on its own. Either drop that clause and the vector (keeping AC-63's "on another kind" only as implied), or say AC-63's clause is enforced by the premises rule. | proposals/backend-provider-v1/vectors.json:adverse assumptions-on-another-kind |

## Rust lane (disposition pass 1)

Rust lane, run this round over the fix diff: tests/backend_provider_v1.rs adds only two `#[trace("TC-431", "FR-331-AC-63")]` attributes, on `tc_431_positive_vectors_are_admitted_with_owned_members` and `tc_431_adverse_vectors_are_refused`. The repository's declared trace form is used correctly. The two generic loops iterate every `positive`/`adverse` entry of vectors.json, so the 3 positive and 9 adverse vectors do execute under them: positives assert zero schema errors and existing owned members; adverses assert a single-patch apply, refusal, and an error at, under or above the patched member (`located_at_patch`). No new harness, API, unsafe code, panic path or wire conversion. Gates: the reviewer ran no cargo command (brief). The author reports `cargo check --locked -j1 --test backend_provider_v1` exit 0. The focused runtime `cargo test --locked -j1 --test backend_provider_v1 -- tc_431_positive_vectors_are_admitted_with_owned_members tc_431_adverse_vectors_are_refused --test-threads 1` at bec3059e is queued on Draco and still PENDING, with no result. rustfmt: the whole file is pre-existing unformatted at base; the two attribute lines need no formatting, so no churn is required. Vector fidelity: each adverse vector except FND-002 isolates its rule (removing `assumptions` hits premises->assumptions; removing `premises` hits assumptions->premises; a string index hits Count; InterfaceMismatch and cause closure hit `required`/`enum`/`additionalProperties` of their own arm). The `provider-answer-unconfirmed` positive is labelled schema-admission only, and FR-331-AC-63 and BP-66 assign provider-origin refusal to the reader (AC-62, TC-986). It is not an acceptance of a provider-forged cause or counterexample.

## Dispositions

Round 2, against b3c66f3554a142998bd211782bd6626b71fa7784 (fix diff bec3059e..b3c66f35).

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | Masked vector removed and redundant kind clause dropped. The accepted language is unchanged: old `assumptions => kind=composition AND premises`; new `assumptions => premises`, and the existing rule gives `premises => kind=composition`. The diff to vectors.json is only that removal. tests/backend_provider_v1.rs is unchanged since bec3059e (git diff --stat empty). |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AC-63 now reads "it refuses ..., and, by the existing `premises` rule alone, `premises` or `assumptions` on another kind, a non-integer inner index, an `interface-mismatch` missing a member, ... and the cause without `premise` or with an extra member". "By the existing premises rule alone" grammatically governs every clause after it, but only the other-kind clause comes from that rule. The index, InterfaceMismatch and cause refusals come from their own arms. Scope the phrase to the other-kind clause, for example in parentheses after it. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:1167 |
