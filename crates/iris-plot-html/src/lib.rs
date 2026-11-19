//! Standalone interactive HTML for borrowed Iris series and scalar fields.

mod backend;
mod error;
mod plot;

pub use backend::HtmlPlotBackend;
pub use error::PlotError;
pub use plot::{ColorScale, MarkerStyle, Plot};
