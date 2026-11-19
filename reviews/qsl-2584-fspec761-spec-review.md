---
id: SR-2026111922300000000002
title: "spec-review of QSL-2584 V2 schema place-reference vocabulary and FR-322-AC-17 one-schema wording"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #761 at 82c88adf; proposals/checked-package-v2/schema.json (OperationIdentity, SemanticTerm application operator, ScalarTypeNode and ExpressionNode semantic_form enums), spec/objects/interfaces/FR-322-checked-package-artifact.md (catalog paragraph lines 482-495, FR-322-AC-17, three wording edits), spec/objects/interfaces/FR-829-carry-a-clock-place-reference-as-a-place-id-term.md (Catalog changes, FR-829-AC-5), spec/test-cases/TC-2084-clock-place-reference-term.md (step 5); context: QVC PR #38 at 6de40bfd catalog diff, tests/checked_package_v2.rs tc_233 and the order-sensitive OperationIdentity tests, filament-contract-ir operation_catalog.rs, FSpec #518 (040c28f30)"
review_set: subset
---

## Summary

Spec review of FSpec PR #761. Ticket: QSL-2584 (ruling comment 8bc9c345). The PR adds `filament.op.clock_reference.ref/eq/ne` to `OperationIdentity`, `place_ref` to the application `operator` enum, `place_reference` to the `expression` `semantic_form` enum and `clock_reference` to the `scalar_type` `semantic_form` enum, and rewrites FR-322-AC-17, FR-829 and TC-2084 to name the one V2 schema. `schema.json` parses, `quire validate` passes on the three changed Markdown files (only pre-existing EARS warnings at FR-322 line 108), and the PR adds no new artifact ids.

## Examined scope

- One schema. `node-identity-preimage.schema.json` was deleted in 040c28f30 (FSpec #518, 138 lines removed). In FSpec only `proposals/checked-package-v2/schema.json` holds `$defs.OperationIdentity`; the other three `*.schema.json` files in that directory have no operation vocabulary. filament-contract-ir main has four schemas under `schemas/`, none with `OperationIdentity` or `filament.op.*`; its `operation_catalog.rs` takes `CHECKED_OPERATION_CATALOG_V1` from the QVC crate (git rev pin) and holds no copy. QVC main's three schema files have no operation vocabulary. The claim holds. (examined)
- Schema entries against the catalog. QVC #38 adds exactly the identities `filament.op.clock_reference.ref/eq/ne` and the operator class `place_ref` (ref), with `eq`/`ne` on the existing `binary` class. The schema adds the same three identities and `place_ref`. `place_reference` and `clock_reference` go into the enums FR-829 names (`ExpressionNode` and `ScalarTypeNode`), and no per-form conditional rule exists for comparable forms (`presence_read`, `uuid`), so no other schema location needs an entry. Literal `value_kind` enums correctly do not get `clock_reference`, since a place reference has no literal. This agrees with FR-829 and the QSL-2178 ruling: `ref(p)` is a place-id term over a `place_owner` (`reference` only) owner. (examined)
- tc_233 after the pin bump. tc_233 compares sets: catalog identities with `OperationIdentity`, law roles, mode kinds and values, member kinds, and catalog operator classes with `SemanticTerm.oneOf[2].operator` (the `application` branch). Once the pin reaches QVC #38, both sides gain the same three identities and `place_ref`, and nothing else changes. The catalog-internal checks also pass: `clock_reference` is in `families`, `any_value` and `any_term` (`any_term` = families minus `object`); `place_owner` lists only families; `rest: any_value` and `result: clock_reference` are known; the new constraint kinds are listed. The order-sensitive tests (tc_1326 after `quantity.pow`, the text/string conversion anchors) are not affected, because the insertion comes after `reference.upcast`. (examined)
- Remaining "both V2 schemas" wording. There are 90 mentions in 23 files (TCs, README:240, FR-322/375/464/469/478/487/341/370/440 ACs, test comments, tc_233's function name). None says that a second schema exists with different content, and none contradicts AC-17. (examined)
- FR-322-AC-17's new clause "no other schema of `proposals/` enumerates `OperationIdentity`". tc_233 does not check it (FND-001). (examined)

## Verdict

Mergeable after QVC #38 merges and the pin bump, once FND-001 is fixed in this PR. Either add a tc_233 assertion that no other `proposals/**/*.json` schema has `$defs.OperationIdentity`, or drop the clause. The schema entries are correct and complete for FR-829. FND-002 and FND-003 are low and do not block. FND-002 justifies one follow-up wording-sweep ticket.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-17 now requires "no other schema of `proposals/` enumerates `OperationIdentity`" (Test TC-233), but tc_233 reads only `SCHEMA` and TC-233 has no step for it. The clause can regress silently. Add the check to tc_233 (and a TC-233 step) or drop the clause | spec/objects/interfaces/FR-322-checked-package-artifact.md:5643; tests/checked_package_v2.rs:6331 |
| FND-002 | low | The new definition ("Where this and the requirements that specify it say 'the V2 schemas', 'both V2 schemas' or 'either schema' … they name `schema.json`") covers FR-322 and the requirements that specify it, but not the TCs (TC-233:180, TC-1921:14/21, TC-310, TC-395, TC-951, TC-367, TC-946, TC-303, TC-1326), the README (line 240) or tc_233's name `…_bound_to_both_schemas`. That is about 88 stale mentions. They are harmless (none contradicts AC-17) but misleading; one follow-up sweep ticket is needed | spec/objects/interfaces/FR-322-checked-package-artifact.md:492-495 |
| FND-003 | low | The new paragraph's obligation, "an `expression` or `scalar_type` `semantic_form` that a catalog entry's node or result form introduces is in that schema's form enumeration", is in no AC and cannot be checked mechanically, because the catalog does not record forms. FR-829-AC-5 covers it only for this change. Either state it as an authoring rule or say that each introducing FR's AC carries it | spec/objects/interfaces/FR-322-checked-package-artifact.md:486-488 |
