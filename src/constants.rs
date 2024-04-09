// USB device descriptors taken from https://git.io/JI4nK
pub const PRODUCT_ID: u16 = 0x01ed;
pub const VENDOR_ID: u16 = 0x27b8;

// taken from blink1-tool: https://git.io/JeWXW (canon: https://git.io/JeWXl)
pub const HID_SET_REPORT: u8 = 0x09;
pub const HID_FEATURE: u16 = 0x03 << 0x08;

// Full command list can be found at github.com/todbot/blink1/blob/9bec7d35/hardware/firmware_mk2/main.c.
// the document can also be found https://github.com/todbot/blink1/blob/main/docs/blink1-hid-commands.md


///  Fade to RGB color       format: { 1, 'c', r,g,b,     th,tl, n }
pub const FADE_COMMAND_ACTION: u8 = 0x63;


/// Set RGB color now       format: { 1, 'n', r,g,b,       0,0, n } (*)
pub const IMMEDIATE_COMMAND_ACTION: u8 = 0x6e;


/// Read current RGB color  format: { 1, 'r', n,0,0,       0,0, n } (2)
pub const READ_RGB_COMMAND_ACTION: u8 = 0x72;


/// Serverdown tickle/off   format: { 1, 'D', on,th,tl,  st,sp,ep } (*)
pub const SERVER_TICKLE_COMMAND_ACTION: u8 = 0x44;


/// PlayLoop  format: { 1, 'p', on,sp,ep,c,    0, 0 } (2)
pub const PLAY_LOOP_COMMAND_ACTION: u8 = 0x70;


///  PlayState read back      format: { 1, 'S', 0,0,0,       0,0, 0 } (2)
pub const PLAY_STATE_READ_BACK_COMMAND_ACTION: u8 = 0x53;


/// Set color pattern line  format: { 1, 'P', r,g,b,     th,tl, p }
pub const SET_COLOR_PATTERN_LINE_COMMAND_ACTION: u8 = 0x50;


/// Save color patterns     format: { 1, 'W', 0,0,0,       0,0, 0 } (2)
pub const SAVE_COLOR_PATTERNS_COMMAND_ACTION: u8 = 0x57;


/// read color pattern line format: { 1, 'R', 0,0,0,       0,0, p }
pub const READ_COLOR_PATTERN_LINE_COMMAND_ACTION: u8 = 0x52;


/// Set ledn  format: { 1, 'l', n,0,0,       0,0, 0 } (2+)
pub const SET_LED_N_COMMAND_ACTION: u8 = 0x6c;


///  Read EEPROM location    format: { 1, 'e', ad,0,0,      0,0, 0 } (1)
pub const READ_EEPROM_LOCATION_COMMAND_ACTION: u8 = 0x65;


/// Write EEPROM location   format: { 1, 'E', ad,v,0,      0,0, 0 } (1)
pub const WRITE_EEPROM_LOCATION_COMMAND_ACTION: u8 = 0x45;


/// Get version  format: { 1, 'v', 0,0,0,       0,0, 0 }
pub const GET_VERSION_COMMAND_ACTION: u8 = 0x76;


/// Test command   format: { 1, '!', 0,0,0,       0,0, 0 }
pub const TEST_COMMAND_ACTION: u8 = 0x21;
