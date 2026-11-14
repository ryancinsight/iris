# Backlog

<a id="IRIS-017"></a>
## IRIS-017 — Add metis to the consumer table and correct its wording
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: `## Consumers` in `docs/book/stack_position.md` lists every stack
  repository whose manifests declare `iris-viz`, and its prose is true of each row.
- Scope: that table and its introduction.
- Gap: metis declares `iris` under `[dependencies]` in `metis-frontend` and
  `metis-ui-lang` and uses `RenderBackend` there (metis `e5395a37ff63`), but has
  no row. "Each row names a package" is false for the virtual workspaces `CFDrs`
  and `ritk`, and `cfd-1d` and `cfd-2d` declare `iris` only as a dev-dependency.
- Oracle: at each member's default-branch commit the repositories whose
  `Cargo.toml` files name `iris-viz` equal the table's rows; the wording says
  "repository" and marks dev-only manifests. Fails at the basis (no metis row).
- Non-goals: the generator (IRIS-019); the `RenderBackend` sentence (IRIS-018).
- Class: `[docs]` `[patch]`. basis: 2661cc561084944b46a59ed0e5ecbb8d191a9f33.

<a id="IRIS-018"></a>
## IRIS-018 — Correct who supplies the `RenderBackend` implementation
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: `docs/book/stack_position.md` states which stack consumers implement
  `RenderBackend` and does not claim it of the others.
- Scope: the "Where Iris Sits" paragraph (`stack_position.md:29-30`).
- Gap: it says consumers "supply the `RenderBackend` implementation". `git grep
  RenderBackend` finds no match in `CFDrs` or `ritk` at their default
  branches; only metis (`metis-frontend`, `metis-ui-lang`) uses it.
- Oracle: for every repository the sentence names as an implementor, `git grep
  "impl.*RenderBackend"` at its default-branch commit matches; none is named
  otherwise. Fails at the basis for `CFDrs` and `ritk`.
- Non-goals: the consumer table (IRIS-017).
- Class: `[docs]` `[patch]`. basis: 2661cc561084944b46a59ed0e5ecbb8d191a9f33.

<a id="IRIS-019"></a>
## IRIS-019 — Check the consumer table against consumer manifests
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: the consumer table cannot drift from the stack's manifests; a committed
  check regenerates its manifest and coverage columns and diffs them.
- Scope: a stack-owned generator in atlas `scripts/` (reverse dependencies need
  every member's `cargo metadata`), a marked generated region in
  `stack_position.md`, and its invocation in iris CI.
- Oracle: (a) at each row's recorded revision the generated manifest column equals
  the table, failing on a hand edit; (b) a scheduled meta scan at current default
  commits fails when a repository depending on `iris-viz` has no row, shown by a
  synthetic omitted row.
- Non-goals: the "Iris items imported" prose column.
- Next: the lead files the atlas meta item that owns the generator and links here.
- Class: `[verification]` `[patch]`. basis: 2661cc561084944b46a59ed0e5ecbb8d191a9f33.

<a id="IRIS-011"></a>
## IRIS-011 — Replace `Normalized::from_u8` with a `From` impl
- Status: todo; priority: tightening; milestone: M-contract-verification; needs: none in iris (cross-repo: ritk).
- Outcome: the byte-to-unit-interval conversion uses the std conversion lattice
  and no public identifier carries a scalar-type suffix.
- Scope: `src/color/normalized.rs` (`from_u8` at :68, doctest :64-65),
  `docs/book/colormaps.md:35,52`, `tests/color_laws.rs:24,28,99`.
- Oracle: `impl From<u8> for Normalized` exists, `from_u8` is gone with no alias,
  every in-repo site is converted, and
  `byte_coordinates_cover_the_uniform_unit_grid_exactly` passes bit for bit
  (`n = value / 255` unchanged; no `TryFrom`).
- Co-evolution: ritk calls `Normalized::from_u8` at 8 sites in `ritk-snap` and
  `ritk-vtk` (ritk `1f480cce6b1f`); `CFDrs` and metis call none. The removal
  breaks 0.1.x, which 0.2.0 admits. An unfiled ritk item prepares the migration
  against this candidate before this PR is ready and lands after it.
- Class: `[arch]` `[major]` (public item removal). basis: 2661cc561084944b46a59ed0e5ecbb8d191a9f33.

<a id="IRIS-015"></a>
## IRIS-015 — Measure the lookup-table fast path
- Status: todo; priority: tightening; milestone: M-contract-verification; needs: none.
- Outcome: the premise behind `LookupTable` (a cached nearest-entry lookup beats
  direct map evaluation) is measured.
- Scope: criterion bench `lookup_table_sample` comparing
  `LookupTable::<M, N>::sample` with `M::sample` for `Grayscale`, `BlueRed`,
  `Viridis`, `Turbo` and `NamedColorMap` at two or three resolutions; a committed
  wall-clock budget.
- Non-goals: optimizing a map before a profile names its bound.
- Oracle: `cargo test --locked --bench lookup_table_sample` passes inside the
  test budget; the committed baseline records median and confidence interval per
  case with the machine class in the header.
- Gap: no `benches/` directory; the README's allocation-free claim has no timing.
- Class: `[perf]` `[patch]`. basis: 2661cc561084944b46a59ed0e5ecbb8d191a9f33.

<a id="IRIS-003"></a>
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
- milestone: M-1.0.
- Current state (2026-08-15, audited pre-change `origin/main` `bc47ce3`): the
  package identity is synchronized as registry package `iris-viz` with Rust
  import path `iris`, and hosted provider CI passes in run `31865323610`. A
  locked package dry run (`cargo package --locked`) and locked documentation
  build remain blocked in the Atlas checkout because Cargo attempts to rewrite
  `Cargo.lock` under the ambient path-patch graph. The hosted release run
  `31462641512` reached the OIDC token step but failed with HTTP 400 because no
  trusted publishing configuration exists for `ryancinsight/iris`; this is an
  external release blocker, not fixed by the provider documentation update.
- Next steps: pass the format, feature, lint, Nextest, doctest, documentation,
  package and hosted CI gates; merge the release automation and publish
  `iris-viz`; configure the exact trusted publisher, enforce
  trusted-publishing-only, and verify the crates.io index and the GitHub
  Release. The trusted-publisher configuration is an external repository
  setting that no repository change can supply.
