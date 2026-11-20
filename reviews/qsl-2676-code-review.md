---
id: SR-2026112005294500000004
title: "Code review — backend provider certificate vectors"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; proposals/backend-provider-v1/schema.json, proposals/backend-provider-v1/vectors.json, tests/backend_provider_v1.rs"
review_set: subset
---

## Summary

Reviewed the changed schema branch, positive vector, and Rust qualification assertions, including the Rust review lane. The rule oracle now compares its independent 25-rule inventory with both positive vectors and schema rules; the member oracle checks enum/property-set equality and the cardinality, but it does not verify the FR-331 row's exact names.

## Verdict

**CONDITIONAL** — one medium test-oracle gap remains for FR-331-AC-46.

## Assurance Context

No applicable AssuranceProfile was found under `spec/`.

## Methods and gates

Rust review was folded into this artifact. The two focused test logs supplied for the rebased head show exit 0 for `tc_431_certificate_rules_have_positive_coverage` and `tc_431_zone_loci_admit_the_refinement_form_rules_and_the_added_members`; `git diff --check origin/main...HEAD` also exited 0. Per the review brief, no aggregate suite or aggregate gate was run. No CI workflow changed. The Quire validation warnings were checked against the supplied run: the `value_object`, two `verifies` vocabulary, and `ac:vague-response` warnings are on unchanged metadata/lines and are not introduced by this diff.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium | The member-set oracle does not verify the FR-331 row's exact names | tests/backend_provider_v1.rs:1793-1794 |

### FND-001 detail

The test compares the member enum with property names walked from the same schema, then checks that the set has 61 entries. It never checks the names in FR-331's “Certificate member” row. A same-cardinality replacement in the row, or a coordinated property/enum replacement that keeps 61 members, can therefore pass while violating FR-331-AC-46's required exact equality. The current row, schema enum, and property walker agree by inspection; the gap is in future drift detection.

## Scope

- `certificate-rejected` malformed-member schema branch: inspected the new `ef_nodes`, `form`, and `state_graph` entries; they are the three additions from FR-418-AC-17.
- Vector `certificate-rejected-non-zeno-mismatch`: inspected the positive outcome and `{member: non_zeno}` locus; it matches FR-418-AC-15.
- `tc_431_certificate_rules_have_positive_coverage`: inspected the 25-entry independent expected set and exact comparisons against vector coverage and schema rule set.
- `tc_431_zone_loci_admit_the_refinement_form_rules_and_the_added_members`: inspected the enum/property walker equality and count check; finding FND-001 is limited to the unverified textual inventory.
