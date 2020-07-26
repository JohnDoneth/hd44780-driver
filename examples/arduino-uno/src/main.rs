#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use hd44780_driver::HD44780;

use embedded_hal::blocking::delay;

use arduino_uno::Delay;


#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    //let mut led = pins.d13.into_output(&mut pins.ddr);

    //led.set_high().void_unwrap();

    //delay.delay_ms(500);

    let mut lcd = HD44780::new_8bit(
    
        pins.d12.into_output(&mut pins.ddr), // Register Select pin
        pins.d13.into_output(&mut pins.ddr), // Enable pin

        

        pins.d11.into_output(&mut pins.ddr),  // d0
        pins.d10.into_output(&mut pins.ddr),  // d1
        pins.d9.into_output(&mut pins.ddr),  // d2
        pins.d8.into_output(&mut pins.ddr),  // d3

        pins.d4.into_output(&mut pins.ddr),  // d4
        pins.d5.into_output(&mut pins.ddr),  // d5
        pins.d6.into_output(&mut pins.ddr),  // d6
        pins.d7.into_output(&mut pins.ddr),  // d7

        &mut delay,
    ).unwrap();

    // Unshift display and set cursor to 0
    lcd.reset(&mut delay).unwrap(); 
    
    // Clear existing characters
    lcd.clear(&mut delay).unwrap(); 

    // Display the following string
    lcd.write_str("Hello, world!", &mut delay).unwrap();
 
    loop { }

    // loop {
    //     led.toggle().void_unwrap();
    //     delay.delay_ms(200);
    //     led.toggle().void_unwrap();
    //     delay.delay_ms(200);
    //     led.toggle().void_unwrap();
    //     delay.delay_ms(200);
    //     led.toggle().void_unwrap();
    //     delay.delay_ms(2800);
    // }
}
