//use core::fmt::Result;
//use core::fmt::Write;

use embedded_hal_async::delay::DelayNs;

pub mod bus;
use bus::WritableDataBus;

use crate::charset::CharsetWithFallback;
use crate::display_size::DisplaySize;
pub use crate::error;
use crate::memory_map::DisplayMemoryMap;
use crate::sealed::Internal;
use crate::setup::non_blocking::DisplayOptions;
use error::Result;

pub use crate::entry_mode;

use entry_mode::{CursorMode, EntryMode};

pub use crate::display_mode;

pub use display_mode::DisplayMode;

pub struct HD44780<B: WritableDataBus, M: DisplayMemoryMap, C: CharsetWithFallback> {
	bus: B,
	memory_map: M,
	charset: C,
	entry_mode: EntryMode,
	display_mode: DisplayMode,
}

pub use crate::Cursor;
pub use crate::CursorBlink;
pub use crate::Direction;
pub use crate::Display;

use self::error::Error;

impl<B, M, C> HD44780<B, M, C>
where
	B: WritableDataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
{
	/// Create an instance of a `HD44780` using a struct implementing
	/// the delay trait.
	/// The delay instance is used to sleep between commands to
	/// ensure the `HD44780` has enough time to process commands.
	///
	/// If there was an error when setting up the display, the settings
	/// are returned as a tuple together with the error. This can be used
	/// to retry on error, or just to get back access to registers or buses.
	pub async fn new<Opt, D: DelayNs>(
		options: Opt,
		delay: &mut D,
	) -> core::result::Result<Self, (Opt, Error<Opt::IoError>)>
	where
		Opt: DisplayOptions<Bus = B, MemoryMap = M, Charset = C>,
	{
		options.new_display(delay, Internal).await
	}

	pub fn destroy(self) -> B {
		self.bus
	}

	pub(crate) fn new_raw(bus: B, memory_map: M, charset: C, entry_mode: EntryMode, display_mode: DisplayMode) -> Self {
		Self { bus, memory_map, charset, entry_mode, display_mode }
	}

	/// Unshifts the display and sets the cursor position to 0
	///
	/// ```rust,ignore
	/// lcd.reset().await?;
	/// ```
	pub async fn reset<'a, D: DelayNs>(&mut self, delay: &'a mut D) -> Result<(), B::Error> {
		self.write_command(0b0000_0010, delay).await
	}

	/// Set if the display should be on, if the cursor should be
	/// visible, and if the cursor should blink
	///
	/// Note: This is equivilent to calling all of the other relavent
	/// methods however this operation does it all in one go to the `HD44780`
	pub async fn set_display_mode<'a, D: DelayNs>(
		&mut self,
		display_mode: DisplayMode,
		delay: &'a mut D,
	) -> Result<(), B::Error> {
		self.display_mode = display_mode;

		let cmd_byte = self.display_mode.as_byte();

		self.write_command(cmd_byte, delay).await?;

		Ok(())
	}

	/// Clear the entire display
	///
	/// ```rust,ignore
	/// lcd.clear().await?;
	/// ```
	pub async fn clear<'a, D: DelayNs>(&mut self, delay: &'a mut D) -> Result<(), B::Error> {
		self.write_command(0b0000_0001, delay).await
	}

	/// Get the memory map information for this display.
	pub fn memory_map(&self) -> &M {
		&self.memory_map
	}

	/// Get the display size.
	pub fn display_size(&self) -> DisplaySize {
		self.memory_map.display_size()
	}

	/// If enabled, automatically scroll the display when a new
	/// character is written to the display
	///
	/// ```rust,ignore
	/// lcd.set_autoscroll(true).await?;
	/// ```
	pub async fn set_autoscroll<'a, D: DelayNs>(&mut self, enabled: bool, delay: &'a mut D) -> Result<(), B::Error> {
		self.entry_mode.shift_mode = enabled.into();

		let cmd = self.entry_mode.as_byte();

		self.write_command(cmd, delay).await
	}

	/// Set if the cursor should be visible
	pub async fn set_cursor_visibility<'a, D: DelayNs>(
		&mut self,
		visibility: Cursor,
		delay: &'a mut D,
	) -> Result<(), B::Error> {
		self.display_mode.cursor_visibility = visibility;

		let cmd = self.display_mode.as_byte();

		self.write_command(cmd, delay).await
	}

	/// Set if the characters on the display should be visible
	pub async fn set_display<'a, D: DelayNs>(&mut self, display: Display, delay: &'a mut D) -> Result<(), B::Error> {
		self.display_mode.display = display;

		let cmd = self.display_mode.as_byte();

		self.write_command(cmd, delay).await
	}

	/// Set if the cursor should blink
	pub async fn set_cursor_blink<'a, D: DelayNs>(
		&mut self,
		blink: CursorBlink,
		delay: &'a mut D,
	) -> Result<(), B::Error> {
		self.display_mode.cursor_blink = blink;

		let cmd = self.display_mode.as_byte();

		self.write_command(cmd, delay).await
	}

	/// Set which way the cursor will move when a new character is written
	///
	/// ```rust,ignore
	/// // Move right (Default) when a new character is written
	/// lcd.set_cursor_mode(CursorMode::Right).await?;
	///
	/// // Move left when a new character is written
	/// lcd.set_cursor_mode(CursorMode::Left).await?;
	/// ```
	pub async fn set_cursor_mode<'a, D: DelayNs>(
		&mut self,
		mode: CursorMode,
		delay: &'a mut D,
	) -> Result<(), B::Error> {
		self.entry_mode.cursor_mode = mode;

		let cmd = self.entry_mode.as_byte();

		self.write_command(cmd, delay).await
	}

	/// Set the cursor position
	///
	/// ```rust,ignore
	/// // Move to line 2
	/// lcd.set_cursor_pos(40).await?;
	/// ```
	pub async fn set_cursor_pos<'a, D: DelayNs>(&mut self, position: u8, delay: &'a mut D) -> Result<(), B::Error> {
		let lower_7_bits = 0b0111_1111 & position;

		self.write_command(0b1000_0000 | lower_7_bits, delay).await
	}

	/// Set the cursor position
	///
	/// ```rust,ignore
	/// // Move to the start of line 3
	/// lcd.set_cursor_pos_xy(0,2).await?;
	/// ```
	pub async fn set_cursor_xy<D: DelayNs>(&mut self, position: (u8, u8), delay: &mut D) -> Result<(), B::Error> {
		let size = self.display_size().get();
		let Some(pos) = self.memory_map.address_for_xy(position.0, position.1) else {
			return Err(Error::Position { position, size });
		};

		self.write_command(0b1000_0000 | pos, delay).await
	}

	/// Shift just the cursor to the left or the right
	///
	/// ```rust,ignore
	/// lcd.shift_cursor(Direction::Left).await?;
	/// lcd.shift_cursor(Direction::Right).await?;
	/// ```
	pub async fn shift_cursor<'a, D: DelayNs>(&mut self, dir: Direction, delay: &'a mut D) -> Result<(), B::Error> {
		let bits = match dir {
			Direction::Left => 0b0000_0000,
			Direction::Right => 0b0000_0100,
		};

		self.write_command(0b0001_0000 | bits | bits, delay).await
	}

	/// Shift the entire display to the left or the right
	///
	/// ```rust,ignore
	/// lcd.shift_display(Direction::Left).await?;
	/// lcd.shift_display(Direction::Right).await?;
	/// ```
	pub async fn shift_display<'a, D: DelayNs>(&mut self, dir: Direction, delay: &'a mut D) -> Result<(), B::Error> {
		let bits = match dir {
			Direction::Left => 0b0000_0000,
			Direction::Right => 0b0000_0100,
		};

		self.write_command(0b0001_1000 | bits, delay).await
	}

	/// Write a single character to the `HD44780`. This `char` just gets downcast to a `u8`
	/// internally, so make sure that whatever character you're printing fits inside that range, or
	/// you can just use [write_byte](#method.write_byte) to have the compiler check for you.
	/// See the documentation on that function for more details about compatibility.
	///
	/// ```rust,ignore
	/// lcd.write_char('A', &'a mut DelayUs).await?; // prints 'A'
	/// ```
	pub async fn write_char<'a, D: DelayNs>(&mut self, data: char, delay: &'a mut D) -> Result<(), B::Error> {
		self.write_byte(self.charset.code_from_utf8_with_fallback(data), delay).await
	}

	async fn write_command<'a, D: DelayNs>(&mut self, cmd: u8, delay: &'a mut D) -> Result<(), B::Error> {
		self.bus.write(cmd, false, delay).await?;

		// Wait for the command to be processed
		delay.delay_us(100).await;
		Ok(())
	}

	/// Writes a string to the HD44780. Internally, this just prints the string byte-by-byte, so
	/// make sure the characters in the string fit in a normal `u8`. See the documentation on
	/// [write_byte](#method.write_byte) for more details on compatibility.
	///
	/// ```rust,ignore
	/// lcd.write_str("Hello, World!", &'a mut DelayUs).await?;
	/// ```
	pub async fn write_str<'a, D: DelayNs>(&mut self, string: &str, delay: &'a mut D) -> Result<(), B::Error> {
		for ch in string.chars() {
			self.write_char(ch, delay).await?;
		}
		Ok(())
	}

	/// Writes a sequence of bytes to the HD44780. See the documentation on the
	/// [write_byte](#method.write_byte) function for more details about compatibility.
	///
	/// ```rust,ignore
	/// lcd.write_bytes(b"Hello, World!", &'a mut DelayUs).await?;
	/// ```
	pub async fn write_bytes<'a, D: DelayNs>(&mut self, string: &[u8], delay: &'a mut D) -> Result<(), B::Error> {
		for &b in string {
			self.write_byte(b, delay).await?;
		}
		Ok(())
	}

	/// Writes a single byte to the HD44780. These usually map to ASCII characters when printed on the
	/// screen, but not always. While it varies depending on the ROM of the LCD, `0x20u8..=0x5b`
	/// and `0x5d..=0x7d` should map to their standard ASCII characters. That is, all the printable
	/// ASCII characters work, excluding `\` and `~`, which are usually displayed as `¥` and `🡢`
	/// respectively.
	///
	/// More information can be found in the Hitachi datasheets for the HD44780.
	///
	/// ```rust,ignore
	/// lcd.write_byte(b'A', &'a mut DelayUs)?; // prints 'A'
	/// lcd.write_byte(b'\\', &'a mut DelayUs)?; // usually prints ¥
	/// lcd.write_byte(b'~', &'a mut DelayUs)?; // usually prints 🡢
	/// lcd.write_byte(b'\x7f', &'a mut DelayUs)?; // usually prints 🡠
	/// ```
	pub async fn write_byte<'a, D: DelayNs>(&mut self, data: u8, delay: &'a mut D) -> Result<(), B::Error> {
		self.bus.write(data, true, delay).await?;

		// Wait for the command to be processed
		delay.delay_us(100).await;

		Ok(())
	}

	// Pulse the enable pin telling the HD44780 that we something for it
	/*fn pulse_enable(&mut self) {
		self.en.set_high();
		self.delay.delay_ms(15u8);
		self.en.set_low();
	}*/
}

//impl<B> Write for HD44780<B>
//where
//    B: DataBus,
//{
//    fn write_str(&mut self, string: &str) -> Result {
//        for c in string.chars() {
//            self.write_char(c);
//        }
//        Ok(())
//    }
//}
