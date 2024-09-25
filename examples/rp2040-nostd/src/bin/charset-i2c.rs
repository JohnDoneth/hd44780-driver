#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use hd44780_driver::{
	charset::CharsetA00, memory_map::MemoryMap1602, setup::DisplayOptionsI2C, Cursor, CursorBlink, Direction, Display,
	DisplayMode, HD44780,
};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico::{
	self as bsp,
	hal::{fugit::RateExtU32, Timer, I2C},
	Gp18I2C1Sda, Gp19I2C1Scl,
};
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
	clocks::{init_clocks_and_plls, Clock},
	pac,
	sio::Sio,
	watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
	info!("Program start");
	let mut pac = pac::Peripherals::take().unwrap();
	let core = pac::CorePeripherals::take().unwrap();
	let mut watchdog = Watchdog::new(pac.WATCHDOG);
	let sio = Sio::new(pac.SIO);

	// External high-speed crystal on the pico board is 12Mhz
	let external_xtal_freq_hz = 12_000_000u32;
	let clocks = init_clocks_and_plls(
		external_xtal_freq_hz,
		pac.XOSC,
		pac.CLOCKS,
		pac.PLL_SYS,
		pac.PLL_USB,
		&mut pac.RESETS,
		&mut watchdog,
	)
	.ok()
	.unwrap();

	let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

	let pins = bsp::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

	let mut led_pin = pins.led.into_push_pull_output();
	led_pin.set_high().unwrap();

	// Reconfigure pins for I2C
	let sda: Gp18I2C1Sda = pins.gpio18.reconfigure();
	let scl: Gp19I2C1Scl = pins.gpio19.reconfigure();

	// Configure I2C interface
	let i2c = I2C::i2c1(pac.I2C1, sda, scl, 100.kHz(), &mut pac.RESETS, &clocks.system_clock);

	// Configure LCD driver with I2C
	let mut options = DisplayOptionsI2C::new(MemoryMap1602::new())
		.with_i2c_bus(i2c, 0x27)
		.with_charset(CharsetA00::QUESTION_FALLBACK);

	let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

	// Initialize LCD driver
	let mut display = loop {
		match HD44780::new(options, &mut timer) {
			Err((options_back, error)) => {
				error!("Error creating LCD Driver: {}", error);
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
			&mut timer,
		)
		.unwrap();

	display.clear(&mut timer).unwrap();
	display.reset(&mut timer).unwrap();

	display.write_str("Hello, world!", &mut timer).unwrap();
	display.set_cursor_xy((19, 1), &mut timer).unwrap();
	display.write_str("ハロー、ワールト゛！", &mut timer).unwrap();

	loop {
		// Scroll left and right
		delay.delay_ms(1000);
		for _ in 0..13 {
			display.shift_display(Direction::Left, &mut timer).unwrap();
			delay.delay_ms(100);
		}
		delay.delay_ms(1000);
		for _ in 0..13 {
			display.shift_display(Direction::Right, &mut timer).unwrap();
			delay.delay_ms(100);
		}
	}
}
