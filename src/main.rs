#![no_std]
#![no_main]

mod func;
mod utils;

extern crate alloc;

use anyhow::Result;
use func::Jeremiah;
//use utils::{Vector2, VehicleBody};
use vexide::{prelude::*, startup::banner::themes::THEME_TRANS};

#[vexide::main(banner(theme = THEME_TRANS))]
async fn main(peripherals: Peripherals) -> Result<()> {
    Jeremiah {
        motor_left_front: Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        motor_left_back: Motor::new(peripherals.port_2, Gearset::Green, Direction::Reverse),
        motor_right_front: Motor::new(peripherals.port_11, Gearset::Green, Direction::Forward),
        motor_right_back: Motor::new(peripherals.port_12, Gearset::Green, Direction::Reverse),
        controller: peripherals.primary_controller,
        screen: peripherals.screen

        // TODO: Measure actual vehicle
//        body: VehicleBody::new(Vector2(8.88, 9.24), 17.75, 18.5, f64::to_radians(90.0)),
    }
    .compete()
    .await;
}
