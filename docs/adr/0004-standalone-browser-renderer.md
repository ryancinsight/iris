# ADR 0004: Standalone browser renderer

- Status: Proposed
- Date: 2026-11-18

## Context

Kwavers writes pressure slices, point clouds, and timelines to interactive HTML.
Iris already owns shared color laws, borrowed scientific views, and the
backend contract. Browser scene composition can depend on those contracts
without adding an HTML dependency to the domain crate.

## Decision

Provide an independent package, `iris-plot-html`, with a concrete, statically
dispatched backend. It accepts binary64 data for the browser number domain
and serializes borrowed views into one backend-owned byte buffer. A Canvas
renderer embedded alongside the JSON handles navigation. Heatmap values
follow row-major order, with one validated coordinate axis per dimension;
scalar-colored points validate one value per point. A non-finite line sample
opens a gap and a non-finite scalar renders as missing. Escaping U+003C in
the JSON prevents label content from closing the embedded data script.

## Alternatives

A graphics API dependency would put device allocation on the export path and
would require an installed graphics stack. A network-hosted browser renderer
would make saved files depend on external access. A raster export alone would
discard hover values and spatial navigation.

## Verification

Assert the exact JSON data and labels, row-major orientation, validation
errors, palette equivalence to Iris, and renderer-buffer reuse. Exercise
rendering and interaction in a browser before closing the Kwavers cutover.
