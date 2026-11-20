---
id: FR-652
title: "Check hybrid enclosure and validated-witness certificates"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-193
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-331
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-481
    type: references
  - target: ix://agent-ix/filament-specification/FR-416
    type: references
---

# FR-652: Check hybrid enclosure and validated-witness certificates

## Description

A hybrid provider MAY return, with a `proved` entry or with a trace, a
certificate stated in the names of the source `hybrid-decl`. When the reader
that holds the checked hybrid model receives one, it SHALL check it in exact
rational interval arithmetic, and SHALL use an accepted certificate to settle
the proof `certified` or to refute through a mode whose flow is not
polynomial.

## Inputs

A hybrid item, its checked hybrid model and claim (FR-481), the provider's
FR-193 result entry, the entry's optional `certificate` member, the trace a
`validated-witness` accompanies, and the item's work limit.

## Outputs

The certificate outcome `accepted` or `rejected{rule, at}`, a refusal of a
certificate the reader cannot read, and the item's settlement as below.

## Behavior

### The provider contract

A hybrid provider is a registered backend whose descriptor's `dynamics` holds
`hybrid` (FR-193 "Routing a hybrid item"). It receives one FR-331 request
item per hybrid claim and returns one FR-193 result entry per item. Its
entry MAY carry `certificate`, one of the three forms below; an entry without
one is settled exactly as FR-193 states. The certificate is checked by the
reader, never by the provider, and a provider's word about its own soundness
changes no settlement.

### Boxes and steps

- A *box* maps each continuous variable of the declaration to an FR-331
  `Interval` of exact rationals, and denotes the states whose every variable
  lies in its interval. The *pointer* of a box or step is its RFC 6901
  pointer in the certificate, rooted at `/certificate`, such as
  `/certificate/boxes/Moving`, `/certificate/steps/3/apriori` or
  `/certificate/dwells/0/7`.
- A *step* is `{mode, time, box, apriori, step}`, and a flowpipe jump step
  also carries `from` (below). `mode` names a mode, `time` is an FR-331
  `Interval` of seconds and `step`, written `h`, is an FR-331 `Rational`
  number of seconds, every exact number in the object form FR-193 "Solver
  result on the wire" requires. Each step has *start instants* `S`, an
  interval, and *covered instants* `C = [S.lower, S.upper + h]`. Its `box`
  encloses the state of every behaviour it carries at that behaviour's start
  instant in `S`, and its `apriori` box, written `B`, encloses that
  behaviour's state at every instant from its start instant to `h` later, so
  `B` holds at every instant of `C`.

### Certificate forms

A `certificate` member SHALL be one of the three forms, each a closed object
with a `type` member.

1. **`inductive-enclosure`**, `{"type": "inductive-enclosure", "boxes":
   {mode: box}}`, for an `inductive-solver` proof: one box `E_m` per mode
   `m`, claiming that every reachable state in mode `m`, at every time and
   after any number of jumps, lies in `E_m`.
