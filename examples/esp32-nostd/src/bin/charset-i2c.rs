#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
	clock::ClockControl, delay::Delay, gpio::Io, i2c::I2C, peripherals::Peripherals, prelude::*, system::SystemControl,
};
use hd44780_driver::{
	charset::CharsetA00, memory_map::MemoryMap1602, setup::DisplayOptionsI2C, Cursor, CursorBlink, Direction, Display,
	DisplayMode, HD44780,
};
use log::{error, info};

#[entry]
fn main() -> ! {
	esp_println::logger::init_logger(log::LevelFilter::Debug);
	let peripherals = Peripherals::take();
	let system = SystemControl::new(peripherals.SYSTEM);
	let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

	let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
	let mut delay = Delay::new(&clocks);

	info!("Start");

	// Configure I2C interface
	let i2c = I2C::new(peripherals.I2C0, io.pins.gpio21, io.pins.gpio22, 100.kHz(), &clocks);

	// Configure LCD driver with I2C
	let mut options = DisplayOptionsI2C::new(MemoryMap1602::new())
		.with_i2c_bus(i2c, 0x27)
		.with_charset(CharsetA00::QUESTION_FALLBACK);

	// Initialize LCD driver
	let mut display = loop {
		match HD44780::new(options, &mut delay) {
			Err((options_back, error)) => {
				error!("Error creating LCD Driver: {error}");
				options = options_back;
				delay.delay_millis(500);
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
		delay.delay_millis(1000);
		for _ in 0..13 {
			display.shift_display(Direction::Left, &mut delay).unwrap();
			delay.delay_millis(100);
		}
		delay.delay_millis(1000);
		for _ in 0..13 {
			display.shift_display(Direction::Right, &mut delay).unwrap();
			delay.delay_millis(100);
		}
	}
}
