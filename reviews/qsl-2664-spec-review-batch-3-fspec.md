---
id: SR-2026112007511802
title: "spec-review of completion spec batch 3 (filament-specification PR 770)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR 770 head bd824b33ead5571a376987a691a1953fbd1252ad; spec/functional/foundation/FR-352; spec/objects/interfaces/{FR-323,interface_005}; spec/test-cases/{TC-2081,TC-278}"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-352
    type: references
  - target: ix://agent-ix/filament-specification/FR-323
    type: references
---

## Summary

Ticket: QSL-2664 (with QSL-2559, QSL-2576, QSL-2235, QSL-2675). One pass of quoin:spec-review over `git diff origin/main...HEAD`, reviewer-only, no builds.

Checked clean: FR-352's closed set of six locus kinds and FR-352-AC-24 match FSL FR-267 and FR-267-AC-28 member for member. FR-323's file-form `equality` rules and FR-323-AC-49 match FSL FR-1039 and FR-850 (`invalid_package`/`missing-file` at `/request/package/equality`, exit 20; `invalid-request` naming `package.equality` for the directory form or a non-string). FR-352-AC-24, FR-323-AC-49 and RE-43..46 collide with nothing on origin/main or in open PRs #769 and #640. `quire validate` exits 0 on every changed file.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | interface_005's `PackageSelection` references `SourcePackageRef`, which no FSpec artifact defines, and FR-323's selection text never mentions `source_package`. | spec/objects/interfaces/interface_005-native-runtime.md:44-45 |
| FND-002 | low | RE-43 and RE-46 expect "Admitted; runs" without a result, and the four non-replay selection vectors sit in TC-278, the replay-envelope vector table. | spec/test-cases/TC-278-replay-envelope-members.md:111-114 |

## Recommendations

- FND-001: add to interface_005 `SourcePackageRef: { type: object, required: [identity, version], properties: { identity: { type: string, minLength: 1 }, version: { type: string, minLength: 1 } }, additionalProperties: false }`. Add to FR-323 after the selection forms: "Either form MAY carry `source_package`, the closed `{identity, version}` object of FSL FR-1039."
- FND-002: for RE-43, expect "Admitted with that file's equality rows; the `synthesis` member `{"declaration": "Pick"}` gives `candidate` 7, exit 0". For RE-46, name the package (for example FSL FR-1039-AC-1's `half`) and expect "`-4`, exit 0".

## Verdict

Mergeable: no high findings. The two findings are a medium and a low, fixable in the same PR.
