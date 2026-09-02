# Checklist

## IRIS-003

- [x] Confirm the `iris` name is not registered on crates.io; the registry
      package identity is `iris-viz` and the Rust import path remains `iris`.
- [x] Add exact-identity package validation and OIDC publishing automation.
- [ ] Pass format, feature, lint, Nextest, doctest, documentation, package,
      and hosted CI gates. Local gates now pass (`cargo fmt --check`,
      `cargo check --all-features`, warning-denied Clippy, 16/16 Nextest,
      doctests, warning-denied Rustdoc, and `cargo package --allow-dirty`).
      In the Atlas checkout overlay, `cargo package --locked` remains blocked
      because Cargo attempts to rewrite `Cargo.lock` under the ambient
      workspace patch graph.
- [ ] Merge the release automation and publish `iris-viz` 0.1.0.
- [ ] Configure the exact trusted publisher, enforce trusted-publishing-only,
      and verify the crates.io index and GitHub Release. External blocker:
      hosted release run `31462641512` failed at the OIDC token step with
      `No Trusted Publishing config found for repository ryancinsight/iris`;
      this repository change does not configure or claim to fix that setting.

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

## IRIS-005

- [x] Regenerate `docs/adr/README.md` from the two canonical `Accepted` ADR
      headers and remove the stale manual overview row from the generated
      index.

## gap-audit-2026-08-20 (owner: atlas-gap-audit)

Ordered execution steps for the items IRIS-006 … IRIS-015 filed by the
2026-08-20 scope-versus-delivery audit. Sequence is dependency-ordered:
verification first, because it decides whether any control point is wrong; then
the public-surface change, which must land before the first publish; then
documentation and hygiene.

- [x] Audit `9672fc0` against README, both Accepted ADRs, and the book SUMMARY;
      record measured evidence in `gap_audit.md`.
- [x] Correct the two false code claims in `docs/book/colormaps.md`: the
      table-defined maps are not `LookupTable` instances, and `LookupTable`
      selects the nearest entry rather than interpolating; `RenderBackend` is
      generic over the view type, not over `ColorMap`.
- [x] IRIS-007 — add the exhaustive-`match` coverage test for
      `NamedColorMap::ALL`. Smallest item, and it guards every later map
      change. **Already landed**: `all_contains_each_variant_once` in
      `src/color/map/named.rs` walks `ALL`, indexes each variant through the
      exhaustive `variant_index` match, and asserts both that no variant
      repeats and that none is omitted. Verified 2026-08-25 against the test
      as it stands, not against a claim about it.
- [ ] IRIS-006 — commit the published reference channel tables as a fixture,
      derive the per-channel bound, and assert each table-defined map against
      it. A deviation beyond the derived bound is a control-point defect to
      root-cause in `src/color/map/table.rs`, never a widened bound.
- [ ] IRIS-008 — implement the view-to-frame backend over
      `ScalarFieldView` and `LookupTable`, asserting frame bytes and storage
      reuse.
- [ ] IRIS-011 — replace `Normalized::from_u8` with `impl From<u8>`, converting
      every in-repo call site in the same change. Sequence before the first
      `iris-viz` publish or accept the major bump.
- [ ] IRIS-010 — move the error-trait impl off the `std` feature gate onto
      `core::error::Error` and assert it under `--no-default-features`.
- [ ] IRIS-013 — read `rustc --version` from a hosted CI run, confirm which
      toolchain the jobs actually use, then make the MSRV job exercise 1.95.
- [x] IRIS-009 and IRIS-012 — decide the ADR index output set once: restore or
      retire the generator, then delete the duplicate `docs/adr/INDEX.md`.
      Done in PR #22: the shared atlas guard is the generator; the duplicate is gone.
- [ ] IRIS-014 — verify each consumer row in
      `docs/book/stack_position.md` against that repository's manifest; drop or
      mark prospective the rows without evidence.
- [ ] IRIS-015 — add the lookup-table criterion benchmark with its committed
      budget and baseline.
