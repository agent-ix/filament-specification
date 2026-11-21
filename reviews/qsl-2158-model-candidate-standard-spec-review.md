---
id: SR-202611210022442313144108555
title: "spec-review of model candidate standard vocabulary"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; QSL-2158 PR802; proposals/filament-v1/definitions/value-accounting.md; proposals/filament-v1/definitions/value-accounting-charge-point-vectors.json; tests/value_accounting_vocabulary.rs; FR-461-AC-9/10; src/lib.rs context"
review_set: base
---

## Summary

The changed definition faithfully mirrors the two kernel ModelCandidate alternatives without adding the generic default work unit.

Frozen subject: e6dc978ae3fab8c31359c2b5f7ce7fe8d32531ff; base: 2ccc41ea6c9b3a6639e46f069c768688efadca94. Ticket: QSL-2158; PR: filament-specification#802. Exact model identifier unavailable; reviewer identity is native-codex/unavailable.

No applicable AssuranceProfile was found in the repository's spec tree. Repository AGENTS.md and CLAUDE.md, README.md, CONTRIBUTING.md, Cargo.toml lints, deny.toml and rust-toolchain.toml were examined for applicable conventions. No dependency, lockfile, feature, export-path, source-library or CI change occurs.

## Qualification

Actual root-owned Titan run at frozen e6dc978ae3fab8c31359c2b5f7ce7fe8d32531ff: focused three tests rc0, 3 passed, 0 failed, 0 ignored. Tests: published_charge_point_list_equals_the_ordered_table; undeclared_spellings_are_not_in_the_vocabulary; a_one_sided_change_to_either_list_is_detected. No local Cargo command ran. The final exact-head all-target check after artifact custody and any current-main rebase remains with the root agent; mergeability is not yet claimed.

Independent static checks: Quire plain Markdown parse of the untyped definition exit 0; explicit FR-461 validation exit 0; git diff --check exit 0. JSON has 66 unique declared spellings, model.candidate last, and 11 unchanged undeclared entries. The diff appends one spelling and preserves all earlier 65 entries. Tool versions actually observed: quoin 0.36.8; quire 0.52.0, engine 0.62.2; spec-artifacts-process@v0.36.0.

## Method selection

Code-review with Rust lane and BASE spec-review ran. Gap-analysis is N/A: no production Rust implementation changed. EARS, dependency, integrity, object, architecture, failure-domain, evidence, risk-complexity and scope-boundary lenses are N/A to this narrow normative mirror: no new FR statement, artifact structure, domain object, relationship, production architecture or independent requirement allocation. No Jev, external model, broad matrix sweep, Kani, clippy or new test target was used. Broader gates were excluded by the authorized brief. Full-suite and downstream kernel/consumer qualification are not claimed.

## Verdict

PASS for spec-review scope — no source findings; the actual focused three-test run passed. Final exact-head all-target validation and mergeability remain with the root agent.

## Review reasoning

The explicit N-work branch covers candidate vectors, states and actual result pairs as well as canonical key node/byte visits before traversal, encoding, decoding, comparison and copying. The storage branch requests zero work and full-new-capacity B result units before each growth, uses checked derivation, includes scratch/output/owned copies, excludes deltas and typed occurrences, and accumulates without refunds. Both branches request no semantic-size counters. The shared-session ledger, original Cancel, atomic admission, first denial and no-reset conditions are retained; WorkScope remains work-only, with existing model and identity limits unchanged. Qualification-seam next_charge explicitly admits zero for storage-only injection. The specific override removes ambiguity from the inherited default +1 wording. No new FR/AC IDs or relationships are introduced, existing FR-461-AC-9/10 remain the qualification contract, and no tracking/digest/pin mechanism is added. The definition is untyped Markdown and was not assigned invented frontmatter or an archetype. Coverage review is limited to the changed vocabulary and existing three-test binding; semantic kernel behavior remains the kernel's responsibility.

## Examined Scope

