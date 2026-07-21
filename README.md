# Iris

Iris is Atlas's public, domain-neutral scientific visualization contract.
It owns normalized color laws, fixed-size color lookup tables, borrowed result
views, and the static render-backend seam. Physics, arrays, GPU mechanics, and
file formats remain in their existing providers.

```rust
use iris::color::{ColorMap, NamedColorMap, Normalized};

let midpoint = Normalized::new(0.5)?;
let rgba = NamedColorMap::Viridis.sample(midpoint);
assert_eq!(rgba.channels(), &[0.204, 0.636, 0.469, 1.0]);
# Ok::<(), iris::IrisError>(())
```

## Boundary

Iris owns:

- validated normalized color and built-in color-map laws;
- const-generic, allocation-free lookup tables;
- borrowed const-rank scalar-field and paired-series views;
- borrow-or-own axis metadata;
- a GAT render seam that can lend backend-owned frame storage.

Iris does not own:

- simulation equations or diagnostic interpretation;
- arrays, meshes, images, or persistent data models;
- GPU devices, buffers, shaders, windows, or user-interface state;
- PNG, DICOM, VTK, HDF5, or other file encoders.

## Architecture

```text
domain result storage
        │ borrow
        ▼
view::{SeriesView, ScalarFieldView}
        │
        ├── color::{ColorMap, LookupTable}
        │
        └── render::RenderBackend
                    │ static implementation
                    ▼
          consumer-owned renderer/encoder
```

The core uses no dynamic dispatch. Map strategies are zero-sized types;
`NamedColorMap` provides exhaustive enum dispatch when runtime selection is
required. Lookup-table resolution and scalar-field rank are const generic.
Views borrow caller storage, and labels use `Cow<'_, str>` so borrowed metadata
does not allocate.

The built-in blue-to-red law is the exact linear map
`c(t) = (t, 0, 1 - t, 1)`. It is intended for consumers that require endpoint
polarity without a neutral midpoint; `CoolWarm` remains the three-pole
blue-white-red alternative.

## Verification

The color invariant follows by induction over each piecewise interval: control
channels are in `[0, 1]`, and linear interpolation is a convex combination of
adjacent channels. Exhaustive grid tests sample every built-in map at 1,025
points. Analytical endpoint and control-point tests establish the named-map
contracts. Layout tests establish that map strategies are zero-sized and do not
increase lookup-table storage. Borrow-identity tests establish zero-copy view
construction.

The built-in names and classifications follow the
[Matplotlib colormap reference](https://matplotlib.org/stable/gallery/color/colormap_reference.html).
`CoolWarm` is explicitly an RGB control-point interpolation, not an
implementation of the Msh interpolation in Moreland's
[Diverging Color Maps for Scientific Visualization](https://www.kennethmoreland.com/color-maps/ColorMaps.pdf).

## License

Licensed under either Apache-2.0 or MIT, at your option.
