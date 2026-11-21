---
id: SR-20261121000798001
title: "code-review review of composition empty-region precedence"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-specification; QSL-2739; FR-491-AC-8 and FR-322 source-map context"
review_set: subset
---

## Summary

Ticket: QSL-2739; PR #798. Frozen head ad16d93bbc58a58d5cdf1b537f875c3b80fcc1e8; base 73e81dcea648ef4d844768ac5c858e512ac62a4d. One acceptance-row specification amendment, reviewed read-only. No Cargo, runtime qualification, source/schema edit, semantic-reader acceptance or AC-10 completion is claimed. quoin 0.36.8; quire 0.52.0 (engine 0.62.2); spec-artifacts-process v0.36.0. The advertised model identity is Codex; the exact backend model identifier is not exposed. No AssuranceProfile frontmatter exists in the local spec tree, so direct review applies.

Selected methods: mandatory code-review and spec-review/base, failure-domain and EARS-conformance. Rust and implementation gap analysis are inapplicable to the one Markdown AC row; integrity/object/dependency/evidence/risk/scope-boundary lenses are not separately selected because artifact structure, domain objects, relationships, verification method and ownership boundaries are unchanged. No criterion-strength/Jev assessment is claimed. Full-repository validation/gates were not run.

## Verdict

PASS — no findings. Specification readiness only; no semantic-reader or implementation acceptance.

## Method and evidence

No runtime, schema, fixture or source code changes. The one-row diff preserves the composition-specific controls and distinguishes generic validation. No duplication, vendoring, invented wire member, warning suppression or extra tracking mechanism is introduced. The corrected acceptance oracle matches the existing reader instead of changing it to satisfy a test.

Inspected FR-491 member-coordinate prose and its explicit generic-source-map precedence, plus FR-322 source-map contract. Independently inspected pinned Contract IR 1cbd86a: crates/filament-contract-model/src/checked_package/v2/source_map.rs declares InvalidSourceMap at line 38 and, at lines 119-134, tests region.start >= region.end and refuses at root source_map / entry / regions / region, with no composition-specific cause. Its missing-key and region-collection checks remain untouched. This corroborates both equality and reversed intervals. Scoped FR-491 ISO validation exits 0 without diagnostics; git diff --check is clean. Computed strict matrix exits 1 with FR-491-AC-8 untagged (zero binders); this records pending implementation rather than runtime success.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

Part numbers split verbatim excerpts of the same examined criterion; they do not create specification IDs.

```yaml
{
  "clean": true,
  "scope": [
    {
      "id": "FR-491-AC-8",
      "part": 1,
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "Independently mutate AC-7's otherwise valid package, retaining V2-schema and generic source-map validity: remove a member's occurrence and matching entry together; change both to an ordinal equal to the member count; change a member region to another locked source, outside the declaration interval, equal to the declaration interval, overlapping an earlier member's region, or two nonoverlapping regions; and replace the member's role in both its occurrence and entry with `expression`. The missing pair refuses `invalid_package`/`invalid-value` at `/source_map`; each other mutation refuses the sam"
    },
    {
      "id": "FR-491-AC-8",
      "part": 2,
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "examined",
      "excerpt": "e code and cause at that entry's `/source_map/{i}`. Separately, a region whose `start >= end` fails the preceding generic source-map check as `invalid_source_map` at `/source_map/{i}/regions/{j}`, before any composition member-coordinate check. No member inherits the declaration's region, and no source text is recompiled."
    },
    {
      "id": "FR-491",
      "part": 1,
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "context_only",
      "excerpt": "After FR-322's generic source-map checks, the reader SHALL examine the\ncomposition node's entries in `source_map` order and refuse the first\ninvalid entry as `invalid_package`/`invalid-value` at `/source_map/{i}`:\na role or ordinal outside the declaration/member pairs above; a\n`regions` array with other than one region; or a member region whose\nsource differs from the declaration's, whose interval is empty or not\nstrictly within that declaration region, or which overlaps an earlier\nmember entry's region. When all entries pass, the reader SHALL refuse a\nmissing required declaration or member oc"
    },
    {
      "id": "FR-491",
      "part": 2,
      "path": "spec/functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md",
      "role": "context_only",
      "excerpt": "currence as\n`invalid_package`/`invalid-value` at `/source_map`. Generic duplicate-key,\nunknown-node and total-occurrence/source-map checks retain their\nexisting refusals and precedence. The member-coordinate check SHALL\nrun after the node's identity, dependency, body-term and canonical-order\nchecks and before static checks 2 to 5."
    },
    {
      "id": "FR-322",
      "path": "spec/objects/interfaces/FR-322-checked-package-artifact.md",
      "role": "context_only",
      "excerpt": "- `source_map` \u2014 the total semantic-occurrence to original source-region\n  mapping. An occurrence key is (`node_id`, `role`, `ordinal`), where role is\n  `declaration`, `type`, `expression`, `anchor`, `claim` or `generated`; a\n  source region is a half-open UTF-8 byte interval `[start,end)` under one exact\n  source-document identity. The role `generated` is the role\n  of every occurrence of a *compiler-derived node*, a node the compiler adds\n  that no source text declares and that derives from exactly one declaration\n  node, its *owner*, of the per-obligation occurrences a refinement owner\n  ca"
    }
  ],
  "findings": [],
  "bindings": []
}
```
