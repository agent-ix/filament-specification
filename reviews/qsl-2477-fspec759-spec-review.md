---
id: SR-2026111921293000000001
title: "spec-review of QSL-2477 FR-520 analyze result schema and canonical bytes"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #759 at 690e5c09; spec/functional/tooling/FR-520-carry-analyze-requests-and-results-as-documents.md (Schema and Canonical bytes bullets, Result schema section, FR-520-AC-8, Dependencies), spec/test-cases/TC-2096-an-analyze-result-validates-against-the-outcome-schema.md; context: FSL PR #3102 at d08c7eaa (FR-286 Published schema, Canonical encoding), FSL fsl-analyze/src/analyze/document.rs write_run and item_document, FSL FR-621, FR-990, FR-1032, FR-1031"
review_set: subset
---

## Summary

Spec review of FSpec PR #759. Ticket: QSL-2477 (the fix-hint half, QSL-2482, is FSL-only). The PR makes the analyze result a `filament-outcome/1` document that validates against FSL's published schema `urn:filament:outcome:1`, requires RFC 8785 bytes, states that request member order does not reach the result, and adds FR-520-AC-8 with TC-2096. `quire validate` passes on both files. FR-520-AC-8 and TC-2096 are not on origin/main (highest FSpec TC there is TC-2084) and no open FSpec PR uses them.

## Examined scope

- Result schema closure. The section says the schema's `analyze` branch describes each item "with the members 'Result items' lists ... and closes every object it describes". FSL's analyze writer (`fsl-analyze/src/analyze/document.rs` `item_document`) also writes `obligations`, `closure`, `global`, `projections` and `recording` (FSL FR-990, FR-1032) and simulation items carry FSL FR-634's members. None of these is in "Result items", so a schema built from this text rejects real conformance and simulation results (FND-001). (examined)
- Canonical bytes and permutation. RFC 8785 is compatible with the result's numbers: FSL writes counts, positions and limits as decimal strings, and quire-canonical refuses only integers past 2^53. The permutation clause is too broad: refusals that name the first defect in request order (FSL FR-1031 for `refinement_transfer`, first unknown member) change with a permutation even when they name one defect (FND-002). (examined)
- FR-520-AC-8. Each mutation fails at the named pointer under a closed 2020-12 schema (`request_index` is required on items, `proven` is not a category, `aggregate` is required at the root by the `analyze` branch). The reversed-member request makes the AC fail for an order-preserving writer. It can fail. (examined)
- TC-2096 matches AC-8 step by step. (examined)
- Dependency edge `depends_on` FSL FR-286 resolves to the FR that publishes the schema. (context_only)

## Verdict

Not mergeable. FND-001 must be fixed in this PR: the closed item member set must be the set FSL writes (FR-621's members plus FR-990/FR-1032's conformance members and FR-634's simulation members), or the section must defer the item member set to FSL FR-286 instead of "Result items". FND-002 should be narrowed in the same round, together with the matching FSL FR-286 sentence. AC-8 itself is sound.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | "Result schema" closes each result item to the members "Result items" lists, but FSL writes `obligations`, `closure`, `global`, `projections`, `recording` (FR-990, FR-1032) and FR-634 simulation members on analyze items; the closed schema the text specifies rejects real results | spec/functional/tooling/FR-520-carry-analyze-requests-and-results-as-documents.md:133-144 |
| FND-002 | medium | "a request and any permutation of its object members give byte-equal results" except multi-defect refusals is false for first-defect refusals chosen in request order (FSL FR-1031 "at the first member in request order"); a two-defect request refused naming one defect changes with a permutation | spec/functional/tooling/FR-520-carry-analyze-requests-and-results-as-documents.md:146-151 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | "so the schema and the writers cannot drift apart" overstates FSL FR-286-AC-31: the completeness test checks only the member paths of hand-built full-member values, so a member a writer adds without a fixture change is not caught (see the FSL review's FND-015) | spec/functional/tooling/FR-520-carry-analyze-requests-and-results-as-documents.md:151-154 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | Round 1 at 2fcf725c added the conformance members, the record's settled_by/backend/non_zeno/memory_bound/trace/steps, finite-replay disposition and root aggregate/registry. The simulation item is still listed as "simulation settings, its traces, coverage and frontier", but FSL writes `findings` (FR-634 "Invariant findings", always present, `[]` when none) and `cause` on simulation items (fsl-analyze/src/analyze/simulation_item.rs:369-372, 441-454, 553-556, 622-636). A closed schema built from this list rejects every simulation result |
| FND-002 | fixed | - |
