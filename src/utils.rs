use core::time::Duration;

pub fn revolutions_from_duration(duration: Duration, rpm: u32) -> f64 {
    (duration.as_secs_f64() / 60.0) * f64::from(rpm)
}