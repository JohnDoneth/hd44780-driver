use core::future::Future;

use embedded_hal::digital::{self, OutputPin};
use embedded_hal_async::{delay::DelayNs, i2c::I2c};
use sealed::SealedDisplayOptions;

use crate::{
	bus::{EightBitBus, FourBitBus, I2CBus, WriteSelect},
	charset::CharsetWithFallback,
	entry_mode::EntryMode,
	error::{Error, Port, Result},
	memory_map::DisplayMemoryMap,
	non_blocking::{bus::WritableDataBus, HD44780},
	sealed::Internal,
	DisplayMode,
};

use super::{DisplayOptions4Bit, DisplayOptions8Bit, DisplayOptionsI2C};

pub(crate) mod sealed {
	use core::future::Future;

	use embedded_hal_async::delay::DelayNs;

	use crate::{
		charset::CharsetWithFallback, memory_map::DisplayMemoryMap, non_blocking::bus::WritableDataBus,
		sealed::Internal,
	};

	use super::DisplayOptionsResult;

	#[doc(hidden)]
	pub trait SealedDisplayOptions: Sized {
		type Bus: WritableDataBus;
		type MemoryMap: DisplayMemoryMap;
		type Charset: CharsetWithFallback;
		type IoError: core::fmt::Debug;

		type Future<'d, D: 'd + DelayNs>: Future<Output = DisplayOptionsResult<Self>>;

		fn new_display<D: DelayNs>(self, delay: &mut D, _: Internal) -> Self::Future<'_, D>;
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
		M: DisplayMemoryMap + 'static,
		C: CharsetWithFallback + 'static,
		RS: OutputPin<Error = E> + 'static,
		RW: WriteSelect<E> + 'static,
		EN: OutputPin<Error = E> + 'static,
		D0: OutputPin<Error = E> + 'static,
		D1: OutputPin<Error = E> + 'static,
		D2: OutputPin<Error = E> + 'static,
		D3: OutputPin<Error = E> + 'static,
		D4: OutputPin<Error = E> + 'static,
		D5: OutputPin<Error = E> + 'static,
		D6: OutputPin<Error = E> + 'static,
		D7: OutputPin<Error = E> + 'static,
		E: digital::Error,
	> DisplayOptions for DisplayOptions8Bit<M, C, RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
}

impl<
		M: DisplayMemoryMap + 'static,
		C: CharsetWithFallback + 'static,
		RS: OutputPin<Error = E> + 'static,
		RW: WriteSelect<E> + 'static,
		EN: OutputPin<Error = E> + 'static,
		D0: OutputPin<Error = E> + 'static,
		D1: OutputPin<Error = E> + 'static,
		D2: OutputPin<Error = E> + 'static,
		D3: OutputPin<Error = E> + 'static,
		D4: OutputPin<Error = E> + 'static,
		D5: OutputPin<Error = E> + 'static,
		D6: OutputPin<Error = E> + 'static,
		D7: OutputPin<Error = E> + 'static,
		E: digital::Error,
	> SealedDisplayOptions for DisplayOptions8Bit<M, C, RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
	type Bus = EightBitBus<RS, RW, EN, D0, D1, D2, D3, D4, D5, D6, D7>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = E;

	type Future<'d, D: 'd + DelayNs> = impl Future<Output = DisplayOptionsResult<Self>> + 'd;

	fn new_display<D: DelayNs>(mut self, delay: &mut D, _: Internal) -> Self::Future<'_, D> {
		async move {
			if let Err(error) = self.pins.rw.select_write(Internal) {
				return Err((self, Error::Io { port: Port::RW, error }));
			}

			let mut bus = EightBitBus::from_pins(self.pins);

			if let Err(error) = init_8bit(&mut bus, &self.entry_mode, delay).await {
				self.pins = bus.destroy();
				return Err((self, error));
			}

			Ok(HD44780::new_raw(bus, self.memory_map, self.charset, self.entry_mode, DisplayMode::default()))
		}
	}
}

impl<
		M: DisplayMemoryMap + 'static,
		C: CharsetWithFallback + 'static,
		RS: OutputPin<Error = E> + 'static,
		RW: WriteSelect<E> + 'static,
		EN: OutputPin<Error = E> + 'static,
		D4: OutputPin<Error = E> + 'static,
		D5: OutputPin<Error = E> + 'static,
		D6: OutputPin<Error = E> + 'static,
		D7: OutputPin<Error = E> + 'static,
		E: digital::Error,
	> DisplayOptions for DisplayOptions4Bit<M, C, RS, RW, EN, D4, D5, D6, D7>
{
}

