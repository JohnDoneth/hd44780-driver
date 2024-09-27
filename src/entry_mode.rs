/// Determines if the cursor should be incremented or decremented on write
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub enum CursorMode {
	#[default]
	Increment,
	Decrement,
}

/// Determines if the screen should be shifted on write
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub enum ShiftMode {
	Enabled,
	#[default]
	Disabled,
}

impl From<bool> for ShiftMode {
	fn from(b: bool) -> ShiftMode {
		if b {
			ShiftMode::Enabled
		} else {
			ShiftMode::Disabled
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "ufmt", derive(ufmt::derive::uDebug))]
pub struct EntryMode {
	pub cursor_mode: CursorMode,
	pub shift_mode: ShiftMode,
}

impl EntryMode {
	pub fn as_byte(&self) -> u8 {
		let cursor_bits = match self.cursor_mode {
			CursorMode::Increment => 0b0000_0010,
			CursorMode::Decrement => 0,
		};

		let shift_bits = match self.shift_mode {
			ShiftMode::Enabled => 0b0000_0001,
			ShiftMode::Disabled => 0,
		};

		0b0000_0100 | cursor_bits | shift_bits
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn cursor_mode() {
		let em = EntryMode { cursor_mode: CursorMode::Increment, shift_mode: Default::default() };

		assert!(em.as_byte() & 0b0000_0010 != 0);

		let em = EntryMode { cursor_mode: CursorMode::Decrement, shift_mode: Default::default() };

		assert!(em.as_byte() & 0b0000_0010 == 0);
	}

	#[test]
	fn shift_mode() {
		let em = EntryMode { cursor_mode: Default::default(), shift_mode: ShiftMode::Enabled };

		assert!(em.as_byte() & 0b0000_0001 != 0);

		let em = EntryMode { cursor_mode: Default::default(), shift_mode: ShiftMode::Disabled };

		assert!(em.as_byte() & 0b0000_0001 == 0);
	}
}
