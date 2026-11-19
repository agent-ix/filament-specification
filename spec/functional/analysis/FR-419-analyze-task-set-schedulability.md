---
id: FR-419
title: "Analyze task-set schedulability"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-142
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-290
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: references
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
  - target: ix://agent-ix/filament-specification/FR-416
    type: references
  - target: ix://agent-ix/filament-specification/FR-322
    type: references
  - target: ix://agent-ix/filament-specification/FR-482
    type: references
  - target: ix://agent-ix/filament-specification/TC-372
    type: verifies
---

# FR-419: Analyze task-set schedulability

## Description

When a package declares a task set and a schedulability or response claim
over it, the analyzer SHALL decide the claim by the exact closed-form analysis
of the named policy in exact rationals, conditional on the task set's stated
premises, and SHALL settle it only after its evidence is recomputed.

## Inputs

A `taskset` declaration of the shared grammar's timed forms (including the
optional `bcet <time>` task member); a `schedulable` or `response` claim; the
FR-290 kind `schedulability`.

## Outputs

`proved` with basis `closed-form{analysis}` and its evidence; `refuted` with
its evidence; or an inconclusive or unsupported result with its typed cause.

## Behavior

### Task sets

A task set is for one preemptive uniprocessor. Each task declares its
arrival (periodic with period `T`, or sporadic with minimum inter-arrival time
`T`), its relative deadline `D`, its best-case execution time `BCET` and
worst-case execution time `C`, its optional `priority` (a natural number; a smaller value is a higher priority),
and its criticality `LO` or `HI`
with budgets `C(LO) <= C(HI)` for a `HI` task. `BCET` is an optional
non-negative lower bound on actual job demand, written as `bcet <time>` after
`deadline` and before `wcet`; when omitted it is zero. A
`LO` task selects one actual demand `d` in `[BCET,C]`; a `HI` task selects
one actual demand `d` in `[BCET,C(HI)]` at activation, independent of mode.
For a `HI` job, `C(LO)` is only the service allowance before a source-authorized
mode change; it does not change selected demand. Release jitter `J` and blocking term `B` are optional members; a
task that declares neither has `J = 0` and `B = 0`. Every value is an exact rational time
quantity (FR-142). A `LO` task declares one budget `C`. A task set with a
non-positive `T`, `D` or `C`, a negative `BCET`, `J` or `B`, `BCET > C` for a
`LO` task or `BCET > C(HI)` for a `HI` task, a `HI` task with
`C(LO) > C(HI)`, or a `LO` task with dual budgets refuses with
`invalid_timed_model`/`invalid-task-set`, naming the task, and a task set
that declares no task refuses the same way, naming the task set. The shared
grammar admits each of these forms, so they refuse at check time, never as
`invalid_syntax`. A `fixed_priority`, `amc` or `response` claim over a task set
in which two tasks have equal priority refuses with
`invalid_timed_model`/`invalid-task-set`, naming both tasks, and one over a task
set in which a task declares no `priority` refuses with the same code, naming
that task; an `edf` claim reads no priority, so equal or missing priorities do
not refuse it. An `edf` claim over a task with a `J` or `B` other than zero,
and an `amc` claim over a task with `D > T`, refuse with
`invalid_timed_model`/`invalid-task-set`, naming the task, because the
analysis does not read the term or does not cover that deadline. A task set
with several of these defects reports each: the task set's own defects in
source order, then each claim's claim-dependent defects, claims in source
order.

In a checked package each `schedulable` or `response` claim is one
`claim`/`schedulability` node that holds the whole task set
([FR-482](../../objects/interfaces/FR-482-checked-package-schedulability-claim-body.md)),
and its obligation identity binds every premise.

### Premises

`C`, `C(LO)`, `C(HI)`, `B` and `J` are stated premises. They are part of the
task set, so part of the subject and its obligation identity, and a verdict
holds for exactly those values.

### Claims

`schedulable N on S under fixed_priority`, `under edf` or `under amc`, and
`response N on S.τ <= d under fixed_priority`, `N` the claim's name and `S`
the task set, each requiring the FR-290 kind `schedulability`. The proof basis
`closed-form{analysis}` names the analysis as `fixed-priority`, `edf` or
`amc-rtb` (FR-331).

### Fixed priority

