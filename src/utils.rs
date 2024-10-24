//! Utility functions.

use core::{
    f64,
    time::Duration,
};

use crate::Float;

/// Calculates how many revolutions it would take to spin for `durations` seconds at `rpm` revolutions per minute.
pub fn revolutions_from_duration(duration: Duration, rpm: u32) -> f64 {
    (duration.as_secs_f64() / 60.0) * f64::from(rpm)
}

/// Calculates how much time it would take to spin `revolutions` revolutions at `rpm` revolutions per minute.
pub fn duration_from_revolutions(revolutions: f64, rpm: u32) -> Duration {
    Duration::from_secs_f64(revolutions / f64::from(rpm) * 60.0)
}

/// Calculates the circumfrence in inches of `degrees` degrees around the wheel.
/// Assumes `radius = 2`.
pub fn distance_from_degrees(degrees: f64) -> f64 {
    0.034 * degrees
}

/// Calculates the distance travelled in inches from how many revolutions a wheel has spun.
/// Assumes `radius = 2`.
pub fn distance_from_revolutions(revolutions: f64, rpm: u32) -> f64 {
    12.57 * (f64::from(rpm) / 60.0 * revolutions)
}

/// Converts revolutions per minute to inches per second.
/// Assumes `radius = 2`.
pub fn rpm_to_ips(rpm: u32) -> f64 {
    (f64::from(rpm) / 60.0) * 12.57
}

/// Calculates the linear velocity of the `VehicleBody`.
/// This returns in/s.
pub fn linear_velocity(velocity_left: f64, velocity_right: f64) -> f64 {
    (velocity_left + velocity_right) / 2.0
}

/// Calculates the angular velocity of the `VehicleBody`.
/// This returns rad/s.
pub fn angular_velocity(velocity_left: f64, velocity_right: f64, wheel_distance: f64) -> f64 {
    (velocity_right - velocity_left) / wheel_distance
}

#[derive(Debug)]
struct Vector2(f64, f64);

fn plot_route(center: Vector2, goal: Vector2) -> (f64, f64) {
    let angle: f64 = ((center.1 - goal.1) / (center.0 - goal.0)).atan().to_degrees();
    let distance: f64 = ((center.0 - goal.0).powi(2) + (center.1 - goal.1).powi(2)).sqrt();
    if goal.0 == center.0 {
        return (-angle, distance);
    } else if goal.0 > center.0 {
        return (angle, -distance);
    }
    (angle, distance)
}
fn revolutions_from_route(center: Vector2, goal: Vector2, rotation: f64){
    let expectedAngle: f64 = plot_route(center, goal).0;
    //TODO: find out how many rotations it takes to rotate 90 degrees
    //TODO: find out how far the robot goes in 1 revolution
} 
