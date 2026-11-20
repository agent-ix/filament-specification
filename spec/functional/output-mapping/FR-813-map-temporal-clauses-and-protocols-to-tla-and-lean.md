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
  - target: ix://agent-ix/filament-specification/FR-363
    type: references
  - target: ix://agent-ix/filament-specification/FR-161
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
and a TLA+ form is `conditional` only on a condition this requirement names.

## Inputs

The obligation's temporal, fairness, protocol and timed-model nodes; the
state, actions and values of FR-812; and the export request's `initial`
snapshot and protocol subject inputs when it names them (FR-817).

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
  the specification: the monitor's states become auxiliary variables
  `m_<clause>` updated in every `Next` disjunct, past operators become
  history variables ("Past operators in TLA+" below), and the clause becomes
  the acceptance condition over the monitor. Each interval operator has its
  own step counter `n_<clause>_<n>`, `<n>` the operator's ordinal among the
  clause's future interval operators in pre-order from 0, ranging over
  `0..b` with `b` that operator's upper bound and saturating there. The
  monitor SHALL be deterministic wherever the clause has a deterministic
  safety, Büchi or co-Büchi monitor, with acceptance `[]~reject`,
  `[]<>accept` or `<>[]accept` respectively; a deterministic monitor reads
  each behaviour along exactly one run, so the encoding is exact and the
  record `preserved`. Auxiliary variables add no behaviour to the original
  variables.
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
  changes no variable of `vars_<clause>` and is a TLA+ stutter.
- **Composed specification.** Every clause that adds a monitor or a
  fairness constraint SHALL be stated over the one form `Spec_<clause> ==
  Init_<clause> /\ [][Next_<clause>]_vars_<clause> /\
  WF_vars_<clause>(Next_<clause>) /\ <model fairness> /\ <clause fairness>`,
  where `<model fairness>` is FR-812's `fairness` conjuncts and `<clause
  fairness>` the clause's own `WF_vars`/`SF_vars` conjuncts, each conjunct
  omitted when it is empty. A clause with fairness and no monitor has no
  `Init_<clause>`, `Next_<clause>` or `vars_<clause>`, and the same form
  reads `Init`, `Next` and `vars` in their place: `Spec_<clause> == Init /\
  [][Next]_vars /\ WF_vars(Next) /\ <model fairness> /\ <clause fairness>`.
  The weak fairness on `Next_<clause>` makes the terminal step taken while it
  changes a monitor variable, so step counters, history variables and shift
  registers advance at a deadlock as FSL's positions do. Lean's `Behaviour`
  already repeats the terminal state at every later position (FR-812), so
  both targets read the same positions.
- The obligation's statement SHALL be `THEOREM Th_<clause> == Spec =>
  <formula>`, with `Spec_<clause>` in place of `Spec` when the clause adds a
  monitor or fairness, written without a proof.

### Quantified clauses and undefined reads

A temporal formula SHALL NOT be quantified over a set that depends on the
state: TLA+ evaluates the set of `\A p \in DOMAIN v_T : F` in the first
state only, while `F`'s temporal operators read later states. FR-361
"Binding of the clause's `over` parameter" fixes the instances of an `over`
binder up front: one instance per key that a member record of some initial
state of the subject admits, every behaviour of the subject checked for every
instance, and a read through the binder at a position where the state holds
no object under the key undefined, which refutes the whole item
(`undefined-evaluation`, FR-363) whatever the polarity of the read. The
export SHALL state exactly that reading:

- **Identity universe.** For each object type `T` that an `over` binder
  ranges over, the module SHALL define `Ids_<T>`, the instance universe:
  - when the request names an `initial` snapshot, the set of the keys of
    `T`'s population in that snapshot, written as a set literal of FR-812
    keys in ascending key order (`Ids_Counter == {<<99>>}`);
  - otherwise `Ids_<T> == Keys_<T>`, below, since every well-typed state
    whose keys lie in `Keys_<T>` is initial (FR-812 `Init == TypeOK`) and so
    admits each of them.
- **Key bound.** When the request names no snapshot, or when some operation
  of the model can add a key to `T`'s population, the set of keys a state can
  hold is not fixed by the request. The module SHALL then declare `CONSTANT
  Keys_<T>` with `ASSUME Keys_<T> \subseteq Seq(Nat) /\ IsFiniteSet(Keys_<T>)`
  (and `ASSUME <snapshot key set> \subseteq Keys_<T>` with a snapshot), whose
  value the model checker's configuration supplies, and SHALL add the
  `TypeOK` conjunct `DOMAIN v_<T> \subseteq Keys_<T>`. The TLA+ record is
  then `conditional` with the condition `bounded-identity-universe{type,
  node}`, `type` the object type's name and `node` its node id, which states
  that the text reads only the behaviours whose keys of `T` lie in the
  supplied `Keys_<T>`. No quantifier or function domain of the text is
  `Seq(Nat)`, so a finite model stays finite. When the snapshot fixes every
  key, `Keys_<T>` is not declared and the record carries no such condition.
- **Statement.** A clause with `over` binders `(c1: T1, …)` SHALL be stated
  as `Spec => \A o_c1 \in Ids_<T1>, … : [](ok_<clause>(o_c1, …)) /\ F`, with
  no antecedent: every instance is checked on every behaviour, including a
  behaviour from an initial state that lacks its key, as FR-361 checks it.
- **Definedness.** `ok_<clause>(o_c1, …)` SHALL be the state predicate that
  every read and partial operation the clause's atoms evaluate is defined:
  the conjunction, over the clause's atoms, of each binder read's presence
  `o_c \in DOMAIN v_<T>` and each FR-812 definedness condition, each
  conjoined under the condition that makes the FSL kernel evaluate it (a read
  behind a `lookup` guard contributes `guard => condition`, and a read the
  guard keeps from evaluating contributes nothing). FSL reads the claim at
  every position of every admitted behaviour (FR-363 "Undefined
  evaluation"), so `[]ok_<clause>` is false exactly on the behaviours FSL
  refutes by an undefined read, and the atoms inside `F` carry no presence
  conjunct of their own. An application `v_<T>[o_c]` outside the domain is a
  TLA+ value the text does not constrain; the formula's value does not
  depend on it, because `[]ok_<clause>` is false on every behaviour that
  reaches such a state, and a checker that reports the application as an
  evaluation error reports it at a state where FSL refutes.
- A binder over a type that is not an object type ranges over its FR-812
  value set, which does not depend on the state, and contributes no presence
  conjunct.
- `Ids_<T>` SHALL replace `DOMAIN v_<T>` in every temporal-level range: the
  theorem's quantifier, the domain of every monitor and history function,
  the acceptance predicate (`accept_<clause>(o_c) == m_<clause>[o_c] =
  "acc"`, read per identity) and, with `Keys_<T>` in its place when it is
  declared, the receiver range of an `each` fairness constraint (`\A r_self
  \in Ids_<T> : WF_vars(op_<op>(r_self))`). An action's own first conjunct
  is its receiver's presence `r_self \in DOMAIN v_<T>` (FR-812), so an action
  over an absent identity is not enabled and its fairness conjunct is
  satisfied vacuously. `DOMAIN v_<T>` stays the range of FR-812's `Next`, of
  a state clause, which read one state, and of `ok_<clause>`.
- Each monitor and history variable of a quantified clause SHALL be a
  function on `Ids_<T>` (on the product of the universes for several
  binders), and the acceptance condition SHALL be read per identity inside
  the quantifier: `Spec_<clause> => \A o_c \in Ids_<T> :
  [](ok_<clause>(o_c)) /\ []<>accept_<clause>(o_c)`.

In Lean, `Ids_<T>` SHALL be the predicate `def Ids_<T> (k : Key_<T>) : Prop
:= exists s : State, Init s /\ F_has_<T> k s.v_<T>` (admitted by some initial
state), `ok_<clause>` the same definedness predicate over a `State`, and the
obligation `def Ob_<clause> : Prop := forall b : Behaviour, forall o_c,
Ids_<T> o_c -> (forall j : Nat, ok_<clause> (b.st j) o_c) /\ holds_<clause> b
o_c 0`. Lean states every key with no bound, so the Lean record carries no
`bounded-identity-universe` condition.

### Past operators in TLA+

FR-092 reads a past operator over the positions back to the origin and,
for a closed interval, over a pre-origin extension at which every atom is
false and every Boolean constant keeps its value. The TLA+ encoding SHALL
state exactly that reading with bounded history variables. The *pre-origin
value* `pre(φ)` of a past-only operand `φ` is the Boolean constant obtained
by evaluating `φ` at a pre-origin position: every atom false, every Boolean
constant unchanged, and every unbounded or `[a,*]` past operator inside `φ`
its empty-range value (`once` and `since` false, `historically` and
`triggered` true), because such an operator reads no position before the
origin; the mapper computes it and writes it as `TRUE` or `FALSE`. An
operand that holds a future operator is not past-only, and its clause falls
under the monitor rules above.

History variables are numbered by the clause's past operators in pre-order
(outer before inner, left before right), from 0, one ordinal per operator,
and named `m_<clause>_h<n>` (an unbounded operator's Boolean), `m_<clause>_r<n>`
(a shift register) and `m_<clause>_c<n>` (a saturating counter); `since` and
`triggered` registers take the operator's ordinal for the right operand `q`
and the ordinal plus one, which no other operator then takes, for the left
operand `p`. The conjuncts of `Mon_<clause>` SHALL appear in reverse ordinal
order (inner before outer), so that every primed history variable an outer
update reads in `φ'` is assigned before it is read. Over a quantified clause
each is a function on `Ids_<T>` as above; the text below writes the
unquantified form. `φ` is the operand's text over unprimed variables and
`φ'` the same text over primed variables.

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
Each atom is a `def at_<clause>_<n>` over a `State` and the binders, `<n>`
the atom's ordinal in the clause in pre-order from 0.
A closed past interval reads a position before 0 as the pre-origin
extension, each atom conjoined with `0 <= j` and so false there. An
unbounded or `[a,*]` past operator bounds its offsets by `0 <= i - d`
instead: `historically φ` at `i` is `forall d : Int, 0 <= d -> 0 <= i - d
-> φ (i - d)`, true when the range is empty, and `once φ` is `exists d :
Int, 0 <= d /\ 0 <= i - d /\ φ (i - d)`, so no position before the origin
is read.
The obligation's statement SHALL be the `def Ob_<clause> : Prop` of
"Quantified clauses and undefined reads", over `holds_<clause> b o_c 0`.
The file states obligations as propositions and contains no `theorem`, no
proof and no `sorry`.

