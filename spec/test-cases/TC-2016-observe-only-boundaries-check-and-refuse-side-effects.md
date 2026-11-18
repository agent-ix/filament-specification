---
id: TC-2016
title: "Observe-only boundaries check and refuse side effects"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-822
    type: verifies
---

# TC-2016: Observe-only boundaries check and refuse side effects

## Description

Verify the source form of observe-only roles and controls and the refusal of
every side-effecting action behind them. Scope: FR-822-AC-1 and FR-822-AC-2.

## Test Procedure

1. Check a clause with `observe role Auditor on Orders::Order;` whose `run`
   holds a `receive` on a channel to `Auditor` and a `recommend` by `Auditor`,
   and a clause whose `run` holds `observe sequence Review` with a `check` and a
   `recommend`; check both again with `observe` removed.
2. Parse `observe observe sequence ...` and check a clause with a capture named
   `observe`.
3. Place each of an `attempt`, `effect`, `send`, `commit` and an `event ... for`
   inside `Review`, then outside it.
4. Under `observe role Auditor`, check an `attempt by Auditor`, a `send` from
   `Auditor`, a compensation template by `Auditor`, a `receive` to `Auditor`, a
   `choice by Auditor` and a `check`.

## Expected Results

- Step 1: both clauses check; without `observe` they check to equal clauses
  except for the recorded boundary.
- Step 2: `invalid_syntax`/`unexpected-token` at the second `observe`; the
  capture checks.
- Step 3: inside, each refuses `ill_typed`/`operator-ineligible` at exactly the
  action's declaration text with `action` naming its form and `boundary`
  `Review`; outside, each checks.
- Step 4: the attempt, send and template refuse with `boundary` `Auditor`
  (`action` `compensate` for the template); the receive, choice and check
  check; two offending actions give two refusals in source order.