2. **`flowpipe`**, `{"type": "flowpipe", "steps": [step, ...]}`, for a
   `bounded-solver` proof: a tree of dwells.
   - **Dwells.** The steps are split, in list order, into *dwells*, each a
     maximal run of consecutive steps. A dwell starts at `steps[0]`, at every
     step that carries `from`, and at every step without `from` whose mode
     differs from the previous step's mode. A dwell whose first step carries
     `from` is a *jump dwell* and that step is its *jump step*; any other
     dwell is an *initial dwell*. Dwells are numbered from 0 in list order,
     and every step of a dwell has the mode of its first step.
   - **Jump steps.** A jump step's `from`, `{"dwell": i, "transition": t}`,
     names the earlier dwell `i` the jumps leave and the zero-based position
     `t` of the transition they take among dwell `i`'s mode's transitions, as
     a `HybridTrace` jump names a transition. The jump step carries every jump
     by transition `t` from dwell `i`, so each pair `(i, t)` has at most one
     jump step.
   - **The tree.** The roots are the initial dwells, and each jump dwell is a
     child of the dwell its `from` names. A dwell has one child for each
     transition that can fire from it, so a dwell may have several children,
     a self-loop gives a child in the dwell's own mode, and two dwells in one
     mode are told apart by their place in the tree. The *depth* of an
     initial dwell is 0, and the depth of a jump dwell is one more than the
     depth of the dwell it leaves; a behaviour in a dwell of depth `k` has
     taken `k` jumps.
   - **Instants.** For the first step of an initial dwell, `S` is `[0, 0]`
     and `time` SHALL be `[0, h]`. For a jump step, `S` is its `time`, the
     instants of the jumps it carries, and `C` is
     `[time.lower, time.upper + h]`. For every other step, `S` is the
     previous step's `S` shifted by the previous step's `h`, and `time` SHALL
     equal `C`. A jump step's `time` is therefore its start instants, every
     other step's `time` is its covered instants, and `time.upper -
     time.lower = h` holds only for a step whose `S` is a single instant.
   - **Dwell endings.** For the last step `L` of a dwell, with `A` its
     advance and `Inv` its mode's invariant (below): the dwell *reaches the
     horizon* `H` when `S_L.lower + h_L >= H`; it *empties* when
     `Contract(Inv, A)` is empty, so no behaviour remains in the dwell after
     `L`; and it *closes* when `Contract(Inv, A) ⊆ box_L`. A dwell that
     closes holds every later state of its behaviours in `B_L`, because the
     flow is autonomous and each later step would repeat `L` from a box
     inside `box_L`; for such a dwell, `C` of its last step is taken to be
     `[S_L.lower, ∞)` wherever this requirement reads covered instants.
3. **`validated-witness`**, `{"type": "validated-witness", "dwells":
   [[step, ...], ...]}`, with a `HybridCounterexample` or a
   `solver-inconclusive` trace: one list of Picard steps per *state* of the
   trace, state 0 the trace's `start` and state `d` the state `jumps[d - 1]`
   reaches, each step in that state's mode. The start point is the trace's
   `start`. A witness step's `S` is the single instant `time.lower` and its
   `C` is its `time`. The first step of dwell 0 starts at 0, each later step
   of a dwell starts where the previous one ends, and a dwell that ends in a
   jump ends with a *witness jump step* of `step` 0 and `time` `[τ, τ]`, `τ`
   the *jump instant*, at which the next dwell's first step starts.

### Interval reading

- **Evaluation.** The reader SHALL evaluate every expression over a box by
  interval arithmetic over exact rationals, with no floating-point step,
  after `let` substitution and expansion of non-recursive calls. A
  comparison or Boolean expression reads, over a box, *definitely true* when
  every state of the box makes it true, *definitely false* when none does,
  and *unknown* otherwise; `not`, `and`, `or` and `implies` combine these
  three values by Kleene's rules (`implies` is `not a or b`), so `not`
  swaps true and false and keeps unknown.
- **Definedness.** An expression is *definitely defined* over a box when it
  has a value at every state of the box, and *definitely without value*
  when it has none at any state of the box, by FR-193's rules: a division
  whose divisor can be 0, and a call argument or result that can leave its
  parameter's or result type's value range, make it not definitely defined;
  a divisor equal to `[0, 0]`, or a value disjoint from the range, makes it
  definitely without value; an `if` considers only the branches its
  condition does not exclude. FR-193 "Continuous values and expressions"
  makes a state at which an expression has no value a violating state of
  every claim, so wherever a rule below requires an expression definitely
  defined over a box, a box over which it is not fails that rule.
- **Contractor.** `Contract(c, X)` narrows the box `X` by the condition `c`
  in one pass over `c`'s top-level conjuncts in source order, each conjunct
  using the box as narrowed so far. A conjunct whose one side is a bare
  continuous variable `x` and whose other side is an expression `e`, with
  `[el, eu]` the value of `e` over the box: `x = e` narrows `x` to its meet
  with `[el, eu]`; `x <= e` and `x < e` lower `x`'s upper bound to `eu` when
  that is smaller; `x >= e` and `x > e` raise `x`'s lower bound to `el` when
  that is larger; the mirrored forms (`e >= x` and so on) narrow alike. A
  `!=` conjunct and every other conjunct form narrow nothing. A meet that
  is empty makes the result *empty*: the condition cannot hold at any state
  of `X`. `Hull(c)` is `Contract(c, X)` with `X` the box whose every
  variable is unbounded; a hull that leaves a variable unbounded lies in no
  certificate box.
- **Guards.** The *guarded part* of a box `X` for a transition is
  `G = Contract(guard, X)`. The *post image* `Post(G)` is the reset's image
  of `G`, contracted by the target mode's invariant: a variable with an `=`
  conjunct `x = e` takes the value of `e` over `G`; a variable with only
  other conjuncts takes the meet of the half-lines they bound (`x < e` gives
  `(-∞, eu]`, `x != e` the whole line); a variable no conjunct names keeps
  its interval in `G`; every `e` reads pre-jump values over `G` (FR-193
  "Resets"); and the result is then `Contract(Inv_n, ·)` for the target mode
  `n`. A transition *can fire* from `X` when `G` is not empty, the guard is
  not definitely false over `G`, and `Post(G)` is not empty; this is FR-193's
  enabling condition read over a box, so every behaviour that may jump from
  `X` by the transition is counted.
- **Tube and advance.** For a step with rates `F`, the interval value of
  each rate over `B`, the *tube* is `box + [0, h] * F` and the *advance* is
  `box + h * F`, per variable, with `[0, h] * [a, b] = [min(0, h a),
  max(0, h b)]`. The tube encloses every state of a behaviour over the step
  while that state lies in `B`, and the advance encloses its state `h`
  after its start instant.
- **Inclusion.** `X ⊆ Y` holds when every interval of `X` lies in the
  interval of `Y` for the same variable; an empty box lies in every box.
  Every rule below states the direction of its inclusion.
- **Boundary face.** The upper face `x_i = u_i` of a box `E` is a *boundary
  face* of an invariant when the invariant has a top-level conjunct
  `x_i <= e`, `x_i < e` or `x_i = e`, or a mirrored form, with `e` free of
  conditionals and reading no continuous variable other than those whose
  rate in the mode is the constant 0, and `u_i >=` the upper bound of `e`
  over `E`; a lower face
  `x_i = l_i` is one when it has `x_i >= e`, `x_i > e` or `x_i = e` with
  `l_i <=` the lower bound of `e` over `E`. Beyond a boundary face that
  conjunct is definitely false, so no behaviour of the mode crosses it: `e`
  reads only variables the flow holds fixed, so its bound over `E` holds
  however the other variables move. An `e` that reads a moving variable, as
  `y` in the invariant `x = y` under `x' = y' = 1`, moves with the
  behaviour, and its face is no boundary face. This
  is decided from the invariant's variable-bound conjuncts by interval
  reasoning alone; the reader never probes a point beyond a face.

**Why contraction is sound.** The contractor removes from a box only states
that falsify a conjunct of the condition: for `x <= e`, a removed state has
`x` above the upper bound of `e` over the box, so above `e` at that state.
A behaviour satisfies its mode's invariant at every instant of its dwell
(FR-193 "Behaviours and positions"), so contracting by the invariant
removes no state a behaviour holds inside the dwell, and contracting by a
guard removes no state at which the guard holds; once `B` encloses a
behaviour over a step, contracting its `box` and its advance is therefore
sound. The Picard inclusion needs more, because it is what makes `B` an
enclosure. A mode is *contraction-admitted* when, after expansion, its
rates and the bounds `e` of its invariant's contracting conjuncts (the
variable-bound conjuncts the contractor reads) contain no conditional and
no recursive call. For such a mode the reader checks
`Contract(Inv, tube) ⊆ B`, and the argument is this. A behaviour follows a
classical solution of the flow (FR-193 "Behaviours and positions"). Let
`t*` be the last instant up to which its state over the step stays in `B`.
Up to `t*` the state is its start state plus the integral of the flow over
states in `B`, so it lies in the tube and, satisfying the invariant, in
`Contract(Inv, tube) ⊆ B`. Suppose it leaves `B` just after `t*` through a
face of variable `x_i`. If the contraction narrowed that face, from a
conjunct `x_i <= e` (or its mirror), the behaviour keeps `x_i <= e` at every
instant, and `e` is at most `B`'s bound at every state of the tube. If it
did not, `B`'s bound is at least the tube's; where it is greater there is
strict room and `x_i` cannot reach the face just after `t*`, so it can
leave only at a face where `B`'s bound equals the tube's, which needs its
rate to reach 0 there. In both cases the excess is bounded by a rate or a
bound `e` read just outside `B`. Rates and bounds free of conditionals and
definitely defined over `B` are Lipschitz near `B`, so the distance `d(t)`
of the state beyond `B` satisfies `d(t) <= C * ∫ d` over `[t*, t]` for a
constant `C`, and `d = 0` by Grönwall's inequality: the behaviour never
leaves `B` within the step. A strict conjunct `x < e` is read as `x <= e`,
which only keeps more states. A conditional, or a recursive call, can make a
rate or a bound jump at a face of `B`, which breaks the Lipschitz step: with
`x' = 1`, `y' = x` and the invariant `x <= if y > 0 then 10 else 0`, the
contracted tube of the box `x [0, 0], y [0, 0]` lies in `B` `x [-1, 0],
y [-1, 0]` although `x = t`. For a mode that is not contraction-admitted
the reader SHALL therefore check the uncontracted inclusion `tube ⊆ B`, the
classical a-priori condition, and still contract `box` and the advance.
Contraction drops only states no behaviour reaches, and every state a
behaviour reaches stays enclosed. A `validated-witness` proves that a
behaviour exists rather than that every behaviour is enclosed, so its
inclusions are checked without contraction.

### Checking order

The reader SHALL check a certificate in this order, and SHALL report only
the first failure.

1. **Read.** The certificate is read by the FR-331 schema: every object is
   closed, every exact number is a `Rational` object, and `type` is one of
   the three forms. A schema failure refuses at the member (FR-193 "Solver
   result on the wire").
2. **Form.** A `bounded-solver` proof takes `flowpipe`, an `inductive-solver`
   proof takes `inductive-enclosure`, and a `refuted` entry, or an
   `inconclusive` entry whose `solver-inconclusive` cause carries a trace,
   takes `validated-witness`; the match reads the basis type, never
   `method`. Any other pairing refuses `invalid_result`/`invalid-value` at
   `/certificate`.
3. **Structure.** The structural refusals below.
4. **Rules.** The rules of the form's table, in table order, *rule by rule*:
   each rule is checked over every element it applies to, in the element
   order the table gives, before the next rule starts. The first failing
   rule and the element it failed at give the rejection `rejected{rule,
   at}`; a certificate is accepted when every rule passes.

The first three are refusals: the reader cannot read the certificate, so it
refuses the entry (`invalid_result` on the analyze path) or the request
(`invalid_runtime_input` on a replay request, at
`/replay/certificate/<member>`) and never reaches a rule or an internal
fault. A rejection is a certificate outcome, never a refusal. Every `at`
is a pointer into the certificate rooted at `/certificate`, on the analyze
path and on a replay request alike, for every rule including `c`,
`claim`, `coverage`, `jump` with no jump step and `work-limit`.

### Structural refusals

The reader is closed-world: every member and element of a certificate is
read by step 1 or 3 or examined by a rule of its form, and an element that
no rule examines is refused, never skipped. Each of these SHALL refuse
`invalid_result` on the analyze path, and `invalid_runtime_input` on a
replay request, at the pointer given:

| Mismatch | Code and pointer |
| --- | --- |
| a `boxes` key that names no mode | `invalid-value` at `/certificate/boxes/<key>` |
| a mode with no `boxes` key | `missing-member` at `/certificate/boxes/<mode>` |
| a step `mode` that names no mode | `invalid-value` at the step's `/mode` |
| a step whose `step` is negative | `invalid-value` at the step's `/step` |
| a step whose `time.lower` is negative or exceeds `time.upper` | `invalid-value` at the step's `/time` |
| an empty `steps` list, or `steps[0]` carrying `from` | `invalid-value` at `/certificate/steps` or `/certificate/steps/0/from` |
| a `from` whose `dwell` is not an earlier dwell | `invalid-value` at the step's `/from/dwell` |
| a `from` whose `transition` is not a transition position of that dwell's mode | `invalid-value` at the step's `/from/transition` |
| a second jump step with the same `from` | `invalid-value` at the second step's `/from` |
| a jump step whose mode is not its transition's target mode | `invalid-value` at the step's `/mode` |
| two initial dwells in one mode | `invalid-value` at the second dwell's first step's `/mode` |
| a witness on a trace with `loop` | `invalid-value` at `/certificate` |
| a witness whose `dwells` count differs from the trace's state count | `invalid-value` at `/certificate/dwells` |
| a witness step whose mode differs from its trace state's mode | `invalid-value` at the step's `/mode` |
| a witness step carrying `from` | `invalid-value` at the step's `/from` |

A structural refusal applies to every certificate the entry or request
carries, whether or not the settlement goes on to check it ("Settlement").

### Rules of `inductive-enclosure`

In each row, `m` ranges over the modes in declared order, `Inv_m` is `m`'s
invariant and the initial entries are read in source order.

| Order | Rule | Inputs | Passes when | `at` on failure |
| --- | --- | --- | --- | --- |
| 1 | `box-shape` | every `boxes` member, in declared mode order | its keys are exactly the declaration's continuous variables and every interval has `lower <= upper` | `/certificate/boxes/<m>` |
| 2 | `a` | each initial entry `(M, c)`; `Inv_M`; `E_M` | with `H = Hull(c and Inv_M)`: `H` is empty, or `c` and `Inv_M` are definitely defined over `H` and `H ⊆ E_M`, so every initial state lies in the box (never `E_M ⊆ H`) | `/certificate/boxes/<M>` |
| 3 | `b` | each mode `m`: `E_m`, its rates, `Inv_m` | every rate of `m` and `Inv_m` are definitely defined over `E_m`; and for each variable `x_i` in declaration order, lower face then upper face, the face being `E_m` with `x_i` fixed at that bound, the face is a boundary face of `Inv_m`, or the rate of `x_i` over the face is definitely `>= 0` on a lower face and definitely `<= 0` on an upper face | `/certificate/boxes/<m>` |
| 4 | `c` | each mode `m`; each transition from `m` to `n`, in position order | the guard is definitely defined over `E_m`; and, with `G = Contract(guard, E_m)`, when the transition can fire from `E_m`, the reset is definitely defined over `G` and `Post(G) ⊆ E_n` | definedness: `/certificate/boxes/<m>`; inclusion: `/certificate/boxes/<n>` |
| 5 | `d` | the claim; each `E_m` | the claim has a form rule (d) admits, and over `E_m` every claim atom, activation condition and capture initializer is definitely defined and every obligation's state formula is definitely true | a form (d) does not admit: `/certificate`; otherwise `/certificate/boxes/<m>` |
| 6 | `work-limit` | the item's work meter | the meter does not stop the check | the box being checked |

Rules (a) to (c) make the boxes an inductive invariant: (a) holds every
initial state, (b) keeps every behaviour inside its mode's box during a
dwell (a flow that points inward on every face, or a face beyond which the
invariant is false, leaves no way out), and (c) holds every post-jump
state. Rule (d) then holds the claim at every reachable state.

**Claim forms of rule (d).** Rule (d) admits a claim under `on origin`, or
under `on each` with any activation condition, with any fairness
constraints, whose body is an *obligation* `always φ` or `always I φ`, `I`
any interval, or a conjunction (`and`) of such obligations, `φ` a *state
formula*: `holds(e)` atoms combined by `not`, `and`, `or` and `implies`,
with no atom reading a capture. Each obligation's `φ` is read over each box
by the three-valued evaluation above, so `always not holds(h.v >= 30)` over
a box `v [40, 50]` reads false and is rejected, while `always not
holds(h.v < 30)` reads true. Because the boxes hold every reachable state
at every instant and every position, a state formula definitely true over
every box holds at every state every obligation reads, from every
activation, under `model-time` and `model-steps` alike; fairness only
removes behaviours, so ignoring it is sound. Any other claim form, such as
a temporal operator inside `φ`, is rejected `d` at `/certificate`.

### Rules of `flowpipe`

Steps are read in list order, dwells in their number order and transitions
in position order. `H`, `J` and `E` are the entry's `horizon`, `jumps` and
`exhaustive` (FR-193 "Proof coverage"); `Inv` is a step's mode's invariant.

| Order | Rule | Inputs | Passes when | `at` on failure |
| --- | --- | --- | --- | --- |
| 1 | `box-shape` | every step's `box`, then its `apriori` | as for `inductive-enclosure` | `/certificate/steps/<k>/box` or `/certificate/steps/<k>/apriori` |
| 2 | `initial` | the initial dwells; then each initial entry `(M, c)` | the first step of every initial dwell has `time` `[0, h]`; and for each entry, with `H_e = Hull(c and Inv_M)`: `H_e` is empty, or `c` and `Inv_M` are definitely defined over `H_e`, an initial dwell in mode `M` exists, and `H_e ⊆` its first step's `box` | that dwell's first step; `/certificate/steps/0` when no initial dwell has mode `M` |
| 3 | `picard` | each step `k`: `box`, `B`, `h`, the rates | every rate and `Inv` are definitely defined over `B`, and `Contract(Inv, tube) ⊆ B` for a contraction-admitted mode, `tube ⊆ B` for any other ("Why contraction is sound") | `/certificate/steps/<k>` |
| 4 | `successor` | each step `k` that is not the first step of its dwell, with the previous step `p` | `time` equals the `C` its `S` derives ("Instants"), and `Contract(Inv, advance of p) ⊆ box_k` | `/certificate/steps/<k>` |
| 5 | `jump` | each dwell `D` of depth `< J`; each transition `τ` of its mode; the steps `k` of `D` with `S_k.lower <= H` | see "Following transitions" | see "Following transitions" |
| 6 | `coverage` | each dwell `D`; `H`, `J`, `E` | see "Coverage" | see "Coverage" |
| 7 | `claim` | the claim; every step's readings | see "Reading the claim" | see "Reading the claim" |
| 8 | `work-limit` | the item's work meter | the meter does not stop the check | the step being checked |

**Following transitions (`jump`).** For a dwell `D` of depth less than `J`
and a transition `τ` of its mode to mode `n`, over the steps `k` of `D`
with `S_k.lower <= H`, the rule SHALL require, in this order:

1. the guard definitely defined over every such `B_k`, else `jump` at
   `/certificate/steps/<k>`;
2. with `G_k = Contract(guard, B_k)` and `K` the steps from whose `B_k` the
   transition can fire, the reset definitely defined over every `G_k` with
   `k` in `K`, else `jump` at `/certificate/steps/<k>`;
3. when `K` is not empty, a jump step `j` whose `from` is `(D, τ)`, else
   `jump` at the first step of `K`;
4. the hull of `Post(G_k)` over `K` included in `box_j` (every post-jump
   state lies in the target dwell's first box), else `jump` at
   `/certificate/steps/<j>`;
5. `time_j` containing the hull of `C_k` over `K` intersected with
   `[0, H]`, or, when the narrowed jump time `N` applies, containing `N`;
   else `jump` at `/certificate/steps/<j>`.

A jump after `H` lies outside the proof region, so intersecting with
`[0, H]` drops no state the proof must read. The *narrowed jump time*
applies when the guard has a top-level conjunct `x = c`, `c` a constant,
whose rate `r` for `x` in `D`'s mode reads only variables whose rate is the
constant 0 there, and the interval of `r` over `D`'s first box excludes 0.
Then every behaviour from `D`'s first box reaches `x = c` exactly once, at
`(c - x0) / r` after its start instant, and `N` is the interval value of
`(c - x) / r` over `D`'s first box, added to `D`'s first step's `S`, and
intersected with `[0, H]`. The reader SHALL accept a narrowed `time` exactly
when it contains `N`. When the source invariant has its boundary at the
guard (a *boundary invariant*, as `v >= 0` with the guard `v = 0`), a box
never holds states on both sides of the guard: the source dwell's boxes stop
at the guard by contraction, and the target dwell starts at the guard's
post image. When it does not, the source dwell continues past the guard, as
a behaviour that does not take the transition may. A transition that cannot
fire from any step needs no jump step, and a jump step whose transition
cannot fire is still read by `picard`, `successor`, `coverage` and `claim`,
which only widens what they read.

**Coverage.** The rule SHALL require, for each dwell `D` in order:

1. `depth(D) <= J`, else `coverage` at `D`'s first step;
2. `D` reaches the horizon, empties or closes ("Dwell endings"), else
   `coverage` at `D`'s last step;
3. when `E` holds `time` and `depth(D) = J`: no transition can fire from
   `B_k` for any step `k` of `D` with `S_k.lower <= H`, so no behaviour
   takes a jump beyond `J` by `H`; else `coverage` at the first such step;
4. when `E` holds `jumps` and `depth(D) < J`: `D` empties or closes, else
   `coverage` at its last step; and every step `k` of `D` from whose `B_k`
   some transition can fire has `C_k.upper <= H`, so no behaviour takes one
   of its first `J` jumps after `H` (the last step of a dwell that closes has
   no upper bound, so no transition may be able to fire from its `B`); else
   `coverage` at the first such step.

With the transitions followed and these four checks passed, the steps
enclose FR-193's proof region, every state a behaviour reaches by time `H`
with at most `J` jumps, and establish each dimension `E` lists. The
rule is incomplete in one stated way: a dwell that empties only on a step
whose covered instants pass `H` cannot show `jumps` at that horizon, so
such a proof stays `proved` and uncertified. FR-193's `Braking` example
with `horizon` 20, `jumps` 1 and `exhaustive` `["jumps"]` is one: its
`Moving` dwell empties only on a step whose covered instants pass 20, and
with `horizon` 21 the same certificate passes.

**Reading the claim.** A `flowpipe` reaches this rule only for a claim
FR-193 coverage lets the proof cover, an `on origin` claim with a horizon;
a claim of any other form is rejected `claim` at `/certificate`. Under
`model-time`:

- Each step gives, for each claim atom, a *reading* over its `B`: the
  atom's three-valued value over `B`, or unknown when the atom is not
  definitely defined over `B`, holding at every instant of `C`. A step from
  which a behaviour may end, because `Inv` is not definitely true over its
  uncontracted tube or it is the last step of a dwell that empties or
  closes (a *may-end* step), also holds its reading at every later instant,
  since FR-193 reads a finite maximal behaviour as repeating its last state.
  This test ignores whether a transition is enabled at the invariant's
  boundary, which would force the behaviour to jump rather than end, so it
  can leave a true claim uncertified and never certifies a false one: every
  `Moving` step of `Braking` is a may-end step and holds `v [0, 20 - k/16]`
  at every later instant, so the true `always [5, 10] holds(h.v <= 15)` is
  not certified.
- At an instant `t`, an atom is definitely true when every reading holding
  at `t` is definitely true, definitely false when every one is definitely
  false, and unknown otherwise or when none holds at `t`. With coverage
  passed every behaviour's state at every instant up to `H` lies in some
  `B` whose reading holds then, so the meet is sound; the pre-jump and
  post-jump positions at one instant read one meet, which can only make
  the value unknown.
- A capture is read over the hull of the initial entries' hulls, losing its
  correlation with later states, which is sound; it SHALL be definitely
  defined and definitely within its type's value range, else `claim` at
  `/certificate`.
- The claim is evaluated by FR-416's pointwise meaning lifted to three
  values: Boolean connectives by Kleene's rules, `always I` as the Kleene
  conjunction over the instants of its interval, `eventually I` as the
  Kleene disjunction, and `until`, `release`, `once`, `historically`,
  `since` and `triggered` by their pointwise definitions with those
  connectives. The rule passes when the value at the origin is definitely
  true and every claim atom is definitely defined over every `B` whose
  covered instants meet the claim region `[0, r]`.

Under `model-steps`, position 0 reads the first `box` of each initial
dwell, and position `p >= 1` reads the `box` of each jump step of depth `p`
and, for the behaviours with fewer than `p` jumps, the state at which each
ends, which FR-193 "Behaviours and positions" makes such a behaviour's state
at every position after its last jump. A behaviour with `j < p` jumps ends
in a dwell of depth `j`, at an instant of a step from which it may end (the
*may-end* steps above), so the reading at position `p` is the meet of the
readings of the `box` of every jump step of depth `p` and of the `B` of
every may-end step of every dwell of depth less than `p`; it is never read
from the box at a behaviour's last jump alone, because the behaviour moves
on from there before it ends. With `x' = 1`, the invariant
`x >= 0 and x <= 1`, `x = 0` initially and no transition, the behaviour
ends at `x = 1`, so position 1 reads `x = 1` and `always [0, 1]
holds(h.x < 1)` is not certified. The claim is evaluated over positions 0
to its horizon `n` as above.

