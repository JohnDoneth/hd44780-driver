use embedded_hal::blocking::delay::{DelayMs, DelayUs};

mod eightbit;
mod fourbit;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;

use error::Result;

pub trait DataBus {

    fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<()>;

    // TODO
    // fn read(...)
}
