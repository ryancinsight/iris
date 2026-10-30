//! Fixed-size sampled color maps.

use core::marker::PhantomData;

use super::quantization::{last_index, nearest_index};
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
    ///
    /// ```compile_fail
    /// use iris::color::{LookupTable, map::Grayscale};
    /// let _ = LookupTable::<Grayscale, 1>::from_map(Grayscale);
    /// ```
    ///
    /// ```compile_fail
    /// use iris::color::{LookupTable, map::Grayscale};
    /// let _ = LookupTable::<Grayscale, 16_777_218>::from_map(Grayscale);
    /// ```
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "N is compile-time bounded to the exact integer range of f32"
    )]
    pub fn from_map(map: M) -> Self {
        const { last_index::<N>() };

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
    ///
    /// The exact represented input is quantized against the ideal uniform
    /// grid. Midpoint ties select the higher index.
    #[must_use]
    pub fn sample(&self, value: Normalized) -> Rgba {
        self.entries[nearest_index::<N>(value)]
    }

    /// Borrow the table entries without copying.
    #[must_use]
    pub const fn entries(&self) -> &[Rgba; N] {
        &self.entries
    }
}
