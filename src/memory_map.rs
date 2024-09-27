use crate::display_size::DisplaySize;

pub type MemoryMap1601Split = StandardMemoryMap<8, 2>;
pub type MemoryMap1601Contiguous = Contiguous1RMemoryMap<16>;
pub type MemoryMap1602 = StandardMemoryMap<16, 2>;
pub type MemoryMap1604 = StandardMemoryMap<16, 4>;
pub type MemoryMap2002 = StandardMemoryMap<20, 4>;
pub type MemoryMap2004 = StandardMemoryMap<20, 4>;
pub type MemoryMap4002 = StandardMemoryMap<40, 2>;

pub trait DisplayMemoryMap {
	/// The address of a character on the display respecting the scrollable margin.
	fn address_for_xy(&self, x: u8, y: u8) -> Option<u8>;

	/// The columns of a line including scrollable margin.
	fn columns_in_line(&self, y: u8) -> u8;

	/// Size of the physical display.
	fn display_size(&self) -> DisplaySize;
}

pub struct StandardMemoryMap<const WIDTH: u8, const HEIGHT: u8, const LINE_WIDTH: u8 = 40>;

impl<const W: u8, const H: u8, const L: u8> StandardMemoryMap<W, H, L> {
	pub const fn new() -> Self {
		Self
	}
}

impl<const W: u8, const H: u8, const L: u8> Default for StandardMemoryMap<W, H, L> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const W: u8, const H: u8, const L: u8> DisplayMemoryMap for StandardMemoryMap<W, H, L> {
	/// Address of a character in a row assuming the most common memory mapping.
	///
	/// See https://web.alfredstate.edu/faculty/weimandn/lcd/lcd_addressing/lcd_addressing_index.html
	fn address_for_xy(&self, x: u8, y: u8) -> Option<u8> {
		if y >= H || x >= self.columns_in_line(y) {
			return None;
		}

		let mut addr = x;
		if (y & 1) > 0 {
			addr += 0x40;
		}
		if (y & 2) > 0 {
			addr += W;
		}
		Some(addr)
	}

	fn columns_in_line(&self, y: u8) -> u8 {
		let scrollable: u8 = const { scrollable_margin(W, H, L) };
		match H {
			// Only rows 3 and 4 can scroll
			3..=4 if (y & 2) == 0 => W,
			2..=4 => W + scrollable,
			_ => unimplemented!("1 and 5+ line displays are not covered by StandardMemoryMap"),
		}
	}

	fn display_size(&self) -> DisplaySize {
		DisplaySize::new(W, H)
	}
}

const fn scrollable_margin(w: u8, h: u8, l: u8) -> u8 {
	l - w * ((h + 1) / 2)
}

/// Memory Map for single-row displays that are using one line / contiguous memory.
pub struct Contiguous1RMemoryMap<const WIDTH: u8, const LINE_WIDTH: u8 = 0x50>;

impl<const W: u8, const L: u8> Contiguous1RMemoryMap<W, L> {
	pub const fn new() -> Self {
		Self
	}
}

impl<const W: u8, const L: u8> Default for Contiguous1RMemoryMap<W, L> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const W: u8, const L: u8> DisplayMemoryMap for Contiguous1RMemoryMap<W, L> {
	/// Address of a character using the less common single-line memory mapping.
	///
	/// See https://web.alfredstate.edu/faculty/weimandn/lcd/lcd_addressing/lcd_addressing_index.html
	fn address_for_xy(&self, x: u8, y: u8) -> Option<u8> {
		if y != 0 || x >= self.columns_in_line(y) {
			None
		} else {
			Some(x)
		}
	}

	fn columns_in_line(&self, _y: u8) -> u8 {
		L
	}

	fn display_size(&self) -> DisplaySize {
		DisplaySize::new(W, 1)
	}
}

#[cfg(test)]
mod tests {
	use crate::memory_map::{DisplayMemoryMap, MemoryMap2004};

	use super::scrollable_margin;

	#[test]
	fn test_scrollable_margin_1602() {
		let res = scrollable_margin(16, 2, 40);
		assert_eq!(res, 24);
	}

	#[test]
	fn test_scrollable_margin_1604() {
		let res = scrollable_margin(16, 4, 40);
		assert_eq!(res, 8);
	}

	#[test]
	fn test_scrollable_margin_2002() {
		let res = scrollable_margin(20, 2, 40);
		assert_eq!(res, 20);
	}

	#[test]
	fn test_scrollable_margin_2004() {
		let res = scrollable_margin(20, 4, 40);
		assert_eq!(res, 0);
	}

	#[test]
	fn test_scrollable_margin_4002() {
		let res = scrollable_margin(40, 2, 40);
		assert_eq!(res, 0);
	}

	#[test]
	fn test_position() {
		let map = MemoryMap2004::new();
		assert_eq!(Some(0), map.address_for_xy(0, 0));
		assert_eq!(Some(19), map.address_for_xy(19, 0));
		assert_eq!(Some(64), map.address_for_xy(0, 1));
		assert_eq!(Some(65), map.address_for_xy(1, 1));
	}

	#[test]
	fn test_invalid_col() {
		let map = MemoryMap2004::new();
		assert_eq!(None, map.address_for_xy(20, 0));
	}

	#[test]
	fn test_invalid_row() {
		let map = MemoryMap2004::new();
		assert_eq!(None, map.address_for_xy(0, 4));
	}

	#[test]
	fn test_invalid_rowcol() {
		let map = MemoryMap2004::new();
		assert_eq!(None, map.address_for_xy(20, 4));
	}
}
