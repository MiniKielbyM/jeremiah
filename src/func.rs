use core::time::Duration;

use vexide::prelude::*;

use crate::utils::{revolutions_from_duration, rpm_to_ipms, Rectangle, Vector2, VehicleBody};

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
        let boundary: Rectangle = Rectangle::new(Vector2(36.0, 72.0), 72.0, 144.0, 0.0); // Example dimensions

        // Set motor targets based on calculated revolutions
        let volts: f64 = 12.0;

        self.motor_left_front.set_voltage(volts).ok();
        self.motor_left_back.set_voltage(volts).ok();
        self.motor_right_front.set_voltage(volts).ok();
        self.motor_right_back.set_voltage(volts).ok();

        loop {
            let projected_body: VehicleBody = self.body.project_future(
                5,
                rpm_to_ipms(self.motor_left_front.voltage().unwrap() as u32) * 10.0,
                rpm_to_ipms(self.motor_right_front.voltage().unwrap() as u32) * 10.0,
            );

            println!(
                "{:#?}\n{:#?}\n{}",
                self.body,
                projected_body,
                projected_body.is_fully_inside(&boundary)
            );
            if !projected_body.is_fully_inside(&boundary) {
                self.motor_left_front.set_voltage(0.0).ok();
                self.motor_left_back.set_voltage(0.0).ok();
                self.motor_right_front.set_voltage(0.0).ok();
                self.motor_right_back.set_voltage(0.0).ok();
                break;
            }
            self.body
                .drive(rpm_to_ipms(100) * 10.0, rpm_to_ipms(100) * 10.0);

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
