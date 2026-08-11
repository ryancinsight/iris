# Checklist

## IRIS-003

- [x] Confirm the `iris` name is not registered on crates.io.
- [x] Add exact-identity package validation and OIDC publishing automation.
- [ ] Pass format, feature, lint, Nextest, doctest, documentation, package,
      and hosted CI gates. Local gates now pass (`cargo fmt --check`,
      `cargo check --all-features`, warning-denied Clippy, 16/16 Nextest,
      doctests, warning-denied Rustdoc, and `cargo package --allow-dirty`).
      In the Atlas checkout overlay, `cargo package --locked` remains blocked
      because Cargo attempts to rewrite `Cargo.lock` under the ambient
      workspace patch graph.
- [ ] Merge the release automation and publish `iris` 0.1.0.
- [ ] Configure the exact trusted publisher, enforce trusted-publishing-only,
      and verify the crates.io index and GitHub Release.

## IRIS-001

- [x] Confirm the Atlas promotion gate and absence of an existing Iris repository.
- [x] Audit duplicated visualization laws and define ownership/non-goals in ADR 0001.
- [x] Implement normalized colors, ZST maps, fixed lookup tables, borrowed views, and render seam.
- [x] Pass format, Clippy, 14/14 Nextest, two doctests, warning-clean Rustdoc,
      feature-seam, cargo-deny, example, and package gates.
- [x] Publish the public repository and verify anonymous access at merged
      revision `e2edd476`.
- [x] Migrate `ritk-snap` and `ritk-vtk`; delete their duplicated color
      computations through RITK PR 46 and close exact-head verification in
      PR 47 at `a36e65df`.
- [x] Register Iris and the initial merged RITK consumer revision through
      Atlas PR 71 at `6740296b`.

## IRIS-002

- [x] Confirm the CFDrs color-law duplicate and the active Kwavers lane.
- [x] Specify the exact blue-to-red law and monotonic/complement proof obligations.
- [x] Implement the zero-sized map and closed-set runtime variant.
- [x] Pass format, dual-feature checks, warning-denied Clippy, 15/15 Nextest,
      two doctests, warning-denied Rustdoc, examples, cargo-deny, and the
      origin-main SemVer baseline (196/196 checks).
- [x] Migrate CFDrs directly and delete its superseded enum and formulas.
- [x] Publish Iris and CFDrs defaults as `ef43861a` and `394c9977`;
      parent-repository pin reconciliation remains owned by Atlas.

## IRIS-004

- [x] Add `Inferno`, `Magma`, and `Turbo` to Iris map strategies and
      `NamedColorMap` runtime dispatch.
- [x] Extend Iris color-law tests for the new built-in maps.
- [x] Migrate `kwavers-analysis` volume-render transfer functions to Iris
      `LookupTable<NamedColorMap, 256>`.
