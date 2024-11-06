pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_basic() {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_lerp_negative_values() {
        assert_eq!(lerp(-10.0, 10.0, 0.5), 0.0);
        assert_eq!(lerp(-20.0, -10.0, 0.5), -15.0);
    }

    #[test]
    fn test_lerp_beyond_bounds() {
        // Testing interpolation factors outside [0,1]
        assert_eq!(lerp(0.0, 10.0, 2.0), 20.0);
        assert_eq!(lerp(0.0, 10.0, -1.0), -10.0);
    }

    #[test]
    fn test_lerp_small_values() {
        assert!((lerp(0.1, 0.2, 0.5) - 0.15).abs() < f32::EPSILON);
        assert!((lerp(0.01, 0.02, 0.5) - 0.015).abs() < f32::EPSILON);
    }
}
