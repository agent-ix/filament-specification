---
id: SR-2026112106460400000006
title: "Base specification review — FR-331 vacuous exhaustive carrier and FR-412 policy hints"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/objects/interfaces/FR-331-backend-provider-envelope.md (exhaustive paragraph, FR-331-AC-60); spec/objects/interfaces/FR-412-probability-certificate.md (policy member row, Policy hints rule, Absence, FR-412-AC-17..19); spec/test-cases/TC-2110-check-fair-minimum-value-bounds-and-measure-certificates.md (steps 9-11); spec/test-cases/TC-431-backend-provider-wire-vectors.md (BP-65)"
review_set: base
---

## Summary

Ticket: QSL-2158. PR: filament-specification#787 at 0f3375a68372bc7db8bfe6b9933215d969a2f023, base f73bd9f3fe5091ab8b459126808d7149cdb02b08. This is one BASE checklist review, as the architecture brief requires; no sub-analyses ran. No AssuranceProfile applies to the reviewed scope.

`quire validate --scope .` passed on the four changed spec files (quire 0.52.0, engine 0.62.2). It reported only three warnings, all pre-existing and outside the diff: FR-331 unresolved type `ProtocolClaimKind`, and the `verifies` edge type twice.

FR-331's exhaustive paragraph matches the architecture ruling. Basis `exhaustive` stays value-free, the only certificate it may carry is the conditional/vacuous one, `exact-value` keeps its `Rational`, and no zero-value or new basis is introduced. The FR-412 hint rule reads correctly for its core cases:
- an absent row means some single action must satisfy every condition;
- a present row is checked as written, with no fallback;
- nonexistent → `malformed` at `policy`.

The defects are in the rule's boundaries with the `exact`/`post` certificates, lower-certificate scope, observability, wire identity and test layout.

## Verdict

**FAIL** — medium ambiguities in the FR-412 hint rule must be fixed before acceptance.

## Review notes

The review did not ask for a duplicate probability engine in the standards harness. FR-412-AC-17..19 semantics are owned downstream by FSL (PR 3285). The reset part's full coverage and properness rule (FR-412-AC-9, -11, -12) is explicitly preserved by the hint bullet, which is consistent.

## Scope

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-331 exhaustive paragraph | examined | `A proved result with basis exhaustive SHALL remain value-free. ... MAY carry its FR-412 certificate with kind conditional, conditional exactly {"mode": "vacuous"} and empty values ... Every other exhaustive result SHALL omit certificate.` |
| FR-331-AC-60 | examined | `The schema admits exhaustive-vacuous-conditional-outcome ... The same certificate remains forbidden on bounded-complete, inductive, reduced, witnessed and bounded-proof. An exact-value basis without its Rational value ... is refused` |
| FR-412 Policy hints rule | examined | `for an upper bound on a minimum or a lower bound on a maximum ... the checker SHALL independently rebuild the current actions at every undecided state. When that state's policy row is absent ... SHALL select the first such action in canonical product-action order.` |
| FR-412 policy member row | examined | `optional per-state action hints, checked by the policy rule below.` |
| FR-412 Absence | examined | policy no longer listed as a required member for lower/maximum or upper/minimum |
| FR-412-AC-17 | examined | `For a product containing only a terminal target state and its terminal stutter ... each without any policy row, are accepted.` |
| FR-412-AC-18 | examined | `At an undecided state with two PostState choices ... 1/4 and 3/4, an upper/minimum bound 1/2 with no policy row selects the first action ...` |
| FR-412-AC-19 | examined | `... whose hint selects the 3/4 action is rejected inequality-fails ... An admitted reset or randomized hint is checked by the same existing-action and all-conditions rule wherever its objective permits that form.` |
| FR-412-AC-9 | context_only | optimum reset policy b accepted |
| FR-412-AC-14 | context_only | lower 1 with policy wait rank-broken; lower 7/16 with policy go accepted |
| TC-2110 steps 9-11 | examined | terminal stutter, PostState pair, hint cases |
| TC-431 BP-65 | examined | `exhaustive-vacuous-conditional-outcome ... tc_431_exhaustive_certificate_is_only_conditional_vacuous` |

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium | The hint rule covers `upper` on a minimum and `lower` on a maximum, but `exact` certificates on a minimum or maximum also need a policy for rank progress ("Exact: both conditions"). That includes the `optimum` `post` part. The rule excludes only the reset part, so it is ambiguous whether hint or mandatory-policy semantics apply to `exact` and `post`. | spec/objects/interfaces/FR-412-probability-certificate.md:124 |
| FND-002 | medium | The hint rule rebuilds actions "at every undecided state", while a `lower` certificate is checked only over its named states and their successors. A partial-product lower proof cannot enumerate every undecided state, so the two scopes conflict. | spec/objects/interfaces/FR-412-probability-certificate.md:127 |
| FND-003 | medium | For an absent row, the normative rule does not say which rejection applies when no single action satisfies all conditions. In particular it is silent when one action meets only the inequality and another only the rank. FR-412-AC-18 asserts `rank-broken` there, a precedence the rule body never states. | spec/objects/interfaces/FR-412-probability-certificate.md:128 |
| FND-004 | medium | "SHALL select the first such action in canonical product-action order" names no consumer or report channel. No wire member or rejection carries the selection, so FR-412-AC-18's "selects the first action" clause is not observable. | spec/objects/interfaces/FR-412-probability-certificate.md:329 |
| FND-005 | medium | A present hint is a `TransitionIdentity` (operation, receiver, arguments), which cannot distinguish two `PostState` choices of one operation. FR-412 does not say how such a row resolves to one rebuilt action, so FR-412-AC-19's hint for "the 3/4 action" on AC-18's PostState product cannot be authored unambiguously. | spec/objects/interfaces/FR-412-probability-certificate.md:330 |
| FND-006 | medium | FR-412-AC-19's last sentence is vacuous. Within the hint rule's scope, no objective permits a reset or randomized choice: those forms exist only in the excluded `reset` part. No test can fail that clause. | spec/objects/interfaces/FR-412-probability-certificate.md:330 |
| FND-007 | medium | FR-412-AC-17's only state is a terminal target, which is decided with a fixed value, and the hint rule applies only at undecided states. The AC can fail only on the deleted mandatory-policy Absence rule, never on the terminal-stutter rebuild it names. | spec/objects/interfaces/FR-412-probability-certificate.md:328 |
| FND-008 | low | In TC-431, the BP-65 row is separated from the BP table by a blank line, so it renders as a paragraph, not a table row. | spec/test-cases/TC-431-backend-provider-wire-vectors.md:106 |
| FND-009 | low | The Verification cells of FR-331-AC-60 and FR-412-AC-17..19 read `Test` without their TC (TC-431, TC-2110), unlike every sibling AC, which reads `Test (TC-…)`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:1093 |

