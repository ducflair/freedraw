use crate::get_stroke_outline_points::get_stroke_outline_points;
use crate::get_stroke_points::get_stroke_points;
use crate::types::{InputPoint, StrokeOptions};

/// Get an array of points describing a polygon that surrounds the input points.
///
/// # Arguments
/// * `points` - An array of points (with optional pressure data)
/// * `options` - Options for the stroke generation
///
/// # Returns
/// An array of points (as `[x, y]`) that define the outline of the stroke
pub fn get_stroke(
    points: &[InputPoint],
    options: &StrokeOptions,
) -> Vec<[f64; 2]> {
    let stroke_points = get_stroke_points(points, options);
    get_stroke_outline_points(&stroke_points, options)
} 