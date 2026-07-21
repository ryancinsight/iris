//! Borrowed paired-coordinate series.

use alloc::borrow::Cow;

use crate::{IrisError, IrisResult};

/// A named zero-copy series with paired coordinate and sample slices.
#[derive(Debug, Clone, PartialEq)]
pub struct SeriesView<'a, X, Y> {
    name: Cow<'a, str>,
    coordinates: &'a [X],
    samples: &'a [Y],
}

impl<'a, X, Y> SeriesView<'a, X, Y> {
    /// Validate and borrow a series.
    ///
    /// # Errors
    ///
    /// Returns [`IrisError::SeriesLengthMismatch`] when the slices differ in
    /// length.
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        coordinates: &'a [X],
        samples: &'a [Y],
    ) -> IrisResult<Self> {
        if coordinates.len() != samples.len() {
            return Err(IrisError::SeriesLengthMismatch {
                coordinates: coordinates.len(),
                samples: samples.len(),
            });
        }
        Ok(Self {
            name: name.into(),
            coordinates,
            samples,
        })
    }

    /// Borrow the series name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Borrow the coordinate slice.
    #[must_use]
    pub const fn coordinates(&self) -> &'a [X] {
        self.coordinates
    }

    /// Borrow the sample slice.
    #[must_use]
    pub const fn samples(&self) -> &'a [Y] {
        self.samples
    }

    /// Iterate paired coordinate and sample references without allocation.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&X, &Y)> {
        self.coordinates.iter().zip(self.samples)
    }
}
