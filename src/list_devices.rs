extern crate simple_error;
extern crate udev;

use std::collections::HashMap;

use libusb::Context;
use udev::Enumerator;

const ID_SERIAL: &str = "ID_SERIAL";

pub fn find_with_udev(
    enumerator: &mut Enumerator,
    keyboards: &HashMap<String, Vec<String>>,
) -> Option<String> {
    enumerator
        .scan_devices()
        .unwrap()
        .flat_map(|d| {
            d.properties()
                .filter(|p| p.name().to_str().unwrap() == ID_SERIAL)
                .map(|p| p.value().to_str().unwrap().to_owned())
                .collect::<Vec<String>>()
        })
        .find(|keyboard_serial_id| keyboards.contains_key(keyboard_serial_id.as_str()))
}

pub fn find_with_libusb(
    context: &Context,
    keyboards: &HashMap<String, Vec<String>>,
) -> Option<String> {
    for device in context.devices().unwrap().iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        let device_id = format!(
            "{:04x}:{:04x}",
            device_desc.vendor_id(),
            device_desc.product_id()
        );
        if keyboards.contains_key(device_id.as_str()) {
            return Some(device_id);
        }
    }
    None
}

pub fn list_devices_with_libusb() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
    }
}
