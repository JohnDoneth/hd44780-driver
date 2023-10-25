#[derive(Clone, Copy)]
pub struct DisplaySize {
	columns: u8,
	lines: u8,
}

impl DisplaySize {
	pub fn new(columns: u8, lines: u8) -> Self {
		Self { columns, lines }
	}

	pub fn get(&self) -> (u8, u8) {
		(self.columns, self.lines)
	}
}

impl Default for DisplaySize {
	fn default() -> Self {
		Self { columns: 20, lines: 4 }
	}
}

#[cfg(test)]
mod test_display_size {
	use super::*;

	#[test]
	fn test_default_get() {
		let std = DisplaySize::default();
		assert_eq!(20, std.get().0);
		assert_eq!(4, std.get().1);
	}
}