Each task's worst-case response time is the least fixpoint of
`R = C_i + B_i + Σ_{j ∈ hp(i)} ⌈(R + J_j) / T_j⌉ · C_j`, iterated from
`C_i + B_i` in exact rationals; the response time is `R + J_i`. The iteration
stops at the fixpoint or as soon as `R + J_i` exceeds the claim's bound (`D_i`, or `d` for a `response` claim). A deadline
beyond the period uses the busy-period extension over the jobs of the level-`i`
busy period: job `q` of that busy period (0 when `D_i <= T_i`, whose busy
period ends with the first job unless that job misses) iterates
`R = (q + 1) · C_i + B_i + Σ_{j ∈ hp(i)} ⌈(R + J_j) / T_j⌉ · C_j` from
`(q + 1) · C_i + B_i`, and its response time is `R + J_i − q · T_i`. A miss
is the task and the job `q` whose iterates reach a response time above the
bound, which is `D_i` for a `schedulable` claim and `d` for a
`response N on S.τ <= d` claim. For `q > 0`, every earlier job `p < q` keeps
the busy period open: its completion `w_p`, the fixpoint of the same
recurrence for job `p`, satisfies `w_p > (p + 1) · T_i − J_i` (strictly);
otherwise job `q` lies past the busy period and is not a miss. The iterates
rise monotonically to the fixpoint, so the first iterate above the bound
proves the miss, with jitter, blocking and `D_i > T_i` included.

### EDF

Utilization above 1 fails. Otherwise the demand bound
`dbf(t) = Σ_i max(0, ⌊(t − D_i) / T_i⌋ + 1) · C_i` must not exceed `t` at any
absolute deadline up to the synchronous busy-period length, checked by Quick
Processor-demand Analysis.

### Mixed criticality (AMC-rtb)

Low-mode response times with `C(LO)`; high-mode response times with `C(HI)`
over higher-priority `HI` tasks; and the mode-change bound that charges
higher-priority `LO` tasks only up to the low-mode response time. A low-mode
miss is a miss and refutes. AMC-rtb is sufficient: a high-mode criterion that
fails settles `inconclusive`, `sufficient-test-failed`.

### Verdicts and evidence

| Outcome | FR-331 value | Evidence |
| --- | --- | --- |
| Schedulable, or response bound met | `proved`, basis `closed-scope`, proof basis `closed-form{analysis}` | Each job's fixpoint sequence of each task (fixed priority, AMC; one job when `D_i <= T_i`); the busy-period length and QPA's check sequence (EDF) |
| Miss | `refuted`, basis `decisive-counterexample` | Fixed priority: the task, the job `q` of its level-`i` busy period that misses, and that job's recurrence iterates up to the first whose response time exceeds the bound; EDF: a `t` with `dbf(t) > t`, or the utilization above 1 |
| AMC high-mode criterion fails | `inconclusive` | `sufficient-test-failed` |

`check_closed_form` recomputes the evidence in exact rationals before the
verdict settles; a disagreement settles `inconclusive`, `replay-parity`.

### Miss counterexample

A closed-form miss is carried as FR-331's canonical counterexample of kind
`schedulability-miss`, so a refuted schedulability item has a counterexample
like every other refuted item, and a reader recomputes the miss from it and
the checked task set:

```json
{"type": "schedulability-miss", "claim": "<qualified name>",
 "analysis": "fixed-priority" | "edf" | "amc-rtb", "unit": "<time unit>",
 "miss": <miss>}
```

`claim` is the declared qualified name of the `schedulable` or `response`
claim; `analysis` is the analysis that found the miss, spelled as the
`closed-form{analysis}` proof basis spells it; `unit` is the task set's
checked time unit, in which every time value of `miss` is stated. Every
value is an FR-181 rational `{"type": "rational", "numerator", "denominator"}`
in lowest terms. `miss` is exactly one of:

- `{"kind": "response", "task": "<task name>", "job": "<decimal>", "iterates": [<rational>, ...], "response": <rational>, "bound": <rational>}`
  for a fixed-priority or AMC-rtb miss: the task, the job `q` of its level-`i`
  busy period that misses, that job's recurrence iterates from
  `(q + 1) · C_i + B_i` up to the first whose response time exceeds the
  bound, that response time `R + J_i − q · T_i`, and the bound (`D_i` for a
  `schedulable` claim, `d` for a `response` claim). Under `amc-rtb` it also
  carries `"mode": "lo"`, since only a low-mode miss refutes.
- `{"kind": "demand", "t": <rational>, "demand": <rational>}` for an EDF
  demand miss: the point `t` and `dbf(t)`, with `dbf(t) > t`.
- `{"kind": "utilization", "utilization": <rational>}` for an EDF set whose
  utilization exceeds 1.

The object is closed. A reader refuses a missing member
`invalid_result`/`missing-member`, an unknown member `invalid_result`/`unknown-member`,
and a `miss` of another `kind`, a `mode` outside `amc-rtb`, an empty `iterates`
or a rational not in lowest terms `invalid_result`/`invalid-value`, each naming
the member by its pointer. The counterexample is recorded only after `check_closed_form`
recomputes it; it has no replay target, because the recomputation is its
check.

