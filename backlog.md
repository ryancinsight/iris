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

## IRIS-006 — Reference-value agreement for palette-approximating maps

- Outcome: every built-in map that documents itself as an approximation of a
  published palette carries a measured, bounded agreement test against that
  published reference, or its documentation stops naming the palette.
- Scope: `Bone`, `Jet`, `Plasma`, `Viridis`, `Inferno`, `Magma`, `Turbo` in
  `src/color/map/table.rs`; a committed reference fixture of the published
  channel tables; a derived per-channel error bound; the map docstrings.
- Non-goals: changing any control point without a recorded derivation; adding
  CAM02-UCS or Lab machinery; claiming perceptual uniformity.
- Acceptance oracle: for each named map, the maximum per-channel deviation from
  the published reference table over the 1,025-point grid is asserted against a
  bound stated in the docstring together with its derivation; the
  self-referential assertions in `tests/color_laws.rs:114-178` are replaced by
  or supplemented with reference-table comparisons.
- Evidence of the gap: `src/color/map/table.rs:108-144` documents Viridis as a
  "Five-control-point approximation of the viridis palette", and
  `tests/color_laws.rs:143-149` asserts `Viridis(0.5) == [0.204, 0.636, 0.469]`
  — the crate's own control point from `table.rs:118,126,133`. No test compares
  any built-in map to a published table, and no approximation error bound is
  stated anywhere in the repository.
- Dependencies: none.
- Risk/change class: `[verification]` `[patch]` (tests and docs only; a control
  point found to be wrong reclassifies the fix to `[minor]`).
- Effort: M.
- Status: todo.

## IRIS-007 — Prove `NamedColorMap::ALL` covers every variant

- Outcome: adding a `NamedColorMap` variant without adding it to `ALL` fails to
  compile or fails a test, so the exhaustive-grid law test cannot silently skip
  a map.
- Scope: one test in `tests/color_laws.rs` that matches exhaustively over
  `NamedColorMap` and asserts membership in `ALL`.
- Non-goals: changing the stable display order of `ALL` or the
  `#[non_exhaustive]` attribute.
- Acceptance oracle: the new test contains an exhaustive `match` over every
  variant with no wildcard arm, and asserts that `ALL` has the same length as
  the variant count and contains each variant exactly once.
- Evidence of the gap: `src/color/map/named.rs:55-70` hardcodes
  `pub const ALL: [Self; 14]` with no compile-time or test-time link to the
  enum declaration at `named.rs:16-51`; `tests/color_laws.rs:39-50` iterates
  `ALL` only, so an omitted variant is never sampled.
- Dependencies: none.
- Risk/change class: `[verification]` `[patch]`.
- Effort: S.
- Status: todo.

## IRIS-008 — Exercise the render seam with a real view-to-frame backend

- Outcome: the `RenderBackend` seam is demonstrated end to end by a backend
  that renders an Iris view through an Iris color law into borrowed frame
  storage, establishing that the declared contract composes with the crate's
  own view and color types.
- Scope: one in-repo backend, as a test or an example, implementing
  `RenderBackend<ScalarFieldView<'_, f32, 2>>` that maps values through a
  `LookupTable` into a reused RGBA byte buffer; value-semantic assertions on
  the resulting frame; a second `render` call establishing storage reuse.
- Non-goals: a GPU backend, an image encoder, a windowing layer, or any new
  dependency.
- Acceptance oracle: the frame bytes are asserted against values computed
  independently from the color law for a known field, and two successive frames
  are shown to occupy the same backing storage.
- Evidence of the gap: the only implementor in the repository is
  `tests/view_contracts.rs:84-95`, `impl RenderBackend<[u8]> for
  ByteFrameBackend`, which copies four bytes and touches neither a view type
  nor a color map; `src/render/backend.rs:7` leaves `V` entirely unbounded, so
  nothing ties the seam to `ScalarField`.
- Dependencies: none.
- Risk/change class: `[verification]` `[patch]`.
- Effort: M.
- Status: todo.

