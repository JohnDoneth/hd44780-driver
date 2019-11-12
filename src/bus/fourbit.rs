use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::OutputPin;

use bus::DataBus;

pub struct FourBitBus<
    RS: OutputPin,
    EN: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    rs: RS,
    en: EN,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<RS: OutputPin, EN: OutputPin, D4: OutputPin, D5: OutputPin, D6: OutputPin, D7: OutputPin>
    FourBitBus<RS, EN, D4, D5, D6, D7>
{
    pub fn from_pins(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> FourBitBus<RS, EN, D4, D5, D6, D7> {
        FourBitBus {
            rs,
            en,
            d4,
            d5,
            d6,
            d7,
        }
    }

    fn write_lower_nibble(&mut self, data: u8) {
        let db0: bool = (0b0000_0001 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;

        if db0 {
            let _ = self.d4.set_high();
        } else {
            let _ = self.d4.set_low();
        }

        if db1 {
            let _ = self.d5.set_high();
        } else {
            let _ = self.d5.set_low();
        }

        if db2 {
            let _ = self.d6.set_high();
        } else {
            let _ = self.d6.set_low();
        }

        if db3 {
            let _ = self.d7.set_high();
        } else {
            let _ = self.d7.set_low();
        }
    }

    fn write_upper_nibble(&mut self, data: u8) {
        let db4: bool = (0b0001_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db7: bool = (0b1000_0000 & data) != 0;

        if db4 {
            let _ = self.d4.set_high();
        } else {
            let _ = self.d4.set_low();
        }

        if db5 {
            let _ = self.d5.set_high();
        } else {
            let _ = self.d5.set_low();
        }

        if db6 {
            let _ = self.d6.set_high();
        } else {
            let _ = self.d6.set_low();
        }

        if db7 {
            let _ = self.d7.set_high();
        } else {
            let _ = self.d7.set_low();
        }
    }
}

impl<RS: OutputPin, EN: OutputPin, D4: OutputPin, D5: OutputPin, D6: OutputPin, D7: OutputPin>
    DataBus for FourBitBus<RS, EN, D4, D5, D6, D7>
{
    fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D) {
        if data {
            let _ = self.rs.set_high();
        } else {
            let _ = self.rs.set_low();
        }

        self.write_upper_nibble(byte);

        // Pulse the enable pin to recieve the upper nibble
        let _ = self.en.set_high();
        delay.delay_ms(2u8);
        let _ = self.en.set_low();

        self.write_lower_nibble(byte);

        // Pulse the enable pin to recieve the lower nibble
        let _ = self.en.set_high();
        delay.delay_ms(2u8);
        let _ = self.en.set_low();

        if data {
            let _ =  self.rs.set_low();
        }
    }
}
