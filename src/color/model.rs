//! Normalized RGBA value and its display-channel convention.
//!
//! RGB channels are normalized sRGB-encoded display values. Alpha is normalized
//! linear opacity. Iris does not apply an sRGB transfer function when it
//! quantizes an [`Rgba`] value to bytes.

use super::normalized::in_unit_interval;
use super::{Normalized, quantization::nearest_index};
use crate::{IrisError, IrisResult};

/// Four normalized channels with sRGB RGB and linear-opacity alpha semantics.
///
/// The red, green, and blue channels are sRGB-encoded values in `[0, 1]`, not
/// linear-light intensities. Color-map interpolation therefore also occurs in
/// encoded sRGB channel space. The alpha channel is normalized linear opacity.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Rgba([f32; 4]);

impl Rgba {
    /// Construct a validated normalized color.
    ///
    /// # Errors
    ///
    /// Returns [`IrisError::InvalidColorChannel`] when any channel is not
    /// finite or lies outside `[0, 1]`.
    pub fn new(channels: [f32; 4]) -> IrisResult<Self> {
        for (channel, value) in channels.iter().copied().enumerate() {
            if !in_unit_interval(value) {
                return Err(IrisError::InvalidColorChannel { channel, value });
            }
        }
        Ok(Self(channels))
    }

    pub(crate) const fn opaque(rgb: [f32; 3]) -> Self {
        debug_assert!(rgb[0] >= 0.0 && rgb[0] <= 1.0);
        debug_assert!(rgb[1] >= 0.0 && rgb[1] <= 1.0);
        debug_assert!(rgb[2] >= 0.0 && rgb[2] <= 1.0);
        Self([rgb[0], rgb[1], rgb[2], 1.0])
    }

    /// Borrow the normalized channels in RGBA order.
    #[must_use]
    pub const fn channels(&self) -> &[f32; 4] {
        &self.0
    }

    /// Convert normalized channels to nearest 8-bit channel values.
    ///
    /// RGB values are already sRGB-encoded, so this applies the uniform
    /// quantizer `round(255v)` directly and performs no transfer-function
    /// conversion. Alpha uses the same quantizer because it is normalized
    /// opacity. Rounding uses the exact represented input, with midpoint ties
    /// selecting the higher byte.
    ///
    /// # Panics
    ///
    /// The integer conversion checks the quantizer's internal range. For
    /// every validated `Rgba`, that range is `0..=255`, so the check cannot fail.
    ///
    /// ```
    /// # use iris::color::Rgba;
    /// let bytes = [0, 128, 255, 128];
    /// let normalized = bytes.map(|value| f32::from(value) / 255.0);
    /// let color = Rgba::new(normalized)?;
    /// assert_eq!(color.to_rgba8(), bytes);
    /// # Ok::<(), iris::IrisError>(())
    /// ```
    #[must_use]
    pub fn to_rgba8(self) -> [u8; 4] {
        self.0.map(|value| {
            // Rgba construction already proves the unit-interval invariant.
            u8::try_from(nearest_index::<256>(Normalized::from_unit_interval(value)))
                .expect("validated channel quantizes to an index in 0..=255")
        })
    }
}