## IRIS-009 — Restore or retire the ADR index generator

- Outcome: the generator provenance stated in `docs/adr/README.md` is true —
  either the generator exists and CI runs regenerate-and-diff, or the header
  stops naming a file that is not in the repository.
- Scope: the `docs/adr/README.md` header and, if the generator is restored, a
  committed generator plus a CI freshness check.
- Non-goals: ADR decision changes; the implementation language is settled by
  the repository's existing scripting convention at implementation time.
- Acceptance oracle: an index check command runs in CI and fails on a
  hand-edited index; or the "Generated by … do not hand-edit" header is gone
  and the index is declared hand-maintained.
- Evidence of the gap: `docs/adr/README.md:3-5` instructs
  `python scripts/adr-index.py generate`, but the repository has no `scripts/`
  directory at `9672fc0` (`ls -d scripts` finds no such path).
- Dependencies: none.
- Risk/change class: `[docs]` `[patch]`.
- Effort: S.
- Status: todo.

## IRIS-010 — Implement the error trait unconditionally

- Outcome: `IrisError` implements the error trait for `no_std` consumers, not
  only under the `std` feature.
- Scope: `src/error.rs`; the feature table in `docs/book/stack_position.md`.
- Non-goals: adding a source chain, which the enum has no wrapped causes for;
  adding an error-handling dependency.
- Acceptance oracle: `cargo check --no-default-features` passes with an
  unconditional `impl core::error::Error for IrisError`, and a test asserts the
  impl is reachable under `--no-default-features`.
- Evidence of the gap: `src/error.rs:75-76` gates the impl on
  `#[cfg(feature = "std")]`. `core::error::Error` is stable far below the
  crate's `rust-version = "1.95"` (`Cargo.toml:12`), so the gate denies
  `no_std` consumers an error trait for no reason, while `src/lib.rs:7-8`
  advertises `no_std` compatibility.
- Dependencies: none.
- Risk/change class: `[arch]` `[minor]` (additive trait impl on a public type).
- Effort: S.
- Status: todo.

## IRIS-011 — Replace `Normalized::from_u8` with a `From` impl

- Outcome: the byte-to-unit-interval conversion is expressed through the std
  conversion lattice, and no public identifier carries a scalar-type suffix.
- Scope: `src/color/normalized.rs`, its doctest at `normalized.rs:47-54`, the
  book snippet in `docs/book/colormaps.md`, and the in-repo call sites at
  `tests/color_laws.rs:24-33` and `tests/color_laws.rs:99`.
- Non-goals: changing the numeric definition `n = value / 255`, which is exact
  and tested; adding a `TryFrom`, since the conversion is infallible.
- Acceptance oracle: `impl From<u8> for Normalized` exists, `from_u8` is gone
  rather than deprecated or re-exported, every in-repo call site is converted
  in the same change, and the exhaustive grid test at
  `tests/color_laws.rs:22-36` still passes bit for bit.
- Evidence of the gap: `src/color/normalized.rs:56` declares
  `pub fn from_u8(value: u8) -> Self`, the only type-suffixed identifier under
  `src/`, while `grep 'impl.*From<' src` returns no matches — a bespoke
  constructor where the std lattice applies.
- Dependencies: this removes a public item; sequence it before the first
  `iris-viz` publish (IRIS-003) or accept the major bump.
- Risk/change class: `[arch]` `[major]` (public item removal).
- Effort: S.
- Status: todo.

## IRIS-012 — Retire the duplicate ADR index `docs/adr/INDEX.md`

- Outcome: exactly one ADR index exists.
- Scope: delete `docs/adr/INDEX.md`, folding any content it carries that
  `docs/adr/README.md` lacks into the ADR bodies; update any reference to it.
- Non-goals: ADR decision changes.
- Acceptance oracle: `docs/adr/` contains one index file, and no repository
  file references `INDEX.md`.
