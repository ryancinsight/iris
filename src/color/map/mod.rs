//! Static color-map strategies and closed-set runtime selection.

mod diverging;
mod interpolation;
mod named;
mod sequential;
mod table;

use super::{Normalized, Rgba};

pub use diverging::{BlueRed, CoolWarm};
pub use named::NamedColorMap;
pub use sequential::{Cool, Grayscale, Hot, Inverted, Rainbow};
pub use table::{Bone, Jet, Plasma, Viridis};

/// A statically dispatched normalized scalar-to-color law.
pub trait ColorMap: Copy {
    /// Evaluate the color law at a validated normalized scalar.
    #[must_use]
    fn sample(self, value: Normalized) -> Rgba;
}
