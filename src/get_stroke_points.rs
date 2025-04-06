use crate::types::{InputPoint, StrokeOptions, StrokePoint};
use crate::vec::{add, dist, is_equal, lrp, sub, uni};
use std::cmp::min;

/// Get an array of points as objects with an adjusted point, pressure, vector, distance, and running_length.
///
/// # Arguments
/// * `points` - An array of points with optional pressure data
/// * `options` - Options for the stroke generation
///
/// # Returns
/// An array of StrokePoint objects
pub fn get_stroke_points(
    points: &[InputPoint],
    options: &StrokeOptions,
) -> Vec<StrokePoint> {
    let streamline = options.streamline.unwrap_or(0.5);
    let size = options.size.unwrap_or(16.0);
    let is_complete = options.last.unwrap_or(false);

    // If we don't have any points, return an empty array
    if points.is_empty() {
        return Vec::new();
    }

    // Find the interpolation level between points
    let t = 0.15 + (1.0 - streamline) * 0.85;

    // Convert all input points to a consistent format [x, y, pressure]
    let mut pts: Vec<([f64; 2], f64)> = points
        .iter()
        .map(|p| match p {
            InputPoint::Array(point, pressure) => (*point, pressure.unwrap_or(0.5)),
            InputPoint::Struct { x, y, pressure } => ([*x, *y], pressure.unwrap_or(0.5)),
        })
        .collect();

    // Add extra points between the two, to help avoid "dash" lines
    // for strokes with tapered start and ends
    if pts.len() == 2 {
        let last = pts[1];
        pts = vec![pts[0]];
        for i in 1..5 {
            let t = i as f64 / 4.0;
            let lerp_point = lrp(pts[0].0, last.0, t);
            pts.push((lerp_point, last.1));
        }
    }

    // If there's only one point, add another point at a 1pt offset
    if pts.len() == 1 {
        let point = pts[0];
        let new_point = add(point.0, [1.0, 1.0]);
        pts.push((new_point, point.1));
    }

    // The stroke_points array will hold the points for the stroke
    // Start it out with the first point, which needs no adjustment
    let mut stroke_points = vec![StrokePoint {
        point: [pts[0].0[0], pts[0].0[1]],
        pressure: if pts[0].1 >= 0.0 { pts[0].1 } else { 0.25 },
        vector: [1.0, 1.0],
        distance: 0.0,
        running_length: 0.0,
    }];

    // A flag to see whether we've already reached out minimum length
    let mut has_reached_minimum_length = false;

    // We use the running_length to keep track of the total distance
    let mut running_length = 0.0;

    // We're set this to the latest point, so we can use it to calculate
    // the distance and vector of the next point
    let mut prev = stroke_points[0].clone();

    let max = pts.len() - 1;

    // Iterate through all of the points, creating StrokePoints
    for i in 1..pts.len() {
        let point = if is_complete && i == max {
            // If we're at the last point, and options.last is true,
            // then add the actual input point
            [pts[i].0[0], pts[i].0[1]]
        } else {
            // Otherwise, using the t calculated from the streamline
            // option, interpolate a new point between the previous
            // point the current point
            lrp(prev.point, pts[i].0, t)
        };

        // If the new point is the same as the previous point, skip ahead
        if is_equal(prev.point, point) {
            continue;
        }

        // How far is the new point from the previous point?
        let distance = dist(point, prev.point);

        // Add this distance to the total "running length" of the line
        running_length += distance;

        // At the start of the line, we wait until the new point is a
        // certain distance away from the original point, to avoid noise
        if i < max && !has_reached_minimum_length {
            if running_length < size {
                continue;
            }
            has_reached_minimum_length = true;
            // TODO: Backfill the missing points so that tapering works correctly
        }

        // Create a new strokepoint (it will be the new "previous" one)
        prev = StrokePoint {
            // The adjusted point
            point,
            // The input pressure (or .5 if not specified)
            pressure: if pts[i].1 >= 0.0 { pts[i].1 } else { 0.5 },
            // The vector from the current point to the previous point
            vector: uni(sub(prev.point, point)),
            // The distance between the current point and the previous point
            distance,
            // The total distance so far
            running_length,
        };

        // Push it to the stroke_points array
        stroke_points.push(prev.clone());
    }

    // Set the vector of the first point to be the same as the second point
    if let Some(second) = stroke_points.get(1) {
        stroke_points[0].vector = second.vector;
    }

    stroke_points
} 