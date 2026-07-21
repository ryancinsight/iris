//! Table-defined piecewise-linear maps.

use super::{ColorMap, interpolation::piecewise};
use crate::color::{Normalized, Rgba};

/// Gray-blue sequential map approximating the standard bone palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Bone;

impl ColorMap for Bone {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 3] = [(0.0, 0.0), (0.746_03, 0.652_78), (1.0, 1.0)];
        const GREEN: [(f32, f32); 4] = [
            (0.0, 0.0),
            (0.365_08, 0.319_44),
            (0.746_03, 0.777_78),
            (1.0, 1.0),
        ];
        const BLUE: [(f32, f32); 3] = [(0.0, 0.0), (0.365_08, 0.444_44), (1.0, 1.0)];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}

/// Blue-cyan-green-yellow-red map approximating the standard jet palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Jet;

impl ColorMap for Jet {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 5] = [
            (0.0, 0.0),
            (0.35, 0.0),
            (0.66, 1.0),
            (0.89, 1.0),
            (1.0, 0.5),
        ];
        const GREEN: [(f32, f32); 6] = [
            (0.0, 0.0),
            (0.125, 0.0),
            (0.375, 1.0),
            (0.64, 1.0),
            (0.91, 0.0),
            (1.0, 0.0),
        ];
        const BLUE: [(f32, f32); 5] = [
            (0.0, 0.5),
            (0.11, 1.0),
            (0.34, 1.0),
            (0.65, 0.0),
            (1.0, 0.0),
        ];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}

/// Five-control-point approximation of the plasma palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Plasma;

impl ColorMap for Plasma {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 5] = [
            (0.0, 0.050),
            (0.25, 0.250),
            (0.5, 0.800),
            (0.75, 0.960),
            (1.0, 0.940),
        ];
        const GREEN: [(f32, f32); 5] = [
            (0.0, 0.030),
            (0.25, 0.010),
            (0.5, 0.130),
            (0.75, 0.520),
            (1.0, 0.975),
        ];
        const BLUE: [(f32, f32); 5] = [
            (0.0, 0.530),
            (0.25, 0.830),
            (0.5, 0.550),
            (0.75, 0.160),
            (1.0, 0.130),
        ];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}

/// Five-control-point approximation of the viridis palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Viridis;

impl ColorMap for Viridis {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 5] = [
            (0.0, 0.267),
            (0.25, 0.128),
            (0.5, 0.204),
            (0.75, 0.632),
            (1.0, 0.993),
        ];
        const GREEN: [(f32, f32); 5] = [
            (0.0, 0.005),
            (0.25, 0.407),
            (0.5, 0.636),
            (0.75, 0.829),
            (1.0, 0.906),
        ];
        const BLUE: [(f32, f32); 5] = [
            (0.0, 0.329),
            (0.25, 0.549),
            (0.5, 0.469),
            (0.75, 0.195),
            (1.0, 0.144),
        ];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}
