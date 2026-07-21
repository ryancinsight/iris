# Backlog

## IRIS-001 — Public visualization foundation

- Outcome: public Iris repository with real shared color computation and two RITK consumers.
- Scope: Iris color/view/render contracts; `ritk-snap` and `ritk-vtk` color-law migration; Atlas registration.
- Non-goals: formats, GPU mechanics, UI state, domain interpretation, Kwavers migration.
- Acceptance: Iris gates pass; both RITK packages pass focused differential tests; public default revisions are anonymously readable; Atlas pins both merged defaults.
- Risk/class: `[arch]` `[minor]`.
- Status: in progress.

## Ready after IRIS-001

- Migrate Kwavers volume-renderer lookup tables when its active shared-tree claim closes.
- Extract repeated CFDrs/Kwavers plot-series assembly onto Iris borrowed views after consumer contract audit.
