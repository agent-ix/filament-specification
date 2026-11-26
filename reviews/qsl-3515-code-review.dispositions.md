<!-- reviewer-dispositions repo=agent-ix/filament-specification visibility=public quoin=0.36.10 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-202611210022442313144108573 round=2 head=4f36fe8403147f609dbf762c45c10ce800ef4bdf mergeable=yes model=claude-sonnet-5-5 run=49252da8-71d5-448a-8791-504da64d95c4 -->

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-002 | fixed | 4f36fe84: AC-9 and its TC-1953 step are removed; the source of creatable keys is recorded Unconfirmed on QSL-3515. |
| FND-003 | fixed | 4f36fe84: FR-806 has both incomplete_population rows, states their order against the policy refusals, and AC-7 with TC-1956 step 6 tests it. |

+++ [reviewer data]

```yaml
{
 "dispositions": [
  {
   "fnd": "FND-002",
   "outcome": "fixed",
   "after_excerpt": "Verify safety pruning, non-safety and bounded policies, model invariants, infeasibility under policy\nand policy clauses with no instance. Scope: FR-803-AC-1 to FR-803-AC-8\nand FR-803-AC-10 to FR-803-AC-12."
  },
  {
   "fnd": "FND-003",
   "outcome": "fixed",
   "after_excerpt": "A `complete: false` population is refused by snapshot admission, so that\nrefusal SHALL be decided before the policy name and policy `over` binding\nrefusals. The refusal of an omitted population needs each policy clause's\n`over` type, so it SHALL be decided after every named policy clause has\nresolved and its `over` binding has been accepted. An omitted population that the\nsnapshot itself refers to is refused earlier, by snapshot admission."
  }
 ]
}
```

+++
