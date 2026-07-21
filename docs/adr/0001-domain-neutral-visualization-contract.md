# ADR 0001: Domain-neutral visualization contract

- Status: Accepted
- Date: 2026-07-21
- Class: `[arch]` `[minor]`

## Context

Atlas's promotion gate requires a real shared implementation, two consumers or
an ownership defect, explicit dependency direction, and deletion of superseded
logic. The source audit found independent named color laws in
`ritk-snap/src/render/colormap.rs`,
`ritk-vtk/src/domain/mapper.rs`, and
`kwavers-analysis/src/visualization/renderer/volume.rs`. RITK also exposes
backend-specific scalar views, while Kwavers and CFDrs build plotting series
from copied coordinate and value vectors.

No existing Atlas provider owns presentation semantics. Leto owns arrays,
Hephaestus owns GPU mechanics, RITK and Consus own formats, and domain packages
own interpretation of their results.

## Decision

Create Iris as a public, independently versioned Rust crate. Its first vertical
slice owns normalized RGBA values, named color laws, const-generic lookup
tables, borrowed series and scalar-field views, and a GAT render-backend seam.

Dependency direction is consumer to Iris. Iris has no dependency on a solver,
array, GPU, UI, or format crate. Static map strategies and render backends
monomorphize; `NamedColorMap` is the closed-set runtime selection boundary.

The first consumer migration replaces the duplicated color-law computations in
RITK's `ritk-snap` and `ritk-vtk` packages. The Kwavers copy remains a separately
scoped migration because its current working tree is actively owned.

## Alternatives

- Keep color laws in RITK: rejected because Kwavers and non-imaging simulation
  outputs already need the same presentation law.
- Place visualization in RITK: rejected because RITK owns medical imaging and
  file-format concerns, not domain-neutral simulation presentation.
- Expose a dynamic renderer object: rejected because current consumers have
  compile-time-known backends and can use static dispatch.

## Invariants and proof obligations

For normalized input `t`, every output channel is finite and in `[0, 1]`.
Analytic maps clamp their intermediate channels. Table maps interpolate two
validated control channels with fraction `u in [0, 1]`; therefore
`(1-u)a + ub` lies between `a` and `b`.

`SeriesView` requires equal coordinate and sample lengths. `ScalarFieldView`
uses checked extent multiplication and requires that product to equal storage
length. These checks occur once at construction, so iteration needs no parallel
validity state.

## Verification

- exhaustive finite/range checks over 1,025 points per built-in map;
- exact endpoint and control-point assertions;
- LUT/direct-map differential tests at every sampled node;
- ZST and table-layout assertions;
- negative cardinality and non-finite-input tests;
- RITK differential tests before deletion of both superseded implementations.

Iris PR 1 merged the consumer-required byte normalization at
`e2edd47615454111b4b0df2e68dc6076161ba457`. RITK PR 46 merged the direct Snap
and VTK migration as `1bc665d4c2d56c97e1b2b51e7135e9a86bf14d08` after its
Rust, Python, and migration-audit matrices passed; PR 47 closed the exact
merge-result verification and formatting repair as
`a36e65dfe1d4401d6950ebc31123205b9db04c50`. Atlas PR 71 registered Iris and
the initial consumer revision at
`6740296bf1ad45c4ff77dc4e2aaa64e2971e4ecd`.

## References

- [Matplotlib colormap reference](https://matplotlib.org/stable/gallery/color/colormap_reference.html)
- Kenneth Moreland, [Diverging Color Maps for Scientific Visualization](https://www.kennethmoreland.com/color-maps/ColorMaps.pdf), 2009.
