use embedded_hal::delay::DelayNs;

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::EightBitBus;
pub use self::fourbit::FourBitBus;
pub use self::i2c::I2CBus;

use crate::error::Result;

pub trait DataBus {
	type Error: core::fmt::Debug;

	fn write<D: DelayNs>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<(), Self::Error>;

	// TODO
	// fn read(...)
}