## New findings (disposition pass 1)

Round 1 covers b9757e34a96b3f2a4612a8e9450d71784b8d79b6 (FR-412 lines 124-157).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | medium | The fix says "the existing checked-acceptance outcome SHALL expose the reconstructed policy to its consumer". No such outcome is defined or cited in FR-412 or elsewhere in spec/ (the phrase occurs only here and in TC-2110). The selection's observable channel therefore rests on an undefined object, and FR-412-AC-17/AC-18 and TC-2110 steps 9-10 inspect it. Fix: cite the owning FSL requirement for the checker's accepted outcome, or define its policy member here as checker output, not wire. | spec/objects/interfaces/FR-412-probability-certificate.md:137-141 |

## New findings (disposition pass 2)

Round 2 covers 97051ccacb36ad4622f2146a5ca30ea36b3fdb2e (FR-412 lines 62-65, 109-123, 145-161, 331-335, 367; TC-2110 step 12).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-011 | medium | The new rule requires `policy` with one choice per checked undecided state for every `exact` certificate under `minimum`, and says "each required choice must satisfy the Exact rule's equality and applicable ranking conditions". That includes fair-minimum `exact`, whose own rule is existential: "every such state with `x < 1` has a tight action ... with a successor of positive probability and a smaller rank". FR-412-AC-13's `exact` certificates state no policy, so their outcomes now depend on that unstated choice. At `s` = `1/2`, policy `go` is accepted and `wait` is `rank-broken`. At `s` = 0 or `1/4`, `wait` gives the stated `rank-broken`, but `go` is not tight and fails equality (`inequality-fails`). Fix: either exempt the Fair minimum `exact` rule (it keeps its tight-action existence check), or state AC-13's policy rows. | spec/objects/interfaces/FR-412-probability-certificate.md:145-155, 208-218, 360 |
| FND-012 | medium | FR-412-AC-20 and TC-2110 step 12 require a missing `post` policy to be rejected "`malformed` at `policy`, retaining the Conditional rule's part attribution for `post`". But the Absence rule writes `malformed` at `{member}`, and FR-331's `malformed` locus (schema.json:1087) is `{member}` with `additionalProperties: false`. Only the state locus (`ProbabilityProductStateLocus`) carries `part`. That part attribution cannot be represented on the wire, and FR-412 never defines a `{member, part}` form. Fix: either drop the part attribution for `malformed` (as FR-412-AC-12 already does for `reset`, "`malformed` at `policy`") or define the locus and its schema branch. | spec/objects/interfaces/FR-412-probability-certificate.md:155, 341, 367; spec/test-cases/TC-2110-check-fair-minimum-value-bounds-and-measure-certificates.md:96-98; proposals/backend-provider-v1/schema.json:1087 |

