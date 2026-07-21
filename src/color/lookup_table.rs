//! Fixed-size sampled color maps.

use core::marker::PhantomData;

use super::{ColorMap, Normalized, Rgba};

/// A fixed-size lookup table sampled from a static color-map strategy.
///
/// `M` is type-level evidence only. It adds no runtime storage, while `N`
/// fixes the table resolution and allocation size at compile time.
#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct LookupTable<M, const N: usize> {
    entries: [Rgba; N],
    map: PhantomData<M>,
}

impl<M: ColorMap, const N: usize> LookupTable<M, N> {
    /// Sample `map` uniformly at both endpoints and `N - 2` interior points.
    ///
    /// # Panics
    ///
    /// Compilation fails for `N < 2` or resolutions too large for exact `f32`
    /// integer representation. These are structural programming errors fixed by
    /// the const argument, not input-dependent failures.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "N is compile-time bounded to the exact integer range of f32"
    )]
    pub fn from_map(map: M) -> Self {
        const {
            assert!(N >= 2, "lookup tables require at least two entries");
            assert!(
                N <= 16_777_217,
                "lookup-table indices must be exactly representable by f32"
            );
        }

        let denominator = (N - 1) as f32;
        let entries = core::array::from_fn(|index| {
            let value = Normalized::from_unit_interval(index as f32 / denominator);
            map.sample(value)
        });
        Self {
            entries,
            map: PhantomData,
        }
    }

    /// Select the nearest uniformly sampled entry.
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "validated unit input and bounded N prove nearest-bin quantization fits usize"
    )]
    pub fn sample(&self, value: Normalized) -> Rgba {
        // This float-to-index conversion is the specified nearest-bin
        // quantizer. Constructor bounds prove the rounded value fits usize.
        let index = (value.get() * (N - 1) as f32 + 0.5) as usize;
        self.entries[index.min(N - 1)]
    }

    /// Borrow the table entries without copying.
    #[must_use]
    pub const fn entries(&self) -> &[Rgba; N] {
        &self.entries
    }
}