### Task automata

A timed model whose operations release tasks of a task set, each by a
`release S.τ on M::o;` declaration, is a task automaton. The frontend lowers
the source model, task set and selected scheduling policy to one finite-product
timed subject. Its discrete state is the source model state paired with the
policy's finite scheduler control and bounded task-instance slots. Its clock
state contains the source clocks and the auxiliary clocks specified by FR-415.
The schedulability item is the timed invariant `always holds(not
deadline_missed)` over that product and uses FR-416's time-divergent behavior
scope. It is settled only after the exact product semantics and any witness
have been replayed under FR-181 and FR-364.

Each release creates one job with a stable identity: the task-set and task
declarations, the source release transition identity, and that release's
zero-based occurrence among transitions with that identity in the replayed
prefix. A job keeps this identity, its declared priority when present,
release time and absolute deadline until completion or removal by an
explicitly specified task-set policy. EDF jobs whose task omits priority have
no priority member in their derived identity; no default priority is
assigned. A scheduler slot is storage for that job, not its identity; moving a
job in the queue or reusing a slot after completion does not rename a live job.

The exact identity above is retained in every replayed task step and timed
witness. It is not the identity of an FR-418 finite certificate edge. A
certificate edge uses the finite `TaskTransitionTemplate` defined by FR-418:
its `job_rank` and `slot` bind one live job in the source product state (or
the newly allocated job in an activation's target state), while the exact
source-transition occurrence and slot generation are instantiated only when
the checker replays a concrete path. The reader's finite closure treats two
product states as equivalent when their active jobs correspond by a
task-preserving bijection that preserves phase, clock associations, queue
order and activation order; completed jobs are absent from both states and
may be renamed. Thus absolute occurrence and generation counters, which can
grow on a periodic cycle, do not enter the finite state key. This quotient
never merges two live jobs or changes an exact replay identity. The reader
recomputes the finite successor relation under the quotient, and FR-364
reconstructs exact job occurrences and generations along any counterexample
prefix before accepting its replay evidence.
Each source declaration `release S.τ on M::o;` resolves `S` and `τ` to exactly
one task-set declaration in `S`; a missing or ambiguous task binding is an
invalid task automaton, refused as a name-resolution failure
(`missing_declaration`/`missing-name` or
`ambiguous_declaration`/`ambiguous-name`), not as `invalid-task-set`. The source operation is that task's **activation**.
All release declarations bound to one task share one activation clock and one
occurrence counter, so adding another source operation cannot bypass the
task's arrival rule. The first activation establishes the task's phase. A
`periodic T` task may activate again only exactly `T` after its previous
activation; a `sporadic T` task may activate again only after at least `T`.
For a periodic task, the product's arrival-clock invariant `x <= T` prevents
time from passing the next required activation; if no bound source operation
can activate it at `x = T`, that path is time-locked and FR-417 reports it.
For a sporadic task, the guard `x >= T` prevents early activation. Both rules
apply to every activation, including zero-time source-operation cycles.

An activation is the job's nominal arrival. Its exact release jitter `j` is
chosen once from `[0,J]`, where absent `jitter` means `J=0`; the instance
becomes ready exactly `j` time units after activation. The deadline clock
starts at nominal activation and its absolute deadline is activation time plus
`D`, matching FR-419's closed-form response bound `R + J`. Jitter delays
availability to the scheduler; it does not shift the deadline. The pending
instance retains its source activation identity and selected jitter, and it
cannot be released twice. This construction is part of the derived subject
and is replayed with its exact delay. Under a schedulable path, a task's live
instances are bounded by the number of nominal activations in the `D` window:
at most `ceil(D/T)+1` for both periodic and sporadic tasks. This bound follows
from the enforced minimum nominal activation spacing `T`; the product does
not rely on WCET as a minimum service amount to bound the queue. It applies to
all live jobs on a no-miss path, including pending jittered releases, because
the absolute deadline remains measured from nominal activation.
If the selected jitter places actual release after the nominal absolute
deadline, the pending job misses at that deadline with its full positive
remaining demand; it receives no service before actual release.
If no finite bound is established for an admitted release mapping,
the analysis stops with replayable bounded-search evidence rather than
truncating or refusing the source behavior.

Release, dispatch, preemption, resumption, task completion, budget change and
deadline miss are derived transitions, each carrying the source/task
provenance and job identity that enables it. Exceeding a finite analysis slot
cap is a search/frontier signal, not a task transition or a counterexample
step. These derived transitions do not claim to be source-model operations.

The product has one processor. Under fixed-priority policy, the ready job with
the smallest declared numeric priority runs; equal priorities of two tasks
are rejected at claim admission. Jobs of one task share its priority; they are ordered by nominal activation
time, then canonical source activation transition identity, then zero-based
occurrence, so an earlier job of the task runs first. Scheduling is
preemptive: whenever a higher-ranked job is eligible, a lower-ranked running
job is preempted. Under EDF, the ready job with the earliest absolute
deadline runs, independent of any declared priority. Equal-deadline jobs are
ordered by ascending canonical task name in Unicode scalar value order
(FR-482), then by canonical source activation transition identity and
zero-based occurrence. Source declaration order and fixed priority do not
affect EDF ties. This tie rule is part of the checked task product and replay.
At a delay of `d`, the running job receives
exactly `d` service and every waiting or preempted job receives zero service;
all source and auxiliary clocks still advance by `d`. A preemption preserves
the suspended job's accumulated service, optional declared priority, identity and absolute
deadline. Completion removes that job and makes the next policy-selected job
eligible to run. A release or scheduler action takes zero time unless its
transition explicitly carries a positive delay. A source completion-feedback
transition is synchronized with the corresponding derived completion step and
retains both transition identities in replay evidence.

The exact fixed-work class applies when the admitted source/product semantics
constrains each job's actual demand to one positive exact value `W_i` (the
singleton case `BCET_i = WCET_i` in the task-automata notation). The task-set
`BCET` supplies a lower bound and `C` (or `C(HI)`) supplies the upper bound;
`BCET = C` for a `LO` task selects its positive singleton directly. The
exact fixed-work route admits a `HI` task only when `BCET = C(LO) = C(HI)`;
mode-change products with unequal budgets remain on bounded exploration. Other existing source/product
constraints may also establish a singleton if the reader proves them for every
job. A WCET `C_i` alone never implies an exact demand. The exact decision
procedure is selected only when the FR-418 reader verifies a finite closed product: it recompiles the source/task product, uses the declared finite `live_job_cap`, and exhaustively enumerates every reachable source control, refined clock region, scheduler state, arrival/jitter phase and queue state up to that cap. It recomputes every legal source, periodic/sporadic activation, jitter, service, completion and feedback successor. The certificate is accepted only if every reachable successor is represented and no reachable attempted allocation exceeds `live_job_cap`; the reader computes positive-delay SCCs from the resulting closed graph. The cap and method label are inputs to this recomputation, not proof assertions. This provides a terminating decision procedure for the verified finite product and makes no universal claim over unbounded queue languages. The paper's no-time-stops premise is handled by graph analysis: only a checked positive-delay continuation establishes time-divergence, while time-locked branches remain under FR-417. If the finite graph does not close within the declared cap, the exact route is unavailable; the task remains admitted and uses bounded exploration, which may return replayable `incomplete`. A queue-growth witness may still establish a particular divergent refutation, but does not establish completeness or authorize `proved`.

