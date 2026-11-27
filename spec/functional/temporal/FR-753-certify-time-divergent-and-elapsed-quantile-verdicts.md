---
id: FR-753
title: "Certify time-divergent and elapsed-quantile verdicts"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-011
    type: implements
  - target: ix://agent-ix/filament-specification/FR-725
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-751
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-421
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-416
    type: depends_on
  - target: ix://agent-ix/filament-specification/FR-417
    type: references
  - target: ix://agent-ix/filament-specification/FR-407
    type: depends_on
---

# FR-753: Certify time-divergent and elapsed-quantile verdicts

## Description

When a timed subject has a cycle of moves that take no time, the provider
SHALL decide and certify its items over the time-divergent behaviours only,
and the checker SHALL find those cycles itself. When a claim is a quantile of
an elapsed measure, the provider SHALL decide and certify it by the quantile
transform over the product with the measure's elapsed clock, and the checker
SHALL rebuild that clock and the transform itself. Neither case settles
`unsupported`.

## Inputs

An FR-421 item and its FR-725 certificate.

## Outputs

FR-751's result.

## Behavior

### Time-divergent cycles

- The checker admits schedulers under which time diverges with probability 1
  (FR-416 "Time divergence"), so no scheduler may stay for ever among moves
  that take no time. The checker SHALL compute the maximal end
  components of the rebuilt product all of whose actions take
  zero time with probability one (discrete moves, races won at delay zero,
  idle observations), by its own end-component decomposition.
- Each such component's states SHALL carry one value. That value is checked
  against the component's exits, the enabled actions at its states that are
  not actions of the component: for a minimum it SHALL equal the least exit
  value, for a maximum the greatest, and ranks are checked on the component as
  one state. An action that stays in the component neither satisfies a
  progress condition nor bounds the value. This is the value over
  time-divergent schedulers, which must leave the component, and FR-412's
  plain conditions would admit the Zeno value a scheduler gets by staying.
- On the region route a positive-time move does not by itself make time
  diverge: a loop that resets one clock and delays it into a region below
  another clock's fraction returns to the same regions while the total delay
  converges. The checker SHALL apply the Alur–Dill progress condition: an end
  component is progressive when, for every clock of the key (the deadline and
  elapsed clocks included), an action of the component resets it, a delay
  move of the component takes it across an integer boundary, or it is at
  its cap at a state of the component. The checker SHALL treat every maximal
  end component that is not progressive exactly as a zero-time component
  above: one value, checked against its exits. A progressive end component
  keeps FR-412's plain conditions, because a scheduler may stay in it forever
  with time diverging. On the digital route every delay move adds a whole
  unit, so every end component that holds a delay move is progressive and only
  zero-time components are collapsed.
- A component with no exit, and a state where time cannot advance and no
  discrete move is enabled, is a time-lock
  ([FR-416](FR-416-evaluate-timed-temporal-claims.md) "Time-locks"). The
  checker SHALL read a run that ends there as FR-416 "How claims read a
  time-lock" states: no further position, so an event not yet met is never met
  and an `always` that the run's positions satisfy holds.

### Elapsed quantiles

- For `quantile q of (elapsed from holds(A) until holds(B) within W) <= c`,
  the measure activates at the first observation where `A` holds, completes
  at the first later observation where `B` holds, and is censored when its
  elapsed clock passes `W` first. The elapsed clock is reset at activation,
  advances with every delay move while active, stops at completion or
  censoring, and is capped at `W + 1`.
- Over every scheduler, the `<=` form holds exactly when the minimum of
  `E[[activated] · ([M <= c] − q)]` is at least 0, and the `>=` form exactly
  when `Pr(M < c | activated) < q` for every scheduler that activates, checked
  by FR-412 as an `upper` certificate of the maximum of
  `E[[activated] · ([M < c] − q)]`: below 0 it holds; at 0 the checker
  searches the actions that are tight under the certificate from the initial
  state and rejects `inequality-fails` at the first activated state it
  reaches.