## Dispositions

Round 1, against b9757e34a96b3f2a4612a8e9450d71784b8d79b6 (delta 0f3375a6..b9757e34). `quire validate --scope` on an exported fix-head tree passed for FR-331, FR-412, TC-431, TC-2110 and both committed SR copies. It showed only the three pre-existing FR-331 warnings. Both committed SR copies are byte-identical to the original review files.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | The hint rule is now limited to `upper`/`lower`, and says `exact` certificates "including the optimum conditional certificate's exact post and reset parts, retain their existing required policy coverage, equality, ranking and properness conditions". But apart from `reset`, no such requirement exists any more. This PR removed the Absence clause ("a lower bound on a maximum or an upper bound on a minimum without `policy`"), and the `policy` member row now lists only optional hints plus `reset` coverage. The Lower condition still takes rank progress "under the policy" for a maximum. So an `exact` (or `post`) certificate on a maximum without `policy` has no stated coverage or `malformed` outcome. Restore an explicit `exact`/`post` policy requirement (member row and Absence) or state the rule here. |
| FND-002 | fixed | The rule now says "all reachable undecided states for `upper`, and only named undecided states for `lower`, retaining the lower rule's least-value treatment of unnamed successors". AC-18 and TC-2110 step 10 add the named-state clause. |
| FND-003 | fixed | Explicit precedence: "if no action satisfies the inequality the checker SHALL reject `inequality-fails` at that state; if some action satisfies the inequality but none of those satisfies the required rank progress, it SHALL reject `rank-broken` there", after the shape and missing-rank checks. This matches AC-18. |
| FND-004 | fixed | The selection now has a named consumer channel: "The existing checked-acceptance outcome SHALL expose the reconstructed policy ... with each selected complete product key and rebuilt action ... not a new certificate or wire member". AC-18 now says "reports ... in the existing checked outcome's reconstructed policy". That outcome is itself undefined, which is recorded separately as FND-010. |
| FND-005 | fixed | A present row's `TransitionIdentity` must "identify exactly one current rebuilt action"; "A nonexistent or ambiguous choice is rejected `malformed` at `policy`". AC-19 now uses distinct operations `low`/`high` and adds the ambiguous-PostState hint → `malformed`. TC-2110 step 11 matches. |
| FND-006 | fixed | The AC-19 reset/randomized sentence was removed. The rule now says "Reset and randomized choices remain confined to the existing conditional `reset` rules". |
| FND-007 | fixed | AC-17 now uses "an active finite-window fraction measure over a terminal model state ... with nonzero remaining window, the product continuation is undecided and its only action is terminal stutter advancing the monitor". That makes the state genuinely undecided, with terminal stutter as the rebuilt action. TC-2110 step 9 matches. |
| FND-008 | fixed | The blank line before BP-65 is removed. TC-431:105 now directly follows BP-64 inside the table. |
| FND-009 | fixed | FR-331-AC-60 reads `Test (TC-431)` and FR-412-AC-17, -18 and -19 read `Test (TC-2110)`. |

Round 2, against 97051ccacb36ad4622f2146a5ca30ea36b3fdb2e (delta b9757e34..97051cca). The committed `reviews/qsl-2158-spec-review-base.md` at 97051cca is byte-identical to the round-1 SR file. `quire validate --scope` on an exported pinned-head tree passed for FR-412, TC-2110 and both SR copies with no diagnostics.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The `policy` member row now reads: "For `kind` `exact` under those minimum or maximum objectives, including an `optimum` certificate's `post` part: required, with exactly one current action choice per checked undecided state". The Absence rule adds `exact` under `minimum`, `maximum`, `workload-minimum` or `workload-maximum` (including `post`) without `policy` or without one choice per checked undecided state. The hint bullet says the checker rejects that "`malformed` at member `policy`, without inferring a replacement action". `reset` keeps its waiting-state coverage. The fix's side effects on fair-minimum `exact` and on `post` locus attribution are recorded as FND-011 and FND-012. |
| FND-010 | fixed | FR-412 now defines a checker-local **derived selected policy**: "the finite association from each complete probability product key where the Policy hints rule selects an action to that independently rebuilt action, whether inferred without a hint or verified from a present hint ... an identity action, the specific `PostState` choice, or terminal stutter ... checker output, outside the wire `ProbabilityCertificate` and FR-331 result envelope". It is reported only on acceptance. AC-17, AC-18 and TC-2110 steps 9-10 now cite it. |
