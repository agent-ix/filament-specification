---
id: SR-2026112106460400000005
title: "Code review (Rust harness lane) — FSpec PR 787 vacuous exhaustive carrier and policy hints"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; proposals/backend-provider-v1/schema.json (Result allOf exhaustive rule); proposals/backend-provider-v1/vectors.json (exhaustive-vacuous-conditional-outcome, exhaustive-with-nonvacuous-certificate); tests/backend_provider_v1.rs (tc_431_exhaustive_certificate_is_only_conditional_vacuous)"
review_set: base
---

## Summary

Ticket: QSL-2158. PR: filament-specification#787, frozen head 0f3375a68372bc7db8bfe6b9933215d969a2f023 over base f73bd9f3fe5091ab8b459126808d7149cdb02b08. Code-review lane with the rust-review/rust-style lane folded in, over the wire schema, vectors and the TC-431 harness test. No production engine changed, so gap-analysis does not apply.

The schema rule correctly admits the published vacuous certificate and refuses `exact`, `ratio`, `optimum`, missing/unknown mode, extra conditional members, nonempty or missing `values`, and `evidence`. It keeps the five other certificate-free bases certificate-free and evidence-free, and `exact-value` still requires a `Rational`. Independent Python jsonschema 4.26 replay of every harness assertion agreed. Control cases showed that the `ratio` and `optimum` refusals come from the new exhaustive rule: the same certificates are admitted under `exact-value`. The `ratio`/`optimum` and adverse-vector negatives are therefore real.

The Rust test was not run here (no cargo, per brief). The remote single exact harness test result is pending from the parent, so this review makes no runtime-green claim.

## Verdict

**FAIL** — the value-free constraint has a wire hole (FND-001).

## Review notes

Rust lane: the test follows the file's existing idiom (`definition`, `composed`, `positive`, `apply`, `validator`), and its `#[trace("TC-431", "FR-331-AC-60")]` binding is correct. `json!` import is used. Assertions carry messages, and each other-basis negative is paired with a positive bare control, so those negatives are real. No unsafe code, panics outside tests, or concurrency.

Reproduction script and output: /tmp/qsl2158-fspec787-review/check.py, /tmp/qsl2158-fspec787-review/check-out.json.

## Scope

| ID | Role | Excerpt |
| --- | --- | --- |
| schema.json Result exhaustive rule | examined | `"then": {"properties": {"certificate": {"allOf": [{"$ref": "#/$defs/ProbabilityCertificate"}, {"required": ["kind", "conditional"], "properties": {"kind": {"const": "conditional"}, "values": {"maxItems": 0}, "conditional": {"required": ["mode"], "properties": {"mode": {"const": "vacuous"}}}}}]}, "evidence": false}}` |
| exhaustive-vacuous-conditional-outcome | examined | positive vector: proved exhaustive, certified, certificate kind conditional, values [], conditional {mode: vacuous} |
| exhaustive-with-nonvacuous-certificate | examined | adverse vector: replaces /results/0/certificate with kind exact, values [] |
| tc_431_exhaustive_certificate_is_only_conditional_vacuous | examined | TC-431 harness test over FR-331-AC-60 |
| FR-331-AC-60 | context_only | The schema admits exhaustive-vacuous-conditional-outcome ... A certificate with ratio or optimum mode, missing conditional, missing mode, an unknown mode or another conditional member is refused on exhaustive, as is evidence ... |

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high | The exhaustive rule constrains only `kind`, `values` and `conditional`. A proved `exhaustive` result whose vacuous certificate also carries nonempty `ranking`, `components` with rational `value`/`gain`, or `policy` is schema-admitted, so the "value-free" result can carry arbitrary numerical data. The harness has no negative for these members. | proposals/backend-provider-v1/schema.json:1465 |
| FND-002 | medium | The exhaustive certificate exception is not tied to a probabilistic result `kind`. The vacuous certificate is admitted on an `exhaustive` result of kind `refinement`, although FR-331 permits it only for a conditional claim. | proposals/backend-provider-v1/schema.json:1465 |

## Dispositions

Round 1, against b9757e34a96b3f2a4612a8e9450d71784b8d79b6 (delta 0f3375a6..b9757e34). Re-verified independently with jsonschema 4.26 over the fix-head schema and vectors (/tmp/qsl2158-fspec787-review/disposition1/check2.py, check2-out.json: 27/27 expectations met). The new harness hunk (tests/backend_provider_v1.rs:810 onward) is rustfmt-clean under edition 2024; rustfmt drift elsewhere in the file predates this PR. Rust test not run here. The fix-head rerun is pending, so no runtime-green claim for b9757e34; the 0f3375a6 focused test was reported green by the parent, not observed by this reviewer.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The exhaustive certificate branch now sets `"ranking": false, "policy": false, "components": false`. Nonempty and empty `ranking`, `policy` and `components` are each refused on the exhaustive result, while the same certificate passes the `ProbabilityCertificate` shape (positive control). The harness adds the same paired control/negative loop. |
| FND-002 | fixed | `dependentSchemas: {certificate: {properties: {kind: {const: "probabilistic-satisfaction"}}}}` in the exhaustive branch. The vacuous certificate is refused on `refinement` and `temporal-satisfaction` results, and both kinds remain admitted bare. The harness asserts both refinement cases. |

Round 2, against 97051ccacb36ad4622f2146a5ca30ea36b3fdb2e (delta b9757e34..97051cca). No new rows: FND-001 and FND-002 were already `fixed` in round 1. `git diff --quiet b9757e34 97051cca -- proposals tests` is clean, so schema, vectors and the Rust harness are byte-unchanged. The committed `reviews/qsl-2158-code-review.md` at 97051cca is byte-identical to the round-1 SR file. Runtime: the parent reports that the exact targeted Rust test on b9757e34 passed on draco (1 passed, 0 failed, 0 ignored, 42 filtered; log /private/tmp/fspec-787-b975-test.log). This reviewer did not observe that run. Because the tested files are byte-unchanged, that report applies equally to 97051cca. Verdict: PASS.