impl<
		M: DisplayMemoryMap + 'static,
		C: CharsetWithFallback + 'static,
		RS: OutputPin<Error = E> + 'static,
		RW: WriteSelect<E> + 'static,
		EN: OutputPin<Error = E> + 'static,
		D4: OutputPin<Error = E> + 'static,
		D5: OutputPin<Error = E> + 'static,
		D6: OutputPin<Error = E> + 'static,
		D7: OutputPin<Error = E> + 'static,
		E: digital::Error,
	> SealedDisplayOptions for DisplayOptions4Bit<M, C, RS, RW, EN, D4, D5, D6, D7>
{
	type Bus = FourBitBus<RS, RW, EN, D4, D5, D6, D7>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = E;

	type Future<'d, D: 'd + DelayNs> = impl Future<Output = DisplayOptionsResult<Self>> + 'd;

	fn new_display<D: DelayNs>(mut self, delay: &mut D, _: Internal) -> Self::Future<'_, D> {
		async move {
			if let Err(error) = self.pins.rw.select_write(Internal) {
				return Err((self, Error::Io { port: Port::RW, error }));
			}

			let mut bus = FourBitBus::from_pins(self.pins);

			if let Err(error) = init_4bit(&mut bus, &self.entry_mode, delay).await {
				self.pins = bus.destroy();
				return Err((self, error));
			}

			Ok(HD44780::new_raw(bus, self.memory_map, self.charset, self.entry_mode, DisplayMode::default()))
		}
	}
}

impl<M: DisplayMemoryMap + 'static, C: CharsetWithFallback + 'static, I2C: I2c + 'static> DisplayOptions
	for DisplayOptionsI2C<M, C, I2C>
{
}

impl<M: DisplayMemoryMap + 'static, C: CharsetWithFallback + 'static, I2C: I2c + 'static> SealedDisplayOptions
	for DisplayOptionsI2C<M, C, I2C>
{
	type Bus = I2CBus<I2C>;
	type MemoryMap = M;
	type Charset = C;
	type IoError = I2C::Error;

	type Future<'d, D: 'd + DelayNs> = impl Future<Output = DisplayOptionsResult<Self>> + 'd;

	fn new_display<D: DelayNs>(mut self, delay: &mut D, _: Internal) -> Self::Future<'_, D> {
		async move {
			let mut bus = I2CBus::new(self.i2c_bus, self.address);

			if let Err(error) = init_4bit(&mut bus, &self.entry_mode, delay).await {
				self.i2c_bus = bus.destroy();
				return Err((self, error));
			}

			Ok(HD44780::new_raw(bus, self.memory_map, self.charset, self.entry_mode, DisplayMode::default()))
		}
	}
}

// Follow the 8-bit setup procedure as specified in the HD44780 datasheet
async fn init_8bit<B: WritableDataBus, D: DelayNs>(
	bus: &mut B,
	entry_mode: &EntryMode,
	delay: &mut D,
) -> Result<(), B::Error> {
	// Wait for the LCD to wakeup if it was off
	delay.delay_ms(15).await;

	// Initialize Lcd in 8-bit mode
	bus.write(0b0011_0000, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_ms(5).await;

	// Sets 8-bit operation and enables 5x7 mode for chars
	bus.write(0b0011_1000, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	bus.write(0b0000_1110, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	// Clear Display
	bus.write(0b0000_0001, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	// Move the cursor to beginning of first line
	bus.write(0b000_0111, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	// Set entry mode
	bus.write(entry_mode.as_byte(), false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	Ok(())
}

async fn init_4bit<B: WritableDataBus, D: DelayNs>(
	bus: &mut B,
	entry_mode: &EntryMode,
	delay: &mut D,
) -> Result<(), B::Error> {
	// Wait for the LCD to wakeup if it was off
	delay.delay_ms(15).await;

	// Initialize Lcd in 4-bit mode
	bus.write(0x33, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_ms(5).await;

	// Sets 4-bit operation and enables 5x7 mode for chars
	bus.write(0x32, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	bus.write(0x28, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	// Clear Display
	bus.write(0x0E, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	// Move the cursor to beginning of first line
	bus.write(0x01, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	// Set entry mode
	bus.write(entry_mode.as_byte(), false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	bus.write(0x80, false, delay).await?;

	// Wait for the command to be processed
	delay.delay_us(100).await;

	Ok(())
}
