---
id: SR-20261120100921
title: "spec review of completion spec batch 5, FSpec PR773 named type arguments and body-location names"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; FR-322 Named type arguments, body_locations name, FR-322-AC-491..494, TC-2090, TC-2098, proposals/checked-package-v2/schema.json, type-argument-alias-vectors.json; PR773 2ff8254148c710be11542a836e524154458a4407"
review_set: subset
---

## Summary

Ticket: QSL-2627 (batch with QSL-2630, QSL-2463, QSL-2701). One spec-review pass over `git diff origin/main...HEAD`. The `alias` member is a printing label in the text: every check and evaluation reads `declaration`, and the reader checks only shape, owner join, key spelling, domain-alias resolution and one key per type. One finding is high: rule 1's locus sentence disagrees with vector TA-03. One is medium: the alias rules' place in FR-322's reader order is not stated exactly.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Rule 1 (Shape) puts an extra member's refusal at the offending member's pointer (`.../alias/name`), but TA-03 and FR-322-AC-492 expect `.../member/alias`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3097 |
| FND-002 | medium | The order of the five alias rules is unclear. Rule 1 is a schema-stage refusal but sits in the list for the operation stage. Rule 5 runs "after every member passes steps 1 to 4", and the text never places it against other applications' constraint refusals. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3093 |
| FND-003 | low | FR-322-AC-493's "replacing each source key with another admitted source key" can give two members one key with different declarations, which rule 5 refuses. | spec/objects/interfaces/FR-322-checked-package-artifact.md:6178 |

## Scope and evidence

- Ruling (a): confirmed. In "Named type arguments", every check and every evaluation reads `declaration` alone. A source or dependency key that is well formed but wrong passes rules 2, 3 and 5 and changes only the printed text. The separate node per alias fits "Structural node identity": the reader compares no derived path with its body, a string segment gives a distinct structural key, and canonical order stays deterministic. The alias holds no node id, so the operand rows are unchanged.
- Ruling (b): confirmed. A population form carries no `alias`.
- Ruling (e): confirmed. The `body_locations` `name` rewrite to `ix://I/<path>` agrees with FSL FR-531's new paragraph and FR-531-AC-40.
- Vectors: the JSON and schema.json both parse. Against `fixtures/positive-operation-identities.json`, node 5 is a `type_argument` lookup, node 7 is a `type_argument` sum.decimal, node 1 is a `field` member, the source identity is `example` and the `model_root` is `acme/orders`. TA-01, TA-02 and TA-04 to TA-09 follow from the rules they name. TA-03 is FND-001.
- Schema: `alias` is added only to the `type_argument` branch, `additionalProperties: false`, `Nonempty` members; closed.
- FSpec PR772 (merged) does not conflict (`git merge-tree` clean). The branch does not contain it, so the PR needs a rebase.
- No collisions for FR-322-AC-491..494 and TC-2098 on origin/main or in open PRs.
- `quire validate` on the changed files exits 0. Its warnings are pre-existing EARS warnings on line 110.

## Verdict

Blocking on FND-001: either fix the rule text or fix the vector. Replacement for rule 1:

"1. **Shape.** An `alias` that is not an object, that lacks `package` or `node`, or that carries any other member fails the V2 schema and refuses `malformed_wire` at `/semantic_graph/nodes/{n}/body/operation/member/alias`. A `package` or `node` that is not a non-empty string refuses `malformed_wire` at that member's pointer below it. An `alias` on any member kind other than `type_argument` fails the schema and refuses `malformed_wire` at that member's `.../member/alias`. These are schema-stage refusals and precede every graph and reader stage."

Replacement for the lead-in and rule 5 (FND-002):

"The reader checks `alias` in two places. Rule 1 is part of the V2 schema check. Rules 2 to 4 run in the operation stage, for each application in graph order, after its member's `declaration` resolves and before that application's constraints. Rule 5 runs once, after the operation stage has passed every application and before the next step of the reader order. So a package with a rule 5 defect and a constraint defect reports the constraint defect."

Replacement for FR-322-AC-493's clause (FND-003): "or replacing each source key with an admitted source key that no other member carries".