The exact fixed-work lowering proposes at most `ceil(D_i / T_i)+1` slots for type `i`, in addition to finite scheduler control. This is a candidate finite cap, not an assumption about post-miss queues: if exploration reaches an attempted allocation beyond it, the reader rejects the finite-product certificate and the analyzer uses the bounded route (or a separately replayed queue-growth refutation). Queue-cap overflow is internal and never a miss or counterexample endpoint. A concrete refutation still requires FR-364 to replay the actual job-specific deadline miss and a checked positive-delay continuation. An existing sound reduction may select another exact finite procedure only when its policy and feedback premises cover the admitted model. The fixed-work product is decided by the
bounded-subtraction construction: each task instance has unit-rate service and
deadline clocks; diagonal clock constraints compare service/deadline values;
completion subtracts the completed job's fixed work from suspended-job service
clocks. A subtraction of constant `c` is enabled only in `c <= x <= M_x` and
leaves `x - c >= 0`; FR-415 fixes and validates every clock ceiling `M_x`.
These clocks never have a rate other than one. The resulting finite refined
region graph is exact for fixed work only after FR-418 has verified closure
under the declared finite live-job cap. A fixed-work task automaton remains
admitted when that finite closure cannot be established, but it uses sound
bounded exploration and may settle `incomplete`, never `unsupported` or a
permanent refusal. It may refute only with a concrete miss and a checked FR-364
continuation.

For a task automaton using a task-set member's execution bounds, each activated
`LO` job selects one exact rational total demand `d` in `[BCET, C]`; each
`HI` job selects one exact rational total demand `d` in `[BCET, C(HI)]`. The choice is made once
at nominal activation, is retained across pending jitter, preemption and replay,
and is never resampled. A zero-demand job completes at activation without
processor service and cannot be a deadline miss.

