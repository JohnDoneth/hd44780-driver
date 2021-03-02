use core::future::Future;
use core::pin::Pin;
use embassy_traits::delay::Delay;

mod eightbit;
mod fourbit;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;

use crate::error::Result;

pub trait DataBus {
    type WriteFuture<'a, D: 'a>: Future<Output = Result<()>>;

    fn write<'a, D: Delay + 'a>(
        &'a mut self,
        byte: u8,
        data: bool,
        delay: Pin<&'a mut D>,
    ) -> Self::WriteFuture<'a, D>;

    // TODO
    // fn read(...)
}
