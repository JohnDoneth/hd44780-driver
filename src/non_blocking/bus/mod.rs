use core::future::Future;
use embassy_traits::delay::Delay;

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;
pub use self::i2c::I2CBus;

use crate::error::Result;

pub trait DataBus {
    type WriteFuture<'a, D: 'a>: Future<Output = Result<()>>;

    fn write<'a, D: Delay + 'a>(
        &'a mut self,
        byte: u8,
        data: bool,
        delay: &'a mut D,
    ) -> Self::WriteFuture<'a, D>;

    // TODO
    // fn read(...)
}