**Criticality mode.** A task product over task set `S` has one criticality
mode, `LO` or `HI`, held in its scheduler control (FR-181). Every initial
product state is in `LO`. The source authorizes the change through a
mode-switch declaration `mode S hi on M::o;` of the shared grammar, which
binds operation `M::o` as a mode switch of `S`. It resolves `S` and `M::o`
as a release declaration does. An unresolved task set or operation refuses
`missing_declaration`/`missing-name`, an operation that resolves to several
declarations refuses `ambiguous_declaration`/`ambiguous-name`, and a second
declaration of one task set on one operation refuses
`conflicting_declaration`/`duplicate-declaration`, each at the declaration;
these are name-resolution failures, not task-set defects, so they never use
`invalid-task-set`. Several operations may be
bound, and one operation may carry both `release` and `mode` declarations. A
product step whose synchronized source transition is a step of a bound
operation, taken from a pre-state in `LO`, is the product's **entry into
HI**, and its post-state is in `HI`. A step of a bound operation taken in
`HI` changes no mode and no allowance. Nothing else enters `HI`: not a job
reaching service `C(LO)`, not a deadline miss, not a source field value, not
a scheduler step. No product step leaves `HI`, and the grammar has no `lo`
form. A task set with no mode-switch declaration on the product's model stays
in `LO` on every path. The mode is named by the declaration, never inferred
from a source field or an operation name, because the product must know which
step authorizes the higher budgets before it can bound any job's service.

**Allowance.** Each live `HI` job has an allowance phase, `lo` or `hi`, in the
product state. Its service allowance is `C(LO)` in phase `lo` and `C(HI)` in
phase `hi`; it does not change the job's selected `d`. A `HI` job activated
in `LO` starts in phase `lo`. One activated in `HI`, including by the entry
step's own source transition, starts in phase `hi` and has no budget-change
step. A `LO` job has no phase, and only its selected demand bounds its
service. No delay carries a running job's service past its allowance. A
running job whose service equals its selected demand completes, so a job with
`d = C(LO)` completes at the threshold with no update. Otherwise, a running
job whose service equals an allowance below its demand is stopped, at the
instant its service reaches the allowance, by a `preempt` step whose
`source_transition` is null (**budget exhaustion**); its delay is the time
since the previous step, as for every step. The job stays live and its deadline clock continues, but it is
not dispatched or resumed while its service is not below its allowance: a job
is eligible for dispatch exactly when it is ready and its service is below its
allowance. The product continues all other enabled source and scheduler
behavior, and the job misses at its deadline if it is still incomplete. A
later deadline miss refutes only with the replay evidence below.

**Budget change.** Immediately after the entry step, and after every
activation carried by the same source transition, the product emits one
`budget-change` for each `HI` job live in the entry step's pre-state, whether
pending jitter, ready, running, preempted or exhausted, and whether or not it
has already missed. They are ordered by ascending canonical task name in
Unicode scalar value order, then canonical source activation transition
identity, then occurrence (the EDF tie tuple without the deadline). Each has
delay zero, the entry step's source transition as its `source_transition`,
and the exact `budget_delta = C(HI) - C(LO)` of its task, which is zero when
the budgets are equal. It moves that job's phase from `lo` to `hi`, and it
preserves `d`, remaining actual work, accumulated service, job identity,
optional declared priority, absolute deadline, slot and generation. After it
an exhausted job is eligible again. The budget changes are urgent and atomic
with the entry: while one is pending (the product is in `HI` and a live `HI`
job is in phase `lo`), no delay, source step or other scheduler step is
enabled, and a deadline miss due at the same instant fires after them. Mode
entry adds and removes no job, `LO` jobs keep running in `HI`, and the
dispatch policy does not depend on the mode. The classic AMC switch at the
instant of overrun is the interleaving in which a bound operation enabled at
that instant is taken before the exhaustion `preempt`; a source guard that
observes exhaustion is not part of the source language.

