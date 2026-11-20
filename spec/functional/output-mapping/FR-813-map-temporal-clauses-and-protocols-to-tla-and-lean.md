---
id: FR-813
title: "Map temporal clauses, fairness, protocol subjects and dense time to TLA+ and Lean 4"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/FR-651
    type: specifies
  - target: ix://agent-ix/filament-specification/FR-812
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-425
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-431
    type: references
  - target: ix://agent-ix/filament-specification/FR-430
    type: references
  - target: ix://agent-ix/filament-specification/FR-464
    type: references
  - target: ix://agent-ix/filament-specification/FR-092
    type: references
  - target: ix://agent-ix/filament-specification/FR-361
    type: references
  - target: ix://agent-ix/filament-specification/FR-415
    type: references
  - target: ix://agent-ix/filament-specification/FR-416
    type: references
  - target: ix://agent-ix/filament-specification/FR-377
    type: references
---

# FR-813: Map temporal clauses, fairness, protocol subjects and dense time to TLA+ and Lean 4

## Description

When the TLA+ or Lean 4 mapper exports a temporal clause, a fairness
constraint, a clause over a protocol subject or a clause over a timed
subject, it SHALL use the encodings below, which state the clause's FSL
meaning over the exported behaviours exactly, so that the untimed forms are
`preserved` in both targets, the dense-time forms are `preserved` in Lean 4,
and the dense-time forms in TLA+ are `conditional` on the one named condition
this requirement defines.

## Inputs

The obligation's temporal, fairness, protocol and timed-model nodes; the
state, actions and values of FR-812; and the export request's `initial`
snapshot when one is named (FR-817).

## Outputs

The temporal, fairness, protocol, timed and theorem regions.

## Behavior

### Temporal clauses in TLA+

- A clause built only from `always` and `eventually` over state formulas
  SHALL be emitted directly with `[]` and `<>`, nested as the clause nests
  them: `always eventually P` is `[]<>P` and `eventually always P` is `<>[]P`.
  No monitor or auxiliary variable is added for such a clause. TLA+ states
  these formulas exactly, so a monitor would add text without adding meaning.
- Every other clause (`until`, `release`, past operators `once`,
  `historically`, `since`, `triggered`, interval operators under a
  step-counting profile) SHALL be emitted as its clause monitor composed with
  the specification: the monitor's states become auxiliary variables `m_<clause>`
  updated in every `Next` disjunct, a step counter `n_<clause>` ranges over
  `0..b` with `b` the interval's upper bound and saturates there, past operators become history variables
  ("Past operators in TLA+" below), and the clause becomes the acceptance
  condition over the monitor. The monitor SHALL be deterministic wherever the
  clause has a deterministic safety, Büchi or co-Büchi monitor, with
  acceptance `[]~reject`, `[]<>accept` or `<>[]accept` respectively; a
  deterministic monitor reads each behaviour along exactly one run, so the
  encoding is exact and the record `preserved`. Auxiliary variables add no
  behaviour to the original variables.
- A clause that has no deterministic Büchi or co-Büchi monitor SHALL be
  emitted with a nondeterministic Büchi monitor, whose acceptance holds when
  some run accepts; TLC and TLAPS read the composed specification over
  every run of the monitor variable, so the record SHALL be `conditional`
  with the condition `nondeterministic-monitor{clause, node}` named in the
  outcome, and never `preserved`. The Lean encoding of the same clause is
  the direct definition below and stays `preserved`.
- Step-sensitive meaning (intervals counted in steps, past operators read at
  step offsets) SHALL be carried by the monitor, whose bounded step counters
  and history variables change only in steps of the subject and stay
  unchanged by stuttering, so TLA+'s finite stuttering does not shift a step
  count; no unbounded counter is added, so a finite model stays finite.
- FSL's terminal stutter is a step (FR-377, FR-431 "Counted steps"), so a
  monitor SHALL advance at a terminal state as at any other position. The
  composed step relation is therefore `Next_<clause> == (Next \/ (~ENABLED
  Next /\ UNCHANGED vars)) /\ Mon_<clause>`: its second disjunct is the
  terminal stutter, which changes no original variable and advances the
  monitor until the monitor reaches a fixed point, after which the step
  changes no variable of `vars_<clause>` and is a TLA+ stutter. The
  composed specification SHALL be `Spec_<clause> == Init_<clause> /\
  [][Next_<clause>]_vars_<clause> /\ WF_vars_<clause>(Next_<clause>)`, so
  the terminal step is taken while it changes a monitor variable: step
  counters, history variables and shift registers advance at a deadlock as
  FSL's positions do. Lean's `Behaviour` already repeats the terminal state
  at every later position (FR-812), so both targets read the same positions.
