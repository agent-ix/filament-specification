---
id: SR-202611141914003
title: "Code review of the STD-661 operation-body reference reader change"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; PR #656 at ea7010a5: tests/checked_package_v2.rs read_operation_bodies and operation_body_carrier_vectors_match_the_closed_member_schema_and_reader"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
---

## Summary

Ticket: STD-661. This is a code review with its Rust lane (rust-review). The Rust edit is a
legitimate reference-reader change. The repo's own vector test reads `operation-body-vectors.json`.
Removing `has_body` from the documents forces the graph-derived has_body, and the new candidate
shape forces the selection-key join. The new `member` string is computed once and serves both the
has_body derivation and the body-node owner check, which matches the spec. `cargo test --test
checked_package_v2` passed 169 of 169.

## Verdict

CONDITIONAL on FND-001. The reader agrees with the vectors only because both read redefinition by
name. That reading is the one FR-154 and the IR reject.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The candidate join in `read_operation_bodies` does not check family membership and matches redefinition by name. It walks `extends` from the receiver to the first type that declares an operation with that name, and never checks that the receiver is in the family of `selected_operation.declaring`. That is why OBR-20 (receiver `Invoice`, outside `Order`'s family) and OBR-22 (`run` redeclared with no `redefines`) pass here, while the IR reader skips both candidates. Fix: once the vectors carry `redefines`, require that the receiver conforms to the selected declaring type and that the effective operation reaches the selected one through `redefines`. | tests/checked_package_v2.rs:22844-22882 |
| FND-002 | low | Types are looked up by bare name across every document (`find(\|ty\| ty["name"] == at_type)`), and `extends` resolves as a bare name in any document. If two documents both declare a type with that name, the join walks the wrong chain. The current vectors have unique names, so nothing fails today. Fix: resolve within the receiver's document, or by identity. | tests/checked_package_v2.rs:22848-22866 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `candidate_row_key` checks conformance, a unique anchor, a unique effective operation (a same-named operation hidden by `redefines`) and `redefines` reachability through distinct conforming supertypes. A wrong join fails the suite: without the `redefines` hiding, OBR-22 would be admitted, and a join by `selected_operation` would make OBC-07 refuse. 169 of 169 tests pass at d9c44085. |
| FND-002 | fixed | Types and operations are keyed by identity `ix://<document>/<type>[/<operation>]`, and `extends` and `redefines` are identities. |
