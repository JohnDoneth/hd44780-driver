use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write;

use crate::{bus::DataBus, error::Result};

pub struct I2CBus<I2C: Write> {
    i2c_bus: I2C,
    address: u8,
}

const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8 = 0b0000_0100;
// const READ_WRITE: u8 = 0b0000_0010; // Not used as no reading of the `HD44780` is done
const REGISTER_SELECT: u8 = 0b0000_0001;

impl<I2C: Write> I2CBus<I2C> {
    pub fn new(i2c_bus: I2C, address: u8) -> I2CBus<I2C> {
        I2CBus { i2c_bus, address }
    }

    /// Write a nibble to the lcd
    /// The nibble should be in the upper part of the byte
    fn write_nibble<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        nibble: u8,
        data: bool,
        delay: &mut D,
    ) {
        let rs = match data {
            false => 0u8,
            true => REGISTER_SELECT,
        };
        let byte = nibble | rs | BACKLIGHT;

        let _ = self.i2c_bus.write(self.address, &[byte, byte | ENABLE]);
        delay.delay_ms(2u8);
        let _ = self.i2c_bus.write(self.address, &[byte]);
    }
}

impl<I2C: Write> DataBus for I2CBus<I2C> {
    fn write<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        byte: u8,
        data: bool,
        delay: &mut D,
    ) -> Result<()> {
        let upper_nibble = byte & 0xF0;
        self.write_nibble(upper_nibble, data, delay);

        let lower_nibble = (byte & 0x0F) << 4;
        self.write_nibble(lower_nibble, data, delay);

        Ok(())
    }
}