### Fairness and terminal states

- Weak and strong fairness of an operation SHALL be `WF_vars(op_<op>)` and
  `SF_vars(op_<op>)` (with existential argument choice) in TLA+, and the
  corresponding `∀ i, (∃ j ≥ i, …)` premises in Lean, conjoined into the
  composed specification above.
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
`protocol_clause` node) holds an `origin` row requiring that clause; a
clause on an `each` row is a construct not lowered ("Constructs not lowered"
below). The subject is the one an analyze request selects with the protocol
binding `{protocol: P, over, roles, capacities}` (FR-520), whose inputs the
export request carries in the same members (FR-817 "Protocol subject
inputs"):

- When the request carries a binding for `P`, the rigid variable
  `pr_<P>_over` is its `over` key (`pr_<P>_over = <key>` in `PInit_<P>`),
  each static role on a model type takes its `roles` binding, and each
  symbolic capacity its `capacities` value. When it carries none,
  `pr_<P>_over` ranges over `Ids_<T>` of the `over` type (`pr_<P>_over \in
  Ids_<T>`), checked for every key as a clause binder is, and a symbolic
  capacity is a TLA+ `CONSTANT` with its declared constraint as an `ASSUME`
  and a Lean `variable` with the constraint as a hypothesis of
  `Ob_<clause>`.
- A role on a whole model is bound by construction (FR-425).
- Each replicated role's universe, from the request's `universes` member, is
  the TLA+ `CONSTANT pr_<P>_universe_<role>` with `ASSUME
  pr_<P>_universe_<role> = <the universe's keys>` and a Lean `variable` of
  that value.

The clause is evaluated at position 0 of every behaviour of the subject. A
clause that rows of several protocols require is stated once per row, in the
order of the rows, and a clause that no row requires is exported over its
model subject. A clause's positions, atoms and distances over the subject
are FR-431's: position 0 is the initial protocol state, every step is a
position, a `holds` atom reads the model observation, and an interval
measures distance, the number of counted steps between two positions
("Intervals over a protocol subject" below).

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
- A queue SHALL be a sequence of entries in send order for a `fifo` channel
  and for an `unordered` channel whose overflow is `loss head` or `loss tail`
  (FR-425 "Binders, counters, queues and roles"), and a set of entries for
  any other `unordered` channel: every entry has its own (message, delivery)
  identity (FR-425), so FR-812's bag of such a queue has every count 1 and
  is that set. A `receive` from an `unordered` queue takes any entry, a
  member of the set or any element of the send-ordered sequence. A queue
  holding no entry has no key in `pr_<P>_queues`, as FR-425 states.
- `PTypeOK_<P>` SHALL hold: each of the seven maps is a function on its
  present keys; for each key `x` of `pr_<P>_queues`, `Len(pr_<P>_queues[x])`
  (a sequence) or `Cardinality(pr_<P>_queues[x])` (a set) is at most the
  capacity of `x.channel`, a symbolic capacity read as its constant; and
  `pr_<P>_over \in Ids_<T>`. Lean's `PState_<P>` well-formedness holds the
  same conjuncts.
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
  (`"init"` in an initial state, `"stutter"` after the terminal stutter),
  which FR-431's counted steps read. Lean's `PState_<P>` holds them as
  `over` and `last`.

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
| `receive` | `s_<P>_<node>(t, q)`: a deliverable entry of queue `q` (the head of the sequence for `fifo`, any entry for `unordered`) from the send node the receive names, whose payload satisfies the receive constraint, the guard predicate, read with the binder bound to the payload; the entry removed |
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
  capacity; `loss head` removes the entry sent first (the first of the
  sequence) and appends the new one; `loss tail` removes the entry sent last
  (the last of the sequence) and appends the new one; `loss new` leaves the
  queue unchanged. A rejected or lost copy still takes its delivery index
  (FR-425), so the bounds update is the same under every policy.
- Each action SHALL set `pr_<P>_last'` to its step kind and leave every
  variable it does not write unchanged. The terminal stutter of the subject
  is the disjunct `~ENABLED PNext_<P> /\ pr_<P>_last' = "stutter" /\
  UNCHANGED <<every other variable>>`.
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
  member's class. A monitored clause composes `PSpec_<P>` exactly as
  "Composed specification" above composes `Spec`. Lean's `PBehaviour_<P>`
  holds the same premises.
- Lean SHALL state the subject as `PInit_<P>`, `PStep_<P>` (the disjunction
  of one `def` per action schema, each a relation on `PState_<P>`) and
  `PBehaviour_<P>` over `Nat -> PState_<P>`, and the obligation as `def
  Ob_<clause> : Prop` over `PBehaviour_<P>`.

**Intervals over a protocol subject.** FR-431 measures an interval by
distance: only `attempt`, `cattempt`, `cfail`, `ctimeout`, `event`,
`activate`, `send` and `receive` steps and the terminal stutter are counted,
and several positions share one distance. The generated predicate
`PK_Counted(k) == k \in {"attempt", "cattempt", "cfail", "ctimeout",
"event", "activate", "send", "receive", "stutter"}` SHALL decide it from
`pr_<P>_last'`. Over a protocol subject:

- each step counter `n_<clause>_<n>` and each `[a,*]` counter
  `m_<clause>_c<n>` SHALL advance only on a step with
  `PK_Counted(pr_<P>_last')` and stay unchanged on every other step; a
  future interval monitor whose counter stands at its upper bound `b`
  rejects (or, for `always[a,b]`, accepts) at the next counted step, because
  every position of distance `b` precedes it;
- a closed-interval register SHALL be `m_<clause>_r<n> \in [0..b ->
  BOOLEAN]`, cell `k` holding the operand over the positions at distance `k`
  back and cell 0 accumulating it over the positions at distance 0. Its
  `Init` is `[k \in 0..b |-> IF k = 0 THEN φ ELSE pre(φ)]`; on a counted
  step it shifts (`[k \in 0..b |-> IF k = 0 THEN φ' ELSE
  m_<clause>_r<n>[k - 1]]`); on an uncounted step only cell 0 changes, to
  `m_<clause>_r<n>[0] \/ φ'` for a register read existentially (`once`,
  the `q` register of `since`, the `p` register of `triggered`) and to
  `m_<clause>_r<n>[0] /\ φ'` for one read universally (`historically`, the
  `p` register of `since`, the `q` register of `triggered`). The readings of
  "Closed interval `[a,b]`" then read `R(k)` as cell `k` for every `k`,
  cell 0 included, which states "some (every) position at distance `k`"
  exactly.

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
  delay that is not a position, `FALSE` initially;
- the variable `idle \in BOOLEAN`, `TRUE` exactly once the behaviour has
  entered its idle tail, `FALSE` initially;
- the action `Tick(d) == /\ ~idle /\ d \in Real /\ d > 0 /\ ~Urgent /\
  TimeInv(d) /\ now' = now + d /\ <every live clock place advanced by d> /\
  tick' = TRUE /\ UNCHANGED <<v_…, idle>>`, where `TimeInv(d)` is the
  conjunction over the model's time invariants of `P => body` with each clock
  read as its value plus `d` (each body is a conjunction of upper bounds, so
  it holds throughout the delay exactly when it holds at its end), and
  `Urgent` is the disjunction of the `urgent when P` predicates and of the
  enabledness of each `urgent O` operation;
- discrete steps: each operation action has the conjunct `~idle`, reads its
  clock guards at the current valuation, writes its resets (`x' = c` at the
  reset place) and keeps every other clock place, with `now' = now` and
  `tick' = FALSE`, so every discrete step is urgent-compatible with delay 0;
- the idle tail of FR-161 and FR-416 "Idle tail": `Quiescent == ~Urgent /\
  ~(P_1 \/ … \/ P_n)`, with `P_1` to `P_n` the predicates of the model's time
  invariants (no time invariant applies and the state is not urgent), and
  the action `Idle(d) == /\ Quiescent /\ d \in Real /\ d > 0 /\ now' = now +
  d /\ <every live clock place advanced by d> /\ tick' = FALSE /\ idle' =
  TRUE /\ UNCHANGED <<v_…>>`: each `Idle` step is a terminal stutter position
  stamped after a positive delay of any length, and once taken only `Idle`
  steps follow, so the behaviour's discrete steps have ended at a quiescent
  state;
- `TNext == Next \/ (\E d \in Real : Tick(d)) \/ (\E d \in Real :
  Idle(d))`, `Spec == Init /\ [][TNext]_vars /\ WF_vars(TNext)`, with `now`,
  `tick`, `idle` and the clock variables in `vars`;
- `NZ == \A r \in Real : <>(now > r)`, the time divergence FR-416 reads
  behaviours under; a behaviour whose discrete steps end at a state that is
  not quiescent cannot diverge and so is not read, as FR-416 does not admit
  it. The statement is `THEOREM Th_<clause> == Spec_<clause> => (NZ => F)`.

A state reached by `Tick` is an instant between FR-416's positions, not a
position; a state reached by a discrete step or by `Idle` is a position. The
clause SHALL read positions only: every monitor and history variable changes
in discrete and `Idle` steps only (`Next_<clause> == ((Next \/ \E d \in Real
: Idle(d)) /\ Mon_<clause>) \/ (\E d \in Real : Tick(d) /\ UNCHANGED
<<m_…>>)`), and a direct form reads `[](tick \/ P)` for `[]P` and `<>(~tick
/\ P)` for `<>P`. The untimed terminal disjunct is not used: the idle tail
is the timed subject's terminal stutter.

**Timed intervals.** An interval `[a,b]` or `[a,*]` in time units SHALL be
read by a clock-reading monitor: the step-counting monitor of the same
operator, with each step counter replaced by the elapsed time since the
obligation instance started, and each bound test written over that elapsed
time in the clause's time unit. Timed interval operators are numbered by
pre-order from 0 among the clause's timed interval operators.

- An operator that is not inside another temporal operator has one
  obligation instance, started at the origin, whose start is 0: no start
  variable is written and the monitor reads `now'` directly.
- An operator inside `always` starts an obligation instance at every
  position. Each instance SHALL be identified by its start time stamp, and
  `m_<clause>_s<n> \in SUBSET Real` SHALL hold the start stamps of the open
  instances (instances with equal start stamps meet the same deadlines and
  are one). At each position the monitor adds `now'`, applies each open
  instance's test with elapsed time `now' - s`, and removes the instances
  that the test settles: for `eventually[a,b] p`, `m_<clause>_s<n>' = {s \in
  m_<clause>_s<n> \cup {now'} : ~(p' /\ a <= now' - s /\ now' - s <= b)}`,
  rejecting when some open `s` has `now' - s > b`; for `always[a,b] p`,
  rejecting when some open `s` has `a <= now' - s /\ now' - s <= b /\ ~p'`
  and removing every `s` with `now' - s > b`. The acceptance condition is
  `[]~reject`.
