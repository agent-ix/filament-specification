---
id: SR-202611210022442313144108544
title: "Code review (rust-review lane, data/fixture lane, manual AC-to-test check) of FSpec PR 794 FR-831 vectors and tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; tests/protocol_correlation_vectors.rs; tests/checked_package_v2.rs; proposals/checked-package-v2/protocol-correlation-vectors.json; proposals/checked-package-v2/fixtures/protocol-correlation-dependency.json; proposals/checked-package-v2/fixtures/protocol-node-vectors.json; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/README.md"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
  - target: ix://agent-ix/filament-specification/TC-2113
    type: references
---

## Summary

Ticket: QSL-2271. Scope: the new Rust qualification test, the vectors and the complete fixture packages, the schema delta and the guard-count bump, all read at frozen head a1268905. Method: dev-tools code-review with its rust-review lane folded in, plus a data lane over the vectors and a manual check of each acceptance criterion against the tests. No reader for the new member exists, so this review judges whether the static vectors and tests are honest and reach the failures they claim. It does not judge any runtime.

quoin:gap-analysis was skipped. The diff has no production code (`src/` and `build.rs` are unchanged); only tests, proposals and spec changed. A manual acceptance-criteria-to-tests check replaced it, labelled below. It is not a full gap audit.

Verified:
- The schema delta. The four `related` records terms, the `recommend` layout rule, `recommend` in the general control operator enum and the closed `ProtocolCorrelationOccurrence` object all match FR-466-AC-19 and FR-831.
- The recommend layout in `protocol-node-vectors.json` and the guard-count bump from 20 to 21.
- PC-REL carries real records on all six forms (`m` 2,1,1,1,1,1), and PC-DEP carries the same shape under `acme/flow`.
- By hand, against the FR-322 node order (package in UTF-8 byte order, then path element by element, proper prefix first, then ordinal), the CO-B01 arithmetic is correct: rows 12,12,13,12,14,9,19 = 91; 17 control nodes scanning to 33 from Main(1) to Shipments/West(2); flags 2,1,1,1,1,1 = 7. So are the first unpaid amounts of CO-B02 to CO-B05, CO-B09, CO-B10, CO-B12 and CO-B13.
- CO-R01 to CO-R16 and CO-R18 to CO-R21 reach their stated first failure on the admitted base graph.

