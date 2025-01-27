use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::{delay::DelayNs, digital};

use crate::sealed::Internal;
use crate::{
	bus::WritableDataBus,
	error::{Error, Port, Result},
};

use super::{ReadSelect, ReadableDataBus, WriteSelect};

#[derive(Debug, Clone, Copy)]
pub struct EightBitBusPins<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
	pub rs: RS,
	pub rw: RW,
	pub en: EN,
	pub d0: D0,
	pub d1: D1,
	pub d2: D2,
	pub d3: D3,
	pub d4: D4,
	pub d5: D5,
	pub d6: D6,
	pub d7: D7,
}

#[derive(Debug)]
pub struct EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
	pins: EightBitBusPins<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>,
}

impl<
		RS: OutputPin<Error = E>,
		RW: WriteSelect<E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E>,
		D1: OutputPin<Error = E>,
		D2: OutputPin<Error = E>,
		D3: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E,
	> EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	pub fn from_pins(
		pins: EightBitBusPins<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>,
	) -> EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
		EightBitBus { pins }
	}

	pub fn destroy(self) -> EightBitBusPins<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
		self.pins
	}

	fn set_bus_bits(&mut self, data: u8) -> Result<(), E> {
		let db0: bool = (0b0000_0001 & data) != 0;
		let db1: bool = (0b0000_0010 & data) != 0;
		let db2: bool = (0b0000_0100 & data) != 0;
		let db3: bool = (0b0000_1000 & data) != 0;
		let db4: bool = (0b0001_0000 & data) != 0;
		let db5: bool = (0b0010_0000 & data) != 0;
		let db6: bool = (0b0100_0000 & data) != 0;
		let db7: bool = (0b1000_0000 & data) != 0;

		self.pins.d0.set_state(db0.into()).map_err(Error::wrap_io(Port::D0))?;
		self.pins.d1.set_state(db1.into()).map_err(Error::wrap_io(Port::D1))?;
		self.pins.d2.set_state(db2.into()).map_err(Error::wrap_io(Port::D2))?;
		self.pins.d3.set_state(db3.into()).map_err(Error::wrap_io(Port::D3))?;
		self.pins.d4.set_state(db4.into()).map_err(Error::wrap_io(Port::D4))?;
		self.pins.d5.set_state(db5.into()).map_err(Error::wrap_io(Port::D5))?;
		self.pins.d6.set_state(db6.into()).map_err(Error::wrap_io(Port::D6))?;
		self.pins.d7.set_state(db7.into()).map_err(Error::wrap_io(Port::D7))?;

		Ok(())
	}
}

impl<
		RS: OutputPin<Error = E>,
		RW,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E> + InputPin<Error = E>,
		D1: OutputPin<Error = E> + InputPin<Error = E>,
		D2: OutputPin<Error = E> + InputPin<Error = E>,
		D3: OutputPin<Error = E> + InputPin<Error = E>,
		D4: OutputPin<Error = E> + InputPin<Error = E>,
		D5: OutputPin<Error = E> + InputPin<Error = E>,
		D6: OutputPin<Error = E> + InputPin<Error = E>,
		D7: OutputPin<Error = E> + InputPin<Error = E>,
		E,
	> EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	fn get_bus_bits(&mut self) -> Result<u8, E> {
		let mut bits = 0u8;
		bits |= self.pins.d0.is_high().map_err(Error::wrap_io(Port::D0))? as u8;
		bits |= (self.pins.d1.is_high().map_err(Error::wrap_io(Port::D1))? as u8) << 1;
		bits |= (self.pins.d2.is_high().map_err(Error::wrap_io(Port::D2))? as u8) << 2;
		bits |= (self.pins.d3.is_high().map_err(Error::wrap_io(Port::D3))? as u8) << 3;
		bits |= (self.pins.d4.is_high().map_err(Error::wrap_io(Port::D4))? as u8) << 4;
		bits |= (self.pins.d5.is_high().map_err(Error::wrap_io(Port::D5))? as u8) << 5;
		bits |= (self.pins.d6.is_high().map_err(Error::wrap_io(Port::D6))? as u8) << 6;
		bits |= (self.pins.d7.is_high().map_err(Error::wrap_io(Port::D7))? as u8) << 7;
		Ok(bits)
	}
}

impl<
		RS: OutputPin<Error = E>,
		RW: WriteSelect<E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E>,
		D1: OutputPin<Error = E>,
		D2: OutputPin<Error = E>,
		D3: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> WritableDataBus for EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	type Error = E;

	fn write<D: DelayNs>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<(), Self::Error> {
		self.pins.rs.set_state(data.into()).map_err(Error::wrap_io(Port::RS))?;

		self.set_bus_bits(byte)?;

		self.pins.en.set_high().map_err(Error::wrap_io(Port::EN))?;
		delay.delay_ms(2u32);
		self.pins.en.set_low().map_err(Error::wrap_io(Port::EN))?;

		Ok(())
	}
}