- An operator inside any other temporal operator (`eventually`, `until`,
  `release`, or another timed interval) is a construct not lowered.

The monitor updates at discrete and `Idle` steps only, so a timed interval
is read at positions, as FR-416 reads it.

**Record.** The TLA+ export of a dense-time clause SHALL be `conditional`
with the condition `real-valued-time{clause, node}`, `clause` the
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
per clock-bearing field), each step either a delay and a discrete step that
respects the time invariants and urgency, or an idle step from a quiescent
state (the same `st`, `ts` strictly increasing, clocks advanced by the
delay) after which every later step is an idle step, and the time
divergence `forall r : Q, exists i : Nat, Q_lt r (ts i)`. A timed interval
bounds the time stamp difference: `eventually[a,b] p` at `i` is `exists d1 :
Int, 0 <= d1 /\ (0 <= (i + d1) /\ Q_le a (Q_sub (b.ts (Int.toNat (i + d1)))
(b.ts (Int.toNat i))) /\ Q_le (Q_sub …) b /\ …)`. FR-416 gives every time
stamp as an exact rational, so this states the FSL meaning exactly and the
Lean record SHALL be `preserved`.

### Conditions

Each condition this requirement names SHALL be written in the record's
`conditions` (FR-298) as the closed JSON object `{"type": "<name>",
<members>}`, in the order the rules above add them, `node` a canonical
node-id string and `clause` and `type` declared names:
`{"type": "nondeterministic-monitor", "clause", "node"}`, `{"type":
"real-valued-time", "clause", "node"}` and `{"type":
"bounded-identity-universe", "type_name", "node"}`, the last naming the
object type in `type_name`.

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
| a clause on an `each` row of a protocol's activation relation, a clause activation other than `on origin`, and their captures (FR-093) | the activation node's `node_tag`/`semantic_form` | not yet stated by this requirement |
| a window-aggregate atom (FR-414) | the aggregate node's `node_tag`/`semantic_form` | not yet stated by this requirement |
| an interval under a step-counting profile other than `filament.temporal.infinite-trace/v1` (the false-extension, event-position and fixed-sample profiles of FR-090) | the clause's profile selection node | not yet stated by this requirement |
| `since[a,*]` or `triggered[a,*]` over a protocol subject | the operator node's `node_tag`/`semantic_form` | not yet stated by this requirement |
| a timed interval inside a temporal operator other than `always` | the interval node's `node_tag`/`semantic_form` | not yet stated by this requirement |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-813-AC-1 | `always holds(c.value <= 5)` over `(c: Counter::Counter)`, exported with the counter's `initial` snapshot, exports to TLA+ `Ids_Counter == {<<99>>}`, `ok_T(o_c) == o_c \in DOMAIN v_Counter` and `THEOREM Th_T == Spec => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ [](v_Counter[o_c].fld_value <= 5)`, with no auxiliary variable and no `CONSTANT Keys_Counter`, and to Lean `def Ids_Counter (k : List Nat) : Prop := exists s : State, Init s /\ F_has_Counter k s.v_Counter`, `def ok_T (s : State) (o_c : List Nat) : Prop := F_has_Counter o_c s.v_Counter` and `def Ob_T : Prop := forall b : Behaviour, forall o_c, Ids_Counter o_c -> (forall j : Nat, ok_T (b.st j) o_c) /\ holds_T b o_c 0`; both `preserved`. The texts `\A o_c \in DOMAIN v_Counter`, `(o_c \in DOMAIN v_Counter) =>` and `F_has_Counter o_c (b.st 0).v_Counter ->` appear nowhere. | Test (TC-1972) |
| FR-813-AC-2 | `holds(c.req) until holds(c.ack)` exports to TLA+ with a monitor variable and a `[]<>accept` condition, and to Lean as the direct `∃ k` definition; both `preserved`. | Test (TC-1972) |
| FR-813-AC-3 | `always[0, 3] holds(c.ok)` under a step-counting profile exports a monitor whose step counter is bounded by 3 in `TypeOK`, and an exported behaviour with two inserted stuttering steps keeps the same verdict. | Test (TC-1972) |
| FR-813-AC-4 | Weak fairness on `deliver` exports `WF_vars(op_deliver)`; a clause required by a two-role protocol with one `fifo` channel of capacity 2 exports channel and role variables and send and receive actions, `preserved` (FR-813-AC-12 to FR-813-AC-16 state the text). | Test (TC-1972) |
| FR-813-AC-5 | The Lean file of AC-1 and AC-2 contains no `theorem`, no proof term and no `sorry`. | Test (TC-1972) |
| FR-813-AC-6 | `eventually always holds(c.value <= 5)` exports to TLA+ directly as `THEOREM Th_T == Spec => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ <>[](v_Counter[o_c].fld_value <= 5)` with no monitor variable, `preserved`. `eventually always (holds(c.value <= 5) since holds(c.value = 0))`, whose operand is not a state formula, exports with the history variable `m_T_h0` of the `since` and `<>[]` over it, `preserved`. A clause with no deterministic Büchi or co-Büchi monitor exports to TLA+ `conditional` with the condition `{"type": "nondeterministic-monitor", "clause": "T", "node": N}`, and to Lean `preserved`. | Test (TC-1972) |
| FR-813-AC-7 | `holds(c.value >= 1) until holds(c.value >= 3)` with the counter's snapshot exports `m_T \in [Ids_Counter -> {"run", "acc", "rej"}]`, an `Init_T` and `Mon_T` that each define `m_T` by `[o_c \in Ids_Counter \|-> …]`, `accept_T(o_c) == m_T[o_c] = "acc"` and `THEOREM Th_T == Spec_T => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ []<>accept_T(o_c)`, and no `DOMAIN v_Counter` in `Init_T`, `Mon_T`, `TypeOK_T` or `accept_T`. Without a snapshot the same clause exports `CONSTANT Keys_Counter`, `ASSUME Keys_Counter \subseteq Seq(Nat) /\ IsFiniteSet(Keys_Counter)`, the `TypeOK` conjunct `DOMAIN v_Counter \subseteq Keys_Counter` and `Ids_Counter == Keys_Counter`, and its TLA+ record is `conditional` with exactly the condition `{"type": "bounded-identity-universe", "type_name": "Counter", "node": N}`, `N` the `Counter` object type node, while its Lean record stays `preserved`; no text of either export holds `Seq(Nat)` as a quantifier range or function domain. A clause over `(n: Int[0, 2])` quantifies `\A o_n \in 0..2 :` and its `ok_T` holds no presence conjunct. | Test (TC-1972) |
| FR-813-AC-8 | With `A1(o_c)` the text of `holds(c.value = 1)` at `o_c` and `A2(o_c)` that of `holds(c.value >= 2)`, `always (holds(c.value >= 2) implies once[1, 2] holds(c.value = 1))` exports to TLA+ exactly: `vars_T == <<vars, m_T_r0>>`; `TypeOK_T` conjunct `m_T_r0 \in [Ids_Counter -> [1..2 -> BOOLEAN]]`; `Init_T` conjunct `m_T_r0 = [o_c \in Ids_Counter \|-> [k \in 1..2 \|-> FALSE]]`; `Mon_T` conjunct `m_T_r0' = [o_c \in Ids_Counter \|-> [k \in 1..2 \|-> IF k = 1 THEN A1(o_c) ELSE m_T_r0[o_c][k - 1]]]`; `Next_T == (Next \/ (~ENABLED Next /\ UNCHANGED vars)) /\ Mon_T`; `Spec_T == Init_T /\ [][Next_T]_vars_T /\ WF_vars_T(Next_T)`; `THEOREM Th_T == Spec_T => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ [](A2(o_c) => (\E k \in 1..2 : m_T_r0[o_c][k]))`; `preserved`. `once[1, 2] not holds(c.value = 1)` initializes its cells `TRUE`. On the behaviour `value` 0, 1, 2, 3, 4 the body holds at positions 0 to 3 and fails at position 4, whose offsets 1 and 2 read `value` 3 and 2; `once[0, 2]` reads `A1(o_c) \/ \E k \in 1..2 : m_T_r0[o_c][k]`. | Test (TC-1972) |
| FR-813-AC-9 | With `A1`, `A1p` (the same text over primed variables) and `A2` as in AC-8, `always (holds(c.value >= 2) implies once[1, *] holds(c.value = 1))` exports to TLA+ exactly: `TypeOK_T` conjunct `m_T_c0 \in [Ids_Counter -> -1..1]`; `Init_T` conjunct `m_T_c0 = [o_c \in Ids_Counter \|-> IF A1(o_c) THEN 0 ELSE -1]`; `Mon_T` conjunct `m_T_c0' = [o_c \in Ids_Counter \|-> IF m_T_c0[o_c] = -1 THEN (IF A1p(o_c) THEN 0 ELSE -1) ELSE IF m_T_c0[o_c] < 1 THEN m_T_c0[o_c] + 1 ELSE 1]`; `THEOREM Th_T == Spec_T => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ [](A2(o_c) => m_T_c0[o_c] = 1)`; `preserved`. On the behaviour `value` 0, 1, 2 the counter reads -1, 0, 1 and the body holds at every position. `once[2, *] holds(c.value >= 1)` on `value` 0, 1, 2, 3 is true at position 3, because the operand first held two steps back; a counter of the steps since the operand last held reads 0 there and gives false, which this criterion rejects. | Test (TC-1972) |
| FR-813-AC-10 | `p since[1, 2] q` exports two registers of length 2, `m_T_r0` over `q` and `m_T_r1` over `p`, read as `\E d \in 1..2 : Q(d) /\ \A e \in 1..(d - 1) : P(e)`; `p since[2, *] q` exports the history Boolean `m_T_h0` of `p since q` and a register `m_T_r0` of length 2 over it with `Init` cells `FALSE`, read as `m_T_r0[o_c][2]`. In `once[1, 1] (holds(c.value = 1) since holds(c.value = 0))` the `Mon_T` conjunct of the inner `m_T_h1` precedes that of the outer `m_T_r0`, and the cells of `m_T_r0` initialize to `FALSE`, the empty-range value of the inner `since`. | Test (TC-1972) |
| FR-813-AC-11 | Every monitor and history variable advances at a terminal state: the counter behaviour from `value` 0 reaches its terminal state at `value` 5 at position 5, so `eventually (holds(c.value = 5) and once[1, 1] holds(c.value = 5))` holds there only at the terminal stutter position 6, and its export with `Next_T == (Next \/ (~ENABLED Next /\ UNCHANGED vars)) /\ Mon_T` states it true. An export whose `Next_T` omits the `~ENABLED Next` disjunct never shifts the register at the terminal state, states the clause false and fails this criterion. | Test (TC-1972) |
| FR-813-AC-12 | Protocol `Hand` over `(k: Counter::Counter)` `on origin`, with roles `a` and `b` on the counter model, a `fifo` channel `ch` from `a` to `b` carrying `Boolean` with FIFO key `true`, delivery `exactly-once-premise`, capacity 1, overflow `block`, `run sequence Main { send Snd via ch as (m) { m }; receive Rcv via ch of Snd as (r) { r } }` and `finish Done`, requiring the temporal clause `T` (`always holds(c.value <= 5)`) on an `origin` row, exported with the counter's snapshot and no protocol binding, exports to TLA+ the variables `pr_Hand_over`, `pr_Hand_last`, `pr_Hand_control`, `pr_Hand_observations`, `pr_Hand_bounds`, `pr_Hand_queues`, `pr_Hand_roles` and `pr_Hand_memory` beside `v_Counter`; `PInit_Hand` holds the conjuncts `pr_Hand_over \in Ids_Counter`, `pr_Hand_last = "init"`, `pr_Hand_control = ([type \|-> "protocol-instance", ordinal \|-> 0] :> [type \|-> "instance-state", finished \|-> FALSE]) @@ ([type \|-> "thread-path", instance \|-> 0, steps \|-> <<>>] :> PK_At("Hand::Main::Snd"))`, `pr_Hand_observations = <<>>`, `pr_Hand_bounds = <<>>`, `pr_Hand_queues = <<>>`, `pr_Hand_memory = <<>>` and `pr_Hand_roles` with exactly the two keys `[type \|-> "role", instance \|-> 0, role \|-> "a"]` and `[type \|-> "role", instance \|-> 0, role \|-> "b"]`, each mapped to `[type \|-> "model-binding", selection \|-> S]` with `S` the counter's `model_root` node identity; `PTypeOK_Hand` with the conjunct `\A x \in DOMAIN pr_Hand_queues : Len(pr_Hand_queues[x]) <= 1`; the actions `s_Hand_Hand_N_Main_N_Snd(t, w_m)`, `s_Hand_Hand_N_Main_N_Rcv(t, q)` and `s_Hand_Hand_N_Done(t, w_d)`; `THEOREM Th_T == PSpec_Hand => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ [](v_Counter[o_c].fld_value <= 5)`; no `Next` disjunct of the model's own operations; `preserved` in both targets. With a protocol binding whose `over` is the key `<<99>>`, `PInit_Hand` holds `pr_Hand_over = <<99>>` in place of `pr_Hand_over \in Ids_Counter`. | Test (TC-1972) |
| FR-813-AC-13 | The send action of AC-12 is exactly: `s_Hand_Hand_N_Main_N_Snd(t, w_m) ==` with the conjuncts `t \in DOMAIN pr_Hand_control`; `pr_Hand_control[t] = PK_At("Hand::Main::Snd")`; `w_m \in BOOLEAN`; `w_m`; `LET q == [type \|-> "queue", instance \|-> t.instance, channel \|-> "Hand::ch", fifo_key \|-> TRUE] o == [type \|-> "message-ordinal", node \|-> "Hand::Main::Snd", thread \|-> t] m == [type \|-> "message-occurrence", instance \|-> t.instance, node \|-> "Hand::Main::Snd", path \|-> t, ordinal \|-> PK_Get(pr_Hand_bounds, o, 0)] n == [type \|-> "next-delivery", message \|-> m] e == [type \|-> "queue-entry", message \|-> m, delivery \|-> PK_Get(pr_Hand_bounds, n, 0), payload \|-> w_m] IN /\ Len(PK_Get(pr_Hand_queues, q, <<>>)) < 1 /\ pr_Hand_queues' = PK_Put(pr_Hand_queues, q, Append(PK_Get(pr_Hand_queues, q, <<>>), e)) /\ pr_Hand_bounds' = PK_Put(PK_Put(pr_Hand_bounds, n, PK_Get(pr_Hand_bounds, n, 0) + 1), o, PK_Get(pr_Hand_bounds, o, 0) + 1)`; `pr_Hand_control' = [pr_Hand_control EXCEPT ![t] = PK_At("Hand::Main::Rcv")]`; `pr_Hand_last' = "send"`; `UNCHANGED <<v_Counter, pr_Hand_over, pr_Hand_observations, pr_Hand_roles, pr_Hand_memory>>`; the binder `m` writes no `observations` entry, because no node reads it. After the send the state's `pr_Hand_queues` and `pr_Hand_bounds` hold exactly FR-425-AC-15's entry and bounds (delivery 0, `message-ordinal` 1, `next-delivery` 1). | Test (TC-1972) |
| FR-813-AC-14 | Guard: the receive action of AC-12 holds the conjunct `LET w_r == Head(pr_Hand_queues[q]).payload IN w_r`. With the send constraint `{ true }`, the state whose queue holds one entry with payload `FALSE` enables no `s_Hand_Hand_N_Main_N_Rcv` instance and the thread waits at `Rcv`, as FR-426 makes the receive wait; with payload `TRUE` the receive removes the entry, deletes the queue's key from `pr_Hand_queues` and rests the root at `PK_At("Hand::Done")`. | Test (TC-1972) |
| FR-813-AC-15 | Choice: a variant whose `Rcv` is followed by `choice C { case yes when r: attempt Inc by a on Counter::Counter::inc …; case no when not r: … }` exports no action for `C`; the receive action sets the thread's rest point by `CASE w_r -> PK_At("Hand::Main::C::yes::Inc") [] ~w_r -> PK_At("Hand::Main::C::no::<first node>")`, and writes the binder entry `[type \|-> "binder", node \|-> "Hand::Main::Rcv", thread \|-> t]` only when a node reachable from that rest point reads `r`. | Test (TC-1972) |
| FR-813-AC-16 | Overflow: with capacity 1 and a queue already holding one entry, the send's queue update is, for `reject`, `pr_Hand_queues' = IF Len(c) < 1 THEN PK_Put(pr_Hand_queues, q, Append(c, e)) ELSE pr_Hand_queues`; for `loss head`, `PK_Put(pr_Hand_queues, q, IF Len(c) < 1 THEN Append(c, e) ELSE Append(Tail(c), e))`; for `loss tail`, `… ELSE Append(SubSeq(c, 1, Len(c) - 1), e)`; for `loss new`, as `reject`; with `c` the text `PK_Get(pr_Hand_queues, q, <<>>)`. Under every policy the bounds update of AC-13 is unchanged, so a rejected copy takes delivery index 0 and the next send's entry has delivery 1. Under `block` the full queue enables no send. The same channel with ordering `unordered` and `loss head` keeps the queue as a sequence in send order with `fifo_key` `PK_Null` and the same `loss head` update, and its receive takes any element; with `unordered` and `reject` the queue is a set and the receive takes any member. | Test (TC-1972) |
| FR-813-AC-17 | A clause that no protocol's activation relation requires is exported over its model subject with FR-812's `Spec`; the same clause required by `Hand` on an `origin` row is exported over `PSpec_Hand`; required by two protocols on `origin` rows it is stated by one theorem per row in row order; required on an `each` row it stops the export with FR-817's construct refusal naming the activation node, exit `21`. | Test (TC-1972) |
| FR-813-AC-18 | A timed model `Lamp` (`time dense unit ms`, object type `Lamp` with `on: Boolean` and clock `x`, operation `release` with guard `self.on and self.x >= 2 ms`, time invariant `when self.on { self.x <= 5 ms }`) exports `EXTENDS` holding `Reals`, the variables `now`, `tick`, `idle` and `k_Lamp_x`, an action `Tick(d)` whose conjuncts include `~idle`, `d \in Real`, `d > 0`, `~Urgent` and `\A k \in DOMAIN v_Lamp : v_Lamp[k].fld_on => k_Lamp_x[k] + d <= 5`, `Quiescent == ~Urgent /\ ~(\E k \in DOMAIN v_Lamp : v_Lamp[k].fld_on)`, an action `Idle(d)` whose conjuncts include `Quiescent`, `d > 0`, `tick' = FALSE` and `idle' = TRUE`, `release`'s action with `~idle`, `k_Lamp_x[r_self] >= 2`, `now' = now` and `tick' = FALSE`, and `NZ == \A r \in Real : <>(now > r)`. The clause `always holds(l.on or not l.on)` over `(l: Lamp::Lamp)` exports to TLA+ `conditional` with exactly one condition `{"type": "real-valued-time", "clause": "T", "node": N}`, `N` the `Lamp` timed model root, and to Lean `preserved` with no condition. | Test (TC-1972) |
| FR-813-AC-19 | With `B0(o_l)` the text of `holds(not l.on)` at `o_l` and `B0p(o_l)` the same text over primed variables, `eventually[0 ms, 5 ms] holds(not l.on)` over the `Lamp` of AC-18 exports to TLA+ exactly: `Init_T` conjunct `m_T = [o_l \in Ids_Lamp \|-> IF (0 <= 0 /\ 0 <= 5) /\ B0(o_l) THEN "acc" ELSE IF 0 > 5 THEN "rej" ELSE "run"]`; `Mon_T` conjunct `m_T' = [o_l \in Ids_Lamp \|-> IF m_T[o_l] /= "run" THEN m_T[o_l] ELSE IF (0 <= now' /\ now' <= 5) /\ B0p(o_l) THEN "acc" ELSE IF now' > 5 THEN "rej" ELSE "run"]`; `Next_T == ((Next \/ \E d \in Real : Idle(d)) /\ Mon_T) \/ (\E d \in Real : Tick(d) /\ UNCHANGED <<m_T>>)`; `THEOREM Th_T == Spec_T => (NZ => \A o_l \in Ids_Lamp : [](ok_T(o_l)) /\ []<>accept_T(o_l))`; and to Lean `holds_T` as `(exists d1 : Int, 0 <= d1 /\ (0 <= (i + d1) /\ Q_le (Q.mk 0 1) (Q_sub (b.ts (Int.toNat (i + d1))) (b.ts (Int.toNat i))) /\ Q_le (Q_sub (b.ts (Int.toNat (i + d1))) (b.ts (Int.toNat i))) (Q.mk 5 1) /\ at_T_0 (b.st (Int.toNat (i + d1))) o_l))`. A `Tick` step leaves `m_T` unchanged, so a behaviour whose delays alone pass 5 ms without a `release` leaves `m_T` at `"run"` until its next discrete step reads `now' > 5` and rejects. | Test (TC-1972) |
| FR-813-AC-20 | The TLA+ export of a clause over a hybrid model `Braking` keeps FR-811-AC-6's `unrepresented` record with one `target-unrepresentable` cause per flow-equation node and adds no `real-valued-time` condition. | Test (TC-1972) |
| FR-813-AC-21 | Over the counter with an `initial` snapshot at `value` 4, whose only behaviour is `value` 4, then 5, then terminal stutter from position 1: `always[2, 3] holds(c.value = 5)` and `eventually[2, 3] holds(c.value = 5)` are true and `eventually[2, 3] holds(c.value = 4)` is false in FSL. The TLA+ exports carry `Next_T == (Next \/ (~ENABLED Next /\ UNCHANGED vars)) /\ Mon_T`, `Spec_T == Init_T /\ [][Next_T]_vars_T /\ WF_vars_T(Next_T)` and a step counter `n_T_0 \in 0..3`; TLC run as an observer reports `holds`, `holds` and `violated` for the three; the Lean `holds_T` of each at position 0 over the `Behaviour` 4, 5, 5, 5, … is true, true and false. An export without the terminal disjunct or without its weak fairness leaves `n_T_0` at 1, so its `eventually[2, 3] holds(c.value = 5)` export reports `violated`, which this criterion rejects. A clause `always[0, 3] holds(c.value >= 4) and eventually[1, 2] holds(c.value = 5)` exports two counters, `n_T_0 \in 0..3` and `n_T_1 \in 0..2`. | Test (TC-1972) |
| FR-813-AC-22 | `fair weak each Counter::Counter::inc; always eventually holds(c.value = 2)` exports `Spec_T == Init /\ [][Next]_vars /\ WF_vars(Next) /\ \A r_self \in Ids_Counter : WF_vars(op_inc(r_self))`, and the monitored `fair weak each Counter::Counter::inc; always eventually (holds(c.value = 2) since holds(c.value = 1))` exports `Spec_T == Init_T /\ [][Next_T]_vars_T /\ WF_vars_T(Next_T) /\ \A r_self \in Ids_Counter : WF_vars(op_inc(r_self))`; no `Spec_T` of either drops the fairness conjunct. No TLA+ text of a temporal clause in the module holds `DOMAIN v_` outside FR-812's `Next`, `TypeOK`, an action body, a state clause and `ok_<clause>`. | Test (TC-1972) |
| FR-813-AC-23 | Unbounded and `[a,*]` past operators start from the empty range: `historically holds(c.value = 1)` exports `Init` `m_T_h0 = [o_c \in Ids_Counter \|-> A1(o_c)]` and is false at position 0 of the counter from `value` 0; `historically[1, *] holds(c.value = 1)` is true at position 0 (its range is empty) in both targets, and its Lean text bounds the offset by `0 <= i - d`; `historically[1, 1] holds(c.value = 1)` is false at position 0, because a closed interval reads the pre-origin extension. An export that reads the pre-origin extension for an unbounded or `[a,*]` operator makes the second case false and fails this criterion. | Test (TC-1972) |
| FR-813-AC-24 | Each construct of "Constructs not lowered", in a build that does not lower it, stops the export with FR-817's `unimplemented_capability`/`unimplemented-meaning` refusal naming that construct, exit `21`, with no package and no file, and never with exit `30`. | Test (TC-1972) |
| FR-813-AC-25 | A counter variant with an operation `drop` that removes its receiver, its snapshot holding `<<99>>` at `value` 0: `always not holds(c.value > 5)` is refuted in FSL with `undefined-evaluation`, reason `absent-key`, at the position after `drop`. Its TLA+ export is `THEOREM Th_T == Spec => \A o_c \in Ids_Counter : [](ok_T(o_c)) /\ [](~(v_Counter[o_c].fld_value > 5))`, which TLC reports `violated` on the behaviour `drop`, and its Lean `Ob_T` is false, the behaviour that takes `drop` at step 1 failing `ok_T (b.st 1) [99]`. An export that makes only the atom false at an absent object (an atom-level presence conjunct) states the clause true after `drop` and fails this criterion. | Test (TC-1972) |
| FR-813-AC-26 | The counter exported without a snapshot and with the model checker's configuration setting `Keys_Counter` to `{<<1>>, <<2>>}`: `Ids_Counter == Keys_Counter`, and the initial state that holds only `<<1>>` gives a behaviour on which `[](ok_T(<<2>>))` is false, so `always holds(c.value <= 5)` is reported `violated` by TLC, as FSL refutes instance `<<2>>` (admitted by another initial state) on that behaviour; the Lean `Ob_T` is false for the same behaviour. An export that guards each instance by its presence in the behaviour's own initial state states the clause true and fails this criterion. | Test (TC-1972) |
| FR-813-AC-27 | The `Lamp` of AC-18 with snapshot `on` false at time 0 is quiescent. `always[2 ms, 3 ms] holds(l.on)` is false in FSL, on the behaviour whose idle tail has a stutter position after a delay of 2 ms. Its TLA+ export reads that position through `Idle(2)` and reports it `violated`; its Lean `Ob_T` is false for the `TBehaviour` with `ts` 0, 2, 4, … and `st` constant. An export with no `Idle` action, in which delays at a quiescent state are `Tick` steps only, never reads a position in the window and states the clause true, which this criterion rejects. | Test (TC-1972) |
| FR-813-AC-28 | Protocol `Two` over `(k: Counter::Counter)` `on origin` with `run sequence Main { attempt A by w on Counter::Counter::inc …; parallel P { branch x attempt B by w on Counter::Counter::inc …; } join all [x]; }` and `finish Done`, from `value` 0, has positions 0 (`value` 0), 1 (`attempt` A, `value` 1), 2 (`fork`), 3 (`attempt` B, `value` 2), 4 (`join`), 5 (`finish`), then terminal stutter, at distances 0, 1, 1, 2, 2, 2, 3. With `A1`, `A1p` and `A2` as in AC-8, `always (holds(c.value = 2) implies once[1, 1] holds(c.value = 1))` exports `m_T_r0 \in [Ids_Counter -> [0..1 -> BOOLEAN]]`, `Init_T` conjunct `m_T_r0 = [o_c \in Ids_Counter \|-> [k \in 0..1 \|-> IF k = 0 THEN A1(o_c) ELSE FALSE]]`, `Mon_T` conjunct `m_T_r0' = [o_c \in Ids_Counter \|-> IF PK_Counted(pr_Two_last') THEN [k \in 0..1 \|-> IF k = 0 THEN A1p(o_c) ELSE m_T_r0[o_c][k - 1]] ELSE [m_T_r0[o_c] EXCEPT ![0] = @ \/ A1p(o_c)]]` and the body `A2(o_c) => m_T_r0[o_c][1]`; its cells (0, 1) read (F, F), (T, F), (T, F), (F, T), (F, T), (F, T) at positions 0 to 5, so the clause holds, as in FSL. `eventually[2, 2] holds(c.value = 2)` is true, its counter `n_T_0` reading 0, 1, 1, 2, 2, 2 at positions 0 to 5. A register that shifts on every step reads `value` 2 in cell 1 at position 5 and a counter that advances on every step reaches 2 at position 2 (`value` 1); each makes its clause false and fails this criterion. | Test (TC-1972) |

## Dependencies

- [FR-651](FR-651-generate-tla-and-lean-output.md),
  [FR-812](FR-812-map-state-actions-and-values-to-tla-and-lean.md);
  [FR-425](../protocol/FR-425-define-protocol-subjects-and-state-keys.md) owns
  protocol subjects and their state, [FR-426](../protocol/FR-426-apply-protocol-steps.md)
  their steps, [FR-431](../protocol/FR-431-count-protocol-steps-for-intervals.md)
  their positions and counted steps, FR-464 the activation relation;
  [FR-092](../temporal/FR-092-evaluate-bounded-past.md) the past reading;
  [FR-361](../temporal/FR-361-read-model-behaviours-as-temporal-traces.md)
  instances and FR-363 undefined evaluation;
  [FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md),
  [FR-416](../temporal/FR-416-evaluate-timed-temporal-claims.md) and FR-161
  dense time and its idle tail; FR-377 the terminal stutter.
