# Changelog

All notable changes are documented in this file.

## Unreleased

### Added

- A crates.io trusted-publishing workflow for the `iris` package.
- A zero-sized linear blue-to-red color-map strategy and corresponding
  runtime-selection variant for direct CFDrs integration.
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
