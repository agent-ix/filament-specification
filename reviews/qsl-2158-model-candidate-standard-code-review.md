---
id: SR-202611210022442313144108554
title: "code-review of model candidate standard vocabulary"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; QSL-2158 PR802; proposals/filament-v1/definitions/value-accounting.md; proposals/filament-v1/definitions/value-accounting-charge-point-vectors.json; tests/value_accounting_vocabulary.rs; FR-461-AC-9/10; src/lib.rs context"
review_set: subset
---

## Summary

The JSON vocabulary append and the existing Rust required-spelling assertion agree with the normative ordered table and amount table.

Frozen subject: e6dc978ae3fab8c31359c2b5f7ce7fe8d32531ff; base: 2ccc41ea6c9b3a6639e46f069c768688efadca94. Ticket: QSL-2158; PR: filament-specification#802. Exact model identifier unavailable; reviewer identity is native-codex/unavailable.

No applicable AssuranceProfile was found in the repository's spec tree. Repository AGENTS.md and CLAUDE.md, README.md, CONTRIBUTING.md, Cargo.toml lints, deny.toml and rust-toolchain.toml were examined for applicable conventions. No dependency, lockfile, feature, export-path, source-library or CI change occurs.

## Qualification

Actual root-owned Titan run at frozen e6dc978ae3fab8c31359c2b5f7ce7fe8d32531ff: focused three tests rc0, 3 passed, 0 failed, 0 ignored. Tests: published_charge_point_list_equals_the_ordered_table; undeclared_spellings_are_not_in_the_vocabulary; a_one_sided_change_to_either_list_is_detected. No local Cargo command ran. The final exact-head all-target check after artifact custody and any current-main rebase remains with the root agent; mergeability is not yet claimed.

Independent static checks: Quire plain Markdown parse of the untyped definition exit 0; explicit FR-461 validation exit 0; git diff --check exit 0. JSON has 66 unique declared spellings, model.candidate last, and 11 unchanged undeclared entries. The diff appends one spelling and preserves all earlier 65 entries. Tool versions actually observed: quoin 0.36.8; quire 0.52.0, engine 0.62.2; spec-artifacts-process@v0.36.0.

## Method selection

Code-review with Rust lane and BASE spec-review ran. Gap-analysis is N/A: no production Rust implementation changed. EARS, dependency, integrity, object, architecture, failure-domain, evidence, risk-complexity and scope-boundary lenses are N/A to this narrow normative mirror: no new FR statement, artifact structure, domain object, relationship, production architecture or independent requirement allocation. No Jev, external model, broad matrix sweep, Kani, clippy or new test target was used. Broader gates were excluded by the authorized brief. Full-suite and downstream kernel/consumer qualification are not claimed.

## Verdict

PASS for code-review scope — no source findings; the actual focused three-test run passed. Final exact-head all-target validation and mergeability remain with the root agent.

## Review reasoning

The added assertion is reachable and fails if model.candidate is omitted even when both authored lists omit it. Existing comparison checks ordered-list equality and amount-set equality; adverse cases detect one-sided changes and undeclared entries remain rejected. Trace attributes continue to bind FR-461-AC-9/10. The dependency-free exported include_str path remains unchanged. Kernel PR191's actual merge is a58474cc105a86f3307a11def3fc001e2459679f; PR192's merged bfbe830aa15cb831a58469bbc0ccc9cbcd09c84a carries ModelCandidate at ALL index 65 of 66 and spelling model.candidate. Kernel earlier order differs from standard table order in existing entries; AC-10 requires set equality, so that inherited difference is not a new defect. No implementation, boundary, safety or concurrency change is introduced by the one Rust string addition.

## Examined Scope

```yaml
{
  "scope": [
    {
      "id": "model.candidate-vector",
      "path": "proposals/filament-v1/definitions/value-accounting-charge-point-vectors.json",
      "role": "examined",
      "excerpt": "\"frame.redefines-link\",\n    \"model.candidate\""
    },
    {
      "id": "required-kernel-spelling",
      "path": "tests/value_accounting_vocabulary.rs",
      "role": "examined",
      "excerpt": "\"model.candidate\","
    },
    {
      "id": "vocabulary-comparison",
      "path": "tests/value_accounting_vocabulary.rs",
      "role": "examined",
      "excerpt": "if published != table {\n        return Err(\"published list and ordered table differ\".to_owned());\n    }"
    },
    {
      "id": "vocabulary-amount-comparison",
      "path": "tests/value_accounting_vocabulary.rs",
      "role": "examined",
      "excerpt": "if amounts != declared {\n        return Err(\"amount table and ordered table differ\".to_owned());\n    }"
    },
    {
      "id": "undeclared-and-adverse-tests",
      "path": "tests/value_accounting_vocabulary.rs",
      "role": "examined",
      "excerpt": "assert!(compare(&published[1..], DEFINITION).is_err());"
    },
    {
      "id": "export",
      "path": "src/lib.rs",
      "role": "context_only",
      "excerpt": "pub const VALUE_ACCOUNTING_CHARGE_POINT_VECTORS: &str =\n    include_str!(\"../proposals/filament-v1/definitions/value-accounting-charge-point-vectors.json\");"
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
    }
  ]
}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Tracker publication

Published one method comment privately to QSL-2158: fdef6f59-b457-4b6a-a991-2d2eadaf2bce. Read-back passed: returned body exactly matches the draft; the YAML block parses as its JSON subset; clean=true, findings=[] and complete scope/excerpt parity match this artifact. The source-repository marker visibility=public is distinct from the explicit destination=private-linear. Initial automatic review rejected the ambiguous destination wording; a single clarified attempt succeeded. No GitHub finding, public share, branch change or label mutation occurred. The brief prohibited label polling, so existing has-review state was not independently queried.
