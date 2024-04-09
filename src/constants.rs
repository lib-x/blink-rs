// USB device descriptors taken from https://git.io/JI4nK
pub const PRODUCT_ID: u16 = 0x01ed;
pub const VENDOR_ID: u16 = 0x27b8;

// taken from blink1-tool: https://git.io/JeWXW (canon: https://git.io/JeWXl)
pub const HID_SET_REPORT: u8 = 0x09;
pub const HID_FEATURE: u16 = 0x03 << 0x08;

// Full command list can be found at github.com/todbot/blink1/blob/9bec7d35/hardware/firmware_mk2/main.c.

//    - Fade to RGB color       format: { 1, 'c', r,g,b,     th,tl, n }
pub const FADE_COMMAND_ACTION: u8 = 0x63;
//    - Set RGB color now       format: { 1, 'n', r,g,b,       0,0, n } (*)
pub const IMMEDIATE_COMMAND_ACTION: u8 = 0x6e;
//    - Read current RGB color  format: { 1, 'r', n,0,0,       0,0, n } (2)
//    - Serverdown tickle/off   format: { 1, 'D', on,th,tl,  st,sp,ep } (*)
//    - PlayLoop                format: { 1, 'p', on,sp,ep,c,    0, 0 } (2)
//    - Playstate readback      format: { 1, 'S', 0,0,0,       0,0, 0 } (2)
//    - Set color pattern line  format: { 1, 'P', r,g,b,     th,tl, p }
//    - Save color patterns     format: { 1, 'W', 0,0,0,       0,0, 0 } (2)
//    - read color pattern line format: { 1, 'R', 0,0,0,       0,0, p }
///// - Set ledn                format: { 1, 'l', n,0,0,       0,0, 0 } (2+)
//    - Read EEPROM location    format: { 1, 'e', ad,0,0,      0,0, 0 } (1)
//    - Write EEPROM location   format: { 1, 'E', ad,v,0,      0,0, 0 } (1)
//    - Get version             format: { 1, 'v', 0,0,0,       0,0, 0 }
//    - Test command            format: { 1, '!', 0,0,0,       0,0, 0 }

