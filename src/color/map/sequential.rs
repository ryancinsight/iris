//! Analytic sequential and cyclic maps.

use super::ColorMap;
use crate::color::{Normalized, Rgba};

/// Monotone grayscale map.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Grayscale;

impl ColorMap for Grayscale {
    fn sample(self, value: Normalized) -> Rgba {
        let value = value.get();
        Rgba::opaque([value; 3])
    }
}

/// Monotone white-to-black grayscale map.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Inverted;

impl ColorMap for Inverted {
    fn sample(self, value: Normalized) -> Rgba {
        let value = 1.0 - value.get();
        Rgba::opaque([value; 3])
    }
}

/// Black-red-yellow-white sequential map.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hot;

impl ColorMap for Hot {
    fn sample(self, value: Normalized) -> Rgba {
        let value = value.get();
        Rgba::opaque([
            (3.0 * value).clamp(0.0, 1.0),
            (3.0 * value - 1.0).clamp(0.0, 1.0),
            (3.0 * value - 2.0).clamp(0.0, 1.0),
        ])
    }
}

/// Cyan-to-magenta sequential map.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cool;

impl ColorMap for Cool {
    fn sample(self, value: Normalized) -> Rgba {
        let value = value.get();
        Rgba::opaque([value, 1.0 - value, 1.0])
    }
}

/// Blue-to-red HSV hue sweep.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rainbow;

impl ColorMap for Rainbow {
    fn sample(self, value: Normalized) -> Rgba {
        let hue = 240.0 * (1.0 - value.get());
        let scaled = hue / 60.0;
        let rgb = if scaled < 1.0 {
            [1.0, scaled, 0.0]
        } else if scaled < 2.0 {
            let fraction = scaled - 1.0;
            [1.0 - fraction, 1.0, 0.0]
        } else if scaled < 3.0 {
            let fraction = scaled - 2.0;
            [0.0, 1.0, fraction]
        } else if scaled < 4.0 {
            let fraction = scaled - 3.0;
            [0.0, 1.0 - fraction, 1.0]
        } else if scaled < 5.0 {
            let fraction = scaled - 4.0;
            [fraction, 0.0, 1.0]
        } else {
            let fraction = scaled - 5.0;
            [1.0, 0.0, 1.0 - fraction]
        };
        Rgba::opaque(rgb)
    }
}
