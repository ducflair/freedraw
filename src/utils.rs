/// Calculates the average of two numbers
fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

/// Converts a stroke's outline points to an SVG path data string
///
/// # Arguments
/// * `points` - The outline points returned by `get_stroke`
/// * `closed` - Whether to close the path with a 'Z' command
///
/// # Returns
/// A string containing SVG path commands
pub fn get_svg_path_from_stroke(points: &[[f64; 2]], closed: bool) -> String {
    let len = points.len();

    if len < 4 {
        return String::new();
    }

    let a = points[0];
    let b = points[1];
    let c = points[2];

    // Format with exactly 2 decimal places for consistent output
    let mut result = format!(
        "M{:.2},{:.2} Q{:.2},{:.2} {:.2},{:.2} T",
        a[0],
        a[1],
        b[0],
        b[1],
        average(b[0], c[0]),
        average(b[1], c[1])
    );

    for i in 2..(len - 1) {
        let a = points[i];
        let b = points[i + 1];
        result.push_str(&format!(
            "{:.2},{:.2} ",
            average(a[0], b[0]),
            average(a[1], b[1])
        ));
    }

    if closed {
        result.push('Z');
    }

    result
} 