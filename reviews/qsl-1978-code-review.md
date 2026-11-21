---
id: SR-2026112114311101
title: "Code review (Markdown/documents lane) — native invocation outcomes and generated plan types"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; PR #786 at 2975a1885c64cd1b90d091dc0c432a226f32e5f5 against main d1ebfc724e539bae217cf73bd14edff45e078e4c; changed files: reviews/qsl-1978-expansion-spec-review-base.md, reviews/qsl-1978-expansion-spec-review-integrity.md, spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/functional/analysis/FR-650-plan-and-optimize-over-a-state-model.md, spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md, spec/functional/analysis/FR-801-bound-and-meter-the-plan-search.md, spec/functional/analysis/FR-804-order-plans-deterministically.md, spec/functional/analysis/FR-805-read-the-plan-request-and-write-the-plan-outcome.md, spec/functional/analysis/FR-806-refuse-plan-requests-with-located-causes.md, spec/functional/analysis/FR-819-plan-over-hybrid-models.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-1603, TC-1950, TC-1951, TC-1954, TC-1955, TC-1966, TC-2037"
review_set: subset
---

## Summary

Ticket: QSL-1978. This is the code-review method for PR #786, run in the
disposition-round-1 pass because the initial review skipped it. The diff
holds no source or test code: it changes only `spec/` Markdown requirements
and test cases, plus the two committed review artifacts. The review
therefore ran the language-independent lanes of `code-review`
(Duplication and Vendoring, Unasked-for Ceremony and File Tracking,
Integrity) over the Markdown documents. It found two duplication findings.

## Verdict

**FAIL — two high duplication findings.** The code-review rule records
duplication as P0 `high`. Both findings are prose restatements of a rule
that has one owner, inside this one repository. Each is fixed by replacing
the restatement with a reference.

Clean lanes:
- **Vendoring:** nothing copied from another repository. The changed files
  are only `spec/**` and `reviews/**`.
- **Ceremony and file tracking:** no ledgers, manifests, digests, pins,
  relocation maps or new gates. The two `reviews/` files are byte-identical
  to the reviewer's original SpecReview artifacts, the custody copies the
  reviewer role requires.
- **Integrity:** no thresholds lowered and no warnings suppressed.
  `quire validate --scope .` over the 18 changed files exits 0. Its eight
  advisories are pre-existing: EARS advisories on FR-804:19, FR-819:24 and
  FR-322:112, none of them changed lines, and FR-181's TC-372 edge-type
  advisory.

Not applicable, because the diff has no code: Test Standards, Mock
Compliance, the two Completeness sections, Edge Case and Logic Review,
the Rust lane (`rust-review`; no `.rs` or `Cargo.toml` change), the React
lane, and `quoin:gap-analysis` (no production code). `quire matrix
--strict` was not run, since the diff adds no test functions to bind. The
spec criterion-to-test-case trace is covered by the integrity analysis,
SR-2026112113535902. No build, runtime test or full gate was run, and none
is claimed.

## Review notes

The scope is PR #786's diff against main at the frozen head, read as
Markdown documents. The duplication search compared each normative
statement that fix commit `2975a188` added with the owner statements it
cites: FR-322's Plan-record bullets, FR-806's S3 table, and FR-819's
hybrid semantics. It also checked the FR-650, FR-804 and FR-805
receiver/ordinal failure wording. FR-805 and FR-804 refer to FR-650's seam
contract rather than restating its exit codes, so they raised no finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Duplication: the shared-argument normalization and collision rule now has three normative statements. FR-322's Plan-record bullet, the owner, states it fully. FR-800 restates it: optional X and required Option<X> are one record-field type, omission flags are unchanged, and the diagnostic is FR-806's S3 refusal with a deterministic pair. The paragraph added under FR-806's S3 table restates the normalization, the omission-flag rule and the third-conflict pair rule again. A change to the rule must edit three files, and they already state the pair rule at three levels of detail. Keep the rule in FR-322 and FR-806's catalog row (code, cause, locus); reduce FR-800 and the FR-806 paragraph to references. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6127-6144; spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md:96-102; spec/functional/analysis/FR-806-refuse-plan-requests-with-located-causes.md:51-57 |
| FND-002 | high | Duplication: the hybrid trajectory-local state semantics are stated normatively three times. FR-819 owns the hybrid execution semantics and the delay/elapsed override. FR-800 and FR-322 both restate them in full: point-valued records, universal quantification over the initial set and nondeterministic parameters, FR-193 enclosure evidence, and that no representative point discharges the obligation. FR-322 should state only the type nodes (builtin `scalar_type`/`rational`, the hybrid-state record). FR-800 needs only the pointer that FR-819 overrides its timed quantity typing for hybrid plans. The semantic paragraph should live once, in FR-819. | spec/functional/analysis/FR-819-plan-over-hybrid-models.md:70-81; spec/functional/analysis/FR-800-define-the-plan-and-goal-model.md:109-117; spec/objects/interfaces/FR-322-checked-package-artifact.md:6195-6200 |
