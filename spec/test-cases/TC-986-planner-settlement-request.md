---
id: TC-986
title: "Planner settlement request"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-492
    type: verifies
---

# TC-986: Planner settlement request

## Description

Admit, refuse and settle FR-492 requests for TC-984 case 1's guarantee item.

Scope: FR-492-AC-1 to FR-492-AC-14 and FR-331-AC-61 to FR-331-AC-62.

## Test Procedure

Build the request for TC-984 case 1's item, each item-backed slot filled with
the complete FR-331 request item and a `proved` result entry of that item,
and the environment slot with its record. Run each case through the real
local Rust boundary:

1. The request as built.
2. Drop the frame slot; swap two slots; relabel the frame slot
   `transition-simulation`; set `package` to another package identity.
3. Replace `slots` with `true`; replace one slot's `result` with `true`.
4. Remove the initialization slot's `result`; remove the environment slot's
   `record`; then build the request for TC-984 case 9's declaration with no
   `progress` member.
5. Fill the transition-simulation slot with a result whose request item
   belongs to the frame slot, then with one compiled against a rebound
   relation; then with a result entry that lacks `certification`.
6. Seed, one per run, the frame slot's result `unsupported`, `incomplete`
   and `inconclusive`; then make both the initialization and frame results
   `inconclusive`.
7. Make the model-claim result `tested`.
8. Make one result a plugin's `trusted` proof; then make every result
   `certified`.
9. Set `max_work` to 2.
10. Read a provider result entry carrying `premise-unproved`, then one
    carrying `transfer-not-preserving`.
11. Replace the clause with `always (holds(c.value >= 1) implies eventually always[0,1] holds(c.value = 2))`, then with a clause using `previous`, every slot proved.
12. Make the model-claim result name an assumption no environment record
    supplies; then, for TC-985 case 1's composition, offer only an
    environment record for the rely slot through `discharges`.
13. Fill the model-claim slot with a result checked without
    `stutter_closure`, then under `fair strong each inc`; fill the
    transition-simulation slot with the frame item's result.
14. Read a provider's `composition` result entry carrying
    `provider-answer-unconfirmed`, plugin and in-process; then the same cause
    on a `value-validity` entry.
15. Answer TC-985 case 7's `composition` item `proved` by a provider, with the
    request's `component-guarantee` items all `proved`; then with the second
    one `inconclusive`; then with it absent from the request; then with the
    request holding none of the component items; then with the second held
    item replaced by one over the same clause node with another fairness set;
    then with the second entry's key `assumptions` altered from the derived
    array; then with the matching item held and `proved` under the derived
    context.
16. Build `assumptions` with a wrong length, an out-of-range index, an index
    of a `rely` entry, a self index, a descending entry, and a well-formed
    array that adds one peer index the declaration does not derive, then one
    that drops a derived index, each on a `composition` item.
17. Build TC-985 case 1's composition request with a `rely` whose peer clause
    no declared guarantee covers at slot 4 and a `rely` whose peer guarantee
    is declared but conditional on an unsupplied fairness constraint at slot 3;
    then only the second; then two absent rely slots.
18. Build a composition request whose peer interfaces are all declared and not
    weaker, and fill the peer guarantee's slot with a result `inconclusive`,
    then `tested`, then leave it unfilled, then supply in `guarantees` only a guarantee over
    another subject; then fill every slot `proved` except that one `rely`
    slot's peer clause has a stronger but different supplied guarantee.

## Expected Results

1. `proved`, `decisive-witness`, every slot outcome and the conclusion
   record.
2. Each `invalid_runtime_input`/`invalid-value` at the member.
3. Each `invalid_runtime_input`/`wrong-value-kind` at the member.
4. `invalid_runtime_input`/`invalid-value` at the slot twice; then
   `inconclusive`, `premise-unproved` naming the no-divergence slot with
   `result` `absent`.
5. `invalid_runtime_input`/`invalid-value` at `/slots/<i>/result` twice;
   then FR-331's `missing-member` refusal at the slot.
6. `unsupported`, `incomplete` and `inconclusive`/`premise-unproved`, each
   naming the frame slot; then the initialization slot.
7. `inconclusive`, `premise-unproved` naming the model-claim slot; no
   `refuted`.
8. `proved` with `trusted`; then `proved` with `uncertified`.
9. `incomplete`, `limit-reached` naming `max_work`.
10. Each refused `unknown-cause`.
11. Each `inconclusive`, `transfer-not-preserving{reason: fragment}` naming
    the interval and the previous operator.
12. `inconclusive`, `premise-unproved` naming the model-claim slot; then
    `premise-unproved` naming the rely slot with `result` `assumption`.
13. Each `invalid_runtime_input`/`invalid-value` at `/slots/<i>/result`.
14. Each refused `invalid_result`/`unknown-cause` at `/cause/type`.
15. `proved` stands; then `inconclusive`, `provider-answer-unconfirmed` naming
    the second `component-guarantee` premise; then naming the first
    `component-guarantee` premise the request does not hold, for the absent
    item and for a request holding none; then naming the second premise for
    the replaced item and for the altered context, a held item never matching
    by clause node alone; the last case stands `proved`; never `replay-refused`.
16. Each refused `invalid_runtime_input`/`invalid-value` at
    `/request/items/<i>/assumptions`.
17. `refuted`, `decisive-counterexample`, `mismatch` naming slot 4's peer
    component, clause node and `reason` `absent` (the absent slot, though slot
    3 is `weaker`); then slot 3 with `weaker`; then the lower absent slot; each
    result's `slots` lists every slot outcome in slot order.
18. Each `inconclusive`, `premise-unproved` naming the slot with its result
    (`inconclusive`, `tested`, `absent`, and `absent` for the pool entry over
    another subject), never `refuted`; the last case `inconclusive`,
    `premise-unproved`.