- Evidence of the gap: `docs/adr/README.md` and `docs/adr/INDEX.md` both
  tabulate ADR 0001 and 0002 with status, in different wording. IRIS-005
  explicitly declared `INDEX.md` a non-goal (`backlog.md:62-63`), which left two
  parallel indexes as the settled state rather than resolving the duplication.
- Dependencies: sequence with IRIS-009 so the index output set is decided once.
- Risk/change class: `[pm-hygiene]` `[patch]`.
- Effort: S.
- Status: todo.

## IRIS-013 — Make CI actually exercise the declared MSRV floor

- Outcome: the `rust-version = "1.95"` claim is verified by a build that
  demonstrably runs on 1.95.
- Scope: `.github/workflows/ci.yml` and `rust-toolchain.toml`.
- Non-goals: lowering or raising the MSRV.
- Acceptance oracle: the MSRV job prints a `rustc --version` showing 1.95 and
  builds the crate; a dependency raising its MSRV above the floor fails that
  job.
- Evidence of the gap: `.github/workflows/ci.yml:25` and `:45` request
  `toolchain: 1.95.0`, but `rust-toolchain.toml:2` pins `channel = "1.97.0"`,
  and a committed `rust-toolchain.toml` is a rustup directory override that
  takes precedence over an installed default toolchain. The workflow therefore
  very likely compiles with 1.97.0, leaving the `Cargo.toml:12` floor
  unexercised. Confirm by reading `rustc --version` in a hosted run before
  choosing the fix — an explicit `cargo +1.95.0`, or suppressing the override
  for that job.
- Dependencies: none.
- Risk/change class: `[verification]` `[patch]`.
- Effort: S.
- Status: todo.

## IRIS-014 — Ground the book's consumer table in evidence

- Outcome: the consumer table in `docs/book/stack_position.md` lists only
  consumers whose Iris dependency is verifiable, each with its evidence.
- Scope: the consumer table in `docs/book/stack_position.md`.
- Non-goals: changing any consumer repository; adding consumers.
- Acceptance oracle: every row names a consumer whose manifest depends on
  `iris-viz`, checked against that repository at a named revision; unverified
  rows are removed or marked prospective.
- Evidence of the gap: the table lists `helios`, `kwavers`, `ritk`, and
  `CFDrs`. This repository carries merge evidence for `ritk`
  (`backlog.md:33-36`), `CFDrs` (`backlog.md:83-87`), and `kwavers`
  (IRIS-004, `backlog.md:43-55`), but none for `helios`. Verification requires
  reading the consumer repositories, which was outside the scope of the
  2026-08-20 single-repository audit.
- Dependencies: read access to the consumer repositories.
- Risk/change class: `[docs]` `[patch]`.
- Effort: S.
- Status: todo.

## IRIS-015 — Measure the lookup-table fast path

- Outcome: the performance premise behind `LookupTable` — that a cached
  nearest-entry lookup beats direct map evaluation by enough to justify the
  type — is measured rather than assumed.
- Scope: one criterion benchmark comparing `LookupTable::<M, N>::sample` with
  `M::sample` across an analytic law (`Grayscale`, `BlueRed`), a table-defined
  law (`Viridis`, `Turbo`), and the runtime-dispatched `NamedColorMap`, at two
  or three resolutions; a committed per-binary wall-clock budget.
- Non-goals: optimizing any map before a profile identifies its bound; a
  benchmark whose single-iteration smoke run cannot fit the 30 s budget in
  `.config/nextest.toml:2`.
- Acceptance oracle: the benchmark smoke-runs inside the test budget in CI, and
  the committed baseline records median and confidence interval per case with
  the machine class in the header.
- Evidence of the gap: the repository has no `benches/` directory at `9672fc0`;
  `src/color/lookup_table.rs` exists purely as a precomputation cache, and the
  README asserts allocation-free, dispatch-free operation with no timing
  evidence behind it.
- Dependencies: none.
- Risk/change class: `[perf]` `[patch]`.
- Effort: S.
- Status: todo.