On failure, `at` is the first step, in list order, among the steps read in
the claim region over whose `B` (or, under `model-steps`, over the `box` or
`B` it is read by) some claim atom is not definitely defined, reads unknown
or reads definitely false; when no step reads so, the first step read in
the claim region.

### Rules of `validated-witness`

Dwells are read in order, and steps within a dwell in order; `Inv` is a
step's mode's invariant, and dwell `d`'s mode is its trace state's mode.

| Order | Rule | Inputs | Passes when | `at` on failure |
| --- | --- | --- | --- | --- |
| 1 | `box-shape` | every step's `box`, then its `apriori` | as for `inductive-enclosure` | `/certificate/dwells/<d>/<k>/box` or `/certificate/dwells/<d>/<k>/apriori` |
| 2 | `witness-start` | the trace's `start`; the initial entries; `dwells[0][0]` | `start.time` is `[0, 0]`; every interval of `start.enclosure` is a point, together the point `p`; some initial entry of `start.mode` has its condition and `Inv` definitely true and defined at `p`; and `dwells[0][0]` has `time.lower` 0 and `p` in its `box` | `/certificate/dwells/0/0` |
| 3 | `witness-picard` | each step `(d, k)` | `time` is `[l, l + h]`, `l` being 0 for the first step of dwell 0, the jump instant of dwell `d - 1` for the first step of dwell `d > 0`, and otherwise the previous step's `time.upper` in the dwell; every rate and `Inv` are definitely defined over `B`; the uncontracted tube `⊆ B`; `Inv` is definitely true over `box`, and over `B` unless the step is the witness's last; and the uncontracted advance of the previous step in the dwell `⊆ box` | `/certificate/dwells/<d>/<k>` |
| 4 | `witness-unique` | each step's `B`; the expanded rates | every `if c then a else b` in the mode's rates has `c` definitely true or definitely false over `B`, and so on in the branch taken; a rate whose expansion reaches a recursive call fails, since no branch can be established | `/certificate/dwells/<d>/<k>` |
| 5 | `witness-guard` | each dwell `d` except the last; `trace.jumps[d]`; the next dwell's first step | see "Jumps of a witness" | the last step of dwell `d` |
| 6 | `witness-violation` | the readings | the witness has a decisive reading, and a trace `end`, when present, equals it | the witness's last step |
| 7 | `work-limit` | the item's work meter | the meter does not stop the check | the step being checked |

