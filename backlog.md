# Backlog

## IRIS-001 — Public visualization foundation

- Outcome: public Iris repository with real shared color computation and two RITK consumers.
- Scope: Iris color/view/render contracts; `ritk-snap` and `ritk-vtk` color-law migration; Atlas registration.
- Non-goals: formats, GPU mechanics, UI state, domain interpretation, Kwavers migration.
- Acceptance: Iris gates pass; both RITK packages pass focused differential tests; public default revisions are anonymously readable; Atlas pins both merged defaults.
- Risk/class: `[arch]` `[minor]`.
- Evidence: Iris PR 1 merged as `e2edd476`; RITK PRs 46 and 47 merged the
  direct two-package consumer cutover and closure as `1bc665d4` and
  `a36e65df`; Atlas PR 71 registered the public provider and initial consumer
  revision as `6740296b`.
- Status: done.

## Ready after IRIS-001

- Migrate Kwavers volume-renderer lookup tables when its active shared-tree claim closes.
- Extract repeated CFDrs/Kwavers plot-series assembly onto Iris borrowed views after consumer contract audit.

## IRIS-002 — CFDrs color-law ownership

- Outcome: Iris owns the exact blue-to-red law required by CFDrs, allowing the
  consumer's duplicated map enum and formulas to be deleted.
- Scope: one additive map strategy, runtime selection, laws, documentation,
  and direct CFDrs migration.
- Non-goals: CFD field interpretation, Plotters rendering, scalar-range
  reduction, and the actively claimed Kwavers renderer.
- Acceptance: exhaustive channel laws and exact endpoint/complement tests pass;
  CFDrs consumes `NamedColorMap` directly and contains no local map enum or
  color formula.
- Risk/class: `[arch]` `[minor]` in Iris; `[arch]` `[major]` in CFDrs.
- Status: in progress.
