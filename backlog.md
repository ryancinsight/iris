# Backlog

<a id="iris-stable-qualification"></a>
## IRIS-STABLE-QUALIFICATION — iris stable-release qualification [patch] — todo
- priority: verification; milestone: M-contract-verification; basis: `a902e959ed89`.
- Outcome: Qualify the field-view and color-map contracts used by renderers. Follow the [release scope](docs/adr/0003-release-scope-1-0.md) and [milestone exits](backlog/milestones.md). Planning targets for `iris-viz` are 0.1.5, then 0.2.0, then 1.0.0. Verification alone can advance the current version.
- Representative manifest `iris-viz 0.1.4`. Target 1.0.0.
- [ ] [patch] Replay the zero-cardinality ScalarFieldView regression and its nonempty controls. Check the output length and the rejection of inconsistent shapes.
- [ ] [patch] Verify the named map reference values, then measure direct evaluation against LookupTable at the resolutions in IRIS-015. Attach the reference colors and benchmark intervals.
- [ ] [major] Reconcile the consumer table with actual renderer calls and verify a clean iris-viz consumer. Close the documentation and registry gates under ADR 0003.
- [ ] [patch] **0.1.5.** Permute zero and nonzero extents in a field view. Check empty cardinality where any extent is zero, and check that a nonempty shape rejects a mismatched buffer. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [minor] **0.2.0.** Pass the field through a real renderer. Compare palette endpoints and quantization transitions to the public color contract and verify the returned extent and buffer length.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. An external visualization caller reproduces the field-view and palette results with the supported feature set. Repeat the applicable exits on the exact RC before stable promotion. Attach exact-revision evidence for the local exits and this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

