#![no_std]
#![no_main]

mod func;
mod utils;

extern crate alloc;

use anyhow::Result;
use func::Jeremiah;
use vexide::{prelude::*, startup::banner::themes::THEME_TRANS};

#[vexide::main(banner(theme = THEME_TRANS))]
async fn main(peripherals: Peripherals) -> Result<()> {
    Jeremiah::new(
        Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        Motor::new(peripherals.port_2, Gearset::Green, Direction::Reverse),
        Motor::new(peripherals.port_11, Gearset::Green, Direction::Reverse),
        Motor::new(peripherals.port_12, Gearset::Green, Direction::Forward),
        peripherals.primary_controller,
        peripherals.screen,
    )
    .compete()
    .await;
}
