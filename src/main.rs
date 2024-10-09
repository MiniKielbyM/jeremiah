#![no_std]
#![no_main]

use core::time::Duration;

use anyhow::Result;
use vexide::{prelude::*, startup::banner::themes::THEME_TRANS};

struct Jeremiah {
    motor_left_front: Motor,
    motor_left_back: Motor,
    motor_right_front: Motor,
    motor_right_back: Motor,
    controller: Controller,
}

impl Compete for Jeremiah {
    async fn autonomous(&mut self) {
        self.motor_left_front
            .set_target(MotorControl::Position(
                Position::from_revolutions(10.0),
                100,
            ))
            .ok();
        self.motor_left_back
            .set_target(MotorControl::Position(
                Position::from_revolutions(10.0),
                100,
            ))
            .ok();

        loop {
            sleep(Duration::from_millis(10)).await;
        }
    }

    async fn driver(&mut self) {
        loop {
            let forward: f64 = self.controller.left_stick.y().unwrap_or_default();
            let turn: f64 = self.controller.right_stick.x().unwrap_or_default();
            let mut left_voltage: f64 = (forward + turn) * Motor::MAX_VOLTAGE;
            let mut right_voltage: f64 = (forward - turn) * Motor::MAX_VOLTAGE;

            // Set the drive motors to our arcade control values.
            // TODO: Make adjacent motors not pull against eachother
            self.motor_left_front
                .set_voltage(left_voltage)
                .ok();
            self.motor_left_back
                .set_voltage(right_voltage)
                .ok();

            self.motor_right_front
                .set_voltage(left_voltage)
                .ok();
            self.motor_right_back
                .set_voltage(right_voltage)
                .ok();

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    }
}

#[vexide::main(banner(theme = THEME_TRANS))]
async fn main(peripherals: Peripherals) -> Result<()> {
    Jeremiah {
        motor_left_front: Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        motor_left_back: Motor::new(peripherals.port_11, Gearset::Green, Direction::Reverse),
        motor_right_front: Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
        motor_right_back: Motor::new(peripherals.port_12, Gearset::Green, Direction::Reverse),
        controller: peripherals.primary_controller,
    }
    .compete()
    .await;

    Ok(())
}
