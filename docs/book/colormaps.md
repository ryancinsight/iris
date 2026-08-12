# Color Maps

Iris provides a closed set of normalized scalar-to-color laws, all sharing the
`ColorMap` trait. Each map is a pure function `c : [0, 1] → [0, 1]⁴` with no
mutable state: the normalized input scalar maps to an RGBA tuple whose four
channels also lie in `[0, 1]`.

## The `ColorMap` Trait

```rust,ignore
pub trait ColorMap: Copy {
    fn sample(self, value: Normalized) -> Rgba;
}
```

`sample` receives a [`Normalized`](./colormaps.md) value — a validated `f32` in
`[0, 1]` — and returns an `Rgba` struct (four normalized `f32` channels). The
`Copy` bound means maps are cheap to pass by value and store inline.

## `Normalized` Input

[`Normalized`] wraps a `f32` that has been validated to be finite and in
`[0, 1]`. Construction returns an `IrisResult`:

```rust,ignore
let n = Normalized::new(0.75)?;      // validated floating-point construction
let n = Normalized::from_u8(192);    // exact: 192 / 255, no validation branch
```

## `Rgba` Output

[`Rgba`] wraps `[f32; 4]` (RGBA channels, each in `[0, 1]`).

## Built-In Named Maps

All built-in maps are available through `NamedColorMap`, a `#[non_exhaustive]`
enum that can be used for runtime color-map selection:

```rust,ignore
use iris::color::NamedColorMap;

let map = NamedColorMap::Viridis;
let color = map.sample(Normalized::from_u8(128));
```

`NamedColorMap::ALL` lists the 14 built-in maps in stable display order.

### Sequential maps

| Variant | Description |
|---------|-------------|
| `Grayscale` | Monotone black-to-white |
| `Inverted` | Monotone white-to-black |
| `Hot` | Black → red → yellow → white |
| `Cool` | Cyan to magenta |
| `Bone` | Gray-blue sequential |
| `Plasma` | Purple-orange-yellow |
| `Viridis` | Perceptually ordered purple-green-yellow |
| `Inferno` | Black-purple-red-orange-yellow |
| `Magma` | Black-purple-red-orange-white |
| `Turbo` | High-dynamic-range rainbow-like |
| `Rainbow` | Blue-to-red HSV hue sweep |

### Diverging maps

| Variant | Description |
|---------|-------------|
| `CoolWarm` | Piecewise-linear blue-white-red |
| `BlueRed` | Linear blue-to-red, no neutral midpoint |

### Multi-hue non-sequential

| Variant | Description |
|---------|-------------|
| `Jet` | Blue-cyan-green-yellow-red |

## Lookup-Table Maps

Maps such as `Viridis`, `Inferno`, `Plasma`, `Magma`, `Turbo`, `Jet`, and
`Bone` are implemented as [`LookupTable`] instances — a fixed-size array of
pre-computed control points. `sample` linearly interpolates between the two
nearest entries, so each output channel lies in the convex hull of its
bracketing control values. Since all control points are in `[0, 1]`, the RGBA
invariant is preserved under interpolation.

## Custom Maps

Implement `ColorMap` on any `Copy` type to add a custom law:

```rust,ignore
#[derive(Clone, Copy)]
struct MyMap;

impl iris::color::ColorMap for MyMap {
    fn sample(self, value: iris::color::Normalized) -> iris::color::Rgba {
        // ...
    }
}
```

The `RenderBackend` trait is generic over any `ColorMap`, so custom maps compose
with the rest of the rendering pipeline without changes to Iris internals.
