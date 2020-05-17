use crate::{Cursor, CursorBlink, Display};

pub struct DisplayMode {
    pub cursor_visibility: Cursor,
    pub cursor_blink: CursorBlink,
    pub display: Display,
}

impl Default for DisplayMode {
    fn default() -> DisplayMode {
        DisplayMode {
            cursor_visibility: Cursor::Visible,
            cursor_blink: CursorBlink::On,
            display: Display::On,
        }
    }
}

impl DisplayMode {
    pub fn as_byte(&self) -> u8 {
        let cursor_blink_bits = match self.cursor_blink {
            CursorBlink::On => 0b0000_0001,
            CursorBlink::Off => 0,
        };

        let cursor_visible_bits = match self.cursor_visibility {
            Cursor::Visible => 0b0000_0010,
            Cursor::Invisible => 0,
        };

        let display_bits = match self.display {
            Display::On => 0b0000_0100,
            Display::Off => 0,
        };

        0b0000_1000 | cursor_visible_bits | cursor_blink_bits | display_bits
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cursor_visible() {
        let dm = DisplayMode {
            cursor_visibility: Cursor::Visible,
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0010 != 0);

        let dm = DisplayMode {
            cursor_visibility: Cursor::Invisible,
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0010 == 0);
    }

    #[test]
    fn cursor_blink() {
        let dm = DisplayMode {
            cursor_blink: CursorBlink::On,
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0001 != 0);

        let dm = DisplayMode {
            cursor_blink: CursorBlink::Off,
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0001 == 0);
    }

    #[test]
    fn display_visible() {
        let dm = DisplayMode {
            display: Display::On,
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0100 != 0);

        let dm = DisplayMode {
            display: Display::Off,
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0100 == 0);
    }
}
