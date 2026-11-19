---
id: FR-364
title: "Carry and replay a temporal counterexample"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-361
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-362
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-363
    type: references
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-366
    type: references
  - target: ix://agent-ix/filament-specification/FR-370
    type: references
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-482
    type: references
---

# FR-364: Carry and replay a temporal counterexample

## Description

When an analysis refutes a temporal item, over a model subject or over a
supplied trace, the replayer SHALL reproduce the provider's temporal
counterexample, carried in the one wire form below, under the replay rules
below before the item settles as refuted. This requirement owns every arm,
kind and member of that wire form.

## Inputs

A temporal counterexample; the checked package; the item's clause and resolved
fairness set; for the model arm, the model subject's initial states and
universes, re-admitted from the request's byte provision; for the observed
arm, each observed document the counterexample names, held in the request's
byte provision by its zero-based position in `request.provision`.

## Outputs

A replay settlement: `reproduced-with-evaluated-witness` with the item
`refuted`; a located replay refusal; or `inconclusive` with cause
`replay-parity`.

## Behavior

### Shapes

| Refutation | Shape |
| --- | --- |
| Reachable-state invariant, safety, bounded | A finite prefix. Under a bounded profile it covers the formula's horizon from every activation it reports, or ends at a terminal state. Under infinite-trace it is a bad prefix: every infinite extension of it violates the formula. |
| Liveness | A lasso: a prefix, possibly empty, and a non-empty loop that re-enters at its first position. A model behaviour that ends at a terminal state is the lasso whose loop is the one terminal stutter step. |
| Deadlock ([FR-366](FR-366-check-deadlock-freedom.md)) | A finite prefix from an initial state to the deadlocked state. Model arm only. |
| Undefined evaluation ([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)) | A finite prefix to the position at which an expression evaluates undefined, for every property form; `loop` is empty. |
| Task-automaton deadline miss ([FR-419](../analysis/FR-419-analyze-task-set-schedulability.md)) | A prefix that reaches a concrete job-specific missed-deadline state at `trace_position` (the zero-based index of its first `deadline-miss` step), followed by exactly one checked continuation: a finite timed loop with positive total delay, a single positive-delay terminal-stutter loop from a quiescent product-terminal state, or a `queue_growth` induction witness. With a finite loop, the prefix may continue past the miss step to the loop entry. With `queue_growth`, the prefix ends at the miss step. Queue-cap overflow is internal and is never a counterexample state. A prefix without a continuation is not a refutation, whatever the claim's safety shape, and is refused before replay as `malformed_wire` at `/loop`. The Reachable-state invariant row and any replay-time exploration of a finite endpoint do not apply to a deadline-miss counterexample of a task-derived subject: its claim is restricted to time-divergent behaviours, and a prefix alone does not show that the miss lies on one. |

### Wire form

A `TemporalCounterexample` is one JSON object with exactly the members of this
table. The table is the one list of its members, arms and kinds: a requirement
that adds a member, an arm or a kind adds it here, and a member, arm or kind
this table does not list refuses before replay.

| Member | Value |
| --- | --- |
| `trace` | the arm: `"model"` for a behaviour of a model subject; `"observed"` for a supplied trace of observed documents |
| `kind` | `"formula"` for a refutation of an authored clause on which the evaluator returns false; `"deadlock"` for a refutation of the deadlock-freedom item (model arm only); `"time-lock"` for a refutation of the time-lock-freedom item ([FR-417](FR-417-settle-time-lock-freedom.md), model arm only); or the undefined-evaluation envelope below |
| `initial_state` | model arm: the zero-based index of the behaviour's initial state in the subject's ordered list of initial states; observed arm: `null` |
| `prefix` | the ordered steps of the prefix |
| `loop` | the ordered steps of the loop; empty for a finite prefix |
| `queue_growth` | present only for a task-automaton deadline-miss counterexample whose divergent continuation has an unbounded live queue; the inductive witness below; absent for every other counterexample and mutually exclusive with a nonempty `loop` |
| `over_binding` | model arm: the reference of the object the clause's `over` parameter is bound to, `null` for `kind: deadlock`; observed arm: `null` |
| `fairness` | the item's resolved fairness set, each member `{kind, operation, granularity}` with `kind` `weak` or `strong` and `granularity` `whole` or `each`, in fairness-set order; empty for `kind: deadlock` |
| `final_delay` | for `kind: time-lock` only (and then required): the exact rational delay that reaches the time-locked state ([FR-417](FR-417-settle-time-lock-freedom.md)); absent for every other kind |
| `refinement_failure` | for a refinement item only: the refinement failure ([FR-379](../analysis/FR-379-report-and-replay-refinement-counterexamples.md)); absent otherwise |
| `trace_position` | for `kind: formula`, the position at which the evaluator returns false, which for a task-automaton deadline-miss counterexample is the zero-based index in `prefix` of its first `deadline-miss` step, not necessarily the prefix's last position; for the undefined-evaluation envelope, the position of its `where`, which is the prefix's last position; `null` for `kind: deadlock` |

