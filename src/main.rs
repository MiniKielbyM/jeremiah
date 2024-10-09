#![no_std]
#![no_main]

use anyhow::Result;
use core::{fmt::Write, time::Duration};
use vexide::{
    devices::screen::{Rect, Text, TextSize},
    prelude::*,
    startup::banner::themes::THEME_TRANS,
};

#[vexide::main(banner(theme = THEME_TRANS))]
async fn main(peripherals: Peripherals) -> Result<()> {
    // We can get the screen directly from peripherals becuase it is always connected to the Brain.
    let mut screen: Screen = peripherals.screen;

    // Print a message to the screen
    write!(screen, "Hello, world!")?;

    // Create a rectangle to be drawn on the screen.
    let rect: Rect = Rect::new((20, 20), (100, 100));


    loop {
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
