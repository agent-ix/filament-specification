---
id: SR-2026111920485000000001
title: "spec-review of QSL-2534 FR-193 hybrid construct locus in a called function's body"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #756 at 1da0c12f; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md (Solver result on the wire, unsupported entry bullet lines 582-612; FR-193-AC-27), spec/test-cases/TC-2051-hybrid-undefined-causes-and-construct-loci-on-the-wire.md (scope, step 3, expected step 3); context: FR-193 Continuous values and expressions, Resets, Exact replay lines 750-800, FR-193-AC-12/AC-24/AC-25, FR-272 unsupported_construct row, native-diagnostics.md line 62, FSL PR #3088 FR-1019 Class at de159d7c, FSL FR-447-AC-7"
review_set: subset
---

## Summary

Spec review of FSpec PR #756. Ticket: QSL-2534. The PR amends FR-193 "Solver result on the wire" so that an `unsupported_construct`/`expression-form` `locus` may name a term node in the body of a function that the named mode's entries call, directly or through a chain of non-recursive calls. A recursive call is not expanded, so a construct it reaches is located at the call term. The PR adds FR-193-AC-27 (fixture `BrakingH`, with `half` called and `spare` not called) and TC-2051 step 3. `quire validate` on both changed files passes. FR-193-AC-27 is not on origin/main, and no other open FSpec PR touches it or TC-2051.

## Examined scope

- The new locus rule against FR-272 and native-diagnostics. FR-272's `unsupported_construct` row keys `mode`, `variable` for a flow and `locus` (the construct's FR-331 `Locus`), and does not say which entry holds the locus, so the amendment fits that row. native-diagnostics.md:62 states no locus rule for this cause. (examined)
- The new locus rule against the rest of FR-193. Two sentences in the same bullet still describe `mode` as the mode whose entry "holds the construct", and say the package maps the locus to that entry. For a body locus those sentences are now wrong (FND-001). (examined)
- The recursive-call rule against FSL PR #3088 FR-1019 at de159d7c. FR-1019 puts a recursive call outside the class and reports it at the call term. It expands only non-recursive calls and reports a construct reached through one at its own span in the function body, naming the mode and variable of the calling entry. FR-193's rule admits exactly those loci. A recursive call inside a function reached through a non-recursive chain is a term node of that body, so both texts place it the same way. FR-193:750-754 ("expansion of non-recursive calls") uses the same term. (examined)
- FR-193-AC-27 rows. The accept row fails if a reader refuses a body locus. The `spare` row fails if a reader accepts any function body. The `mode Stopped` row and the `variable v` row each fail if a reader ignores the calling entry. `Stopped`'s entries and `Moving`'s `v` rate call no function, so `/diagnostic/detail/locus` is the right pointer and not `mode` or `variable`, since both exist. Every row can fail. (examined)
- The fixture types. `half` and `spare` have the type `(x: Rational[0, 20; 1, 2]): Rational[0, 10; 1, 4]` with body `x / 2`. FR-193 "Continuous values and expressions" makes only the value range a call precondition (`[0, 20]` and `[0, 10]`, which `x / 2` meets). This is the same shape as the existing `drag` precedent in FR-193-AC-12 and FSL FR-447-AC-7. (examined)
- Ids and validation. FR-193-AC-27 and TC-2051 step 3 are new, and step 3 follows steps 1 and 2. `quire validate` passes. (examined)

## Verdict

Not mergeable yet. Two low wording and coverage fixes are needed, each one line in this PR. The lead may instead defer FND-002. The locus rule is sound, and it agrees with FSL #3088's FR-1019 class and source-order text. Merge order: this PR first, then FSL #3088.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The bullet still defines `mode` as "the mode whose flow, guard, reset, invariant or initial entry holds the construct" and says the package maps a `Locus` "to the mode's flow, guard, reset, invariant or initial entry that holds it". For a locus in a called function's body, the construct is in that function's declaration and the entry holds only the call, so these two sentences now contradict the new "`mode` and `variable` name the entry that holds the call". Fix: "holds the construct or the call that reaches it" and "or to the function declaration whose body holds it" | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:586-596 |
| FND-002 | low | The new SHALL has a recursive branch: a recursive call is not expanded, so a construct it reaches is located at the call term. FR-193-AC-27 and TC-2051 step 3 have no row for it. A reader that walks into every called body, recursive or not, passes AC-27. A row is needed in which an entry calls a recursive function, a `locus` at the call term is read, and a `locus` inside that function's body is refused at `/diagnostic/detail/locus` | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:848 |