The arms are coupled. The replayer SHALL refuse before replay, as
`malformed_wire`, an observed-arm counterexample that has a non-null
`initial_state` or `over_binding`, a model-arm step in its `prefix` or `loop`,
a `kind` of `deadlock` or `time-lock` (which have no observed form), or an
undefined-evaluation envelope whose `where` carries `state_key` or
`state_keys`. The replayer SHALL refuse the same way a model-arm counterexample
that has a null `initial_state`, an observed-arm step (a `document`) in its
`prefix` or `loop`, or an undefined-evaluation envelope whose `where` carries
`document`. A counterexample that fails any of these conditions
is a mix of the arms.

A model-arm step is `{transition, post_state}`: `transition` is the step's
[FR-181](../runtime/FR-181-simulate-finite-executions.md) transition identity,
or `{"type":"terminal-stutter"}` for the terminal stutter step; `post_state` is
the state key of the step's post-state. An
observed-arm step is `{document}`: the zero-based position in `request.provision` of the observed
document at that position. An observed trace whose positions are record
samples of a declared record trace subject, not provisioned documents, has
the observed-arm step `{sample}` instead: `sample` is the `U64String`
zero-based index of the position in the request trace's own position list,
prefix positions first and then loop positions. The index, not the sample's
value, is the position's identity, because two equal samples at different
positions are different positions of the trace, and replay reads the sample
at that index from the same request. A sample step carries no other member;
one counterexample's steps are all `document` steps or all `sample` steps.
The loop's entry is the position after the prefix's last step.

For a task-automaton deadline-miss counterexample, the prefix's
`trace_position` step is the concrete missed-deadline step of a named live
job: its absolute deadline has been reached and its exact remaining demand is
positive. After that step the product's miss latch is set and no further
`deadline-miss` step occurs on the path or in its continuation; the claim is
already violated, and the continuation only shows that the behaviour is legal
and time-divergent. An
internal queue-cap overflow is not an endpoint and does not itself refute. If
overflow occurs, the producer must derive an actual missed-deadline state from
the legal task trace and provide that trace and state here. Exactly one of a
non-empty finite `loop` or `queue_growth` is then present; a counterexample
with neither is refused before replay as `malformed_wire` at `/loop`, one with
both as `malformed_wire` at `/queue_growth`. No other member witnesses
divergence: an asserted divergence flag cannot be replayed, while a loop with
positive delay can, so a member such as `divergence` is an unknown member and
refuses before replay. The finite loop is a concrete
positive-delay product path whose final state has the same source state and
canonical scheduler control as its entry and whose clock valuations belong to
the same FR-418 refined region. Its exact delays sum to a strictly positive
rational. A loop of one `terminal-stutter` step with positive delay is a
finite loop when its entry state enables no source or scheduler transition,
holds no live or pending job, is not urgent and has no applicable source or
task time invariant (a periodic arrival invariant `x <= T` included); region
equality is not required for it, because every delay is admissible from that
state forever. This is the terminal-state lasso of the Liveness row applied to
the product. When a completed slot is reused on the loop, the replayer
alpha-renames the next iteration's job occurrence while preserving task
declaration, slot, generation relation, demand choice and transition
provenance; immutable live-job identities may not be merged.

