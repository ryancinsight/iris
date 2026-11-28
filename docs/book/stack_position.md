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

Domain solvers and applications depend on Iris for the visualization vocabulary
and supply the array storage. Of the consumers below, only `metis` implements
`RenderBackend` (`metis-ui-lang` for `DisplayList` on `Framebuffer`); `kwavers`
and `metis-frontend` import the trait to call a backend. Iris itself has no
dependency on any physics or solver package. The view-contract
suite demonstrates this seam with a scalar-field backend that maps through a
`LookupTable` into reused RGBA storage; that conformance backend is separate
from any device or window implementation.

## Consumers

Each row names a repository whose manifests declare a direct dependency on
`iris-viz`. The revision is the repository's default-branch commit at which the
manifests and the `iris` imports were read on 2026-11-27. A manifest name
without a mark lists the package under `[dependencies]`. A mark replaces that
with the tables it names: `workspace` is `[workspace.dependencies]`, which
member manifests inherit with `workspace = true`; `dev` is
`[dev-dependencies]`; `optional` is a `[dependencies]` entry with
`optional = true`. `(workspace, dev)` declares it in both tables.

| Consumer | Manifests declaring `iris-viz` | Verified at | Iris items imported |
|----------|--------------------------------|-------------|---------------------|
| `CFDrs` | `Cargo.toml` (workspace), `crates/cfd-1d/Cargo.toml` (dev), `crates/cfd-2d/Cargo.toml` (dev), `crates/cfd-schematics/Cargo.toml` | `b76f00a72132` | `NamedColorMap`, `ColorMap`, `Normalized`, `Rgba` for schematic and analysis-field color; the `cfd-1d` and `cfd-2d` imports are in their examples |
| `kwavers` | `Cargo.toml` (workspace), `crates/kwavers-analysis/Cargo.toml` (optional) | `34fa3f47ae07` | `NamedColorMap`, `Rgba`, `ScalarFieldView`, `SeriesView`, `RenderBackend` for the `plotting` feature of `kwavers-analysis` |
| `metis` | `Cargo.toml` (workspace, dev), `crates/metis-frontend/Cargo.toml`, `crates/metis-ui-lang/Cargo.toml` | `0f1cfc9bc916` | `RenderBackend`; `metis-ui-lang` implements it for `DisplayList` on `Framebuffer` |
| `ritk` | `Cargo.toml` (workspace), `crates/ritk-snap/Cargo.toml`, `crates/ritk-vtk/Cargo.toml` | `fe35fe8630ab` | `ColorMap`, `NamedColorMap`, `LookupTable`, `Normalized` for slice, fusion, and volume-render color and VTK lookup tables |

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
