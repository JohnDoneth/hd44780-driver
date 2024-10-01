use core::future::Future;
use embedded_hal_async::delay::DelayNs;

pub use crate::bus::I2CBus;
pub use crate::bus::{EightBitBus, EightBitBusPins};
pub use crate::bus::{FourBitBus, FourBitBusPins};

use crate::error::Result;

pub trait WritableDataBus {
	type Error: core::fmt::Debug;

	type Future<'a, D: 'a + DelayNs>: Future<Output = Result<(), Self::Error>>
	where
		Self: 'a;

	fn write<'a, D: DelayNs + 'a>(&'a mut self, byte: u8, data: bool, delay: &'a mut D) -> Self::Future<'a, D>;
}

pub trait ReadableDataBus {
	type Error: core::fmt::Debug;

	type Future<'a, D: 'a + DelayNs>: Future<Output = Result<u8, Self::Error>>
	where
		Self: 'a;

	fn read<'a, D: DelayNs + 'a>(&'a mut self, data: bool, delay: &'a mut D) -> Self::Future<'a, D>;
}