`queue_growth` is a second, inductive continuation form for a queue that grows
without bound, where a finite full-state loop cannot close. It contains a
finite `stem` from the concrete miss state to the inductive entry, symbolic
queue words `before` and `after`, an exact finite cycle script, the source
control and FR-418 refined clock region at both cycle boundaries, the positive
elapsed time, and the occurrence/generation renaming. A symbolic
queue word is `prefix · growth_block^(k+b) · suffix` for every integer
`k >= 0`, evaluated at cycle `k+c`; `b` is `block_offset` and `c` is
`cycle_offset`. The nonempty `growth_block` is the repeated cohort. Each job
template records its task declaration, occurrence, generation, virtual queue
position and nominal activation/deadline offsets using two-index affine forms
`base + cycle*k + cohort*i`, where outer cycle `k` and repeated-block copy
index `i` are independent natural numbers. Prefix templates use `i=0`, each
repeated-block copy uses its zero-based copy index, and suffix templates use
`i=k+b`. It also records selected fixed
work, remaining work, jitter, optional declared priority, current service
allowance and scheduler status. The script names every release,
dispatch, preemption, resumption, service amount, completion, budget change
and source feedback transition in the cycle; it states the exact symbolic
policy order among all ready template jobs and identifies the concrete missed
job only in the prefix. Each fresh cycle activation carries both its exact
source release-transition identity and its derived task-transition identity.

The witness states a positive integer `iteration_shift = s`. The `before`
word has `cycle_offset=0` and a finite `block_offset=b`; `after` has
`cycle_offset=s` and `block_offset=b+s`. It satisfies
`after(k) = before(k+s)` for every integer `k >= 0`, after applying the
declared identity map and exact two-index affine clock/identity updates. This
is checked as an equality of regular queue words and every per-job field, not
just a cardinality comparison. The checker derives `backlog_growth` as
`s * length(growth_block)`; a supplied value that differs refuses. Every
retained old occurrence maps injectively to one output occurrence with the
same task declaration, exact demand, slot-generation lineage and source
provenance. Each map entry identifies its source and destination template and
affine cycle/cohort mapping with an explicit source cohort index range and
destination function `(k,i) -> (k+s,i+δ)` (or an explicit prefix/suffix
mapping); distinct live inputs cannot map to one output. The union of map
domains covers every retained source occurrence exactly once. A consumed head
has no map only when the cycle contains its exact completion and feedback.
Every newly appended occurrence is outside all map ranges and has one matching
fresh release identity. For the representative overload, `before(k)` is
`head · G^(k+1)`, where `head` and each `G` job have two units of demand. The
finite stem completes the concrete missed job 0 over `[1,2]` and admits job 2,
so induction starts at `t=2` with head job 1 and `G` job 2. Each two-unit
cycle serves and completes its head, admits the next periodic job at `t=3`
and the following one at `t=4`, and makes the old first `G` job the next head.
Thus `after(k)` is `head' · G^(k+2)`, where `head'` is the old first `G` job,
and `after(k) = before(k+1)`. The two-index occurrence/deadline formulas
advance by two source periods per cycle and one period per cohort copy. The
live queue grows by one.

The checker verifies the cycle against the product transition rules and proves
the one-cycle lifting rule for every `k`: source control and refined region
return; each old live occurrence is either preserved by the injective
renaming or removed by its replayed completion; each new occurrence has a
unique task/release identity and generation; each service/completion and
source-feedback edge is enabled in the stated scheduler order; the output word
is exactly the declared `after` transform; and `after` has strictly more live
jobs than `before`. The renaming is injective over every retained live job and
cannot merge two identities. Clock-region and per-template clock updates are
recomputed, including saturation, diagonal guards, bounded subtraction and
nominal deadlines. The exact cycle delays sum to a positive rational. These
checks establish by induction an infinite legal product run with unbounded
queue growth and unbounded elapsed time. A source-only time-progress premise,
a queue-cap overflow, or a finite trace that does not prove this lifting rule
is not a continuation witness. A reader refuses a malformed or non-liftable
certificate before settling `refuted`.