Rules 3 and 4 make the steps enclose the one solution from `p`: the tube
inclusion bounds it over each step, `witness-unique` makes the flow one
Lipschitz expression over each `B` so the solution is unique, and the
invariant definitely true over every box and every `B` before the last
step makes that solution a behaviour, which keeps its dwell's invariant at
every instant (FR-193 "Behaviours and positions").

**Jumps of a witness.** For dwell `d` and the transition `t` that
`trace.jumps[d]` names, to mode `n`, with `X` the `box` of dwell `d`'s last
step and `Y` the `box` of dwell `d + 1`'s first step, `witness-guard`
SHALL require, in this order: dwell `d`'s last step is a witness jump step
(`step` 0, `time` `[τ, τ]`); the guard of `t` definitely defined and
definitely true over `X`, so the behaviour may jump at `τ`; the reset
definitely defined over `X`; for each variable with an `=` conjunct, the
value of its `e` over `X` included in `Y`'s interval, and for each variable
no conjunct names, its interval in `X` included in `Y`'s; every other reset
conjunct definitely true reading its post-jump variable over `Y` and its
`e` over `X`; `Inv_n` definitely true over `Y`; `trace.jumps[d].state.time`
containing `τ` and `trace.jumps[d].state.enclosure` meeting `Y`; and, when
`trace.jumps[d].before` is present, its `time` containing `τ` and its
enclosure meeting `X`. The jump instant is derived from the witness, never
taken from the trace: a trace whose jump time does not contain `τ` cannot
shift the witness's later instants.