Completion occurs only after the
selected demand `d` units of scheduled service; a budget increase never adds
work or resets service. Variable-demand and mode-change products use sound
bounded exploration with the exact rational demands and allowance deltas
selected on each explored path. An exact demand selected from a declared
interval is an admitted behavior and may support a refutation only when its
timed witness replays; exploring selected representatives never proves that
omitted demands are safe. A finite negative search can settle `proved`
only when its certificate checks an exact abstraction covering every admitted
demand, mode change and scheduler successor; sampled or endpoint-only paths
never establish absence. When an unresolved continuous successor prevents
expansion before a configured bound is reached, the analysis settles
`incomplete` with cause
`bounded-search-incomplete{reason: unresolved-dynamics}` and the replayable
FR-331 task-search frontier naming the exact prefix and open dimensions. When
the configured task-product work bound or another actual search limit stops
expansion, the analysis settles `incomplete`, `limit-reached`, with that
reached limit and the replayable frontier. A live-job or state cap used only to bound exploration is not a
task-queue overflow in the modeled system and cannot refute schedulability.
A concrete miss is a refutation only after FR-364 replays each exact rational
demand, delay, derived scheduler step, source mode transition, allowance update
and missed-deadline state, and replays the counterexample's carried FR-364
continuation (finite loop, terminal-stutter loop or `queue_growth`). The
producer finds that continuation from the FR-418 closed graph or by forward
exploration, and in both cases records it on the wire. Replay does not settle
`refuted` from an exploration of its own. The fixed-work finite-region
decision procedure is not claimed for interval demand with completion
feedback and preemption; it is undecidable in general (Fersman et al.,
Theorem 5). A budget change has the same rule: unless an exact finite
abstraction covers its demand updates, bounded exploration may refute by an
exact miss witness but cannot prove by a finite negative prefix. A
limit-stopped result names the actual stopped limit and value; an unresolved
continuous choice names `bounded-search-incomplete` and no limit. Both carry
the replayable frontier. It never turns a bounded negative search into `proved`
or an undecidable class into a permanent refusal. A time-locking product state
is reported separately by FR-417 and is not a deadline miss unless the claim's
time-divergent scope is witnessed.

The paper's fixed-work proof assumes that the underlying timed automaton has
no time-stops. This product does not inherit that assumption automatically:
FR-416 limits the claim to time-divergent behaviours and FR-417 reports
time-locks. The exact fixed-work path is selected only after the verifier
checks closure of the finite product described above. Every deadline-miss
counterexample is an FR-364 model-arm `TemporalCounterexample`: its prefix
carries the exact miss step, with each step typed as a source transition or
`TaskTransitionIdentity`, and its time-divergent continuation is a replayed
finite loop or `queue_growth` witness. FR-364 replays the miss and continuation
before the result settles `refuted`. A finite-slot cap is only an internal
search signal; it is
never the counterexample endpoint. A time-lock-only path or slot-cap signal
without a replayed deadline miss and checked continuation is not a refutation.

