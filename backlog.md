# Backlog

## IRIS-003 — Crates.io release automation

- Outcome: publish `iris-viz` from GitHub Releases without a long-lived registry
  token in repository secrets.
- Scope: package metadata, exact release identity validation, package dry-run,
  crates.io OIDC trusted publishing, and distribution documentation.
- Non-goals: version changes, additional packages, or consumer dependency
  updates.
- Acceptance: local package and repository gates pass; hosted CI passes; the
  merged `iris-viz` package is indexed on crates.io; the exact GitHub workflow is
  the crate's trusted publisher and trusted-publishing-only mode is enabled.
- Risk/class: `[patch]` release infrastructure.
- Status: in progress.
- Current state (2026-08-15, audited pre-change `origin/main` `bc47ce3`): the
  package identity is synchronized as registry package `iris-viz` with Rust
  import path `iris`, and hosted provider CI passes in run `31865323610`. A
  locked package dry run (`cargo package --locked`) and locked documentation
  build remain blocked in the Atlas checkout because Cargo attempts to rewrite
  `Cargo.lock` under the ambient path-patch graph. The hosted release run
  `31462641512` reached the OIDC token step but failed with HTTP 400 because no
  trusted publishing configuration exists for `ryancinsight/iris`; this is an
  external release blocker, not fixed by the provider documentation update.

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

- Extract repeated CFDrs/Kwavers plot-series assembly onto Iris borrowed views after consumer contract audit.

## IRIS-004 — Kwavers color-law consolidation

- Outcome: migrate Kwavers volume-render transfer functions to Iris so shared
  visualization laws are owned by one package.
- Scope: add missing Iris named maps required by Kwavers and remove duplicated
  map tables from `kwavers-analysis`.
- Non-goals: Kwavers GPU mechanics, domain interpretation, and plot-series
  extraction.
- Acceptance: Iris exposes `Inferno`, `Magma`, and `Turbo` as built-in
  `NamedColorMap` variants; `kwavers-analysis` consumes Iris `LookupTable` and
  contains no local implementations for those maps.
- Risk/class: `[arch]` `[minor]`.
- Status: done.

## IRIS-005 — ADR index governance slice

- Outcome: synchronize the generated ADR index with Iris's canonical ADR
  headers.
- Scope: `docs/adr/README.md`, the provider checklist, and this backlog entry.
- Non-goals: no ADR decision changes and no changes to the manual architecture
  overview `docs/adr/INDEX.md`.
- Acceptance: the generated index lists ADR 0001 and 0002 as `Accepted` and
  excludes the non-ADR `INDEX.md` overview.
- Risk/class: `[patch]` documentation cleanup.
- Evidence: both ADR files already carry `Status: Accepted`; the index now
  matches the generator's canonical output.
- Status: done 2026-08-14.

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
- Evidence: Iris PR 3 merged the provider law as `ef43861a`; CFDrs PR 303
  merged the direct consumer cutover as `394c9977`. The consumer passed 176
  `cfd-schematics` tests, 10 focused iterator/window tests, 16 doctests,
  warning-denied Clippy and Rustdoc, feature checks, and a rendered Venturi
  pressure-field inspection.
- Status: done.
