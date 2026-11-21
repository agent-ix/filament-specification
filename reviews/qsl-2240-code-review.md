---
id: SR-20261121181000001
title: code-review review of source zone transport
type: SpecReview
analysis: code-review
scope: agent-ix/filament-specification;
  spec/objects/interfaces/FR-418-zone-certificate.md,
  proposals/backend-provider-v1/schema.json,
  proposals/backend-provider-v1/zone-certificate-vectors.json,
  tests/zone_certificate_wire.rs
review_set: subset
---

## Summary

Ticket: QSL-2240. Independent bounded review of filament-specification#799.

## Verdict

**PASS** — No finding in the examined PR scope; runtime and broader assurance limits below apply.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Static Rust/schema review: the changed test invokes the real schema registry and shared JCS encoder. Negative cases cover negative/null part, unknown comparison, zero denominator, delay plus part, and unknown edge fields. The object guard on the existing transition union keeps null from matching the loose protocol branch. Existing source identity, finite template, protocol and temporal branches retain their schemas. No production algorithm or proof basis changes.

Frozen HEAD and GitHub PR head both 6ad8263b96549652186a66cc5f233acddf6af335; worktree clean. Diff base: 4cb190856402eaf7a2a75fa62838b8d1b76b4924. Four changed files only. No applicable AssuranceProfile was found in spec/. No source, branch, or PR edits were made.

No Cargo/build/test/Python jobs were started. The initial admission hold was later lifted by the lead; existing meaningful focused logs made duplicate runtime jobs unnecessary. Existing raw logs fspec-source-transport-fixed.log and fspec-source-regression.log show 1/1 and 2/2 tests passing, and their .rc files each contain 0. These are inspected prior runs, not new independent runtime verification. Quire FR-418 validation returned 0 with existing disallowed-edge-type warnings and 1/1 grammar-clean. git diff --check reports a trailing blank line at tests/zone_certificate_wire.rs:393 (rc 2); cosmetic, no contract consequence.

## Scope

```yaml
scope:
  - id: FR-418-AC-14
    path: spec/objects/interfaces/FR-418-zone-certificate.md
    role: examined
    excerpt: "A task-product certificate may use `closed-no-release-product/v1` only
      with `live_job_cap: 0`, closure
      `recompute-all-source-successors-no-task-release/v1`, checked task demand
      intervals, no scheduler clock metadata and no task-transition edge; the
      reader recomputes every source successor from every initial state and
      accepts this closure only when no checked release binding is reachable.
      Its source-context transport preserves every LU row with its original
      clock-place identity, source guard part, horizon constraint and delay-only
      edge. Clock-place names come from the original source'"
  - id: FR-418/edges
    path: spec/objects/interfaces/FR-418-zone-certificate.md
    role: examined
    excerpt: "For each node and each recomputed source transition or finite
      task-transition template enabled from it, the index of a target node. A
      source edge retains its optional zero-based convex guard `part` and
      optional `horizon` array of original place-index, operator and exact
      rational-bound constraints. A delay-only edge has `transition: null` and
      no `part`; it retains any horizon fragment. Absence of a horizon fragment
      differs from an empty fragment. Source operation identities are unchanged.
      |"
  - id: ZoneCertificate.edges
    path: proposals/backend-provider-v1/schema.json
    role: examined
    excerpt: '"transition": {"description": "The unchanged source identity or finite
      task template; null is a delay-only edge and never an operation."'
  - id: ZoneHorizonConstraint
    path: proposals/backend-provider-v1/schema.json
    role: examined
    excerpt: '"required": ["place", "operator", "bound"]'
  - id: source_transport_vectors/ZC-31
    path: proposals/backend-provider-v1/zone-certificate-vectors.json
    role: examined
    excerpt: '"id": "ZC-31"'
  - id: tc_371_source_transport_preserves_fragment_and_delay_shapes
    path: tests/zone_certificate_wire.rs
    role: examined
    excerpt: fn tc_371_source_transport_preserves_fragment_and_delay_shapes() {
  - id: TC-371/ZC-31
    path: spec/test-cases/TC-371-zone-certificate-checker.md
    role: examined
    excerpt: ZC-10, ZC-11 and ZC-12 each with `admitted` removed
  - id: FR-418-AC-16
    path: spec/objects/interfaces/FR-418-zone-certificate.md
    role: context_only
    excerpt: "Over a Zeno-only variant of `Rpc` (time invariant `x <= 0 ms` at every
      state, so no positive delay is admissible and every infinite path is
      zero-delay), the certificates of `possible` of `phase = Replied` and of
      `always possible` of `phase = Idle` carry no `admitted` member, are valid
      `ZoneCertificate`s, and are accepted, the items settling `proved` with
      basis `zone-certified`; either certificate given an `admitted` member
      settles `inconclusive`, `certificate-rejected` with rule `malformed` at
      `{member: admitted}`; ZC-01's `NoLateReply` certificate with `admitted`
      removed is a valid "
```

## Method selection and limits

Rust review is folded into this code-review artifact. Dependency analysis was not selected: no relationships edges changed. EARS analysis was not selected: the main requirement statement is unchanged; scoped FR-418 validation reported zero grammar findings. Risk-complexity was not selected: no new algorithm, performance promise, concurrency or external contract was introduced. Criterion-strength, security, architecture and application lenses were not applicable to this bounded schema completion. Object review uses installed analysis: base because the installed SpecReview analysis enum has no object value. Optional gap semantic review was not run. Full repository gap assurance remains unclaimed; the assigned four-file PR scope and external checker ownership control this review.

Commands run: git status --short, git rev-parse HEAD, git merge-base origin/main HEAD, git diff origin/main...HEAD, gh pr view 799 --json headRefOid,baseRefName,title,body (head matches); quoin write . --types SpecReview (0); quoin --version (0.29.1); quoin module list (0); quire matrix --scope . --format json (0); quoin matrix --repo . --json (0); quire coverage --scope . --json (0); quoin advise --json (0); quire validate --scope . spec/objects/interfaces/FR-418-zone-certificate.md --summary (0); quire validate --scope . <each explicit scratchpad artifact path> (all 0). No Cargo job or ad-hoc test was run. git diff --check returns 2 solely for the trailing blank line already disclosed. Linear posting and structured YAML/id/scope read-back checks passed for all eight methods; has-review label already existed and was ensured on QSL-2240. No GitHub comment was posted, in accordance with the dispatch.

## Disposition round 1 verification

Reviewed fix head 3d63fc8dfe64cc85f7114e16bf4bd3e56b97927b independently; local and GitHub heads match and worktree is clean. The fix changes only the new source-vector identifier and test diagnostic to ZC-32, adds its matching TC-371 row, and removes the trailing blank line. Existing ZC-31 admitted-removal row and test remain unchanged. The ZC-32 row accurately names the existing positive and negative shape assertions and explicitly reserves semantic acceptance to independent reader recomputation. No schema, certificate payload, LU row, transition identity, fragment data, canonical encoder or executable test assertion changed. No new finding in the affected contract scope.

Current result: PASS in this method’s examined scope; no unresolved finding. git diff --check origin/main...HEAD returned 0; quire validate --scope . spec/objects/interfaces/FR-418-zone-certificate.md spec/test-cases/TC-371-zone-certificate-checker.md --summary returned 0 (2/2 grammar-clean, existing relationship warnings only). Each latest canonical scratchpad artifact was revalidated. No duplicate Cargo/build/test run is needed for an identifier/diagnostic/documentation-only fix. Prior meaningful 1/1 shape and 2/2 finite/canonical passes remain applicable; no new runtime execution is claimed.
