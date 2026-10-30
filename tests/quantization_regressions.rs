//! Exact nearest-bin laws, including the neighbors of rounding boundaries.

use iris::color::{LookupTable, Normalized, Rgba, map::Grayscale};

#[test]
fn lut_selects_lower_entry_immediately_below_midpoint() {
    let table = LookupTable::<Grayscale, 2>::from_map(Grayscale);
    let input = Normalized::new(f32::from_bits(0x3eff_ffff)).expect("below one half");
    assert_eq!(table.sample(input), table.entries()[0]);
}

#[test]
fn rgba_rounds_below_half_to_the_nearest_byte() {
    // Exact scaled value: 255 * v = 16_777_215 / 33_554_432 < 1/2.
    let value = f32::from_bits(0x3b00_8080);
    let color = Rgba::new([value; 4]).expect("normalized channels");
    assert_eq!(color.to_rgba8(), [0; 4]);
}

fn check_bytes(value: f32) {
    let actual = Rgba::new([value; 4]).expect("unit interval").to_rgba8();
    // A binary32 significand times 255 needs at most 32 bits. Binary64's
    // 53-bit significand holds this product exactly, independently of the
    // production integer codec. abs() canonicalizes the permitted -0.0.
    let expected = (f64::from(value) * 255.0).round().abs();
    for byte in actual {
        assert_eq!(f64::from(byte).to_bits(), expected.to_bits(), "{value:?}");
    }
}

#[test]
fn every_byte_midpoint_and_neighbor_rounds_to_the_exact_nearest_byte() {
    for lower in 0_u8..255 {
        let boundary = (f32::from(lower) + 0.5) / 255.0;
        for bits in boundary.to_bits() - 2..=boundary.to_bits() + 2 {
            check_bytes(f32::from_bits(bits));
        }
    }
}

#[test]
fn byte_quantization_covers_exponents_subnormals_endpoints_and_signed_zero() {
    for bits in (0..=1.0_f32.to_bits()).step_by(65_537) {
        check_bytes(f32::from_bits(bits));
    }
    for value in [-0.0, 0.0, f32::from_bits(1), f32::MIN_POSITIVE, 1.0] {
        check_bytes(value);
    }
    assert_eq!(
        Rgba::new([0.0, 0.5, 1.0, f32::from_bits(0x3b00_8080)])
            .expect("distinct normalized channels")
            .to_rgba8(),
        [0, 128, 255, 0]
    );
}

fn check_table<const N: usize>() {
    let table = LookupTable::<Grayscale, N>::from_map(Grayscale);
    let last = u16::try_from(N - 1).expect("test resolutions fit u16");
    let check = |value: f32| {
        // At most 40 significant bits here: this product is exact in binary64.
        let nearest = (f64::from(value) * f64::from(last)).round().abs();
        let expected_index = (0..=last)
            .find(|index| f64::from(*index).to_bits() == nearest.to_bits())
            .expect("exact index lies in the table");
        assert_eq!(
            table.sample(Normalized::new(value).expect("unit interval")),
            table.entries()[usize::from(expected_index)],
            "N={N}, value={value:?}"
        );
    };
    for lower in 0..last {
        let boundary = (f32::from(lower) + 0.5) / f32::from(last);
        for bits in boundary.to_bits() - 2..=boundary.to_bits() + 2 {
            check(f32::from_bits(bits));
        }
    }
    for value in [-0.0, 0.0, f32::from_bits(1), f32::MIN_POSITIVE, 0.5, 1.0] {
        check(value);
    }
}

#[test]
fn table_midpoint_neighbors_respect_odd_and_even_resolutions() {
    check_table::<2>();
    check_table::<3>();
    check_table::<4>();
    check_table::<5>();
    check_table::<8>();
    check_table::<9>();
    check_table::<16>();
    check_table::<17>();
    check_table::<256>();
    check_table::<257>();
    check_table::<1024>();
    check_table::<1025>();
}
