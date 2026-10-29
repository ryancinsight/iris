# Palette Reference Agreement

Iris's seven table-defined named palettes are approximations. Their names do
not promise exact Matplotlib output or perceptual uniformity. The following
limits characterize the current implementation on a specified finite grid;
they are not a newly selected application fidelity requirement.

## Reference and sampling

The oracle is Matplotlib **3.11.2**, signed release commit
[`d3ca9172e392048ec242b90e8423e88d1a3000b2`](https://github.com/matplotlib/matplotlib/tree/d3ca9172e392048ec242b90e8423e88d1a3000b2).
`tests/fixtures/palette_reference/` retains its default 256-entry RGB tables,
source and fixture SHA256 hashes, release identity and upstream license notice.
Bone and Jet originate in `_cm.py`; the five listed palettes in `_cm_listed.py`.
The released wheel's three source files and license were checked against that
commit before generation. The generator verifies the actual Matplotlib lookup
on every comparison input, rather than assuming interpolation semantics.

For every `k=0..=1024`, both implementations receive the exact binary coordinate
`t=k/1024`. Matplotlib's default LUT selects `min(floor(256*t),255)`, which is
`min(k/4,255)` with integer division on this grid. Iris executes its existing
piecewise-linear law in f32. Its observed f32 channels are represented exactly
in f64 solely for comparison with the independent f64 reference. Alpha is 1.

For each map and RGB channel, measure
`M = max_k(abs(Iris(k/1024)-reference[min(k/4,255)]))` and document the outward
ceiling `B = ceil(M*1_000_000)/1_000_000`. The tests assert every deviation is at
most B and the measured maximum is greater than B minus 0.000001. This pins the
derivation as well as the upper limit; changing colors requires a new justified
measurement, not an unexplained tolerance increase.

## Measured per-channel bounds

| Palette | Red | Green | Blue |
| --- | ---: | ---: | ---: |
| Bone | 0.005341 | 0.003498 | 0.003567 |
| Jet | 0.017756 | 0.013163 | 0.013317 |
| Plasma | 0.244877 | 0.150198 | 0.172135 |
| Viridis | 0.283955 | 0.087324 | 0.193904 |
| Inferno | 0.082404 | 0.151097 | 0.272143 |
| Magma | 0.051013 | 0.083189 | 0.072631 |
| Turbo | 0.402869 | 0.498381 | 0.387017 |

These measurements use the unchanged palette values in published Iris 0.1.2
(commit 345b91f0). Several differences are large; applications requiring close
agreement should assess the numbers rather than infer fidelity from a palette
name. No pre-existing numerical accuracy limit identifies these differences as
production defects, and this verification increment changes no control points.
The finite grid does not bound the continuous interval or perceptual error.

## Reproduction and integrity

Use an isolated reference-only Python environment with exactly
`matplotlib==3.11.2` and `numpy==2.4.6`, then run
`python tools/generate_palette_reference.py --check`.
The source hashes are checked before import. `--check` regenerates in memory
and requires byte identity; it never rewrites fixtures. Normal CI uses
`--verify-fixtures`, requiring only Python's standard library, to verify the
committed hashes, dimensions, finite normalized channels and license notice.

`cargo test --locked --test palette_reference` compares the actual Iris API to
the committed reference without requiring Matplotlib at runtime. Existing
endpoint, interpolation and normalized-color laws remain alongside this
independent oracle. Matplotlib is not a Rust or production dependency; its
fixture data retains the upstream terms in `MATPLOTLIB-LICENSE`.
