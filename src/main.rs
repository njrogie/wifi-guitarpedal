#![no_std]
#![no_main]

use embedded_graphics::{
    pixelcolor::BinaryColor,
    draw_target::DrawTarget,
    primitives::{
        Circle,
        PrimitiveStyle,
        Primitive
    },
    Drawable,
    geometry::Point
};

use esp_backtrace as _;
use esp_hal::{peripherals::Peripherals, prelude::*};
use ssd1306::prelude::*;

mod esp32wroom32;
use esp32wroom32::ESP32WROOM32LCDBoardUtils as BoardUtils;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let utils = BoardUtils::new(peripherals);

    // initialize the display
    let mut display = esp32wroom32::get_display(utils.i2c);
    display.init().unwrap();

    // initialize wifi


    loop {
     
    }
}
