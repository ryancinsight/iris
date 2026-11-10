# ADR 0003: Iris release scope for 1.0

- Status: Accepted
- Date: 2026-11-09
- Class: `[arch]` `[minor]`

## Context

Iris (`iris-viz` 0.1.4) provides domain-neutral visualization contracts for the
Atlas stack. The backlog contains open correctness, verification, API, release,
and documentation work (including IRIS-016, IRIS-003, IRIS-006, IRIS-010,
IRIS-011, IRIS-014, IRIS-015). A milestone ladder is required to order progress
to 1.0 with runnable exit checks.

## Decision

Adopt a three-milestone ladder to 1.0 recorded in `backlog/milestones.md`:

- **M-correctness** (0.1.5): resolve open correctness defects with
  value-semantic evidence and green gates.
- **M-contract-verification** (0.2.0): complete contract, API-surface, and
  verification/documentation items required for a stable 1.0 boundary.
- **M-1.0** (1.0.0): release-ready (authority-gated): release automation and
  package verification complete, public docs/examples current, and gates green
  with no open correctness items.

## Alternatives

- Ship 1.0 without intermediate verification milestones: rejected because open
  correctness and contract-verification items remain.
- Keep roadmap only in backlog prose: rejected because the stack requires a
  runnable milestone ladder.

## Consequences

- Open backlog items carry `milestone:` assignments aligned to the ladder.
- Milestones close only when their exit checks pass on fetched default commits;
  release/publication remains authority-gated.
- This ADR is rewritten in place as scope evidence changes.