- The obligation's statement SHALL be `THEOREM Th_<clause> == Spec =>
  <formula>`, with `Spec_<clause>` in place of `Spec` when the clause adds a
  monitor or fairness, written without a proof.

### Quantified clauses in TLA+

A temporal formula SHALL NOT be quantified over a set that depends on the
state: TLA+ evaluates `\A p \in DOMAIN v_T : F` at the first state only for
the set but would read a different set at every state a temporal operator
reaches. The `over` binder of a clause is bound at the clause's origin
(FR-361 "Binding of the clause's `over` parameter": one instance per key
admitted at an initial state), so the export SHALL quantify over a constant
identity universe and guard each identity at the origin:

- For each object type `T` that an `over` binder ranges over, the module
  SHALL define the constant `Ids_<T>`, the universe of keys of `T`'s
  population. When the request names an `initial` snapshot, `Ids_<T>` is the
  set of the keys of `T`'s population in that snapshot, written as a set
  literal of FR-812 keys in ascending key order (`Ids_Counter == {<<99>>}`).
  When it names none, every well-typed state is initial (FR-812 `Init ==
  TypeOK`), so `Ids_<T> == Seq(Nat)`, the key space `TypeOK` gives `DOMAIN
  v_<T>`; and when some operation of the model can add a key to `T`'s
  population, `Ids_<T> == Seq(Nat)` whether or not a snapshot is named.
  `Ids_<T>` therefore holds every key that can be present in any reachable
  state, and so every key that can be an instance of a binder or a receiver.
- `Ids_<T>` SHALL replace `DOMAIN v_<T>` in every temporal-level range: the
  theorem's quantifier, the domain of every monitor and history function,
  the acceptance predicate (`accept_<clause>(p_c) == m_<clause>[p_c] =
  "acc"`, read per identity) and the receiver range of an `each` fairness
  constraint (`\A p_self \in Ids_<T> : WF_vars(op_<op>(p_self))`). An
  action's own first conjunct is its receiver's presence `r_self \in DOMAIN
  v_<T>` (FR-812), so an action over an absent identity is not enabled and
  its fairness conjunct is satisfied vacuously. `DOMAIN v_<T>` stays the
  range of FR-812's `Next` and of a state clause, which read one state.
- A clause with `over` binders `(p1: T1, …)` SHALL be stated as `Spec =>
  \A p_p1 \in Ids_<T1>, … : (p_p1 \in DOMAIN v_<T1> /\ …) => F`. The
  antecedent stands outside every temporal operator, so TLA+ reads it in the
  initial state: an identity is an instance exactly when its object is
  present at the origin, which is FSL's admitted key.
- A binder over a type that is not an object type ranges over its FR-812
  value set, which does not depend on the state, and carries no antecedent.
- Each monitor and history variable of a quantified clause SHALL be a
  function on `Ids_<T>` (on the product of the universes for several
  binders), and the acceptance condition SHALL be read per identity inside
  the quantifier: `Spec_<clause> => \A p_c \in Ids_<T> : (p_c \in DOMAIN
  v_<T>) => []<>accept_<clause>(p_c)`.
- An atom that reads the object of a binder SHALL carry the binder's
  presence `p_c \in DOMAIN v_<T>` as the first conjunct of its definedness
  condition (FR-812 "Arithmetic and undefinedness"): `(p_c \in DOMAIN
  v_Counter /\ v_Counter[p_c].fld_value <= 5)`. An object absent at a later
  position then makes the atom false, as an undefined read refutes the FSL
  clause, and a monitor indexed by an identity whose object is absent never
  applies `v_<T>` outside its domain.

The Lean encoding already quantifies at the origin (`forall p_c,
F_has_<T> p_c (b.st 0).v_<T> -> holds_<clause> b p_c 0`) and is unchanged.

### Past operators in TLA+

FR-092 reads a past operator over the positions back to the origin and,
for a closed interval, over a pre-origin extension at which every atom is
false and every Boolean constant keeps its value. The TLA+ encoding SHALL
state exactly that reading with bounded history variables. The *pre-origin
value* `pre(φ)` of a past-only operand `φ` is the Boolean constant obtained
by evaluating `φ` with every atom false, recursively through its Boolean and
past operators; the mapper computes it and writes it as `TRUE` or `FALSE`.
An operand that holds a future operator is not past-only, and its clause
falls under the monitor rules above.

History variables are numbered by the clause's past operators in pre-order
(outer before inner, left before right), from 0, one ordinal per operator,
and named `m_<clause>_h<n>` (an unbounded operator's Boolean), `m_<clause>_r<n>`
(a shift register) and `m_<clause>_c<n>` (a saturating counter); `since` and
`triggered` registers take the operator's ordinal for the right operand `q`
and the ordinal plus one, which no other operator then takes, for the left
operand `p`. Over a quantified clause each is a function on `Ids_<T>` as
above; the text below writes the unquantified form. `φ` is the operand's
text over unprimed variables and `φ'` the same text over primed variables.

**Origin of each form.** Only a closed interval `[a,b]` reads positions
before the origin, through the pre-origin extension. An unbounded operator
and a lower-bounded `[a,*]` operator range over positions `0` through `i -
a` only (FR-092 "Positions before the origin", FR-367) and never read a
position before the origin, whatever the operand. Their history therefore
starts from the empty-range value: `once` and `since` false, `historically`
and `triggered` true, before position 0 contributes.

**Unbounded operators.** Each SHALL be one Boolean `m_<clause>_h<n>`
updated from the empty-range value: `once φ` has `Init` `φ` and `Mon`
`φ' \/ m_<clause>_h<n>`; `historically φ` has `Init` `φ` and `Mon` `φ' /\
m_<clause>_h<n>`; `p since q` has `Init` `q` and `Mon` `q' \/ (p' /\
m_<clause>_h<n>)`; `p triggered q` has `Init` `q` and `Mon` `q' /\ (p' \/
m_<clause>_h<n>)`. Each `Init` is the update applied to the empty-range
value at position 0.

**Closed interval `[a,b]`, `b >= 1`.** The operand's truth at the `b`
previous positions SHALL be held in a shift register of length `b`:

- `TypeOK_<clause>` conjunct: `m_<clause>_r<n> \in [1..b -> BOOLEAN]`.
- `Init_<clause>` conjunct: `m_<clause>_r<n> = [k \in 1..b |-> pre(φ)]`, so
  every cell the origin's history does not reach reads the pre-origin
  extension.
- `Mon_<clause>` conjunct: `m_<clause>_r<n>' = [k \in 1..b |-> IF k = 1 THEN
  φ ELSE m_<clause>_r<n>[k - 1]]`: cell `k` holds the operand's truth `k`
  positions back, and the step shifts the current truth into cell 1.
- Reading, with `R(0)` the operand's current text `φ` and `R(k)` the cell
  `m_<clause>_r<n>[k]` for `k >= 1`:
  - `once[a,b] φ` is `\E k \in a..b : R(k)` and `historically[a,b] φ` is
    `\A k \in a..b : R(k)`, each written with `R(0)` as a separate disjunct
    or conjunct when `a = 0`: `φ \/ \E k \in 1..b : m_<clause>_r<n>[k]`;
  - `p since[a,b] q` is `\E d \in a..b : Q(d) /\ \A e \in a..(d - 1) : P(e)`
    and `p triggered[a,b] q` is `\A d \in a..b : Q(d) \/ \E e \in a..(d - 1)
    : P(e)`, with `Q` reading `q`'s register and `P` reading `p`'s.

An interval `[0,0]` reads the operand's current text and adds no variable.

**Lower-bounded interval `[a,*]`.** FR-092 reads these operators over
positions `0` through `i - a` only, with no pre-origin extension.

- `once[a,*] φ` holds exactly when `φ` first held at least `a` steps ago,
  which a saturating counter of the steps since `φ` first held decides:
  `TypeOK_<clause>` conjunct `m_<clause>_c<n> \in -1..a`; `Init_<clause>`
  conjunct `m_<clause>_c<n> = IF φ THEN 0 ELSE -1`; `Mon_<clause>`
  conjunct `m_<clause>_c<n>' = IF m_<clause>_c<n> = -1 THEN (IF φ' THEN 0
  ELSE -1) ELSE IF m_<clause>_c<n> < a THEN m_<clause>_c<n> + 1 ELSE a`;
  reading `m_<clause>_c<n> = a`. The value `-1` stands for "has not held".
- `historically[a,*] φ` is `~once[a,*] ~φ`: the same counter over `~φ`,
  read as `m_<clause>_c<n> /= a`.
