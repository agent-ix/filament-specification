---
id: SR-2026112005294500000009
title: "QSL-2222 PR781 spec-review/base"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR781 at 87bd8c4458383ca63c99d476980335073f10dc7d; seven normative files, backend-provider schema/vectors and tests/backend_provider_v1.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-specification/FR-368", type: references }
---

## Summary

Ticket: QSL-2222. Independent spec-review/base at the frozen head. No applicable AssuranceProfile was found; direct review applies. No branch edits, Cargo builds or aggregate gate were performed.

## Verdict

PASS for the scoped normative requirement edits; code finding remains independently blocking.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Coverage

Reviewed identity, vocabulary, limits, maximum/proof permutations, source anchor and no-profile refusal rules. AC-27 and BP-64 explicitly qualify authored oracle shape, not native evaluation; no native producer pass is claimed. Base/derived item cardinalities and provider item kind mapping are coherent.

Every changed file was examined. Scope data below records clean as well as defective units. Authored schema/vector checks do not discharge downstream strict-reader or native compiler qualification. The new fixture adds six declaration/0 occurrences without extending the base's 83-entry source_map; none of those six node keys appears in the base mapping, and no patch writes /source_map.

```json
{
  "scope": [
    {
      "id": "FR-368-AC-13",
      "path": "spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md",
      "role": "examined",
      "excerpt": "The `item_kind` of a temporal item is one of `formula`, `deadlock-freedom`, `time-lock-freedom`, `race-freedom`, `loop-invariant`, `loop-termination` and `refinement`; each of the seven is admitted, and `time-lock`, `race`, `deadlock`, `refine`, `Formula`, `loop`, `loop-variant` and `loop_invariant` are each refused before consumption."
    },
    {
      "id": "FR-368-AC-15",
      "path": "spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md",
      "role": "examined",
      "excerpt": "A `deadlock-freedom`, `time-lock-freedom`, `race-freedom`, `loop-invariant` or `loop-termination` item whose `fairness` holds a constraint is refused before consumption; a `refinement` item carries the concrete fairness set."
    },
    {
      "id": "FR-368-AC-26",
      "path": "spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md",
      "role": "examined",
      "excerpt": "A protocol subject whose protocol holds `repeat A` with the proof `invariant { left >= 0 } variant { left }` and no `maximum`, `repeat B` with a proof and a `maximum` of 3, and `repeat C` with an empty `proof` derives a `loop-invariant` and a `loop-termination` item for `A`, a `loop-invariant` item only for `B` and no item for `C`, each with an empty `fairness`;"
    },
    {
      "id": "FR-368-AC-27",
      "path": "spec/functional/temporal/FR-368-negotiate-temporal-model-check-items.md",
      "role": "examined",
      "excerpt": "A protocol subject over a protocol clause of a unit that selects no protocol profile, whose protocol holds a `repeat` with a non-empty `proof`, has no anchor for the `repeat`'s loop items: each is settled `unsupported`, `unsupported_projection`/`unsupported-requested-capability`, with a `detail` naming the item kind and that the unit selects no protocol profile, by the producer, which writes no request item for it."
    },
    {
      "id": "FR-322",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "examined",
      "excerpt": "`source_map` — the total semantic-occurrence to original source-region mapping. An occurrence key is (`node_id`, `role`, `ordinal`), where role is `declaration`, `type`, `expression`, `anchor`, `claim` or `generated`;"
    },
    {
      "id": "FR-171",
      "path": "spec/functional/protocol/FR-171-evaluate-dynamic-control.md",
      "role": "examined",
      "excerpt": "A loop with a maximum terminates by that maximum and has no termination obligation."
    },
    {
      "id": "FR-290",
      "path": "spec/objects/protocol/FR-290-protocol-claim-kind.md",
      "role": "examined",
      "excerpt": "a loop item anchored on a `protocol`/`control` node, is `temporal-trace`."
    },
    {
      "id": "FR-520",
      "path": "spec/functional/tooling/FR-520-carry-analyze-requests-and-results-as-documents.md",
      "role": "context_only",
      "excerpt": "`temporal.item_kind` is `formula` (the default), `deadlock-freedom`, `time-lock-freedom`, `race-freedom`, `loop-invariant`, `loop-termination` or `refinement`, as FR-331 places it."
    },
    {
      "id": "TC-431",
      "path": "spec/test-cases/TC-431-backend-provider-wire-vectors.md",
      "role": "examined",
      "excerpt": "This checks the authored settlement oracle, not native producer execution."
    }
  ]
}
```

## Dispositions

Disposition round 1 inspected both original findings. Every new control has one explicit source-map row under the locked example identity; the test checks all graph occurrences and exact loop claim pairs, then removes Bounded's row and confirms resolution fails. No new finding. Lead evidence: 42 provider tests pass and focused Clippy/Quire pass; the regression failed on the original unmapped fixture before the repair. No build or source edit by this reviewer. The normative spec and the explicit limitation that AC-27 qualifies authored settlement expectations are unchanged.
