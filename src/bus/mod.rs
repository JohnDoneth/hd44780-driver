use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

mod eightbit;
mod fourbit;
mod i2c;

pub use self::eightbit::{EightBitBus, EightBitBusPins};
pub use self::fourbit::{FourBitBus, FourBitBusPins};
pub use self::i2c::I2CBus;

use crate::error::Result;
use crate::sealed::Internal;

pub trait WritableDataBus {
	type Error: core::fmt::Debug;

	fn write<D: DelayNs>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<(), Self::Error>;
}

pub trait ReadableDataBus {
	type Error: core::fmt::Debug;

	fn read<D: DelayNs>(&mut self, data: bool, delay: &mut D) -> Result<u8, Self::Error>;
}

mod sealed {
	use crate::sealed::Internal;

	#[doc(hidden)]
	pub trait SealedWriteSelect<E>: Sized {
		fn select_write(&mut self, _: Internal) -> Result<(), E>;
	}

	#[doc(hidden)]
	pub trait SealedReadSelect<E>: Sized {
		fn select_read(&mut self, _: Internal) -> Result<(), E>;
	}
}

pub trait WriteSelect<E>: sealed::SealedWriteSelect<E> {}
pub trait ReadSelect<E>: sealed::SealedReadSelect<E> {}

impl<P> WriteSelect<P::Error> for P where P: OutputPin {}
impl<P> ReadSelect<P::Error> for P where P: OutputPin {}

impl<P> sealed::SealedWriteSelect<P::Error> for P
where
	P: OutputPin,
{
	fn select_write(&mut self, _: Internal) -> core::result::Result<(), P::Error> {
		self.set_low()
	}
}

impl<P> sealed::SealedReadSelect<P::Error> for P
where
	P: OutputPin,
{
	fn select_read(&mut self, _: Internal) -> core::result::Result<(), P::Error> {
		self.set_high()
	}
}

/// This can be used instead of a pin for RW in four and eight bit bus configurations
/// to specify that the RW pin of the LCD has been pulled low permanently.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct WriteOnlyMode;

impl<E> WriteSelect<E> for WriteOnlyMode {}

impl<E> sealed::SealedWriteSelect<E> for WriteOnlyMode {
	fn select_write(&mut self, _: Internal) -> core::result::Result<(), E> {
		Ok(())
	}
}
