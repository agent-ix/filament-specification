---
id: TC-1360
title: "Precondition and difference facts discharge range obligations"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-146
    type: verifies
---

# TC-1360: Precondition and difference facts discharge range obligations

## Description

Check that a postcondition's range obligation is discharged by exactly the
FR-146 precondition facts (item 6) and difference facts (item 7), and by
nothing broader.

## Test Procedure

Select `filament.model.complete/v1`. The bank fixture is a model with object
type `Account` (`balance: Int[0, 100]`, `limit: Int[0, 100]`), a population of
`Account`, and the operation `withdraw(amount: Int[0, 100])` with frame
`modifies [Account/balance]`, redefining nothing and redefined by nothing. The
base unit declares `pre CanAfford` on `Account::withdraw` with the body
`amount <= self.balance` and `post Debited` on `Account::withdraw` with the body
`self.balance = pre(self.balance) - amount`. "Refused at `D`" means
`refused { code: undefined_expression, cause: unproved-range }` at node `D`.

| Vector | Change to the base unit | Expected |
| --- | --- | --- |
| P01 | none | checks; `pre(self.balance) - amount` is typed `Int[0, 100]` |
| P02 | `CanAfford` declared after `Debited` | checks, with the same clause node identities as P01 |
| P03 | `CanAfford` removed | refused at `pre(self.balance) - amount` (proved `[-100, 100]`, required `[0, 100]`) |
| P04 | `CanAfford` on a second operation `deposit(amount: Int[0, 100])` instead | refused at `pre(self.balance) - amount` |
| P05 | a subtype `Premium` redeclares `withdraw` with `redefines Account/withdraw`, frame unchanged | refused at `pre(self.balance) - amount` |
| P06 | `CanAfford` on `Premium::withdraw` (inherited) instead | refused at `pre(self.balance) - amount` in `Debited` |
| P07 | `invariant Room` on `Account` with body `self.limit = self.balance - self.limit`, and `CanAfford`'s body `amount <= self.balance and self.limit <= self.balance` | refused at `self.balance - self.limit` |
| P08 | `CanAfford`'s body `amount < self.balance` and `Debited`'s body `self.balance = (pre(self.balance) - amount) - 1` | checks |
| P09 | `CanAfford`'s body `self.balance >= amount`; then `not (amount > self.balance)` | checks; checks |
| P10 | `CanAfford`'s body `amount = self.balance`; `amount + 0 <= self.balance`; `amount <= self.limit and self.limit <= self.balance`; `amount <= self.balance or amount <= 5` | each refused at `pre(self.balance) - amount` |
| P11 | `CanAfford`'s body `self.balance + amount <= 100` and `Debited`'s body `self.balance = pre(self.balance) + amount`; then `CanAfford`'s body `self.balance + amount <= 101` | checks, `pre(self.balance) + amount` typed `Int[0, 100]` by the transferred sum bound; refused at `pre(self.balance) + amount` (proved `[0, 101]`) |
| P12 | `CanAfford`'s body `amount < 0 and amount <= self.balance` (unreachable true outcome) | refused at `pre(self.balance) - amount` |
| P13 | the function `function f using v(a: Int[0, 100], b: Int[0, 100], r: Int[0, 100]): Boolean pure { a <= b implies r = b - a }` | checks |

## Expected Results

Every vector gives exactly its expected outcome, and no transferred fact adds a
node, obligation record or requirement record to the checked package.