The selected demands, activation jitter and every mode-change allowance delta
in the represented prefix and cycle are carried by their task-transition
identities and checked as exact rationals. The source activation is checked
against the bound task's periodic or sporadic inter-arrival rule; actual
release is exactly the selected jitter after nominal activation, and the
absolute deadline remains nominal activation plus `D`. Each `LO` job's
selected `d` is in `[BCET,C]`; each `HI` job's selected `d` is in
`[BCET,C(HI)]`. A `HI` job has LO-mode allowance `C(LO)` until an admitted
source transition enters HI mode. Reaching service `C(LO)` alone does not
change mode. A completed job at the threshold has no update; an incomplete
job with no enabled LO-to-HI source transition remains at that allowance. A
source-authorized `budget-change` carries exactly
`budget_delta = C(HI)-C(LO)`, leaves actual `d` and remaining work unchanged,
and preserves job identity, optional declared priority and deadline.

For a task-derived timed subject, a model-arm `transition` may be the typed
`task-transition` identity defined by FR-181 and FR-331. Its `action` names an
activation, release, dispatch, preemption, resumption, completion, budget change
or deadline-miss step. A finite queue-cap overflow has no task-transition identity and
may appear only as a `queue` frontier in bounded-search evidence; it is
refused as a counterexample step. Each task identity carries the task-set/task
declarations, job sequence, slot and generation, the nominal source activation,
the job's exact selected rational demand and jitter and absolute deadline,
plus `priority` exactly when its checked task declaration has one; absent
priority is omitted, never synthesized. This same presence/value rule applies
to each `QueueGrowthJob` template and `QueueGrowthTransition` in a
`queue_growth` witness: all carry the checked value if declared and omit it
otherwise. A supplied priority must equal the checked task declaration, and a
missing or extra priority is a replay refusal for either carrier. Replay
derives the scheduling policy from the bound checked claim. Under fixed
priority it recomputes ready order by ascending numeric priority. Under EDF it
recomputes ready order, including every symbolic `ready_order`, by absolute
deadline, with ties by ascending canonical task name in Unicode scalar value
order (FR-482), then canonical source activation transition identity and
zero-based occurrence. Source declaration order and any declared task
priority do not affect EDF ties. Declared priority is provenance only and is
ignored for EDF ordering. The triggering source or scheduler transition is
also checked. A
`budget-change` identity also carries the exact non-negative `budget_delta`;
it does not alter `demand`.
All steps for one live job repeat equal immutable identity fields; replay
refuses a change to any of them. A source activation and its bound task's
activation are checked as one derived transition; subsequent actual release
is checked at the selected jitter delay. A completion-feedback step checks
the derived completion and the source transition it enables.

The **undefined-evaluation envelope** is the one shape of an undefined
evaluation for every engine:
`{"type": "undefined-evaluation", "where": {...}, "cause": <reason>}`. `cause`
is the undefined value's catalogued reason. `where` names the evaluation point
with these key names:

| Key | Value |
| --- | --- |
| `position` | the trace position of the evaluation point |
| `state_key` | model arm: the state key of the state there |
| `document` | observed arm: the `U64String` position in `request.provision` of the observed document there |
| `sample` | observed arm over record samples, in place of `document`: the `U64String` index of the sample position there in the request trace's position list |
| `locus` | the source locus of the expression that has no value |
| `transition` | for an undefined guard only: the FR-181 transition identity whose guard it is |
| `time` | for a hybrid counterexample only: the FR-331 `Interval` of seconds that holds the decisive instant, a point when the instant is rational; a hybrid `where` carries exactly `position`, `time` and `locus` ([FR-193](../analysis/FR-193-analyze-continuous-hybrid-models.md) "Solver result on the wire"), since a continuous behaviour's evaluation point is an instant within a trace state's dwell, not a position |
| `predicate` | for a predicate of a state-graph claim only: `"P"`, `"Q"`, `"X"` or `"Y"`, the predicate that has no value ([FR-393](../../objects/interfaces/FR-393-state-graph-evidence.md)) |
| `variables` | for a hyperproperty only: the trace or execution variables with their aliases, `[{"variable":"<ident>","alias":"<ident>"}, ...]`, in the order of the plural keys ([FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md)) |

