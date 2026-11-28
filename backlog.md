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

<a id="IRIS-021"></a>
## IRIS-021 — Derive the registry example's version from the manifest
- Status: todo; priority: verification; milestone: M-contract-verification; needs: none.
- Outcome: the inline-table example, written `<key> = { ... }` on one line, at both sites, `docs/book/stack_position.md:61` and the `Cargo.toml:6` comment, is the dependency entry under the key `[lib].name` whose table is `{ package = <[package].name>, version = <[package].version> }` of `Cargo.toml`, with no other key (a `git` or `features` key changes what a consumer gets), and a check that the `verify` job runs fails when either site differs.
- Scope: `docs/book/stack_position.md:59-62`, `Cargo.toml:6`, a new `tools/check_registry_version.py` (parses both sites with `tomllib` and compares each whole table with the one derived from `[lib]` and `[package]`) with its test in `tools/tests`, and one step in the `verify` job of `.github/workflows/ci.yml` beside the palette-fixture step.
- Non-goals: the consumer table (IRIS-019); any version bump; varying `[lib].name` in (c): a dependent may choose any key beside `package` (Cargo reference, "Renaming dependencies in Cargo.toml", https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml, which does not mention `[lib].name`), so the key follows `[lib].name` by convention, which keeps consumers' `use iris::...` unchanged.
- Gap (read and run at `0849566d2823`): the example says `0.1.1`, `Cargo.toml:9` says `0.1.4`. crates.io lists 0.1.0 to 0.1.4, none yanked, newest 0.1.4 (API 2026-11-28T00:33Z; sparse index 00:56Z). `0.1.1` is a valid caret requirement that resolves to 0.1.4, so the example is not unresolvable; the chapter's "Consumers write:" is false of the version, since the `iris-viz` entry in each of the four consumers' root `[workspace.dependencies]` (CFDrs `b76f00a72132`, kwavers `34fa3f47ae07`, metis `0f1cfc9bc916`, ritk `fe35fe8630ab`) reads `0.1.4`.
- Oracle, each fails at the basis. (a) `python -I -c "import re,sys,tomllib as t;m=t.load(open('Cargo.toml','rb'));p=m['package'];k=m['lib']['name'];w={k:{'package':p['name'],'version':p['version']}};r={f:[t.loads(x) for x in re.findall(k+r' = \{[^}]*\}',open(f,encoding='utf-8').read())] for f in ('docs/book/stack_position.md','Cargo.toml')};print(r,w);sys.exit(any(x!=[w] for x in r.values()))"` prints each site's parsed tables and the expected one and exits 1 (both sites read `0.1.1`); it exits 0 after a correct fix, whatever the key order.
- (b) `python -I -c "import sys,yaml;w=yaml.safe_load(open('.github/workflows/ci.yml'));o=w.get('on',w.get(True));j=w['jobs']['verify'];s=[x for x in j['steps'] if str(x.get('run')).strip()=='python3 -I tools/check_registry_version.py'];q=o.get('pull_request') if isinstance(o,dict) else None;t={'opened','synchronize','reopened'};ok=len(s)==1 and set(s[0])<={'id','name','run','shell','working-directory'} and s[0].get('shell') in (None,'bash') and s[0].get('working-directory') in (None,'.') and set(j)<={'name','runs-on','timeout-minutes','steps','permissions'} and not {'env','defaults'}&set(w) and ((isinstance(o,dict) and 'pull_request' in o and (not q or (set(q)=={'types'} and t<=set(q['types'])))) or (isinstance(o,list) and 'pull_request' in o) or o=='pull_request');sys.exit(not ok)"` exits 0 only if `verify` has exactly one step whose stripped `run` is `python3 -I tools/check_registry_version.py` with keys among `id`, `name`, `run`, `shell: bash`, `working-directory: .`; the job has keys among `name`, `runs-on`, `timeout-minutes`, `steps`, `permissions` (no `needs`, `if`, `continue-on-error`, `env`, `defaults`, `strategy`, `uses`); the workflow has no `env` or `defaults`; `on` names `pull_request`, with no filter other than a `types` list that includes `opened`, `synchronize` and `reopened`. It needs PyYAML (`pip install pyyaml`; no file of this repository declares it).
- (c) With V the manifest version read at test time, and `GITHUB_ACTIONS=true` and `CI=true` set, `python3 -I tools/check_registry_version.py` exits 0 on the fixed tree; on trees with the manifest and both examples set together to V+patch, to V+minor and to V with `-rc.1` appended; and on a tree with `[package].name` and both examples' `package` set together to another name. It exits nonzero when only the manifest is set to V+patch, V+minor, V+major, V with `0` appended or V with `-rc.1` appended, and when only `[package].name` changes. It exits nonzero when one site, or both sites together in the same way, change to an example that loses `package`, has another `package`, or gains any key besides `package` and `version` (the table's keys are exactly those two): each of `git`, `features`, `default-features`, `path`, `optional`, `registry`, `branch`, `rev`, and one more key the tester invents at test time that no list in this item names. It exits nonzero when only one site's example has another version, is written under another key, has a TOML syntax error (unquoted version) or is removed, and when the chapter file is missing. It exits nonzero when one site holds a second example with another version, before or after the real one, on the same line, on the adjacent line, as a chapter prose line after the code fence, or in a second chapter code fence before or after the real one (so a check that reads only the first match, the first code block or fixed lines fails).
- Limits of (b): it reads the target step, the job and workflow keys above, and not the other steps of `verify` (an earlier step can alter the script), other jobs or workflows, `on.push`, the runner image, or the repository rules that make `verify` required.
- Wrong fixes tried in scratch, each rejected by at least one clause: both literals set to `0.1.4` alone ((b), (c)); a commented-out step, a `|| true` step, `continue-on-error`, `if: false`, a step in another job, a `needs:` on a skipped job, a removed or path-filtered `pull_request` trigger, job `env` or `defaults` ((b)); a script that always exits 0 or 1, exits 0 under `GITHUB_ACTIONS` or `CI`, compares to a hard-coded `0.1.4`, special-cases `0.1.5`, hard-codes the package name, compares the two sites with each other and only the version with the manifest, checks `version` only, `package` only, major.minor only, a substring or the numeric part only, ignores the `Cargo.toml:6` comment, tolerates a missing example, skips a missing chapter file, exits 0 on any exception, reads the manifest version by an unanchored regex (first match), reads only the first example, the first code block or fixed lines, denies only a fixed list of extra keys, however long and the ones named in (c) included, or finds the table by `package` and ignores its key ((c)); an example without `package` ((a), (c)).
- Next: write the check, its test and the CI step, then correct both literals. An earlier oracle draft is at `refs/pull/106/head`.
- Class: `[docs]` `[verification]` `[patch]`. basis: 49bf9ab6e0c5413661649caf48bc4080b9215da6.

