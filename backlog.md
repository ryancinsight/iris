# Backlog

## IRIS-003 — Crates.io release automation

- Outcome: publish `iris` from GitHub Releases without a long-lived registry
  token in repository secrets.
- Scope: package metadata, exact release identity validation, package dry-run,
  crates.io OIDC trusted publishing, and distribution documentation.
- Non-goals: version changes, additional packages, or consumer dependency
  updates.
- Acceptance: local package and repository gates pass; hosted CI passes; the
  merged package is indexed on crates.io; the exact GitHub workflow is the
  crate's trusted publisher and trusted-publishing-only mode is enabled.
- Risk/class: `[patch]` release infrastructure.
- Status: in progress.
- Current state (2026-08-11): local repository gates pass in the Atlas
  checkout (`fmt`, feature checks, warning-denied Clippy, Nextest, doctest,
  warning-denied Rustdoc, and `cargo package --allow-dirty`). A locked
  package dry run (`cargo package --locked`) is still blocked in this overlay
  because Cargo attempts to rewrite `Cargo.lock` under the ambient path-patch
  graph; hosted CI/publish/trusted-publisher steps remain open.

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
