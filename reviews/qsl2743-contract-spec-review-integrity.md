---
id: SR-2026112106460400000012
title: "spec-review/integrity review of FR-830 selected model-union and model-tuple witness rows"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-830-carry-selected-model-union-witness-rows.md; spec/test-cases/TC-2112-model-union-witness-rows-join-the-selected-document.md; proposals/checked-package-v2/model-union-witness-vectors.json; QSL-2743; PR 789"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-830
    type: references
---

## Summary

Ticket: QSL-2743. PR agent-ix/filament-specification#789 at fb65d9db. This review checks completeness, consistency and atomicity of the FR-830 contract and its TC-2112 procedure, judged against the adopted arch ruling (separate `model_unions`/`model_tuples` sidecar channels, I04 unchanged, one shared native Work/Storage budget, full vs reached, the FR-531-AC-13 boundary words).

Clean: closed row shapes, both channels required, step order 1..7 with tuples before unions, coverage last, the atomic result, the baseline-relative budget policy, no static totals, and no invented native amounts or locus source. The authority boundary uses the exact ruled words (FR-830:280). Each AC maps to TC-2112 and to named vector groups.

## Verdict

**CONDITIONAL.** Three consistency gaps make parts of AC-2, AC-11 and the step 7 locus impossible to verify as stated (FND-001..FND-003).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Under `reached`, "a row outside the closure is not inspected, charged or retained" (FR-830:134-135, AC-2 "neither inspected nor charged") contradicts step 1. Step 1 refuses `Malformed` on rows "not ascending in `structural_node` order in either list", which reads every row's node, and a closed-format failure is "the identity-file reader's `Malformed`", which parses the whole file. A conforming reader cannot both check order/format over all rows and leave out-of-closure rows uninspected and uncharged. No budget vector asserts the "not charged" half either: nothing compares MUW-03's consumption to MUW-02's baseline. State which inspection and charges apply to an out-of-closure row (format/order only, then no join or retention), and add that budget relation. | FR-830:133-137,178-187; FR-830-AC-2; model-union-witness-vectors.json:542 (MUW-03) |
| FND-002 | medium | MUW-B-02 probe 3 (`{work: E-1}` with E "the number of work charges the closure takes") expects "the denial falls on a closure edge before any row is read". E is a count, not an amount (charges may exceed 1 unit). FR-830 orders the preflight only "before the first row" and closure edges only "before any row", without ordering the two. So the expected locus is not derivable, and the limit is an absolute value where the stated policy is baseline-relative. State the preflight-vs-closure order and express the probe relative to the measured consumption at the last closure-edge charge. | FR-830:249-260,273-276; model-union-witness-vectors.json:7879 (MUW-B-02) |
| FND-003 | low | Step 7 places a `reached` absent row "at its node" (FR-830:237-238), but R-36, R-37, R-38, R-44, R-47 and TC-04 assert `locus.pointer` `/model_unions` or `/model_tuples`, and TC-2112 compares `locus.pointer`. The contract gives no sidecar pointer for an absent row, so the asserted pointer is a vector-only choice. State the pointer (the list member) beside the structural node locus. | FR-830:170-174,234-238; model-union-witness-vectors.json:7125 |

## Scope

- FR-830 Carrier and scope (examined; FND-001)
- FR-830 Checks steps 1-7 (examined; FND-001, FND-003)
- FR-830 Work and storage (examined; FND-002)
- FR-830 Authority boundary (examined, clean)
- FR-830-AC-2, AC-11 (examined; FND-001, FND-002)
- FR-830-AC-1, AC-3..AC-10, AC-12..AC-15 (examined, clean for integrity; vector defects in SR-2026112106460400000010)
- TC-2112 Test Procedure steps 1-3 and Expected Results (examined, clean)
- /tmp/ir-qsl2743-arch-ruling.txt (context_only)
