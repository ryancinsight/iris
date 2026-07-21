//! Piecewise-linear interpolation shared by table-defined maps.

pub(super) fn linear(a: f32, b: f32, fraction: f32) -> f32 {
    a + fraction * (b - a)
}

pub(super) fn linear_rgb(a: [f32; 3], b: [f32; 3], fraction: f32) -> [f32; 3] {
    [
        linear(a[0], b[0], fraction),
        linear(a[1], b[1], fraction),
        linear(a[2], b[2], fraction),
    ]
}

pub(super) fn piecewise<const N: usize>(value: f32, points: &[(f32, f32); N]) -> f32 {
    const { assert!(N > 0, "piecewise maps require at least one point") };

    if value <= points[0].0 {
        return points[0].1;
    }
    if value >= points[N - 1].0 {
        return points[N - 1].1;
    }

    let mut lower = 0;
    let mut upper = N - 1;
    while upper - lower > 1 {
        let middle = usize::midpoint(lower, upper);
        if value < points[middle].0 {
            upper = middle;
        } else {
            lower = middle;
        }
    }

    let (x0, y0) = points[lower];
    let (x1, y1) = points[upper];
    linear(y0, y1, (value - x0) / (x1 - x0))
}
