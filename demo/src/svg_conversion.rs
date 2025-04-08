use freedraw::{get_stroke, get_svg_path_from_stroke, InputPoint, StrokeOptions, TaperOptions, TaperType};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use serde_json::Value;

fn load_test_data(filename: &str) -> serde_json::Value {
    let mut file = File::open(format!("../tests/{}", filename)).expect(&format!("Could not open {}", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    serde_json::from_str(&contents).expect("Could not parse JSON")
}

// New function to calculate viewBox based on path data
fn calculate_viewbox(points: &[InputPoint]) -> String {
    if points.is_empty() {
        return "0 0 100 100".to_string(); // Default viewBox
    }
    
    // Initialize min/max values
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    
    // Find the bounds of all points
    for point in points {
        let (x, y) = match point {
            InputPoint::Array([x, y], _) => (*x, *y),
            InputPoint::Struct { x, y, .. } => (*x, *y),
        };
        
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    
    // Add padding (20% of the size)
    let width = max_x - min_x;
    let height = max_y - min_y;
    let padding_x = width * 0.2;
    let padding_y = height * 0.2;
    
    // Ensure minimum size for very small drawings
    let width = width.max(10.0) + padding_x * 2.0;
    let height = height.max(10.0) + padding_y * 2.0;
    
    // Return the viewBox string
    format!("{} {} {} {}", 
        min_x - padding_x, 
        min_y - padding_y, 
        width, 
        height
    )
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

fn generate_svg_file(path_data: &str, filename: &str, color: &str, input_points: &[InputPoint]) {
    // Create the dist directory if it doesn't exist
    let dist_dir = Path::new("examples/svg");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir).expect("Failed to create dist directory");
    }

    let file_path = dist_dir.join(filename);
    
    // Calculate viewBox based on input points
    let viewbox = calculate_viewbox(input_points);
    
    let svg_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}" preserveAspectRatio="xMidYMid meet">
  <path d="{}" 
        fill="{}" 
        stroke="none" 
        stroke-width="0"
        stroke-linejoin="round" 
        stroke-linecap="round" />
</svg>"#,
        viewbox, path_data, color
    );
    
    let mut file = File::create(file_path).expect("Failed to create SVG file");
    file.write_all(svg_content.as_bytes()).expect("Failed to write SVG content");
    
    println!("Generated SVG file: examples/svg/{}", filename);
}

// Also create a function to generate both fill and stroke versions for comparison
fn generate_svg_file_with_stroke(path_data: &str, filename: &str, fill_color: &str, stroke_color: &str, stroke_width: f64, input_points: &[InputPoint]) {
    // Create the dist directory if it doesn't exist
    let dist_dir = Path::new("examples/svg");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir).expect("Failed to create dist directory");
    }

    let file_path = dist_dir.join(filename);
    
    // Calculate viewBox based on input points
    let viewbox = calculate_viewbox(input_points);
    
    let svg_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}" preserveAspectRatio="xMidYMid meet">
  <path d="{}" 
        fill="{}" 
        stroke="{}" 
        stroke-width="{}"
        stroke-linejoin="round" 
        stroke-linecap="round" />
</svg>"#,
        viewbox, path_data, fill_color, stroke_color, stroke_width
    );
    
    let mut file = File::create(file_path).expect("Failed to create SVG file");
    file.write_all(svg_content.as_bytes()).expect("Failed to write SVG content");
    
    println!("Generated SVG file: examples/svg/{}", filename);
}

// Process a raw points array from sample.json, flash.json, etc.
fn process_raw_points(name: &str, points: &[Value], color: &str) {
    // Convert points to InputPoints
    let input_points: Vec<InputPoint> = points
        .iter()
        .map(|point| {
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
        })
        .collect();
    
    // Generate SVGs with different options
    let options_variations = [
        // name suffix, thinning, size, smoothing, streamline, simulate_pressure
        ("_default", 0.5, 16.0, 0.5, 0.5, false),
        ("_thin", 0.75, 10.0, 0.5, 0.5, false),
        ("_thick", 0.25, 20.0, 0.5, 0.5, false),
        ("_simulated", 0.5, 16.0, 0.5, 0.5, true),
    ];
    
    for (suffix, thinning, size, smoothing, streamline, simulate_pressure) in options_variations {
        // Basic options
        let options = StrokeOptions {
            size: Some(size),
            thinning: Some(thinning),
            smoothing: Some(smoothing),
            streamline: Some(streamline),
            simulate_pressure: Some(simulate_pressure),
            ..Default::default()
        };
        
        let stroke = get_stroke(&input_points, &options);
        let path_data = get_svg_path_from_stroke(&stroke, true);
        
        // Generate both fill and stroke versions
        let filename = format!("{}{}",  name, suffix);
        generate_svg_file(&path_data, &format!("{}.svg", filename), color, &input_points);
        generate_svg_file_with_stroke(
            &path_data, 
            &format!("{}_stroke.svg", filename), 
            "transparent", 
            color, 
            2.0, 
            &input_points
        );
        
        // Add tapered versions
        let tapered_options = StrokeOptions {
            size: Some(size),
            thinning: Some(thinning),
            smoothing: Some(smoothing),
            streamline: Some(streamline),
            simulate_pressure: Some(simulate_pressure),
            start: Some(TaperOptions {
                taper: Some(TaperType::Bool(true)),
                ..Default::default()
            }),
            end: Some(TaperOptions {
                taper: Some(TaperType::Bool(true)),
                ..Default::default()
            }),
            ..Default::default()
        };
        
        let stroke = get_stroke(&input_points, &tapered_options);
        let path_data = get_svg_path_from_stroke(&stroke, true);
        
        let filename = format!("{}{}_tapered", name, suffix);
        generate_svg_file(&path_data, &format!("{}.svg", filename), color, &input_points);
        generate_svg_file_with_stroke(
            &path_data, 
            &format!("{}_stroke.svg", filename), 
            "transparent", 
            color, 
            2.0, 
            &input_points
        );
    }
}

