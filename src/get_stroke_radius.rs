/// Compute a radius based on the pressure.
///
/// # Arguments
/// * `size` - The base size of the stroke
/// * `thinning` - The effect of pressure on the stroke's size
/// * `pressure` - The pressure (between 0 and 1)
/// * `easing` - An optional easing function to apply to the pressure
pub fn get_stroke_radius(
    size: f64,
    thinning: f64,
    pressure: f64,
    easing: Option<fn(f64) -> f64>,
) -> f64 {
    let t = match easing {
        Some(ease_fn) => ease_fn(0.5 - thinning * (0.5 - pressure)),
        None => 0.5 - thinning * (0.5 - pressure),
    };
    
    size * t
} 