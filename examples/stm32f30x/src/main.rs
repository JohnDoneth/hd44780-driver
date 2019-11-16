#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use hal::gpio::GpioExt;
use hal::flash::FlashExt;
use hal::rcc::RccExt;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use core::fmt::Write;


// Connections:
// VSS: GND
// VDD: 5V
// V0:  10k poti between 5V and GND
// RS:  PD1
// RW:  GND
// E:   PD2
// D4-D7: PD4-PD7
// A:   5V
// K:   GND

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = hal::delay::Delay::new(cp.SYST, clocks);

    let rs = gpiod.pd1.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let en = gpiod.pd2.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let b4 = gpiod.pd4.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let b5 = gpiod.pd5.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let b6 = gpiod.pd6.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let b7 = gpiod.pd7.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);

    if let Ok(mut lcd) = HD44780::new_4bit(rs, en, b4, b5, b6, b7, delay) {
        let _ = lcd.reset();
        let _ = lcd.clear();
        let _ = lcd.set_display_mode(
            DisplayMode {
                display: Display::On,
                cursor_visibility: Cursor::Visible,
                cursor_blink: CursorBlink::On,
            }
        );
        lcd.write_str("Hello, world!").unwrap();
    }

    loop {}
}
