use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, Operation};

use crate::error::{Error, Port};
use crate::{bus::WritableDataBus, error::Result};

use super::ReadableDataBus;

pub struct I2CBus<I2C> {
	i2c_bus: I2C,
	address: u8,
}

const BACKLIGHT: u8 = 0b0000_1000;
const ENABLE: u8 = 0b0000_0100;
const READ: u8 = 0b0000_0010;
const REGISTER_SELECT: u8 = 0b0000_0001;

impl<I2C> I2CBus<I2C> {
	pub fn new(i2c_bus: I2C, address: u8) -> I2CBus<I2C> {
		I2CBus { i2c_bus, address }
	}

	pub fn destroy(self) -> I2C {
		self.i2c_bus
	}
}

impl<I2C: I2c> I2CBus<I2C> {
	/// Write a nibble to the lcd
	/// The nibble should be in the upper part of the byte
	fn write_nibble<D: DelayNs>(&mut self, nibble: u8, data: bool, delay: &mut D) -> Result<(), I2C::Error> {
		let rs = match data {
			false => 0u8,
			true => REGISTER_SELECT,
		};
		let byte = nibble | rs | BACKLIGHT;

		self.i2c_bus.write(self.address, &[byte, byte | ENABLE]).map_err(Error::wrap_io(Port::I2C))?;
		delay.delay_ms(2u32);
		self.i2c_bus.write(self.address, &[byte]).map_err(Error::wrap_io(Port::I2C))
	}

	/// Read a nibble from the lcd
	/// The nibble will be in the upper part of the byte
	fn read_nibble<D: DelayNs>(&mut self, data: bool, delay: &mut D) -> Result<u8, I2C::Error> {
		let rs = match data {
			false => 0u8,
			true => REGISTER_SELECT,
		};
		let byte = 0b11110000 | READ | BACKLIGHT | rs;

		let mut read_byte = 0;
		self.i2c_bus.write(self.address, &[byte, byte | ENABLE]).map_err(Error::wrap_io(Port::I2C))?;
		delay.delay_ms(2u32);
		self.i2c_bus
			.transaction(
				self.address,
				&mut [Operation::Read(core::slice::from_mut(&mut read_byte)), Operation::Write(&[byte])],
			)
			.map_err(Error::wrap_io(Port::I2C))?;

		Ok(read_byte)
	}
}

impl<I2C: I2c> WritableDataBus for I2CBus<I2C> {
	type Error = I2C::Error;

	fn write<D: DelayNs>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<(), Self::Error> {
		let upper_nibble = byte & 0xF0;
		self.write_nibble(upper_nibble, data, delay)?;

		let lower_nibble = (byte & 0x0F) << 4;
		self.write_nibble(lower_nibble, data, delay)?;

		Ok(())
	}
}

impl<I2C: I2c> ReadableDataBus for I2CBus<I2C> {
	type Error = I2C::Error;

	fn read<D: DelayNs>(&mut self, data: bool, delay: &mut D) -> Result<u8, Self::Error> {
		let upper_nibble = self.read_nibble(data, delay)?;
		let lower_nibble = self.read_nibble(data, delay)?;

		Ok((upper_nibble & 0xF0) | (lower_nibble >> 4))
	}
}

#[cfg(feature = "async")]
mod non_blocking {
	use core::future::Future;
	use embedded_hal_async::delay::DelayNs;
	use embedded_hal_async::i2c::I2c;

	use crate::{
		error::{Error, Port, Result},
		non_blocking::bus::DataBus,
	};

	use super::{I2CBus, BACKLIGHT, ENABLE, REGISTER_SELECT};

	impl<I2C: I2c> I2CBus<I2C> {
		/// Write a nibble to the lcd
		/// The nibble should be in the upper part of the byte
		async fn write_nibble_non_blocking<'a, D: DelayNs + 'a>(
			&mut self,
			nibble: u8,
			data: bool,
			delay: &'a mut D,
		) -> Result<(), I2C::Error> {
			let rs = match data {
				false => 0u8,
				true => REGISTER_SELECT,
			};
			let byte = nibble | rs | BACKLIGHT;

			self.i2c_bus.write(self.address, &[byte, byte | ENABLE]).await.map_err(Error::wrap_io(Port::I2C))?;
			delay.delay_ms(2).await;
			self.i2c_bus.write(self.address, &[byte]).await.map_err(Error::wrap_io(Port::I2C))
		}
	}

	impl<I2C: I2c + 'static> WritableDataBus for I2CBus<I2C> {
		type Error = I2C::Error;

		type WriteFuture<'a, D: 'a + DelayNs> = impl Future<Output = Result<(), Self::Error>> + 'a;

		fn write<'a, D: DelayNs + 'a>(
			&'a mut self,
			byte: u8,
			data: bool,
			delay: &'a mut D,
		) -> Self::WriteFuture<'a, D> {
			async move {
				let upper_nibble = byte & 0xF0;
				self.write_nibble_non_blocking(upper_nibble, data, delay).await?;

				let lower_nibble = (byte & 0x0F) << 4;
				self.write_nibble_non_blocking(lower_nibble, data, delay).await?;

				Ok(())
			}
		}
	}
}
