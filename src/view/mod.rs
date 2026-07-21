//! Zero-copy domain-result views.

mod axis;
mod scalar_field;
mod series;

pub use axis::Axis;
pub use scalar_field::{ScalarField, ScalarFieldView};
pub use series::SeriesView;
