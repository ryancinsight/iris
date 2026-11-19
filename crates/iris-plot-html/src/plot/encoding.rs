use iris::{
    color::{ColorMap, Normalized},
    view::SeriesView,
};
use serde::{Serialize, Serializer, ser::SerializeSeq};

use super::{ColorScale, MarkerStyle, Plot, PlotKind};

#[derive(Serialize)]
pub(crate) struct PlotPayload<'a> {
    title: &'a str,
    #[serde(flatten)]
    data: Payload<'a>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Payload<'a> {
    Lines {
        labels: [&'a str; 2],
        series: Series<'a>,
    },
    Heatmap {
        labels: [&'a str; 2],
        x: &'a [f64],
        y: &'a [f64],
        values: &'a [f64],
        scale: Scale<'a>,
    },
    Scatter {
        labels: [&'a str; 3],
        points: &'a [[f64; 3]],
        markers: Markers<'a>,
    },
}

impl<'a> From<&Plot<'a>> for PlotPayload<'a> {
    fn from(plot: &Plot<'a>) -> Self {
        let data = match plot.kind {
            PlotKind::Lines { labels, series } => Payload::Lines {
                labels,
                series: Series(series),
            },
            PlotKind::Heatmap {
                labels,
                x,
                y,
                field,
                scale,
            } => Payload::Heatmap {
                labels,
                x,
                y,
                values: field.as_slice(),
                scale: scale.into(),
            },
            PlotKind::Scatter {
                labels,
                points,
                style,
            } => Payload::Scatter {
                labels,
                points,
                markers: style.into(),
            },
        };
        Self {
            title: plot.title,
            data,
        }
    }
}

struct Series<'a>(&'a [SeriesView<'a, f64, f64>]);

impl Serialize for Series<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Line<'a> {
            name: &'a str,
            x: &'a [f64],
            y: &'a [f64],
        }

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for view in self.0 {
            sequence.serialize_element(&Line {
                name: view.name(),
                x: view.coordinates(),
                y: view.samples(),
            })?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
struct Scale<'a> {
    label: &'a str,
    colors: Vec<[u8; 4]>,
}

impl<'a> From<ColorScale<'a>> for Scale<'a> {
    fn from(scale: ColorScale<'a>) -> Self {
        Self {
            label: scale.label,
            colors: (u8::MIN..=u8::MAX)
                .map(|channel| scale.map.sample(Normalized::from_u8(channel)).to_rgba8())
                .collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "style", rename_all = "snake_case")]
enum Markers<'a> {
    Solid {
        color: [u8; 4],
        radius: u8,
    },
    Scaled {
        values: &'a [f64],
        scale: Scale<'a>,
        radius: u8,
    },
}

impl<'a> From<MarkerStyle<'a>> for Markers<'a> {
    fn from(style: MarkerStyle<'a>) -> Self {
        match style {
            MarkerStyle::Solid { color, radius } => Self::Solid {
                color: color.to_rgba8(),
                radius,
            },
            MarkerStyle::Scaled {
                values,
                scale,
                radius,
            } => Self::Scaled {
                values,
                scale: scale.into(),
                radius,
            },
        }
    }
}
