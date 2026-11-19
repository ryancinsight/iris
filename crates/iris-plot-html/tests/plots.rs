//! Value-level checks against the encoded browser data.

use iris::{
    color::{ColorMap, NamedColorMap, Normalized, Rgba},
    render::RenderBackend,
    view::{ScalarFieldView, SeriesView},
};
use iris_plot_html::{ColorScale, HtmlPlotBackend, MarkerStyle, Plot, PlotError};
use serde_json::{Value, json};
use tracing_subscriber::fmt::format::FmtSpan;

fn trace() -> tracing::subscriber::DefaultGuard {
    tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_span_events(FmtSpan::FULL)
            .with_test_writer()
            .finish(),
    )
}

fn encoded(frame: &[u8]) -> (&str, Value) {
    let html = std::str::from_utf8(frame).expect("HTML document is UTF-8");
    let (_, encoded) = html.split_once("id=\"data\">").expect("data script");
    let (data, _) = encoded.split_once("</script>").expect("data script closes");
    (data.trim(), serde_json::from_str(data).expect("plot JSON"))
}

#[test]
fn lines_retain_samples_labels_and_track_missing_values() {
    let _trace = trace();
    let samples = tracing::info_span!("arrange").in_scope(|| [2.0, f64::NAN, -3.0]);
    let name = "A </script><script>unexpected</script>";
    let views = [SeriesView::new(name, &[0.5, 1.0, 2.0], &samples).expect("paired series")];
    let plot = Plot::lines(name, ["Time (s)", "Pressure (Pa)"], &views);
    let mut renderer = HtmlPlotBackend::default();

    let (data, parsed) = tracing::info_span!("act")
        .in_scope(|| encoded(renderer.render(&plot).expect("standalone HTML")));
    let _assert = tracing::info_span!("assert").entered();
    assert!(!data.contains('<'));
    assert_eq!(
        parsed,
        json!({
            "kind": "lines",
            "title": name,
            "labels": ["Time (s)", "Pressure (Pa)"],
            "series": [{"name": name, "x": [0.5, 1.0, 2.0], "y": [2.0, null, -3.0]}]
        })
    );
}

#[test]
fn heatmap_uses_row_major_values_and_the_iris_palette() {
    let _trace = trace();
    let (x, y, values) = tracing::info_span!("arrange").in_scope(|| {
        (
            [1.0, 2.5],
            [-3.0, -2.0, 5.0],
            [1.0, 2.0, 4.0, 7.0, 11.0, 16.0],
        )
    });
    let field = ScalarFieldView::new(&values, [3, 2]).expect("valid field");
    let scale = ColorScale {
        label: "Pressure (Pa)",
        map: NamedColorMap::Viridis,
    };
    let mut renderer = HtmlPlotBackend::default();
    let (parsed, shape_failure, axis_failure) = tracing::info_span!("act").in_scope(|| {
        let plot =
            Plot::heatmap("slice", ["X (m)", "Y (m)"], &x, &y, field, scale).expect("valid axes");
        let (_, parsed) = encoded(renderer.render(&plot).expect("rendered plot"));
        (
            parsed,
            Plot::heatmap("slice", ["X", "Y"], &y, &x, field, scale),
            Plot::heatmap("slice", ["X", "Y"], &[2.0, 1.0], &y, field, scale),
        )
    });
    let _assert = tracing::info_span!("assert").entered();
    assert_eq!(parsed["kind"], json!("heatmap"));
    assert_eq!(parsed["x"], json!(x));
    assert_eq!(parsed["y"], json!(y));
    assert_eq!(parsed["values"], json!(values));
    assert_eq!(parsed["scale"]["label"], json!("Pressure (Pa)"));
    for channel in [0, 137, 255] {
        let expected = scale.map.sample(Normalized::from_u8(channel)).to_rgba8();
        assert_eq!(
            parsed["scale"]["colors"][usize::from(channel)],
            json!(expected)
        );
    }
    assert!(matches!(shape_failure, Err(PlotError::ShapeMismatch)));
    assert!(matches!(axis_failure, Err(PlotError::InvalidAxis)));
}

#[test]
fn scatter_data_and_color_values_are_preserved_on_frame_reuse() {
    let _trace = trace();
    let points = tracing::info_span!("arrange").in_scope(|| [[0.0, 0.5, -1.0], [5.0, -0.75, 0.0]]);
    let style = MarkerStyle::Scaled {
        values: &[4.0, -1.0],
        scale: ColorScale {
            label: "Scalar",
            map: NamedColorMap::CoolWarm,
        },
        radius: 3,
    };
    let mut renderer = HtmlPlotBackend::default();
    let (first, second, missing_value) = tracing::info_span!("act").in_scope(|| {
        let colored =
            Plot::scatter("colored", ["x", "y", "z"], &points, style).expect("valid markers");
        let first = encoded(renderer.render(&colored).expect("colored plot")).1;
        let solid = MarkerStyle::Solid {
            color: Rgba::new([1.0, 0.0, 0.0, 1.0]).expect("red"),
            radius: 4,
        };
        let plain = Plot::scatter("plain", ["x", "y", "z"], &points, solid).expect("valid markers");
        let second = encoded(renderer.render(&plain).expect("plain plot")).1;
        let missing = Plot::scatter("bad", ["x", "y", "z"], &points[..1], style);
        (first, second, missing)
    });
    let _assert = tracing::info_span!("assert").entered();
    assert_eq!(first["points"], json!(points));
    assert_eq!(first["markers"]["values"], json!([4.0, -1.0]));
    assert_eq!(second["title"], json!("plain"));
    assert_eq!(
        second["markers"],
        json!({"style":"solid","color":[255,0,0,255],"radius":4})
    );
    assert!(matches!(missing_value, Err(PlotError::InvalidMarkers)));
}
