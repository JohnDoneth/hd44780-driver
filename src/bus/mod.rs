use embedded_hal::delay::DelayNs;

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::{EightBitBus, EightBitBusPins};
pub use self::fourbit::{FourBitBus, FourBitBusPins};
pub use self::i2c::I2CBus;

use crate::error::Result;

pub trait WritableDataBus {
	type Error: core::fmt::Debug;

	fn write<D: DelayNs>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<(), Self::Error>;
}

pub trait ReadableDataBus {
	type Error: core::fmt::Debug;

	fn read<D: DelayNs>(&mut self, data: bool, delay: &mut D) -> Result<u8, Self::Error>;
}
