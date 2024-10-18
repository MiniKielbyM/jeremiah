#![no_std]
#![no_main]

mod func;
mod utils;

extern crate alloc;

use anyhow::Result;
use func::Jeremiah;
use utils::{Rectangle, Vector2};
use vexide::{prelude::*, startup::banner::themes::THEME_TRANS};

#[vexide::main(banner(theme = THEME_TRANS))]
async fn main(peripherals: Peripherals) -> Result<()> {
    Jeremiah::new(
        Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        Motor::new(peripherals.port_2, Gearset::Green, Direction::Reverse),
        Motor::new(peripherals.port_11, Gearset::Green, Direction::Forward),
        Motor::new(peripherals.port_12, Gearset::Green, Direction::Reverse),
        peripherals.primary_controller,
        peripherals.screen,
        // TODO: Measure actual vehicle
        Rectangle::new(Vector2(0.0, 0.0), 150.0, 100.0, 0.0),
    )
    .compete()
    .await;
}
