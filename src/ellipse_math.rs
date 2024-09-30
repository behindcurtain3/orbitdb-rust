pub struct EllipseMath;

impl EllipseMath {
    pub fn semi_latus_rectum(semi_major_axis: f64, eccentricity: f64) -> f64 {
        if eccentricity == 0.0 {
            // i.e., a circle
            semi_major_axis
        } else {
            semi_major_axis * (1.0 - eccentricity * eccentricity)
        }
    }

    pub fn radius_at_true_anomaly(angle: f64, semi_latus_rectum: f64, eccentricity: f64) -> f64 {
        (semi_latus_rectum / (1.0 + eccentricity * angle.cos())).abs()
    }
}