An engine whose evaluation point is not one position names it with the same
keys in their plural forms (`positions`, `state_keys`), and its own
requirement states which keys its `where` carries.

A counterexample travels as an
[FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
`counterexamples` entry whose identity binds the item's subject or supplied
trace. Every engine returns this one type. An engine reads each step from its
own encoding and names the initial state by index, so its counterexample
replays with no engine present.

### Replay

The replayer SHALL, in this order:

1. Re-admit the checked package. Model arm: re-admit the subject's initial
   states and universes, and select initial state `initial_state`. Observed
   arm: admit each step's document from the byte provision; a position the
   provision does not hold refuses `invalid_runtime_input`/`invalid-value`.
2. Model arm: re-execute each step of the prefix, then of the loop, through
   the successor relation of
   [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md). A step's
   `post_state` selects which successor of its transition identity the step
   took, since one identity can have several post-states. A step whose
   transition identity is not enabled at its pre-state refuses
   `invalid_runtime_input`/`invalid-value`. A terminal stutter step at a state
   that is not terminal refuses `invalid_runtime_input`/`invalid-value`.
   For a task-derived subject, use FR-419's product successor relation: replay
   the source transition and its task release/completion feedback, verify the
   selected demand is within the admitted interval, verify policy order and
   job slot generation, apply exact bounded subtraction and source-authorized
   budget allowance updates, and compare the full source-plus-scheduler
   post-state key. Every
   subtraction must satisfy its declared `c <= x <= M_x` side condition and
   leave a non-negative clock. A deadline-miss refutation also requires the
   exact time-divergent continuation checked by FR-418; a queue-cap overflow
   is never a counterexample endpoint, and a task-derived counterexample
   without exactly one continuation has already been refused `malformed_wire`
   before replay. Each live job's `deadline` is recomputed as its nominal
   activation plus checked `D`, and its `demand` is checked against the
   checked interval at activation. Every step that carries either value is
   compared with the recomputed one, and the first mismatch in step order
   refuses `invalid_runtime_input`/`invalid-value` at that step's
   `/transition/deadline` or `/transition/demand` pointer, naming the supplied
   and the recomputed value; a tampered value is never reported only as a
   transition that is not enabled. `prefix[trace_position]` must be the
   first `deadline-miss` step, else the counterexample refuses
   `invalid_runtime_input`/`invalid-value` at `/trace_position`. When `queue_growth` is
   present, replay its `stem` immediately after the prefix and require its last
   product state to equal the witness's `source_entry`, `region_entry` and
   `before(0)` queue valuation exactly.
   Observed arm: the trace is the steps' documents in order.
3. For a `queue_growth` witness, require an empty finite `loop` and check its
   source entry and exit state and DBM are equal, its DBM clock order and
   matrix dimensions are canonical, and `elapsed` equals the exact sum of its
   cycle delays and is strictly positive. Instantiate the queue words at
   `k=0`, execute every cycle step using the task-product transition relation,
   and check every ready-order word against the policy derived from the bound
   claim: fixed-priority ordering by checked priority, or EDF ordering by
   absolute deadline and the FR-419 deterministic tie rule. Verify that all releases obey their task's
   inter-arrival and jitter constraints; each service amount is charged only
   to the selected running job; completions and feedback cite the exact
   enabling identities; deadlines, demands, allowances, bounded subtractions
   and refined clock regions agree with the source and FR-415 semantics. Then
   check the symbolic lifting obligations: every retained job has exactly one
   destination under the injective occurrence/generation map; every removed
   job is completed in the script; every fresh job is a legal release with a
   fresh identity; the transformed output word equals `before(k+iteration_shift)`
   after the injective identity map for every `k >= 0`; and the cycle's
   source/clock-region and per-template affine clock updates preserve this
   recurrence. It derives the positive backlog growth from the iteration shift
   and growth-block length. The checker must prove the cycle script's guards, policy choice,
   feedback and queue transform by the stated word induction, not by sampling
   a finite number of repetitions. Any failed obligation refuses
   `invalid_runtime_input`/`invalid-value`; a valid witness denotes an infinite
   time-divergent continuation. It does not by itself replay or replace the
   finite concrete miss prefix.
4. If `queue_growth` is absent, for an ordinary lasso check that the loop's
   last step returns to the loop's entry (model arm: its post-state is the
   entry state; observed arm: its document is the entry document; for sample steps, whose indices name the request's own lasso positions, the loop's indices are exactly the request trace's loop positions in order), else refuse
   `invalid_runtime_input`/`invalid-value`. For a task-automaton deadline-miss
   lasso, check instead that the final source-plus-scheduler control equals
   the loop entry modulo canonical live-slot alpha-renaming, the exact clock
   valuations occupy the same FR-418 refined region, and the exact sum of
   loop delays is positive. A zero-delay loop, a non-equivalent region, a
   changed live job's task/demand/deadline/declared priority, or a stale slot generation
   refuses `invalid_runtime_input`/`invalid-value`. A one-step
   `terminal-stutter` loop is checked instead against the quiescent
   product-terminal rule of the wire form, without region equality: an entry
   that enables a source or scheduler transition, holds a live or pending job,
   is urgent or has an applicable time invariant refuses at
   `/loop/0/transition`, and a stutter `delay` that is not positive refuses at
   `/loop/0/delay`, both `invalid_runtime_input`/`invalid-value`.
4. Fairness. Model arm, for a lasso: recompute the enabled transition
   identities at every loop state from the model, and check every constraint
   of `fairness` against the loop by
   [FR-362](FR-362-apply-fairness-constraints.md): weak and strong each by its
   own lasso rule. A loop that fails a constraint is an unfair lasso and
   refuses `invalid_runtime_input`/`invalid-value`, naming the constraint.
   Observed arm: a non-empty `fairness` settles `unsupported`, cause
   `unsupported_projection`/`missing-fairness-premise`, naming its first
   constraint, with no formula evaluated, since an observed trace carries no
   enabledness (FR-362).
5. For `kind: formula`, evaluate the
   clause's formula over the behaviour with the trace evaluator. A lasso
   denotes the infinite trace that repeats its loop forever: a position past
   the represented steps reads the position congruent to it modulo the loop
   length, and past operators read the denoted trace back to position 0.
   Evaluation false at `trace_position` agrees with the engine; any other
   result disagrees.
6. For the undefined-evaluation envelope, evaluate at `where` the expression
   its `locus` names: the clause's formula over the prefix for a claim, the
   guard of `where.transition` at the last state for a guard, the
   `terminal when` predicate at the last state, or the state constraint at
   the last state, the state `where.state_key` names, for a state constraint.
   The undefined value at
   `trace_position`, with the envelope's own `locus` and `cause`, agrees with
   the engine. Replay reads both from the counterexample, never from the item.
   A defined value there, or an undefined value at another position,
   expression or reason, disagrees.
7. For `kind: deadlock`, at the last state enumerate the enabled transition
   identities and, when the model declares `terminal when P`, evaluate `P`.
   No enabled identity and `P` false (or no `terminal` declaration) agrees
   with the engine; an enabled identity or `P` true disagrees.

Agreement settles `reproduced-with-evaluated-witness` and the item `refuted`.
Disagreement settles the item `inconclusive`, cause `replay-parity`. A refusal
settles no `refuted`.

### Validity, not identity

A conformance vector checks that a returned counterexample reproduces and has
the expected `kind` and shape. It never checks which of several valid
counterexamples an engine returned.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-364-AC-1 | A liveness refutation is a lasso with a non-empty loop, a safety refutation a finite prefix with an empty `loop`, a terminated behaviour's refutation a lasso whose loop is the one terminal stutter step, and a deadlock refutation a finite prefix ending at the deadlocked state with `kind: deadlock`. | Test (TC-295, TC-297) |
| FR-364-AC-2 | A counterexample carries exactly the members `trace`, `kind`, `initial_state`, `prefix`, `loop`, `over_binding`, `fairness` and `trace_position`, plus the required `final_delay` of a `time-lock` counterexample, `refinement_failure` of a refinement item, or `queue_growth` of an unbounded task-miss continuation; each model-arm step carries its FR-181 transition identity or the terminal-stutter identity and its post-state key; each observed-arm step carries its document position; each fairness member carries its kind, operation and granularity. A missing, extra or mistyped member, or an arm or kind the wire-form table does not list, refuses before replay. | Test |
| FR-364-AC-3 | A valid formula counterexample replays through the model, passes the fairness check and evaluates false at `trace_position`, settling `reproduced-with-evaluated-witness` and the item `refuted`. | Test (TC-293, TC-295) |
| FR-364-AC-4 | A loop whose last post-state is not its entry state, a step whose transition is not enabled, and a terminal stutter step at a non-terminal state refuse `invalid_runtime_input`/`invalid-value`. | Test (TC-295) |
| FR-364-AC-5 | A lasso that fails a weak or a strong constraint of its fairness set, with enabledness recomputed from the model, refuses as an unfair lasso naming the constraint and settles no `refuted`. | Test (TC-295, TC-299) |
| FR-364-AC-6 | A counterexample on which the evaluator returns true at `trace_position` settles `inconclusive`, cause `replay-parity`; a deadlock counterexample whose last state enables a transition or satisfies `P` settles the same. | Test (TC-295, TC-297) |
| FR-364-AC-7 | A lasso whose formula reads a past operator across the loop boundary evaluates by the infinite trace the lasso denotes. | Test (TC-200) |
| FR-364-AC-8 | A counterexample of an undefined evaluation is a finite prefix with an empty `loop`, a `kind` that is the envelope `{"type": "undefined-evaluation", "where", "cause"}` and `trace_position` at its last position; its replay re-evaluates the claim, guard, `terminal when` predicate or state constraint the locus names at the reported state and reproduces the undefined value with the locus and reason the envelope carries, read from the counterexample alone, settling `reproduced-with-evaluated-witness` and the item `refuted` with cause `undefined-evaluation`; a `trace_position` at which the expression is defined settles `inconclusive`, cause `replay-parity`. | Test (TC-305) |
| FR-364-AC-9 | An observed-arm formula counterexample whose documents are held by the byte provision, with an empty `fairness`, replays over the documents' trace and settles `reproduced-with-evaluated-witness` when the formula is false at `trace_position`; the same counterexample with a non-empty `fairness` settles `unsupported`, cause `missing-fairness-premise`, with no formula evaluated; a step position the provision does not hold refuses `invalid_runtime_input`/`invalid-value`. | Test (TC-295) |
| FR-364-AC-10 | The wire schema admits an observed-arm formula counterexample (null `initial_state` and `over_binding`, two `document` steps) and an observed-arm undefined-evaluation envelope whose `where` carries `position`, `document` and `locus`; it refuses an observed-arm counterexample that names an initial state, carries an `over_binding`, carries a model-arm step in its `prefix` or in its `loop`, has `kind` `deadlock`, has `kind` `time-lock` with its `final_delay`, or has an envelope whose `where` carries `state_key` or `state_keys`; and it refuses a model-arm counterexample that has a null `initial_state`, carries a `document` step in its `prefix` or in its `loop`, or has an envelope whose `where` carries `document`. | Test (TC-431) |
| FR-364-AC-14 | The backend-provider schema admits the typed RT-25 `queue_growth` wire vector and refuses omission of `iteration_shift`, zero `elapsed`, and a simultaneous nonempty `loop`; the vector test recomputes the two-unit cycle delay, work balance, positive backlog increment, and verifies `after(k)=before(k+1)` by affine queue-word expansion for the authored index range. These are wire/vector checks only; the task replay and queue-growth behavior above remain normative and require behavioral verification in the downstream FSL implementation. | Test |
| FR-364-AC-15 | The optional `priority` member on every task-transition identity, `QueueGrowthJob` and `QueueGrowthTransition` is a faithful projection of the checked task declaration: it is absent for a task with no declared priority and equals the declared value otherwise. Replay refuses a missing, extra or forged value for every carrier. Replay recomputes fixed-priority order from checked declared values and EDF order from checked deadlines, then canonical task name, activation transition identity and occurrence; source declaration order and declared priority do not affect EDF ties, including symbolic queue-growth `ready_order`. | Test (TC-372 RT-31, RT-32) |
| FR-364-AC-16 | An observed-arm counterexample over a record-sample lasso with prefix samples `level` 0, 0 and loop sample `level` 2 is admitted with `prefix` `[{"sample": "0"}, {"sample": "1"}]` and `loop` `[{"sample": "2"}]`, and replays against the request that supplied the samples. A step carrying both `sample` and `document`, a `sample` that is a JSON number, `"01"` or an index beyond the trace's positions, a counterexample mixing `sample` and `document` steps, and a loop whose sample indices are not the request's loop positions are each refused before replay. An undefined-evaluation envelope over the same trace whose `where` carries `sample` `"1"` (with `position` and `locus`) is admitted; its `where` carrying `document` instead, both `sample` and `document`, or a `sample` that is not the index of the envelope's `position` is refused before replay. | Test (TC-1700) |
| FR-364-AC-17 | A task-automaton counterexample whose prefix holds a `deadline-miss` step, with `loop: []` and no `queue_growth`, is refused `malformed_wire` at `/loop` before replay. The same with both a non-empty `loop` and `queue_growth` is refused at `/queue_growth`. One carrying an extra top-level `divergence` member is refused at `/divergence`. A `kind: time-lock` counterexample whose prefix passes a `deadline-miss` step, with no `loop` and no `queue_growth`, is admitted: the rule binds the deadline-miss refutation, `kind: formula`, only. Over the `task-queue-growth-overload` vector: removing `/queue_growth` gives the first refusal, adding a one-step `loop` gives the second, adding `divergence` `{"elapsed": {"numerator": "1", "denominator": "1"}}` gives the third. | Test (TC-431) |
| FR-364-AC-18 | A task counterexample whose prefix continues past its `trace_position` miss to a closed positive-delay loop replays. The same with `trace_position` on a non-miss step, or with a second `deadline-miss` step after it, refuses `invalid_runtime_input`/`invalid-value` at `/trace_position` or at that step. The `task-queue-growth-overload` vector, whose `trace_position` is `0` for its one-step prefix whose only step is the miss, carries no `deadline-miss` step in its `queue_growth.stem`. | Test (TC-431 BP-58; replay cases by FSL TC-3008) |
| FR-364-AC-19 | A single `terminal-stutter` loop with delay 1 from a quiescent product-terminal state replays. With delay 0 it refuses at `/loop/0/delay`. From a state with a live job or an enabled source transition it refuses at `/loop/0/transition`. | Test (FSL TC-704, TC-3008) |
| FR-364-AC-20 | A miss step whose `deadline` or `demand` differs from the recomputed job value refuses at that step's `/transition/deadline` or `/transition/demand`, naming the supplied and the recomputed value. The same value changed on every step of the job refuses at the job's `activate` step. | Test (FSL TC-3008) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  transition identity and the state key.
- [FR-361](FR-361-read-model-behaviours-as-temporal-traces.md) owns the
  successor relation and the terminal stutter step.
- [FR-362](FR-362-apply-fairness-constraints.md) owns the lasso fairness rules.
- [FR-365](FR-365-build-canonical-temporal-counterexample.md) owns the
  canonical counterexample of an exhaustive explicit-state analysis.

## References

- filament-spec-language ADR-018 CX-1 to CX-4, SM-7, SM-8, DL-4, DL-5 and QS-8;
  ADR-019 SR-8, SV-4 and QS-5, QS-6; Linear STD-131 and STD-132; STD-135 and
  STD-136 (the `predicate` and `variables` keys).
