//! Contract tests for borrowed views and lending render backends.

use iris::{
    IrisError,
    render::RenderBackend,
    view::{Axis, ScalarField, ScalarFieldView, SeriesView},
};

#[test]
fn series_view_borrows_inputs_and_pairs_values() {
    let coordinates = [0.0_f32, 0.5, 1.0];
    let samples = [2.0_f64, 3.0, 5.0];
    let series = SeriesView::new("pressure", &coordinates, &samples).expect("cardinalities match");

    assert!(core::ptr::eq(series.coordinates(), &coordinates));
    assert!(core::ptr::eq(series.samples(), &samples));
    assert_eq!(
        series.iter().collect::<Vec<_>>(),
        vec![(&0.0, &2.0), (&0.5, &3.0), (&1.0, &5.0)]
    );
}

#[test]
fn series_view_reports_both_cardinalities() {
    let error = SeriesView::new("invalid", &[0_u8, 1], &[4_u8]).expect_err("lengths differ");
    assert_eq!(
        error,
        IrisError::SeriesLengthMismatch {
            coordinates: 2,
            samples: 1,
        }
    );
}

#[test]
fn scalar_field_preserves_shape_and_lends_values() {
    let values = [1_u16, 2, 3, 4, 5, 6];
    let field = ScalarFieldView::new(&values, [2, 3]).expect("shape cardinality matches");

    assert_eq!(field.extents(), &[2, 3]);
    assert!(core::ptr::eq(field.as_slice(), &values));
    assert_eq!(field.values().copied().sum::<u16>(), 21);
    assert_eq!(ScalarField::shape(&field), &[2, 3]);
}

#[test]
fn scalar_field_rejects_cardinality_mismatch() {
    let error = ScalarFieldView::new(&[1_u8, 2, 3], [2, 2]).expect_err("shape needs four values");
    assert_eq!(
        error,
        IrisError::ShapeCardinalityMismatch {
            expected: 4,
            actual: 3,
        }
    );
}

#[test]
fn axis_metadata_can_borrow_or_own() {
    let borrowed = Axis::with_unit("time", "s");
    let owned = Axis::unitless(String::from("iteration"));
    assert_eq!(borrowed.label(), "time");
    assert_eq!(borrowed.unit(), Some("s"));
    assert_eq!(owned.label(), "iteration");
    assert_eq!(owned.unit(), None);
}

struct ByteFrameBackend {
    frame: [u8; 4],
}

impl RenderBackend<[u8]> for ByteFrameBackend {
    type Error = core::convert::Infallible;
    type Frame<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn render<'a>(&'a mut self, view: &[u8]) -> Result<Self::Frame<'a>, Self::Error> {
        self.frame.copy_from_slice(view);
        Ok(&self.frame)
    }
}

#[test]
fn render_backend_lends_reused_frame_storage() {
    let mut backend = ByteFrameBackend { frame: [0; 4] };
    let frame = backend
        .render(&[3, 1, 4, 1])
        .expect("backend is infallible");
    assert_eq!(frame, &[3, 1, 4, 1]);
}
