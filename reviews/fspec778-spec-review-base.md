---
id: SR-2026112105041303
title: "Spec review (base): PR 778 regenerated body_locations vectors against FR-322 binder link and node identity"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; proposals/checked-package-v2/checked-term-data-vectors.json (body_locations.packages f and h, BL-01..03, BL-R1..R10, BL-S1..S6) at 6313e9a5 against a34bb5cc; spec/objects/interfaces/FR-322-checked-package-artifact.md (The binder link, Node identity, Operand index segments, Body locations)"
review_set: subset
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
---

## Summary

Ticket: QSL-2691. This pass reviews the vector regeneration in 8813374b against the spec; the spec text itself was reviewed in SR-2026112104184601, whose FND-002 asked for exactly this regeneration.

**Is regenerating the let ids what the spec requires?** Yes. FR-322 "The binder link" requires the binder, its level and its type to be "part of the `let` node's identity". Identity is the structural key, the `node_id` with each operand slot rebuilt ("Rebuilding the key"); `dependencies` do not enter it. So the binder must be an operand of the let's derived path, as the `owner_model` rule already does for protocol clauses: "part of the node's path and enters the node's exact local `dependencies`". The reader recomputes no id and does not compare a derived path with the body. So the regeneration is producer-correct and the reader is indifferent to it. It also keeps the vector out of the duplicate-`node_id` trap: two lets that differ only in an unread binder's annotation would otherwise share a path. The new `binder` operand sits before `name`, in the RFC 8785 member order of the binding term.

**Independent validation** (a Python script over both revisions; f and h each validated against `schema.json` with jsonschema 4.19.2, alone and with every BL-01..03 and BL-R1..R10 row set inserted):

- All four lets (f 18, 19; h 17, 18) carry `binder`. Each binder names a `value`/`parameter` `#ieee_operand` node, is in the let's `dependencies` and sits in its path. The outer let binds the lower level: f binds at 3 and 4, h at 2 and 3.
- Every node-held and document-held `NodeRef` resolves (`source_map` included), and the structural keys are unique. Every operand row is numbered by first occurrence in RFC 8785 order, with no repeat and no unused entry.
- Each application's `dependencies` are exactly its reference targets plus the binder, in ascending node-id order.
- Between a34bb5cc and 6313e9a5 only the keys of the four let nodes change. No graph index moves, and the function nodes still reach the outer let.
- Every BL-01..03 and BL-R1..R10 row resolves to the same application index as before (f: 12, 10, 13, 6, 8, 9, 11, 15, 17; h: 12, 10, 13, 16, 16, 6, 8, 9, 11). No row names a let node, so the BL vectors need no re-keying, as the coder claimed. BL-S1..S6 are all refused by their schema defs.

## Verdict

**CONDITIONAL**: the regeneration is correct and consistent. One low wording finding, and one low pre-existing mismatch between the vectors and AC-490's prose.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "The binder link" says the application's `dependencies` SHALL include the binder node, "so the binder, its level and its type ... are part of the `let` node's identity". Identity is the rebuilt `node_id` path, and `dependencies` do not enter it. The rule never says the binder is an operand of the let's derived path, which is what makes it identity and what 8813374b did. Compare the `owner_model` rule ("part of the node's path and enters the node's exact local `dependencies`"). A producer reading only this paragraph could keep the old path and collide two lets that differ only in an unread binder. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3483-3489 |
| FND-002 | low | Pre-existing (outside the delta), and it fixes the binder levels: AC-490 names `function f(p, q)` and `function h(p, r)`, but package `f` has a third parameter `r` (node 20, level 2), so f's lets bind at levels 3 and 4, not 2 and 3. Package `h`'s parameters are `p` and `q`. The vectors are internally consistent, but they do not show the functions the AC text describes. | proposals/checked-package-v2/checked-term-data-vectors.json body_locations.packages.f nodes 20-21, packages.h node 19 |

## Scope

- FR-322 "The binder link" (examined; FND-001)
- FR-322 node identity, derived path label, operand rows (examined, clean)
- FR-322-AC-490 vectors f/h (examined; FND-002)
- FR-322-AC-512 (context_only)

## Dispositions

Round 1, against 5b70cf1a (delta 6313e9a5..5b70cf1a).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | "The binder link" now says the binder node "is also an operand of the `let` node's derived `path`, written after the `let` form and before the name", and that two lets that differ only in an unread binder have different ids. That matches the regenerated vectors. |
| FND-002 | fixed | AC-490 now reads `f(p, q, r)` (`r` declared and never read, lets at levels 3 and 4) and `h(p, q) { (p + q) <= (p + q) }` (lets at levels 2 and 3), and TC-2090 step 1 matches. A grep finds no stale `h(p, r)` or two-parameter `f` text left in spec/ or proposals/. The vectors are unchanged in the delta. |

Round 3, against b691a618 (delta 83af4f90..b691a618).

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | FR-464 line 189 now reads "strictly ascending node-id order, compared on the structural keys the members resolve to (FR-322 "Rebuilding the key"), never on their wire bytes". Lines 228 and 271 say the same, and the graph-position wording is gone (git grep finds no "graph rank" or "lists its nodes in" in spec/ or proposals/). This agrees with FR-322 graph order (producer's choice, never required sorted), Locality, AC-95 and the structural-key node order, and with contract-ir #1107's structural-key ranks. |
| FND-004 | fixed | Lines 203-208 now say an id held inside a node "carries operand slots numbered per node (FR-322 "Locality"), never graph indexes", and that the producer knows the structural keys, so the order is computable. |

## New findings (disposition pass 2)

Round 2 covers 83af4f90 (QSL-2717, FR-464 lines 189, 202-207, 228, 271).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | The new FR-464 set rule orders a reference set by "the resolved graph rank of the nodes they reference, the order the graph lists its nodes in". That is `semantic_graph.nodes` array order, and it contradicts FR-322 in three places. (1) "Graph order ... the producer chooses. A reader neither sorts the nodes nor requires them in any sort order" and FR-322-AC-95, where a reversed graph is admitted with no order refusal: under this rule, reversing the graph makes every protocol set noncanonical. (2) Locality: a node's bytes "do not depend on graph order", but a set's member order would. (3) The node order "is defined on structural keys ... never of graph indexes". It also misreads the reader it cites. In contract-ir PR #1107 (c68a838e), `GraphIndex::with_ranks` ranks each node by its structural key in `meter.structural_order`, and `ResolvedKey` says that rank and `CheckedNodeId::cmp` "agree on every pair the graph ranks". So the "rank" is ascending structural-key (node-id) order, not graph position. The claims that `dependencies` and `related` use graph order also conflict with FR-322, where `dependencies` are "node-id-ascending". Fix: keep "strictly ascending node-id order", and say it compares the structural keys the members resolve to (FR-322 "Rebuilding the key"), never the members' wire bytes; drop "the order the graph lists its nodes in". Apply the same change at lines 228 and 271. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:189, 202-207, 228, 271; spec/objects/interfaces/FR-322-checked-package-artifact.md:280-292, 1881-1895, 1811-1817, 1992-1997 |
| FND-004 | low | The rationale sentence says "a wire id holds a graph index". A node-held id, which is what a set member's reference is, holds operand slots, never graph indexes (FR-322 "Locality": "A node's bytes hold no graph index of its own graph"). The real reason a producer cannot sort by wire bytes is that one node's slot numbering differs from another's; the structural key is still known to the producer. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:203-206 |
