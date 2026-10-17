//! Empty-domain cardinality must not depend on axis ordering.

use iris::{
    IrisError,
    view::{ScalarField, ScalarFieldView},
};

fn assert_empty<const D: usize>(shape: [usize; D]) {
    let backing = [7_u16];
    let empty = &backing[..0];
    let field = ScalarFieldView::new(empty, shape).expect("a zero factor gives zero cardinality");
    assert_eq!(field.extents(), &shape);
    assert_eq!(ScalarField::shape(&field), &shape);
    assert!(core::ptr::eq(field.as_slice(), empty));
    let mut values = field.values();
    assert_eq!(values.len(), 0);
    assert_eq!(values.next(), None);
}

#[test]
fn empty_zero_last_survives_max_then_two_prefix() {
    assert_empty([usize::MAX, 2, 0]);
}

#[test]
fn empty_zero_last_survives_two_then_max_prefix() {
    assert_empty([2, usize::MAX, 0]);
}

#[test]
fn zero_last_nonempty_reports_zero_cardinality_mismatch() {
    assert_eq!(
        ScalarFieldView::new(&[9_u8], [usize::MAX, 2, 0]),
        Err(IrisError::ShapeCardinalityMismatch {
            expected: 0,
            actual: 1,
        })
    );
}

#[test]
fn zero_first_accepts_the_same_empty_extent_multiset() {
    assert_empty([0, usize::MAX, 2]);
    assert_empty([0, 2, usize::MAX]);
}

#[test]
fn zero_middle_accepts_the_same_empty_extent_multiset() {
    assert_empty([usize::MAX, 0, 2]);
    assert_empty([2, 0, usize::MAX]);
}

#[test]
fn small_zero_extent_permutations_and_rank_one_stay_empty() {
    assert_empty([0, 2, 3]);
    assert_empty([2, 0, 3]);
    assert_empty([2, 3, 0]);
    assert_empty([0]);
}

#[test]
fn nonzero_extent_products_still_report_overflow() {
    for shape in [[usize::MAX, 2], [2, usize::MAX]] {
        assert_eq!(
            ScalarFieldView::new(&[] as &[u8], shape),
            Err(IrisError::ShapeCardinalityOverflow)
        );
    }
}

#[test]
fn representable_large_product_reports_exact_mismatch() {
    assert_eq!(
        ScalarFieldView::new(&[] as &[u8], [usize::MAX, 1]),
        Err(IrisError::ShapeCardinalityMismatch {
            expected: usize::MAX,
            actual: 0,
        })
    );
    assert_eq!(
        ScalarFieldView::new(&[9_u8], [0, usize::MAX, 2]),
        Err(IrisError::ShapeCardinalityMismatch {
            expected: 0,
            actual: 1,
        })
    );
}

#[test]
fn ordinary_field_retains_values_shape_and_borrow_identity() {
    let values = [1_u16, 2, 3, 4, 5, 6];
    let field = ScalarFieldView::new(&values, [2, 3]).expect("six stored values");
    assert_eq!(field.extents(), &[2, 3]);
    assert!(core::ptr::eq(field.as_slice(), &values));
    assert_eq!(field.values().copied().sum::<u16>(), 21);
}
