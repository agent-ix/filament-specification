---
id: SR-202611141914001
title: "Failure-domain review of the STD-661 operation-body candidate join"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; PR #656 at ea7010a5: FR-322 Operation-body carrier (has_body derivation, check order, candidate join), FR-322-AC-397, TC-987, proposals/checked-package-v2/operation-body-vectors.json; context FR-154, FR-150, quire-contract-ir origin/main 3f716ff1 operation_bodies.rs and model_members.rs"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
  - { target: ix://agent-ix/quire-specification/TC-987, type: references }
---

## Summary

Ticket: STD-661. The ticket's three problems were real. Joining by `selected_operation` alone
refused valid packages with several candidates. OBR-21 could not be reached because the owner join
runs first. OBR-20 held two defects. The has_body derivation now matches the IR's `owned` set
(a `function` kind with a `body` clause, owned by the member). The check order matches the IR's
`validate`: graph stages, then `operation_effects`, then `operation_bodies`, then
`checked_term_data`. Dropping the owner `identity` comparison does not weaken anything. FR-154
names every type `ix://<package identity>/<artifact id>`, and the FR-322 owner join only admits a
`ModelOwner` whose identity's document declares its `node`. So when an owner `node` equals the
member, the owner's identity is already the row's document, and STD-660 FND-003 still holds.

The candidate join rule and its new vectors do not express FR-154 redefinition or the dispatch
family, and two vectors do not reach their recorded locus under the rule the PR itself states.

## Verdict

FAIL on FND-001 and FND-002.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | OBR-20's candidate never reaches the carrier, so the vector cannot produce its recorded locus. The candidate's table family is `Order.settle`, and its receiver is `Invoice`. In OBC-04 `Invoice` does not extend `Order`, and `Invoice.settle` redefines nothing, so the receiver is outside the family. The new FR-322 text says such a candidate "is not the carrier's to refuse; the dispatch join of FR-322-AC-290 refuses it". The IR carrier skips it too: `operation_in_family` first requires `conforms_to(Invoice, Order)`. So the package reaches dispatch validation, not `/operation_bodies/1/body`. The vector also still holds two defects: an out-of-family receiver, and a `no_body` row that disagrees with the candidate's body. Fix: keep the candidate inside its family. For example, set `selected_operation.declaring` to `Invoice` with receiver `Invoice`. | proposals/checked-package-v2/operation-body-vectors.json:1780-1820; spec/objects/interfaces/FR-322-checked-package-artifact.md:4010-4027 |
| FND-002 | high | OBC-07 and OBR-22 do not express redefinition. `FastService.run` and `SafeService.run` carry no `redefines`, and the excerpt format has no field for one. FR-154 says an operation that shares a supertype operation's name, without `Operation.redefines`, "stays a second member of that name". FR-150 refuses a name that resolves to distinct originals as ambiguous. So each receiver's `run` is ambiguous and outside `Service.run`'s family. The IR's `resolve`, then `effective_operation_owner` or `operation_in_family`, joins neither candidate. OBC-07 is then not the admitted case TC-987 describes ("two types redefining the operation"), and OBR-22 is not refused at the carrier. Fix: add `redefines` (the redefined operation's identity) to the operation excerpt, state it in the note, and set it on both operations. | proposals/checked-package-v2/operation-body-vectors.json:819-870; spec/test-cases/TC-987-operation-body-carrier.md:31 |
| FND-003 | medium | The candidate join rule matches by name and uses an undefined term. It says the receiver "itself when it declares (redefines) the operation `selected_operation.operation`, otherwise the nearest ancestor that declares it". Under FR-154, declaring a same-named operation is not redefinition. "Its table's family" is defined nowhere in FR-322. Two implementers would read this differently: the Rust reference reader goes by name, while the IR reader requires conformance plus a redefinition chain to the anchor. Fix: define the family as follows. The receiver conforms to `selected_operation.declaring`, and its unique effective operation of that name reaches the selected operation through `Operation.redefines` links. Then join the row of the type that declares that effective operation. | spec/objects/interfaces/FR-322-checked-package-artifact.md:4010-4021 |
| FND-004 | low | The new "Check order" paragraph places the carrier after the graph stages and before `checked_term_data`. It does not place it relative to `operation_effects`, which the IR validates first. So a package with both an `operation_effects` defect and a carrier defect has no stated first refusal. Fix: add "after `operation_effects`" to the paragraph. | spec/objects/interfaces/FR-322-checked-package-artifact.md:3981-3990 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | OBR-20 now selects `Invoice.settle` with receiver `Invoice`: the candidate is in its family and joins the Invoice row, so the carrier refuses the `no_body` row at `/operation_bodies/1/body`. One defect. Checked against the reference reader and against `operation_in_family`, `effective_operation_owner` and `operation_bodies::validate` in quire-contract-ir origin/main. |
| FND-002 | fixed | The operation excerpt format carries an optional `redefines` (the redefined operation's identity; the note states it) and `extends` lists supertype identities. `FastService.run` and `SafeService.run` name `ix://acme/svc/Service/run`, so OBC-07 is admitted and OBR-22 is refused at the carrier. |
| FND-003 | fixed | FR-322 "Candidate join" defines the family: the receiver conforms to the selected declaring type, the declaring type has one anchor operation of that name, the receiver has one effective operation of that name, and that operation reaches the anchor through `Operation.redefines`. The join row is the type that declares the effective operation. No name-only matching. |
| FND-004 | fixed | The check order paragraph and AC-397 place the carrier after `operation_effects` and before `checked_term_data`. |
