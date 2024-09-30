use std::f64::consts::PI;
use crate::vector3::Vector3;
use crate::orbit_db::OrbitDB;
use crate::ellipse_math::EllipseMath;

pub struct OrbitMath;

impl OrbitMath {
    pub fn get_position(a: f64, e: f64, lo_an: f64, ao_p: f64, i: f64, true_anomaly: f64) -> Vector3 {
        let p = EllipseMath::semi_latus_rectum(a, e);
        let r = EllipseMath::radius_at_true_anomaly(true_anomaly, p, e);
        let angle_from_lo_an = true_anomaly + ao_p;
        let x = lo_an.cos() * angle_from_lo_an.cos() - lo_an.sin() * angle_from_lo_an.sin() * i.cos();
        let y = lo_an.sin() * angle_from_lo_an.cos() + lo_an.cos() * angle_from_lo_an.sin() * i.cos();
        let z = i.sin() * angle_from_lo_an.sin();
        Vector3::new(x, y, z) * r
    }

    pub fn get_mean_anomaly_from_time(mean_anomaly_at_epoch: f64, mean_motion: f64, seconds_from_epoch: f64) -> f64 {
        (mean_anomaly_at_epoch + mean_motion * seconds_from_epoch) % (2.0 * PI)
    }

    pub fn get_eccentric_anomaly(orbit: &OrbitDB, mean_anomaly: f64) -> f64 {
        let mut e = mean_anomaly;
        for _ in 0..10 {  // Maximum 10 iterations
            let e_next = e - (e - orbit.eccentricity * e.sin() - mean_anomaly) / (1.0 - orbit.eccentricity * e.cos());
            if (e_next - e).abs() < 1e-6 {
                return e_next;
            }
            e = e_next;
        }
        e  // Return best approximation if not converged
    }

    pub fn true_anomaly_from_eccentric_anomaly(eccentricity: f64, eccentric_anomaly: f64) -> f64 {
        let cos_e = eccentric_anomaly.cos();
        let sin_e = eccentric_anomaly.sin();
        let true_anomaly = (((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt() * sin_e / (1.0 - eccentricity * cos_e)).atan2(
            (cos_e - eccentricity) / (1.0 - eccentricity * cos_e)
        );
        (true_anomaly + 2.0 * PI) % (2.0 * PI)  // Ensure result is in [0, 2π)
    }

    pub fn get_hyperbolic_mean_anomaly_from_time(mean_motion: f64, seconds_from_epoch: f64) -> f64 {
        mean_motion * seconds_from_epoch
    }

    pub fn get_hyperbolic_anomaly(orbit: &OrbitDB, hyperbolic_mean_anomaly: f64) -> f64 {
        let mut f = hyperbolic_mean_anomaly;
        for _ in 0..10 {  // Maximum 10 iterations
            let f_next = f - (orbit.eccentricity * f.sinh() - f - hyperbolic_mean_anomaly) / (orbit.eccentricity * f.cosh() - 1.0);
            if (f_next - f).abs() < 1e-6 {
                return f_next;
            }
            f = f_next;
        }
        f  // Return best approximation if not converged
    }

    pub fn true_anomaly_from_hyperbolic_anomaly(eccentricity: f64, hyperbolic_anomaly: f64) -> f64 {
        let true_anomaly = ((eccentricity + 1.0) / (eccentricity - 1.0)).sqrt() * (hyperbolic_anomaly / 2.0).tanh();
        true_anomaly.atan() * 2.0
    }
}