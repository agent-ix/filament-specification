---
id: SR-2026111918484900000001
title: "spec-review of QSL-2464 FR-402 reserved slot names and Body check order"
type: SpecReview
analysis: integrity
scope: "agent-ix/filament-specification; PR #751 at ad252f61; spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md (Reserved slot names, Reader order step 3, FR-402-AC-12, FR-402-AC-13), spec/test-cases/TC-2082-reserved-slot-names-and-body-check-order.md, spec/objects/interfaces/FR-322-checked-package-artifact.md (Quantified guards and invariants, FR-322-AC-263, FR-322-AC-295), spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md (FR-415-AC-24), spec/test-cases/TC-367-clock-declarations.md (CK-37)"
review_set: subset
---

## Summary

Spec review of FSpec PR #751. The PR reserves the slot-parameter name form, splits FR-402 "Reader order" step 3 into four ordered checks, and renames timed quantifier binders to "place binders". Ticket: QSL-2464. The review compared the PR with filament-contract-ir main at 2ec296a1, which includes #1074: `is_slot_parameter_name` and the `over` check in `temporal.rs`, `check_hyper_shape` step 3, the placement visitor, and the `tc_486_*` tests. It also compared the PR with FR-370 "Placement and profile fit" and "Reader order", the FR-322 body-step order, and the FR-272 cause catalog. `quire validate` passes on the changed files with pre-existing warnings only.

## Examined scope

- Reserved slot form against the contract-ir decoder (examined). The form is: tag `slot`/`op`/`rcv`/`arg`, `_s` before each field, field counts 2/1/1/4, and units that are an alphanumeric, `__` or `_x` with two uppercase hex digits, decoding to non-empty, well-formed UTF-8. It equals `is_slot_parameter_name`. Every look-alike the prose and AC-12 name gets the class the decoder gives it: `slot_sv`, `op_sa_sb`, `rcv_s`, `op_sa_x4`, `slotted` and `operand` are not reserved, while `op_sa`, `op_sx`, `slot_sx_sy` and `arg_sa_sVault_sstep_si` are reserved. The PR's examples match the `tc_486_a_model_root_over_refuses_everywhere_the_spec_names` table.
- Refusal placement (examined). The positions where source identifiers become `value`/`parameter` names are listed with a catch-all. `ts#0` and other names with non-identifier characters are correctly outside the form. No existing spec or fixture name falls into the form except the slot encodings themselves.
- Step 3 order (examined). The order is: non-temporal body (`operator-ineligible`), then non-empty `fairness` (`arguments/4`), then a root that is not the first subject's (`arguments/0`), with the finite-domain non-boolean root on its own branch. This matches `check_hyper_shape` and the pinned `tc_486_the_wrapper_fairness_refusal_precedes_the_model_root_refusal`.
- Removed "formula reads a slot parameter" clause (examined). The claim that FR-370's temporal step owns this refusal is true. FR-322 lists the temporal step before the hyperproperty step, FR-370 "Reader order" states the `over` rule, and contract-ir `check_clause` refuses at `arguments/0`.
- Rename in FR-322, FR-415-AC-24, FR-322-AC-263/295 and CK-37 (examined). It is consistent where edited. Leftovers are listed in FND-005.
- Ids (examined). TC-2082 is free: main ends at TC-2081 and no open PR uses TC-2082. FR-402-AC-12 and AC-13 are free. Stale PR #640 adds an FR-402-AC-10 row, which main already has, and edits step 4 (Fairness), not step 3. It will conflict textually at the AC table but takes no id from #751.

## Verdict

Not mergeable yet. Two medium findings need small edits. FND-001: one clause of AC-13 is refused one step earlier than the AC claims, so it does not test the order it states. FND-002: the new refusal has no payload in the FR-272 catalog row it uses. The decoder equivalence, the look-alike classification, the step 3 order and the FR-370 hand-off are all correct.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-402-AC-13's last clause and TC-2082 step 7 retarget `body` to a `temporal`/`formula` node. FR-370 "Placement and profile fit" refuses that reference earlier as misplaced in a hyperproperty body, in the temporal step, with the same `ill_typed`/`operator-ineligible` at the hyperproperty node (contract-ir temporal.rs placement visitor). So the clause never reaches the Body check it claims to test. Retarget to a non-temporal node that is not misplaced, for example a Boolean `expression` node | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:317 |
| FND-002 | medium | The new `ambiguous_declaration`/`ambiguous-name` refusal names one identifier and a slot tag at one locus. The FR-272 row for that cause requires "the distinct conflicting declaration/authority selections and loci", and the PR does not amend it, so the payload of this variant is undefined | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:184 |
| FND-003 | low | "The names the encoding can produce are therefore reserved" is narrower than the form the PR defines, which is the set the decoder accepts. That set includes non-canonical escapes the encoder never writes, such as `op_s_x41`, which decodes to `A`. The behaviour matches contract-ir, but the sentence and "the decoder above" do not: the section above gives an encoding and a split argument without "uppercase" | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:163 |
| FND-004 | low | AC-12's admitted look-alikes omit the two that only the UTF-8 and uppercase clauses exclude: escaped bytes that are not UTF-8 (`op_s_xC3`) and lowercase hex (`op_s_xc3`). contract-ir's tc_486 covers the first | spec/objects/interfaces/FR-402-hyperproperty-obligation-identity.md:316 |
| FND-005 | low | Rename leftovers. FR-415-AC-22 still says "ordered slot `binders`". `timed-clause-node-vectors.json` `forms` (lines 95, 103, cited by FR-322-AC-261) and the CL-57/CL-59/CL-62 subjects still say "slot parameter", and line 103 says "quantified place slot parameter nodes" | spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md:541 |

## Dispositions

Round 1.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-402-AC-13's last clause and TC-2082 step 7 retarget `body` to a Boolean `expression` node, which FR-370's placement check admits, so the refusal comes from the Body check |
| FND-002 | fixed | FR-272's `ambiguous_declaration` row and the native-diagnostics row state this variant's payload: the identifier, its declaring source region as the locus, and the slot tag it decodes as |
| FND-003 | fixed | The reserved set is stated as the names the decoder accepts, including non-canonical escapes such as `op_s_x41`; the pointer names "Slot parameters"; the split argument says uppercase hexadecimal |
| FND-004 | fixed | AC-12 and TC-2082 classify `op_s_xC3` and `op_s_xc3` as not reserved and `op_s_x41` as reserved |
| FND-005 | fixed | FR-415-AC-22 says place binders; the `timed-clause-node-vectors.json` description strings (`forms` part/binders, CL-57, CL-59, CL-62 subjects) say place binder. No test compares those strings |
