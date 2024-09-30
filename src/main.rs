use chrono::{Duration, Utc};
use rand::Rng;
use std::time::{Instant, Duration as StdDuration};


mod vector3;
mod ellipse_math;
mod orbit_math;
mod orbit_db;

use orbit_db::OrbitDB;

const NUM_ORBITS: usize = 5000;
const NUM_RUNS: usize = 10;

fn main() {
    let mut rng = rand::thread_rng();
    let mut orbits: Vec<OrbitDB> = Vec::with_capacity(NUM_ORBITS);

    // Create NUM_ORBITS random OrbitDB instances
    for _ in 0..NUM_ORBITS {
        orbits.push(OrbitDB {
            epoch: Utc::now(),
            orbital_period: Duration::seconds(rng.gen_range(3600..86400)), // 1 hour to 1 day
            eccentricity: rng.gen_range(0.0..0.9),
            mean_anomaly_at_epoch: rng.gen_range(0.0..std::f64::consts::TAU),
            mean_motion: rng.gen_range(0.1..1.0),
            semi_major_axis: rng.gen_range(1e6..1e8), // 1000 km to 100,000 km
            longitude_of_ascending_node: rng.gen_range(0.0..std::f64::consts::TAU),
            argument_of_periapsis: rng.gen_range(0.0..std::f64::consts::TAU),
            inclination: rng.gen_range(0.0..std::f64::consts::FRAC_PI_2),
        });
    }

    let mut durations: Vec<StdDuration> = Vec::with_capacity(NUM_RUNS);

    for run in 1..=NUM_RUNS {
        let start_time = Instant::now();
        let update_time = Utc::now() + Duration::days(1); // Update positions for 1 day in the future

        for orbit in &mut orbits {
            let _new_position = orbit.get_position(update_time);
        }

        let duration = start_time.elapsed();
        durations.push(duration);
        println!("Run {}: {:?}", run, duration);
    }

    // Calculate statistics
    let total_duration: StdDuration = durations.iter().sum();
    let average_duration = total_duration / NUM_RUNS as u32;
    let min_duration = durations.iter().min().unwrap();
    let max_duration = durations.iter().max().unwrap();

    println!("\nBenchmark Results for {} runs with {} orbits:", NUM_RUNS, NUM_ORBITS);
    println!("  Fastest run: {:?}", min_duration);
    println!("  Slowest run: {:?}", max_duration);
    println!("  Average duration: {:?}", average_duration);
    println!("  Total duration: {:?}", total_duration);
}