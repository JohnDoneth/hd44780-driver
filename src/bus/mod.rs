use embedded_hal::blocking::delay::{DelayMs, DelayUs};

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;
pub use self::i2c::I2CBus;

use crate::error::Result;

pub trait DataBus {
    fn write<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        byte: u8,
        data: bool,
        delay: &mut D,
    ) -> Result<()>;

    // TODO
    // fn read(...)
}
