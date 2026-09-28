//! Closed-set runtime color-map selection.

use super::{
    BlueRed, Bone, ColorMap, Cool, CoolWarm, Grayscale, Hot, Inferno, Inverted, Jet, Magma, Plasma,
    Rainbow, Turbo, Viridis,
};
use crate::color::{Normalized, Rgba};

/// Define the closed named-map set with its exhaustive dispatchers.
///
/// `display_order` is the published `ALL` order, which is deliberately not the
/// declaration order. The entries are the declaration order: it pins the ADR
/// 0002 implicit discriminants, so `BlueRed` stays last. Each entry is
/// `(docs, Variant, label, strategy)`.
macro_rules! named_color_maps {
    (
        display_order: [$($display:ident),+ $(,)?];
        $(
            $(#[doc = $doc:literal])+
            $variant:ident => $label:literal, $strategy:ident,
        )+
    ) => {
        /// Built-in normalized color laws with sRGB-encoded RGB output.
        ///
        /// All variants use normalized sRGB-encoded RGB channels and normalized
        /// linear opacity. Their interpolation is not linear-light.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[non_exhaustive]
        pub enum NamedColorMap {
            $(
                $(#[doc = $doc])+
                $variant,
            )+
        }

        impl NamedColorMap {
            /// Built-in maps in stable display order.
            pub const ALL: [Self; 14] = [
                $( Self::$display, )+
            ];

            /// Return the human-readable map name.
            #[must_use]
            pub const fn label(self) -> &'static str {
                match self {
                    $( Self::$variant => $label, )+
                }
            }
        }

        impl ColorMap for NamedColorMap {
            fn sample(self, value: Normalized) -> Rgba {
                match self {
                    $( Self::$variant => $strategy.sample(value), )+
                }
            }
        }

        #[cfg(test)]
        const fn variant_index(map: NamedColorMap) -> usize {
            // The declaration-order discriminant is distinct per variant, so
            // the coverage test needs no hand-written slot list here.
            map as usize
        }
    };
}

named_color_maps! {
    display_order: [
        BlueRed, Grayscale, Inverted, Hot, Cool, Bone, Jet, Plasma, Viridis, Inferno, Magma, Turbo,
        CoolWarm, Rainbow,
    ];

    /// Monotone black-to-white grayscale in normalized sRGB-encoded channels.
    Grayscale => "Grayscale", Grayscale,
    /// Monotone white-to-black grayscale in normalized sRGB-encoded channels.
    Inverted => "Inverted", Inverted,
    /// Black-red-yellow-white sequential map in normalized sRGB-encoded channels.
    Hot => "Hot", Hot,
    /// Cyan-to-magenta sequential map in normalized sRGB-encoded channels.
    Cool => "Cool", Cool,
    /// Gray-blue sequential map in normalized sRGB-encoded channels.
    Bone => "Bone", Bone,
    /// Blue-cyan-green-yellow-red map in normalized sRGB-encoded channels.
    Jet => "Jet", Jet,
    /// Purple-orange-yellow sequential map in normalized sRGB-encoded channels.
    Plasma => "Plasma", Plasma,
    /// Perceptually ordered purple-green-yellow sequential map with normalized
    /// sRGB-encoded channels.
    Viridis => "Viridis", Viridis,
    /// Piecewise-linear blue-white-red diverging map in normalized sRGB-encoded
    /// channels.
    CoolWarm => "Cool-warm", CoolWarm,
    /// Blue-to-red HSV hue sweep with normalized sRGB-encoded channels.
    Rainbow => "Rainbow", Rainbow,
    /// Black-purple-red-orange-yellow sequential map in normalized sRGB-encoded
    /// channels.
    Inferno => "Inferno", Inferno,
    /// Black-purple-red-orange-white sequential map in normalized sRGB-encoded
    /// channels.
    Magma => "Magma", Magma,
    /// High-dynamic-range rainbow-like sequential map with normalized
    /// sRGB-encoded channels.
    Turbo => "Turbo", Turbo,
    /// Linear blue-to-red map with no neutral midpoint in normalized
    /// sRGB-encoded channels.
    BlueRed => "Blue-red", BlueRed,
}

#[cfg(test)]
mod tests {
    use super::{NamedColorMap, variant_index};

    #[test]
    fn all_contains_each_variant_once() {
        let mut seen = [false; NamedColorMap::ALL.len()];
        for map in NamedColorMap::ALL {
            let index = variant_index(map);
            let slot = seen
                .get_mut(index)
                .expect("every variant index fits the ALL array");
            assert!(!*slot, "NamedColorMap::ALL repeats {}", map.label());
            *slot = true;
        }

        assert!(
            seen.into_iter().all(|present| present),
            "NamedColorMap::ALL omits a variant"
        );
    }
}
