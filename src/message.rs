use super::constants::{*};
use std::time::Duration;
use crate::color::Color;

/// Represents a command processable by the specification outlined in the [blink1 docs](https://git.io/JenDr).
#[derive(Debug, Copy, Clone)]
pub enum Message {
    Off,
    Fade(Color, Duration, Option<u8>),
    Immediate(Color, Option<u8>),
    ReadRGB(u8),
    ServerTickle(bool, u8, u8, u8),
    PlayLoop(bool, u8, u8, u8),
    PlayStateRead,
    SetColorPattern(Color, u8, u8, u8),
    SaveColorPatterns,
    ReadColorPattern(u8),
    SetLedN(u8),
    ReadEEPROM(u8),
    WriteEEPROM(u8, u8),
    GetVersion,
    TestCommand,
}

impl Default for Message {
    fn default() -> Self {
        Self::Off
    }
}

impl Message {
    /// Returns the buffer that will be written to the blink(1) usb device based on the specification
    /// outlined in the [blink1 docs](https://git.io/JenDr).
    pub fn buffer(&self) -> [u8; 8] {
        match self {
            Message::Off => Message::Immediate(Color::Three(0x00, 0x00, 0x00), None).buffer(),
            Message::Fade(color, duration, index) => {
                let (r, g, b) = color.rgb();
                // Divide by 10 and truncate into two parts
                let dms = duration.as_millis().checked_div(10).unwrap_or(0) as u16;
                let th = dms.checked_shr(8).unwrap_or(0) as u8;
                let tl = dms.checked_rem(0xff).unwrap_or(0) as u8;
                [0x01, FADE_COMMAND_ACTION, r, g, b, th, tl, index.unwrap_or(0x00)]
            }
            // NOTE: immediate sets with index is not supported, recommended workaround:
            // https://github.com/todbot/blink1/issues/251
            Message::Immediate(color, Some(index)) => {
                Message::Fade(*color, Duration::from_millis(0), Some(*index)).buffer()
            }
            Message::Immediate(color, None) => {
                let (r, g, b) = color.rgb();
                [0x01, IMMEDIATE_COMMAND_ACTION, r, g, b, 0x00, 0x00, 0]
            }
            Message::ReadRGB(n) => [0x01, READ_RGB_COMMAND_ACTION, *n, 0x00, 0x00, 0x00, 0x00, *n],
            Message::ServerTickle(on, th, tl, st) => {
                [0x01, SERVER_TICKLE_COMMAND_ACTION, *on as u8, *th, *tl, *st, 0x00, 0x00]
            }
            Message::PlayLoop(on, start_pos, end_pos, count) => {
                [0x01, PLAY_LOOP_COMMAND_ACTION, *on as u8, *start_pos, *end_pos, *count, 0x00, 0x00]
            }
            Message::PlayStateRead => {
                [0x01, PLAY_STATE_READ_BACK_COMMAND_ACTION, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
            }
            Message::SetColorPattern(color, time_high, time_low, pos) => {
                let (r, g, b) = color.rgb();
                [0x01, SET_COLOR_PATTERN_LINE_COMMAND_ACTION, r, g, b, *time_high, *time_low, *pos]
            }
            Message::SaveColorPatterns => {
                [0x01, SAVE_COLOR_PATTERNS_COMMAND_ACTION, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
            }
            Message::ReadColorPattern(pos) => {
                [0x01, READ_COLOR_PATTERN_LINE_COMMAND_ACTION, 0x00, 0x00, 0x00, 0x00, 0x00, *pos]
            }
            Message::SetLedN(n) => {
                [0x01, SET_LED_N_COMMAND_ACTION, *n, 0x00, 0x00, 0x00, 0x00, 0x00]
            }
            Message::ReadEEPROM(addr) => {
                [0x01, READ_EEPROM_LOCATION_COMMAND_ACTION, *addr, 0x00, 0x00, 0x00, 0x00, 0x00]
            }
            Message::WriteEEPROM(addr, val) => {
                [0x01, WRITE_EEPROM_LOCATION_COMMAND_ACTION, *addr, *val, 0x00, 0x00, 0x00, 0x00]
            }
            Message::GetVersion => {
                [0x01, GET_VERSION_COMMAND_ACTION, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
            }
            Message::TestCommand => {
                [0x01, TEST_COMMAND_ACTION, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
            }
        }
    }
}

impl From<&str> for Message {
    fn from(input: &str) -> Self {
        Message::Immediate(Color::from(input), None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Message};

    #[test]
    fn test_index_fade() {
        let red = Message::Fade(Color::from("red"), std::time::Duration::from_secs(1), Some(1));
        assert_eq!(red.buffer()[7], 0x01);
    }

    #[test]
    fn test_index_now() {
        let red = Message::Immediate(Color::from("red"), Some(10));
        assert_eq!(red.buffer()[7], 0x0A);
    }

    #[test]
    fn test_noindex() {
        let red = Message::from("red");
        assert_eq!(red.buffer()[7], 0x00);
    }

    #[test]
    fn test_red() {
        let red = Message::from("red");
        assert_eq!(red.buffer()[2..5], [0xff, 0x00, 0x00])
    }

    #[test]
    fn test_green() {
        let red = Message::from("green");
        assert_eq!(red.buffer()[2..5], [0x00, 0xff, 0x00])
    }

    #[test]
    fn test_blue() {
        let red = Message::from("blue");
        assert_eq!(red.buffer()[2..5], [0x00, 0x00, 0xff])
    }

    #[test]
    fn test_off() {
        let red = Message::from("off");
        assert_eq!(red.buffer()[2..5], [0x00, 0x00, 0x00])
    }
}