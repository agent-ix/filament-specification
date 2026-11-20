---
id: SR-202611200301100
title: "base review of the QSL-2647 export encodings spec batch (FR-813, FR-811, FR-817, FR-651, FR-814, FR-815)"
type: SpecReview
analysis: base
scope: "agent-ix/filament-specification; spec/functional/output-mapping/FR-813, FR-811, FR-817, FR-651, FR-814, FR-815; spec/test-cases/TC-1972, TC-2047 (PR #764 head 9dde28dcb4c57aae1c3a1fb05fa5d1082e4150cf)"
review_set: base
---

## Summary

Ticket: QSL-2647. Base spec review of PR #764, weighted on whether each TLA+
and Lean encoding means what the cited FSL semantics say. The past-operator
encodings (unbounded, closed `[a,b]` shift register, `[a,*]` first-held
counter, `since`/`triggered` through a register over the unbounded Boolean),
the terminal step `Next_T` with weak fairness, the FR-817 construct refusal
and the admission-cause rule check out by hand against FR-092, FR-367,
FR-361, FR-377 and FR-272. Five high findings remain: the monitor `Spec_T`
drops fairness, the absent-object reading has the wrong polarity and the
wrong instance set, protocol-subject intervals are counted per step instead
of FR-431 distance, and dense-time terminal (idle-tail) positions are not
positions of the TLA+ or Lean encoding. `quire validate` exits 0 on every
changed file.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `Spec_<clause>` for a monitored clause omits FR-812's `fairness` and the clause's own fairness conjuncts; a monitored clause with fairness exports a weaker spec under `preserved`. AC-22 uses a second, unreconciled form `Spec_T == Spec /\ ...`. | FR-813.md:95-97, FR-813.md:546, FR-812.md:44 |
| FND-002 | high | The atom-level presence conjunct makes an atom over an absent object false, but FR-361 makes the whole item refuted (`absent-key`, `undefined-evaluation`); under negation (`always not holds(...)`) TLA+ states a clause FSL refutes as holding. | FR-813.md:147-153, FR-361.md:256-264 |
| FND-003 | high | The origin antecedent `p_c \in DOMAIN v_T` is read per behaviour, but FR-361 instances are keys admitted by any initial state of the subject; a behaviour from an initial state lacking the key refutes in FSL and is vacuous in TLA+ (the snapshot-less `Init == TypeOK` case of AC-7). | FR-813.md:135-139, FR-813.md:531, FR-361.md:187-196 |
| FND-004 | high | Over a protocol subject an interval counts FR-431 distance (only counted steps; several positions share one distance) but the step counters and shift registers advance on every step; no distance encoding is stated, yet the record is `preserved`. | FR-813.md:301-305, FR-813.md:64-68, FR-813.md:196-213, FR-431.md:60-75 |
| FND-005 | high | Dense time: FSL extends a quiescent terminal state by stutter positions after positive delays (FR-161 idle tail); the TLA+ `Next_T` has no terminal disjunct and Tick states are not positions, and Lean `TBehaviour` states no idle tail; `always[2,3]` at an idle tail is false in FSL and holds in the TLA+ export. | FR-813.md:462-466, FR-813.md:490-496, FR-161.md:131-137 |
| FND-006 | medium | The `each` row form `[](pr_<P>_last = "activate" => F)` anchors monitors and past histories at position 0, not at the per-activation origin of FR-093/FR-828; ruling 4 already defers this construct, and the text must not stay as an encoding. | FR-813.md:296-299, FR-093.md:73-78 |
| FND-007 | medium | Nested past operators: `Mon` reads an inner history primed (`φ'` contains `m_<clause>_h<k>'`) and the ordinals are pre-order (outer first); TLC needs a primed variable assigned before it is read, so the conjunct order must be stated (inner first). | FR-813.md:170-178, FR-813.md:188-194 |
| FND-008 | medium | `Ids_<T> == Seq(Nat)` whenever an operation can add a key makes a finite snapshot model unenumerable by TLC (infinite quantifier and monitor function domains), contradicting "a finite model stays finite". | FR-813.md:120-125, FR-813.md:83-87 |
| FND-009 | medium | Timed intervals: one start variable `m_<clause>_s<n>` per operator cannot track the overlapping obligations of a nested timed interval (`always eventually[1 ms, 2 ms] p`); the encoding is unstated for those shapes. | FR-813.md:468-477 |
| FND-010 | medium | One step counter `n_<clause>` per clause; a clause with two interval operators needs two, and FR-814's `n_` class has no ordinal. | FR-813.md:66-68, FR-814.md:60 |
| FND-011 | medium | An `unordered` channel under `loss head`/`loss tail` is a set in FR-813, but the implementation keeps it as a sequence in send order (ruling 2: the owning FR gains the sentence and the export encodes it). | FR-813.md:326-330, FR-813.md:387-392, FR-425.md:224 |
| FND-012 | low | AC-19 uses `B0p(p_l)` without defining it (AC-9 defines `A1p`). | FR-813.md:543 |
| FND-013 | low | FR-817-AC-16 expects `incomplete` under `export.nodes` 1 "at the first node", which holds only when the protocol clause node is not first in classification order. | FR-817.md:348, FR-817.md:311-312 |
| FND-014 | low | The old text bounded each channel variable by its capacity in `TypeOK`/`WF`; the new protocol section states no `TypeOK` for the `pr_<P>_*` variables. | FR-813.md:307-343 |
| FND-015 | low | "`Seq(Nat)`, the key space `TypeOK` gives `DOMAIN v_<T>`" cites a rule FR-812 does not state (it exists only in the implementation's `TypeOK`). | FR-813.md:120-122, FR-812.md:38 |
| FND-016 | low | The conditions `real-valued-time{clause, node}` and `nondeterministic-monitor{clause, node}` are typed objects, but FR-298 types `conditions` as String; the wire form is not stated. | FR-813.md:479-481, FR-298.md:31 |
| FND-017 | low | `pre(φ)` of an unbounded past operator nested in a closed interval is ambiguous: "every atom false" gives `historically p` false, while the empty-range rule gives true. | FR-813.md:163-166, FR-813.md:180-186 |

## Verdict

Not mergeable: FND-001 to FND-005 are high. Checked and clean: the unbounded
past updates; the closed `[a,b]` register and its pre-origin `Init`; the
`[a,*]` first-held counter and the `since`/`triggered` register reading (hand
traces below); AC-8, AC-9, AC-10, AC-11, AC-21 and AC-23 expected values;
weak fairness on the terminal disjunct, which changes no liveness verdict
because FSL's terminal stutter is the only continuation and the WF only
removes TLA+ behaviours that halt monitor progress; the FR-425 state-key
mapping and the FR-426 step table; the overflow updates for `fifo`; FR-817's
refusal shape (FR-272 `unimplemented_capability`/`unimplemented-meaning`) and
the admission-cause rule; FR-811, FR-651 and FR-815 consistency; no AC id
collision with origin/main or open PRs; nothing specified before is dropped
except the protocol `TypeOK` bound (FND-014).

Hand traces (infinite-trace profile):

- `once[1,2] (v = 1)` over v = 0,1,2,3,4: register cells (k=1,k=2) per
  position (F,F), (F,F), (T,F), (F,T), (F,F); the body holds at 0-3 and fails
  at 4. Matches FR-092 and AC-8.
- `once[2,*] (v >= 1)` over 0,1,2,3: the first-held counter reads -1,0,1,2,
  so it is true at 3; a since-last-held counter reads 0 at 3. Matches AC-9.
- `historically[1,*] (v = 1)` over 1,1,0,1: a counter over `not φ` reads
  -1,-1,0,1, read `/= 1`: T,T,T,F, equal to FR-092's range `0..i-1`.
- `p since[1,*] q` with q = T,F,F,F and p = F,T,T,F: unbounded `h` =
  T,T,T,F; a length-1 register reads F,T,T,T, equal to FR-092 at each i.
