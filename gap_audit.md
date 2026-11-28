# Iris gap audit

## Incremental release baseline

Reviewed [iris-viz 0.1.4](Cargo.toml) at [a902e959ed89](https://github.com/ryancinsight/iris/tree/a902e959ed893baf899a96c5e973f1bfecc40df9). The exact version appears in the [registry](https://crates.io/crates/iris-viz/0.1.4).

The next [patch] checkpoint uses [zero cardinality permutations](tests/zero_cardinality_permutations.rs). [Target version and acceptance criteria](backlog.md#iris-stable-qualification).

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

### LUT output has value-semantic verification, but only at sample nodes

`tests/color_laws.rs:181-187` differentially checks
`LookupTable::<Grayscale, 256>::sample` against `Grayscale::sample` at all 256
nodes, and `:190-196` establishes that the strategy is zero-sized and adds no
table storage. Both are genuine value-semantic assertions. What is absent is
off-node coverage: `LookupTable::sample`
(`src/color/lookup_table.rs:59-64`) is a nearest-bin quantizer, so its
between-node error against the direct map is bounded by table resolution and
map slope — a derivable bound with no test behind it. There is no golden-image
verification in the repository; for a crate that emits color rather than
images, the per-channel reference comparison in `tests/palette_reference.rs` is
the stronger oracle.

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
`crate-iris-viz-v<version>`. `README.md`, `CHANGELOG.md`, and `backlog.md` now
use that split consistently.

The audited pre-change provider default was
`bc47ce37376127b65c064b93dac256555f44ff65`. Hosted provider CI passes at that
exact revision in run `31865323610`.

## Local Atlas-overlay limitation

In the dirty Atlas checkout, `cargo fmt --all -- --check` and
`cargo metadata --locked --no-deps --format-version 1` pass. The locked
documentation build is blocked before compilation because the ambient Atlas
path-patch graph causes Cargo to attempt a `Cargo.lock` rewrite under
`--locked`. This is environment/stack resolution evidence, not a provider
source failure.

<a id="empty-claim-commits"></a>
## Empty claim commits reach the default branch
- Risk: a claim commit with no tree change merges unfolded, so default-branch history holds commits that change nothing; folding the claim into the first real commit has no check.
- Evidence (read at `49bf9ab6e0c5`): three non-merge commits on `origin/main` have an empty diff, two of them `Item:` claims (`193b9d0200dc`, `becaa8ba8bb2`) and `6a5e4d45b72e`; found with `git rev-list --no-merges` plus an empty `git diff-tree -r`. The atlas `scripts/git-hooks/pre-commit` and `pre-push`, read at filing, have no empty-commit test (an `empty`/`diff-tree` search finds unrelated comments only).
- Re-open trigger: an atlas pre-push or conformance check rejects a pushed non-merge commit with an empty diff; then delete this entry. The check belongs to atlas, so this is an external requirement, not an iris item.
