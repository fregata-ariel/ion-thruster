//! 3D vector type and operations.
//!
//! Uses `[f64; 3]` for SIMD-friendly contiguous layout.

/// A 3D vector stored as `[f64; 3]` for cache-friendly, SIMD-ready layout.
pub type Vec3 = [f64; 3];

/// Vec3 operations as free functions to avoid orphan rule issues.
/// All operations are `#[inline]` for auto-vectorization.
pub mod ops {
    use super::Vec3;

    pub const ZERO: Vec3 = [0.0, 0.0, 0.0];

    #[inline]
    pub fn add(a: Vec3, b: Vec3) -> Vec3 {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }

    #[inline]
    pub fn sub(a: Vec3, b: Vec3) -> Vec3 {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    #[inline]
    pub fn scale(a: Vec3, s: f64) -> Vec3 {
        [a[0] * s, a[1] * s, a[2] * s]
    }

    #[inline]
    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    #[inline]
    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    #[inline]
    pub fn magnitude_squared(a: Vec3) -> f64 {
        dot(a, a)
    }

    #[inline]
    pub fn magnitude(a: Vec3) -> f64 {
        magnitude_squared(a).sqrt()
    }

    #[inline]
    pub fn normalize(a: Vec3) -> Vec3 {
        let mag = magnitude(a);
        debug_assert!(mag > 0.0, "Cannot normalize zero vector");
        scale(a, 1.0 / mag)
    }

    #[inline]
    pub fn distance(a: Vec3, b: Vec3) -> f64 {
        magnitude(sub(a, b))
    }

    #[inline]
    pub fn distance_squared(a: Vec3, b: Vec3) -> f64 {
        magnitude_squared(sub(a, b))
    }

    /// Accumulate: `target += src`
    #[inline]
    pub fn add_assign(target: &mut Vec3, src: Vec3) {
        target[0] += src[0];
        target[1] += src[1];
        target[2] += src[2];
    }

    /// Accumulate scaled: `target += s * src`
    #[inline]
    pub fn add_scaled(target: &mut Vec3, src: Vec3, s: f64) {
        target[0] += src[0] * s;
        target[1] += src[1] * s;
        target[2] += src[2] * s;
    }
}

#[cfg(test)]
mod tests {
    use super::ops::*;

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert!((dot(a, b) - 32.0).abs() < 1e-12);
    }

    #[test]
    fn test_cross_product() {
        let x = [1.0, 0.0, 0.0];
        let y = [0.0, 1.0, 0.0];
        let z = cross(x, y);
        assert!((z[0]).abs() < 1e-12);
        assert!((z[1]).abs() < 1e-12);
        assert!((z[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_normalize() {
        let v = [3.0, 4.0, 0.0];
        let n = normalize(v);
        assert!((magnitude(n) - 1.0).abs() < 1e-12);
    }
}
