---
id: SR-2026111616144800000002
title: "Code review — FSpec published test-case accessors"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; Cargo.toml, Cargo.lock, README.md, src/lib.rs, tests/published_files.rs, FR-660, TC-1640"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: references
---

## Summary

Reviewed PR #689 at frozen head `7014ba13cad58c2e1b465367d3a379d633495b6c` against fresh `main`. The published-file implementation and the new test-case bytes/listing check are consistent with FR-660-AC-6; the release-version equality clause in FR-660-AC-1 remains unasserted by its tagged test.

## Verdict

**CONDITIONAL** — one medium test-coverage finding; no implementation defect was found in the changed API documentation or test-case access path. The package version/tag relationship should be verified before QSL-2365 closes.

## Findings

| ID      | Severity | Summary                                                        | Refs                       |
| ------- | -------- | -------------------------------------------------------------- | -------------------------- |
| FND-001 | medium   | FR-660-AC-1 release-version equality has no executable assertion | tests/published_files.rs:28 |

## Review notes

- `build.rs` on fresh `main` generates the published byte table from both `proposals/` and `spec/test-cases/`, then sorts the repository-relative paths. `published_file` performs exact path lookup and `published_files` preserves that sorted order.
- The added test compares every test-case file byte-for-byte, checks the TC-210 path, and compares the test-case prefix listing with the full source-tree list. The test is tagged to FR-660-AC-6.
- `tests/published_files.rs` retains the FR-660-AC-1 tag but checks proposal bytes and path absence only. `tests/dependency_pins.rs` checks dependency source declarations, not that this package's version equals the release tag. Consequently the AC's version/tag clause can drift without these tests failing. PR #689 intentionally defers creating `v0.1.1` until merge; verify that tag against package version before closing the ticket.
- `rustfmt --edition 2024 --check src/lib.rs tests/published_files.rs` passed. Repository-wide `cargo fmt --all -- --check` reports formatting differences in many unchanged files. `git diff --check` passed.
- `quoin matrix --repo . --json` was unavailable because the installed architecture module rejected its `record_keys` field; `quire matrix --scope . --format json` succeeded and showed FR-660-AC-1 and FR-660-AC-6 tagged. The PR body reports the focused `published_files` and `dependency_pins` tests passed (3 passed, 0 failed). An independent rerun remained queued on all six locked-build lanes and was canceled before Cargo launched. A targeted Clippy rerun also remained queued and was canceled before Cargo launched.
- The code is mergeable with this non-blocking coverage finding. QSL-2365 still needs the post-merge `v0.1.1` tag and language pin update before ticket completion.

## Dispositions

| FND     | outcome | reason |
| ------- | ------- | ------ |
| FND-001 | fixed   | Added an assertion that the Cargo package version equals the planned `v0.1.1` release tag without its `v`. |
