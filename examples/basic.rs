use freedraw::{get_stroke, InputPoint, StrokeOptions};

fn main() {
    // Create some example points forming a simple curve
    let points = vec![
        InputPoint::Array([100.0, 100.0], Some(0.5)),
        InputPoint::Array([120.0, 110.0], Some(0.6)),
        InputPoint::Array([150.0, 120.0], Some(0.7)),
        InputPoint::Array([200.0, 130.0], Some(0.8)),
        InputPoint::Array([250.0, 120.0], Some(0.7)),
        InputPoint::Array([280.0, 110.0], Some(0.6)),
        InputPoint::Array([300.0, 100.0], Some(0.5)),
    ];

    // Create options with custom settings
    let options = StrokeOptions {
        size: Some(20.0),
        thinning: Some(0.5),
        smoothing: Some(0.5),
        streamline: Some(0.5),
        simulate_pressure: Some(false),
        start: Some(freedraw::TaperOptions {
            taper: Some(freedraw::TaperType::Bool(true)),
            ..Default::default()
        }),
        end: Some(freedraw::TaperOptions {
            taper: Some(freedraw::TaperType::Bool(true)),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Generate the stroke outline
    let outline = get_stroke(&points, &options);

    // Print the resulting outline points
    println!("Stroke outline points ({} points):", outline.len());
    for (i, point) in outline.iter().enumerate() {
        println!("Point {}: [{:.1}, {:.1}]", i, point[0], point[1]);
    }
} 