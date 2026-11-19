use std::{error::Error, fmt};

/// An invalid plot input or an error while encoding the HTML document.
#[derive(Debug)]
pub enum PlotError {
    /// The heatmap axes and scalar-field extents differ.
    ShapeMismatch,
    /// A heatmap axis is empty, nonfinite, or not strictly increasing.
    InvalidAxis,
    /// A scaled marker needs one value per point and a positive radius.
    InvalidMarkers,
    /// An output could not be serialized as JSON.
    Serialization(serde_json::Error),
}

impl fmt::Display for PlotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShapeMismatch => formatter.write_str("heatmap axes do not match the field"),
            Self::InvalidAxis => formatter.write_str("heatmap axes must be finite and increasing"),
            Self::InvalidMarkers => formatter.write_str("invalid scatter marker radius or values"),
            Self::Serialization(error) => error.fmt(formatter),
        }
    }
}

impl Error for PlotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for PlotError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}
