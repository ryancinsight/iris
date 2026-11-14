# Position in the Stack

## What Iris Owns

Iris is the scientific visualization contract layer for Atlas. It owns:

- **Normalized color laws** — `ColorMap` trait and all built-in implementations
  (sequential, diverging, lookup-table maps)
- **Validated scalar and color types** — `Normalized`, `Rgba`, `LookupTable`
- **Zero-copy domain result views** — `ScalarFieldView`, `SeriesView`, `Axis`
- **Backend-independent rendering seam** — `RenderBackend` trait

Iris does **not** own domain physics, numerical solvers, array storage, or
file-format encoding. Those stay with their respective providers.

## Where Iris Sits

```text
eunomia ──► aequitas ──► ... domain solvers and applications ...
                               │
                               ▼
                            iris (visualization contracts)
                               │
                               ▼
                        domain solver or application
                        (owns rendering backend + output format)
```

Domain solvers and applications depend on Iris for the visualization vocabulary;
they supply the `RenderBackend` implementation and the array storage. Iris
itself has no dependency on any physics or solver package. The view-contract
suite demonstrates this seam with a scalar-field backend that maps through a
`LookupTable` into reused RGBA storage; that conformance backend is separate
from any device or window implementation.

## Consumers

Each row names a package whose manifests declare a direct dependency on
`iris-viz`. The revision is the consumer's default-branch commit at which the
manifests and the `iris` imports were read on 2026-11-14.

| Consumer | Manifests declaring `iris-viz` | Verified at | Iris items imported |
|----------|--------------------------------|-------------|---------------------|
| `CFDrs` | `Cargo.toml`, `crates/cfd-1d/Cargo.toml`, `crates/cfd-2d/Cargo.toml`, `crates/cfd-schematics/Cargo.toml` | `e4001eeec4f4` | `NamedColorMap`, `ColorMap`, `Normalized`, `Rgba` for schematic and analysis-field color |
| `ritk` | `Cargo.toml`, `crates/ritk-snap/Cargo.toml`, `crates/ritk-vtk/Cargo.toml` | `2953c69ec3c4` | `ColorMap`, `NamedColorMap`, `LookupTable`, `Normalized` for slice, fusion, and volume-render color and VTK lookup tables |

### Prospective consumers

`helios` declares no `iris-viz` dependency at `3f5d668bb4f1`. `kwavers`
declares none at `d8d711af5a76`; its `Cargo.lock` resolves `iris-viz` only
through `ritk-vtk`. Neither is a consumer until one of its manifests names the
package.

## Registry Name

The crate publishes as `iris-viz` on crates.io (the name `iris` was already
registered). Consumers write:

```toml
[dependencies]
iris = { package = "iris-viz", version = "0.1.1" }
```

All import paths remain `use iris::…` unchanged.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Link the standard library; without it the crate is `no_std` with `alloc` |
| `serde` | no | Derive `Serialize`/`Deserialize` for `NamedColorMap` |

Iris is `no_std`-compatible with an allocator when `default-features = false`.
`IrisError` implements `core::error::Error` in both configurations.
