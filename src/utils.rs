//! Utility functions.

use core::{
    f64,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    time::Duration,
};

use crate::Float;

/// Calculates how many revolutions it will take to spin for `durations` seconds at `rpm` revolutions per minute.
pub fn revolutions_from_duration(duration: Duration, rpm: u32) -> f64 {
    (duration.as_secs_f64() / 60.0) * f64::from(rpm)
}

/// Calculates the distance travelled in inches from how many revolutions a wheel has spun.
/// Assumes `radius = 2`.
pub fn distance_from_revolutions(revolutions: f64, rpm: u32) -> f64 {
    12.57 * (f64::from(rpm) / 60.0 * revolutions)
}

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
        let s: f64 = angle.sin();
        let c: f64 = angle.cos();
        for p in &mut self.points {
            p.0 -= origin.0;
            p.1 -= origin.1;

            let xnew: f64 = p.0 * c - p.1 * s;
            let ynew: f64 = p.0 * s + p.1 * c;

            p.0 = xnew + origin.1;
            p.1 = ynew + origin.1;
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
            let mut futhers: Futhers = Futhers::default();

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
                    .is_none_or(|x: FuthersChild| x.signed_distance > signed_distance)
                {
                    futhers.min = Some(FuthersChild {
                        signed_distance,
                        corner,
                        projected,
                    });
                }
                if futhers
                    .max
                    .is_none_or(|x: FuthersChild| x.signed_distance < signed_distance)
                {
                    futhers.max = Some(FuthersChild {
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
struct Futhers {
    min: Option<FuthersChild>,
    max: Option<FuthersChild>,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct FuthersChild {
    signed_distance: f64,
    corner: Vector2,
    projected: Vector2,
}
