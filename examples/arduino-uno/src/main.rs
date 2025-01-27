#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::Delay;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use numtoa::NumToA;

// This seems to be a magic number
// https://docs.rs/hd44780-driver/0.4.0/hd44780_driver/struct.HD44780.html#method.set_cursor_pos
const LINE2_POSITION: u8 = 40;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut delay = Delay::new();
    let d2 = pins.d2.into_output(&mut pins.ddr);
    let d3 = pins.d3.into_output(&mut pins.ddr);
    let d4 = pins.d4.into_output(&mut pins.ddr);
    let d5 = pins.d5.into_output(&mut pins.ddr);
    let d11 = pins.d11.into_output(&mut pins.ddr); // enable
    let d12 = pins.d12.into_output(&mut pins.ddr); // rs

    let display_mode = DisplayMode {
        cursor_visibility: Cursor::Invisible,
        cursor_blink: CursorBlink::Off,
        display: Display::On,
    };
    let mut lcd = HD44780::new_4bit(d12, d11, d5, d4, d3, d2, &mut delay).unwrap();

    // Configure LCD
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();
    lcd.set_display_mode(display_mode, &mut delay).unwrap();

    // Write first line
    lcd.set_cursor_pos(0, &mut delay).unwrap();
    lcd.write_str("Hello, rust!", &mut delay).unwrap();
    let mut seconds: u16 = 0;

    // Buffer for second line
    let mut buffer = [0u8; 16];

    loop {
        // Write second line
        lcd.set_cursor_pos(LINE2_POSITION, &mut delay).unwrap();
        let line2_bytes = seconds.numtoa(10, &mut buffer);
        lcd.write_bytes(line2_bytes, &mut delay).unwrap();

        // Sleep
        arduino_uno::delay_ms(1000);
        seconds += 1;
    }
}