- `p since[a,*] q` holds exactly when the unbounded `p since q` held `a`
  positions back. It SHALL be emitted as the unbounded history Boolean
  `m_<clause>_h<n>` of `p since q` (`Init`: `q`; `Mon`: `q' \/ (p' /\
  m_<clause>_h<n>)`) read through a shift register of length `a` over that
  Boolean, both numbered by the operator's ordinal, whose `Init` cells are
  `FALSE` because no position before the origin is read; with `a = 0` the
  Boolean is read directly. `p triggered[a,*] q` is the same over the
  history Boolean of `p triggered q` (`Init`: `q`; `Mon`: `q' /\ (p' \/
  m_<clause>_h<n>)`) with `Init` cells `TRUE`.

Every register, counter and history variable is bounded, changes only in
`Mon_<clause>` and is unchanged by a TLA+ stutter, so a finite model stays
finite and stuttering shifts no offset.

### Temporal clauses in Lean 4

Every clause SHALL be emitted as a `def` over a `Behaviour` (FR-812) and a
position, by direct recursion on the clause: `always p` at `i` as `∀ j ≥ i,
p b j`; `p until q` as `∃ k ≥ i, q b k ∧ ∀ j, i ≤ j → j < k → p b j`; past
operators over positions `≤ i`; intervals by bounds on `j - i` in steps.
A closed past interval reads a position before 0 as the pre-origin
extension, each atom conjoined with `0 <= j` and so false there. An
unbounded or `[a,*]` past operator bounds its offsets by `0 <= i - d`
instead: `historically φ` at `i` is `forall d : Int, 0 <= d -> 0 <= i - d
-> φ (i - d)`, true when the range is empty, and `once φ` is `exists d :
Int, 0 <= d /\ 0 <= i - d /\ φ (i - d)`, so no position before the origin
is read.
The obligation's statement SHALL be a `def Ob_<clause> : Prop := ∀ b :
Behaviour, holds_<clause> b 0`. The file states obligations as propositions
and contains no `theorem`, no proof and no `sorry`.

### Fairness and terminal states

- Weak and strong fairness of an operation SHALL be `WF_vars(op_<op>)` and
  `SF_vars(op_<op>)` (with existential argument choice) in TLA+, and the
  corresponding `∀ i, (∃ j ≥ i, …)` premises in Lean.
- A terminal state stutters forever in both targets (FR-812 `Behaviour`),
  which is FSL's terminal stutter (FR-377).

### Clauses over supplied traces

A clause over a supplied, finite trace (observation and finite-window
clauses) SHALL be emitted as an operator (TLA+) or a `def` (Lean) over a
finite sequence of observation records, applied at the clause's activation
positions; it has no `Spec`.

### Protocol subjects

A clause over a protocol subject SHALL be exported over the FR-425 protocol
subject's own transition system: the export's state is FR-425's protocol
state, its steps are FR-426's steps, and it adds no step, state component or
reading of its own. The model's operations run only as the protocol's
attempts (FR-425 "Protocol subject"), so the subject's step relation
replaces FR-812's `Next` and `Step`.

**Binding.** A temporal clause SHALL be exported over the subject of a
protocol `P` exactly when `P`'s activation relation (FR-464 "Activation
relation", which filament-spec-language FR-449 derives from the
`protocol_clause` node) holds a row requiring that clause. The subject is the
one an analyze request selects with the protocol binding `{protocol: P, over,
roles, capacities}` (FR-520), with these bindings: the protocol's `over`
parameter ranges over `Ids_<T>` of its type, guarded at the origin as a
clause binder is ("Quantified clauses in TLA+"), held in the rigid variable
`pr_<P>_over`; a role on a whole model is bound by construction (FR-425); a
symbolic capacity is a TLA+ `CONSTANT` with its declared constraint as an
`ASSUME`, and a Lean `variable` with the constraint as a hypothesis of
`Ob_<clause>`. An `origin` row evaluates the clause at position 0 of every
behaviour of the subject. An `each` row evaluates it at the position after
each `activate` step of `P` (FR-428): the TLA+ statement is
`[](pr_<P>_last = "activate" => F)` and the Lean statement quantifies over
the positions whose step is an `activate` step. A clause that rows of
several protocols require is stated once per row, in the order of the rows,
and a clause that no row requires is exported over its model subject. A
clause's positions, atoms and counted steps over the subject are FR-431's:
position 0 is the initial protocol state, every step is a position, a
`holds` atom reads the model observation, and an interval counts only
counted steps.

**State components.** Each member of FR-425's state key SHALL be one TLA+
variable and one field of the Lean structure `PState_<P>`:

| FR-425 member | Content | TLA+ | Lean field |
| --- | --- | --- | --- |
| `semantic` | the model state | FR-812's population variables `v_<T>` | `st : State` |
| `control` | instance entries with their finish state (`finished`), thread entries with their rest point (control position) and disposition, parallel instances, registrations, commit entries | `pr_<P>_control`, a function from the present entry keys to entry values | `control : List (Prod J J)` |
| `observations` | binder values per (binder node, thread path), attempt records | `pr_<P>_observations` | `observations : List (Prod J J)` |
| `bounds` | repeat counters, attempt counts, pending sends, message ordinals, business-effect ordinals, next delivery indices | `pr_<P>_bounds` | `bounds : List (Prod J J)` |
| `queues` | channel contents per (instance, channel, FIFO key) | `pr_<P>_queues` | `queues : List (Prod J J)` |
| `roles` | static role bindings and replicated role instance maps | `pr_<P>_roles` | `roles : List (Prod J J)` |
| `memory` | the memory component (empty under `sc`) | `pr_<P>_memory` | `memory : List (Prod J J)` |

- An entry key and an entry value SHALL be FR-425's typed JSON object
  written as a TLA+ record with the same member names (`[type |->
  "thread-path", instance |-> 0, steps |-> <<>>]`), a JSON array as a
  sequence, a payload or binder value as its FR-812 value, a `"<decimal>"`
  member as the natural number it spells, and JSON `null` as the generated
  constant `PK_Null == [null |-> TRUE]`, which equals no other value.
- A queue SHALL be a sequence of entries for a `fifo` channel and a set of
  entries for an `unordered` channel: every entry of a queue has its own
  (message, delivery) identity (FR-425), so FR-812's bag of a queue has
  every count 1 and is that set. A queue holding no entry has no key in
  `pr_<P>_queues`.
- In Lean, `J` is the generated inductive type of FR-181's canonical JSON
  values (`null`, `bool`, `num` over `Int`, `str`, `arr`, `obj` over
  `List (Prod String J)`), a `"<decimal>"` member is `J.str` of its digits,
  and each `List (Prod J J)` is sorted by FR-181's key order with distinct
  keys, a `PState_<P>` well-formedness conjunct. Two exported states are
  therefore equal exactly when their FR-425 state keys are equal, in both
  targets, so the export neither merges states FSL keeps apart nor splits
  states FSL merges.
- Two auxiliary variables carry what the clause reads and add no behaviour:
  `pr_<P>_over` (the `over` binding, unchanged by every step) and
  `pr_<P>_last`, the FR-426 step kind of the step that reached the state
  (`"init"` in an initial state), which FR-431's counted steps and an `each`
  row read. Lean's `PState_<P>` holds them as `over` and `last`.

**Initial states.** `PInit_<P>` SHALL state FR-425 "Initial states" over
FR-812's `Init`: for an `on origin` subject, instance 0 with `finished`
false, its root thread settled from the start of `run`, empty queues, no
binders and the static role bindings; for an `on each` subject, no
instance.

**Steps.** Each FR-426 step kind SHALL be one action schema, named `s_<P>_`
followed by the mangled static declaration path of its node (or of its
channel, template or role), and `PNext_<P>` SHALL be the disjunction of every
instance of every schema, each quantified over its owning thread `t \in
DOMAIN pr_<P>_control` and its record, payload or entry, so that the threads
of a `parallel` interleave. Each schema states the step's enabledness and
effect exactly as FR-426 states them and settles the moved threads by
FR-426 "Settling":

| FR-426 step kind | Action schema |
| --- | --- |
| `attempt` | `s_<P>_<node>(t, r_self, p_…)`: `t` rests at the node; FR-812's `op_<op>(r_self, p_…)` over the model variables restricted to the contract clauses the node selects; the binder record satisfies the node's constraint; the gate of the memory model holds |
| `recommend` | as `attempt`, with every model variable unchanged |
| `event` | `s_<P>_<node>(t, w_<binder>)`: a record of the node's type satisfying its constraint; for an `effect`, its attempt's binder entry present for `t` |
| `send` | `s_<P>_<node>(t, w_<binder>)`: a payload of the channel's `carries` type satisfying the send constraint; the queue updated by the channel's overflow policy (below); the message ordinal and next delivery index advanced as FR-425 states; on a delivery count interval channel, one schema instance per count `d` with FR-426 "Interval sends" |
| `receive` | `s_<P>_<node>(t, q)`: a deliverable entry of queue `q` (the head of the sequence for `fifo`, any member for `unordered`) from the send node the receive names, whose payload satisfies the receive constraint, the guard predicate, read with the binder bound to the payload; the entry removed |
| `duplicate`, `lose` | `s_<P>_<channel>_duplicate(q, e)` and `s_<P>_<channel>_lose(q, e)`, present only under the delivery policies FR-426 admits them for |
| `fork`, `join` | `s_<P>_<parallel>_fork(t)` creating the parallel instance and its branch threads at the smallest free ordinal, and `s_<P>_<parallel>_join(t)` testing the join policy over the dispositions |
| `timeout` | `s_<P>_<await>_timeout(t)` |
| `finish` | `s_<P>_<finish>(t, w_<binder>)`: every other thread of the instance has left `running`; the instance entry's `finished` becomes true and the root rests at end |
| `fence` | `s_<P>_<node>(t)` with the memory model's fence rule |
| `cattempt`, `cfail`, `ctimeout`, `cend`, `cexpire` | one schema each per compensation template, as FR-427 states them |
| `activate`, `spawn`, `retire` | one schema each, as FR-428 states them |
| `memory` | one schema per memory step of the resolved memory model (FR-434) |

- A `choice`, a `repeat`, a `check`, a `sequence` and a branch end are
  structural moves, not steps (FR-426 "Settling"), and SHALL have no action
  of their own: the action of the step after which a thread meets one sets
  the thread's next rest point, repeat counter and binder entries to what
  settling gives, written as a `CASE` over the choice's case guards in case
  order, an `IF` over a repeat's guard and counter and an `IF` over a
  check's value, each guard reading the thread's binder entries in
  `pr_<P>_observations`.
- A step SHALL write a binder entry exactly when FR-425's reach rule keeps
  it at the thread's post-step rest point, and SHALL remove the entries the
  rule drops, as the mapper decides from the control graph.
- The overflow policy of a full queue (one holding as many entries as the
  channel's capacity) SHALL give the send's queue update: `block` enables
  the send only below capacity; `reject` leaves the queue unchanged at
  capacity; `loss head` removes the first entry of the sequence and appends
  the new one; `loss tail` removes the last entry and appends the new one;
  `loss new` leaves the queue unchanged. A rejected or lost copy still
  takes its delivery index (FR-425), so the bounds update is the same under
  every policy.
- Each action SHALL set `pr_<P>_last'` to its step kind and leave every
  variable it does not write unchanged.
