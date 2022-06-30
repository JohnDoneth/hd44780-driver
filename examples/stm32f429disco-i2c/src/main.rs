#![no_std]
#![no_main]

extern crate panic_halt;

use crate::hal::{pac::Peripherals, prelude::*};
use cortex_m_rt::entry;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use stm32f4xx_hal as hal;

const I2C_ADDRESS: u8 = 0x27;

#[entry]
fn main() -> ! {
	let cp = cortex_m::Peripherals::take().unwrap();
	let dp = Peripherals::take().unwrap();

	let rcc = dp.RCC.constrain();
	let gpiob = dp.GPIOB.split();

	let clocks = rcc.cfgr.sysclk(16.MHz()).pclk1(8.MHz()).freeze();
	let mut delay = cp.SYST.delay(&clocks);

	// NOTE: You cannot pick random pins, use valid options from the pinpack such as PB6/PB7
	let scl = gpiob.pb6.into_alternate::<4>().internal_pull_up(true).set_open_drain();
	let sda = gpiob.pb7.into_alternate::<4>().internal_pull_up(true).set_open_drain();

	let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);

	let mut lcd = HD44780::new_i2c(i2c, I2C_ADDRESS, &mut delay).expect("Init LCD failed");

	let _ = lcd.reset(&mut delay);
	let _ = lcd.clear(&mut delay);
	let _ = lcd.set_display_mode(
		DisplayMode { display: Display::On, cursor_visibility: Cursor::Visible, cursor_blink: CursorBlink::On },
		&mut delay,
	);
	let _ = lcd.write_str("Hello, STM32f4!", &mut delay);

	loop {}
}