For fixed work with a verified closed finite product, settlement distinguishes
three cases: a reachable miss with a checked finite loop or FR-364
`queue_growth` witness settles `refuted`; a miss whose continuations are
time-locked only is reported through FR-417 and does not refute; and no miss on
any time-divergent behavior settles `proved`. If the declared exact-mode cap
overflows, the certificate does not close and that run uses bounded
exploration; it may settle `incomplete` with replayable FR-331 frontier
evidence. The exact finite graph constructs checked continuation witnesses
for its refutations. A fixed-work task automaton for which finite product
closure cannot be established remains admitted on the bounded-search route.
A queue-growth induction can replay one unbounded continuation but does not
establish completeness for the fixed-work class.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-419-AC-1 | Task set `Ctl` with `(C, T = D)` of `(1, 4)`, `(2, 6)`, `(5, 12)` ms under fixed priority settles `proved` with response times `1`, `3`, `12` and the third task's fixpoint sequence `5, 9, 12, 12`; under EDF it settles `proved` at utilization exactly 1. | Test |
| FR-419-AC-2 | With `C = 6` for the third task, fixed priority settles `refuted` with the third task, job 0 and iterates 6, 10, 13 (response time 13 above 12); EDF settles `refuted` with utilization `13/12`. | Test |
| FR-419-AC-3 | An AMC set whose high-mode criterion fails settles `inconclusive`, `sufficient-test-failed`; a low-mode miss settles `refuted`. | Test |
| FR-419-AC-4 | `check_closed_form` recomputes the evidence before settlement, and altered evidence settles `inconclusive`, `replay-parity`. | Test |
| FR-419-AC-5 | Changing one stated premise changes the obligation identity; an invalid task set refuses `invalid_timed_model`/`invalid-task-set` naming the task; a `fixed_priority`, `amc` or `response` claim over a task with no `priority`, and a `fixed_priority`, `amc` or `response` claim over two tasks of equal priority, refuse `invalid_timed_model`/`invalid-task-set` naming that task (both tasks for the clash), and an `edf` claim over the same sets is admitted; a non-positive `D` and a `LO` task with dual budgets refuse the same way. | Test |
| FR-419-AC-6 | A fixed-work task automaton lowers to the exact finite bounded-subtraction product and a TT-1 claim only when FR-418 verifies a closed finite product; a task product with no reachable checked release binding may also be proved by the FR-418 no-release closure; interval demand with completion feedback and preemption is searched soundly, and an unresolved continuous successor or an exhausted task-product work bound settles `incomplete` with replayable FR-331 task-search evidence and respectively `bounded-search-incomplete{reason: unresolved-dynamics}` or `limit-reached`, never `proved` or `unsupported`. A negative deadline-miss result carries an FR-364 model-arm `TemporalCounterexample` whose prefix identifies each step as a source transition or typed `TaskTransitionIdentity`, and FR-364 replays the exact miss and a checked time-divergent continuation before `refuted`. | Test (TC-372) |
| FR-419-AC-7 | Tasks `a` (`C = 1`, `T = D = 4`, priority 1) and `b` (`C = 2`, `T = D = 6`, jitter 1, blocking 3, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterates 5, 7 (response time `7 + 1 = 8` above 6). | Test |
| FR-419-AC-8 | Tasks `a` (`C = 1`, `T = D = 2`, priority 1) and `b` (`C = 1`, `T = 3`, `D = 1`, blocking 1, priority 2) under fixed priority settle `refuted` with task `b`, job 0 and iterate 2. Evidence naming `b`'s job 2 with iterates 4, 6, 7, 8 settles `inconclusive`, `replay-parity`, because job 1's completion 6 does not exceed `2 · 3`. | Test |
| FR-419-AC-9 | `Ctl`'s `response T3 on Ctl.t3 <= 11 ms under fixed_priority` (`t3` the third task) settles `refuted` with the third task, job 0 and iterates 5, 9, 12 (the bound is the claim's `d`, 11); `<= 12 ms` settles `proved`. | Test |
| FR-419-AC-10 | Each of these parses and refuses at check time with `invalid_timed_model`/`invalid-task-set`, never `invalid_syntax`: `taskset Empty { }` (naming `Empty`); a task with `bcet -1 ms`; a task with `bcet 2 ms` and `wcet 1 ms`; a `hi` task with `bcet 4 ms`, `wcet 2 ms`, `wcet_hi 3 ms`; a task with `jitter -1 ms`; a task with `blocking -1 ms`; a `lo` task with `wcet_hi 2 ms`; a task with `blocking 1 ms` under `edf`; a task with `jitter 1 ms` under `edf`; a task with `deadline 5 ms` and `periodic 4 ms` under `amc` (each naming the task). A task set with `t1`'s `jitter -1 ms` and `t2`'s `blocking -1 ms`, under `edf`, reports `t1`'s and `t2`'s task-set defects in that order, then the claim's two `edf` defects in the same order. | Test |
| FR-419-AC-11 | An EDF task may omit priority and its task-transition identity omits the member; EDF selects the earliest absolute deadline and resolves ties by canonical task name in Unicode scalar value order, source activation transition identity and occurrence. Source declaration order and fixed priority do not affect EDF tie order. A declared priority, if present, is carried and checked but does not change EDF order. A forged priority, a priority on a task that declares none, or an EDF ready order inconsistent with deadlines/ties is refused; fixed-priority task identities require and replay the checked declared priority. | Test (TC-372) |
| FR-419-AC-12 | Replaying a fixed-work task-product certificate template across a periodic cycle instantiates successive absolute activation occurrences without renumbering any live job; after completion, reusing its finite rank/slot preserves the identities of all other live jobs and gives the new job the next exact prefix occurrence. Replayed deadline-miss evidence carries those exact occurrences and generations, and a template whose source transition, live-job rank or slot does not match the recomputed path is rejected. | Test (TC-371) |
| FR-419-AC-13 | `Ctl` with the third WCET 6 under `fixed_priority` is refuted with the counterexample `{"type": "schedulability-miss", "claim": Q, "analysis": "fixed-priority", "unit": "ms", "miss": {"kind": "response", "task": "c", "job": "0", "iterates": [6, 10, 13], "response": 13, "bound": 12}}`, each number an FR-181 rational with denominator 1; under `edf` with `{"kind": "utilization", "utilization": 13/12}`. The reader refuses the first with `miss.kind` `late`, with `iterates` empty, with `mode` under `fixed-priority`, and with `utilization` written `26/24`, each `invalid_result`/`invalid-value`, and with an extra member `invalid_result`/`unknown-member`, each naming the member. | Test (TC-2006) |
| FR-419-AC-14 | Model `C` (`time dense`; one object with `hi: Bool` initially false and a clock `y` never reset; fields `h_on` and `l_on`, initially false; operations `fire_h` (post `self.h_on = true`) and `fire_l` (post `self.l_on = true`), each always enabled, and `go_hi` guarded `not self.hi and y >= 12 ms` with post `self.hi = true`; `time invariant when not self.hi { y <= 12 ms }`, and `time invariant when not (self.h_on and self.l_on) { y <= 0 ms }`, which forces both first releases at 0, so `h` and `l` activate at 0, 10, 20, …) and task set `MC` (`h` periodic 10 ms, deadline 10 ms, `bcet 3 ms`, `wcet 2 ms`, priority 1, `hi wcet_hi 3 ms`; `l` periodic 10 ms, deadline 10 ms, `bcet` = `wcet` = 4 ms, priority 2, `lo`) with `release MC.h on C::fire_h;`, `release MC.l on C::fire_l;` and `mode MC hi on C::go_hi;`, under `fixed_priority`: on every path `h` job 0 is stopped at 2, when its service reaches 2, by a `preempt` with null `source_transition`, is not dispatched or resumed before 12, and misses at 10. The `go_hi` step at 12 is followed at once by `budget-change` for `h` job 0, then `h` job 1, each with `budget_delta` 1, delay 0 and `source_transition` `go_hi`, before any other step. `h` job 2 (activated at 20) has none. With `12 ms` replaced by `2 ms` in the guard and the invariant there is no miss. Without the `mode` declaration there is no `budget-change` on any path and no `h` job completes. | Test (TC-372) |
| FR-419-AC-15 | Over `MC` with `h`'s `bcet 2 ms`, on the path that selects `d = 2` for `h` job 0, that job completes at 2 with no exhaustion `preempt` and no `budget-change`. Over `MC` with a third task `e` (periodic 10 ms, deadline 10 ms, `bcet` = `wcet` = 1 ms, priority 3, `hi wcet_hi 1 ms`), released by `release MC.e on C::fire_e;` over an always-enabled `fire_e` whose post sets `self.e_on = true` (with `self.e_on` added to the start invariant, so `e` also activates at 0), and `go_hi` at 2, entry at 2 emits `budget-change` for `e` job 0 with `budget_delta` 0, before the one for `h` job 0 (`e` precedes `h` by task name). With `go_hi` at 10 ms and `release MC.h on C::go_hi;` added beside `mode MC hi on C::go_hi;`, on the path where `go_hi` is the activation of `h` job 1 at 10 (exactly one period after job 0), that job starts in phase `hi` and has no `budget-change`, while the live `h` job 0 receives one. A variant whose `go_hi` has no `not self.hi` guard and fires again in `HI` emits no `budget-change` and no mode change at its second step. A delay that would carry a phase-`lo` job's service past `C(LO)`, and a `dispatch` of an exhausted job, are not product successors. | Test (TC-372) |

