mod constants;
mod color;
mod error;
mod message;

use std::fmt;
use std::time::Duration;
use rusb::{request_type, Context, Device, DeviceHandle, Direction, Recipient, RequestType, UsbContext};
use crate::constants::{HID_FEATURE, HID_SET_REPORT, PRODUCT_ID, VENDOR_ID};
use crate::error::BlinkError;
use crate::message::Message;
use crate::color::Color;

/// Main struct representing a blink(1) device
pub struct Blink {
    device: Device<Context>,
}

impl Blink {
    /// Create a new Blink instance by finding and connecting to a blink(1) device
    pub fn new() -> Result<Self, BlinkError> {
        let context = Context::new()?;
        
        // Find all USB devices
        for device in context.devices()?.iter() {
            if is_blinker(&device) {
                return Ok(Blink { device });
            }
        }
        
        Err(BlinkError::NotFound)
    }

    /// Send a message to the blink(1) device
    pub fn send(&self, message: &Message) -> Result<usize, BlinkError> {
        send(&self.device, message)
    }

    /// Set LED color immediately
    pub fn set_color(&self, color: Color, led: Option<u8>) -> Result<usize, BlinkError> {
        self.send(&Message::Immediate(color, led))
    }

    /// Fade to color over duration
    pub fn fade_to_color(&self, color: Color, duration: Duration, led: Option<u8>) -> Result<usize, BlinkError> {
        self.send(&Message::Fade(color, duration, led))
    }

    /// Read current RGB color for specified LED
    pub fn read_rgb(&self, led: u8) -> Result<usize, BlinkError> {
        self.send(&Message::ReadRGB(led))
    }

    /// Enable/disable server tickle mode with timing parameters
    pub fn server_tickle(&self, enable: bool, time_high: u8, time_low: u8, state: u8) -> Result<usize, BlinkError> {
        self.send(&Message::ServerTickle(enable, time_high, time_low, state))
    }

    /// Play pattern loop
    pub fn play_loop(&self, play: bool, start_pos: u8, end_pos: u8, count: u8) -> Result<usize, BlinkError> {
        self.send(&Message::PlayLoop(play, start_pos, end_pos, count))
    }

    /// Turn off all LEDs
    pub fn off(&self) -> Result<usize, BlinkError> {
        self.send(&Message::Off)
    }

    /// Read current play state
    pub fn read_play_state(&self) -> Result<usize, BlinkError> {
        self.send(&Message::PlayStateRead)
    }

    /// Set a color pattern line
    pub fn set_color_pattern(&self, color: Color, time_high: u8, time_low: u8, position: u8) -> Result<usize, BlinkError> {
        self.send(&Message::SetColorPattern(color, time_high, time_low, position))
    }

    /// Save current color patterns to EEPROM
    pub fn save_patterns(&self) -> Result<usize, BlinkError> {
        self.send(&Message::SaveColorPatterns)
    }

    /// Read a color pattern line
    pub fn read_pattern(&self, position: u8) -> Result<usize, BlinkError> {
        self.send(&Message::ReadColorPattern(position))
    }

    /// Set active LED number
    pub fn set_led_n(&self, n: u8) -> Result<usize, BlinkError> {
        self.send(&Message::SetLedN(n))
    }

    /// Read from EEPROM location
    pub fn read_eeprom(&self, address: u8) -> Result<usize, BlinkError> {
        self.send(&Message::ReadEEPROM(address))
    }

    /// Write to EEPROM location
    pub fn write_eeprom(&self, address: u8, value: u8) -> Result<usize, BlinkError> {
        self.send(&Message::WriteEEPROM(address, value))
    }

    /// Get firmware version
    pub fn get_version(&self) -> Result<usize, BlinkError> {
        self.send(&Message::GetVersion)
    }

    /// Run test command
    pub fn test(&self) -> Result<usize, BlinkError> {
        self.send(&Message::TestCommand)
    }
}

/// Check if a USB device is a blink(1) device
fn is_blinker(device: &Device<Context>) -> bool {
    device
        .device_descriptor()
        .map(|desc| desc.num_configurations() > 0 && desc.product_id() == PRODUCT_ID && desc.vendor_id() == VENDOR_ID)
        .unwrap_or(false)
}

/// Send a message to the blink(1) device
fn send(device: &Device<Context>, message: &Message) -> Result<usize, BlinkError> {
    let config = device.active_config_descriptor()?;
    let mut handle: DeviceHandle<Context> = device.open()?;
    let interface_num = config.interfaces().next().ok_or(BlinkError::NotFound)?.number();

    // Detach kernel driver if active
    if let Ok(active) = handle.kernel_driver_active(interface_num) {
        if active {
            handle.detach_kernel_driver(interface_num)?;
        }
    }

    // Claim interface
    handle.claim_interface(interface_num)?;

    // Create request type for HID feature
    let request_type = request_type(Direction::Out, RequestType::Class, Recipient::Interface);
    
    // Send message buffer to device
    let result = handle.write_control(
        request_type,
        HID_SET_REPORT,
        HID_FEATURE,
        interface_num.into(),
        &message.buffer(),
        Duration::from_secs(1),
    );

    // Release interface
    handle.release_interface(interface_num)?;

    // Reattach kernel driver if needed
    if handle.kernel_driver_active(interface_num)? {
        handle.attach_kernel_driver(interface_num)?;
    }

    Ok(result?)
}

impl fmt::Debug for Blink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blink(1) device")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_blink() {
        match Blink::new() {
            Ok(_) => println!("Successfully connected to blink(1)"),
            Err(e) => println!("Failed to connect: {:?}", e),
        }
    }

    #[test]
    fn test_color_cycle() -> Result<(), BlinkError> {
        let blink = Blink::new()?;
        
        // Test red
        blink.set_color(Color::Red, None)?;
        std::thread::sleep(Duration::from_secs(1));
        
        // Test green
        blink.set_color(Color::Green, None)?;
        std::thread::sleep(Duration::from_secs(1));
        
        // Test blue
        blink.set_color(Color::Blue, None)?;
        std::thread::sleep(Duration::from_secs(1));
        
        // Turn off
        blink.off()?;
        
        Ok(())
    }
}