**Readings and the decisive box.** The reader reads each claim atom by
three-valued interval evaluation over the two boxes each witness step
holds: its `box`, which encloses the solution at the one instant
`time.lower` at which the step starts, and its a-priori box `B`, which
encloses the solution at every instant of the step's `time`. A reading over
`box` holds at that one instant, a reading over `B` at every instant of
`time`, both at the position of the step's dwell, and an instant of the
claim region that no definite reading covers reads unknown. Readings are
ordered by position, then by the instant at which they end (`time.lower`
for a `box` reading, `time.upper` for a `B` reading), a `box` reading
before a `B` reading that ends at the same instant. The *decisive reading*
is the earliest reading at which either the claim, evaluated as under
"Reading the claim" over the readings up to and including it with every
later instant unknown, is definitely false at the origin, so the prefix up
to it is a bad prefix of the claim for every value the unknown readings
could take; or a claim atom, activation condition or capture initializer is
definitely without value over its box at an instant of the claim region. A
`B` reading of the witness's last step counts only when `Inv` is definitely
true over that `B`. The decisive box is the decisive reading's box. A box
that straddles an atom's boundary, as `v` in `[1, 41]` against `v < 30`,
reads that atom unknown, and an unknown reading never refutes, even when
the true solution lies on the violating side: the reader never settles an
unknown reading by a midpoint, an endpoint or a sample of the box, because
only a definitely false reading proves that every solution the box encloses
violates the claim. When the trace carries FR-193's `end`, `end.time` SHALL
equal the decisive reading's instants (the point interval of a `box`
reading, the step's `time` for a `B` reading) and `end.enclosure` SHALL
equal its box. A provider that cannot build a witness with a decisive box
returns its trace without one, and the item stays `inconclusive` as FR-193
"Trace replay" states.

**Decisive readings of an `on each` claim.** A witness accompanies a trace
with no `loop` (a witness on a lasso is refused), so the activation rules
of FSL FR-450 apply to a finite prefix and no period count arises. Each
activation instant `τ` creates one obligation with its own captured values,
and a past operator in the body, the guard or a capture reads the witness
from position 0 (FR-193 "Activations and captures"). The reader reads
activations at the instants of the witness's `box` readings: the instant
`time.lower` of a step is an *activation* when the activation condition is
definitely true and definitely defined over that step's `box`, and its
captures are read over that `box`. A reading is decisive for an `on each`
claim when, over the readings up to and including it with every later
instant unknown, some activation's obligation, evaluated from its instant
by the three-valued meaning of "Reading the claim", is definitely false, or
some activation has a capture definitely outside its type's value range
(kind `undefined-evaluation`, cause `outside-capture-range`) or an atom,
activation condition or capture definitely without value. Of several such readings the
earliest in reading order is decisive, as for an `on origin` claim, and its
instants, a point for a `box` reading, contain the infimum FSL FR-450 takes
as the decisive instant when that infimum lies in the reading. A behaviour
whose violating activations form an open set with no earliest element is
refuted by a witness only when some box reading inside that set is
decisive; the witness never reports an instant no reading holds.

### Work limit

Checking is bounded by the item's work limit: each evaluation of one
expression over one box, the unbounded box of a hull included, is one unit
charged to the item's work meter. A check the meter stops is
`rejected{rule: "work-limit"}` at the pointer the running rule would name
for the element being checked (for `box-shape`, which evaluates nothing,
no unit is charged), and is never `accepted`.

### Rule names

A rejection names the first failing rule by one of these 17 spellings.

| Form | Rule | Spelling |
| --- | --- | --- |
| every form | a box keyed differently from the declaration's variables, or with `lower > upper` | `box-shape` |
| `inductive-enclosure` | (a) initial containment | `a` |
| `inductive-enclosure` | (b) faces | `b` |
| `inductive-enclosure` | (c) transitions | `c` |
| `inductive-enclosure` | (d) claim | `d` |
| `flowpipe` | each initial dwell starts at 0 and its first box contains its mode's initial states | `initial` |
| `flowpipe` | a step's a-priori box satisfies the contracted Picard condition | `picard` |
| `flowpipe` | a step's time and box follow from the previous step of its dwell | `successor` |
| `flowpipe` | every transition that can fire is followed by a jump step whose `box` and `time` contain what "Following transitions" requires | `jump` |
| `flowpipe` | the dwells cover the proof's `horizon`, `jumps` and `exhaustive` | `coverage` |
| `flowpipe` | the claim is definitely true over the readings | `claim` |
| `validated-witness` | the start is a point satisfying an initial entry | `witness-start` |
| `validated-witness` | each dwell's steps satisfy the Picard condition and enclose the solution as a behaviour | `witness-picard` |
| `validated-witness` | each conditional takes one branch over each `B` | `witness-unique` |
| `validated-witness` | each jump's guard is definitely true where it is taken, and the jump binds to the trace | `witness-guard` |
| `validated-witness` | the witness has a decisive box: a reading over a step's `box` or `B` at which the claim is definitely violated, or an expression is definitely without value | `witness-violation` |
| every form | the item's work limit stopped the check | `work-limit` |

### Settlement

- **What is checked.** A certificate SHALL be checked only when the
  settlement rests on it: the certificate of a `proved` entry that FR-193
  settles `proved`, and the `validated-witness` of a trace that exact
  replay returns `not-exact` for. Any other certificate is recorded
  unchecked, with its form and no outcome: the certificate of a proof
  FR-193 settles `insufficient-horizon`, and the witness of a trace exact
  replay decides, whose replay is refused, or whose replay the work limit
  stops. A trace exact replay decides settles by exact replay alone,
  whatever witness it carries.
- The record of a `proved` entry whose certificate is accepted SHALL carry
  the FR-331 certification `certified`, whatever provider produced it. The
  entry itself keeps the label its provider wrote, `uncertified` from a
  first-party provider or `trusted` from a plugin (FR-331 "Certification"),
  because the provider cannot vouch for the core's check. A rejected
  certificate SHALL leave the record as FR-193 settles it without one (a
  first-party proof `uncertified`, a plugin's `trusted`), still `proved`,
  and SHALL record `certificate-rejected{rule, at}` beside it; it never
  turns a proof into a refutation or an `inconclusive`. A proof certificate
  check the work limit stops is such a rejection, `rule` `work-limit`.
- A `proved` entry with a `bounded-solver` or `inductive-solver` basis MAY
  carry a certificate; a `refuted` entry and an `inconclusive` entry MAY
  carry only a `validated-witness`; no other entry carries one.
- Every trace FR-193 "Trace replay" replays statically only (a trace that
  is not a point trace, such as one whose `end` holds the decisive box, a
  trace that dwells in a mode without a polynomial flow, or one whose replay
  would evaluate a recursive call) SHALL settle `refuted` when its
  `validated-witness` is accepted, from a `solver-inconclusive` cause, or
  from a `refuted` entry whose claimed kind agrees with the kind the
  witness gives (FSL FR-450), with the trace as its `HybridCounterexample`
  and one `counterexamples` entry; a `refuted` entry settles `refuted` only
  after its witness is accepted, and otherwise `inconclusive`,
  `replay-parity`. Its replay settlement is FR-364's
  `reproduced-with-validated-witness`, distinct from
  `reproduced-with-evaluated-witness` so a reader can tell a refutation
  proved by exact interval enclosure from one replayed exactly. The
  counterexample's kind is `undefined-evaluation` when the decisive box is
  one over which an expression is definitely without value, and `formula`
  otherwise. When the witness is absent or rejected, the item settles as
  FR-193 states: static replay only and no refutation, a `refuted` entry
  settling `inconclusive`, `replay-parity`.
- A witness check the work limit stops SHALL settle the item `incomplete`,
  `limit-reached{limit: work_units, value, setting}`, with the witness
  recorded `rejected{rule: "work-limit", at}`; it is never `refuted` and
  never `replay-parity`, since the check did not decide the witness.
- A trace whose witness was accepted MAY end with FR-193's `end` holding the
  decisive box, because the solution's values at the decisive instant are
  in general irrational: `end.time` is the instant of a `box` reading as a
  point interval, or the step's `time` for an a-priori box reading, and
  `end.enclosure` is that box.
- A replay request (FR-323) for an item refuted through an accepted witness
  SHALL carry that `validated-witness` beside the `HybridCounterexample`.
  The replay SHALL read it by "Checking order" steps 1 to 3 before it
  replays, run static and then exact replay, and check the witness by its
  rules only when exact replay returns `not-exact`, so the refutation
  replays from the request alone and a trace exact replay decides is
  settled by exact replay.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-652-AC-1 | Over FR-193's `Braking` without its variable `t`, an `inductive-solver` proof of `always holds(h.v < 30)` with boxes `Moving: v [0, 20], a [1, 2]` and `Stopped: v [0, 0], a [1, 2]` is accepted (the `v = 0` face of `Moving` lies on its invariant's boundary) and settles `certified`; with `Moving: v [0, 19]` it is rejected naming rule (a), since the initial state `v = 20` lies outside, and the proof stays `uncertified` with `certificate-rejected` recorded. | Test (TC-1605) |
