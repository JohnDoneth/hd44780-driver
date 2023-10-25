use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write;

use crate::{bus::DataBus, error::Result};

/// This module supports I2C backpacks with a MCP23008 IC, like
/// the one from adafruit.
/// Connections as follows:
///
/// <table>
/// <tr><th>MCP23008 pin</th><th>name</th><th>LCD pin</th></tr>
/// <tr><td>0</td><td>N/C</td><td></td></tr>
/// <tr><td>1</td><td>RS</td><td>4</td></tr>
/// <tr><td>2</td><td>E</td><td>6</td></tr>
/// <tr><td>3</td><td>DB4</td><td>11</td></tr>
/// <tr><td>4</td><td>DB5</td><td>12</td></tr>
/// <tr><td>5</td><td>DB6</td><td>13</td></tr>
/// <tr><td>6</td><td>DB7</td><td>14</td></tr>
/// <tr><td>7</td><td>Backlight</td><td></td></tr>
/// </table>

pub struct I2CMCP23008Bus<I2C: Write> {
	i2c_bus: I2C,
	address: u8,
	backlight: u8,
}

const REG_IODIR: u8 = 0x00;
const REG_GPIO: u8 = 0x09;

impl<I2C: Write> I2CMCP23008Bus<I2C> {
	/// Create a new instance of the MCP23008 I2C driver. The address of those
	/// devices is 0b010_0xxx where x is configured by bootstrap pins.
	pub fn new(i2c_bus: I2C, address: u8, backlight: bool) -> Result<I2CMCP23008Bus<I2C>> {
		let backlight = if backlight { 0b1000_0000 } else { 0 };
		let mut mcp23008 = I2CMCP23008Bus { i2c_bus, address, backlight };
		// Set to reset values according to datasheet
		mcp23008.write_reg(REG_IODIR, 0b1111_1111)?;
		for reg in 0x01u8..0x0A {
			mcp23008.write_reg(reg, 0)?;
		}
		// Configure pins 1..=7 as outputs, see pin mapping above
		mcp23008.write_reg(REG_IODIR, 0b0000_0001)?;
		Ok(mcp23008)
	}

	/// Turns the backlight on or off based on the value of backlight.
	pub fn set_backlight(&mut self, backlight: bool) -> Result<()> {
		self.backlight = if backlight { 0b1000_0000 } else { 0 };
		self.set_pins(self.backlight)
	}

	fn write_reg(&mut self, reg: u8, value: u8) -> Result<()> {
		let data = [reg, value];
		self.i2c_bus.write(self.address, &data).map_err(|_| crate::error::Error)
	}

	fn set_pins(&mut self, pins: u8) -> Result<()> {
		self.write_reg(REG_GPIO, pins)
	}
}

impl<I2C: Write> DataBus for I2CMCP23008Bus<I2C> {
	fn write<D: DelayUs<u16> + DelayMs<u8>>(&mut self, byte: u8, data: bool, delay: &mut D) -> Result<()> {
		let rs = if data { 0b10 } else { 0b00 };
		let en = 0b0000_0100;
		let backlight = self.backlight;

		let upper_nibble = (byte & 0xF0) >> 4;
		let lower_nibble = byte & 0x0F;

		// upper nibble: [d7 d6 d5 d4]
		// Pulse EN
		// lower nibble: [d3 d2 d1 d0]
		// Pulse EN

		let pins = rs | backlight | (upper_nibble << 3);
		self.set_pins(pins)?;

		delay.delay_ms(1);

		let pins = rs | en | backlight | (upper_nibble << 3);
		self.set_pins(pins)?;

		delay.delay_ms(1);

		let pins = rs | backlight | (lower_nibble << 3);
		self.set_pins(pins)?;

		delay.delay_ms(1);

		let pins = rs | en | backlight | (lower_nibble << 3);
		self.set_pins(pins)?;

		delay.delay_ms(1);

		let pins = backlight | (lower_nibble << 3);
		self.set_pins(pins)?;

		Ok(())
	}
}
