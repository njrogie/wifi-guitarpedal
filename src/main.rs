#![no_std]
#![no_main]

use embedded_graphics::{
    draw_target::DrawTarget, geometry::{Point, Size}, mono_font::{
        iso_8859_10::FONT_6X10,
        MonoTextStyleBuilder,
    }, pixelcolor::BinaryColor, primitives::{
        Primitive,  
        PrimitiveStyleBuilder, 
        Rectangle
    }, text::Text, Drawable
};

use esp_backtrace as _;
use esp_hal::{peripherals::Peripherals, prelude::*};
use ssd1306::prelude::*;

mod esp32wroom32;
use esp32wroom32::ESP32WROOM32LCDBoardUtils as BoardUtils;
use wifi_guitarpedal::convert_adc_to_str;

#[entry]
fn main() -> ! { 
    let peripherals = Peripherals::take();
    let mut utils = BoardUtils::new(peripherals);

    // initialize the display
    let mut display = esp32wroom32::get_display(utils.i2c);
    display.init().unwrap();
    let _ = display.set_rotation(DisplayRotation::Rotate180);

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();
    
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    // initialize wifi
    loop {
        Rectangle::new(Point::new(0, 0), Size::new(127, 63))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();
        
        let read = utils.adc.read(&mut utils.adc_pin);
        match read {
            Ok(it) => {
                let it = 4095 - it;
                let buffer = convert_adc_to_str(it);
                Text::with_alignment(&buffer, Point::new(63,32), text_style, embedded_graphics::text::Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
                let _ = display.flush();
                let _ = display.clear(BinaryColor::Off);
            }
            Err(_) => {

            }
        }
    }
}
