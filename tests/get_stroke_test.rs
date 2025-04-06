use freedraw::{get_stroke, get_svg_path_from_stroke, InputPoint, StrokeOptions};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

fn load_test_data() -> serde_json::Value {
    let mut file = File::open("tests/inputs.json").expect("Could not open input.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    serde_json::from_str(&contents).expect("Could not parse JSON")
}

fn convert_json_to_input_points(points_json: &[Value]) -> Vec<InputPoint> {
    points_json
        .iter()
        .map(|point| {
            if point.is_array() {
                let coords = point.as_array().unwrap();
                if coords.len() >= 3 {
                    InputPoint::Array(
                        [
                            coords[0].as_f64().unwrap(),
                            coords[1].as_f64().unwrap(),
                        ],
                        Some(coords[2].as_f64().unwrap()),
                    )
                } else {
                    InputPoint::Array(
                        [
                            coords[0].as_f64().unwrap(),
                            coords[1].as_f64().unwrap(),
                        ],
                        None,
                    )
                }
            } else {
                let obj = point.as_object().unwrap();
                InputPoint::Struct {
                    x: obj["x"].as_f64().unwrap(),
                    y: obj["y"].as_f64().unwrap(),
                    pressure: obj.get("pressure").and_then(|p| p.as_f64()),
                }
            }
        })
        .collect()
}

#[test]
fn test_get_stroke_with_default_options() {
    let test_data = load_test_data();
    
    // Test with number pairs instead of simple curve
    let points = test_data["numberPairs"].as_array().unwrap();
    let input_points = convert_json_to_input_points(points);
    
    let options = StrokeOptions::default();
    let result = get_stroke(&input_points, &options);
    
    // Ensure we got some results
    assert!(!result.is_empty());
    
    // Ensure the first and last points form a closed path
    if result.len() > 1 {
        assert_eq!(result[0], result[result.len() - 1]);
    }
}

#[test]
fn test_get_stroke_with_custom_options() {
    let test_data = load_test_data();
    
    // Test with object pairs instead of pressure curve
    let points = test_data["objectPairs"].as_array().unwrap();
    let input_points = convert_json_to_input_points(points);
    
    let options = StrokeOptions {
        size: Some(20.0),
        thinning: Some(0.5),
        smoothing: Some(0.5),
        streamline: Some(0.5),
        simulate_pressure: Some(false),
        ..Default::default()
    };
    
    let result = get_stroke(&input_points, &options);
    
    // Ensure we got some results
    assert!(!result.is_empty());
    
    // Ensure the first and last points form a closed path
    if result.len() > 1 {
        assert_eq!(result[0], result[result.len() - 1]);
    }
}

#[test]
fn test_get_stroke_edge_cases() {
    let test_data = load_test_data();
    
    // Test with one point
    let one_point = test_data["onePoint"].as_array().unwrap();
    let input_points = convert_json_to_input_points(one_point);
    let result = get_stroke(&input_points, &StrokeOptions::default());
    assert!(!result.is_empty());
    
    // Test with two points
    let two_points = test_data["twoPoints"].as_array().unwrap();
    let input_points = convert_json_to_input_points(two_points);
    let result = get_stroke(&input_points, &StrokeOptions::default());
    assert!(!result.is_empty());
    
    // Test with empty array
    let result = get_stroke(&[], &StrokeOptions::default());
    assert_eq!(result.len(), 0);
}

#[test]
fn test_svg_path_conversion() {
    let test_data = load_test_data();
    
    // Test with many points instead of simple curve
    let points = test_data["manyPoints"].as_array().unwrap();
    let input_points = convert_json_to_input_points(points);
    
    let options = StrokeOptions {
        size: Some(20.0),
        thinning: Some(0.5),
        smoothing: Some(0.5),
        streamline: Some(0.5),
        ..Default::default()
    };
    
    let stroke = get_stroke(&input_points, &options);
    
    // Convert the stroke to an SVG path
    let path_data = get_svg_path_from_stroke(&stroke, true);
    
    // Ensure the path is not empty
    assert!(!path_data.is_empty());
    
    // The path should start with M and contain Q and T commands
    assert!(path_data.starts_with('M'));
    assert!(path_data.contains('Q'));
    assert!(path_data.contains('T'));
    
    // Should end with Z for closed path
    assert!(path_data.ends_with('Z'));
    
    // Print the SVG path for demonstration
    println!("SVG Path for Many Points: {}", path_data);
    
    // Test with number pairs instead of pressure curve
    let points = test_data["numberPairs"].as_array().unwrap();
    let input_points = convert_json_to_input_points(points);
    
    let stroke = get_stroke(&input_points, &options);
    let path_data = get_svg_path_from_stroke(&stroke, true);
    
    // Print the SVG path for demonstration
    println!("SVG Path for Number Pairs: {}", path_data);
} 