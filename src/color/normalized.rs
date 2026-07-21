//! Validated unit-interval scalar.

use crate::{IrisError, IrisResult};

/// A finite scalar in the closed interval `[0, 1]`.
///
/// The transparent representation makes the validation boundary zero-cost
/// after construction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Normalized(f32);

impl Normalized {
    /// Construct a normalized value.
    ///
    /// # Errors
    ///
    /// Returns [`IrisError::NonFiniteNormalized`] for NaN or infinity and
    /// [`IrisError::NormalizedOutOfRange`] outside `[0, 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use iris::color::Normalized;
    ///
    /// assert_eq!(Normalized::new(0.25)?.get(), 0.25);
    /// assert!(Normalized::new(1.01).is_err());
    /// # Ok::<(), iris::IrisError>(())
    /// ```
    pub fn new(value: f32) -> IrisResult<Self> {
        if !value.is_finite() {
            return Err(IrisError::NonFiniteNormalized { value });
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(IrisError::NormalizedOutOfRange { value });
        }
        Ok(Self(value))
    }

    /// Return the validated scalar.
    #[must_use]
    pub const fn get(self) -> f32 {
        self.0
    }

    pub(crate) const fn from_unit_interval(value: f32) -> Self {
        debug_assert!(value >= 0.0 && value <= 1.0);
        Self(value)
    }
}
