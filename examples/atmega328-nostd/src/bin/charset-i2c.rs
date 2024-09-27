#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_hal::delay::DelayNs as _;
use hd44780_driver::{
	charset::CharsetA00, memory_map::MemoryMap1602, setup::DisplayOptionsI2C, Cursor, CursorBlink, Direction, Display,
	DisplayMode, HD44780,
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	let peripherals = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(peripherals);

	// Setup USB Serial
	let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);

	let mut delay = Delay::new();

	ufmt::uwriteln!(serial, "Start").unwrap();

	// Configure I2C interface
	let i2c =
		arduino_hal::I2c::new(peripherals.TWI, pins.a4.into_pull_up_input(), pins.a5.into_pull_up_input(), 100_000);

	// Configure LCD driver with I2C
	let mut options = DisplayOptionsI2C::new(MemoryMap1602::new())
		.with_i2c_bus(i2c, 0x27)
		.with_charset(CharsetA00::QUESTION_FALLBACK);

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

	// Disable cursor
	display
		.set_display_mode(
			DisplayMode { display: Display::On, cursor_visibility: Cursor::Invisible, cursor_blink: CursorBlink::Off },
			&mut delay,
		)
		.unwrap();

	display.clear(&mut delay).unwrap();
	display.reset(&mut delay).unwrap();

	display.write_str("Hello, world!", &mut delay).unwrap();
	display.set_cursor_xy((19, 1), &mut delay).unwrap();
	display.write_str("ハロー、ワールト゛！", &mut delay).unwrap();

	loop {
		// Scroll left and right
		delay.delay_ms(1000);
		for _ in 0..13 {
			display.shift_display(Direction::Left, &mut delay).unwrap();
			delay.delay_ms(100);
		}
		delay.delay_ms(1000);
		for _ in 0..13 {
			display.shift_display(Direction::Right, &mut delay).unwrap();
			delay.delay_ms(100);
		}
	}
}
