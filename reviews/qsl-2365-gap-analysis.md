---
id: SR-2026111616144800000003
title: "Gap analysis — FSpec published test-case accessors"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-specification; FR-660-AC-1, FR-660-AC-6, build.rs, src/lib.rs, tests/published_files.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: references
---

## Summary

The scoped manual acceptance-to-test check confirms the new test-case file accessor behavior and identifies a missing assertion for the release-version equality clause. The Quoin matrix analysis could not run because the installed architecture module fails to load; the available Quire matrix confirms both reviewed criteria have trace binders.

## Verdict

**CONDITIONAL** — the changed path-family requirement has test evidence, but FR-660-AC-1's package-version/tag relation is not asserted and the full Quoin gap-analysis method was unavailable.

## Findings

| ID      | Severity | Summary                                                        | Refs                       |
| ------- | -------- | -------------------------------------------------------------- | -------------------------- |
| FND-001 | medium   | FR-660-AC-1 release-version equality has no executable assertion | tests/published_files.rs:28 |

## Coverage

- Manual check: `published_file` returns exact test-case bytes and `published_files("spec/test-cases/")` returns the sorted path set; the new test covers TC-210 and carries the FR-660-AC-6 trace tag.
- `quire matrix --scope . --format json` reports FR-660-AC-6 tagged by `published_file_returns_test_case_bytes_and_lists_paths_in_order` and FR-660-AC-1 tagged by `published_file_returns_every_proposal_byte_and_lists_fixture_paths_in_order`.
- Quoin gap analysis was unavailable: `quoin matrix --repo . --json` exited 2 with `CORE_REFUSED/QQ-1010` while loading `spec-objects-architecture` (`record_keys` is not accepted by the installed module schema). The manual check does not claim repository-wide reverse-gap or stub coverage.
- The PR body reports the focused `published_files` and `dependency_pins` tests passed (3 passed, 0 failed); an independent rerun remained queued on all six locked-build lanes and was canceled before Cargo launched.
- Plan completion: not assessed

## Dispositions

| FND     | outcome | reason |
| ------- | ------- | ------ |
| FND-001 | fixed   | Added an assertion that the Cargo package version equals the planned `v0.1.1` release tag without its `v`. |
