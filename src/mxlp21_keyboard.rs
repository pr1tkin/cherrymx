use rusb::{
    devices, Device, DeviceDescriptor, DeviceHandle, GlobalContext, Result
};
use std::time::Duration;
pub const CHERRY: u16 = 0x046a; // Vendor
pub const MXLP21: u16 = 0x01b2; // Device "MX-LP 2.1 Compact Wireless Mechanical Keyboard
const INDEX: u16 = 0x0000;
const TIMEOUT_MS: u64 = 50;

const REQ_TYPE: u8 = 0x21;
const REQ: u8 = 0x09;

const VALUE: u16 = 0x0204;

const ENDPOINT: u8 = 0x81;

const CMD_LEN: usize = 64;

pub trait MXLP21DeviceDescriptor {
    fn vendor_id(&self) -> u16;
    fn product_id(&self) -> u16;
}

impl MXLP21DeviceDescriptor for DeviceDescriptor {
    fn vendor_id(&self) -> u16 {
        self.vendor_id()
    }

    fn product_id(&self) -> u16 {
        self.product_id()
    }
}

pub fn find_mxlp21_keyboard() -> Option<Device<GlobalContext>> {
    devices().unwrap().iter().find(|device| {
        let desc = device.device_descriptor().unwrap();
        is_mxlp21_keyboard(&desc)
    })
}

pub fn is_mxlp21_keyboard(descriptor: &dyn MXLP21DeviceDescriptor) -> bool {
    descriptor.vendor_id() == CHERRY && descriptor.product_id() == MXLP21
}

pub fn show_info(device: &Device<GlobalContext>) {
    let descriptor = device.device_descriptor().unwrap();

    send_command_wrapper(device, |h| do_show_info(&descriptor, h));
}

fn do_show_info(descriptor: &DeviceDescriptor, handle: &DeviceHandle<GlobalContext>) {
    let timeout = std::time::Duration::from_millis(TIMEOUT_MS);
    let lang = handle.read_languages(timeout).unwrap()[0];

    println!(
        "Manufacturer: {}",
        handle
            .read_manufacturer_string(lang, descriptor, timeout)
            .unwrap_or_default()
    );

    println!(
        "Product:      {}",
        handle
            .read_product_string(lang, descriptor, timeout)
            .unwrap_or_default()
    );

    println!(
        "Serial:       {}",
        handle
            .read_serial_number_string(lang, descriptor, timeout)
            .unwrap_or_default()
    );
}

fn send_command_wrapper(
    device: &Device<GlobalContext>,
    cmd_fn: impl Fn(&DeviceHandle<GlobalContext>),
) {
    let mut handle = device.open().expect("Unable to open device!");

    let mut kernel_driver_detached = false;

    // Then we detach the kernel driver so that we can access the device
    if handle.kernel_driver_active(INDEX as u8).unwrap() {
        handle
            .detach_kernel_driver(INDEX as u8)
            .expect("Unable to detach kernel USB driver");

        kernel_driver_detached = true;
    }

    // Now we claim the interface
    handle
        .claim_interface(INDEX as u8)
        .expect("Unable to claim interface for device");

    // Do our thing
    cmd_fn(&handle);

    handle
        .release_interface(INDEX as u8)
        .expect("Unable to release interface for device");

    // Let the kernel take over again
    if kernel_driver_detached {
        handle
            .attach_kernel_driver(INDEX as u8)
            .expect("Unable to attach kernel USB driver");
    }
}


pub fn set_color(device: &Device<GlobalContext>, color: u32)  {
    send_command_wrapper(device, |h| {
        send_color(h, color);
    });
}

fn send_color(handle: &DeviceHandle<GlobalContext>, color: u32) {
    let command = format!(
        "0476020609000055000304020000{:06x}0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        color
    );

    let _bytes_sent = send_command(handle, &command).unwrap();
}

fn send_command(handle: &DeviceHandle<GlobalContext>, command: &str) -> Result<usize> {
    let mut bytes = [0u8; CMD_LEN];
    hex::decode_to_slice(command, &mut bytes).unwrap();
    send_to_keyboard(handle, &mut bytes)
}

fn send_to_keyboard(handle: &DeviceHandle<GlobalContext>, bytes: &mut [u8]) -> Result<usize> {
    handle.write_control(
        REQ_TYPE,
        REQ,
        VALUE,
        INDEX,
        bytes,
        Duration::from_millis(TIMEOUT_MS),
    )?;

    handle.read_interrupt(ENDPOINT, bytes, Duration::from_millis(TIMEOUT_MS))
}
