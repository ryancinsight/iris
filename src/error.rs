//! Typed contract failures.

use core::fmt;

/// A failure while validating a visualization-domain value or view.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum IrisError {
    /// A normalized value was not finite.
    NonFiniteNormalized {
        /// The rejected value.
        value: f32,
    },
    /// A finite normalized value was outside the closed unit interval.
    NormalizedOutOfRange {
        /// The rejected value.
        value: f32,
    },
    /// A normalized color channel was invalid.
    InvalidColorChannel {
        /// Zero-based RGBA channel index.
        channel: usize,
        /// The rejected value.
        value: f32,
    },
    /// Coordinate and sample cardinalities differed.
    SeriesLengthMismatch {
        /// Number of coordinates.
        coordinates: usize,
        /// Number of samples.
        samples: usize,
    },
    /// Multiplying field extents overflowed `usize`.
    ShapeCardinalityOverflow,
    /// The field shape and storage cardinalities differed.
    ShapeCardinalityMismatch {
        /// Product of the shape extents.
        expected: usize,
        /// Number of stored scalar values.
        actual: usize,
    },
}

impl fmt::Display for IrisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NonFiniteNormalized { value } => {
                write!(formatter, "normalized value must be finite, got {value}")
            }
            Self::NormalizedOutOfRange { value } => {
                write!(formatter, "normalized value must be in [0, 1], got {value}")
            }
            Self::InvalidColorChannel { channel, value } => write!(
                formatter,
                "RGBA channel {channel} must be finite and in [0, 1], got {value}"
            ),
            Self::SeriesLengthMismatch {
                coordinates,
                samples,
            } => write!(
                formatter,
                "series coordinates ({coordinates}) and samples ({samples}) must have equal length"
            ),
            Self::ShapeCardinalityOverflow => {
                formatter.write_str("scalar-field shape cardinality overflowed usize")
            }
            Self::ShapeCardinalityMismatch { expected, actual } => write!(
                formatter,
                "scalar-field shape requires {expected} values, got {actual}"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IrisError {}

/// Result type for Iris contract validation.
pub type IrisResult<T> = Result<T, IrisError>;
