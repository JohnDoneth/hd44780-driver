use core::future::Future;
use embedded_hal::digital::{self, OutputPin};
use embedded_hal_async::delay::DelayNs;

use crate::error::{Error, Port, Result};
use crate::non_blocking::bus::DataBus;

pub struct FourBitBus<RS: OutputPin, EN: OutputPin, D4: OutputPin, D5: OutputPin, D6: OutputPin, D7: OutputPin> {
	rs: RS,
	en: EN,
	d4: D4,
	d5: D5,
	d6: D6,
	d7: D7,
}

impl<
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E,
	> FourBitBus<RS, EN, D4, D5, D6, D7>
{
	pub fn from_pins(rs: RS, en: EN, d4: D4, d5: D5, d6: D6, d7: D7) -> FourBitBus<RS, EN, D4, D5, D6, D7> {
		FourBitBus { rs, en, d4, d5, d6, d7 }
	}

	fn write_lower_nibble(&mut self, data: u8) -> Result<(), E> {
		let db0: bool = (0b0000_0001 & data) != 0;
		let db1: bool = (0b0000_0010 & data) != 0;
		let db2: bool = (0b0000_0100 & data) != 0;
		let db3: bool = (0b0000_1000 & data) != 0;

		self.d4.set_state(db0.into()).map_err(Error::wrap_io(Port::D4))?;
		self.d5.set_state(db1.into()).map_err(Error::wrap_io(Port::D5))?;
		self.d6.set_state(db2.into()).map_err(Error::wrap_io(Port::D6))?;
		self.d7.set_state(db3.into()).map_err(Error::wrap_io(Port::D7))?;

		Ok(())
	}

	fn write_upper_nibble(&mut self, data: u8) -> Result<(), E> {
		let db4: bool = (0b0001_0000 & data) != 0;
		let db5: bool = (0b0010_0000 & data) != 0;
		let db6: bool = (0b0100_0000 & data) != 0;
		let db7: bool = (0b1000_0000 & data) != 0;

		self.d4.set_state(db4.into()).map_err(Error::wrap_io(Port::D4))?;
		self.d5.set_state(db5.into()).map_err(Error::wrap_io(Port::D5))?;
		self.d6.set_state(db6.into()).map_err(Error::wrap_io(Port::D6))?;
		self.d7.set_state(db7.into()).map_err(Error::wrap_io(Port::D7))?;

		Ok(())
	}
}

impl<
		RS: OutputPin<Error = E> + 'static,
		EN: OutputPin<Error = E> + 'static,
		D4: OutputPin<Error = E> + 'static,
		D5: OutputPin<Error = E> + 'static,
		D6: OutputPin<Error = E> + 'static,
		D7: OutputPin<Error = E> + 'static,
		E: digital::Error,
	> DataBus for FourBitBus<RS, EN, D4, D5, D6, D7>
{
	type Error = E;

	type WriteFuture<'a, D: 'a + DelayNs> = impl Future<Output = Result<(), Self::Error>> + 'a;

	fn write<'a, D: DelayNs + 'a>(&'a mut self, byte: u8, data: bool, delay: &'a mut D) -> Self::WriteFuture<'a, D> {
		async move {
			self.rs.set_state(data.into()).map_err(Error::wrap_io(Port::RS))?;

			self.write_upper_nibble(byte)?;

			// Pulse the enable pin to recieve the upper nibble
			self.en.set_high().map_err(Error::wrap_io(Port::EN))?;
			delay.delay_ms(2).await;
			self.en.set_low().map_err(Error::wrap_io(Port::EN))?;

			self.write_lower_nibble(byte)?;
			// Pulse the enable pin to recieve the lower nibble
			self.en.set_high().map_err(Error::wrap_io(Port::EN))?;
			delay.delay_ms(2).await;
			self.en.set_low().map_err(Error::wrap_io(Port::EN))?;

			if data {
				self.rs.set_low().map_err(Error::wrap_io(Port::RS))?;
			}

			Ok(())
		}
	}
}
