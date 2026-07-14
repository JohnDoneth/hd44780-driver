use embedded_hal::delay::DelayNs;

use crate::{
	bus::WritableDataBus,
	charset::{Charset, CharsetWithFallback},
	error::Result,
	memory_map::DisplayMemoryMap,
	HD44780,
};

pub struct DisplayWriter<'display, 'delay, Display, Delay> {
	display: &'display mut Display,
	delay: &'delay mut Delay,
	line: u8,
	col: u8,
	line_max: u8,
	col_min: u8,
	col_max: u8,
	current_col_max: u8,
	implicit_newline: bool,
	done: bool,
}

impl<'display, 'delay, B, M, C, Delay> DisplayWriter<'display, 'delay, HD44780<B, M, C>, Delay>
where
	B: WritableDataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
	Delay: DelayNs,
{
	fn new(
		display: &'display mut HD44780<B, M, C>,
		position: (u8, u8),
		max: (u8, u8),
		delay: &'delay mut Delay,
	) -> Result<Self, B::Error> {
		display.set_cursor_xy(position, delay)?;
		let this = Self {
			current_col_max: display.memory_map().columns_in_line(position.1),
			display,
			delay,
			col: position.0,
			line: position.1,
			col_max: max.0,
			line_max: max.1,
			col_min: position.0,
			implicit_newline: false,
			done: false,
		};
		Ok(this)
	}

	fn new_line(&mut self) {
		self.done |= self.line == self.line_max;
		if !self.done {
			self.line += 1;
			self.col = self.col_min;
			self.done |= self.display.set_cursor_xy((self.col, self.line), self.delay).is_err();
			self.current_col_max = self.display.memory_map().columns_in_line(self.line).min(self.col_max);
		}
	}
}

impl<'display, 'delay, B, M, C, Delay> core::fmt::Write for DisplayWriter<'display, 'delay, HD44780<B, M, C>, Delay>
where
	B: WritableDataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
	Delay: DelayNs,
{
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for ch in s.chars() {
			self.write_char(ch)?;
		}
		Ok(())
	}

	fn write_char(&mut self, ch: char) -> core::fmt::Result {
		if ch == '\n' {
			self.done |= self.col == self.current_col_max;
			self.new_line();
			return Ok(());
		}

		// Space is promoted to new line on implicit line breaks
		if self.implicit_newline && self.display.charset().is_whitespace(ch) {
			self.implicit_newline = false;
			return Ok(());
		}
		self.implicit_newline = false;

		if self.done || self.display.write_char(ch, self.delay).is_err() {
			return Err(core::fmt::Error);
		}

		// Continue on new line
		if self.col == self.current_col_max {
			self.implicit_newline = true;
			self.new_line();
		} else {
			self.col += 1;
		}

		Ok(())
	}
}

#[cfg(feature = "ufmt")]
impl<'display, 'delay, B, M, C, Delay> ufmt::uWrite for DisplayWriter<'display, 'delay, HD44780<B, M, C>, Delay>
where
	B: WritableDataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
	Delay: DelayNs,
{
	type Error = crate::Error<B::Error>;

	fn write_str(&mut self, s: &str) -> core::result::Result<(), Self::Error> {
		for ch in s.chars() {
			self.write_char(ch)?;
		}
		Ok(())
	}

	fn write_char(&mut self, ch: char) -> core::result::Result<(), Self::Error> {
		if ch == '\n' {
			self.done |= self.col == self.current_col_max;
			self.new_line();
			return Ok(());
		}

		// Space is promoted to new line on implicit line breaks
		if self.implicit_newline && self.display.charset().is_whitespace(ch) {
			self.implicit_newline = false;
			return Ok(());
		}
		self.implicit_newline = false;

		if self.done || self.display.write_char(ch, self.delay).is_err() {
			return Err(core::fmt::Error);
		}

		// Continue on new line
		if self.col == self.current_col_max {
			self.implicit_newline = true;
			self.new_line();
		} else {
			self.col += 1;
		}

		Ok(())
	}
}

impl<B, M, C> HD44780<B, M, C>
where
	B: WritableDataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
{
	pub fn writer<'display, 'delay, Delay: DelayNs>(
		&'display mut self,
		position: (u8, u8),
		max: (u8, u8),
		delay: &'delay mut Delay,
	) -> Result<DisplayWriter<'display, 'delay, Self, Delay>, B::Error> {
		DisplayWriter::new(self, position, max, delay)
	}
}
