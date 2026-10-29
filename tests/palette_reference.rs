//! Independent release-pinned agreement for the documented approximations.

use iris::color::{ColorMap, NamedColorMap, Normalized};

struct Reference {
    name: &'static str,
    map: NamedColorMap,
    json: &'static [u8],
    bound: [f64; 3],
}

#[test]
fn named_approximations_keep_their_documented_reference_agreement() {
    let references = [
        Reference {
            name: "bone",
            map: NamedColorMap::Bone,
            json: include_bytes!("fixtures/palette_reference/bone.json"),
            bound: [0.005_341, 0.003_498, 0.003_567],
        },
        Reference {
            name: "jet",
            map: NamedColorMap::Jet,
            json: include_bytes!("fixtures/palette_reference/jet.json"),
            bound: [0.017_756, 0.013_163, 0.013_317],
        },
        Reference {
            name: "plasma",
            map: NamedColorMap::Plasma,
            json: include_bytes!("fixtures/palette_reference/plasma.json"),
            bound: [0.244_877, 0.150_198, 0.172_135],
        },
        Reference {
            name: "viridis",
            map: NamedColorMap::Viridis,
            json: include_bytes!("fixtures/palette_reference/viridis.json"),
            bound: [0.283_955, 0.087_324, 0.193_904],
        },
        Reference {
            name: "inferno",
            map: NamedColorMap::Inferno,
            json: include_bytes!("fixtures/palette_reference/inferno.json"),
            bound: [0.082_404, 0.151_097, 0.272_143],
        },
        Reference {
            name: "magma",
            map: NamedColorMap::Magma,
            json: include_bytes!("fixtures/palette_reference/magma.json"),
            bound: [0.051_013, 0.083_189, 0.072_631],
        },
        Reference {
            name: "turbo",
            map: NamedColorMap::Turbo,
            json: include_bytes!("fixtures/palette_reference/turbo.json"),
            bound: [0.402_869, 0.498_381, 0.387_017],
        },
    ];
    for reference in references {
        let rows: Vec<[f64; 3]> =
            serde_json::from_slice(reference.json).expect("release-pinned RGB reference parses");
        assert_eq!(rows.len(), 256, "{} reference length", reference.name);
        assert!(
            rows.iter()
                .flatten()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        );
        let mut maximum = [0.0_f64; 3];
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
                // f64 represents the observed f32 exactly and compares it to
                // Matplotlib's independent f64 reference; Iris still computes f32.
                let error = (f64::from(actual[channel]) - expected[channel]).abs();
                maximum[channel] = maximum[channel].max(error);
                assert!(
                    error <= reference.bound[channel],
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
                observed > reference.bound[channel] - 0.000_001,
                "{} channel {channel}: documented ceiling is no longer tight",
                reference.name
            );
        }
    }
}
