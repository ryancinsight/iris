//! Closed-set runtime color-map selection.

use super::{
    BlueRed, Bone, ColorMap, Cool, CoolWarm, Grayscale, Hot, Inferno, Inverted, Jet, Magma, Plasma,
    Rainbow, Turbo, Viridis,
};
use crate::color::{Normalized, Rgba};

/// Built-in normalized color laws with sRGB-encoded RGB output.
///
/// All variants use normalized sRGB-encoded RGB channels and normalized linear
/// opacity. Their interpolation is not linear-light.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum NamedColorMap {
    /// Monotone black-to-white grayscale in normalized sRGB-encoded channels.
    Grayscale,
    /// Monotone white-to-black grayscale in normalized sRGB-encoded channels.
    Inverted,
    /// Black-red-yellow-white sequential map in normalized sRGB-encoded channels.
    Hot,
    /// Cyan-to-magenta sequential map in normalized sRGB-encoded channels.
    Cool,
    /// Gray-blue sequential map in normalized sRGB-encoded channels.
    Bone,
    /// Blue-cyan-green-yellow-red map in normalized sRGB-encoded channels.
    Jet,
    /// Purple-orange-yellow sequential map in normalized sRGB-encoded channels.
    Plasma,
    /// Perceptually ordered purple-green-yellow sequential map with normalized
    /// sRGB-encoded channels.
    Viridis,
    /// Piecewise-linear blue-white-red diverging map in normalized sRGB-encoded
    /// channels.
    CoolWarm,
    /// Blue-to-red HSV hue sweep with normalized sRGB-encoded channels.
    Rainbow,
    /// Black-purple-red-orange-yellow sequential map in normalized sRGB-encoded
    /// channels.
    Inferno,
    /// Black-purple-red-orange-white sequential map in normalized sRGB-encoded
    /// channels.
    Magma,
    /// High-dynamic-range rainbow-like sequential map with normalized
    /// sRGB-encoded channels.
    Turbo,
    /// Linear blue-to-red map with no neutral midpoint in normalized
    /// sRGB-encoded channels.
    BlueRed,
}

impl NamedColorMap {
    /// Built-in maps in stable display order.
    pub const ALL: [Self; 14] = [
        Self::BlueRed,
        Self::Grayscale,
        Self::Inverted,
        Self::Hot,
        Self::Cool,
        Self::Bone,
        Self::Jet,
        Self::Plasma,
        Self::Viridis,
        Self::Inferno,
        Self::Magma,
        Self::Turbo,
        Self::CoolWarm,
        Self::Rainbow,
    ];

    /// Return the human-readable map name.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::BlueRed => "Blue-red",
            Self::Grayscale => "Grayscale",
            Self::Inverted => "Inverted",
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Bone => "Bone",
            Self::Jet => "Jet",
            Self::Plasma => "Plasma",
            Self::Viridis => "Viridis",
            Self::Inferno => "Inferno",
            Self::Magma => "Magma",
            Self::Turbo => "Turbo",
            Self::CoolWarm => "Cool-warm",
            Self::Rainbow => "Rainbow",
        }
    }
}

impl ColorMap for NamedColorMap {
    fn sample(self, value: Normalized) -> Rgba {
        match self {
            Self::BlueRed => BlueRed.sample(value),
            Self::Grayscale => Grayscale.sample(value),
            Self::Inverted => Inverted.sample(value),
            Self::Hot => Hot.sample(value),
            Self::Cool => Cool.sample(value),
            Self::Bone => Bone.sample(value),
            Self::Jet => Jet.sample(value),
            Self::Plasma => Plasma.sample(value),
            Self::Viridis => Viridis.sample(value),
            Self::Inferno => Inferno.sample(value),
            Self::Magma => Magma.sample(value),
            Self::Turbo => Turbo.sample(value),
            Self::CoolWarm => CoolWarm.sample(value),
            Self::Rainbow => Rainbow.sample(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NamedColorMap;

    const fn variant_index(map: NamedColorMap) -> usize {
        match map {
            NamedColorMap::BlueRed => 0,
            NamedColorMap::Grayscale => 1,
            NamedColorMap::Inverted => 2,
            NamedColorMap::Hot => 3,
            NamedColorMap::Cool => 4,
            NamedColorMap::Bone => 5,
            NamedColorMap::Jet => 6,
            NamedColorMap::Plasma => 7,
            NamedColorMap::Viridis => 8,
            NamedColorMap::CoolWarm => 9,
            NamedColorMap::Rainbow => 10,
            NamedColorMap::Inferno => 11,
            NamedColorMap::Magma => 12,
            NamedColorMap::Turbo => 13,
        }
    }

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
