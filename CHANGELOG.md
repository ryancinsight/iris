# Changelog

All notable changes are documented in this file.

## Unreleased

### Added

- The normalized sRGB-encoded RGB and linear-opacity alpha convention, direct
  `round(255v)` byte quantization, and the non-linear-light interpolation
  contract are now documented and tested.
- A crates.io trusted-publishing workflow for the `iris-viz` package; the Rust
  import path remains `iris`.
- A zero-sized linear blue-to-red color-map strategy and corresponding
  runtime-selection variant for direct CFDrs integration.
- Inferno, magma, and turbo built-in color-map strategies and runtime variants
  for consumer migration of duplicated visualization laws.
- Public normalized color laws and const-generic lookup tables.
- Branch-free conversion from the complete 8-bit grid to normalized color
  coordinates.
- Zero-copy series and const-rank scalar-field views.
- GAT render-backend contract for lending reusable frame storage.
- ADR, executable examples, law tests, documentation, and CI gates.
- Document and pin the zero-extent contract for `ScalarFieldView::new`: a
  shape containing a zero extent is valid only with an empty `values` slice,
  preserving the borrowed storage pointer with no sentinel allocation. A
  value-semantic test asserts the empty view keeps its extents, its storage
  pointer, and its shape.
- Add an exhaustive defining-module law test that rejects duplicate or omitted
  entries in `NamedColorMap::ALL` while retaining the public enum's
  `#[non_exhaustive]` compatibility contract.