- The actions SHALL read and write the key maps through the generated
  helpers `PK_Get(f, x, y) == IF x \in DOMAIN f THEN f[x] ELSE y`,
  `PK_Put(f, x, y) == [z \in DOMAIN f \cup {x} |-> IF z = x THEN y ELSE
  f[z]]`, `PK_Del(f, x) == [z \in DOMAIN f \ {x} |-> f[z]]` and `PK_At(n) ==
  [type |-> "thread-state", rest |-> [type |-> "rest", kind |-> "node", node
  |-> n], disposition |-> "running"]`, and remove a key whose entry FR-425
  removes (an emptied queue, a dropped binder, a removed parallel instance)
  with `PK_Del`, so a map's domain is always FR-425's set of present keys.
- `PSpec_<P> == PInit_<P> /\ [][PNext_<P>]_Pvars_<P> /\
  WF_Pvars_<P>(PNext_<P>) /\ <fairness>`, where `<fairness>` holds one
  `WF_Pvars_<P>` or `SF_Pvars_<P>` conjunct per member of the clause's
  resolved fairness set (FR-430: scheduler, authored and memory
  constraints), over the disjunction of the action instances of the
  member's class. Lean's `PBehaviour_<P>` holds the same premises.
- Lean SHALL state the subject as `PInit_<P>`, `PStep_<P>` (the disjunction
  of one `def` per action schema, each a relation on `PState_<P>`) and
  `PBehaviour_<P>` over `Nat -> PState_<P>`, and the obligation as `def
  Ob_<clause> : Prop` over `PBehaviour_<P>`.

