//! Normalized color values, color-map strategies, and lookup tables.
//!
//! Every map is a function `c: [0, 1] -> [0, 1]^4`. The validated
//! [`Normalized`] input makes the function domain explicit. Piecewise-linear
//! maps linearly interpolate adjacent control points, so each output channel
//! remains inside the convex hull of its bracketing channel values. Since all
//! control points lie in `[0, 1]`, interpolation preserves the RGBA invariant.

mod lookup_table;
pub mod map;
mod model;
mod normalized;

pub use lookup_table::LookupTable;
pub use map::{ColorMap, NamedColorMap};
pub use model::Rgba;
pub use normalized::Normalized;
