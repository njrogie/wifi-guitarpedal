use esp_hal::{
    clock::ClockControl, i2c::I2C, peripherals::{
        Peripherals, 
        I2C1, WIFI
    }, prelude::*, Delay, IO
};

use ssd1306::{
    mode::BufferedGraphicsMode, 
    prelude::I2CInterface, 
    rotation::DisplayRotation, 
    size::DisplaySize128x64, 
    I2CDisplayInterface, 
    Ssd1306
};

pub struct ESP32WROOM32LCDBoardUtils {
    pub i2c: I2C<'static, I2C1>,
    pub delay: Delay,
    pub wifi: WIFI
}

impl ESP32WROOM32LCDBoardUtils {
    pub fn new(peripherals: Peripherals) -> ESP32WROOM32LCDBoardUtils {
        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

        let i2c = I2C::new(peripherals.I2C1,io.pins.gpio5, io.pins.gpio4, _fugit_RateExtU32::kHz(100), &clocks);
        let delay = Delay::new(&clocks);
        let wifi = peripherals.WIFI;

        ESP32WROOM32LCDBoardUtils {
            i2c: i2c,
            delay: delay,
            wifi: wifi
        }
    }
}

pub fn get_display (i2c: I2C<'static, I2C1>) -> Ssd1306<I2CInterface<I2C<'static, I2C1>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>{
    let interface = I2CDisplayInterface::new(i2c);

    return Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
}

pub fn get_wifi(wifi: WIFI)  {

}