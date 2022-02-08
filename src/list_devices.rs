extern crate simple_error;
extern crate udev;

use std::{collections::HashMap, io, time::Duration};

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

        let handle = match device.open() {
            Ok(handle) => handle,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        let timeout = Duration::from_secs(1);
        let language = handle.read_languages(timeout).unwrap();

        let product_name = handle
            .read_product_string(language[0], &device_desc, timeout)
            .ok();

        println!(
            "Name {:?} ID {:04x}:{:04x}",
            product_name,
            device.address(),
            device_desc.vendor_id(),
        );
    }
}

pub fn list_devices_with_udev() -> io::Result<()> {
    let mut enumerator = udev::Enumerator::new()?;

    for device in enumerator.scan_devices()? {
        println!();
        println!("{:#?}", device);

        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    Ok(())
}
