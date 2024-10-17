//! Utility functions.

use alloc::vec::Vec;
use core::{
    f64,
    ops::{Add, Mul, Sub},
    time::Duration,
};

use crate::Float;

/// Calculates how many revolutions it will take to spin for `durations` seconds at `rpm` revolutions per minute.
pub fn revolutions_from_duration(duration: Duration, rpm: u32) -> f64 {
    (duration.as_secs_f64() / 60.0) * f64::from(rpm)
}

/// Converts radians to degrees.
pub fn to_degrees(radians: f64) -> f64 {
    radians * 180.0 / f64::consts::PI
}

/// Converts degrees to radians.
pub fn to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}

/// A coordinate on a 2D grid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2(f64, f64);

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
        Vector2(
            line.origin.0 + line.direction.0 * dotvalue,
            line.origin.1 + line.direction.1 * dotvalue,
        )
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2(self.0 * rhs.0, self.1 * rhs.1)
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
    pub p1: Vector2,
    pub p2: Vector2,
    pub p3: Vector2,
    pub p4: Vector2,
    pub angle: f64,
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
            let s: f64 = angle.sin();
            let c: f64 = angle.cos();
            for p in points.iter_mut() {
                p.0 -= origin.0;
                p.1 -= origin.1;

                let xnew: f64 = p.0 * c - p.1 * s;
                let ynew: f64 = p.0 * s + p.1 * c;

                p.0 = xnew + origin.1;
                p.1 = ynew + origin.1;
            }
        }
        Self {
            p1: points[0],
            p2: points[1],
            p3: points[2],
            p4: points[3],
            width,
            height,
            angle,
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
        let mut points: [&mut Vector2; 4] =
            [&mut self.p1, &mut self.p2, &mut self.p3, &mut self.p4];
        let s: f64 = angle.sin();
        let c: f64 = angle.cos();
        for p in points.iter_mut() {
            p.0 -= origin.0;
            p.1 -= origin.1;

            let xnew: f64 = p.0 * c - p.1 * s;
            let ynew: f64 = p.0 * s + p.1 * c;

            p.0 = xnew + origin.1;
            p.1 = ynew + origin.1;
        }
    }

    /// Checks if 2 `Rectangle`s collide/intersect.
    pub fn is_colliding(&self, other: &Rectangle) -> bool {
        self.is_projection_collide(other) && other.is_projection_collide(self)
    }

    /// Checks if a projection from a `Rectangle` collides with another.
    fn is_projection_collide(&self, other: &Rectangle) -> bool {
        let lines: [Line; 2] = other.get_axis();
        let corners: [Vector2; 4] = [self.p1, self.p2, self.p3, self.p4];

        for (dimension, line) in lines.iter().enumerate() {
            // .0 = min, .1 = max
            // .0 = signed_distance, .1 = corner, .2 = projected
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
