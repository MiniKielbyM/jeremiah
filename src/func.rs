use core::time::Duration;

use vexide::prelude::*;

use crate::utils::revolutions_from_duration;

pub struct Jeremiah {
    pub motor_left_front: Motor,
    pub motor_left_back: Motor,
    pub motor_right_front: Motor,
    pub motor_right_back: Motor,

    pub controller: Controller,
    pub screen: Screen,
}

impl Compete for Jeremiah {
    async fn autonomous(&mut self) {
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

        loop {
            sleep(Duration::from_millis(10)).await;
        }
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
