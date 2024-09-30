use chrono::{DateTime, Duration, Utc};
use crate::vector3::Vector3;
use crate::orbit_math::OrbitMath;

pub struct OrbitDB {
    pub epoch: DateTime<Utc>,
    pub orbital_period: Duration,
    pub eccentricity: f64,
    pub mean_anomaly_at_epoch: f64,
    pub mean_motion: f64,
    pub semi_major_axis: f64,
    pub longitude_of_ascending_node: f64,
    pub argument_of_periapsis: f64,
    pub inclination: f64,
}

impl OrbitDB {
    pub fn get_position(&self, time: DateTime<Utc>) -> Vector3 {
        let true_anomaly = self.get_true_anomaly(time);
        OrbitMath::get_position(
            self.semi_major_axis,
            self.eccentricity,
            self.longitude_of_ascending_node,
            self.argument_of_periapsis,
            self.inclination,
            true_anomaly,
        )
    }

    pub fn get_true_anomaly(&self, time: DateTime<Utc>) -> f64 {
        let mut time_since_epoch = time - self.epoch;
        let mut orbit = self.clone();

        // Don't attempt to calculate large timeframes.
        while time_since_epoch > orbit.orbital_period && orbit.orbital_period.num_nanoseconds().unwrap() != 0 {
            let years = time_since_epoch.num_nanoseconds().unwrap() / orbit.orbital_period.num_nanoseconds().unwrap();
            time_since_epoch -= Duration::nanoseconds(years * orbit.orbital_period.num_nanoseconds().unwrap());
            orbit.epoch += Duration::nanoseconds(years * orbit.orbital_period.num_nanoseconds().unwrap());
        }

        let seconds_from_epoch = time_since_epoch.num_seconds() as f64;

        if orbit.eccentricity < 1.0 {
            // elliptical orbit
            let o_m0 = orbit.mean_anomaly_at_epoch;
            let o_m1 = OrbitMath::get_mean_anomaly_from_time(o_m0, orbit.mean_motion, seconds_from_epoch);
            let o_e = OrbitMath::get_eccentric_anomaly(&orbit, o_m1);
            OrbitMath::true_anomaly_from_eccentric_anomaly(orbit.eccentricity, o_e)
        } else {
            // hyperbolic orbit
            let o_mh = OrbitMath::get_hyperbolic_mean_anomaly_from_time(orbit.mean_motion, seconds_from_epoch);
            let o_f = OrbitMath::get_hyperbolic_anomaly(&orbit, o_mh);
            OrbitMath::true_anomaly_from_hyperbolic_anomaly(orbit.eccentricity, o_f)
        }
    }
}

// Implement Clone for OrbitDB
impl Clone for OrbitDB {
    fn clone(&self) -> Self {
        OrbitDB {
            epoch: self.epoch,
            orbital_period: self.orbital_period,
            eccentricity: self.eccentricity,
            mean_anomaly_at_epoch: self.mean_anomaly_at_epoch,
            mean_motion: self.mean_motion,
            semi_major_axis: self.semi_major_axis,
            longitude_of_ascending_node: self.longitude_of_ascending_node,
            argument_of_periapsis: self.argument_of_periapsis,
            inclination: self.inclination,
        }
    }
}