//! Normalized RGBA value.

use crate::{IrisError, IrisResult};

/// Four normalized red, green, blue, and alpha channels.
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
