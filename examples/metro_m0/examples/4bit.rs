#![feature(used)]
#![no_std]

extern crate cortex_m_semihosting;

#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;

#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate atsamd21_hal;
extern crate metro_m0;

use metro_m0::clock::GenericClockController;
use metro_m0::delay::Delay;
use metro_m0::{CorePeripherals, Peripherals};

extern crate hd44780_driver;

use hd44780_driver::HD44780;

extern crate embedded_hal;

fn main() {
    let mut peripherals = Peripherals::take().unwrap();

    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::new(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = metro_m0::pins(peripherals.PORT);
    let delay = Delay::new(core.SYST, &mut clocks);

    let mut lcd = HD44780::new_4bit(

        pins.d4.into_open_drain_output(&mut pins.port), // Register Select pin
        pins.d3.into_open_drain_output(&mut pins.port), // Enable pin

        pins.d9.into_open_drain_output(&mut pins.port),  // d4
        pins.d10.into_open_drain_output(&mut pins.port), // d5
        pins.d11.into_open_drain_output(&mut pins.port), // d6
        pins.d12.into_open_drain_output(&mut pins.port), // d7

        delay,
    );

    // Unshift display and set cursor to 0
    lcd.reset();

    // Clear existing characters
    lcd.clear();

    // Display the following string
    lcd.write_str("Hello, world!");

    // Move the cursor to the second line
    lcd.set_cursor_pos(40);

    // Display the following string on the second line
    lcd.write_str("I'm on line 2!");

    loop { }
}
