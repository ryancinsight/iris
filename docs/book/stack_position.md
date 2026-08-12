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

```
eunomia ──► aequitas ──► ... solvers (helios, kwavers, CFDrs, ritk) ...
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
itself has no dependency on any physics or solver package.

## Consumers

| Consumer | How Iris is used |
|----------|-----------------|
| `helios` | Dose-map and imaging scalar field views |
| `kwavers` | Ultrasound pressure field visualization |
| `ritk` | Medical image series rendering |
| `CFDrs` | Fluid simulation field views |

## Registry Name

The crate publishes as `iris-viz` on crates.io (the name `iris` was already
registered). Consumers write:

```toml
[dependencies]
iris = { package = "iris-viz", version = "0.1.0" }
```

All import paths remain `use iris::…` unchanged.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Enable `std`-only functionality |
| `serde` | no | Derive `Serialize`/`Deserialize` for `NamedColorMap` |

Iris is `no_std`-compatible with an allocator when `default-features = false`.
