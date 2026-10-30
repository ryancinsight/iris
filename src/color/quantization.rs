//! Exact nearest-bin quantization of the validated binary32 coordinate.

use super::Normalized;

/// The existing structural bound shared by table construction and sampling.
pub(crate) const fn last_index<const N: usize>() -> usize {
    assert!(N >= 2, "lookup tables require at least two entries");
    assert!(
        N <= 16_777_217,
        "lookup-table indices must be exactly representable by f32"
    );
    N - 1
}

/// Round the exact product `(N - 1) * value` to an index, with ties upward.
///
/// A normal binary32 input is `significand / 2^shift`, where the significand
/// has 24 bits and `shift = 150 - biased_exponent`. The structural bound makes
/// the integer product less than `2^48`. For `shift >= 49` it is below one
/// half, including every subnormal and either signed zero. Shifting by
/// `min(shift - 1, 48)` therefore gives twice the scaled value, truncated,
/// or zero for those tiny inputs. Adding one and halving rounds exactly with
/// upward ties, without rounding a floating-point product first. The shifted
/// value is at most `2 * (N - 1)`, so the accumulator and result both fit.
#[inline]
pub(crate) fn nearest_index<const N: usize>(value: Normalized) -> usize {
    let last = const { last_index::<N>() };
    let bits = value.get().to_bits();
    let half_shift = (149 - ((bits >> 23) & 0xff)).min(48);
    let significand = u64::from((bits & 0x007f_ffff) | 0x0080_0000);
    let numerator = significand * last as u64;
    (((numerator >> half_shift) + 1) >> 1) as usize
}

#[cfg(test)]
mod tests {
    use super::{Normalized, nearest_index};

    #[test]
    fn maximum_resolution_preserves_odd_integral_indices() {
        // 0.5 + 2^-24 is exactly index 2^23 + 1 at the largest resolution.
        // Adding 0.5 in binary32 would round that odd integer up again.
        let input = Normalized::new(f32::from_bits(0x3f00_0001)).expect("unit interval");
        assert_eq!(nearest_index::<16_777_217>(input), 8_388_609);
    }

    #[test]
    fn maximum_resolution_matches_an_independent_exact_product() {
        // Binary64 exactly holds a binary32 value times a 24-bit integer:
        // at most 48 significant bits, within its 53-bit significand.
        for bits in (0..=1.0_f32.to_bits()).step_by(65_537).chain([
            0,
            1,
            0x007f_ffff,
            0x0080_0000,
            0x32ff_ffff,
            0x3300_0000,
            0x3300_0001,
            0x3eff_ffff,
            0x3f00_0000,
            0x3f00_0001,
            0x3f7f_ffff,
            0x3f80_0000,
        ]) {
            let value = f32::from_bits(bits);
            let input = Normalized::new(value).expect("unit interval");
            let actual = u32::try_from(nearest_index::<16_777_217>(input)).expect("24-bit index");
            let expected = (f64::from(value) * 16_777_216.0).round();
            assert_eq!(f64::from(actual).to_bits(), expected.to_bits(), "{bits:#x}");
        }
        assert_eq!(
            nearest_index::<16_777_217>(Normalized::new(-0.0).expect("signed zero")),
            0
        );
    }
}
