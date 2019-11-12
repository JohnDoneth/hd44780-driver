use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::OutputPin;

use bus::DataBus;

pub struct EightBitBus<
    RS: OutputPin,
    EN: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    rs: RS,
    en: EN,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    pub fn from_pins(
        rs: RS,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
        EightBitBus {
            rs,
            en,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
        }
    }

    fn set_bus_bits(&mut self, data: u8) {
        let db0: bool = (0b0000_0001 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;
        let db4: bool = (0b0001_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db7: bool = (0b1000_0000 & data) != 0;

        if db0 {
            let _ = self.d0.set_high();
        } else {
            let _ = self.d0.set_low();
        }

        if db1 {
            let _ = self.d1.set_high();
        } else {
            let _ = self.d1.set_low();
        }

        if db2 {
            let _ = self.d2.set_high();
        } else {
            let _ = self.d2.set_low();
        }

        if db3 {
            let _ = self.d3.set_high();
        } else {
            let _ = self.d3.set_low();
        }

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

impl<
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > DataBus for EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D) {
        if data {
            let _ = self.rs.set_high();
        } else {
            let _ = self.rs.set_low();
        }

        self.set_bus_bits(byte);

        let _ = self.en.set_high();
        delay.delay_ms(2u8);
        let _ = self.en.set_low();

        if data {
            let _ = self.rs.set_low();
        }
    }
}