## Dependencies

- Consumer facts ([FR-661](ix://agent-ix/filament-specification/FR-661)): `CF-IR-03` (the schedulability claim roots) and `CF-IR-04` (the obligation identity of a schedulability claim occurrence).
- [FR-142](../type-model/FR-142-evaluate-quantities-and-units.md) supplies
  exact time quantities.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) carries the
  kind `schedulability`.
- [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) supply the
  task-automaton lowering target and its time-divergent claim scope.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) defines the
  task-product state and transition identity; [FR-364](FR-364-replay-temporal-counterexamples.md)
  replays task-miss witnesses; [FR-418](../../objects/interfaces/FR-418-zone-certificate.md)
  checks fixed-work refined-region certificates; [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries bounded-search evidence.

## References

- FSL ADR-026 RT-1 to RT-8, OV-10.
- Owner ruling recorded on QSL-373: native closed-form schedulability; WCET is
  a stated premise. The later QSL-585 task-automaton contract supersedes its
  unsupported disposition for variable-demand and mode-change products: those
  products use sound bounded exploration and may settle `incomplete`.
- M. Joseph and P. Pandya, 1986; N. Audsley et al., 1993; J. Lehoczky, 1990;
  S. Baruah, L. Rosier and R. Howell, 1990; F. Zhang and A. Burns, 2009;
  S. Baruah, A. Burns and R. Davis, 2011; E. Fersman et al., 2007.
- Ticket: STD-139 (paired with QSL-373).
- E. Fersman, P. Krčál, P. Pettersson and W. Yi, “Task Automata: Schedulability,
  Decidability and Undecidability,” *Information and Computation* 205(8), 2007,
  Theorems 2 and 5, DOI 10.1016/j.ic.2007.01.009. The fixed-work result is
  design evidence for the exact class above; Theorem 5 explains why interval
  demand with completion feedback and preemption may require an incomplete
  analysis and does not justify refusal.
