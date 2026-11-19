use crate::PlotError;
use iris::{
    color::{NamedColorMap, Rgba},
    view::{ScalarFieldView, SeriesView},
};

mod encoding;
pub(crate) use encoding::PlotPayload;

/// An Iris color law and the physical quantity shown by its legend.
#[derive(Debug, Clone, Copy)]
pub struct ColorScale<'a> {
    /// Physical quantity and units.
    pub label: &'a str,
    /// Normalized Iris color law.
    pub map: NamedColorMap,
}

/// Solid or value-scaled coloring for a spatial point cloud.
#[derive(Debug, Clone, Copy)]
pub enum MarkerStyle<'a> {
    /// Draw one color at every point.
    Solid {
        /// Normalized display color.
        color: Rgba,
        /// Marker radius in display pixels.
        radius: u8,
    },
    /// Compute the marker color from a scalar at each point.
    Scaled {
        /// One value per point.
        values: &'a [f64],
        /// Color law and caption.
        scale: ColorScale<'a>,
        /// Marker radius in display pixels.
        radius: u8,
    },
}

/// A plot borrowing its labels and data. Browser coordinates use binary64
/// values, so the encoder accepts the same floating-point representation.
#[derive(Debug)]
pub struct Plot<'a> {
    title: &'a str,
    kind: PlotKind<'a>,
}

#[derive(Debug)]
enum PlotKind<'a> {
    Lines {
        labels: [&'a str; 2],
        series: &'a [SeriesView<'a, f64, f64>],
    },
    Heatmap {
        labels: [&'a str; 2],
        x: &'a [f64],
        y: &'a [f64],
        field: ScalarFieldView<'a, f64, 2>,
        scale: ColorScale<'a>,
    },
    Scatter {
        labels: [&'a str; 3],
        points: &'a [[f64; 3]],
        style: MarkerStyle<'a>,
    },
}

impl<'a> Plot<'a> {
    /// Borrow paired line series with horizontal and vertical axis labels.
    #[must_use]
    pub const fn lines(
        title: &'a str,
        labels: [&'a str; 2],
        series: &'a [SeriesView<'a, f64, f64>],
    ) -> Self {
        Self {
            title,
            kind: PlotKind::Lines { labels, series },
        }
    }

    /// Borrow a row-major field with shape [vertical samples, horizontal samples].
    ///
    /// # Errors
    ///
    /// Returns an error for malformed axes or a field with different extents.
    pub fn heatmap(
        title: &'a str,
        labels: [&'a str; 2],
        x: &'a [f64],
        y: &'a [f64],
        field: ScalarFieldView<'a, f64, 2>,
        scale: ColorScale<'a>,
    ) -> Result<Self, PlotError> {
        if field.extents() != &[y.len(), x.len()] {
            return Err(PlotError::ShapeMismatch);
        }
        if !valid_axis(x) || !valid_axis(y) {
            return Err(PlotError::InvalidAxis);
        }

        Ok(Self {
            title,
            kind: PlotKind::Heatmap {
                labels,
                x,
                y,
                field,
                scale,
            },
        })
    }

    /// Borrow three-dimensional points with optional scalar coloring.
    ///
    /// # Errors
    ///
    /// Returns an error for a zero marker radius or mismatched scalar data.
    pub fn scatter(
        title: &'a str,
        labels: [&'a str; 3],
        points: &'a [[f64; 3]],
        style: MarkerStyle<'a>,
    ) -> Result<Self, PlotError> {
        let valid = match style {
            MarkerStyle::Solid { radius, .. } => radius > 0,
            MarkerStyle::Scaled { radius, values, .. } => {
                radius > 0 && values.len() == points.len()
            }
        };
        if !valid {
            return Err(PlotError::InvalidMarkers);
        }

        Ok(Self {
            title,
            kind: PlotKind::Scatter {
                labels,
                points,
                style,
            },
        })
    }
}

fn valid_axis(axis: &[f64]) -> bool {
    !axis.is_empty()
        && axis.iter().all(|value| value.is_finite())
        && axis.windows(2).all(|pair| pair[0] < pair[1])
}
