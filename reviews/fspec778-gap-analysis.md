---
id: SR-2026112105041302
title: "Gap analysis: PR 778 code-lane delta, FR-322-AC-490/510/512 test coverage"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-specification; git diff a34bb5cc..6313e9a5: tests/checked_package_v2.rs, proposals/checked-package-v2/checked-term-data-vectors.json against FR-322-AC-490, FR-322-AC-510, FR-322-AC-512"
review_set: subset
---

## Summary

Ticket: QSL-2691. A planless criterion-to-test check over the delta's acceptance criteria, done by grepping the `#[trace]` tags and reading the assertions. The repository has no production reader, so the tests' deciders are the executable statement. Plan completion: not assessed.

- FR-322-AC-512 is tagged on TC-2099 and fully asserted: IC-14 admitted, five mutations at their loci, and IC-21/IC-15 refused through the binder link.
- FR-322-AC-490 is tagged on TC-2090. Its new binder-link clause for the f/h `let`s is not asserted.
- FR-322-AC-510 (Test TC-2090) has no trace tag anywhere in `tests/`. TC-2090 asserts its BL-03 encoding, BL-R9/R10 and BL-S5/S6, but BL-D1 and BL-D2 (the deep-body vectors that QSL-2691's "Done when" names) are exercised by no test.

## Verdict

**CONDITIONAL**: two medium coverage gaps, both fixable inside this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-510 is untraced and partly untested. `tc_2090_function_body_location_rows` is tagged `#[trace("TC-2090", "FR-322-AC-490")]` only, so the matrix shows AC-510 with no test even though TC-2090 asserts most of it. BL-D1 (100,000 left-nested `+`, at most 200,000 entries, k-th path k zeros) and BL-D2 (100,000 nested `let`s, `{up: 1, down: [1, 0]}` after the first, path k ones then a zero) are never built or run. | tests/checked_package_v2.rs:25561; proposals/checked-package-v2/checked-term-data-vectors.json body_locations.deep_body |
| FND-002 | medium | FR-322-AC-490's binder-link clause is unasserted. No test checks that f's lets (nodes 18, 19) and h's lets (17, 18) carry `binder` naming the `#ieee_operand` parameter of their level, with that node among their `dependencies`; TC-2090 only schema-validates the packages and runs the body-location decider. A regression that drops `binder` from the f/h vectors would still pass. The existing `let_binder_link_refusal` cannot be reused as is (code-review SR-2026112105041301 FND-001). | tests/checked_package_v2.rs:25563-25700 |

## Scope

- FR-322-AC-490 (examined; FND-002)
- FR-322-AC-510 (examined; FND-001)
- FR-322-AC-512 (examined, clean)

## Dispositions

Round 1, against 5b70cf1a (delta 6313e9a5..5b70cf1a).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `tc_2090_function_body_location_rows` is now tagged `FR-322-AC-510`. A new `tc_2090_deep_body_vectors` (tagged TC-2090, FR-322-AC-510) builds BL-D1 and BL-D2 packages and runs them through `decide_body_locations` at 1,000 terms, asserting that the k-th rebuilt path is k zeros (BL-D1) or k ones then a zero (BL-D2). It also replays the rule at the vector's 100,000 terms, checking path length, ending entry and at most 3 entries per row. |
| FND-002 | fixed | TC-2090 now asserts, for f lets 19/18 and h lets 18/17, that `binder` names the `#ieee_operand` parameter node (5 and 7), the binding name equals the parameter name, the binder is in `dependencies` and is an operand of the let's path, and the inner let binds the next level. `let_binder_link_refusal` returns `None` over both packages. |

Round 2, against 83af4f90 (delta 5b70cf1a..83af4f90).

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | 5425bd12 asserts `entries <= 200_000` ("at most 200,000 up and down entries together"), counting `up` and `down` together as AC-510 states. The actual count is 199,999. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | BL-D1's size bound is checked more weakly than AC-510 states. The AC says the 100,000 rows' "`up` and `down` entries number at most 200,000 together". The test asserts `entries - full <= 200_000`, which counts only the `down` entries (99,999), and `entries <= 3 * full`. Neither fails if the `up` values plus the `down` entries exceed 200,000. The AC's bound is `entries <= 200_000` (actual: 199,999). | tests/checked_package_v2.rs (tc_2090_deep_body_vectors, final asserts) |
