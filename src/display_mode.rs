
pub struct DisplayMode {
    pub cursor_visible: bool,
    pub cursor_blink: bool,
    pub display_visible: bool,
}

impl Default for DisplayMode {

    fn default() -> DisplayMode {
        DisplayMode {
            cursor_visible: true,
            cursor_blink: true,
            display_visible: true,
        }
    }

}

impl DisplayMode {

    pub fn as_byte(&self) -> u8 {

        let cursor_blink_bits = match self.cursor_blink {
            true  => 0b0000_0001,
            false => 0,
        };

        let cursor_visible_bits = match self.cursor_visible {
            true  => 0b0000_0010,
            false => 0,
        };

        let display_visible_bits = match self.display_visible {
            true  => 0b0000_0100,
            false => 0,
        };

        0b0000_1000 
            | cursor_visible_bits 
            | cursor_blink_bits 
            | display_visible_bits
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn cursor_visible() {
        
        let dm = DisplayMode {
            cursor_visible : true, 
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0010 != 0);

        let dm = DisplayMode {
            cursor_visible : false, 
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0010 == 0);

    }

    #[test]
    fn cursor_blink() {
        
        let dm = DisplayMode {
            cursor_blink : true, 
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0001 != 0);

        let dm = DisplayMode {
            cursor_blink : false, 
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0001 == 0);

    }

    #[test]
    fn display_visible() {
        
        let dm = DisplayMode {
            display_visible : true, 
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0100 != 0);

        let dm = DisplayMode {
            display_visible : false, 
            ..Default::default()
        };

        assert!(dm.as_byte() & 0b0000_0100 == 0);

    }

}