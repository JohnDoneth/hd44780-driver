#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::entry;
use hal::prelude::*;
use hal::flash::FlashExt;
use hal::i2c::I2c;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

// Connections:
// VSS: GND
// VDD: 5V
// SCL: PB6
// SDA: PB9
// I2C address : 0x3F

const I2C_ADDRESS: u8 = 0x3F;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = hal::delay::Delay::new(cp.SYST, clocks);

    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let mut lcd = HD44780::new_i2c(i2c, I2C_ADDRESS, delay);
    lcd.reset();
    lcd.clear();
    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Visible,
            cursor_blink: CursorBlink::On,
        }
    );
    let _ = lcd.write_str("Hello, world!");

    loop {}
}
