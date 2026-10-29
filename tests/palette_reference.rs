//! Independent release-pinned agreement for the documented approximations.

use iris::color::{ColorMap, NamedColorMap, Normalized};

struct Reference {
    name: &'static str,
    map: NamedColorMap,
    json: &'static [u8],
    bound: [u32; 3],
}

const DENOMINATOR: u128 = 1_u128 << 80;

fn fixed80(value: f64) -> u128 {
    let bits = value.to_bits();
    if bits.trailing_zeros() >= 63 {
        return 0;
    }
    assert_eq!(bits >> 63, 0, "negative RGB channel");
    assert!(
        bits <= 1.0_f64.to_bits(),
        "non-finite or unnormalized channel"
    );
    let exponent = u32::try_from((bits >> 52) & 0x7ff).expect("eleven exponent bits fit u32");
    // value = significand * 2^(exponent - 1023 - 52). Every nonzero value in
    // this finite corpus has exponent >= 995, so scaling by 2^80 is exact.
    // A future unsupported value requires re-deriving oracle precision.
    let shift = exponent
        .checked_sub(1023 + 52 - 80)
        .expect("sample cannot be represented exactly at oracle precision");
    let significand = (bits & 0x000f_ffff_ffff_ffff) | (1_u64 << 52);
    let scaled = u128::from(significand)
        .checked_shl(shift)
        .expect("normalized channel fits");
    assert!(scaled <= DENOMINATOR);
    scaled
}

#[test]
fn exact_ceiling_keeps_the_remainder_float_scaling_lost() {
    let error = fixed80(0.25).abs_diff(fixed80(0.494_877));
    assert!(error * 1_000_000 > 244_877 * DENOMINATOR);
    assert!(error * 1_000_000 <= 244_878 * DENOMINATOR);
}

#[test]
fn named_approximations_keep_their_documented_reference_agreement() {
    let references = [
        Reference {
            name: "bone",
            map: NamedColorMap::Bone,
            json: include_bytes!("fixtures/palette_reference/bone.json"),
            bound: [5_341, 3_498, 3_567],
        },
        Reference {
            name: "jet",
            map: NamedColorMap::Jet,
            json: include_bytes!("fixtures/palette_reference/jet.json"),
            bound: [17_756, 13_163, 13_317],
        },
        Reference {
            name: "plasma",
            map: NamedColorMap::Plasma,
            json: include_bytes!("fixtures/palette_reference/plasma.json"),
            bound: [244_878, 150_198, 172_135],
        },
        Reference {
            name: "viridis",
            map: NamedColorMap::Viridis,
            json: include_bytes!("fixtures/palette_reference/viridis.json"),
            bound: [283_955, 87_324, 193_904],
        },
        Reference {
            name: "inferno",
            map: NamedColorMap::Inferno,
            json: include_bytes!("fixtures/palette_reference/inferno.json"),
            bound: [82_404, 151_097, 272_143],
        },
        Reference {
            name: "magma",
            map: NamedColorMap::Magma,
            json: include_bytes!("fixtures/palette_reference/magma.json"),
            bound: [51_013, 83_189, 72_631],
        },
        Reference {
            name: "turbo",
            map: NamedColorMap::Turbo,
            json: include_bytes!("fixtures/palette_reference/turbo.json"),
            bound: [402_869, 498_381, 387_017],
        },
    ];
    for reference in references {
        let encoded: Vec<[u64; 3]> =
            serde_json::from_slice(reference.json).expect("release-pinned RGB reference parses");
        let rows: Vec<[f64; 3]> = encoded
            .into_iter()
            .map(|row| row.map(f64::from_bits))
            .collect();
        assert_eq!(rows.len(), 256, "{} reference length", reference.name);
        assert!(
            rows.iter()
                .flatten()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        );
        let mut maximum = [0_u128; 3];
        for step in 0_u16..=1024 {
            let input = Normalized::new(f32::from(step) / 1024.0)
                .expect("exact binary grid coordinate is normalized");
            let color = reference.map.sample(input);
            let actual = color.channels();
            assert_eq!(
                actual[3].to_bits(),
                1.0_f32.to_bits(),
                "{} alpha at {step}",
                reference.name
            );
            // The released default LUT uses floor(256*t), clamping t=1 to 255.
            let expected = rows[usize::from((step / 4).min(255))];
            for channel in 0..3 {
                // Both source values are exact dyadics. Integer comparison
                // avoids rounding away a remainder at a decimal boundary.
                let error =
                    fixed80(f64::from(actual[channel])).abs_diff(fixed80(expected[channel]));
                maximum[channel] = maximum[channel].max(error);
                assert!(
                    error * 1_000_000 <= u128::from(reference.bound[channel]) * DENOMINATOR,
                    "{} step {step} channel {channel}: error {error} exceeds {}",
                    reference.name,
                    reference.bound[channel]
                );
            }
        }
        // The documentation states the six-decimal outward ceiling, not an
        // arbitrary loose tolerance. A changed measurement requires re-derivation.
        for (channel, observed) in maximum.into_iter().enumerate() {
            assert!(
                observed * 1_000_000 > u128::from(reference.bound[channel] - 1) * DENOMINATOR,
                "{} channel {channel}: documented ceiling is no longer tight",
                reference.name
            );
        }
    }
}
