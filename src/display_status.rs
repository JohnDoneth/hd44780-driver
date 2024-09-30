use embedded_hal::delay::DelayNs;

use crate::{bus::ReadableDataBus, charset::CharsetWithFallback, error::Result, memory_map::DisplayMemoryMap, HD44780};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct DisplayStatus {
	pub busy: bool,
	pub address: u8,
}

impl<B, M, C> HD44780<B, M, C>
where
	B: ReadableDataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
{
	pub fn read_status<D: DelayNs>(&mut self, delay: &mut D) -> Result<DisplayStatus, B::Error> {
		let status_byte = self.bus.read(false, delay)?;

		Ok(DisplayStatus { busy: status_byte & 0x80 > 0, address: status_byte & 0x7f })
	}
}
