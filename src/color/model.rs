//! Normalized RGBA value and its display-channel convention.
//!
//! RGB channels are normalized sRGB-encoded display values. Alpha is normalized
//! linear opacity. Iris does not apply an sRGB transfer function when it
//! quantizes an [`Rgba`] value to bytes.

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
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
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
    /// opacity.
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
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "validated normalized channels prove rounded quantizer output lies in u8"
    )]
    pub fn to_rgba8(self) -> [u8; 4] {
        self.0.map(|value| {
            // This narrowing conversion is the specified uniform quantizer
            // q(v) = round(255v), whose range is proven to fit in u8.
            (value * 255.0 + 0.5) as u8
        })
    }
}
