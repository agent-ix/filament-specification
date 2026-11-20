---
id: SR-2026112005183200000002
title: "spec-review of completion spec batch 1, FSpec PR #766"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #766 at 272e75a7; git diff origin/main...HEAD: proposals/filament-v1/shared-grammar.md (align-skip), spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md, spec/functional/source-package/FR-501-settle-ambiguous-and-contradictory-grammar-forms.md, spec/test-cases/TC-1008-operation-names-and-member-lists.md, spec/test-cases/TC-373-hybrid-verdict-map.md; context: FR-149, FR-398, TC-337, FSL #3143 FR-172/FR-450"
review_set: subset
---

## Summary

Spec review of FSpec PR #766, completion spec batch 1. Tickets: QSL-2599 (align skip) and QSL-2529 (FR-193 past operators and infimum). The PR was checked against FSL #3143's FR-172 and FR-450 and the arch rulings.

## Examined scope

- shared-grammar `align-skip`, FR-501 item 8, FR-501-AC-8 and AC-9, and TC-1008 (examined): clean. The empty list parses and FR-172 refuses it at S3. This agrees with FR-398-AC-3, TC-337 HB-09 and FSL FR-172-AC-9. The FR-501-AC-8 text that was removed reappears in AC-9, so nothing is dropped. The trailing-comma case agrees with `list()` (fsl-cst/src/grammar.rs:179-183), which leaves the comma to the outer rule.
- FR-193 "On each" past-operator bullet (examined): clean. It agrees with FSL FR-450.
- FR-193 "Trace replay" decisive-instant paragraph (examined): FND-002 and FND-003.
- FR-193-AC-28 and TC-373 HY-22 (examined): FND-001.
- Ids: FR-193-AC-28 and FR-501-AC-9 are free on origin/main and in every open FSpec PR. No file is shared with batch 2 (#767) or with any other open PR.
- `quire validate --scope .` exits 0 on every changed file except the known `proposals/filament-v1/shared-grammar.md` frontmatter failure.

## Verdict

Not mergeable as is. FR-193-AC-28's capture is never out of range, and the decisive-instant paragraph gives a different kind than AC-28 expects. Both need the same fix as FSL #3143 FND-001 and FND-002. The align-skip half is clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-193-AC-28: `capture v0: Rational[0, 1; 1, 2] = q.v` is never out of range. The type's members are 0, 1/2 and 1, so its value range is [0, 1] (FR-149 "Bounded rationals"), and FR-193 applies a capture's value range only. AC-28's "no value exactly for the activation instants in (1/2, 1)" is therefore false, and the expected `undefined-evaluation` cannot occur. Replace with `capture v0: Rational[0, 1; 1, 2] = q.v + 1/2; eventually [0, 1] holds(h.v + 1/2 >= v0)`, whose capture leaves [0, 1] exactly for τ in (1/2, 1). Same edit in TC-373 HY-22 | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:863 |
| FND-002 | high | The text contradicts AC-28. "if an expression has no value at the decisive instant ... `undefined-evaluation`" together with "An expression with no value only after the decisive instant changes nothing" makes the open-below case (no value only after the infimum 1/2) not `undefined-evaluation`, while AC-28 expects `undefined-evaluation`. Replace the second sentence with: "When the decisive instant is the infimum of an open set, the kind is that of the set: `undefined-evaluation` when an expression has no value at every instant of some interval `(d, d + e)`, `formula` otherwise; outside that case an expression with no value only after the decisive instant changes nothing." | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:797-805 |
| FND-003 | medium | Point 4: "as the activation instants of an `on each` obligation that fails exactly on an open stretch are" makes the decisive set the activation instants. That holds only for a capture, which is evaluated at activation. For a formula failure the decisive instants are the bad-prefix ends; for FSL FR-750-AC-12(a) they are (3/2, 2). Write "as the no-value instants of a capture that is out of range exactly on an open stretch of activation instants are; for a formula failure the decisive instants are the ends of the bad prefixes, not the activation instants" | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:791-794 |