<a id="IRIS-017"></a>
## IRIS-017 — Correct the consumer table to match consumer manifests
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: `## Consumers` and `### Prospective consumers` in `docs/book/stack_position.md` name every registered member whose manifests declare `iris-viz`, and the prose is true of each row.
- Scope: `docs/book/stack_position.md:36-52` (also the third line of IRIS-STABLE-QUALIFICATION).
- Gap, read at default heads: metis `bbf6ffc97bad` declares `iris` in `metis-frontend` and `metis-ui-lang` `[dependencies]` and in the root package `[dev-dependencies]`; it has no row. kwavers `eb91da65678b` declares it in `[workspace.dependencies]` and as an optional dependency of `kwavers-analysis` (feature `plotting`); lines 49-51 say it declares none.
- Gap: "Each row names a package" is false for `CFDrs` `b76f00a72132` and `ritk` `99be8f8c2ba9` (virtual roots, only `[workspace.dependencies]`); `cfd-1d` and `cfd-2d` declare `iris` only under `[dev-dependencies]`.
- Oracle: at each registered member's default-branch commit, the members whose `Cargo.toml` dependency tables (never comments: athena, gaia and tyche match only comments) declare `iris-viz` equal the rows; the wording says "repository" and marks dev-only and optional declarations. Fails at the basis.
- Non-goals: the generator (IRIS-019); the `RenderBackend` sentence (IRIS-018).
- Class: `[docs]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-018"></a>
## IRIS-018 — Correct who supplies the `RenderBackend` implementation
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: `docs/book/stack_position.md` names which stack consumers implement `RenderBackend` and claims it of no other.
- Scope: the "Where Iris Sits" paragraph (`docs/book/stack_position.md:29-30`).
- Gap: it says consumers "supply the `RenderBackend` implementation". At default heads only metis `bbf6ffc97bad` implements it (`crates/metis-ui-lang/src/layout/display/render.rs:6`, `impl iris::render::RenderBackend<DisplayList> for Framebuffer`). `metis-frontend` and kwavers `eb91da65678b` (`kwavers-analysis/src/plotting/output.rs:2`) import it to call a backend (`iris_plot_html::HtmlPlotBackend`, a registry crate outside the member list).
- Gap: `git grep RenderBackend` matches nothing in `CFDrs` `b76f00a72132`, `ritk` `99be8f8c2ba9` or helios `ea487343c0ad`.
- Oracle: for every repository the sentence names as an implementor, `git grep -n "impl.*RenderBackend"` matches at its default-branch commit, and no repository is named otherwise. Fails at the basis for `CFDrs` and `ritk`.
- Non-goals: the consumer table (IRIS-017).
- Class: `[docs]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-019"></a>
## IRIS-019 — Check the consumer table against consumer manifests
- Status: todo; priority: verification; milestone: M-contract-verification; needs: IRIS-017.
- Outcome: the consumer table cannot drift from the stack's manifests; a committed check regenerates its manifest and coverage columns and diffs them.
- Scope: a stack-owned generator in atlas `scripts/` (reverse dependencies need every member's manifests), a marked generated region in `stack_position.md`, and its invocation in iris CI.
- Gap: atlas `8a771bd3a658` has no such script in `scripts/` and no board item names one; the hand-edited table has drifted (IRIS-017).
- Oracle: (a) at each row's recorded revision the generated manifest column equals the table, failing on a hand edit; (b) a scheduled meta scan at current default commits fails when a registered member declaring `iris-viz` has no row, shown by a synthetic omitted row.
- Non-goals: the "Iris items imported" prose column.
- Next: the lead files the atlas meta item that owns the generator and links here.
- Class: `[verification]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-011"></a>
## IRIS-011 — Replace `Normalized::from_u8` with a `From` impl
- Status: todo; priority: tightening; milestone: M-contract-verification; needs: none in iris (cross-repo: ritk).
- Outcome: the byte-to-unit-interval conversion uses the std conversion lattice and no public identifier carries a scalar-type suffix.
- Scope: `src/color/normalized.rs` (`from_u8` at :68, doctest :64-65), `docs/book/colormaps.md:35,52`, `tests/color_laws.rs:24,28,99`.
- Oracle: `impl From<u8> for Normalized` exists, `from_u8` is gone with no alias, every in-repo site is converted, and `byte_coordinates_cover_the_uniform_unit_grid_exactly` passes bit for bit (`n = value / 255` unchanged; no `TryFrom`).
- Co-evolution: ritk `99be8f8c2ba9` calls `Normalized::from_u8` at 8 sites (7 in `ritk-snap`, 1 in `ritk-vtk` at `domain/mapper.rs:186`) and cites it in `docs/adr/0011-iris-visualization-provider.md:33`; `CFDrs`, metis and kwavers call none. No ritk board file names it. The removal breaks 0.1.x, which 0.2.0 admits; a ritk migration item lands after this one.
- Class: `[arch]` `[major]` (public item removal). basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-015"></a>
## IRIS-015 — Measure the lookup-table fast path
- Status: todo; priority: tightening; milestone: M-contract-verification; needs: none.
- Outcome: the premise behind `LookupTable` (a cached nearest-entry lookup beats direct map evaluation) is measured.
- Scope: criterion bench `lookup_table_sample` comparing `LookupTable::<M, N>::sample` with `M::sample` for `Grayscale`, `BlueRed`, `Viridis`, `Turbo` and `NamedColorMap` at two or three resolutions; a committed wall-clock budget.
- Non-goals: optimizing a map before a profile names its bound.
- Oracle: `cargo test --locked --bench lookup_table_sample` passes inside the test budget; the committed baseline records median and confidence interval per case with the machine class in the header.
- Gap: iris has no `benches/` directory; `README.md:28` claims allocation-free lookup tables with no timing behind it.
- Class: `[perf]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-004"></a>
## IRIS-004 — Kwavers color-law consolidation
- Status: todo; priority: tightening; milestone: M-contract-verification; needs: none.
- Outcome: kwavers volume-render transfer functions come from Iris, so each shared color law has one owner.
- Scope: kwavers `crates/kwavers-analysis/src/visualization/renderer/volume.rs` and its `ColorScheme` match; `NamedColorMap` already provides Grayscale, Plasma, Viridis, Inferno, Magma and Turbo.
- Non-goals: Kwavers GPU mechanics, domain interpretation, plot-series extraction.
- Oracle: `kwavers-analysis` builds its transfer-function tables from Iris `LookupTable`, and `git grep -n "fn [a-z]*_colormap" -- crates/kwavers-analysis/src` matches nothing at kwavers' default-branch commit. Fails at the basis.
- Gap: the earlier `done` status did not hold. At kwavers `eb91da65678b`, `volume.rs:151-241` defines six local `*_colormap()` tables (match at :131-137), `visualization/` imports nothing from `iris`, and `git log -S inferno_colormap` shows only the crate-extraction commit. Iris is used only under `plotting/`.
- Next: a kwavers member item; no kwavers board file names this work.
- Class: `[arch]` `[minor]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

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
- Status: todo.
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


## Ready after IRIS-001

- Extract repeated CFDrs/Kwavers plot-series assembly onto Iris borrowed views after consumer contract audit.

