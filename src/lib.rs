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

fn is_blinker(device: &Device<Context>) -> bool {
    device
        .device_descriptor()
        .map(|desc| desc.num_configurations() > 0 && desc.product_id() == PRODUCT_ID && desc.vendor_id() == VENDOR_ID)
        .unwrap_or(false)
}

fn send(device: &Device<Context>, message: &Message) -> Result<usize, BlinkError> {
    let config = device.active_config_descriptor()?;
    let mut handle: DeviceHandle<Context> = device.open()?;
    let interface_num = config.interfaces().next().ok_or(BlinkError::NotFound)?.number();

    if let Ok(active) = handle.kernel_driver_active(interface_num) {
        if active {
            handle.detach_kernel_driver(interface_num)?;
        }
    }

    handle.claim_interface(interface_num)?;

    let buffer = message.buffer();
    let time = Duration::new(0, 100);
    let r_type = request_type(Direction::Out, RequestType::Class, Recipient::Interface);
    let request_value: u16 = HID_FEATURE | (buffer[0] as u16);
    let out = handle.write_control(r_type, HID_SET_REPORT, request_value, 0x00, &buffer, time);
    out.map_err(BlinkError::from)
}

/// Wraps the [`rusb::Context`](rusb::Context) type.
pub struct Blinkers {
    context: Context,
}

impl fmt::Debug for Blinkers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blinkers {{ }}")
    }
}

impl Blinkers {
    fn from_context(ctx: Context) -> Self {
        Blinkers { context: ctx }
    }

    pub fn new() -> Result<Self, BlinkError> {
        let context: Context = Context::new()?;
        Ok(Blinkers::from_context(context))
    }

    pub fn send(&self, cmd: Message) -> Result<usize, BlinkError> {
        let devices = self.context.devices()?;
        devices
            .iter()
            .filter(is_blinker)
            .map(|d| send(&d, &cmd))
            .collect::<Result<Vec<usize>, BlinkError>>()
            .map(|d| d.iter().sum())
    }

    pub fn device_count(&self) -> Result<usize, BlinkError> {
        let devices = self.context.devices()?;
        Ok(devices.iter().filter(is_blinker).count())
    }
}
