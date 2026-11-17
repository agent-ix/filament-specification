---
id: FR-660
title: "Version the checked-package wire, the standard's published files and the consumer facades, and pin them through Cargo"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-002
    type: implements
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-661
    type: references
---

# FR-660: Version the checked-package wire, the standard's published files and the consumer facades, and pin them through Cargo

## Description

Each Filament repository that produces or consumes the
`filament.checked-package/v2` wire, the standard's published files or a
producer's public facades SHALL build against releases of the other
repositories that it names through Cargo. A change in another repository then
reaches its build only when it moves its own pin.

## Rationale

The wire, the shared vectors and the public facades are shared by several
repositories. A dependency that follows another repository's moving branch, or
a test that reads the standard from a checkout next to the consumer, breaks the
consumer's build on every upstream change and turns each break into an
adaptation ticket. A version that the consumer pins makes the moment of
adaptation the consumer's choice, and the version number says whether any
adaptation is needed.

## Inputs

A release of the standard, a release of a producer repository that publishes
consumer facades, and a consumer's Cargo manifest and lockfile.

## Outputs

The consumer's build resolves every cross-repository dependency to one released
version, and its readers that refuse wire documents of any other document type.

## Behavior

### What is versioned

1. **The checked-package wire.** The wire has two version facts. The
   `contract_version` member (FR-322) is the document type: a reader admits only
   the one document type its pinned standard release defines. The revision of
   the wire's closed sets and rules (node tags, members, operation identities,
   refusal codes, reader order) is the release version of the standard that
   publishes them. A document type changes, for example from
   `filament.checked-package/v2` to `filament.checked-package/v3`, only when
   bytes admitted under the old type would be read with a different meaning.
   Any other revision keeps the document type and is a new release of the
   standard. Because readers are closed, an extension of a closed set (a new
   node tag, member, operation identity or refusal code) is a breaking release:
   an older reader refuses a document that uses it with its existing typed
   refusal (`unsupported_node_tag`, `unknown_member`, `malformed_wire`) and
   never reads it with another meaning.
2. **The standard's published files.** The standard is the Cargo package
   `filament-specification-qualification` (library `filament_specification`).
   Its published files are every file under `proposals/` (the wire schemas,
   fixtures, vector files, catalogs and the consumer-fact inventory (FR-661))
   and every file under `spec/test-cases/` (the test cases that a consumer reads
   by path, such as `spec/test-cases/TC-210-finite-simulation-reproducibility.md`).
   The library SHALL expose each published file's exact bytes by its
   repository-relative path through `published_file(path)`, which returns
   `None` for any path outside those two trees, with the same exact-bytes rule
   for both, and `published_files(prefix)`, which lists the
   published files under a directory prefix in path order. A release of the
   standard is a git tag `vX.Y.Z` whose package `version` is `X.Y.Z`.
3. **A producer's consumer facades.** A producer repository that publishes the
   public modules named in its consumer-facade bindings (FR-661) versions them
   with the repository's release. Every crate that holds such a module carries
   the repository's workspace version, and the crates are released together
   under one git tag `vX.Y.Z`. One version per repository is chosen because a
   git dependency resolves a whole repository at one commit, so separate crate
   versions inside one repository would not let a consumer pin them apart.

### How a consumer pins

A cross-repository dependency between Filament repositories SHALL be a Cargo
dependency that names a release: a git dependency with `tag = "vX.Y.Z"`, or a
registry dependency with a `version` requirement. A `branch` dependency is not a
pin and SHALL NOT appear in a committed manifest. `Cargo.lock` records the
resolved commit. The Cargo manifest and lockfile are the only record of which
release a repository builds against: no source file, test, Makefile, script or
document holds a version table, a hash, a digest or a ledger of expected
versions of another repository. Co-development of two repositories uses a local,
uncommitted Cargo `[patch]` override; it is never committed.