- The checker SHALL derive activation, completion, censoring and the reward
  of every terminal state itself.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-753-AC-1 | TC-374's `Retx` with an added operation `toggle` that is always enabled, takes no time and flips a new Boolean `spun`: `Deadline` at 0.99 over every scheduler settles `proved`, `exact-value{99/100}`, and the checker accepts its certificate, in which the two states of each toggle pair carry one value. | Test (TC-1820) |
| FR-753-AC-2 | For AC-1's claim, a certificate that values every undecided state at 0, the value of a scheduler that toggles forever at time 0, is rejected `certificate-rejected`, rule `inequality-fails`, at a state of a toggle component. | Test (TC-1820) |
| FR-753-AC-3 | `expected elapsed until holds(m.delivered) >= 1 ms` over `Retx` and every scheduler settles `proved` on the minimum `10/9 ms`; an `exact` certificate that values the retry cycle, which holds delay moves, as though it took no time, giving the initial state `1 ms`, is rejected `certificate-rejected`, rule `inequality-fails`. | Test (TC-1820) |
| FR-753-AC-4 | Over `Retx` and every scheduler, with `M` = `elapsed from holds(not m.delivered) until holds(m.delivered) within 4 ms`: `quantile 0.5 of M <= 2 ms` settles `proved` (minimum of the transform `2/5`) with an accepted `exact` certificate; `quantile 0.5 of M <= 1 ms` settles `refuted` (minimum `−1/2`), and a certificate stating `1/2` at the initial state is rejected `inequality-fails`. | Test (TC-1820) |
| FR-753-AC-5 | Over the same model, `quantile 0.5 of M >= 1 ms` settles `proved` with an accepted `upper` certificate on the `region` route whose initial value is `−1/2`; `quantile 0.9 of M >= 2 ms` settles `refuted`, and an `upper` certificate with initial value `0` is rejected `inequality-fails` at the initial state, the first activated state its tight actions reach. | Test (TC-1820) |
| FR-753-AC-6 | `Retx` under a workload with `delay Msg::send ~ discrete { 5 ms: 1 }`, whose window excludes its only delay, is time-locked at its initial state: `probability <= 0.999 [eventually[0 ms, 4 ms] holds(m.delivered)]` settles `proved`, `exact-value{0}`, with an accepted certificate, and a certificate valuing that state at 1 is rejected `inequality-fails`. | Test (TC-1820) |
| FR-753-AC-7 | TC-374's `Retx` with a second clock `y` and an operation `again`, guarded `not m.delivered and y < 1 ms` and resetting `y`, which makes the subject strict and selects the `region` route: `Deadline` at 0.99 over every scheduler settles `proved`, `exact-value{99/100}`, with an accepted `region` certificate. The loop "reset `y`, delay into `0 < frac(y) < frac(x)`, reset `y`" returns to the same regions while `x` crosses no integer, so it is not progressive; an `exact` certificate that values every undecided state at 0, the value of a scheduler that loops there forever, is rejected `certificate-rejected`, rule `inequality-fails`, at a state of that component. | Test (TC-1820) |

## Dependencies

- [FR-725](../../objects/interfaces/FR-725-timed-product-probability-certificate.md),
  [FR-751](../analysis/FR-751-check-a-timed-product-certificate-independently.md).
- [FR-416](FR-416-evaluate-timed-temporal-claims.md) admits only
  time-divergent behaviours; [FR-417](FR-417-settle-time-lock-freedom.md)
  reports time-locks; [FR-407](../analysis/FR-407-state-probabilistic-claims.md)
  defines the quantile and elapsed forms.

## References

- L. de Alfaro, "How to specify and verify the long-run average behavior of
  probabilistic systems", 1998 (end-component quotients).
- M. Kwiatkowska, G. Norman, D. Parker and J. Sproston, "Performance analysis
  of probabilistic timed automata using digital clocks", 2006.
