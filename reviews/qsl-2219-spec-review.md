---
id: SR-20261119181014
title: "spec review of QSL-2219 relationship endpoint ends and attempt state-clause cycles (FSL #3069, FSpec #748)"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-spec-language; FR-218 Relationships bullet, FR-880 Relationships and cycle bullets, FR-880-AC-12, FR-880-AC-13, TC-3069 at bfc728b1c; agent-ix/filament-specification; FR-322 group-member paragraph, FR-464 paragraph, FR-464-AC-25, TC-946 K24/K25 at 913a57b99"
review_set: subset
---

## Summary

Ticket: QSL-2219. FR-218's endpoint-end typing agrees with FR-085/FR-086: an admitted endpoint is a Port, and a Port's interface type classifies as an Interface, which is an object-type export. A hand trace of K25 confirms the refusal falls at `Pre` under FR-322's cycle rule whether or not `Pre` is the component's first member, so the AC can fail. FR-322-AC-29 already restricts the refusal to model declaration nodes. Only medium and low defects were found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FSpec FR-341's identity bullet still says a state_clause node_id is always the derived structural path, and FR-322:4339 says structurally equal clauses share one node; neither is scoped to "outside a recursion group" after the FR-322 amendment | spec/objects/interfaces/FR-341-checked-package-state-clause-body.md:233 |
| FND-002 | medium | TC-946 K24 is "K16 with joinPred replaced", but K16's Holder references joinPred; the node set and edges of K24 (Holder kept or dropped) are not stated, unlike K16 and FR-322-AC-285's member list | spec/test-cases/TC-946-checked-package-protocol-node-family.md:126 |
| FND-003 | low | "written in ordinal order" is not a total order: ordinals are unique only per (package, path), and the group members have distinct paths | spec/functional/FR-880-write-every-checked-protocol-construct-at-s4.md:69 |

## Verdict

The QSL-2219 part has no high finding. It ships in the same two PRs as QSL-2512, which are blocked by that ticket's findings. quire validate exits 0 in both worktrees. No id collisions with open PRs.
