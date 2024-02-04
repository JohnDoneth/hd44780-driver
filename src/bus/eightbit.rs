use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use crate::{
	bus::DataBus,
	error::{Error, Result},
};

pub struct EightBitBus<
	RS: OutputPin,
	EN: OutputPin,
	D0: OutputPin,
	D1: OutputPin,
	D2: OutputPin,
	D3: OutputPin,
	D4: OutputPin,
	D5: OutputPin,
	D6: OutputPin,
	D7: OutputPin,
> {
	rs: RS,
	en: EN,
	d0: D0,
	d1: D1,
	d2: D2,
	d3: D3,
	d4: D4,
	d5: D5,
	d6: D6,
	d7: D7,
}

impl<
		RS: OutputPin,
		EN: OutputPin,
		D0: OutputPin,
		D1: OutputPin,
		D2: OutputPin,
		D3: OutputPin,
		D4: OutputPin,
		D5: OutputPin,
		D6: OutputPin,
		D7: OutputPin,
	> EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	#[allow(clippy::too_many_arguments)]
	pub fn from_pins(
		rs: RS,
		en: EN,
		d0: D0,
		d1: D1,
		d2: D2,
		d3: D3,
		d4: D4,
		d5: D5,
		d6: D6,
		d7: D7,
	) -> EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
		EightBitBus { rs, en, d0, d1, d2, d3, d4, d5, d6, d7 }
	}

	fn set_bus_bits(&mut self, data: u8) -> Result<()> {
		let db0: bool = (0b0000_0001 & data) != 0;
		let db1: bool = (0b0000_0010 & data) != 0;
		let db2: bool = (0b0000_0100 & data) != 0;
		let db3: bool = (0b0000_1000 & data) != 0;
		let db4: bool = (0b0001_0000 & data) != 0;
		let db5: bool = (0b0010_0000 & data) != 0;
		let db6: bool = (0b0100_0000 & data) != 0;
		let db7: bool = (0b1000_0000 & data) != 0;

		if db0 {
			self.d0.set_high().map_err(|_| Error)?;
		} else {
			self.d0.set_low().map_err(|_| Error)?;
		}

		if db1 {
			self.d1.set_high().map_err(|_| Error)?;
		} else {
			self.d1.set_low().map_err(|_| Error)?;
		}

		if db2 {
			self.d2.set_high().map_err(|_| Error)?;
		} else {
			self.d2.set_low().map_err(|_| Error)?;
		}

		if db3 {
			self.d3.set_high().map_err(|_| Error)?;
		} else {
			self.d3.set_low().map_err(|_| Error)?;
		}

		if db4 {
			self.d4.set_high().map_err(|_| Error)?;
		} else {
			self.d4.set_low().map_err(|_| Error)?;
		}

		if db5 {
			self.d5.set_high().map_err(|_| Error)?;
		} else {
			self.d5.set_low().map_err(|_| Error)?;
		}

		if db6 {
			self.d6.set_high().map_err(|_| Error)?;
		} else {
			self.d6.set_low().map_err(|_| Error)?;
		}

		if db7 {
			self.d7.set_high().map_err(|_| Error)?;
		} else {
			self.d7.set_low().map_err(|_| Error)?;
		}

		Ok(())
	}
}

impl<
		RS: OutputPin,
		EN: OutputPin,
		D0: OutputPin,
		D1: OutputPin,
		D2: OutputPin,
		D3: OutputPin,
		D4: OutputPin,
		D5: OutputPin,
		D6: OutputPin,
		D7: OutputPin,
	> DataBus for EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	fn write<D: DelayNs>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<()> {
		if data {
			self.rs.set_high().map_err(|_| Error)?;
		} else {
			self.rs.set_low().map_err(|_| Error)?;
		}

		self.set_bus_bits(byte)?;

		self.en.set_high().map_err(|_| Error)?;
		delay.delay_ms(2u32);
		self.en.set_low().map_err(|_| Error)?;

		if data {
			self.rs.set_low().map_err(|_| Error)?;
		}

		Ok(())
	}
}
