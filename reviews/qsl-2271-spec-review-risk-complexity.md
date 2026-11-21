---
id: SR-202611210022442313144108553
title: "spec-review/risk-complexity review of the FR-831 budget and complexity"
type: SpecReview
analysis: risk-complexity
scope: "agent-ix/filament-specification; QSL-2271; PR 794 head a1268905049929576de4a93789a6e06222cdec25; spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md (Reader checks, Budget); protocol-correlation-vectors.json budget"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-831
    type: references
---

## Summary

Ticket: QSL-2271. Budget assessment against filament-contract-ir FR-358 (origin/main):
- Read storage is heap bytes requested from the allocator, so G = groups × size_of(native group record) is consistent with FR-358. G is honestly left unmeasured until a reader exists, and no assurance is claimed for a specific ABI.
- The offset 124 is grounded in the data and stated relative to a baseline measured on the whole admission.
- Charges are interleaved with no unmetered preflight. The flags and view block are paid in full before they are requested.
- Alias and source_map entries are borrowed, with copy-then-charge stated.
- Charges are cumulative, with no reset, and a denied charge consumes nothing (CO-B13).
- No new limit field is added.

The remaining risk is reader-landing volatility, already disclosed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The row charge `1 + S + L` counts path *segments*, not bytes. Join 1 (resolving the full compound NodeId) and the key comparison against the predecessor (package and path segment text compared element by element) both examine caller-controlled text whose byte length is uncharged. FR-358 requires a scan over caller-controlled text to charge each examined byte, and the coverage merge-walk's node-id comparisons are likewise unmetered. Long package or segment text can therefore cost work that is never charged. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:134-157, spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:175-180 |

## Verdict

Changes requested on one medium item: charge the examined NodeRef bytes, or state why they are bounded and already paid by an earlier admission charge.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-831 step 1 claims the join resolution, key comparison and source_map lookup "examine only this row's `control` text and `alias`". The key comparison also reads the predecessor row's control text, up to the first differing byte, so as many as 2B bytes are examined while only B is charged. FR-358 charges each examined byte, so the statement is inaccurate and the comparison is undercharged by up to a factor of 2. | spec/objects/interfaces/FR-831-carry-the-authored-correlation-clauses-of-protocol-controls.md:146-154 |

## Dispositions

Round 1, head ce96a4235251d09d6b4c58d1e13e9c2a4d9ed679.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The row charge is now `1 + B + L` (B = bytes of control.package plus every path segment, L = alias bytes), and the node charge is `1 + Bn` (+ O + m when eligible), both on the FR-358 work counter and charged before inspection. No new counter, no unmetered preflight, and copied bytes are still charged. The vectors' amounts are regenerated and recomputed by the test. |
| FND-002 | still-open | New this round. Correct the statement and either charge the predecessor bytes the comparison examines or state why they are already paid (the predecessor's own row charge). |

## Dispositions (round 2)

Round 2. Head 7e8fcaf566cd05bb24068b0e26403816a8034086 (fix 7e8fcaf5 on top of a merge of origin/main 9add8a0f), local == remote.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The row charge is now one charge, `1 + B + L + Bp`, before inspection, where Bp is the previous row's control text (0 for row 0; the previous alias is not charged), and the false "only this row" wording is gone. Coverage is now `1 + 2·Bn (+ O + m)`, paying for both sides of each node-id comparison. The key, join order, roles, FR-358 counters, G and flag read_storage are unchanged. I recomputed by hand from the data: PC-REL rows 44,81,82,81,84,80,91 = 543 and scan 1341, total 1884; B03 91@542, B04 61@543, B05 88@1883, B12 88@1179, B15 271@323, B16 78@116. The test's Rust model matches, and CO-B16's differing predecessor and current lengths rule out both the "no predecessor" and the "2× current" variants. |
