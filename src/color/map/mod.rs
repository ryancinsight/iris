//! Static color-map strategies and closed-set runtime selection.
//!
//! RGB channels use normalized sRGB-encoded display values throughout this
//! module. Interpolation is performed in that encoded space, not in
//! linear-light RGB; [`Rgba::to_rgba8`](crate::color::Rgba::to_rgba8) applies
//! direct nearest-byte quantization at the output boundary.

mod diverging;
mod interpolation;
mod named;
mod sequential;
mod table;

use super::{Normalized, Rgba};

pub use diverging::{BlueRed, CoolWarm};
pub use named::NamedColorMap;
pub use sequential::{Cool, Grayscale, Hot, Inverted, Rainbow};
pub use table::{Bone, Inferno, Jet, Magma, Plasma, Turbo, Viridis};

/// A statically dispatched normalized scalar-to-sRGB color law.
///
/// Implementations return RGB channels as normalized sRGB-encoded display
/// values and alpha as normalized linear opacity. An implementation that needs
/// linear-light arithmetic must perform the explicit transfer at its own
/// domain boundary; the built-in maps and their interpolation do not do so.
pub trait ColorMap: Copy {
    /// Evaluate the color law at a validated normalized scalar.
    #[must_use]
    fn sample(self, value: Normalized) -> Rgba;
}
