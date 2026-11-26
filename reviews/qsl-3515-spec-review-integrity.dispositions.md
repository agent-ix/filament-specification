<!-- reviewer-dispositions repo=agent-ix/filament-specification visibility=public quoin=0.36.10 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-202611210022442313144108574 round=2 head=4f36fe8403147f609dbf762c45c10ce800ef4bdf mergeable=yes model=claude-sonnet-5-5 run=49252da8-71d5-448a-8791-504da64d95c4 -->

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-002 | fixed | 4f36fe84: each population's universe is the snapshot's listed keys in list order, so FR-361's instance order is defined for a plan. |

+++ [reviewer data]

```yaml
{
 "dispositions": [
  {
   "fnd": "FND-002",
   "outcome": "fixed",
   "after_excerpt": "For this rule the universe of each population is the keys of the\n  objects the initial snapshot lists for it, in the snapshot's list order, so\n  the admitted keys are the distinct keys of the objects the snapshot holds\n  in the populations that overlap `T`, and FR-361's instance order is read\n  over those lists. Each instance has its own monitor or automaton. A key\n  under which the snapshot holds no object is not an instance, even when a\n  step creates an object under it."
  }
 ]
}
```

+++
