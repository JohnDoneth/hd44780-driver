#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_hal::delay::DelayNs as _;
use hd44780_driver::{memory_map::MemoryMap1602, setup::DisplayOptionsI2C, HD44780};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	let peripherals = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(peripherals);

	// Setup USB Serial
	let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);

	let mut delay = Delay::new();

	ufmt::uwriteln!(serial, "Start\r").unwrap();

	// Configure I2C interface
	let i2c =
		arduino_hal::I2c::new(peripherals.TWI, pins.a4.into_pull_up_input(), pins.a5.into_pull_up_input(), 100_000);

	// Configure LCD driver with I2C
	let mut options = DisplayOptionsI2C::new(MemoryMap1602::new()).with_i2c_bus(i2c, 0x27);

	// Initialize LCD driver
	let mut display = loop {
		match HD44780::new(options, &mut delay) {
			Err((options_back, error)) => {
				ufmt::uwriteln!(serial, "Error creating LCD Driver: {}", error).unwrap();
				options = options_back;
				delay.delay_ms(500);
				// try again
			}
			Ok(display) => break display,
		}
	};

	display.clear(&mut delay).unwrap();
	display.reset(&mut delay).unwrap();

	display.write_str("Hello, world!", &mut delay).unwrap();
	let status = display.read_status(&mut delay).unwrap();
	ufmt::uwriteln!(serial, "{:?}\r", status).unwrap();

	loop {
		delay.delay_ms(1000);
	}
}
