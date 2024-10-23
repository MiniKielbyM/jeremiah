use core::time::Duration;

use vexide::prelude::*;

use crate::utils::{revolutions_from_duration, rpm_to_ips/*Rectangle, Vector2, VehicleBody*/};

pub struct Jeremiah {
    pub motor_left_front: Motor,
    pub motor_left_back: Motor,
    pub motor_right_front: Motor,
    pub motor_right_back: Motor,

    pub controller: Controller,
    pub screen: Screen,
    pub body: VehicleBody,
}

impl Compete for Jeremiah {
    async fn autonomous(&mut self) {
        // Define the boundaries of your operating area
  //      let boundary: Rectangle = Rectangle::new(Vector2(36.0, 72.0), 72.0, 144.0, 0.0); // Example dimensions

        // Set motor targets based on calculated revolutions
        let target_revolutions: f64 = revolutions_from_duration(Duration::from_secs(15), 100);

        self.motor_left_front
            .set_target(MotorControl::Position(
                Position::from_revolutions(-target_revolutions),
                100,
            ))
            .ok();

        self.motor_left_back
            .set_target(MotorControl::Position(
                Position::from_revolutions(-target_revolutions),
                100,
            ))
            .ok();

        self.motor_right_front
            .set_target(MotorControl::Position(
                Position::from_revolutions(-target_revolutions),
                100,
            ))
            .ok();
        self.motor_right_back
            .set_target(MotorControl::Position(
                Position::from_revolutions(target_revolutions),
                100,
            ))
            .ok();

    /*    loop {
            let projected_body: VehicleBody = self.body.project_future(
                5,
                rpm_to_ips(self.motor_left_front.velocity().unwrap_or(0).unsigned_abs()),
                rpm_to_ips(
                    self.motor_right_front
                        .velocity()
                        .unwrap_or(0)
                        .unsigned_abs(),
                ),
                12.0,
            );

            if !projected_body.is_fully_inside(&boundary) {
                self.motor_left_front.brake(BrakeMode::Hold).ok();
                self.motor_left_back.brake(BrakeMode::Hold).ok();
                self.motor_right_front.brake(BrakeMode::Hold).ok();
                self.motor_right_back.brake(BrakeMode::Hold).ok();
                break;
            }

            sleep(Duration::from_millis(10)).await;
        } */

    }

    async fn driver(&mut self) {
        loop {
            let forward_left: f64 = self.controller.left_stick.y().unwrap_or_default();
            let forward_right: f64 = self.controller.right_stick.y().unwrap_or_default();
            let left_voltage: f64 = forward_left * Motor::MAX_VOLTAGE;
            let right_voltage: f64 = forward_right * Motor::MAX_VOLTAGE;

            // Set the drive motors to our arcade control values.
            self.motor_left_front.set_voltage(-left_voltage).ok();
            self.motor_left_back.set_voltage(left_voltage).ok();

            self.motor_right_front.set_voltage(right_voltage).ok();
            self.motor_right_back.set_voltage(-right_voltage).ok();

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    }
}
