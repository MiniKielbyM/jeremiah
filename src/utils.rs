//! Utility functions.

use core::{
    f64,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    time::Duration,
};
use crate::Float;

const PI: f64 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348610454326648213393607260249141273724587006606315588174881520920962829254091715364367892590360011330530548820466521384146951941511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076961511609433057270365759591953092186117381932611793105118548074;

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
struct Vector2 {
    x: f64,
    y: f64,
}

fn plot_route(center: Vector2, goal: Vector2) -> [f64; 2] {
    
    let slope = (center.y - goal.y) / (center.x - goal.x);
    let angle = slope.to_degrees();
    let distance = ((center.x - goal.x).powi(2) + (center.y - goal.y).powi(2)).sqrt();
    if goal.x == center.x {
        [-angle, distance]
    } 
    else if goal.x > center.x {
        [angle, -distance]
    } 
    else {
        [angle, distance]
    }
}

// Helper method to convert radians to degrees
trait RadiansToDegrees {
    fn to_degrees(self) -> f64;
}

impl RadiansToDegrees for f64 {
    fn to_degrees(self) -> f64 {
        self * (180.0 / PI)
    }
}


/*
/// A coordinate on a 2D grid.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2(pub f64, pub f64);

impl Vector2 {
    pub fn magnitude(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn rotate(&self, theta: f64) -> Self {
        Self(
            self.0 * theta.cos() - self.1 * theta.sin(),
            self.0 * theta.sin() + self.1 * theta.cos(),
        )
    }

    pub fn project(&self, line: Line) -> Self {
        let dotvalue: f64 = line.direction.0 * (self.0 - line.origin.0)
            + line.direction.1 * (self.1 - line.origin.1);
        Self(
            line.origin.0 + line.direction.0 * dotvalue,
            line.origin.1 + line.direction.1 * dotvalue,
        )
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

/// A line in 2D space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    origin: Vector2,
    direction: Vector2,
}

/// A rectangle used for boundary box checking.
/// This shape should be rotated around its center.
#[derive(Clone, Debug, PartialEq)]
pub struct Rectangle {
    points: [Vector2; 4],
    angle: f64,
    width: f64,
    height: f64,
    origin: Vector2,
}

impl Rectangle {
    /// Creates a new `Rectangle`.
    pub fn new(origin: Vector2, width: f64, height: f64, angle: f64) -> Self {
        let mut points: [Vector2; 4] = [
            Vector2(origin.0 - width / 2.0, origin.1 - height / 2.0),
            Vector2(origin.0 + width / 2.0, origin.1 - height / 2.0),
            Vector2(origin.0 + width / 2.0, origin.1 + height / 2.0),
            Vector2(origin.0 - width / 2.0, origin.1 + height / 2.0),
        ];
        if angle != 0.0 {
            for p in &mut points {
                *p -= origin;
                *p = p.rotate(angle) + origin;
            }
        }
        Self {
            points,
            angle,
            width,
            height,
            origin,
        }
    }

    pub fn get_axis(&self) -> [Line; 2] {
        let ox: Vector2 = Vector2(1.0, 0.0);
        let oy: Vector2 = Vector2(0.0, 1.0);
        let rx: Vector2 = ox.rotate(self.angle);
        let ry: Vector2 = oy.rotate(self.angle);
        [
            Line {
                origin: self.origin,
                direction: Vector2(rx.0, rx.1),
            },
            Line {
                origin: self.origin,
                direction: Vector2(ry.0, ry.1),
            },
        ]
    }

    /// Rotates a `Rectangle` using `origin` as its center of rotation.
    pub fn rotate_around_point(&mut self, origin: Vector2, angle: f64) {
        for p in &mut self.points {
            *p -= origin;
            *p = p.rotate(angle) + origin;
        }
    }

    /// Re-calculates `self.points`.
    fn recalculate_points(&mut self) {
        let mut points: [Vector2; 4] = [
            Vector2(
                self.origin.0 - self.width / 2.0,
                self.origin.1 - self.height / 2.0,
            ),
            Vector2(
                self.origin.0 + self.width / 2.0,
                self.origin.1 - self.height / 2.0,
            ),
            Vector2(
                self.origin.0 + self.width / 2.0,
                self.origin.1 + self.height / 2.0,
            ),
            Vector2(
                self.origin.0 - self.width / 2.0,
                self.origin.1 + self.height / 2.0,
            ),
        ];
        if self.angle != 0.0 {
            for p in &mut points {
                *p -= self.origin;
                *p = p.rotate(self.angle) + self.origin;
            }
        }
        self.points = points;
    }

    /// Moves a shape relatively. `displacement` will be rotated by `self.angle`.
    pub fn move_shape(&mut self, displacement: Vector2) {
        self.origin += displacement;
        self.recalculate_points();
    }

    /// Checks if 2 `Rectangle`s collide/intersect.
    pub fn is_colliding(&self, other: &Self) -> bool {
        self.is_projection_collide(other) && other.is_projection_collide(self)
    }

    /// Checks if a projection from a `Rectangle` collides with another.
    fn is_projection_collide(&self, other: &Self) -> bool {
        let lines: [Line; 2] = other.get_axis();
        let corners: [Vector2; 4] = self.points;

        for (dimension, line) in lines.iter().enumerate() {
            let mut futhers: Projection = Projection::default();

            // Size of `other` half size on line direction
            let rect_half_size: f64 = (if dimension == 0 {
                other.width
            } else {
                other.height
            }) / 2.0;

            for corner in corners {
                let projected: Vector2 = corner.project(*line);
                let cp: Vector2 = projected - other.origin;

                // Sign: Same direction of `other` axis : true.
                let sign: bool = (cp.0 * line.direction.0) + (cp.1 * line.direction.1) > 0.0;
                let signed_distance: f64 = cp.magnitude() * (if sign { 1.0 } else { -1.0 });

                if futhers
                    .min
                    .is_none_or(|x: ProjectionResult| x.signed_distance > signed_distance)
                {
                    futhers.min = Some(ProjectionResult {
                        signed_distance,
                        corner,
                        projected,
                    });
                }
                if futhers
                    .max
                    .is_none_or(|x: ProjectionResult| x.signed_distance < signed_distance)
                {
                    futhers.max = Some(ProjectionResult {
                        signed_distance,
                        corner,
                        projected,
                    });
                }
            }

            if !(futhers.min.unwrap().signed_distance < 0.0
                && futhers.max.unwrap().signed_distance > 0.0
                || futhers.min.unwrap().signed_distance.abs() < rect_half_size
                || futhers.max.unwrap().signed_distance.abs() < rect_half_size)
            {
                return false;
            }
        }

        true
    }

    /// Checks if this `Rectangle` is fully inside another.
    pub fn is_fully_inside(&self, other: &Self) -> bool {
        self.origin.0 + self.width / 2.0 <= other.origin.0 + other.width / 2.0
            && self.origin.0 - self.width / 2.0 >= other.origin.0 - other.width / 2.0
            && self.origin.1 + self.height / 2.0 <= other.origin.1 + other.height / 2.0
            && self.origin.1 - self.height / 2.0 >= other.origin.1 - other.height / 2.0
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
struct Projection {
    min: Option<ProjectionResult>,
    max: Option<ProjectionResult>,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct ProjectionResult {
    signed_distance: f64,
    corner: Vector2,
    projected: Vector2,
}

pub type VehicleBody = Rectangle;

impl VehicleBody {
    /// Projects this `Rectangle` `moment` moments into the future to see how it moves.
    /// A safe bet for setting `moment` should be 1-5.
    pub fn project_future(
        &self,
        revolutions: i32,
        velocity_left: f64,
        velocity_right: f64,
        wheel_distance: f64,
    ) -> Self {
        let mut instance: Self = self.clone();
        for _ in 0..revolutions {
            instance.drive(velocity_left, velocity_right, wheel_distance);
        }

        instance
    }

    /// This moves the `VehicleBody` in 2D space, according to the motor velocity.
    pub fn drive(&mut self, velocity_left: f64, velocity_right: f64, wheel_distance: f64) {
        let v: f64 = linear_velocity(velocity_left, velocity_right);
        let omega: f64 = angular_velocity(velocity_left, velocity_right, wheel_distance);

        self.angle += omega;
        self.move_shape(Vector2(v * self.angle.cos(), v * self.angle.sin()));
    }
}

//calculates distance and expected angle to move to position

/*
    TODO: boundary checking

    This should be projected a few (3-5) revolutions in advance.

    Let V(L) be the velocity of the front left motor in inches per second,
    and V(R) be that of the front right motor.
    Let D be the distance in inches between the 2 sides of wheels.

    Linear velocity: V = (V(L) + V(R)) / 2
    Angular velocity: W = (V(R) - V(L)) / D

    Body angle += W
    Body X += V * cos(Body angle)
    Body Y += V * sin(Body angle)
*/
*/
