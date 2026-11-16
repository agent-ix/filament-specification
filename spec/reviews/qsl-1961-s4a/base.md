---
id: SR-2026111616144800000001
title: "Base review of QSL-1961 S4a (CF-PR-04 removal, FR-170 symbolic capacity)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification PR #664 at 4e97f7785eec72904dc0f17b28ca3553a5a9a64e; diff origin/main...HEAD: proposals/consumer-facades/consumer-facts.json, FR-661 (Behavior, AC-1, AC-2), FR-170 (Outputs, Behavior, AC-3), FR-217 (Dependencies, Status)"
review_set: base
review_date: "2026-11-16"
relationships:
  - { target: ix://agent-ix/filament-specification/FR-661, type: references }
  - { target: ix://agent-ix/filament-specification/FR-170, type: references }
  - { target: ix://agent-ix/filament-specification/FR-217, type: references }
---

## Summary

Ticket: QSL-1961 (S4a). Base checklist over the four changed files, with
cross-artifact consistency checks against FR-054, FR-465, FR-326, FR-331,
FR-272, FR-660, TC-201, TC-1645 and, for consumer-fact ids, the producer
repository agent-ix/filament-spec-language at origin/main (FR-519, FR-761,
docs/consumer-facades.json, FR-575). Analysis lenses were not run: the
questions in scope are consistency questions the base checklist covers, and
this is a four-file spec-only diff. The pre-existing `quire validate` failure
on `object:` FRs (missing frontmatter id, FR-217 among them) is not counted.

Role internality: clean. No FSpec reference to CF-PR-04 or to internality
as a producer-read fact remains; FR-661's FR-217 edge and FR-661-AC-2's
"role internality" item are removed; FR-217 Status ("a consumer reads it
from the binding authority's input and never from the ... checked package")
agrees with FR-050-AC-10, FR-211, FR-465-AC-8 and TC-947 RL17. FR-464:772
still cites CF-PR-02, whose id and meaning are unchanged. FR-660: no release
exists (package version 0.0.0, no tags), so no release promise is broken.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Renumbering CF-PR-05..07 to CF-PR-04..06 reassigns three ids to other facts, which FR-661 forbids ("never reused", "An id is never reassigned to another fact"). The producer's committed bindings (FSL docs/consumer-facades.json:21-24) map CF-PR-04 to role_internality, 05 to await_anchor, 06 to choice_visibility, 07 to attempts_maximum; FR-661-AC-3's check verifies only id existence and accessor resolution, so after the pin move 04/05/06 pass bound to the wrong facts and only 07 fails, and FSL FR-519:256 cites 05..07 with their old meanings (FR-661-AC-4). Retiring CF-PR-04 needs AC-1's "consecutive ordinals" relaxed, because consecutive plus never-reused cannot both hold once a fact is removed. | proposals/consumer-facades/consumer-facts.json:50-62, spec/functional/ecosystem/FR-661-hold-a-consumer-fact-inventory.md:66-67,74,108 | wrong-requirement |
| FND-002 | medium | FR-054 still says the queue's capacity "is a literal bound or a symbolic name with a discharged bound", which contradicts the new FR-170 Behavior: a symbolic capacity whose bound is not discharged is carried, and stepping it settles `unsupported`. | spec/functional/protocol/FR-054-bind-channel-premises.md:61-63, spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:39-43 | wrong-requirement |
| FND-003 | medium | FR-170's typed `unsupported` names no cause or diagnostic. FR-331 requires exactly one of `cause` or `diagnostic` on `unsupported`, closes the unsupported cause set (FR-331:330), and records each requirement's added cause (FR-439, FR-427 at FR-331:722-725). FR-170 adds none, so FSpec has no code for the case, two implementers can pick different shapes, and TC-201's "exact typed disposition" has nothing to compare against. FSL FR-575 uses `unsupported_projection` with slug `symbolic-queue-capacity`, for projection and realizability only. | spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:41-43,86, spec/objects/interfaces/FR-331-backend-provider-envelope.md:330,696,722-725 | missing-requirement |
| FND-004 | medium | No FSpec requirement says how a symbolic capacity's bound is "discharged". There is no request member or binding input for it: FR-465 says only that "execution discharges" it, and FR-326 lists it as a bound with analysis bounds as request members. So the discharged-symbolic branch of FR-170-AC-3 cannot be set up by TC-201. FR-171-AC-3 gives a missing symbolic bound `requires-bound`, not `unsupported`. | spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:39-41,86, spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:246-248, spec/objects/interfaces/FR-326-checked-protocol-artifact.md:52 | wrong-requirement |
| FND-005 | low | "naming the queue" names something that has no name: FR-465 says a queue has no declaration and is identified by its one channel. The result should name the channel's qualified name (FR-429 rendering) and the capacity's symbolic `name`. | spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:42,86, spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:242 | wrong-requirement |

## Verdict

Changes requested. Clean: the role-internality removal is consistent with
FR-217 Status, FR-050, FR-211 and FR-465 in FSpec. FR-170's new rule
(carry, settle typed unsupported, assume no bound) agrees in intent with
FR-465 ("carries the name and no value"), FR-326 and FSL FR-575.

FND-001 conflicts with the planner's ruling to remove CF-PR-04 and
renumber, so it goes to the planner to decide and is not overridden here.
The recommended smallest fix is to retire the id: restore CF-PR-05..07,
leave a gap at 04, change FR-661:66-67 to "increases within the consumer;
the id of a removed fact is retired and never reused", and change
FR-661-AC-1's "consecutive ordinals from `01`" to "strictly increasing
ordinals from `01`". Under that fix the producer only deletes its CF-PR-04
binding, which the ticket already plans, and its check fails loudly
("binding names an id the inventory does not hold") until it does. If the
ruling stands, FR-661:67 and :74 must change in the same PR so the spec
does not forbid what it just did, and FSL must remap
docs/consumer-facades.json and FR-519:256, which the ticket's FSL item (a)
does not list.

FND-003 options: add an FR-331 unsupported cause such as
`symbolic-capacity{channel, name}` (FR-170 adds it), or have FR-170 name the
FR-272 diagnostic FSL already emits (`unsupported_projection`). FR-272's
`unbounded-extent` is a negotiation cause and does not fit a step-time
settlement.
