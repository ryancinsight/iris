//! Borrowed const-rank scalar fields.

use crate::{IrisError, IrisResult};

/// Common lending access to scalar-field values.
pub trait ScalarField {
    /// Stored scalar type.
    type Scalar;

    /// Lending iterator over values.
    type Values<'a>: ExactSizeIterator<Item = &'a Self::Scalar>
    where
        Self: 'a;

    /// Borrow the runtime shape.
    fn shape(&self) -> &[usize];

    /// Iterate scalar references without copying.
    fn values(&self) -> Self::Values<'_>;
}

/// A validated, zero-copy, const-rank scalar-field view.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarFieldView<'a, T, const D: usize> {
    values: &'a [T],
    shape: [usize; D],
}

impl<'a, T, const D: usize> ScalarFieldView<'a, T, D> {
    /// Validate the shape against a borrowed scalar slice.
    ///
    /// # Errors
    ///
    /// Returns [`IrisError::ShapeCardinalityOverflow`] when the extent product
    /// overflows and [`IrisError::ShapeCardinalityMismatch`] when it differs
    /// from `values.len()`.
    pub fn new(values: &'a [T], shape: [usize; D]) -> IrisResult<Self> {
        const { assert!(D > 0, "scalar fields require positive rank") };

        let expected = shape.iter().try_fold(1_usize, |cardinality, extent| {
            cardinality.checked_mul(*extent)
        });
        let Some(expected) = expected else {
            return Err(IrisError::ShapeCardinalityOverflow);
        };
        if expected != values.len() {
            return Err(IrisError::ShapeCardinalityMismatch {
                expected,
                actual: values.len(),
            });
        }
        Ok(Self { values, shape })
    }

    /// Borrow the const-rank shape.
    #[must_use]
    pub const fn extents(&self) -> &[usize; D] {
        &self.shape
    }

    /// Borrow the contiguous scalar values.
    #[must_use]
    pub const fn as_slice(&self) -> &'a [T] {
        self.values
    }
}

impl<T, const D: usize> ScalarField for ScalarFieldView<'_, T, D> {
    type Scalar = T;
    type Values<'a>
        = core::slice::Iter<'a, T>
    where
        Self: 'a;

    fn shape(&self) -> &[usize] {
        &self.shape
    }

    fn values(&self) -> Self::Values<'_> {
        self.values.iter()
    }
}
