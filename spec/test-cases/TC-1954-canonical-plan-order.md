---
id: TC-1954
title: "Canonical plan order and deterministic outcomes"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-804
    type: verifies
  - target: ix://agent-ix/filament-specification/FR-181-AC-34
    type: verifies
---

# TC-1954: Canonical plan order and deterministic outcomes

## Description

Verify the tie keys, native invocation/outcome order and byte-identical outcomes across worker counts and limits. Scope: FR-804-AC-1 to FR-804-AC-6 and FR-181-AC-34.

## Test Procedure

1. Plan FR-650-AC-1's counter.
2. Plan with an objective on which a 2-step and a 3-step plan tie.
3. Order `move(a, c1)`, `move(b, c0)` and `move(a, c0)`.
4. Run one request with one and eight workers, and twice under one `plan_search.max_work`.
5. Use two operations `first(flag: Boolean)` and `second(flag: Boolean)`
   in that declaration order, each with live receivers `a`, `b` of the
   same actual object type and universe, with identity bytes `00` and
   `01`, so their FR-181 reference values order `a < b`. Both flags are admitted. Enumerate
   invocation tuples from a state containing both live receivers.
6. Over TC-1951's finite `v`/result domains, use a postcondition admitting
   exactly `(0, S0)`, `(0, S2)`, `(1, S0)` for one invocation. Supply the
   native outcomes in different orders and with a repeated equal pair.
   Re-derive their canonical order for search, replay and certificate
   checking, comparing full state-key bytes for the second key. Read
   the native typed result/post-state relation before simulation graph
   coalescing: `(0,S0)` and `(1,S0)` remain two planning outcomes even
   if the simulator records their one transition/post-state graph edge.
7. In separate variants of TC-1951's named native model, change `choose`'s
   result type to `Float32` or `Float64`, fix its frame to the unchanged
   state `S0`, and admit exactly the four result bit patterns in the table
   below. Select the existing IEEE 754 default floating profile, domain
   `Int[1,1]`, true goal, empty policy and objective `length`. Reorder the
   result generation; do not turn the result into the objective.

   | Result type | Canonical result bits at ordinals 0, 1, 2, 3 |
   | --- | --- |
   | `Float32` | `80000000`, `00000000`, `7fc00001`, `7fc00002` |
   | `Float64` | `8000000000000000`, `0000000000000000`, `7ff8000000000001`, `7ff8000000000002` |

## Expected Results

- Step 1: `inc, double, double`.
- Step 2: the 2-step plan.
- Step 3: `move(a, c0)`, `move(a, c1)`, `move(b, c0)`.
- Step 4: byte-identical documents.
- Step 5: `first(a,false)`, `first(a,true)`, `first(b,false)`,
  `first(b,true)`, then the four `second` invocations in the same receiver
  and argument order. Receiver order precedes argument order.
- Step 6: ordinals 0, 1, 2 select `(0, S0)`, `(0, S2)`, `(1, S0)`;
  a repeated equal pair has no additional ordinal or visited edge.
  Search, replay and certificate checking derive that same order from
  the uncoalesced native relation, with no serialized trace result member.
- Step 7: selected-profile `totalOrder` gives the listed ordinals, keeping
  negative zero, positive zero and both NaN payloads distinct. The
  objective remains integer `length`; `explored.nan_objectives` is 0,
  because a NaN operation result is not a NaN objective. Equal requests
  give the same selected step identity independent of generation order.