pub fn main() {
    // Create the dist directory if it doesn't exist
    let dist_dir = Path::new("examples/svg");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir).expect("Failed to create dist directory");
    }
    
    println!("Generating SVG examples from test data...");
    
    // --- Process inputs.json examples ---
    let test_data = load_test_data("inputs.json");
    
    // Define a list of examples to process with their configurations
    let examples = [
        // Name, key in test data, color, thinning, size, smoothing, streamline, simulate_pressure
        ("manyPoints", "manyPoints", "#3498db", 0.5, 20.0, 0.5, 0.5, false),
        ("numberPairs", "numberPairs", "#e74c3c", 0.5, 15.0, 0.5, 0.5, false),
        ("objectPairs", "objectPairs", "#2ecc71", 0.2, 15.0, 0.8, 0.6, true),
        ("withDuplicates", "withDuplicates", "#9b59b6", -0.3, 25.0, 0.6, 0.4, true),
        ("onePoint", "onePoint", "#f39c12", 0.5, 10.0, 0.5, 0.5, true),
        ("twoPoints", "twoPoints", "#16a085", 0.5, 15.0, 0.5, 0.5, true),
        ("twoEqualPoints", "twoEqualPoints", "#d35400", 0.5, 10.0, 0.5, 0.5, true),
    ];
    
    // Helper function to apply different options for tapered and non-tapered variants
    let generate_variants = |name: &str, points: &[Value], color: &str, thinning: f64, size: f64, 
                            smoothing: f64, streamline: f64, simulate_pressure: bool| {
        let input_points = convert_json_to_input_points(points);
        
        // Basic options (no tapers)
        let options = StrokeOptions {
            size: Some(size),
            thinning: Some(thinning),
            smoothing: Some(smoothing),
            streamline: Some(streamline),
            simulate_pressure: Some(simulate_pressure),
            ..Default::default()
        };
        
        let stroke = get_stroke(&input_points, &options);
        let path_data = get_svg_path_from_stroke(&stroke, true);
        
        // Generate both fill-only and stroke+fill versions for comparison
        generate_svg_file(&path_data, &format!("{}.svg", name), color, &input_points);
        generate_svg_file_with_stroke(
            &path_data, 
            &format!("{}_with_stroke.svg", name), 
            "transparent", 
            color, 
            2.0, 
            &input_points
        );
        
        // Tapered options (start and end tapers)
        if input_points.len() > 1 {
            let tapered_options = StrokeOptions {
                size: Some(size),
                thinning: Some(thinning),
                smoothing: Some(smoothing),
                streamline: Some(streamline),
                simulate_pressure: Some(simulate_pressure),
                start: Some(TaperOptions {
                    taper: Some(TaperType::Bool(true)),
                    ..Default::default()
                }),
                end: Some(TaperOptions {
                    taper: Some(TaperType::Bool(true)),
                    ..Default::default()
                }),
                ..Default::default()
            };
            
            let stroke = get_stroke(&input_points, &tapered_options);
            let path_data = get_svg_path_from_stroke(&stroke, true);
            
            // Generate both fill-only and stroke+fill versions for comparison
            generate_svg_file(&path_data, &format!("{}_tapered.svg", name), color, &input_points);
            generate_svg_file_with_stroke(
                &path_data, 
                &format!("{}_tapered_with_stroke.svg", name), 
                "transparent", 
                color, 
                2.0, 
                &input_points
            );
        }
    };
    
    // Process each example in the list
    for (name, key, color, thinning, size, smoothing, streamline, simulate_pressure) in examples {
        if let Some(points) = test_data[key].as_array() {
            generate_variants(
                name, 
                points, 
                color, 
                thinning, 
                size, 
                smoothing, 
                streamline, 
                simulate_pressure
            );
        }
    }
    
    // --- Process sample.json ---
    println!("\nProcessing sample.json...");
    let sample_data = load_test_data("sample.json");
    if let Value::Array(points) = sample_data {
        process_raw_points("sample", &points, "#1abc9c");
    }
    
    // --- Process flash.json ---
    println!("\nProcessing flash.json...");
    let flash_data = load_test_data("flash.json");
    if let Value::Array(points) = flash_data {
        process_raw_points("flash", &points, "#8e44ad");
    }
    
    // --- Process corners.json ---
    println!("\nProcessing corners.json...");
    let corners_data = load_test_data("corners.json");
    if let Some(corners) = corners_data["corners"].as_array() {
        process_raw_points("corners", corners, "#e67e22");
    }
    
    println!("\nAll SVG files generated in the examples/svg directory.");
} 