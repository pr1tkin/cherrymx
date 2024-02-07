use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use rusb::{UsbContext};
use rusb::Device;
use crate::service::mxlp21_keyboard::{CHERRY, MXLP21};

pub enum HotplugMessage {
    DeviceArrived(String),
    DeviceLeft(String),
}

struct HotPlugHandler {
    sender: Sender<HotplugMessage>,
}

impl HotPlugHandler {
    pub fn new( sender: Sender<HotplugMessage>) -> Self { HotPlugHandler { sender } }
}
impl rusb::Hotplug<rusb::Context>  for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<rusb::Context>) {
        match device.device_descriptor() {
            Ok(descriptor) => {
                if descriptor.vendor_id() == CHERRY && descriptor.product_id() == MXLP21 {
                    println!("Specific device arrived: {:?}", device);
                    let description = format!("Vendor ID: {:04x}, Product ID: {:04x}",
                                              descriptor.vendor_id(),
                                              descriptor.product_id());
                    let message = HotplugMessage::DeviceArrived(description);
                    self.sender.send(message).expect("Failed to send message from device_arrived");
                }
            },
            Err(e) => eprintln!("Error getting device descriptor: {:?}", e),
        }
    }

    fn device_left(&mut self, device: Device<rusb::Context>) {
        match device.device_descriptor() {
            Ok(descriptor) => {
                if descriptor.vendor_id() == CHERRY && descriptor.product_id() == MXLP21 {
                    println!("Specific device left: {:?}", device);
                    let description = format!("Vendor ID: {:04x}, Product ID: {:04x}",
                                              descriptor.vendor_id(),
                                              descriptor.product_id());
                    let message = HotplugMessage::DeviceLeft(description);
                    self.sender.send(message).expect("Failed to send message from device_left");
                }
            },
            Err(e) => eprintln!("Error getting device descriptor: {:?}", e),
        }
    }
}

impl Drop for HotPlugHandler {
    fn drop(&mut self) {
        println!("HotPlugHandler dropped");
    }
}

pub fn setup_hotplug() -> rusb::Result<(JoinHandle<()>, Sender<()>, Receiver<HotplugMessage>)> {
    let (tx_termination, rx_termination) = mpsc::channel::<>();
    let (tx, rx) = mpsc::channel::<HotplugMessage>();
    if rusb::has_hotplug() {
        let handle = thread::spawn(move || {
            let context = rusb::Context::new().expect("Failed to create USB context");
            let hotplug_handler = HotPlugHandler::new(tx);

            let _registration = rusb::HotplugBuilder::new()
                .enumerate(true)
                .register(&context, Box::new(hotplug_handler))
                .expect("Failed to register hotplug callback");

            loop {
                context.handle_events(None).expect("Failed to handle USB events");
                if rx_termination.try_recv().is_ok() {
                    println!("Terminating hotplug event handling thread.");
                    break;
                }
            }
        });

        Ok((handle, tx_termination, rx))
    } else {
        eprintln!("libusb hotplug API unsupported");
        Err(rusb::Error::Other)
    }
}