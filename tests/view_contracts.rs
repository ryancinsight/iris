//! Contract tests for borrowed views and lending render backends.

use iris::{
    IrisError,
    color::{LookupTable, Normalized, map::Grayscale},
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
fn scalar_field_accepts_empty_zero_extent_and_preserves_storage() {
    let backing = [7_u16];
    let values = &backing[..0];
    let field = ScalarFieldView::new(values, [0, 3]).expect("zero extent has zero cardinality");

    assert_eq!(field.extents(), &[0, 3]);
    assert!(core::ptr::eq(field.as_slice(), values));
    assert_eq!(field.values().len(), 0);
    assert_eq!(ScalarField::shape(&field), &[0, 3]);
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

struct ScalarColorBackend<const N: usize> {
    colors: LookupTable<Grayscale, N>,
    frame: Vec<u8>,
}

impl<'view, const N: usize> RenderBackend<ScalarFieldView<'view, f32, 2>>
    for ScalarColorBackend<N>
{
    type Error = IrisError;
    type Frame<'frame>
        = &'frame [u8]
    where
        Self: 'frame;

    fn render<'frame>(
        &'frame mut self,
        view: &ScalarFieldView<'view, f32, 2>,
    ) -> Result<Self::Frame<'frame>, Self::Error> {
        self.frame.clear();
        for value in view.values().copied() {
            let normalized = Normalized::new(value)?;
            self.frame
                .extend_from_slice(&self.colors.sample(normalized).to_rgba8());
        }
        Ok(self.frame.as_slice())
    }
}

#[test]
fn render_backend_maps_scalar_view_and_reuses_rgba_storage() {
    let first_values = [0.0_f32, 0.5, 1.0, 0.25];
    let first = ScalarFieldView::new(&first_values, [2, 2]).expect("shape cardinality matches");
    let second_values = [1.0_f32, 0.0, 0.75, 0.25];
    let second = ScalarFieldView::new(&second_values, [2, 2]).expect("shape cardinality matches");
    let mut backend = ScalarColorBackend::<5> {
        colors: LookupTable::<Grayscale, 5>::from_map(Grayscale),
        frame: Vec::with_capacity(first_values.len() * 4),
    };

    let first_pointer = {
        let first_frame = backend.render(&first).expect("normalized field values");
        assert_eq!(
            first_frame,
            &[
                0, 0, 0, 255, 128, 128, 128, 255, 255, 255, 255, 255, 64, 64, 64, 255
            ]
        );
        first_frame.as_ptr()
    };

    let second_frame = backend.render(&second).expect("normalized field values");
    assert_eq!(
        second_frame,
        &[
            255, 255, 255, 255, 0, 0, 0, 255, 191, 191, 191, 255, 64, 64, 64, 255
        ]
    );
    assert_eq!(second_frame.as_ptr(), first_pointer);
}