```yaml
{
  "scope": [
    {
      "id": "ordered-model-candidate",
      "path": "proposals/filament-v1/definitions/value-accounting.md",
      "role": "examined",
      "excerpt": "`model.candidate` before caller-declared candidate work (argument vectors, candidate states or evaluated result pairs), and before canonical plan-policy key work or storage growth as specified below"
    },
    {
      "id": "model-candidate-work",
      "path": "proposals/filament-v1/definitions/value-accounting.md",
      "role": "examined",
      "excerpt": "no size-counter change; `work_units += N`, `result_units += 0`, instead of the default one work unit, where `N` is the actual count of caller-owned candidate argument vectors, candidate states or evaluated result pairs, or the actual node/byte visits for canonical plan-policy key traversal, encoding, decoding, comparison or copying; charged before that work"
    },
    {
      "id": "model-candidate-storage",
      "path": "proposals/filament-v1/definitions/value-accounting.md",
      "role": "examined",
      "excerpt": "no size-counter change; `work_units += 0`, `result_units += B`, instead of the default one work unit, before each canonical plan-policy key storage growth; `B` is the full new requested capacity in bytes for scratch storage, output storage or owned key copies, derived with checked arithmetic, not the capacity delta or typed value occurrence count"
    },
    {
      "id": "model-candidate-ledger",
      "path": "proposals/filament-v1/definitions/value-accounting.md",
      "role": "examined",
      "excerpt": "The two `model.candidate` alternatives use exactly their stated work and result\nadditions; neither receives an additional default work unit. Successful storage\nreservations accumulate without refunds. Canonical plan-policy key work uses the\nestablished search, replay or checker-session evaluation ledger and its original\n`Cancel`, preserving atomic admission and first-denial behavior, with no per-key\nor per-instance reset."
    },
    {
      "id": "model-candidate-workscope",
      "path": "proposals/filament-v1/definitions/value-accounting.md",
      "role": "examined",
      "excerpt": "A work-only\n`WorkScope` does not share the result ledger. This charge point does not change\nmodel `max_state_key_bytes` or `IdentityLimits`."
    },
    {
      "id": "qualification-model-candidate",
      "path": "proposals/filament-v1/definitions/value-accounting.md",
      "role": "examined",
      "excerpt": "normalization row, or the stated `model.candidate` work addition, including\nzero for its storage-only alternative)."
    },
    {
      "id": "FR-461-AC-9",
      "path": "spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md",
      "role": "examined",
      "excerpt": "The spellings the `filament.value.accounting/v1` ordered charge-point table declares equal, in table order, the spellings of the published list `value-accounting-charge-point-vectors.json`, and every spelling has an amount row; the set includes `equality.plan-form`, `frame.write`, `frame.redefines-link`, `declaration.check` and `temporal.visit`."
    },
    {
      "id": "FR-461-AC-10",
      "path": "spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md",
      "role": "examined",
      "excerpt": "A spelling the table does not declare, such as `temporal.visits`, is absent from the ordered table (and therefore the amount table) and from the published list, and a one-sided change to either fails the comparison; `tests/value_accounting_vocabulary.rs` enforces this half. A conforming kernel's test compares its `ChargePoint::ALL` `as_str()` set with `VALUE_ACCOUNTING_CHARGE_POINT_VECTORS` and requires each undeclared entry to fail `from_code`."
    },
    {
      "id": "FR-368-model-candidate",
      "path": "agent-ix/filament-exact/spec/functional/FR-368-meter-charge-and-limit-vocabulary.md",
      "role": "context_only",
      "excerpt": "`ModelCandidate`, spelled `model.candidate`, names caller-owned candidate generation, including argument vectors, candidate states and result pairs. A batch requests its actual count through ordinary work charging. It is appended at index 65; earlier point identities and order remain unchanged."
    },
    {
      "id": "FR-368-key-work",
      "path": "agent-ix/filament-exact/spec/functional/FR-368-meter-charge-and-limit-vocabulary.md",
      "role": "context_only",
      "excerpt": "`Charge::new(ChargePoint::ModelCandidate).work(Integer::from(N))` charges `WorkUnits += N` and zero result units for actual node and byte visits, before traversal, encoding, decoding, comparison or copying."
    },
    {
      "id": "FR-368-key-storage",
      "path": "agent-ix/filament-exact/spec/functional/FR-368-meter-charge-and-limit-vocabulary.md",
      "role": "context_only",
      "excerpt": "`Charge::new(ChargePoint::ModelCandidate).work(Integer::zero()).results(B)` charges zero work and `ResultUnits += B` before each storage growth. `B` is the full new requested capacity in bytes, computed with checked arithmetic, including scratch, output and owned key copies; it is neither a capacity delta nor a typed-value occurrence count. Successful reservations accumulate without refunds."
    }
  ]
}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Tracker publication

Published one method comment privately to QSL-2158: ac0e1ff7-0c5e-439c-93fc-e26a0a6072c8. Read-back passed: returned body exactly matches the draft; the YAML block parses as its JSON subset; clean=true, findings=[] and complete scope/excerpt parity match this artifact. The source-repository marker visibility=public is distinct from the explicit destination=private-linear. Initial automatic review rejected the ambiguous destination wording; a single clarified attempt succeeded. No GitHub finding, public share, branch change or label mutation occurred. The brief prohibited label polling, so existing has-review state was not independently queried.

