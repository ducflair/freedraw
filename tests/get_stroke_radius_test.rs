use freedraw::get_stroke_radius;

#[test]
fn test_stroke_radius_with_zero_thinning() {
    // When thinning is zero, the radius should be half the size
    assert_eq!(get_stroke_radius(100.0, 0.0, 0.0, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, 0.0, 0.25, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, 0.0, 0.5, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, 0.0, 0.75, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, 0.0, 1.0, None), 50.0);
}

#[test]
fn test_stroke_radius_with_positive_thinning() {
    // With 0.5 thinning, it should scale between 25% and 75%
    assert_eq!(get_stroke_radius(100.0, 0.5, 0.0, None), 25.0);
    assert_eq!(get_stroke_radius(100.0, 0.5, 0.25, None), 37.5);
    assert_eq!(get_stroke_radius(100.0, 0.5, 0.5, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, 0.5, 0.75, None), 62.5);
    assert_eq!(get_stroke_radius(100.0, 0.5, 1.0, None), 75.0);

    // With 1.0 thinning, it should scale between 0% and 100%
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.0, None), 0.0);
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.25, None), 25.0);
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.5, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.75, None), 75.0);
    assert_eq!(get_stroke_radius(100.0, 1.0, 1.0, None), 100.0);
}

#[test]
fn test_stroke_radius_with_negative_thinning() {
    // With -0.5 thinning, it should scale between 75% and 25%
    assert_eq!(get_stroke_radius(100.0, -0.5, 0.0, None), 75.0);
    assert_eq!(get_stroke_radius(100.0, -0.5, 0.25, None), 62.5);
    assert_eq!(get_stroke_radius(100.0, -0.5, 0.5, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, -0.5, 0.75, None), 37.5);
    assert_eq!(get_stroke_radius(100.0, -0.5, 1.0, None), 25.0);

    // With -1.0 thinning, it should scale between 100% and 0%
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.0, None), 100.0);
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.25, None), 75.0);
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.5, None), 50.0);
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.75, None), 25.0);
    assert_eq!(get_stroke_radius(100.0, -1.0, 1.0, None), 0.0);
}

#[test]
fn test_stroke_radius_with_easing() {
    // With exponential easing, values should be squared
    let square_easing = |t: f64| t * t;

    // With 1.0 thinning and exponential easing
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.0, Some(square_easing)), 0.0);
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.25, Some(square_easing)), 6.25);
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.5, Some(square_easing)), 25.0);
    assert_eq!(get_stroke_radius(100.0, 1.0, 0.75, Some(square_easing)), 56.25);
    assert_eq!(get_stroke_radius(100.0, 1.0, 1.0, Some(square_easing)), 100.0);

    // With -1.0 thinning and exponential easing
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.0, Some(square_easing)), 100.0);
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.25, Some(square_easing)), 56.25);
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.5, Some(square_easing)), 25.0);
    assert_eq!(get_stroke_radius(100.0, -1.0, 0.75, Some(square_easing)), 6.25);
    assert_eq!(get_stroke_radius(100.0, -1.0, 1.0, Some(square_easing)), 0.0);
} 