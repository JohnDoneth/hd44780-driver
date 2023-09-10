use core::future::Future;
use embedded_hal_async::delay::DelayUs;

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;
pub use self::i2c::I2CBus;

use crate::error::Result;

pub trait DataBus {
	type WriteFuture<'a, D: 'a + DelayUs>: Future<Output = Result<()>>
	where
		Self: 'a;

	fn write<'a, D: DelayUs + 'a>(&'a mut self, byte: u8, data: bool, delay: &'a mut D) -> Self::WriteFuture<'a, D>;

	// TODO
	// fn read(...)
}
