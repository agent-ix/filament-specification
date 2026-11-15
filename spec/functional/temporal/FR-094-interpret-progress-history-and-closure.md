---
id: FR-094
title: "Interpret temporal progress, history and closure"
type: FR
relationships:
  - target: ix://agent-ix/filament-specification/US-008
    type: implements
  - target: ix://agent-ix/filament-specification/FR-091
    type: references
  - target: ix://agent-ix/filament-specification/FR-092
    type: references
  - target: ix://agent-ix/filament-specification/FR-243
    type: references
  - target: ix://agent-ix/filament-specification/FR-248
    type: references
  - target: ix://agent-ix/filament-specification/FR-252
    type: references
  - target: ix://agent-ix/filament-specification/FR-255
    type: references
  - target: ix://agent-ix/filament-specification/FR-256
    type: references
  - target: ix://agent-ix/filament-specification/FR-257
    type: references
  - target: ix://agent-ix/filament-specification/FR-258
    type: references
  - target: ix://agent-ix/filament-specification/FR-295
    type: references
  - target: ix://agent-ix/filament-specification/FR-289
    type: references
  - target: ix://agent-ix/filament-specification/FR-268
    type: references
  - target: ix://agent-ix/filament-specification/FR-294
    type: references
  - target: ix://agent-ix/filament-specification/FR-160
    type: references
  - target: ix://agent-ix/filament-specification/FR-114
    type: references
---
## Description

When an evaluator consumes a validated progress or closure assertion, it shall
apply that assertion only to the named clock, subjects, decision scope,
surrounding execution, interval and history boundary. The evaluator shall
preserve the independent progress/closure states and the FR-295 distinction
among complete, incomplete and contradicted inputs.

## Inputs

An active temporal obligation; its required future horizon and past-history
bound; exact decision-scope and surrounding-execution identities; validated
progress/completeness/closure assertions supplied by the observation authority;
and any earlier result identities affected by them.

## Outputs

An updated temporal disposition carrying assessment execution, independent
decision-scope and surrounding-execution progress/closure, settlement basis and
exact decision support through the shared result contract, or a typed
contradiction, incomplete or refused result. Historical result bytes are
retained when a later assertion invalidates their premises.

## Behavior

Progress shall be monotonic within its identified binding. A regressing or
same-revision conflicting assertion shall produce a typed contradiction or
refusal and shall not roll back progress, restamp closure or replace earlier
result bytes.

Progress shall cover an inclusive boundary. A progress watermark `W` is an inclusive
reading of tick `W`; an instant stamped `W` is covered, an instant stamped `W + 1`
is not.

Where a supplying observation contract uses a half-open window or watermark,
the evaluator shall consume only an FR-289 mapping that proves inclusive
coverage through the temporal boundary. It shall not reinterpret a half-open
end equal to the inclusive deadline as coverage of that deadline.

A timestamped watermark at `W` permits settlement of a
deadline `D` only when `W >= D` and completeness covers every required source,
subject and valuation through `D`. A fixed-sample progress assertion advances
sample ticks even when no business predicate is true. Event-position time shall
not advance during silence merely because wall time passed.

A complete closed decision scope shall authorize the selected closed-boundary
rule and settlement basis `closed-scope`. It shall not restamp the surrounding
execution as closed. An open decision scope shall leave unobserved admitted
continuations possible, but may settle from a `decisive-witness` or
`decisive-counterexample` only under [FR-091](FR-091-evaluate-bounded-future.md).
All other open futures remain `unsettled` and pending.

The evaluator shall classify end-of-file, consumer shutdown or a file labelled
closed without complete required observations as an incomplete input.

The evaluator shall not apply false extension or finite-window empty truth to
an incomplete input.

When a resource ceiling stops assessment of one obligation, it shall return a
non-Boolean resource disposition for that obligation. A sibling obligation that
already settled over complete exact decision support shall retain its prior
truth, settlement basis and support; the stop shall not erase or restamp it.

Assessment execution, decision-scope progress/closure, surrounding-execution
progress/closure, truth, settlement basis and completeness shall remain
independent. The temporal handoff shall retain the exact decision-support set.
A missing fact inside that set makes the associated truth unavailable; a
missing fact outside a completed support set remains a completeness/adequacy
gap without falsifying or delaying that settled truth.

Past evaluation shall require history through its computed lower boundary or an
authoritative execution origin. A later record inside an interval previously
declared complete is a completeness contradiction supplied by Agent F's
contract. The evaluator shall supersede or invalidate affected results without
rewriting their historical bytes. The v1 evaluator shall not infer watermark
authority or automatically repair settled history.

The progress an evaluator retains for a binding SHALL be carried from one
evaluation request to the next, and from one observation to the next of an
incremental assessment (FR-114), only as an explicit progress ledger that the
caller supplies and the evaluator returns; two requests carrying the same ledger
bytes apply the monotonic rule above exactly as one long-lived evaluator would,
and a request without a ledger asserts no retained progress.

### Late-data cutoff

A late-data cutoff decides which observations that arrive after progress has
moved past their event instant still enter an evaluation. It is distinct from
the history cutoff: a history cutoff bounds how far back retained history
reaches and makes earlier positions missing history; a late-data cutoff bounds
how late an observation may arrive and never makes history missing. Neither
substitutes for the other, and each keeps its own key (FR-268).

A selected late-data cutoff is on the clause's clock and carries a rule
identity and version. Two v1 rules exist:

| Rule | Parameter | An observation with event instant `t` admitted at arrival watermark `A` is `late` when |
| --- | --- | --- |
| `filament.temporal.late-data.allowed-lateness/v1` | allowed lateness `d >= 0` in the clock's unit | `A > t + d` |
| `filament.temporal.late-data.fixed-instant/v1` | cutoff instant `C` | `t <= C` and `A > C` |

The event instant `t` of an observation with an uncertainty interval is its
interval's upper end (FR-160), so uncertainty never makes an observation late
earlier than its latest possible instant. The arrival watermark `A` is the
progress watermark of the same binding at the moment the observation is
admitted; an observation supplied in a recorded trace carries its arrival
watermark explicitly, and one that carries none is admitted at the watermark
that precedes it in arrival order. Every other observation is `on-time`
(FR-294).

- Under a selected late-data cutoff a window with deadline `D` is established
  only when the watermark also covers `D + d` under the allowed-lateness rule,
  or `max(D, C)` under the fixed-instant rule, so no `on-time` observation can
  arrive after the window it belongs to has settled.
- The evaluator shall admit an `on-time` observation into every evaluation
  whose window it falls in, in event-time order, even when it arrives out of
  arrival order.
- The evaluator shall exclude a `late` observation from every evaluation. It
  shall retain the observation with its classification, its identity and the
  result identities whose windows it falls in, and report the late
  observations beside each such result.
- A late observation shall not rewrite, restamp or restart a settled result.
  When it contradicts a completeness assertion it raises the FR-295
  contradiction that FR-094-AC-5 states. A recomputation that admits it is a
  revisioned repair (FR-160) whose new result links the prior result and names
  the late observation.
- With no late-data cutoff selected, no observation is classified late:
  an observation that arrives after its window is established is a
  completeness contradiction when the window was asserted complete, and is
  otherwise admitted by revisioned repair only.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-094-AC-1 | A timestamped response exactly at an inclusive deadline participates before progress settles the window; the same response after the deadline does not. | Test (TC-115) |
| FR-094-AC-2 | With no business event, complete timestamp/sample progress through a deadline settles the applicable bounded obligation, while event-position time does not advance. | Test (TC-115) |
| FR-094-AC-3 | Open/closed decision-scope progress/closure, open/closed surrounding-execution progress/closure, assessment execution and FR-295 complete/incomplete/contradicted input remain independently represented; changing one axis cannot populate another. | Test (TC-116) |
| FR-094-AC-4 | Progress for a foreign clock, subject, source set or binding cannot settle the obligation. | Test (TC-115) |
| FR-094-AC-5 | A late completeness contradiction identifies the assertion and affected historical result; no earlier bytes or profile identity are restamped. | Test (TC-116) |
| FR-094-AC-6 | Missing predecessor/history on restart requires replay or an admitted state-restoration contract and cannot resume as a fresh complete stream. | Test (TC-118) |
| FR-094-AC-7 | `closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled` and `unavailable` are accepted only with their valid truth, scope and exact deciding-fact combinations; every one-axis substitution is refused. | Test (TC-116) |
| FR-094-AC-8 | A same-binding progress regression or conflicting assertion revision produces a typed contradiction/refusal and cannot roll back progress, closure or an earlier result identity. | Test (TC-115) |
| FR-094-AC-9 | A resource ceiling stopping one obligation leaves an independently settled sibling result unchanged, while the stopped obligation emits no Boolean fallback. | Test (TC-116) |
| FR-094-AC-10 | A half-open observation end/watermark equal to an inclusive deadline cannot settle it; an FR-289 carrier with exact same-clock coverage strictly beyond the endpoint admits the endpoint and filters later carrier members. | Test (TC-115) |
| FR-094-AC-11 | A progress watermark `W` covers an instant stamped `W` and does not cover one stamped `W + 1` (`FWV-45` to `FWV-47`, `proposals/finite-window/finite-window-vectors.json`). | Test (TC-980) |
| FR-094-AC-12 | Under `filament.temporal.late-data.allowed-lateness/v1` with `d` 2, over a window with deadline 5: the window is not established at watermark 6 and is established at watermark 7; an observation stamped 4 admitted at arrival watermark 6 is `on-time` and enters the evaluation; one stamped 3 admitted at arrival watermark 6 is `late`, is excluded from the result, is reported beside it with its identity, and leaves the result's bytes and identity unchanged. Under `filament.temporal.late-data.fixed-instant/v1` with `C` 5, the observation stamped 3 admitted at watermark 6 is `late` and one stamped 6 is `on-time`. An observation with interval `[2, 4]` under allowed lateness 2 admitted at watermark 6 is `on-time`. | Test (TC-1043) |
| FR-094-AC-13 | A late-data cutoff and a history cutoff selected together keep separate keys: changing either changes only its own key and the result identity; a late observation never makes history missing, a history cutoff never classifies an observation `late`, and a revisioned repair that admits a late observation produces a new result linked to the prior result and naming the observation. With no late-data cutoff selected, the same late arrival into a window asserted complete is a completeness contradiction and is not classified `late`. | Test (TC-1043) |

## Dependencies

- Agent F owns the validity, provenance and replay form of observation,
  progress, completeness and contradiction assertions. This requirement owns
  only their temporal interpretation.
- Agent B owns the shared result-dimension wire contract.

## Status

Accepted by FS04 for progress, history, closure, contradiction and retained
result distinctions. Executable end-to-end evidence remains Planned.
