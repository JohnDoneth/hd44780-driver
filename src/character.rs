use core::iter::once;

use embedded_hal::delay::DelayNs;

use crate::{bus::DataBus, charset::CharsetWithFallback, error::Result, memory_map::DisplayMemoryMap, HD44780};

pub struct CharacterDefinition {
	pub pattern: [u8; 10],
	pub cursor: u8,

	// /// Since lines 12 to 16 are not used for display,
	// /// they can be used for general data RAM.
	// pub data: Option<[u8; 5]>,
}

impl<B, M, C> HD44780<B, M, C>
where
	B: DataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
{
	pub fn define_custom_character<D: DelayNs>(
		&mut self,
		code: u8,
		def: &CharacterDefinition,
		delay: &mut D,
	) -> Result<(), B::Error> {
        self.write_command(0b01000000 | (code & 0b11) << 4, delay)?;
        delay.delay_us(100);

        let lines = def.pattern.iter().cloned().chain(once(def.cursor));
        for line in lines {
            self.write_byte(line, delay)?;
        }

        // Change back to DDRAM
		self.set_cursor_pos(0, delay)?;
		Ok(())
	}
}