Baseline: `tc_2090_function_body_location_rows` (BL-06) fails rc=101 at `tests/checked_package_v2.rs:25606` with message `BL-06`, identically on merge-base 9f5b30a5 (an extracted tree, run against this PR's target dir) and on head a1268905. It is pre-existing and not caused by this PR. It lives in `checked_package_v2` and is separate from the new test file, so it masks no new-fixture defect.

Manual AC-to-test check (labelled; not a gap audit):
- AC-6 is really executed: schema first error paths.
- AC-3's six-form half is executed. Its `operator-ineligible` refusal half is not executed.
- AC-1, AC-2, AC-4, AC-5 and AC-7 are tagged by tests that execute no reader decision.
- AC-8 is tagged by a test whose result follows by construction.
- AC-9, FR-322-AC-514 and FR-466-AC-20 are untagged, which is honest.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | CO-R17 claims its foreign-package row 1 "also sorts before row 0", but package `other` sorts after `example` in UTF-8 byte order, so the vector has no key defect and cannot show that a join defect precedes a key defect of the same row (FR-831-AC-5). No other vector does. | proposals/checked-package-v2/protocol-correlation-vectors.json (reader_refusals CO-R17) |
| FND-002 | medium | Coverage refusals CO-R18 to CO-R21 record only the locus `/protocol_correlation_occurrences`. The retained control, occurrence and first uncovered `canonical_index` that FR-831 step 6 requires appear only in `why` prose, so a reader that reports the wrong control or index still matches every expectation. | proposals/checked-package-v2/protocol-correlation-vectors.json (CO-R18..CO-R21); FR-831 step 6 |
| FND-003 | medium | CO-B07 (long alias) has no `edit`: its row is prose ("alias of 64 ASCII 'a'"), an abstract projection standing in for a package case. The test only asserts the tautology `assert_eq!(1 + 5 + 64, 70)`. | proposals/checked-package-v2/protocol-correlation-vectors.json (budget CO-B07); tests/protocol_correlation_vectors.rs:285 |
| FND-004 | medium | Trace tags claim FR-831-AC-1, AC-2, AC-4, AC-5 and AC-7, and AC-8 by construction, on tests that execute no reader decision. `quire matrix` lists those criteria as `tagged`, which inflates coverage of unexecuted carrier outcomes. | tests/protocol_correlation_vectors.rs:125,177,247,323 |
| FND-005 | low | The budget test checks hard-coded constants (`costs[0]==12`, `costs[6]==19`, last scan `2`). It never reads the vectors' `first_unpaid` amount or locus or the `limit_over_baseline` of CO-B02 to CO-B13, so those recorded expectations can drift without failing. | tests/protocol_correlation_vectors.rs:286-289 |
| FND-006 | low | The coverage test's doc claims rows are "in structural order" but never checks row key order. It groups rows by `path` alone, ignoring `ordinal` and `occurrence`. Its source-map lookup hard-codes ordinal 0 and PC-REL for every group not in `acme/flow`. `scan_costs` assumes O = 1 (`1 + 1 + m`) instead of counting declaration occurrences. | tests/protocol_correlation_vectors.rs:174-244 |

## Verdict

Changes requested (four medium, two low). The schema fix, the complete packages and most of the vectors are sound and data-grounded. Fix the CO-R17 premise and pin the coverage refusal data. Make CO-B07 a concrete edit. Either drop the reader-outcome trace tags or scope them to what is executed.

## Verification limits

Rust runs, one at a time, under `flock /tmp/cargo-slot.lock` with `-j4`, a focused exact filter and rc captured before grep:
- `checked_package_v2 tc_2090_function_body_location_rows` on merge-base 9f5b30a5: rc=101.
- The same test on head a1268905: rc=101.

The author's 14 named passing tests were not rerun (receipts accepted). `quire matrix --scope .` rc=0. No reader for the member exists, so no carrier outcome was executed.

## Dispositions

Round 1. Checked against fix commit 2d7441fa and merge ce96a423 (head ce96a4235251d09d6b4c58d1e13e9c2a4d9ed679, local == remote). Merge integrity: against origin/main 73e81dce, the PR's net diff touches only its own files. The schema delta is unchanged (`protocol_correlation_occurrences`, `ProtocolCorrelationOccurrence`, the four related records terms, the recommend rule). Upstream FR-322-AC-514/515 (composition) and the claim_names composition row are preserved. This PR's criterion moved to FR-322-AC-516, and TC-2113 cites it. No stale AC-514 reference remains in FR-831, FR-466, TC-2113, the README, the vectors or the tests. `quire matrix --scope .` (rc=0) tags only FR-831-AC-6, AC-10 and AC-11. The author's receipts are accepted: 8 named `protocol_correlation_vectors` tests plus 5 named layout, manifest and family tests, all rc=0. No new concern required a rerun.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | CO-R17 row 1 now names `acme/orders`, which is not this package (a join defect) and sorts before `example` in UTF-8 byte order (a key defect). The test asserts both defects and the join locus. |
| FND-002 | fixed | CO-R18 to CO-R21 now carry `expect.retained` {control, occurrence, canonical_index}. The test recomputes the first uncovered position from each package and compares all three. |
| FND-003 | fixed | CO-B07 is now a concrete one-row edit with a 64-byte alias (104 = 1 + 39 + 64, limit 103). New CO-B15 has a 200-byte control segment, charged 232 before the join would refuse it. The tautology is gone. |
| FND-004 | fixed | The reader-outcome trace tags are removed. Only AC-6 (schema), AC-10 (vector, package and charge consistency) and AC-11 (member separateness) are tagged. AC-1 to AC-5, AC-7 to AC-9 and AC-12 are untagged and stated as pending in TC-2113 Boundaries and FR-831 Status. |
| FND-005 | fixed | Every budget case is recomputed from its own package and rows: counter, first unpaid amount, locus, limit and prior work, with G sampled at 6, 96 and 1200. No hard-coded constants remain. I checked the arithmetic by hand (rows 44,44,45,44,47,42,53; Main 31, Advised 41, West 45; B07 104; B15 232). |
| FND-006 | fixed | The full key is now compared (NodeRef in UTF-8 byte order, then occurrence ordinal, then source_ordinal). source_map is looked up by the actual (node_id, role, ordinal) in the case's package, and O is counted from the node's occurrences. The mutation test shows the check fails on swap, gap, occurrence 1, foreign package, index >= m, repeated key, missing entry and missing row. No ordinal 0, PC-REL or O = 1 is hard-coded. |
