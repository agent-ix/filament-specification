---
id: TC-1603
title: "Plan and optimize with certificates"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-650
    type: verifies
---

# TC-1603: Plan and optimize with certificates

## Description

Verify plan synthesis over a state model, policy, objectives and the optimality and closure certificates. Scope: FR-650-AC-1 to FR-650-AC-6.

## Test Procedure

1. Run `Reach` over the `inc`/`double` counter model from `value = 1` with goal value 8 and plan lengths `Int[0, 5]`, first minimizing length and then the per-step cost sum.
2. Run it with goal value 21, and with lengths `Int[0, 2]`.
3. Add the policy `always holds(c.value != 4)`, a policy naming no clause and the policy `eventually holds(c.value = 3)`, which is evaluated on each completed plan.
4. Tamper with the certificate; stop a run by its work limit; inject a plan with a disabled invocation.

## Expected Results

- Step 1: `optimal` `inc, double, double` at 3 with an accepted `potential` certificate, `certified`; then `optimal` `inc, inc, inc, double` at 8.
- Step 2: `infeasible` with accepted closure certificates.
- Step 3: the plan avoids value 4 and replays with the policy holding; the missing policy refuses as stated, and the `eventually` policy gives `optimal` `inc, inc, inc, double`.
- Step 4: the certificate rejected naming the rule and entry, result `feasible`; `incomplete` with its best plan; a method failure naming the disabled step.