impl<
		RS: OutputPin<Error = E>,
		RW: WriteSelect<E> + ReadSelect<E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E> + InputPin<Error = E>,
		D1: OutputPin<Error = E> + InputPin<Error = E>,
		D2: OutputPin<Error = E> + InputPin<Error = E>,
		D3: OutputPin<Error = E> + InputPin<Error = E>,
		D4: OutputPin<Error = E> + InputPin<Error = E>,
		D5: OutputPin<Error = E> + InputPin<Error = E>,
		D6: OutputPin<Error = E> + InputPin<Error = E>,
		D7: OutputPin<Error = E> + InputPin<Error = E>,
		E: digital::Error,
	> ReadableDataBus for EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	type Error = E;

	fn read<D: DelayNs>(&mut self, data: bool, delay: &mut D) -> Result<u8, Self::Error> {
		self.pins.rs.set_state(data.into()).map_err(Error::wrap_io(Port::RS))?;

		self.set_bus_bits(0xff)?;
		self.pins.rw.select_read(Internal).map_err(Error::wrap_io(Port::RW))?;

		self.pins.en.set_high().map_err(Error::wrap_io(Port::EN))?;
		delay.delay_ms(2u32);
		let read_byte = self.get_bus_bits()?;
		self.pins.en.set_low().map_err(Error::wrap_io(Port::EN))?;

		self.pins.rw.select_write(Internal).map_err(Error::wrap_io(Port::RW))?;

		Ok(read_byte)
	}
}

#[cfg(feature = "async")]
mod non_blocking {
	use core::future::Future;
	use embedded_hal::digital::{self, InputPin, OutputPin};
	use embedded_hal_async::delay::DelayNs;

	use crate::{
		bus::{ReadSelect, WriteSelect},
		error::{Error, Port, Result},
		non_blocking::bus::{ReadableDataBus, WritableDataBus},
		sealed::Internal,
	};

	use super::EightBitBus;

	impl<
			RS: OutputPin<Error = E> + 'static,
			RW: WriteSelect<E> + 'static,
			EN: OutputPin<Error = E> + 'static,
			D0: OutputPin<Error = E> + 'static,
			D1: OutputPin<Error = E> + 'static,
			D2: OutputPin<Error = E> + 'static,
			D3: OutputPin<Error = E> + 'static,
			D4: OutputPin<Error = E> + 'static,
			D5: OutputPin<Error = E> + 'static,
			D6: OutputPin<Error = E> + 'static,
			D7: OutputPin<Error = E> + 'static,
			E: digital::Error,
		> WritableDataBus for EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
	{
		type Error = E;

		type Future<'a, D: 'a + DelayNs> = impl Future<Output = Result<(), Self::Error>> + 'a;

		fn write<'a, D: DelayNs + 'a>(&'a mut self, byte: u8, data: bool, delay: &'a mut D) -> Self::Future<'a, D> {
			async move {
				self.pins.rs.set_state(data.into()).map_err(Error::wrap_io(Port::RS))?;

				self.set_bus_bits(byte)?;

				self.pins.en.set_high().map_err(Error::wrap_io(Port::EN))?;
				delay.delay_ms(2).await;
				self.pins.en.set_low().map_err(Error::wrap_io(Port::EN))?;

				if data {
					self.pins.rs.set_low().map_err(Error::wrap_io(Port::RS))?;
				}

				Ok(())
			}
		}
	}

	impl<
			RS: OutputPin<Error = E> + 'static,
			RW: WriteSelect<E> + ReadSelect<E> + 'static,
			EN: OutputPin<Error = E> + 'static,
			D0: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D1: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D2: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D3: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D4: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D5: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D6: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			D7: OutputPin<Error = E> + InputPin<Error = E> + 'static,
			E: digital::Error,
		> ReadableDataBus for EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
	{
		type Error = E;

		type Future<'a, D: 'a + DelayNs> = impl Future<Output = Result<u8, Self::Error>> + 'a;

		fn read<'a, D: DelayNs + 'a>(&'a mut self, data: bool, delay: &'a mut D) -> Self::Future<'a, D> {
			async move {
				self.pins.rs.set_state(data.into()).map_err(Error::wrap_io(Port::RS))?;

				self.set_bus_bits(0xff)?;
				self.pins.rw.select_read(Internal).map_err(Error::wrap_io(Port::RW))?;

				self.pins.en.set_high().map_err(Error::wrap_io(Port::EN))?;
				delay.delay_ms(2u32).await;
				let read_byte = self.get_bus_bits()?;
				self.pins.en.set_low().map_err(Error::wrap_io(Port::EN))?;

				self.pins.rw.select_write(Internal).map_err(Error::wrap_io(Port::RW))?;

				Ok(read_byte)
			}
		}
	}
}
