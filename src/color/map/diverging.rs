//! Diverging maps.

use super::{ColorMap, interpolation::linear_rgb};
use crate::color::{Normalized, Rgba};

/// Linear blue-to-red map with no neutral midpoint.
///
/// For `t` in `[0, 1]`, this map is defined by
/// `c(t) = (t, 0, 1 - t, 1)`. Each channel therefore remains normalized, the
/// red channel is monotone non-decreasing, and the blue channel is monotone
/// non-increasing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BlueRed;

impl ColorMap for BlueRed {
    fn sample(self, value: Normalized) -> Rgba {
        let value = value.get();
        Rgba::opaque([value, 0.0, 1.0 - value])
    }
}

/// Piecewise-linear blue-white-red diverging map.
///
/// This is an RGB interpolation contract. It does not claim the perceptual Msh
/// interpolation of Moreland's full cool-to-warm algorithm.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CoolWarm;

impl ColorMap for CoolWarm {
    fn sample(self, value: Normalized) -> Rgba {
        const COOL: [f32; 3] = [0.23, 0.30, 0.75];
        const WHITE: [f32; 3] = [1.0; 3];
        const WARM: [f32; 3] = [0.71, 0.016, 0.15];

        let value = value.get();
        let rgb = if value <= 0.5 {
            linear_rgb(COOL, WHITE, value * 2.0)
        } else {
            linear_rgb(WHITE, WARM, (value - 0.5) * 2.0)
        };
        Rgba::opaque(rgb)
    }
}
