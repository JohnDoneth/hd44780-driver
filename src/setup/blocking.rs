use embedded_hal::{
	delay::DelayNs,
	digital::{self, OutputPin},
	i2c::I2c,
};
use sealed::SealedDisplayOptions;

use crate::{
	bus::{DataBus, EightBitBus, FourBitBus, I2CBus},
	charset::CharsetWithFallback,
	entry_mode::EntryMode,
	error::{Error, Result},
	memory_map::DisplayMemoryMap,
	sealed::Internal,
	DisplayMode, HD44780,
};

use super::{DisplayOptions4Bit, DisplayOptions8Bit, DisplayOptionsI2C};

pub(crate) mod sealed {
	use embedded_hal::delay::DelayNs;

	use crate::{bus::DataBus, charset::CharsetWithFallback, memory_map::DisplayMemoryMap, sealed::Internal};

	use super::DisplayOptionsResult;

	#[doc(hidden)]
	pub trait SealedDisplayOptions: Sized {
		type Bus: DataBus;
		type MemoryMap: DisplayMemoryMap;
		type Charset: CharsetWithFallback;
		type IoError: core::fmt::Debug;

		fn new_display<D: DelayNs>(self, delay: &mut D, _: Internal) -> DisplayOptionsResult<Self>;
	}
}

/// Use this as an argument to [`HD44780::new`].
/// - [`DisplayOptionsI2C`]
/// - [`DisplayOptions4Bit`]
/// - [`DisplayOptions8Bit`]
pub trait DisplayOptions: sealed::SealedDisplayOptions {}

type HD44780FromOptions<Options> = HD44780<
	<Options as SealedDisplayOptions>::Bus,
	<Options as SealedDisplayOptions>::MemoryMap,
	<Options as SealedDisplayOptions>::Charset,
>;
type DisplayOptionsResult<Options> =
	core::result::Result<HD44780FromOptions<Options>, (Options, Error<<Options as SealedDisplayOptions>::IoError>)>;

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E>,
		D1: OutputPin<Error = E>,
		D2: OutputPin<Error = E>,
		D3: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> DisplayOptions for DisplayOptions8Bit<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D0: OutputPin<Error = E>,
		D1: OutputPin<Error = E>,
		D2: OutputPin<Error = E>,
		D3: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> SealedDisplayOptions for DisplayOptions8Bit<M, C, RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	type Bus = EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = E;

	fn new_display<D: DelayNs>(mut self, delay: &mut D, _: Internal) -> DisplayOptionsResult<Self> {
		let mut bus = EightBitBus::from_pins(self.pins);

		if let Err(error) = init_8bit(&mut bus, &self.entry_mode, delay) {
			self.pins = bus.destroy();
			return Err((self, error));
		}

		Ok(HD44780::new_raw(bus, self.memory_map, self.charset, self.entry_mode, DisplayMode::default()))
	}
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> DisplayOptions for DisplayOptions4Bit<M, C, RS, EN, D4, D5, D6, D7>
{
}

impl<
		M: DisplayMemoryMap,
		C: CharsetWithFallback,
		RS: OutputPin<Error = E>,
		EN: OutputPin<Error = E>,
		D4: OutputPin<Error = E>,
		D5: OutputPin<Error = E>,
		D6: OutputPin<Error = E>,
		D7: OutputPin<Error = E>,
		E: digital::Error,
	> SealedDisplayOptions for DisplayOptions4Bit<M, C, RS, EN, D4, D5, D6, D7>
{
	type Bus = FourBitBus<RS, EN, D4, D5, D6, D7>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = E;

	fn new_display<D: DelayNs>(mut self, delay: &mut D, _: Internal) -> DisplayOptionsResult<Self> {
		let mut bus = FourBitBus::from_pins(self.pins);

		if let Err(error) = init_4bit(&mut bus, &self.entry_mode, delay) {
			self.pins = bus.destroy();
			return Err((self, error));
		}

		Ok(HD44780::new_raw(bus, self.memory_map, self.charset, self.entry_mode, DisplayMode::default()))
	}
}

impl<M: DisplayMemoryMap, C: CharsetWithFallback, I2C: I2c> DisplayOptions for DisplayOptionsI2C<M, C, I2C> {}

impl<M: DisplayMemoryMap, C: CharsetWithFallback, I2C: I2c> SealedDisplayOptions for DisplayOptionsI2C<M, C, I2C> {
	type Bus = I2CBus<I2C>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = I2C::Error;

	fn new_display<D: DelayNs>(mut self, delay: &mut D, _: Internal) -> DisplayOptionsResult<Self> {
		let mut bus = I2CBus::new(self.i2c_bus, self.address);

		if let Err(error) = init_4bit(&mut bus, &self.entry_mode, delay) {
			self.i2c_bus = bus.destroy();
			return Err((self, error));
		}

		Ok(HD44780::new_raw(bus, self.memory_map, self.charset, self.entry_mode, DisplayMode::default()))
	}
}

// Follow the 8-bit setup procedure as specified in the HD44780 datasheet
fn init_8bit<B: DataBus, D: DelayNs>(bus: &mut B, entry_mode: &EntryMode, delay: &mut D) -> Result<(), B::Error> {
	// Wait for the LCD to wakeup if it was off
	delay.delay_ms(15u32);

	// Initialize Lcd in 8-bit mode
	bus.write(0b0011_0000, false, delay)?;

	// Wait for the command to be processed
	delay.delay_ms(5u32);

	// Sets 8-bit operation and enables 5x7 mode for chars
	bus.write(0b0011_1000, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	bus.write(0b0000_1110, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Clear Display
	bus.write(0b0000_0001, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Move the cursor to beginning of first line
	bus.write(0b000_0111, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Set entry mode
	bus.write(entry_mode.as_byte(), false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	Ok(())
}

fn init_4bit<B: DataBus, D: DelayNs>(bus: &mut B, entry_mode: &EntryMode, delay: &mut D) -> Result<(), B::Error> {
	// Wait for the LCD to wakeup if it was off
	delay.delay_ms(15u32);

	// Initialize Lcd in 4-bit mode
	bus.write(0x33, false, delay)?;

	// Wait for the command to be processed
	delay.delay_ms(5u32);

	// Sets 4-bit operation and enables 5x7 mode for chars
	bus.write(0x32, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	bus.write(0x28, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Clear Display
	bus.write(0x0E, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Move the cursor to beginning of first line
	bus.write(0x01, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	// Set entry mode
	bus.write(entry_mode.as_byte(), false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	bus.write(0x80, false, delay)?;

	// Wait for the command to be processed
	delay.delay_us(100);

	Ok(())
}
