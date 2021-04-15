use core::future::Future;
use embassy_traits::delay::Delay;
use embassy_traits::i2c::I2c;

use crate::error::Result;
use crate::non_blocking::bus::DataBus;

pub struct I2CBus<I2C: I2c> {
    i2c_bus: I2C,
    address: u8,
}

const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8 = 0b0000_0100;
// const READ_WRITE: u8 = 0b0000_0010; // Not used as no reading of the `HD44780` is done
const REGISTER_SELECT: u8 = 0b0000_0001;

impl<I2C: I2c> I2CBus<I2C> {
    pub fn new(i2c_bus: I2C, address: u8) -> I2CBus<I2C> {
        I2CBus { i2c_bus, address }
    }

    /// Write a nibble to the lcd
    /// The nibble should be in the upper part of the byte
    async fn write_nibble<'a, D: Delay + 'a>(&mut self, nibble: u8, data: bool, delay: &'a mut D) {
        let rs = match data {
            false => 0u8,
            true => REGISTER_SELECT,
        };
        let byte = nibble | rs | BACKLIGHT;

        let _ = self
            .i2c_bus
            .write(self.address, &[byte, byte | ENABLE])
            .await;
        delay.delay_ms(2u8 as u64).await;
        let _ = self.i2c_bus.write(self.address, &[byte]).await;
    }
}

impl<I2C: I2c + 'static> DataBus for I2CBus<I2C> {
    type WriteFuture<'a, D: 'a> = impl Future<Output = Result<()>> + 'a;

    fn write<'a, D: Delay + 'a>(
        &'a mut self,
        byte: u8,
        data: bool,
        delay: &'a mut D,
    ) -> Self::WriteFuture<'a, D> {
        async move {
            let upper_nibble = byte & 0xF0;
            self.write_nibble(upper_nibble, data, delay).await;

            let lower_nibble = (byte & 0x0F) << 4;
            self.write_nibble(lower_nibble, data, delay).await;

            Ok(())
        }
    }
}
