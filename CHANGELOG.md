# Changelog

All notable changes are documented in this file.

## 0.1.3

### Added

- [patch] Release-pinned Matplotlib reference fixtures and measured RGB agreement
  tests for Bone, Jet, Plasma, Viridis, Inferno, Magma and Turbo; document the
  six-decimal outward bounds on the 1,025-point grid. Keep all palette values
  and public APIs unchanged. Preserve fixture provenance and upstream notices.

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

### Fixed

- [patch] Make scalar-field zero cardinality independent of axis order. After
  checked prefix overflow, recognize a later zero extent; accept empty storage
  or report the exact zero/actual mismatch. Keep nonzero overflow, positive-rank
  enforcement, borrowed storage and iterator behavior unchanged.
- Add dimension-permutation, nonzero-overflow, zero-sized-element and rank-zero
  regression controls, including no-default coverage. Keep layout-provider and
  publication/authentication contracts outside this correction.

The crate version moves to 0.1.2 with this correction. Source/oracle and
caller/resource evidence is in `docs/verification/zero-cardinality.md`; the
release/authentication acceptance in IRIS-003 stays open, and no whole-render
performance guarantee is claimed.
