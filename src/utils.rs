use core::time::Duration;

/// Calculates how many revolutions it will take to spin for `durations` seconds at `rpm` revolutions per minute.
pub fn revolutions_from_duration(duration: Duration, rpm: u32) -> f64 {
    (duration.as_secs_f64() / 60.0) * f64::from(rpm)
}

pub fn revolutions_from_distance_for_duration(distance: f64, duration: Duration, rpm: u32) -> f64 {
    (distance / 360.0) / (duration.as_secs_f64() / 60.0) * f64::from(rpm)
}