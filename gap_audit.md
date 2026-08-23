# Iris gap audit

## Finding 2026-08-20: iris scope-vs-delivery audit

Static audit of `9672fc0876306fbb76d58fb3af37e8cbd5998b42` on `main`. No build,
test, lint, or benchmark ran — the shared Atlas target directory and its global
lock were unavailable to this pass, so every statement below is grounded in a
file, a line, or a filesystem query. No claim here asserts that any gate passes.

### Measured size and floor

One package (`iris-viz`, library path `iris`), 1,174 source lines across 18
files, 18 `#[test]` functions (all in `tests/`, one of them `serde`-gated), 6
doctests, 4 book chapters, 2 Accepted ADRs, 1 example, no `benches/`.

Conformance floor, measured over `src/`, `tests/`, and `examples/`: zero
`todo!`/`unimplemented!`, zero TODO/FIXME/HACK, zero `#[allow(`, five
`#[expect(` sites each carrying a `reason`, zero `dyn ` sites, zero production
`unwrap()`, zero files over 500 lines (largest is
`src/color/map/table.rs` at 298), zero junk-drawer modules, zero
`pub use … as …` shims, one type-suffixed public identifier
(`Normalized::from_u8`), and `missing_docs` denied both in `Cargo.toml:38` and
`src/lib.rs:11` with `unsafe_code` forbidden. CI pins every action to a full
commit SHA, declares least-privilege `permissions`, and sets `timeout-minutes`
on both jobs. This is the strongest floor of any audited repository in the
stack; the gaps below are verification and documentation gaps, not hygiene
gaps.

### Color science: the crate does not implement colorspace conversion, by declaration

There is no sRGB transfer function, no linear-light RGB, no XYZ, no Lab/Luv,
and no HSL in the crate. This is a declared boundary, not an omission: the
README states that `Rgba` stores sRGB-encoded channels, that interpolation
happens in that encoded space, and that `to_rgba8` applies `round(255v)` with
no transfer; `src/color/model.rs:9-13`, `src/color/map/mod.rs:3-6`, and ADR
0001's decision section carry the same statement. The one color-model
conversion present is an inline HSV hue sweep in `src/color/map/sequential.rs:58-81`
serving the `Rainbow` law only. The convention is pinned by an exhaustive
256-value round-trip test (`tests/color_laws.rs:76-90`). Verdict: the encoding
contract is stated consistently in six places and verified; no colorspace
capability is claimed and none is delivered.

### The load-bearing verification gap: palette approximations have a self-referential oracle

Seven built-in maps document themselves as approximations of published
palettes — `Bone` and `Jet` as "approximating the standard … palette",
`Plasma`, `Viridis`, `Inferno`, `Magma`, and `Turbo` as five- or
nine-control-point approximations (`src/color/map/table.rs:6,31,70,108,146,196,246`).
No test in the repository compares any of them to a published table, and no
approximation error bound is stated anywhere. What
`tests/color_laws.rs:114-178` asserts instead is that each map returns its own
declared control point: `Viridis(0.5) == [0.204, 0.636, 0.469]` at
`tests/color_laws.rs:143-149` restates the constants at
`src/color/map/table.rs:118,126,133`. That assertion cannot fail while the
constants and the interpolator agree, so it establishes interpolator
correctness at a node and nothing about palette agreement. Filed as IRIS-006.

The perceptual claim is narrower than the risk. `NamedColorMap::Viridis` is
documented as "Perceptually ordered" (`src/color/map/named.rs:31-33`) and the
book repeats it; the README is careful to scope the matplotlib citation to
"names and classifications", and ADR 0001 and `src/color/map/diverging.rs:23-27`
both explicitly disclaim Moreland's Msh interpolation for `CoolWarm`. So no
perceptual-uniformity or colorblind-safety claim is made that evidence would
have to back. The residual exposure is that a five-control-point
piecewise-linear resampling in sRGB space does not preserve the property that
makes the source palette worth naming, and nothing in the repository bounds how
far it departs. IRIS-006 closes that by measurement, not by argument.

### The render seam is declared but never exercised against its own view types

`RenderBackend` (`src/render/backend.rs:7-21`) is dyn-free by construction: the
GAT `Frame<'a>` makes the trait dyn-incompatible, so the README's
"core uses no dynamic dispatch" holds structurally and is corroborated by the
zero `dyn ` sites measured above. But `V` is left entirely unbounded, nothing
ties the seam to `ScalarField`, and the sole implementor in the repository is
`impl RenderBackend<[u8]> for ByteFrameBackend` at
`tests/view_contracts.rs:84-95` — a four-byte `copy_from_slice` that touches no
view type and no color map. The declared composition "view → color law → lent
frame storage" is therefore unverified end to end. Filed as IRIS-008.

### LUT output has value-semantic verification, but only at sample nodes

`tests/color_laws.rs:181-187` differentially checks
`LookupTable::<Grayscale, 256>::sample` against `Grayscale::sample` at all 256
nodes, and `:190-196` establishes that the strategy is zero-sized and adds no
table storage. Both are genuine value-semantic assertions. What is absent is
off-node coverage: `LookupTable::sample`
(`src/color/lookup_table.rs:59-64`) is a nearest-bin quantizer, so its
between-node error against the direct map is bounded by table resolution and
map slope — a derivable bound with no test behind it, and a second reason
IRIS-006's bound work is the right next increment. There is no golden-image
verification in the repository; for a crate that emits color rather than
images, the per-channel reference comparison of IRIS-006 is the stronger oracle
and is filed in preference to a golden image.

### Claim cross-check against README and the Accepted ADRs

