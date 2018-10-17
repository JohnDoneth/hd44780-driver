extern crate linux_embedded_hal;
extern crate hd44780_driver;

use linux_embedded_hal::{Delay, Pin};
use linux_embedded_hal::sysfs_gpio::Direction;

use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};

fn main() {

    let rs = Pin::new(26);
    let en = Pin::new(22);

    let db0 = Pin::new(19);
    let db1 = Pin::new(13);
    let db2 = Pin::new(6);
    let db3 = Pin::new(5);
    let db4 = Pin::new(21);
    let db5 = Pin::new(20);
    let db6 = Pin::new(16);
    let db7 = Pin::new(12);

    rs.export().unwrap();
    en.export().unwrap();
    
    db0.export().unwrap();
    db1.export().unwrap();
    db2.export().unwrap();
    db3.export().unwrap();
    db4.export().unwrap();
    db5.export().unwrap();
    db6.export().unwrap();
    db7.export().unwrap();

    rs.set_direction(Direction::Low).unwrap();
    en.set_direction(Direction::Low).unwrap();
    
    db0.set_direction(Direction::Low).unwrap();
    db1.set_direction(Direction::Low).unwrap();
    db2.set_direction(Direction::Low).unwrap();
    db3.set_direction(Direction::Low).unwrap();
    db4.set_direction(Direction::Low).unwrap();
    db5.set_direction(Direction::Low).unwrap();
    db6.set_direction(Direction::Low).unwrap();
    db7.set_direction(Direction::Low).unwrap();

    let mut lcd = HD44780::new_8bit(
        rs,
        en,
        db0,
        db1,
        db2,
        db3,
        db4,
        db5,
        db6,
        db7,
        Delay,
    );

    lcd.reset();
    
    lcd.clear();

    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Visible,
            cursor_blink: CursorBlink::On,
        }
    );
    
    lcd.write_str("Hello, world!");

}
