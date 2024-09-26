#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	let peripherals = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(peripherals);

	// Digital pin 13 is also connected to an onboard LED marked "L"
	let mut led = pins.d13.into_output();
	led.set_high();

	let mut serial = arduino_hal::default_serial!(peripherals, pins, 115200);

	loop {
		ufmt::uwriteln!(serial, "uwu").unwrap();
		led.toggle();
		arduino_hal::delay_ms(100);
		led.toggle();
		arduino_hal::delay_ms(100);
		led.toggle();
		arduino_hal::delay_ms(100);
		led.toggle();
		arduino_hal::delay_ms(800);
	}
}
