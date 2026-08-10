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

/// Nine-control-point approximation of the inferno palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Inferno;

impl ColorMap for Inferno {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 9] = [
            (0.0, 0.001),
            (0.125, 0.100),
            (0.25, 0.276),
            (0.375, 0.478),
            (0.5, 0.659),
            (0.625, 0.821),
            (0.75, 0.937),
            (0.875, 0.988),
            (1.0, 0.988),
        ];
        const GREEN: [(f32, f32); 9] = [
            (0.0, 0.0),
            (0.125, 0.031),
            (0.25, 0.044),
            (0.375, 0.066),
            (0.5, 0.137),
            (0.625, 0.268),
            (0.75, 0.449),
            (0.875, 0.653),
            (1.0, 0.880),
        ];
        const BLUE: [(f32, f32); 9] = [
            (0.0, 0.014),
            (0.125, 0.184),
            (0.25, 0.397),
            (0.375, 0.467),
            (0.5, 0.432),
            (0.625, 0.326),
            (0.75, 0.208),
            (0.875, 0.118),
            (1.0, 0.381),
        ];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}

/// Nine-control-point approximation of the magma palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Magma;

impl ColorMap for Magma {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 9] = [
            (0.0, 0.001),
            (0.125, 0.118),
            (0.25, 0.304),
            (0.375, 0.504),
            (0.5, 0.689),
            (0.625, 0.857),
            (0.75, 0.974),
            (0.875, 0.998),
            (1.0, 0.987),
        ];
        const GREEN: [(f32, f32); 9] = [
            (0.0, 0.0),
            (0.125, 0.051),
            (0.25, 0.080),
            (0.375, 0.119),
            (0.5, 0.196),
            (0.625, 0.328),
            (0.75, 0.524),
            (0.875, 0.730),
            (1.0, 0.914),
        ];
        const BLUE: [(f32, f32); 9] = [
            (0.0, 0.014),
            (0.125, 0.260),
            (0.25, 0.437),
            (0.375, 0.500),
            (0.5, 0.483),
            (0.625, 0.422),
            (0.75, 0.384),
            (0.875, 0.524),
            (1.0, 0.764),
        ];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}

/// Nine-control-point approximation of the turbo palette.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Turbo;

impl ColorMap for Turbo {
    fn sample(self, value: Normalized) -> Rgba {
        const RED: [(f32, f32); 9] = [
            (0.0, 0.190),
            (0.125, 0.230),
            (0.25, 0.160),
            (0.375, 0.214),
            (0.5, 0.464),
            (0.625, 0.739),
            (0.75, 0.945),
            (0.875, 0.990),
            (1.0, 0.879),
        ];
        const GREEN: [(f32, f32); 9] = [
            (0.0, 0.073),
            (0.125, 0.318),
            (0.25, 0.519),
            (0.375, 0.682),
            (0.5, 0.801),
            (0.625, 0.872),
            (0.75, 0.869),
            (0.875, 0.683),
            (1.0, 0.314),
        ];
        const BLUE: [(f32, f32); 9] = [
            (0.0, 0.022),
            (0.125, 0.545),
            (0.25, 0.698),
            (0.375, 0.634),
            (0.5, 0.455),
            (0.625, 0.260),
            (0.75, 0.168),
            (0.875, 0.085),
            (1.0, 0.065),
        ];

        let value = value.get();
        Rgba::opaque([
            piecewise(value, &RED),
            piecewise(value, &GREEN),
            piecewise(value, &BLUE),
        ])
    }
}
