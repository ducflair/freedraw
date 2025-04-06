/// The options object for `get_stroke` or `get_stroke_points`.
///
/// * `size` - The base size (diameter) of the stroke.
/// * `thinning` - The effect of pressure on the stroke's size.
/// * `smoothing` - How much to soften the stroke's edges.
/// * `easing` - An easing function to apply to each point's pressure.
/// * `simulate_pressure` - Whether to simulate pressure based on velocity.
/// * `start` - Cap, taper and easing for the start of the line.
/// * `end` - Cap, taper and easing for the end of the line.
/// * `last` - Whether to handle the points as a completed stroke.
#[derive(Clone)]
pub struct StrokeOptions {
    pub size: Option<f64>,
    pub thinning: Option<f64>,
    pub smoothing: Option<f64>,
    pub streamline: Option<f64>,
    pub easing: Option<fn(f64) -> f64>,
    pub simulate_pressure: Option<bool>,
    pub start: Option<TaperOptions>,
    pub end: Option<TaperOptions>,
    pub last: Option<bool>,
}

impl Default for StrokeOptions {
    fn default() -> Self {
        Self {
            size: None,
            thinning: None,
            smoothing: None,
            streamline: None,
            easing: None,
            simulate_pressure: None,
            start: None,
            end: None,
            last: None,
        }
    }
}

/// Options for tapering at the start or end of a stroke
#[derive(Clone)]
pub struct TaperOptions {
    pub cap: Option<bool>,
    pub taper: Option<TaperType>,
    pub easing: Option<fn(f64) -> f64>,
}

impl Default for TaperOptions {
    fn default() -> Self {
        Self {
            cap: None,
            taper: None,
            easing: None,
        }
    }
}

/// Represents either a boolean or a numeric taper value
#[derive(Debug, Clone)]
pub enum TaperType {
    Bool(bool),
    Number(f64),
}

impl std::fmt::Debug for StrokeOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrokeOptions")
            .field("size", &self.size)
            .field("thinning", &self.thinning)
            .field("smoothing", &self.smoothing)
            .field("streamline", &self.streamline)
            .field("easing", &if self.easing.is_some() { "Fn" } else { "None" })
            .field("simulate_pressure", &self.simulate_pressure)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("last", &self.last)
            .finish()
    }
}

impl std::fmt::Debug for TaperOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaperOptions")
            .field("cap", &self.cap)
            .field("taper", &self.taper)
            .field("easing", &if self.easing.is_some() { "Fn" } else { "None" })
            .finish()
    }
}

/// The points returned by `get_stroke_points`, and the input for `get_stroke_outline_points`.
#[derive(Debug, Clone)]
pub struct StrokePoint {
    pub point: [f64; 2],
    pub pressure: f64,
    pub distance: f64,
    pub vector: [f64; 2],
    pub running_length: f64,
}

/// Represents an input point with optional pressure
#[derive(Debug, Clone)]
pub enum InputPoint {
    Array([f64; 2], Option<f64>),
    Struct { x: f64, y: f64, pressure: Option<f64> },
} 