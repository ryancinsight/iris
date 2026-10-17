//! Logical empty-domain cardinality is invariant under axis permutations.

use iris::{
    IrisError,
    view::{ScalarField, ScalarFieldView},
};

fn permutations(mut visit: impl FnMut([usize; 4]), input: [usize; 4]) {
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    if a != b && a != c && a != d && b != c && b != d && c != d {
                        visit([input[a], input[b], input[c], input[d]]);
                    }
                }
            }
        }
    }
}

fn assert_empty(shape: [usize; 4]) {
    let backing = [7_u16];
    let values = &backing[..0];
    let field = ScalarFieldView::new(values, shape).expect("a zero extent makes an empty domain");
    assert_eq!(field.extents(), &shape);
    assert_eq!(ScalarField::shape(&field), &shape);
    assert!(core::ptr::eq(field.as_slice(), values));
    assert_eq!(field.values().len(), 0);
    assert_eq!(field.values().next(), None);
}

#[test]
fn every_rank_four_axis_order_accepts_the_empty_domain() {
    let mut count = 0;
    permutations(
        |shape| {
            assert_empty(shape);
            count += 1;
        },
        [usize::MAX, 2, 0, 1],
    );
    assert_eq!(count, 24);
}

#[test]
fn every_rank_four_axis_order_rejects_nonempty_storage_with_zero_expected() {
    permutations(
        |shape| {
            assert_eq!(
                ScalarFieldView::new(&[9_u8], shape),
                Err(IrisError::ShapeCardinalityMismatch {
                    expected: 0,
                    actual: 1,
                })
            );
        },
        [usize::MAX, 2, 0, 1],
    );
}

#[test]
fn multiple_zero_extents_accept_each_axis_order() {
    permutations(assert_empty, [usize::MAX, 2, 0, 0]);
}

#[test]
fn every_nonzero_overflow_order_still_reports_overflow() {
    // MAX is odd: 2 * (MAX / 2 + 1) is exactly MAX + 1.
    let large = usize::MAX / 2 + 1;
    permutations(
        |shape| {
            assert_eq!(
                ScalarFieldView::<u8, 4>::new(&[], shape),
                Err(IrisError::ShapeCardinalityOverflow)
            );
        },
        [large, 2, 1, 1],
    );
}

#[test]
fn every_representable_nonzero_order_keeps_exact_mismatch_cardinality() {
    // 2 * floor(MAX / 2) is exactly MAX - 1 and fits in usize.
    permutations(
        |shape| {
            assert_eq!(
                ScalarFieldView::<u8, 4>::new(&[], shape),
                Err(IrisError::ShapeCardinalityMismatch {
                    expected: usize::MAX - 1,
                    actual: 0,
                })
            );
        },
        [usize::MAX / 2, 2, 1, 1],
    );
}

#[test]
fn zero_sized_scalar_storage_still_has_logical_cardinality() {
    let backing = [(); 3];
    for shape in [[0, usize::MAX, 2], [usize::MAX, 0, 2]] {
        assert_eq!(
            ScalarFieldView::new(&backing, shape),
            Err(IrisError::ShapeCardinalityMismatch {
                expected: 0,
                actual: 3,
            })
        );
        let empty = &backing[..0];
        let field = ScalarFieldView::new(empty, shape).expect("zero logical values");
        assert!(core::ptr::eq(field.as_slice(), empty));
        assert_eq!(field.values().len(), 0);
    }
}
