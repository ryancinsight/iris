//! Behavioral laws for normalized color maps and lookup tables.

use core::mem::size_of;

use iris::color::{ColorMap, LookupTable, NamedColorMap, Normalized, Rgba, map::Grayscale};

fn normalized(value: f32) -> Normalized {
    Normalized::new(value).expect("test value lies in the unit interval")
}

#[test]
fn normalization_rejects_non_finite_and_out_of_domain_values() {
    assert!(Normalized::new(f32::NAN).is_err());
    assert!(Normalized::new(f32::INFINITY).is_err());
    assert!(Normalized::new(-f32::EPSILON).is_err());
    assert!(Normalized::new(1.0 + f32::EPSILON).is_err());
}

#[test]
fn byte_coordinates_cover_the_uniform_unit_grid_exactly() {
    let mut previous = Normalized::from_u8(0).get();
    assert_eq!(previous.to_bits(), 0.0_f32.to_bits());

    for value in 1_u8..=u8::MAX {
        let actual = Normalized::from_u8(value).get();
        let expected = f32::from(value) / 255.0;
        assert_eq!(actual.to_bits(), expected.to_bits());
        assert!(actual > previous);
        previous = actual;
    }

    assert_eq!(previous.to_bits(), 1.0_f32.to_bits());
}

#[test]
fn every_named_map_preserves_normalized_rgba_invariant() {
    for map in NamedColorMap::ALL {
        for step in 0_u16..=1024 {
            let value = normalized(f32::from(step) / 1024.0);
            let color = map.sample(value);
            let channels = color.channels();
            assert!(channels.iter().all(|channel| channel.is_finite()));
            assert!(channels.iter().all(|channel| (0.0..=1.0).contains(channel)));
            assert_eq!(channels[3].to_bits(), 1.0_f32.to_bits());
        }
    }
}

#[test]
fn grayscale_is_monotone_and_channel_equal() {
    let mut previous = 0_u8;
    for step in 0_u16..=255 {
        let [red, green, blue, alpha] = Grayscale
            .sample(normalized(f32::from(step) / 255.0))
            .to_rgba8();
        let expected = u8::try_from(step).expect("test step is at most 255");
        assert_eq!([red, green, blue], [expected; 3]);
        assert!(red >= previous);
        assert_eq!(alpha, 255);
        previous = red;
    }
}

#[test]
fn analytic_endpoints_and_control_points_are_exact() {
    assert_eq!(
        NamedColorMap::Grayscale.sample(normalized(0.0)).to_rgba8(),
        [0, 0, 0, 255]
    );
    assert_eq!(
        NamedColorMap::Grayscale.sample(normalized(1.0)).to_rgba8(),
        [255; 4]
    );
    assert_eq!(
        NamedColorMap::Inverted.sample(normalized(0.0)).to_rgba8(),
        [255; 4]
    );
    assert_eq!(
        NamedColorMap::Cool.sample(normalized(0.0)).to_rgba8(),
        [0, 255, 255, 255]
    );
    assert_eq!(
        NamedColorMap::Cool.sample(normalized(1.0)).to_rgba8(),
        [255, 0, 255, 255]
    );
    assert_eq!(
        NamedColorMap::Viridis
            .sample(normalized(0.5))
            .channels()
            .map(f32::to_bits),
        [0.204, 0.636, 0.469, 1.0].map(f32::to_bits)
    );
    assert_eq!(
        NamedColorMap::CoolWarm
            .sample(normalized(0.5))
            .channels()
            .map(f32::to_bits),
        [1.0; 4].map(f32::to_bits)
    );
}

#[test]
fn lookup_table_matches_direct_map_at_sample_nodes() {
    let table = LookupTable::<Grayscale, 256>::from_map(Grayscale);
    for index in 0_u16..=255 {
        let value = normalized(f32::from(index) / 255.0);
        assert_eq!(table.sample(value), Grayscale.sample(value));
    }
}

#[test]
fn strategy_is_zero_sized_and_table_has_no_strategy_storage() {
    assert_eq!(size_of::<Grayscale>(), 0);
    assert_eq!(
        size_of::<LookupTable<Grayscale, 256>>(),
        256 * size_of::<Rgba>()
    );
}

#[cfg(feature = "serde")]
#[test]
fn named_map_serde_round_trip_preserves_selection() {
    for map in NamedColorMap::ALL {
        let encoded = serde_json::to_string(&map).expect("named map serializes");
        let decoded: NamedColorMap =
            serde_json::from_str(&encoded).expect("named map deserializes");
        assert_eq!(decoded, map);
    }
}