| FR-652-AC-2 | A `flowpipe` certificate whose step violates the Picard condition is rejected naming that step, and the `bounded-solver` proof stays `uncertified` with `certificate-rejected` recorded; a well-formed one covering horizon 10 settles `certified`. | Test (TC-1605) |
| FR-652-AC-3 | Over `Grow`, one mode `Up` with invariant `v >= 1 and v <= 100`, initial `v = 1` and flow `v' = 1 - 1 / (1 + v * v)`, which is not polynomial, against its hybrid claim `Below`, `always [0, 40] holds(h.v < 30)`: a point trace from `v = 1` with a `validated-witness` of nine steps of 4 s, step `k` at time `[4k, 4k + 4]` with `box` `v [l_k, 1 + 4k]` and `B` `v [l_k, 5 + 4k]`, `l` being 1, 3, 33/5, 21/2, 72/5, 183/10, 111/5, 261/10, 30, is accepted, its decisive box the `box` `v [30, 33]` at time 32 over which `v < 30` is definitely false, and settles `refuted` with one `counterexamples` entry. The same trace stays `inconclusive` without a witness; with that witness cut after its step at `[28, 32]`, whose `B` `v [261/10, 33]` straddles 30; and with one step at `[0, 40]` whose `box` is `v [1, 1]` and whose `B` `v [1, 41]` satisfies the Picard condition but straddles 30. Each cut witness is rejected `witness-violation` at its last step, `/certificate/dwells/0/7` and `/certificate/dwells/0/0`. | Test (TC-1605) |
| FR-652-AC-4 | A `flowpipe` certificate on an `inductive-solver` entry refuses `invalid_result`/`invalid-value` at `/certificate`; a box missing variable `a` is rejected `box-shape` at `/certificate/boxes/Moving`, and FR-652-AC-5's certificate with `steps/3` `apriori` `v [1, 0], a [1, 2]` is rejected `box-shape` at `/certificate/steps/3/apriori`; a check stopped by the work limit is `rejected{rule: work-limit}`. | Test (TC-1605) |
| FR-652-AC-5 | Over `Braking` without `t`, whose `Moving` invariant `v >= 0` is a boundary invariant of its guard `v = 0`, a `flowpipe` certificate whose 160 `Moving` steps of `step` 1/16 cover `[0, 10]`, step `k` with `time` `[k/16, (k + 1)/16]` and `box` and `B` `v [0, 20 - k/16], a [1, 2]`, each contracted to `v >= 0`, and whose one jump step, `steps/160`, carries `from` `{"dwell": 0, "transition": 0}` with `Stopped: v [0, 0], a [1, 2]`, `step` 10 and `time` `[0, 10]`, the narrowed jump time (the value `[0, 20]` of `(0 - v0) / (-a)` over `v0 [0, 20], a [1, 2]`, offset by the start instant 0 and intersected with the horizon `[0, 10]`), is accepted against `always [0, 10] holds(h.v < 30)` with an entry of `horizon` 10, `jumps` 1 and `exhaustive` `["time"]`, and settles `certified`; the same with jump-step `time` `[1, 10]` is rejected with rule `jump` at `/certificate/steps/160`, since it omits the jump at time 0 from `v0 = 0`; the same with `time` `[0, 20]` is accepted, since it contains the narrowed jump time, and its width 20 differs from its `step` 10 without failing any rule, a jump step's `time` being its start instants; against `always [0, 10] holds(h.v > 5)` the claim is not definitely true over the first `Moving` step and the certificate is rejected with rule `claim` at `/certificate/steps/0`. | Test (TC-1605) |
| FR-652-AC-6 | The schema admits a first-party `proved` `inductive-solver` entry labelled `uncertified` carrying an `inductive-enclosure` certificate and a `bounded-solver` entry carrying a `flowpipe` certificate whose `step` is `{"numerator": "1", "denominator": "16"}`, and refuses the same with `step` `"1/16"`, a box bound written `"0"`, a certificate `type` outside the three forms, the entry labelled `certified`, and a `flowpipe` on an `inconclusive` entry; when the reader accepts the `inductive-enclosure` the record is `certified` while the entry stays `uncertified`, and when it rejects it the record stays `proved`, `uncertified`, with `certificate-rejected{rule, at}` beside it and not `inconclusive`. | Test (TC-2049) |
| FR-652-AC-7 | Over `Rise`, `hybrid Rise using p { initial { mode Up when { v >= 0 and v <= 20 }; } mode Up invariant { v >= 0 and v <= 41/2 } flow { v' = 1; } }`, an `inductive-solver` proof of `always holds(h.v <= 21)` with the box `Up: v [0, 41/2]` is accepted (the upper face lies on the conjunct `v <= 41/2`, the lower face has rate 1 `>= 0`); with `Up: v [0, 39/2]` it is rejected `a` at `/certificate/boxes/Up`, since the initial hull `v [0, 20]` is not included in the box; with `Up: v [0, 20]` it is rejected `b` at `/certificate/boxes/Up`, since the rate 1 on the upper face points out and `20` is below the conjunct's bound `41/2`, so states with `20 < v <= 41/2` escape (the step-back finding that a probe at one point beyond the face accepted this box). Over `Twin`, `hybrid Twin using p { initial { mode M when { x = 0 and y = 0 }; } mode M invariant { x = y } flow { x' = 1; y' = 1; } }`, the box `M: x [0, 1], y [0, 1]` against `always holds(h.x <= 1)` is rejected `b` at `/certificate/boxes/M`: the conjunct `x = y` reads `y`, whose rate is 1, so neither upper face is a boundary face, and the rate 1 points out (the behaviour reaches `x = 2` at time 2). | Test (TC-1605) |
| FR-652-AC-8 | Over `Braking` without `t`: FR-652-AC-1's accepted boxes with `Stopped: v [0, 0], a [1, 3/2]` are rejected `c` at `/certificate/boxes/Stopped`, the post image `v [0, 0], a [1, 2]` of `Moving`'s guarded part not being included; FR-652-AC-1's accepted boxes against `always holds(h.v < 10)` are rejected `d` at `/certificate/boxes/Moving`, against `always holds(1 / h.v < 100)` are rejected `d` at `/certificate/boxes/Moving` since `1 / v` is not definitely defined over `v [0, 20]`, and against `always [0, 10] eventually [0, 1] holds(h.v < 30)` are rejected `d` at `/certificate`. Over `Hold`, `hybrid Hold using p { initial { mode H when { v >= 40 and v <= 50 }; } mode H invariant { v >= 0 and v <= 50 } flow { v' = 0; } }`, the box `H: v [40, 50]` is accepted against `always not holds(h.v < 30)` and rejected `d` at `/certificate/boxes/H` against `always not holds(h.v >= 30)`, whose atom reads definitely true and its negation definitely false. | Test (TC-1605) |
| FR-652-AC-9 | Over `Braking` without `t`: FR-652-AC-5's accepted certificate with `steps/0` `box` `v [0, 1/32], a [1, 2]` is rejected `initial` at `/certificate/steps/0`, since the initial states up to `v = 20` lie outside it (the step-back finding that the first box was never compared with the initial states); with `steps/0` `time` `[0, 1/8]` it is rejected `initial` at `/certificate/steps/0`. Over `Braking` without `t` with `initial { mode Moving when { v = 20 and a >= 1 and a <= 2 }; mode Stopped when { v = 0 and a = 1 }; }`, two initial modes, FR-652-AC-5's steps followed by an initial `Stopped` dwell, one step without `from` with `time` `[0, 10]`, `step` 10 and `box` and `B` `v [0, 0], a [1, 1]`, and then FR-652-AC-5's jump step with `from` `{"dwell": 0, "transition": 0}`, are accepted; without that initial `Stopped` dwell they are rejected `initial` at `/certificate/steps/0`, since no initial dwell has mode `Stopped`. | Test (TC-1605) |
| FR-652-AC-10 | Over FR-652-AC-5's accepted certificate: the step `steps/0` `B` is `v [0, 20]` although its uncontracted tube is `v [-1/8, 20]`, and it is accepted because the tube is contracted by `v >= 0` before the inclusion; with `steps/3` `apriori` `v [0, 20 - 3/16 - 1/32], a [1, 2]` it is rejected `picard` at `/certificate/steps/3`; with `steps/6` `box` `v [0, 20 - 6/16 - 1/32], a [1, 2]` it is rejected `successor` at `/certificate/steps/6`, since the contracted advance of `steps/5` reaches `20 - 6/16`; with `steps/1` `time` `[1/8, 3/16]` it is rejected `successor` at `/certificate/steps/1`, since `steps/1` starts at `1/16`. `Braking`'s `Moving` mode is contraction-admitted (its rates `-a` and `0` and its invariant's bounds have no conditional). Over `Cond`, `hybrid Cond using p { initial { mode M when { x = 0 and y = 0 }; } mode M invariant { x <= if y > 0 then 10 else 0 } flow { x' = 1; y' = x; } }`, which is not contraction-admitted, the one step `{M, time [0, 1], box x [0, 0], y [0, 0], B x [-1, 0], y [-1, 0], step 1}` under an entry of `horizon` 1, `jumps` 0 and `exhaustive` `["time"]` against `always [0, 1] holds(h.x <= 0)` is rejected `picard` at `/certificate/steps/0`: the uncontracted tube `x [0, 1], y [-1, 0]` is not included in `B`, although its contraction is, and the behaviour has `x = t`. | Test (TC-1605) |
| FR-652-AC-11 | Over `Fork`, `hybrid Fork using p { initial { mode A when { x = 0 }; } mode A invariant { x >= 0 and x <= 10 } flow { x' = 1; } transition to B when { x >= 5 } reset { x = 100 }; transition to C when { x >= 7 } reset { x = 0 }; mode B invariant { x >= 0 and x <= 200 } flow { x' = 0; } mode C invariant { x >= 0 and x <= 1 } flow { x' = 0; } }`, with an entry of `horizon` 10, `jumps` 1 and `exhaustive` `["time"]`: the certificate of ten `A` steps, step `k` with `time` `[k, k + 1]`, `step` 1, `box` `x [k, k]` and `B` `x [k, k + 1]`, then a `B` jump step with `from` `{"dwell": 0, "transition": 0}`, `time` `[4, 10]`, `step` 6 and `box` and `B` `x [100, 100]`, then a `C` jump step with `from` `{"dwell": 0, "transition": 1}`, `time` `[6, 10]`, `step` 4 and `box` and `B` `x [0, 0]`, a dwell with two children, is accepted against `always [0, 10] holds(h.x >= 0)` and rejected `claim` at `/certificate/steps/10` against `always [0, 10] holds(h.x < 50)`; the ten `A` steps alone are rejected `jump` at `/certificate/steps/4` against either claim, the first step from whose `B` `x >= 5` can fire (the step-back finding that a flowpipe of source steps alone was accepted while a jump to `x = 100` violates the claim); without the `C` jump step they are rejected `jump` at `/certificate/steps/6`; with the `B` jump step's `time` `[5, 10]` they are rejected `jump` at `/certificate/steps/10`, since the jumps from `steps/4` may occur at time 4 and no equality conjunct narrows the time; with its `box` `x [99, 99]` they are rejected `jump` at `/certificate/steps/10`. | Test (TC-1605) |
| FR-652-AC-12 | Over `Tick`, `hybrid Tick using p { initial { mode T when { x = 0 }; } mode T invariant { x >= 0 and x <= 1 } flow { x' = 1; } transition to T when { x = 1 } reset { x = 0 }; }`, a self-loop, with an entry of `horizon` 3/2, `jumps` 1 and `exhaustive` `["time"]`, against `always [0, 3/2] holds(h.x <= 1)`: the steps `{T, time [0, 1], box x [0, 0], B x [0, 1], step 1}`, `{T, time [1, 3/2], box x [1, 1], B x [1, 1], step 1/2}` and `{T, from {dwell 0, transition 0}, time [1, 1], box x [0, 0], B x [0, 1/2], step 1/2}` are accepted, the jump step's `time` containing the narrowed jump time `[1, 1]`; the same third step without `from` continues dwell 0 and is rejected `successor` at `/certificate/steps/2`, since it would start at time 3/2. | Test (TC-1605) |
| FR-652-AC-13 | Coverage, with `Braking` without `t` and `always [0, 10] holds(h.v < 30)`: FR-652-AC-5's 160 `Moving` steps alone, under an entry of `horizon` 10, `jumps` 0 and `exhaustive` `["time"]`, are rejected `coverage` at `/certificate/steps/0`, since `v = 0` can fire on the first step of a depth-0 dwell, so a behaviour takes a jump the entry denies (the step-back finding that `jumps` was compared with the step count and `exhaustive` ignored); FR-652-AC-5's accepted certificate under `exhaustive` `["time", "jumps"]` is rejected `coverage` at `/certificate/steps/159`, since that last step closes the `Moving` dwell while `v = 0` can still fire from its `B`, whose covered instants then have no upper bound, so the steps do not show that every first jump comes by time 10. Over FR-652-AC-11's `Fork`, its first five `A` steps with the `B` jump step at `time` `[4, 5]` are rejected `coverage` at `/certificate/steps/4`, the `A` dwell ending at time 5 without emptying or closing. Over `Drop`, `hybrid Drop using p { initial { mode A when { x = 1 }; } mode A invariant { x >= 0 and x <= 1 } flow { x' = -1; } transition to B when { x = 0 } reset { x = 0 }; mode B invariant { x = 0 } flow { x' = 0; } }`, the steps `{A, time [0, 1], box x [1, 1], B x [0, 1], step 1}`, `{A, time [1, 2], box x [0, 0], B x [0, 0], step 1}` and `{B, from {dwell 0, transition 0}, time [1, 1], box x [0, 0], B x [0, 0], step 1}` are accepted against `always [0, 2] holds(h.x <= 1)` under an entry of `horizon` 2, `jumps` 1 and `exhaustive` `["time", "jumps"]`, the `A` dwell emptying after its last step. | Test (TC-1605) |
| FR-652-AC-14 | The `claim` rule: over `Hold` (FR-652-AC-8), the one step `{H, time [0, 10], box v [40, 50], B v [40, 50], step 10}` under an entry of `horizon` 10, `jumps` 0 and `exhaustive` `["time"]` is accepted against `always [0, 10] holds(h.v >= 30)`, and rejected `claim` at `/certificate/steps/0` against `always [0, 10] not holds(h.v >= 30)` and against `always [0, 10] (holds(h.v < 45) implies holds(h.v < 30))`, whose readings are false and unknown at every instant (the step-back finding that `not` and `implies` were applied to whole-box results); FR-652-AC-5's certificate is rejected `claim` at `/certificate/steps/0` against `always [0, 10] holds(1 / h.v < 100)`, its atom not definitely defined over `v [0, 20]`; FR-652-AC-13's `Drop` certificate under `clock "model-steps"` with an entry of `horizon` 2, `jumps` 1 and `exhaustive` `["jumps"]` is accepted against `always [0, 1] holds(h.x <= 1)` and rejected `claim` at `/certificate/steps/1` against `always [0, 1] holds(h.x = 1)`, position 1 reading the meet of the jump step's box `x [0, 0]` and the `B` `x [0, 0]` of the may-end `A` step `steps/1`; over `Ramp`, `hybrid Ramp using p { initial { mode R when { x = 0 }; } mode R invariant { x >= 0 and x <= 1 } flow { x' = 1; } }`, with no transition, the steps `{R, time [0, 1], box x [0, 0], B x [0, 1], step 1}` and `{R, time [1, 2], box x [1, 1], B x [1, 1], step 1}` under `clock "model-steps"` with an entry of `horizon` 2, `jumps` 1 and `exhaustive` `["jumps"]` are rejected `claim` at `/certificate/steps/1` against `always [0, 1] holds(h.x < 1)`, position 1 reading the state `x = 1` at which the behaviour ends, from the `B` of the may-end step `steps/1`. | Test (TC-1605) |
| FR-652-AC-15 | `validated-witness` rules over FR-652-AC-3's trace and nine-step witness: with `dwells/0/0` `box` `v [2, 3]` it is rejected `witness-start` at `/certificate/dwells/0/0`; with `dwells/0/3` `apriori` `v [21/2, 15]` it is rejected `witness-picard` at `/certificate/dwells/0/3`; over `GrowLow`, `Grow` with invariant `v >= 1 and v <= 31`, it is rejected `witness-picard` at `/certificate/dwells/0/7`, `Inv` not being definitely true over `v [261/10, 33]`; with the trace carrying `end` `(time [32, 32], v [30, 33])` it is accepted, and with `end` `(time [31, 31], v [30, 33])` it is rejected `witness-violation` at `/certificate/dwells/0/8`. Over `Shift`, `hybrid Shift using p { initial { mode Up when { v = 3 }; } mode Up invariant { v >= 1 and v <= 100 } flow { v' = if v > 2 then 1 else 1/2; } }`, against `always [0, 10] holds(h.v < 5)`, the point trace from `v = 3` with the witness `{time [0, 2], box v [3, 3], B v [3, 5], step 2}`, `{time [2, 3], box v [5, 5], B v [5, 6], step 1}` is accepted, its decisive box `v [5, 5]` at time 2; the same model with initial `v = 1` and the witness `{time [0, 4], box v [1, 1], B v [1, 5], step 4}`, `{time [4, 5], box v [3, 5], B v [3, 6], step 1}` is rejected `witness-unique` at `/certificate/dwells/0/0`, `v > 2` being unknown over `v [1, 5]`. | Test (TC-1605) |
| FR-652-AC-16 | Over `GrowJump`, `Grow` with the transition `transition to Hold when { v >= 3 } reset { v = 50 }` and the mode `mode Hold invariant { v >= 1 and v <= 100 } flow { v' = 0; }`, against `always [0, 10] holds(h.v < 30)`: the trace from `(Up, time 0, v = 1)` by transition 0 to `(Hold, time [4, 4], v [50, 50])` with the witness whose dwell 0 is `{time [0, 4], box v [1, 1], B v [1, 5], step 4}` and the witness jump step `{time [4, 4], box v [3, 5], B v [3, 5], step 0}`, and whose dwell 1 is `{time [4, 8], box v [50, 50], B v [50, 50], step 4}`, is accepted, its decisive box `v [50, 50]` at time 4 in position 1; the same with the trace's jump state at `time [5, 5]` is rejected `witness-guard` at `/certificate/dwells/0/1`, since the trace's jump time does not contain the witness's jump instant 4 (the step-back finding that the trace's jump time was trusted and shifted later instants); the same with the guard `v >= 4` is rejected `witness-guard` at `/certificate/dwells/0/1`, the guard being unknown over `v [3, 5]`. | Test (TC-1605) |
| FR-652-AC-17 | Structural refusals over `Braking` without `t`, each `invalid_result` at the analyze path and `invalid_runtime_input` at `/replay/certificate/...` on a replay request, never an internal fault: an `inductive-enclosure` with a `Halted` box (`invalid-value` at `/certificate/boxes/Halted`) or with no `Stopped` box (`missing-member` at `/certificate/boxes/Stopped`); FR-652-AC-5's certificate with `steps/0` `step` `-1/16` (`invalid-value` at `/certificate/steps/0/step`), with `steps/0` `time` `[1, 0]` (`invalid-value` at `/certificate/steps/0/time`), with the jump step's `from` `{"dwell": 1, "transition": 0}` (`invalid-value` at `/certificate/steps/160/from/dwell`), with `{"dwell": 0, "transition": 1}` (`invalid-value` at `/certificate/steps/160/from/transition`), and with the jump step's `mode` `Moving` (`invalid-value` at `/certificate/steps/160/mode`); FR-652-AC-3's witness with a second dwell for its one-state trace (`invalid-value` at `/certificate/dwells`), with a step carrying `from` (`invalid-value` at `/certificate/dwells/0/0/from`), and on a trace with `loop` (`invalid-value` at `/certificate`). | Test (TC-1605) |
| FR-652-AC-18 | Settlement edges: a `bounded-solver` proof of `always [0, 10] holds(h.v < 30)` over `Braking` without `t` with `horizon` 1, which FR-193 settles `insufficient-horizon`, carrying FR-652-AC-5's certificate records the certificate with its form and no outcome, unchecked; FR-193-AC-17's `Braking` point trace, which exact replay reproduces, as a provider's `refuted` entry carrying a structurally valid witness whose `dwells/0/0` `box` misses its start, settles `refuted` by exact replay with that witness unchecked, and with `before` `v = 1`, which exact replay does not reproduce, settles `inconclusive`, `replay-parity` with the witness unchecked; FR-652-AC-3's provider `refuted` entry with its nine-step witness under a `work_units` bound that stops the witness check settles `incomplete`, `limit-reached{limit: work_units}`, with the witness recorded `rejected{rule: work-limit}`, never `refuted`; FR-652-AC-1's accepted certificate under a work limit of one evaluation is rejected `work-limit` at `/certificate/boxes/Moving`. | Test (TC-1605) |
| FR-652-AC-19 | Rejections for definedness: over `Recip`, `hybrid Recip using p { initial { mode M when { x = 1 }; } mode M invariant { x >= 0 and x <= 2 } flow { x' = 1 / x; } }`, the step `{M, time [0, 1], box x [1, 1], B x [0, 2], step 1}` under an entry of `horizon` 1, `jumps` 0 and `exhaustive` `["time"]` against `always [0, 1] holds(h.x <= 2)` is rejected `picard` at `/certificate/steps/0`, the rate `1 / x` not being definitely defined over `x [0, 2]`; over `ForkDiv`, FR-652-AC-11's `Fork` with transition 0's reset `x = 100 / (x - 6)`, FR-652-AC-11's accepted certificate is rejected `jump` at `/certificate/steps/5`, the reset not being definitely defined over the guarded part `x [5, 6]` of `steps/5` (from `steps/4` the post image `-100` breaks `B`'s invariant, so the transition cannot fire there); over `GrowJump` (FR-652-AC-16) with the reset `v = 50 / (v - 4)`, FR-652-AC-16's witness is rejected `witness-guard` at `/certificate/dwells/0/1`, the reset not being definitely defined over `v [3, 5]`. | Test (TC-1605) |

## Dependencies

- [FR-193](FR-193-analyze-continuous-hybrid-models.md) owns hybrid models,
  claims, verdicts, traces, replay and proof coverage.
- FR-331 owns the result entry, its certification values and the
  certificate schema, `from` included.
- FR-416 owns the pointwise meaning the `claim` rule and the decisive
  reading lift to three values.