**Fidelity.** A protocol subject is never a timed subject (FR-425), so its
export reads no clock, and FR-426's `timeout` step is enabled while its
`await` waits, with no deadline. Every component and step above is FR-425's
and FR-426's own, so a clause over a protocol subject SHALL be `preserved`
in both targets whenever the clause itself is (a nondeterministic monitor
keeps its `conditional` record). A delivery count interval `[lo,hi]` is a
per-send count and reads no clock (FR-465), so it is the interval send
schema above and stays `preserved`.

### Dense time

A clause over a timed subject with a `time dense` source (FR-415), and every
timed interval it reads (FR-416), SHALL be exported as follows. A hybrid
model's flow equations keep FR-651's and FR-811's `unrepresented` handling:
its modes and jumps are lowered, each flow is an uninterpreted relation, and
the record names each flow node by `target-unrepresentable`.

**TLA+ real-time encoding.** The module SHALL extend the standard module
`Reals` and add:

- the variable `now \in Real`, the current time stamp, 0 initially;
- one variable `k_<T>_<f>` per clock-bearing field `f` of object type `T`
  (FR-415 "Clock places"), a function from the present keys of `v_<T>` to
  the field's FR-812 value with each `Clock` position a `Real` in the
  model's time unit `u`, 0 initially at every present place;
- the variable `tick \in BOOLEAN`, `TRUE` exactly in a state reached by a
  delay, `FALSE` initially;
- the action `Tick(d) == /\ d \in Real /\ d > 0 /\ ~Urgent /\ TimeInv(d) /\
  now' = now + d /\ <every live clock place advanced by d> /\ tick' = TRUE
  /\ UNCHANGED <<v_…>>`, where `TimeInv(d)` is the conjunction over the
  model's time invariants of `P => body` with each clock read as its value
  plus `d` (each body is a conjunction of upper bounds, so it holds
  throughout the delay exactly when it holds at its end), and `Urgent` is
  the disjunction of the `urgent when P` predicates and of the enabledness
  of each `urgent O` operation;
- discrete steps: each operation action reads its clock guards at the
  current valuation, writes its resets (`x' = c` at the reset place) and
  keeps every other clock place, with `now' = now` and `tick' = FALSE`, so
  every discrete step is urgent-compatible with delay 0;
- `TNext == Next \/ (\E d \in Real : Tick(d))`, `Spec == Init /\
  [][TNext]_vars /\ WF_vars(TNext)`, with `now`, `tick` and the clock
  variables in `vars`;
- `NZ == \A r \in Real : <>(now > r)`, the time divergence FR-416 reads
  behaviours under, and the statement `THEOREM Th_<clause> == Spec_<clause>
  => (NZ => F)`.

A state reached by `Tick` is an instant between FR-416's positions, not a
position. The clause SHALL read positions only: every monitor and history
variable changes in discrete steps only (`Next_<clause> == (Next /\
Mon_<clause>) \/ (\E d \in Real : Tick(d) /\ UNCHANGED <<m_…>>)`), and a
direct form reads `[](tick \/ P)` for `[]P` and `<>(~tick /\ P)` for `<>P`.

**Timed intervals.** An interval `[a,b]` or `[a,*]` in time units SHALL be
read by a clock-reading monitor: the step-counting monitor of the same
operator, with each step counter replaced by the elapsed time since the
operator's obligation started, `now - m_<clause>_s<n>`, where
`m_<clause>_s<n> \in Real` records `now` at the position where the
obligation starts, and each bound test written over that elapsed time in
the clause's time unit. For an operator whose obligation starts at the
clause's origin the start is 0, the variable is omitted and the monitor
reads `now'` directly. The monitor updates at discrete steps only, so a
timed interval is read at positions, as FR-416 reads it.

**Record.** The TLA+ export of a dense-time clause SHALL be `conditional`
with the one condition `real-valued-time{clause, node}`, `clause` the
obligation's name and `node` the node id of the timed model root it reads.
The condition states: the text reads only time-divergent behaviours (the
`NZ` antecedent, the non-Zeno premise of FR-416 and FR-651), and its delays
range over `Real`, which an explicit-state model checker cannot enumerate,
so a checker verdict over the text holds only for the finite delay set the
checker is given in place of `Real`, while a proof over the text holds for
every delay. FR-815 lists this condition as the assumption of any result
over the record.