Verified true: the README doctest value matches `table.rs`; the 1,025-point
exhaustive grid claim matches `tests/color_laws.rs:39-50` (`0..=1024`); the ZST
and table-layout claim matches `:190-196`; the borrow-identity claim matches
`tests/view_contracts.rs:15-16,41,65` (`core::ptr::eq`); ADR 0002's four proof
obligations each map to an assertion at `tests/color_laws.rs:92-111` and
`:114-178`; ADR 0002's "appended so every existing implicit discriminant
remains stable" matches `BlueRed` being last in the declaration at
`src/color/map/named.rs:50`. The `Cow`-based borrow-or-own axis claim matches
`src/view/axis.rs:7-10`.

Verified false, and corrected in this pass: `docs/book/colormaps.md` stated
that the table-defined maps "are implemented as `LookupTable` instances" whose
`sample` "linearly interpolates between the two nearest entries" — both halves
wrong, since those maps are piecewise control-point tables evaluated by
`src/color/map/interpolation.rs:18-42` and `LookupTable::sample` selects the
nearest entry without interpolating; and that "`RenderBackend` is generic over
any `ColorMap`", which `src/render/backend.rs:7` contradicts.

Unverifiable from inside this repository, filed rather than edited:
`docs/book/stack_position.md` lists `helios` as a consumer, for which this
repository holds no merge evidence, while `ritk`, `CFDrs`, and `kwavers` each
have it in `backlog.md` (IRIS-006 … IRIS-015 detail this as IRIS-014).

### Process and provenance defects

`docs/adr/README.md:3-5` instructs regeneration via `python
scripts/adr-index.py`, but no `scripts/` directory exists at this revision — a
generator contract with no generator, so the "do not hand-edit" header is
unenforceable (IRIS-009). Two ADR indexes coexist, `docs/adr/README.md` and
`docs/adr/INDEX.md`, the second explicitly excluded from IRIS-005's scope
(`backlog.md:62-63`) and therefore left as a parallel source of truth
(IRIS-012). `.github/workflows/ci.yml:25,45` request toolchain 1.95.0 while
`rust-toolchain.toml:2` pins 1.97.0; a committed toolchain file is a rustup
directory override, so the `Cargo.toml:12` MSRV floor is probably not
exercised by any job (IRIS-013). `src/error.rs:75-76` gates the error trait on
the `std` feature although the crate advertises `no_std` support at
`src/lib.rs:7-8` and `core::error::Error` is long stable below the declared
floor (IRIS-010). `src/color/normalized.rs:56` is the crate's only
type-suffixed public identifier and the place where the std conversion lattice
should apply; `grep 'impl.*From<' src` returns nothing (IRIS-011). There are no
benchmarks behind the `LookupTable` precomputation premise (IRIS-015).

### Completeness

Roughly 82 % of the declared scope is delivered and verified. Denominator: the
five capabilities in the README "Boundary" section, the decisions of ADR 0001
and ADR 0002, and the four chapters in `docs/book/SUMMARY.md`, weighted 40 %
capability implementation, 25 % verification depth, 20 % documentation, 15 %
conformance floor. Capability implementation is near complete — every declared
capability exists as real, input-sensitive code with no stub. Verification
carries the loss: palette agreement has a self-referential oracle, the render
seam has no real exercise, `ALL` has no exhaustiveness proof, and the MSRV
floor is probably unexercised. Documentation loses the two corrected false
claims and the phantom generator. The floor loses almost nothing.

### Working-tree observation, not acted on

At audit time the tree carried a staged-only `Cargo.lock` change adding 228
lines of `[[patch.unused]]` entries for stack members
(`athena-core`, `hephaestus-cuda`, and others) that Iris does not depend on —
Atlas development-overlay residue, and a reversal of `c10b328`
("build(iris): Restore the standalone Cargo.lock form"). This audit made no
change to it and left it staged exactly as found. Whoever owns that change
should strip the overlay entries before committing.

## Provider boundary

Iris owns domain-neutral visualization contracts: normalized color values,
named color maps, fixed lookup tables, borrowed series and scalar-field views,
and the render-backend seam. It does not own domain interpretation, file
formats, GPU mechanics, or consumer-specific scalar-range semantics.

## IRIS-003 — Crates.io release automation

The registry package identity is `iris-viz`; the Rust library and import path
remain `iris`. The release artifact prescription is
`crate-iris-viz-v<version>`. `README.md`, `CHANGELOG.md`, `backlog.md`, and
`checklist.md` now use that split consistently.

The audited pre-change provider default was
`bc47ce37376127b65c064b93dac256555f44ff65`. Hosted provider CI passes at that
exact revision in run `31865323610`.

### External release blocker

The crates.io trusted-publishing configuration for `ryancinsight/iris` is not
present. Hosted release run `31462641512`, targeting the earlier release
revision `ab3eea284e41a9649941253a7b7a69c839907fb3`, passed package validation
but failed while requesting the short-lived OIDC token with HTTP 400:
`No Trusted Publishing config found for repository ryancinsight/iris`.

This is external registry configuration and remains open. The documentation
and PM synchronization does not configure the publisher, enforce
trusted-publishing-only mode, publish `iris-viz`, or verify the crates.io
index. Re-open this item after the exact trusted publisher is configured and a
new `crate-iris-viz-v<version>` release completes validation and publication.

## Local Atlas-overlay limitation

In the dirty Atlas checkout, `cargo fmt --all -- --check` and
`cargo metadata --locked --no-deps --format-version 1` pass. The locked
documentation build is blocked before compilation because the ambient Atlas
path-patch graph causes Cargo to attempt a `Cargo.lock` rewrite under
`--locked`. This is environment/stack resolution evidence, not a provider
source failure.
