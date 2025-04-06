use crate::get_stroke_radius::get_stroke_radius;
use crate::types::{StrokeOptions, StrokePoint};
use crate::vec::{add, dist2, dpr, lrp, mul, neg, per, prj, rot_around, sub, uni};
use std::f64::consts::PI;

// This is the rate of change for simulated pressure. It could be an option.
const RATE_OF_PRESSURE_CHANGE: f64 = 0.275;

// Browser strokes seem to be off if PI is regular, a tiny offset seems to fix it
const FIXED_PI: f64 = PI + 0.0001;

/// Get an array of points (as `[x, y]`) representing the outline of a stroke.
///
/// # Arguments
/// * `points` - An array of StrokePoints as returned from `get_stroke_points`
/// * `options` - Options for the stroke generation
///
/// # Returns
/// An array of points (as `[x, y]`) that define the outline of the stroke
pub fn get_stroke_outline_points(
    points: &[StrokePoint],
    options: &StrokeOptions,
) -> Vec<[f64; 2]> {
    let size = options.size.unwrap_or(16.0);
    let smoothing = options.smoothing.unwrap_or(0.5);
    let thinning = options.thinning.unwrap_or(0.5);
    let simulate_pressure = options.simulate_pressure.unwrap_or(true);
    let is_complete = options.last.unwrap_or(false);

    // Define the easing function or use the default (identity function)
    let easing_fn = options.easing.unwrap_or(|t| t);

    // Get start and end options with defaults
    let start_options = options.start.clone().unwrap_or_default();
    let end_options = options.end.clone().unwrap_or_default();

    // Cap and taper settings
    let cap_start = start_options.cap.unwrap_or(true);
    let cap_end = end_options.cap.unwrap_or(true);

    // Taper start easing
    let taper_start_ease = start_options.easing.unwrap_or(|t| t * (2.0 - t));
    
    // Taper end easing
    let taper_end_ease = end_options.easing.unwrap_or(|t| 1.0 - (1.0 - t).powi(3));

    // We can't do anything with an empty array or a stroke with negative size
    if points.is_empty() || size <= 0.0 {
        return vec![];
    }

    // The total length of the line
    let total_length = points.last().map(|p| p.running_length).unwrap_or(0.0);

    // The minimum allowed distance between points (squared)
    let min_distance = (size * smoothing).powi(2);

    // Our collected left and right points
    let mut left_pts: Vec<[f64; 2]> = Vec::new();
    let mut right_pts: Vec<[f64; 2]> = Vec::new();

    // Previous pressure (start with average of first five pressures,
    // in order to prevent fat starts for every line. Drawn lines
    // almost always start slow!
    let mut prev_pressure = points.iter().take(10).fold(points[0].pressure, |acc, curr| {
        let mut pressure = curr.pressure;

        if simulate_pressure {
            // Speed of change - how fast should the the pressure changing?
            let sp = f64::min(1.0, curr.distance / size);
            // Rate of change - how much of a change is there?
            let rp = f64::min(1.0, 1.0 - sp);
            // Accelerate the pressure
            pressure = f64::min(1.0, acc + (rp - acc) * (sp * RATE_OF_PRESSURE_CHANGE));
        }

        (acc + pressure) / 2.0
    });

    // The current radius
    let mut radius = get_stroke_radius(
        size,
        thinning,
        points.last().map(|p| p.pressure).unwrap_or(0.5),
        Some(easing_fn),
    );

    // The radius of the first saved point
    let mut first_radius: Option<f64> = None;

    // Previous vector
    let mut prev_vector = points[0].vector;

    // Previous left and right points
    let mut pl = points[0].point;
    let mut pr = pl;

    // Temporary left and right points
    let mut tl = pl;
    let mut tr = pr;

    // Keep track of whether the previous point is a sharp corner
    // ... so that we don't detect the same corner twice
    let mut is_prev_sharp_corner = false;

    // Determine taper settings
    let taper_start = match &start_options.taper {
        Some(taper) => match taper {
            crate::types::TaperType::Bool(false) => 0.0,
            crate::types::TaperType::Bool(true) => f64::max(size, total_length),
            crate::types::TaperType::Number(value) => *value,
        },
        None => 0.0,
    };

    let taper_end = match &end_options.taper {
        Some(taper) => match taper {
            crate::types::TaperType::Bool(false) => 0.0,
            crate::types::TaperType::Bool(true) => f64::max(size, total_length),
            crate::types::TaperType::Number(value) => *value,
        },
        None => 0.0,
    };

    // Iterate through the points and generate the outline
    for (i, curr) in points.iter().enumerate() {
        // Skip the first point
        if i == 0 {
            continue;
        }

        // Get the current point and vector
        let point = curr.point;
        let vector = curr.vector;
        let distance = curr.distance;
        let running_length = curr.running_length;

        // Calculate the current pressure
        let mut pressure = curr.pressure;

        // Simulate pressure if needed
        if thinning > 0.0 && simulate_pressure {
            let sp = f64::min(1.0, distance / size);
            let rp = f64::min(1.0, 1.0 - sp);
            pressure = f64::min(
                1.0,
                prev_pressure + (rp - prev_pressure) * (sp * RATE_OF_PRESSURE_CHANGE),
            );
        }

        prev_pressure = pressure;

        // Calculate the current radius
        if thinning > 0.0 {
            radius = get_stroke_radius(size, thinning, pressure, Some(easing_fn));
        } else {
            radius = size / 2.0;
        }

        // Store the first radius value
        if first_radius.is_none() {
            first_radius = Some(radius);
        }

        // Apply tapering if needed
        let ts = if running_length < taper_start {
            taper_start_ease(running_length / taper_start)
        } else {
            1.0
        };

        let te = if total_length - running_length < taper_end {
            taper_end_ease((total_length - running_length) / taper_end)
        } else {
            1.0
        };

        radius = f64::max(0.01, radius * f64::min(ts, te));

        // Calculate the normal vector for this point
        let normal_vector = per(vector);

        // Calculate the offset points for this point
        let offset_vector = mul(normal_vector, radius);
        let left_point = add(point, offset_vector);
        let right_point = add(point, neg(offset_vector));

        // Check if we need to handle sharp corners
        let is_sharp_corner = dpr(prev_vector, vector) < 0.0;

        if is_sharp_corner && !is_prev_sharp_corner {
            // Add the last point - skip if too close to the previous point
            if dist2(left_point, pl) > min_distance {
                left_pts.push(left_point);
                pl = left_point;
            }

            if dist2(right_point, pr) > min_distance {
                right_pts.push(right_point);
                pr = right_point;
            }
        } else {
            // We're in a curve (or straight line)

            if !is_prev_sharp_corner {
                // Create the next offset point
                let prev_normal = per(prev_vector);
                let offset_a = mul(prev_normal, radius);
                
                // Calculate temporary left and right points
                tl = add(point, offset_a);
                tr = add(point, neg(offset_a));

                // Add the previous offset points
                if dist2(pl, tl) > min_distance {
                    left_pts.push(tl);
                    pl = tl;
                }

                if dist2(pr, tr) > min_distance {
                    right_pts.push(tr);
                    pr = tr;
                }
            }

            // Add the current offset points
            if dist2(pl, left_point) > min_distance {
                left_pts.push(left_point);
                pl = left_point;
            }

            if dist2(pr, right_point) > min_distance {
                right_pts.push(right_point);
                pr = right_point;
            }
        }

        // Set variables for the next iteration
        prev_vector = vector;
        is_prev_sharp_corner = is_sharp_corner;
    }

    // Add caps if needed
    let mut result = Vec::new();

    // Cap start
    if cap_start && !left_pts.is_empty() && first_radius.is_some() {
        let first_point = points[0].point;
        let first_normal = per(points[0].vector);
        let offset_vector = mul(first_normal, first_radius.unwrap());

        let start_left = add(first_point, offset_vector);
        let start_right = add(first_point, neg(offset_vector));

        // Add the first cap
        result.push(start_left);

        // Add semicircular cap
        if points.len() > 1 {
            let steps = 4;
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let angle = FIXED_PI - t * FIXED_PI;
                result.push(rot_around(start_right, first_point, angle));
            }
        } else {
            result.push(start_right);
        }
    }

    // Add left side points
    result.extend(left_pts);

    // Add right side points in reverse
    for p in right_pts.iter().rev() {
        result.push(*p);
    }

    // Cap end
    if cap_end && !right_pts.is_empty() {
        let last_point = points.last().map(|p| p.point).unwrap_or_default();
        let last_vector = points.last().map(|p| p.vector).unwrap_or_default();
        let last_normal = per(last_vector);
        
        let last_radius = if points.len() > 1 {
            let last_pressure = points.last().map(|p| p.pressure).unwrap_or(0.5);
            if thinning > 0.0 {
                get_stroke_radius(size, thinning, last_pressure, Some(easing_fn))
            } else {
                size / 2.0
            }
        } else {
            first_radius.unwrap_or(radius)
        };

        let tapered_radius = if taper_end > 0.0 {
            0.01
        } else {
            last_radius
        };

        let offset_vector = mul(last_normal, tapered_radius);
        let end_left = add(last_point, offset_vector);
        let end_right = add(last_point, neg(offset_vector));

        // Add semicircular cap
        if points.len() > 1 && cap_end {
            let steps = 4;
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                result.push(rot_around(end_left, last_point, t * FIXED_PI));
            }
        } else {
            result.push(end_right);
        }
    }

    // Close the path
    if !result.is_empty() && result.len() > 1 {
        result.push(result[0]);
    }

    result
} 