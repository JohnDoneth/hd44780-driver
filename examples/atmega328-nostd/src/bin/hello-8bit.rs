#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_hal::delay::DelayNs as _;
use hd44780_driver::{bus::{EightBitBusPins, WriteOnlyMode}, memory_map::MemoryMap1602, setup::DisplayOptions8Bit, HD44780};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	let peripherals = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(peripherals);

	// Setup USB Serial
	let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);

	let mut delay = Delay::new();

	ufmt::uwriteln!(serial, "Start").unwrap();

	// Configure LCD driver with 10 pins
	let options = DisplayOptions8Bit::new(MemoryMap1602::new()).with_pins(EightBitBusPins {
		rs: pins.d12.into_output(),
		rw: WriteOnlyMode,
		en: pins.d11.into_output(),

		d0: pins.d10.into_opendrain(),
		d1: pins.d9.into_opendrain(),
		d2: pins.d8.into_opendrain(),
		d3: pins.d7.into_opendrain(),
		d4: pins.d6.into_opendrain(),
		d5: pins.d5.into_opendrain(),
		d6: pins.d4.into_opendrain(),
		d7: pins.d3.into_opendrain(),
	});

	// Initialize LCD driver
	// Note: IO Error is infallible, thus unwrapping won't panic here
	let mut display = HD44780::new(options, &mut delay).unwrap_or_else(|_| unreachable!());

	display.clear(&mut delay).unwrap();
	display.reset(&mut delay).unwrap();

	display.write_str("Hello, world!", &mut delay).unwrap();

	loop {
		delay.delay_ms(1000);
	}
}