A consumer reads the standard's published files only through the pinned
`filament_specification` library (`published_file`, `published_files` and the
named constants). No consumer locates the standard through an environment
variable, a sibling checkout or any other filesystem path.

### What a version bump promises

Releases follow Cargo's semantic-version rules: while the major version is `0`,
the minor position is the breaking position and the patch position is the
compatible position.

- **Breaking.** A release is breaking when it changes the expected result
  (admission, refusal, code, cause, locus, stage or value) of an existing
  published case; removes or renames a published file, case, constant or
  accessor; extends or narrows a closed wire set; changes the meaning of a
  consumer fact (FR-661) or the signature or meaning of a bound accessor;
  removes an inventory fact; or, for a producer, moves its standard pin to a
  breaking standard release.
- **Compatible.** A release is compatible when it only adds: a published file,
  a case that exercises already-specified behavior, a constant or accessor, or
  an inventory fact whose accessors exist.
- **Editorial.** A change that alters no published byte that a case or
  accessor reads is a compatible release.

A consumer that pins a release keeps building against it whatever the upstream
repository merges. A compatible release asks nothing of the consumer. Moving a
pin across a breaking release is the consumer's adaptation, and its scope is the
breaking changes that release's notes list; each listed change names the
requirement, published case or inventory fact it changes. Adapting to a new
release is therefore one bounded change per breaking release the consumer
chooses to take, not a change per upstream commit.

### Producers and readers of the wire

A producer writes the document type its pinned standard release defines, and a
reader admits that one document type. A reader refuses a document with any
other `contract_version` as `unknown_contract_version`, with no cause, at
`/contract_version`, infers nothing from the rest of the document and returns no
partial package (FR-322 "Identity and validation"). A dependency package read
during linking is a wire document like any other: one of another document type
refuses before it is linked.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-660-AC-1 | The standard's package `version` equals the release tag without its `v`; `published_file` returns the exact bytes of every file under `proposals/` by its repository-relative path, and `None` for a path outside `proposals/` and `spec/test-cases/` and for a path under them that names no file; `published_files("proposals/checked-package-v2/fixtures/")` lists exactly the files of that directory in path order. | Test (TC-1640) |
| FR-660-AC-2 | In a consumer of the standard's published files, `cargo metadata --locked` lists `filament-specification-qualification` from a git source with a `tag` or from a registry, every published vector the consumer runs is read through the pinned library, and no Makefile, build script, source or test of the consumer reads an environment variable or a filesystem path to locate the standard. | Test (TC-1641) |
| FR-660-AC-3 | Every cross-repository Filament dependency in a committed manifest names a `tag` or a registry `version`; a manifest with a `branch` dependency on a Filament repository fails the consumer's pin check, naming the dependency, and no file outside `Cargo.toml` and `Cargo.lock` records an expected release version, hash or digest of another repository. | Test (TC-1642) |
| FR-660-AC-4 | A reader built against a standard release refuses the release's positive fixture with `contract_version` replaced by `filament.checked-package/v3`, and by `filament.checked-package/v1`, as `unknown_contract_version`, with no cause, at `/contract_version`, and returns no package; a producer built against the same release writes exactly the release's document type. | Test (TC-1643) |
| FR-660-AC-5 | A consumer's resolved standard dependency, as `cargo metadata --offline` reports it, comes from a git source with a `tag` or from a registry, and its package `version` is a semantic version `X.Y.Z` that equals the tag without its `v`, so Cargo's compatibility rules carry the release's bump promise; a resolved `branch` source fails, naming the dependency. | Test (TC-1644) |
| FR-660-AC-6 | `published_file("spec/test-cases/TC-210-finite-simulation-reproducibility.md")` returns that file's exact bytes, and `published_files("spec/test-cases/")` lists the files of that directory in path order. | Test (TC-1640) |

## Dependencies

- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns the
  wire, its document type and the `unknown_contract_version` refusal.
- [FR-661](FR-661-hold-a-consumer-fact-inventory.md) owns the consumer-fact
  inventory whose facts this requirement versions.
