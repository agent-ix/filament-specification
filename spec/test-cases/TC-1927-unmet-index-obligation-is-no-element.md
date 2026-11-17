---
id: TC-1927
title: "An unmet index obligation is undefined with reason no-element"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-144-AC-34
    type: verifies
---

# TC-1927: An unmet index obligation is undefined with reason no-element

## Description

Verify the FR-144 "Indexing" outcome of an index read whose index obligation is unmet: under direct kernel evaluation, which runs no linking proof, a sequence or ordered-set index out of range and a map key with no entry are each `undefined` with the native-diagnostics undefined reason `no-element`, located at the `filament.op.collection.index` application.

## Test Procedure

Select `filament.value.complete/v1` and evaluate directly in the kernel, with no linking proof, over `s: Sequence<Integer>[0, 3]` holding `7, 8, 7`, `o: OrderedSet<Integer>[0, 3]` holding `7, 8` and `m: Map<Int[0, 3], Boolean>` holding the single key `1`. Evaluate `s[-1]`, `s[3]`, `o[2]` and `m[0]`, recording each outcome, its reason, its locus and its charges. Then evaluate `s[2]`, `o[1]` and `m[1]`.

## Expected Results

Each of `s[-1]`, `s[3]`, `o[2]` and `m[0]` is `undefined` with reason `no-element`, located at its `filament.op.collection.index` application, after both operands charge and with no `collection.element` or `collection.result-retain` charge and no value. None is refused, incomplete, a default, a wrapped or a clamped element. `s[2]`, `o[1]` and `m[1]` complete `7`, `8` and the value at key `1`.
