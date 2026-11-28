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

<a id="IRIS-018"></a>
## IRIS-018 — Correct who supplies the `RenderBackend` implementation
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: `docs/book/stack_position.md` names which stack consumers implement `RenderBackend` and claims it of no other.
- Scope: the "Where Iris Sits" paragraph (`docs/book/stack_position.md:29-30`).
- Gap: it says consumers "supply the `RenderBackend` implementation". At default heads only metis `bbf6ffc97bad` implements it (`crates/metis-ui-lang/src/layout/display/render.rs:6`, `impl iris::render::RenderBackend<DisplayList> for Framebuffer`). `metis-frontend` and kwavers `eb91da65678b` (`kwavers-analysis/src/plotting/output.rs:2`) import it to call a backend (`iris_plot_html::HtmlPlotBackend`, a registry crate outside the member list).
- Gap: `git grep RenderBackend` matches nothing in `CFDrs` `b76f00a72132`, `ritk` `99be8f8c2ba9` or helios `ea487343c0ad`.
- Oracle: the sentence names metis as the implementor, and `git grep -n "impl.*RenderBackend"` matches at the default-branch commit of every repository it names; a sentence that names none, or names `CFDrs` or `ritk`, fails. Fails at the basis: it names none.
- Non-goals: the consumer table.
- Class: `[docs]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-019"></a>
## IRIS-019 — Check the consumer table against consumer manifests
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: the consumer table cannot drift from the stack's manifests; a committed check regenerates its manifest and coverage columns and diffs them.
- Scope: a stack-owned generator in atlas `scripts/` (reverse dependencies need every member's manifests), a marked generated region in `stack_position.md`, and its invocation in iris CI.
- Gap: atlas `a850dbc26a31` has no such script in `scripts/` and no board item names one; the hand-edited table drifted from the manifests before its last correction. Neither (a) nor (b) reads a row's revision against the member's tip: CFDrs `e522d8dd83f0` (an ancestor of the default tip `3dbc3bfc6898`) declares `iris` in `[workspace.dependencies]`, `[dependencies]` and a `[patch]` table of the root manifest, the tip in `[workspace.dependencies]` only (`tomllib` scan of both revisions).
- Oracle: (a) at each row's recorded revision the generated manifest column equals the table, failing on a hand edit; (b) a scheduled meta scan at current default commits fails when a registered member declaring `iris-viz` has no row, shown by a synthetic omitted row; (c) each row's revision is its member's default tip or an ancestor of it (`git merge-base --is-ancestor`) and the manifest column generated at the tip equals the one at that revision, run in the scheduled meta scan of (b) so a member's later manifest change fails a stale row without waiting for an iris PR, shown by a synthetic CFDrs row regenerated at `e522d8dd83f0`, which passes (a) and fails (c) (derived: no generator exists to run). Wrong fix tried: checking only that the revision resolves (`git cat-file -e e522d8dd83f0`) accepts that row. Fails at the basis: no generator exists.
- Non-goals: the "Iris items imported" prose column.
- Next: the lead files the atlas meta item that owns the generator and links here.
- Class: `[verification]` `[patch]`. basis: a99f4037015e3f403e36e9e4643fa47f1dd11db0.

<a id="IRIS-021"></a>
## IRIS-021 — Derive the registry example's version from the manifest
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: the inline-table `iris = { package = "iris-viz", version = ... }` example at both sites, `docs/book/stack_position.md:61` and the `Cargo.toml:6` comment, states the `[package]` version of `Cargo.toml`, and a check that the `verify` job runs fails when either site differs.
- Scope: `docs/book/stack_position.md:59-62`, `Cargo.toml:6`, a new `tools/check_registry_version.py` (parses both sites with `tomllib`) with its test in `tools/tests`, and one step in the `verify` job of `.github/workflows/ci.yml` beside the palette-fixture step.
- Non-goals: the consumer table (IRIS-019); any version bump.
- Gap (read and run at `a99f4037015e`): the example says `0.1.1`, `Cargo.toml:9` says `0.1.4`. crates.io lists 0.1.0 to 0.1.4, none yanked, newest 0.1.4 (API 2026-11-28T00:33Z; sparse index 00:56Z). `0.1.1` is a valid caret requirement that resolves to 0.1.4, so the example is not unresolvable; the chapter's "Consumers write:" is false of the version, since the `iris-viz` entry in each of the four consumers' root `[workspace.dependencies]` (CFDrs `b76f00a72132`, kwavers `34fa3f47ae07`, metis `0f1cfc9bc916`, ritk `fe35fe8630ab`) reads `0.1.4`.
- Oracle, each fails at the basis: (a) `python -I -c "import re,sys,tomllib as t;v=t.load(open('Cargo.toml','rb'))['package']['version'];r={f:[t.loads(m)['iris']['version'] for m in re.findall(r'iris = \{[^}]*\}',open(f,encoding='utf-8').read())] for f in ('docs/book/stack_position.md','Cargo.toml')};print(r,v);sys.exit(any(x!=[v] for x in r.values()))"` prints `{'docs/book/stack_position.md': ['0.1.1'], 'Cargo.toml': ['0.1.1']} 0.1.4` and exits 1; it exits 0 after a correct fix, whatever the key order; (b) `python -I -c "import sys,yaml;j=yaml.safe_load(open('.github/workflows/ci.yml'))['jobs']['verify'];s=[x for x in j['steps'] if x.get('run')=='python3 tools/check_registry_version.py'];sys.exit(not(len(s)==1 and set(s[0])<={'name','run'} and not j.get('continue-on-error') and 'if' not in j))"` exits 0 (a step of `verify` runs exactly `python3 tools/check_registry_version.py`, with no `||`, `if` or `continue-on-error`); (c) `python3 tools/check_registry_version.py` exits 0 on the fixed tree and nonzero on each of three copies of it: `Cargo.toml` `version` set to `0.1.5`; only the chapter example's version changed; only the `Cargo.toml:6` example's version changed.
- Wrong fixes tried in scratch, each rejected by one clause: both literals set to `0.1.4` alone ((b), (c)); a correct script with a commented-out step, a `|| true` step, `continue-on-error: true`, `if: false` or a step in another job ((b)); a script that always exits 0, always exits 1, compares to a hard-coded `0.1.4`, or ignores the `Cargo.toml:6` comment ((c)).
- Next: write the check, its test and the CI step, then correct both literals.
- Class: `[docs]` `[verification]` `[patch]`. basis: a99f4037015e3f403e36e9e4643fa47f1dd11db0.

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
- Gap: the earlier `done` status did not hold. At kwavers `eb91da65678b`, `volume.rs:151-241` defines six local `*_colormap()` tables (match at :131-137), `visualization/` imports nothing from `iris`, and `git log -S inferno_colormap -- crates/kwavers-analysis/src` lists only the crate-extraction commit `a101ef2fdd`. Iris is used only under `plotting/`.
- Next: a kwavers member item; no kwavers board file names this work.
- Class: `[arch]` `[minor]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-002"></a>
## IRIS-002 — Remove CFDrs's local heatmap color law
- Status: todo; priority: tightening; milestone: M-contract-verification; needs: none.
- Outcome: CFDrs heatmap fills take their color from an Iris `ColorMap`, so no consumer carries a local color formula.
- Scope: CFDrs `crates/cfd-schematics/src/heatmap/{palette.rs,svg.rs,tests.rs}`; an additive Iris map if no `NamedColorMap` reproduces the law.
- Gap: at CFDrs `b76f00a72132`, `palette.rs:1-7` defines `cancer_cav_color` (linear #FFD700 to #CC0000), called at `svg.rs:158`; `analysis_field/color.rs:3` already uses `NamedColorMap`. Searching non-example, non-test sources for `* 255` and `as u8` finds only `palette.rs:4-5`.
- Oracle: `git grep -n cancer_cav_color -- crates` matches nothing at CFDrs' default-branch commit. Fails at the basis (7 matches).
- Class: `[arch]` `[minor]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-020"></a>
## IRIS-020 — Re-point ladder and ADR 0003 ids that resolve to nothing
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: every `IRIS-nnn` in `backlog/milestones.md` and ADR 0003 names a board heading or a commit with its `Item:` trailer.
- Scope: `backlog/milestones.md:5`, `docs/adr/0003-release-scope-1-0.md:11-12`; moving an exit check needs an ADR 0003 revision note and a judge verdict.
- Gap: IRIS-006 is delivered and has no board heading; `git log --all --grep='^Item: IRIS-006'` lists none (IRIS-010, 014 and 016 have 2, 1 and 5).
- Oracle: for each id in those files, a heading `<a id="IRIS-nnn">` exists at the default-branch commit or `git log --all --grep="^Item: IRIS-nnn"` is nonempty. Fails for IRIS-006.
- Class: `[docs]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-003"></a>
## IRIS-003 — Crates.io release automation
- Status: blocked; priority: verification; milestone: M-1.0; needs: none.
- Outcome: publish `iris-viz` from GitHub Releases without a long-lived registry
  token in repository secrets.
- Non-goals: version changes, additional packages, or consumer dependency
  updates.
- Blocker: trusted-publishing-only is a registry setting only the owner can enable. Re-open trigger: the owner reports it enabled, or the API in the oracle shows it.
- Gap (crates.io API, read this session): 0.1.1 to 0.1.4 carry `trustpub_data` (github, `ryancinsight/iris`); 0.1.0 carries none; `trustpub_only` is `false`. Unverified: whether a publisher is registered now.
- Oracle: `GET https://crates.io/api/v1/crates/iris-viz` returns `trustpub_only: true`. Fails at the basis.
- Risk/class: `[patch]` release infrastructure.
- basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.

<a id="IRIS-007"></a>
## IRIS-007 — Prove `NamedColorMap::ALL` covers every variant
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: adding a `NamedColorMap` variant without adding it to `ALL` fails to
  compile or fails a test, so the exhaustive-grid law test cannot silently skip
  a map.
- Scope: `src/color/map/named.rs` (the `named_color_maps!` macro and `all_contains_each_variant_once`).
- Non-goals: changing the stable display order of `ALL` or the
  `#[non_exhaustive]` attribute.
- Gap (read, not run): `ALL` is `[Self; 14]` built from `display_order` (:39-41); the test sizes `seen` by `ALL.len()` and iterates only `ALL` (:116-130), so a variant missing from `ALL` is never seen.
- Oracle: append a variant after `BlueRed` (discriminant 14) and leave `display_order` unchanged; `cargo nextest run --locked --all-features` fails. It passes today.
- Class: `[verification]` `[patch]`. basis: 6ac18225deb0af1631fa1a5f76cc360b5a6c6b6b.