<a id="IRIS-022"></a>
## IRIS-022 — Close the M-correctness milestone
- Status: todo; priority: verification; milestone: M-correctness; needs: none.
- Outcome: once its five exit commands pass in an export of a default-branch commit C, run outside the Atlas overlay, M-correctness is gone from the ladder (and from M-contract-verification's `needs:`) and a `blocked` release item names 0.1.5 and `Closing commit C`, as the roadmap rule asks.
- Scope: `backlog/milestones.md:2-4`, `backlog.md` (the release item).
- Gap (read at `49bf9ab6e0c5`): M-correctness (0.1.5) is the earliest open block; its one named item IRIS-016 is retired (`48ef9ae`); no item carried `milestone: M-correctness`, ten open items carried M-contract-verification and IRIS-003 carried M-1.0. Two source commits since `crate-iris-viz-v0.1.4` (`087ee38`, `5c13d8d`) are unreleased. `CHANGELOG.md` heading "Unreleased" sits below 0.1.3 and holds the 0.1.2 entry (`78c4781` is in `crate-iris-viz-v0.1.2`): drift, carried by the release item. The `ci` run on that commit succeeded, but `verify` runs the block's commands without `--locked` (`ci.yml:68-74`), which does not show they pass with it; `gap_audit.md:111-118` records that a `--locked` build inside the Atlas overlay is blocked before compilation, so oracle (2) runs outside it. Whether the five pass is a hypothesis.
- Oracle, at the repository root with `origin/main` fetched; both exit 0, and (1) exits 1 today. (1) `python -I -c "import re,subprocess,sys;m=open('backlog/milestones.md',encoding='utf-8').read();b=open('backlog.md',encoding='utf-8').read();r=[x for x in re.split(r'<a id=',b) if 'Status: blocked' in x and '0.1.5' in x and 'release authority' in x];c=[s for x in r for s in re.findall(r'Closing commit ([0-9a-f]{12,40})\b',x)];ok=lambda s:subprocess.run(['git','merge-base','--is-ancestor',s,'origin/main'],capture_output=True).returncode==0;sys.exit(not('M-correct' not in m and set(re.findall(r'<a id=\"(M-[^\"]+)\"',m))=={'M-contract-verification','M-1.0'} and any(map(ok,c))))"` exits 0 only if the ladder holds exactly the other two blocks, no `M-correct` text remains, and a `blocked` item names 0.1.5, `release authority` and `Closing commit <sha>` (12 to 40 hex) with `<sha>` an ancestor of `origin/main`.
- (2) with `ATLAS_STACK_ROOT` set, `python -I -c "import io,os,shutil,subprocess,sys,tarfile,tempfile;from pathlib import Path;r=Path(os.environ['ATLAS_STACK_ROOT']);sys.path.insert(0,str(r/'scripts'));import lockfile as L;d=tempfile.mkdtemp();tarfile.open(fileobj=io.BytesIO(subprocess.run(['git','archive',sys.argv[1]],capture_output=True,check=True).stdout)).extractall(d,filter='data');e=dict(os.environ,CARGO_TARGET_DIR=str(L.shared_target_dir(r/'repos'/'iris'/'Cargo.toml')));A=[['fmt','--all','--','--check'],['clippy','--locked','--all-targets','--all-features','--','-D','warnings'],['nextest','run','--locked','--all-features'],['test','--locked','--doc','--all-features'],['doc','--locked','--no-deps','--all-features']];c=[L.run_outside_the_overlay(a,Path(d)/'Cargo.toml',env=e).returncode for a in A];shutil.rmtree(d,ignore_errors=True);print(c);sys.exit(any(c))" <sha>` exports `<sha>` and runs the block's five commands through `run_outside_the_overlay` of atlas `scripts/lockfile.py` (a function, not a command-line mode, so this import is the only entry; it runs cargo from a neutral directory, which drops the overlay, and the shared target directory is restored by the `CARGO_TARGET_DIR` set here); it exits 0 only if all five exit 0 (a 600 s per-command deadline expiry raises and exits 1).
- Wrong fixes, each rc 1 from (1): deleting the block but leaving `needs: M-correctness`; deleting it with no release item; the release item with the block kept; the release item `todo`, or without 0.1.5 or `release authority`; naming no `Closing commit`, or one that is not on `origin/main` (the lane's branch head), or one that does not resolve; the anchor renamed; deleting another block. From (2): a close made without running the commands, or run in the stack overlay.
- Next: in a lane run (2) on the default tip. All pass: delete the block, drop `needs: M-correctness`, file the release item with that tip as `Closing commit` (blocker: release authority; re-open: the user's explicit release authority; also the `CHANGELOG.md` re-heading). One fails: file that failure as an item on M-correctness and re-scope this one.
- Class: `[docs]` `[verification]` `[patch]`. basis: 49bf9ab6e0c5413661649caf48bc4080b9215da6.

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
