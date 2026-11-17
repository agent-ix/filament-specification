---
id: TC-1640
title: "Standard release version and published-file accessors"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-660
    type: verifies
---

# TC-1640: Standard release version and published-file accessors

## Description

Verify that the standard's package version matches its release tag and that the library exposes every published file by path. Scope: FR-660-AC-1, FR-660-AC-6.

## Test Procedure

Read the package `version` from `Cargo.toml` and the release tag of the commit. Walk `proposals/` and, for every file, call `published_file` with its repository-relative path and compare the bytes with the file. Call `published_file` with `spec/functional/ecosystem/FR-660-version-the-wire-the-published-files-and-the-consumer-facades.md` and with `proposals/checked-package-v2/fixtures/absent.json`. Call `published_files("proposals/checked-package-v2/fixtures/")`.

## Expected Results

- The version equals the tag without its `v`.
- Every file's bytes are returned exactly.
- Both other paths return `None`.
- The TC-210 file's bytes are returned exactly, and the `spec/test-cases/` listing holds exactly that directory's files, in path order.
- The listing holds exactly the fixture directory's files, in path order.
