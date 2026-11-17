---
id: SR-20261117154552
title: "Base review of QSL-1887 FR-785 symbolic channel capacity binding"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification PR #682 at 6ff2c3c7f63a32662db7235848deb3650970de4c; diff origin/main...HEAD: FR-785 (new), TC-1911 (new), FR-170, FR-054, FR-425, FR-433, FR-465, FR-326, FR-735, FR-331"
review_set: base
review_date: "2026-11-17"
relationships:
  - { target: ix://agent-ix/filament-specification/FR-785, type: references }
  - { target: ix://agent-ix/filament-specification/TC-1911, type: references }
  - { target: ix://agent-ix/filament-specification/FR-425, type: references }
  - { target: ix://agent-ix/filament-specification/FR-170, type: references }
  - { target: ix://agent-ix/filament-specification/FR-331, type: references }
---

## Summary

Ticket: QSL-1887. Base checklist over the ten changed files, with failure-domain
style consistency checks against FR-181 (requires-bound, line 285-287),
FR-425 root classification and role binding refusals (FR-425:100-125,
AC-4, AC-13), FR-171-AC-3, FR-175 (Derived-artifact identity, AC-3, AC-6),
FR-735-AC-4, FR-433 Wire and Replay, FR-429 `full-queue`, FR-272
`invalid_runtime_input` causes, FR-331 request identity (line 923), in-process
providers (line 336-363) and the `Temporal.subject` schema in
`proposals/backend-provider-v1/schema.json`. No remaining text says a symbolic
capacity is `unsupported`, and no "discharged bound" phrase without a
mechanism remains in FR-054, FR-170, FR-465 or FR-326. The pre-existing
`quire validate` failure on `object:` FRs (FR-326, FR-331) is not counted.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-785 claims the capacity binding enters the request identity, but FR-331 request identity is built from wire members ("each temporal item's subject"), the wire `Temporal.subject` is closed to `initial_states`, `universes`, `proof_bounds`, and an in-process request has no request identity (FR-331:361-363). So AC-4's "different ... request identities" is false for a request frame and vacuous in-process. Roles are carried the same way, and FR-425 claims only obligation identity for them. Drop "request identity" from FR-785 Identity, AC-4 and TC-1911 step 4, keep obligation identity, and name where the `proved` label lives (the obligation identity's subject), since no FR-331 result entry carries a subject. | spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:104-107,113-116,149, spec/test-cases/TC-1911-symbolic-channel-capacity-binding.md:47-48, spec/objects/interfaces/FR-331-backend-provider-envelope.md:361-363,923 | wrong-requirement |
| FND-002 | medium | The unbound-name rule is written without scope: FR-785 Description, Outputs and Stepping ("without one the subject returns `requires-bound`") and FR-170 ("an unbound one returns `requires-bound` before any expansion") contradict FR-785 Realizability and FR-735, where an unbound realizability item settles `inconclusive`/`premise-not-stated`. FR-785:78-80 also makes the unbound name an FR-425 root, and realizability reads "the subject's binding", so it does not say whether FR-425's root classification runs for a realizability item and returns `requires-bound` first. Scope `requires-bound` to the items FR-425 classifies (simulation, model check, temporal, refinement, derived items), and state the realizability exception in the Admission section. | spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:32-35,46-48,78-83,97,137-139, spec/functional/protocol/FR-170-evaluate-channel-queue-semantics.md:40-43, spec/functional/protocol/FR-425-define-protocol-subjects-and-state-keys.md:119-125 | wrong-requirement |
| FND-003 | medium | FR-785 says the capacity premise (name and value) enters the realizability key, and FR-175 refuses an absent key member with `invalid_runtime_input`/`missing-member` (FR-175-AC-6). With no binding the value is absent, so FR-175 gives a refusal where FR-785 and FR-735-AC-4 give `inconclusive`/`premise-not-stated`. State that an unbound capacity is a premise recorded as not stated, so the key member is present, or that `premise-not-stated` settles before the key is built. | spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:134-139, spec/functional/protocol/FR-175-analyze-protocol-realizability.md:58-75,89 | wrong-requirement |
| FND-004 | medium | FR-785-AC-3's "before root classification" cannot fail as written: no case combines an admission defect with an unbound name. A build that classifies roots first passes AC-3. Add `{"Other": "1"}` with `Bound` unbound, which must refuse `unknown-member` and not return `requires-bound`, to AC-3 and TC-1911 step 3. | spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:68-69,148, spec/test-cases/TC-1911-symbolic-channel-capacity-binding.md:26-27,44-46 | wrong-requirement |
| FND-005 | low | FR-785-AC-1 "a second channel ... takes the same value" and TC-1911 step 1's check "Both channels hold at most the bound value" still pass when the second channel steps at a smaller value. Require that at `"2"` the second channel queues two messages, or that its state keys equal those of a `capacity 2` literal. | spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:146, spec/test-cases/TC-1911-symbolic-channel-capacity-binding.md:22-23,39-40 | wrong-requirement |
| FND-006 | low | FR-785-AC-5 and TC-1911 step 5 expect "a refusal before replay" for a counterexample without `capacities` and one with `capacities` over a clause with no capacity name, but name no code. Name them, for example `invalid_runtime_input`/`missing-member` and `unknown-member` at `subject.capacities`, so the expected result is exact. | spec/functional/protocol/FR-785-bind-symbolic-channel-capacities.md:150, spec/test-cases/TC-1911-symbolic-channel-capacity-binding.md:49-51 | wrong-requirement |

## Verdict

Changes requested (four medium, two low). No high findings and no
hash/digest/pin antipattern.

Clean, examined: FR-054, FR-465 and FR-326 now name the discharge mechanism.
FR-425 Inputs, Protocol subject and root classification name the binding.
FR-433 `subject.capacities` and its replay rules hold up: at `"2"` the
`"1"` deadlock's last state enables `send Q`, so FR-433 step 5 gives
`replay-parity`, because the state key does not carry capacity. Soundness is
labelled correctly: results hold at the bound values only, a refutation stands
on its own, there is no extension by monotonicity, and the binding enters
obligation identity and witness identity. The admission refusals use
FR-272-listed `invalid_runtime_input` causes and run before expansion.
Role bindings refuse with `wrong-role-mapping`, but a missing role is a refusal
while a missing capacity is `requires-bound`, so generic causes fit here. The
FR-331 `premise-not-stated` and `cut-count` edits are consistent. TC-1911 has
a step for every AC. The repo keeps no separate trace table.

Departure from the literal ruling (realizability with no binding gives
`inconclusive`/`premise-not-stated`): accept it. TL-11's exit forbids silent
finite truncation, and an explicit `inconclusive` is not silent.
FR-170-AC-3 is about queue transitions. FR-175-AC-3 and FR-735-AC-4 already
require `inconclusive` for a missing channel premise. This only holds once
FND-002 scopes the `requires-bound` statements and FND-003 settles the key
member.

FR-331 wire unchanged: consistent with how roles are carried. The only text
that implies a wire member is FR-785's request-identity claim (FND-001).
FR-433's `subject` is FR-433's own counterexample wire, not FR-331's schema.
