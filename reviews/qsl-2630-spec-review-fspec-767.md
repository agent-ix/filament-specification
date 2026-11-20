---
id: SR-20261120051602
title: "Base review of completion spec batch 2, FSpec half (PR #767)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; PR #767 at ecc084e980, diff origin/main...HEAD: FR-149, FR-814, FR-817, QSL-2638 wording in FR-322/341/370/375/440/464/469/478/487, TC-233/303/310/367/395/940/946/951/965/1326/1921/1973/1976/2097, README, protocol-node-vectors.json"
review_set: base
---

## Summary

Ticket: QSL-2630 (batch: QSL-2636, QSL-2598, QSL-2638). FR-149 adds the tuple constructor position (AC-31), consistent with FSL FR-065-AC-10/11 and Contract IR FR-347-AC-15. FR-814/FR-817 add per-binding-site name-map entries for bound names (FR-814-AC-11, FR-817-AC-19). QSL-2638 wording is applied across specs, TCs, README and one vector note. The new FR-814 sentence that every other prefixed name names one node is false for field and enum member names.

## Verdict

FAIL on FND-001. AC ids FR-149-AC-31, FR-814-AC-11 and FR-817-AC-19 are free on main and in #765, #766, #695 and #640. No file is shared with another open PR. quire validate exits 0 on every changed spec file (README fails on main too, known). The sampled QSL-2638 edits change no meaning; four introduce grammar slips (FND-003 to FND-006). The tc_233 rename and three doc comments in tests/checked_package_v2.rs are left for a code PR, as the PR body says; that file is also edited by #765.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-814 says every prefixed name outside the bound classes names one FSL node and has one entry, but `fld_` and `e_` names come from the member name alone and FSpec FR-154 keys members per owner, so two object types with a field `total` share `fld_total` | spec/functional/output-mapping/FR-814-write-canonical-tla-and-lean-text.md:128-129 |
| FND-002 | medium | FR-814 says a trace or error is read through the entry whose `binding` is the definition it names; an `x_`/`o_`/`w_` binding is a quantifier, let, lambda or binder construct that no TLC step or error names, and one definition can hold two sites of one name | spec/functional/output-mapping/FR-814-write-canonical-tla-and-lean-text.md:131-137 |
| FND-003 | low | FR-370-AC-17 "both the catalog and the V2 schema enumerates it" is ungrammatical after the edit | spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:474 |
| FND-004 | low | TC-1921 "the operation catalog and the V2 schema carries" is ungrammatical after the edit | spec/test-cases/TC-1921-population-only-in-the-catalog-and-the-reader.md:14 |
| FND-005 | low | FR-322 "The V2 schema checks only the closed root shape; they cannot join" keeps a plural pronoun | spec/objects/interfaces/FR-322-checked-package-artifact.md:833-834 |
| FND-006 | low | FR-322 "After the V2 schema check" was already singular; the edit made it "checks" | spec/objects/interfaces/FR-322-checked-package-artifact.md:1822 |
