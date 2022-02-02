extern crate simple_error;
extern crate udev;

use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::process::{Command, Output};

use simple_error::bail;
use udev::Device;
use udev::Enumerator;

const ID_SERIAL: &str = "ID_SERIAL";
type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn find_for_serial_ids<'a>(
    enumerator: &mut Enumerator,
    keyboards: &'a HashMap<&str, Vec<&str>>,
    // ) -> Option<(&'a str, Vec<&'a str>)> {
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

pub fn find_by_serial_id(enumerator: &mut Enumerator, id_serial: &str) -> Option<Device> {
    enumerator.scan_devices().unwrap().find(|d| {
        d.properties().any(|p| {
            p.name().to_str().unwrap() == ID_SERIAL && p.value().to_str().unwrap() == id_serial
        })
    })
}

// TODO: should probably take arguments or read from the config file.
pub fn set_layout(program: &str, args: &[&str]) -> BoxResult<Output> {
    return match Command::new(program).args(args).output() {
        Ok(output) => Ok(output),
        Err(error) => bail!("set_layout error: {:?}", error),
    };
}