**Lean 4.** Lean SHALL state time over the rationals directly, with FR-812's
generated `Q`: `structure TBehaviour` holds `st : Nat -> State`, `ts : Nat
-> Q` (each position's time stamp, `ts 0` zero and `ts` non-decreasing),
`ck : Nat -> Clocks` (the clock places, a `structure Clocks` of one field
per clock-bearing field), each step a delay and a discrete step that
respects the time invariants and urgency, and the time divergence `forall
r : Q, exists i : Nat, Q_lt r (ts i)`. A timed interval bounds the time
stamp difference: `eventually[a,b] p` at `i` is `exists d1 : Int, 0 <= d1
/\ (0 <= (i + d1) /\ Q_le a (Q_sub (b.ts (Int.toNat (i + d1))) (b.ts
(Int.toNat i))) /\ Q_le (Q_sub …) b /\ …)`. FR-416 gives every time stamp
as an exact rational, so this states the FSL meaning exactly and the Lean
record SHALL be `preserved`.

### Constructs not lowered

A construct that FR-811's table marks `preserved` or `conditional` and that
the mapper does not lower SHALL stop the export with the typed refusal of
FR-817 "A construct this build does not lower", which names the construct
and its node; it never assembles a package and never stops with an
unexplained mapper failure. The refusal covers in particular, until each is
lowered:

| Construct | `construct` named | Encoding |
| --- | --- | --- |
| a clause over a protocol subject | the protocol clause node's `node_tag`/`semantic_form` | "Protocol subjects" above |
| a clause over a timed subject, a timed interval | the timed model root's or the interval node's `node_tag`/`semantic_form` | "Dense time" above |
| a nondeterministic monitor | the operator node's `node_tag`/`semantic_form` | "Temporal clauses in TLA+" above |
| a clause activation other than `on origin`, and its captures (FR-093) | the activation node's `node_tag`/`semantic_form` | not yet stated by this requirement |
| a window-aggregate atom (FR-414) | the aggregate node's `node_tag`/`semantic_form` | not yet stated by this requirement |
| an interval under a step-counting profile other than `filament.temporal.infinite-trace/v1` (the false-extension, event-position and fixed-sample profiles of FR-090) | the clause's profile selection node | not yet stated by this requirement |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-813-AC-1 | `always holds(c.value <= 5)` over `(c: Counter::Counter)`, exported with the counter's `initial` snapshot, exports to TLA+ as `THEOREM Th_T == Spec => \A p_c \in Ids_Counter : (p_c \in DOMAIN v_Counter) => []((p_c \in DOMAIN v_Counter /\ v_Counter[p_c].fld_value <= 5))` with `Ids_Counter == {<<99>>}` and no auxiliary variable, and to Lean as `Ob_<clause>` over `Behaviour`; both `preserved`. The text `\A p_c \in DOMAIN v_Counter : []` appears nowhere. | Test (TC-1972) |
| FR-813-AC-2 | `holds(c.req) until holds(c.ack)` exports to TLA+ with a monitor variable and a `[]<>accept` condition, and to Lean as the direct `∃ k` definition; both `preserved`. | Test (TC-1972) |
| FR-813-AC-3 | `always[0, 3] holds(c.ok)` under a step-counting profile exports a monitor whose step counter is bounded by 3 in `TypeOK`, and an exported behaviour with two inserted stuttering steps keeps the same verdict. | Test (TC-1972) |
| FR-813-AC-4 | Weak fairness on `deliver` exports `WF_vars(op_deliver)`; a clause required by a two-role protocol with one `fifo` channel of capacity 2 exports channel and role variables and send and receive actions, `preserved` (FR-813-AC-12 to FR-813-AC-16 state the text). | Test (TC-1972) |
| FR-813-AC-5 | The Lean file of AC-1 and AC-2 contains no `theorem`, no proof term and no `sorry`. | Test (TC-1972) |
| FR-813-AC-6 | `eventually always holds(c.value <= 5)` exports to TLA+ directly as `THEOREM Th_T == Spec => \A p_c \in Ids_Counter : (p_c \in DOMAIN v_Counter) => <>[]((p_c \in DOMAIN v_Counter /\ v_Counter[p_c].fld_value <= 5))` with no monitor variable, `preserved`. `eventually always (holds(c.value <= 5) since holds(c.value = 0))`, whose operand is not a state formula, exports with the history variable `m_T_h0` of the `since` and `<>[]` over it, `preserved`. A clause with no deterministic Büchi or co-Büchi monitor exports to TLA+ `conditional` with `nondeterministic-monitor` naming the clause, and to Lean `preserved`. | Test (TC-1972) |
| FR-813-AC-7 | `holds(c.value >= 1) until holds(c.value >= 3)` with the counter's snapshot exports `m_T \in [Ids_Counter -> {"run", "acc", "rej"}]`, an `Init_T` and `Mon_T` that each define `m_T` by `[p_c \in Ids_Counter \|-> …]`, and `THEOREM Th_T == Spec_T => \A p_c \in Ids_Counter : (p_c \in DOMAIN v_Counter) => []<>accept_T(p_c)` with `accept_T(p_c) == m_T[p_c] = "acc"`, and no `DOMAIN v_Counter` in `Init_T`, `Mon_T`, `TypeOK_T` or `accept_T`. Without a snapshot the same clause exports `Ids_Counter == Seq(Nat)`. A clause over `(n: Int[0, 2])` quantifies `\A p_n \in 0..2 :` with no antecedent. A behaviour in which a key absent at the origin becomes present satisfies the antecedent false for that key, so the theorem does not read it. | Test (TC-1972) |
| FR-813-AC-8 | With `A1(p_c)` the text of `holds(c.value = 1)` at `p_c` and `A2(p_c)` that of `holds(c.value >= 2)`, `always (holds(c.value >= 2) implies once[1, 2] holds(c.value = 1))` exports to TLA+ exactly: `vars_T == <<vars, m_T_r0>>`; `TypeOK_T` conjunct `m_T_r0 \in [Ids_Counter -> [1..2 -> BOOLEAN]]`; `Init_T` conjunct `m_T_r0 = [p_c \in Ids_Counter \|-> [k \in 1..2 \|-> FALSE]]`; `Mon_T` conjunct `m_T_r0' = [p_c \in Ids_Counter \|-> [k \in 1..2 \|-> IF k = 1 THEN A1(p_c) ELSE m_T_r0[p_c][k - 1]]]`; `Next_T == (Next \/ (~ENABLED Next /\ UNCHANGED vars)) /\ Mon_T`; `THEOREM Th_T == Spec_T => \A p_c \in Ids_Counter : (p_c \in DOMAIN v_Counter) => [](A2(p_c) => (\E k \in 1..2 : m_T_r0[p_c][k]))`; `preserved`. `once[1, 2] not holds(c.value = 1)` initializes its cells `TRUE`. On the behaviour `value` 0, 1, 2, 3, 4 the body holds at positions 0 to 3 and fails at position 4, whose offsets 1 and 2 read `value` 3 and 2; `once[0, 2]` reads `A1(p_c) \/ \E k \in 1..2 : m_T_r0[p_c][k]`. | Test (TC-1972) |
| FR-813-AC-9 | With `A1`, `A1p` (the same text over primed variables) and `A2` as in AC-8, `always (holds(c.value >= 2) implies once[1, *] holds(c.value = 1))` exports to TLA+ exactly: `TypeOK_T` conjunct `m_T_c0 \in [Ids_Counter -> -1..1]`; `Init_T` conjunct `m_T_c0 = [p_c \in Ids_Counter \|-> IF A1(p_c) THEN 0 ELSE -1]`; `Mon_T` conjunct `m_T_c0' = [p_c \in Ids_Counter \|-> IF m_T_c0[p_c] = -1 THEN (IF A1p(p_c) THEN 0 ELSE -1) ELSE IF m_T_c0[p_c] < 1 THEN m_T_c0[p_c] + 1 ELSE 1]`; `THEOREM Th_T == Spec_T => \A p_c \in Ids_Counter : (p_c \in DOMAIN v_Counter) => [](A2(p_c) => m_T_c0[p_c] = 1)`; `preserved`. On the behaviour `value` 0, 1, 2 the counter reads -1, 0, 1 and the body holds at every position. `once[2, *] holds(c.value >= 1)` on `value` 0, 1, 2, 3 is true at position 3, because the operand first held two steps back; a counter of the steps since the operand last held reads 0 there and gives false, which this criterion rejects. | Test (TC-1972) |
| FR-813-AC-10 | `p since[1, 2] q` exports two registers of length 2, `m_T_r0` over `q` and `m_T_r1` over `p`, read as `\E d \in 1..2 : Q(d) /\ \A e \in 1..(d - 1) : P(e)`; `p since[2, *] q` exports the history Boolean `m_T_h0` of `p since q` and a register `m_T_r0` of length 2 over it with `Init` cells `FALSE`, read as `m_T_r0[p_c][2]`. | Test (TC-1972) |
| FR-813-AC-11 | Every monitor and history variable advances at a terminal state: the counter behaviour from `value` 0 reaches its terminal state at `value` 5 at position 5, so `eventually (holds(c.value = 5) and once[1, 1] holds(c.value = 5))` holds there only at the terminal stutter position 6, and its export with `Next_T == (Next \/ (~ENABLED Next /\ UNCHANGED vars)) /\ Mon_T` states it true. An export whose `Next_T` omits the `~ENABLED Next` disjunct never shifts the register at the terminal state, states the clause false and fails this criterion. | Test (TC-1972) |
| FR-813-AC-12 | Protocol `Hand` over `(k: Counter::Counter)` `on origin`, with roles `a` and `b` on the counter model, a `fifo` channel `ch` from `a` to `b` carrying `Boolean` with FIFO key `true`, delivery `exactly-once-premise`, capacity 1, overflow `block`, `run sequence Main { send Snd via ch as (m) { m }; receive Rcv via ch of Snd as (r) { r } }` and `finish Done`, requiring the temporal clause `T` (`always holds(c.value <= 5)`), exports to TLA+ the variables `pr_Hand_over`, `pr_Hand_last`, `pr_Hand_control`, `pr_Hand_observations`, `pr_Hand_bounds`, `pr_Hand_queues`, `pr_Hand_roles` and `pr_Hand_memory` beside `v_Counter`; `PInit_Hand` holds the conjuncts `pr_Hand_over \in Ids_Counter`, `pr_Hand_over \in DOMAIN v_Counter`, `pr_Hand_last = "init"`, `pr_Hand_control = ([type \|-> "protocol-instance", ordinal \|-> 0] :> [type \|-> "instance-state", finished \|-> FALSE]) @@ ([type \|-> "thread-path", instance \|-> 0, steps \|-> <<>>] :> PK_At("Hand::Main::Snd"))`, `pr_Hand_observations = <<>>`, `pr_Hand_bounds = <<>>`, `pr_Hand_queues = <<>>`, `pr_Hand_memory = <<>>` and `pr_Hand_roles` with exactly the two keys `[type \|-> "role", instance \|-> 0, role \|-> "a"]` and `[type \|-> "role", instance \|-> 0, role \|-> "b"]`, each mapped to `[type \|-> "model-binding", selection \|-> S]` with `S` the counter's `model_root` node identity; the actions `s_Hand_Hand_N_Main_N_Snd(t, w_m)`, `s_Hand_Hand_N_Main_N_Rcv(t, q)` and `s_Hand_Hand_N_Done(t, w_d)`; `THEOREM Th_T == PSpec_Hand => \A p_c \in Ids_Counter : (p_c \in DOMAIN v_Counter) => []((p_c \in DOMAIN v_Counter /\ v_Counter[p_c].fld_value <= 5))`; no `Next` disjunct of the model's own operations; `preserved` in both targets. | Test (TC-1972) |
| FR-813-AC-13 | The send action of AC-12 is exactly: `s_Hand_Hand_N_Main_N_Snd(t, w_m) ==` with the conjuncts `t \in DOMAIN pr_Hand_control`; `pr_Hand_control[t] = PK_At("Hand::Main::Snd")`; `w_m \in BOOLEAN`; `w_m`; `LET q == [type \|-> "queue", instance \|-> t.instance, channel \|-> "Hand::ch", fifo_key \|-> TRUE] o == [type \|-> "message-ordinal", node \|-> "Hand::Main::Snd", thread \|-> t] m == [type \|-> "message-occurrence", instance \|-> t.instance, node \|-> "Hand::Main::Snd", path \|-> t, ordinal \|-> PK_Get(pr_Hand_bounds, o, 0)] n == [type \|-> "next-delivery", message \|-> m] e == [type \|-> "queue-entry", message \|-> m, delivery \|-> PK_Get(pr_Hand_bounds, n, 0), payload \|-> w_m] IN /\ Len(PK_Get(pr_Hand_queues, q, <<>>)) < 1 /\ pr_Hand_queues' = PK_Put(pr_Hand_queues, q, Append(PK_Get(pr_Hand_queues, q, <<>>), e)) /\ pr_Hand_bounds' = PK_Put(PK_Put(pr_Hand_bounds, n, PK_Get(pr_Hand_bounds, n, 0) + 1), o, PK_Get(pr_Hand_bounds, o, 0) + 1)`; `pr_Hand_control' = [pr_Hand_control EXCEPT ![t] = PK_At("Hand::Main::Rcv")]`; `pr_Hand_last' = "send"`; `UNCHANGED <<v_Counter, pr_Hand_over, pr_Hand_observations, pr_Hand_roles, pr_Hand_memory>>`; the binder `m` writes no `observations` entry, because no node reads it. After the send the state's `pr_Hand_queues` and `pr_Hand_bounds` hold exactly FR-425-AC-15's entry and bounds (delivery 0, `message-ordinal` 1, `next-delivery` 1). | Test (TC-1972) |
| FR-813-AC-14 | Guard: the receive action of AC-12 holds the conjunct `LET w_r == Head(pr_Hand_queues[q]).payload IN w_r`. With the send constraint `{ true }`, the state whose queue holds one entry with payload `FALSE` enables no `s_Hand_Hand_N_Main_N_Rcv` instance and the thread waits at `Rcv`, as FR-426 makes the receive wait; with payload `TRUE` the receive removes the entry, deletes the queue's key from `pr_Hand_queues` and rests the root at `PK_At("Hand::Done")`. | Test (TC-1972) |
| FR-813-AC-15 | Choice: a variant whose `Rcv` is followed by `choice C { case yes when r: attempt Inc by a on Counter::Counter::inc …; case no when not r: … }` exports no action for `C`; the receive action sets the thread's rest point by `CASE w_r -> PK_At("Hand::Main::C::yes::Inc") [] ~w_r -> PK_At("Hand::Main::C::no::<first node>")`, and writes the binder entry `[type \|-> "binder", node \|-> "Hand::Main::Rcv", thread \|-> t]` only when a node reachable from that rest point reads `r`. | Test (TC-1972) |
| FR-813-AC-16 | Overflow: with capacity 1 and a queue already holding one entry, the send's queue update is, for `reject`, `pr_Hand_queues' = IF Len(c) < 1 THEN PK_Put(pr_Hand_queues, q, Append(c, e)) ELSE pr_Hand_queues`; for `loss head`, `PK_Put(pr_Hand_queues, q, IF Len(c) < 1 THEN Append(c, e) ELSE Append(Tail(c), e))`; for `loss tail`, `… ELSE Append(SubSeq(c, 1, Len(c) - 1), e)`; for `loss new`, as `reject`; with `c` the text `PK_Get(pr_Hand_queues, q, <<>>)`. Under every policy the bounds update of AC-13 is unchanged, so a rejected copy takes delivery index 0 and the next send's entry has delivery 1. Under `block` the full queue enables no send. | Test (TC-1972) |
| FR-813-AC-17 | A clause that no protocol's activation relation requires is exported over its model subject with FR-812's `Spec`; the same clause required by `Hand` on an `origin` row is exported over `PSpec_Hand`; required by two protocols it is stated by one theorem per row in row order; required on an `each` row its TLA+ body is `[](pr_<P>_last = "activate" => F)`. | Test (TC-1972) |
| FR-813-AC-18 | A timed model `Lamp` (`time dense unit ms`, object type `Lamp` with `on: Boolean` and clock `x`, operation `release` with guard `self.on and self.x >= 2 ms`, time invariant `when self.on { self.x <= 5 ms }`) exports `EXTENDS` holding `Reals`, the variables `now`, `tick` and `k_Lamp_x`, an action `Tick(d)` whose conjuncts include `d \in Real`, `d > 0`, `~Urgent` and `\A k \in DOMAIN v_Lamp : v_Lamp[k].fld_on => k_Lamp_x[k] + d <= 5`, `release`'s action with `k_Lamp_x[r_self] >= 2`, `now' = now` and `tick' = FALSE`, and `NZ == \A r \in Real : <>(now > r)`. The clause `always holds(l.on or not l.on)` over `(l: Lamp::Lamp)` exports to TLA+ `conditional` with exactly one condition `real-valued-time{clause: T, node}`, `node` the `Lamp` timed model root, and to Lean `preserved` with no condition. | Test (TC-1972) |
| FR-813-AC-19 | With `B0(p_l)` the text of `holds(not l.on)` at `p_l`, `eventually[0 ms, 5 ms] holds(not l.on)` over the `Lamp` of AC-18 exports to TLA+ exactly: `Init_T` conjunct `m_T = [p_l \in Ids_Lamp \|-> IF (0 <= 0 /\ 0 <= 5) /\ B0(p_l) THEN "acc" ELSE IF 0 > 5 THEN "rej" ELSE "run"]`; `Mon_T` conjunct `m_T' = [p_l \in Ids_Lamp \|-> IF m_T[p_l] /= "run" THEN m_T[p_l] ELSE IF (0 <= now' /\ now' <= 5) /\ B0p(p_l) THEN "acc" ELSE IF now' > 5 THEN "rej" ELSE "run"]`; `Next_T == (Next /\ Mon_T) \/ (\E d \in Real : Tick(d) /\ UNCHANGED <<m_T>>)`; `THEOREM Th_T == Spec_T => (NZ => \A p_l \in Ids_Lamp : (p_l \in DOMAIN v_Lamp) => []<>accept_T(p_l))`; and to Lean `holds_T` as `(exists d1 : Int, 0 <= d1 /\ (0 <= (i + d1) /\ Q_le (Q.mk 0 1) (Q_sub (b.ts (Int.toNat (i + d1))) (b.ts (Int.toNat i))) /\ Q_le (Q_sub (b.ts (Int.toNat (i + d1))) (b.ts (Int.toNat i))) (Q.mk 5 1) /\ a_T_0 (b.st (Int.toNat (i + d1))) p_l))`. A `Tick` step leaves `m_T` unchanged, so a behaviour whose delays alone pass 5 ms without a `release` leaves `m_T` at `"run"` until its next discrete step reads `now' > 5` and rejects. | Test (TC-1972) |
| FR-813-AC-20 | The TLA+ export of a clause over a hybrid model `Braking` keeps FR-811-AC-6's `unrepresented` record with one `target-unrepresentable` cause per flow-equation node and adds no `real-valued-time` condition. | Test (TC-1972) |
| FR-813-AC-21 | Over the counter with an `initial` snapshot at `value` 4, whose only behaviour is `value` 4, then 5, then terminal stutter from position 1: `always[2, 3] holds(c.value = 5)` and `eventually[2, 3] holds(c.value = 5)` are true and `eventually[2, 3] holds(c.value = 4)` is false in FSL. The TLA+ exports carry `Next_T == (Next \/ (~ENABLED Next /\ UNCHANGED vars)) /\ Mon_T` and `Spec_T == Init_T /\ [][Next_T]_vars_T /\ WF_vars_T(Next_T)` and a step counter `n_T \in 0..3`; TLC run as an observer reports `holds`, `holds` and `violated` for the three; the Lean `holds_T` of each at position 0 over the `Behaviour` 4, 5, 5, 5, … is true, true and false. An export without the terminal disjunct or without its weak fairness leaves `n_T` at 1, so its `eventually[2, 3] holds(c.value = 5)` export reports `violated`, which this criterion rejects. | Test (TC-1972) |
| FR-813-AC-22 | `fair weak each Counter::Counter::inc; always eventually holds(c.value = 2)` exports `Spec_T == Spec /\ \A p_self \in Ids_Counter : WF_vars((op_inc(p_self)))`; no TLA+ text of a temporal clause in the module holds `DOMAIN v_` outside FR-812's `Next`, `TypeOK`, an action body, a state clause, an atom's presence conjunct and a theorem's origin antecedent. | Test (TC-1972) |
| FR-813-AC-23 | Unbounded and `[a,*]` past operators start from the empty range: `historically holds(c.value = 1)` exports `Init` `m_T_h0 = [p_c \in Ids_Counter \|-> A1(p_c)]` and is false at position 0 of the counter from `value` 0; `historically[1, *] holds(c.value = 1)` is true at position 0 (its range is empty) in both targets, and its Lean text bounds the offset by `0 <= i - d`; `historically[1, 1] holds(c.value = 1)` is false at position 0, because a closed interval reads the pre-origin extension. An export that reads the pre-origin extension for an unbounded or `[a,*]` operator makes the second case false and fails this criterion. | Test (TC-1972) |
| FR-813-AC-24 | Each construct of "Constructs not lowered", in a build that does not lower it, stops the export with FR-817's `unimplemented_capability`/`unimplemented-meaning` refusal naming that construct, exit `21`, with no package and no file, and never with exit `30`. | Test (TC-1972) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-812](FR-812-map-state-actions-and-values-to-tla-and-lean.md);
  [FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md) owns
  protocol subjects and their state, [FR-426](../protocol/FR-426-apply-protocol-steps.md)
  their steps, [FR-431](../protocol/FR-431-count-protocol-steps-for-intervals.md)
  their positions and counted steps, FR-464 the activation relation;
  [FR-092](../temporal/FR-092-evaluate-bounded-past.md) the past reading;
  [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md) and
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) dense time;
  FR-377 the terminal stutter.
