//! Closed-set runtime color-map selection.

use super::{
    BlueRed, Bone, ColorMap, Cool, CoolWarm, Grayscale, Hot, Inverted, Jet, Plasma, Rainbow,
    Viridis,
};
use crate::color::{Normalized, Rgba};

/// Built-in normalized color laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum NamedColorMap {
    /// Monotone black-to-white grayscale.
    Grayscale,
    /// Monotone white-to-black grayscale.
    Inverted,
    /// Black-red-yellow-white sequential map.
    Hot,
    /// Cyan-to-magenta sequential map.
    Cool,
    /// Gray-blue sequential map.
    Bone,
    /// Blue-cyan-green-yellow-red map.
    Jet,
    /// Purple-orange-yellow sequential map.
    Plasma,
    /// Perceptually ordered purple-green-yellow sequential map.
    Viridis,
    /// Piecewise-linear blue-white-red diverging map.
    CoolWarm,
    /// Blue-to-red HSV hue sweep.
    Rainbow,
    /// Linear blue-to-red map with no neutral midpoint.
    BlueRed,
}

impl NamedColorMap {
    /// Built-in maps in stable display order.
    pub const ALL: [Self; 11] = [
        Self::BlueRed,
        Self::Grayscale,
        Self::Inverted,
        Self::Hot,
        Self::Cool,
        Self::Bone,
        Self::Jet,
        Self::Plasma,
        Self::Viridis,
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
            Self::CoolWarm => CoolWarm.sample(value),
            Self::Rainbow => Rainbow.sample(value),
        }
    }
}
