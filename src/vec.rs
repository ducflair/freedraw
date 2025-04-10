/// Negate a vector.
pub fn neg(a: [f64; 2]) -> [f64; 2] {
    [-a[0], -a[1]]
}

/// Add vectors.
pub fn add(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

/// Subtract vectors.
pub fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

/// Vector multiplication by scalar
pub fn mul(a: [f64; 2], n: f64) -> [f64; 2] {
    [a[0] * n, a[1] * n]
}

/// Vector division by scalar.
pub fn div(a: [f64; 2], n: f64) -> [f64; 2] {
    [a[0] / n, a[1] / n]
}

/// Perpendicular rotation of a vector A
pub fn per(a: [f64; 2]) -> [f64; 2] {
    [a[1], -a[0]]
}

/// Dot product
pub fn dpr(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

/// Get whether two vectors are equal.
pub fn is_equal(a: [f64; 2], b: [f64; 2]) -> bool {
    a[0] == b[0] && a[1] == b[1]
}

/// Length of the vector
pub fn len(a: [f64; 2]) -> f64 {
    (a[0].powi(2) + a[1].powi(2)).sqrt()
}

/// Length of the vector squared
pub fn len2(a: [f64; 2]) -> f64 {
    a[0] * a[0] + a[1] * a[1]
}

/// Dist length from A to B squared.
pub fn dist2(a: [f64; 2], b: [f64; 2]) -> f64 {
    len2(sub(a, b))
}

/// Get normalized / unit vector.
pub fn uni(a: [f64; 2]) -> [f64; 2] {
    div(a, len(a))
}

/// Dist length from A to B
pub fn dist(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((a[1] - b[1]).powi(2) + (a[0] - b[0]).powi(2)).sqrt()
}

/// Mean between two vectors or mid vector between two vectors
#[allow(dead_code)]
pub fn med(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    mul(add(a, b), 0.5)
}

/// Rotate a vector around another vector by r (radians)
pub fn rot_around(a: [f64; 2], c: [f64; 2], r: f64) -> [f64; 2] {
    let s = r.sin();
    let c_val = r.cos();

    let px = a[0] - c[0];
    let py = a[1] - c[1];

    let nx = px * c_val - py * s;
    let ny = px * s + py * c_val;

    [nx + c[0], ny + c[1]]
}

/// Interpolate vector A to B with a scalar t
pub fn lrp(a: [f64; 2], b: [f64; 2], t: f64) -> [f64; 2] {
    add(a, mul(sub(b, a), t))
}

/// Project a point A in the direction B by a scalar c
#[allow(dead_code)]
pub fn prj(a: [f64; 2], b: [f64; 2], c: f64) -> [f64; 2] {
    add(a, mul(b, c))
} 