---
id: TC-2063
title: "A family guard part carries its quantifier"
type: TC
relationships:
  - target: ix://agent-ix/filament-specification/FR-322
    type: verifies
---
# TC-2063: A family guard part carries its quantifier

## Description

Verify the `quantifier` binding of a quantified guard part over a collection
with no declared maximum. Scope: FR-322-AC-479, FR-322-AC-484.

## Test Procedure

1. Over `us: Sequence<Timer>` with no declared maximum, read the package of
   the guard `forall(e in self.us: e.x < 5 ms)`, then of the same guard with
   `exists`.
2. Evaluate both guards with `us` empty and with `us` holding clocks at
   3 ms and 7 ms.
3. Read the package of the bounded guard `forall(e in self.ts: e.x < 5 ms)`
   over `ts: Sequence<Timer>[0, 3]`.
4. Read step 1's `forall` package with `quantifier` removed, the step 3
   package with `quantifier` `each` added to a part, and step 1's package
   with `quantifier` `all`.

5. Replay `CL-65` and `CL-66` and read guard node 47's `node_id`, and that
   of `CL-57`.

## Expected Results

- Step 1: one family part, `binders` `[e]`, `quantifier` `each` for `forall`
  and `some` for `exists`, one constraint with slot term `e`; the two guard
  node ids differ.
- Step 2: empty: `each` enabled, `some` not; 3 ms and 7 ms: `each` not
  enabled, `some` enabled.
- Step 3: the per-length parts, no `quantifier`, node id unchanged.
- Step 4: the missing binding refuses `invalid_package`/`timed-node-reference`
  at the part; the bounded part's `quantifier` refuses the same at the
  binding's value (`CL-64`); `all` fails the closed schema, `malformed_wire`
  (`CL-63`).
- Step 5: both admitted; the paths end `"quantifier", "0", "each"` and
  `"quantifier", "0", "some"`, the two ids differ, and `CL-57`'s path has no
  `quantifier` segment.

Test: `tests/checked_package_v2.rs::tc_2063_family_part_quantifier_is_in_the_guard_id_path`.
