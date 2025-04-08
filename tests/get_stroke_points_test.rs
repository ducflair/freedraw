use freedraw::{get_stroke_points, InputPoint, StrokeOptions};

#[test]
fn test_get_stroke_points_with_empty_array() {
    let points: Vec<InputPoint> = vec![];
    let options = StrokeOptions::default();
    
    let result = get_stroke_points(&points, &options);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_stroke_points_with_one_point() {
    let points = vec![InputPoint::Array([100.0, 100.0], Some(0.5))];
    let options = StrokeOptions::default();
    
    let result = get_stroke_points(&points, &options);
    assert!(result.len() >= 1);
    
    // First point should match the input
    assert_eq!(result[0].point, [100.0, 100.0]);
    assert_eq!(result[0].pressure, 0.5);
    assert_eq!(result[0].running_length, 0.0);
}

#[test]
fn test_get_stroke_points_with_multiple_points() {
    let points = vec![
        InputPoint::Array([100.0, 100.0], Some(0.5)),
        InputPoint::Array([200.0, 150.0], Some(0.7)),
        InputPoint::Array([300.0, 100.0], Some(0.5)),
    ];
    
    let options = StrokeOptions::default();
    let result = get_stroke_points(&points, &options);
    
    // There should be at least one point for each input point
    assert!(result.len() >= 3);
    
    // First point should match the input
    assert_eq!(result[0].point, [100.0, 100.0]);
    assert_eq!(result[0].pressure, 0.5);
    assert_eq!(result[0].running_length, 0.0);
    
    // Last point should have a non-zero running length
    assert!(result.last().unwrap().running_length > 0.0);
}

#[test]
fn test_get_stroke_points_with_object_input() {
    let points = vec![
        InputPoint::Struct { x: 100.0, y: 100.0, pressure: Some(0.5) },
        InputPoint::Struct { x: 200.0, y: 150.0, pressure: Some(0.7) },
        InputPoint::Struct { x: 300.0, y: 100.0, pressure: Some(0.5) },
    ];
    
    let options = StrokeOptions::default();
    let result = get_stroke_points(&points, &options);
    
    // There should be at least one point for each input point
    assert!(result.len() >= 3);
    
    // First point should match the input
    assert_eq!(result[0].point, [100.0, 100.0]);
    assert_eq!(result[0].pressure, 0.5);
    assert_eq!(result[0].running_length, 0.0);
}

#[test]
fn test_get_stroke_points_with_simulated_pressure() {
    let points = vec![
        InputPoint::Array([100.0, 100.0], None),
        InputPoint::Array([200.0, 150.0], None),
        InputPoint::Array([300.0, 100.0], None),
    ];
    
    let options = StrokeOptions {
        simulate_pressure: Some(true),
        ..Default::default()
    };
    
    let result = get_stroke_points(&points, &options);
    
    // There should be at least one point for each input point
    assert!(result.len() >= 3);
    
    // Points should have default pressure values
    for point in &result {
        assert!(point.pressure >= 0.0);
        assert!(point.pressure <= 1.0);
    }
} 