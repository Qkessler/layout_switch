extern crate udev;

use std::io;
use std::process::Command;
use std::process::Output;

use udev::Device;
use udev::Enumerator;

const ID_SERIAL: &str = "ID_SERIAL";

pub fn find_for_serial_ids(enumerator: &mut Enumerator, ids: &[&str]) -> Option<Vec<Device>> {
    Some(
        enumerator
            .scan_devices()
            .unwrap()
            .filter(|d| {
                d.properties().any(|p| {
                    p.name().to_str().unwrap() == ID_SERIAL
                        && ids.contains(&p.value().to_str().unwrap())
                })
            })
            .collect::<Vec<Device>>(),
    )
}

pub fn find_by_serial_id(enumerator: &mut Enumerator, id_serial: &str) -> Option<Device> {
    enumerator.scan_devices().unwrap().find(|d| {
        d.properties().any(|p| {
            p.name().to_str().unwrap() == ID_SERIAL && p.value().to_str().unwrap() == id_serial
        })
    })
}

// TODO: should probably take arguments or read from the config file.
pub fn set_layout(program: &str, args: &[&str]) -> io::Result<Output> {
    return match Command::new(program).args(args).output() {
        Ok(output) => Ok(output),
        Err(error) => Err(error),
    };
}

pub fn list_devices() -> io::Result<()> {
    let mut enumerator = Enumerator::new()?;

    println!("{:#?}", find_by_serial_id(&mut enumerator, "TKC_Portico"));

    let keyboard_list = vec!["TKC_Portico"];
    println!(
        "{:#?}",
        find_for_serial_ids(&mut enumerator, &keyboard_list)
    );

    // println!("{:#?}", keyboards);

    // for device in enumerator.scan_devices()? {
    //     println!();
    //     println!("{:#?}", device);

    //     println!("  [properties]");
    //     for property in device.properties() {
    //         println!("    - {:?} {:?}", property.name(), property.value());
    //     }
    // }

    // println!("  [attributes]");
    // for attribute in device.attributes() {
    //     println!("    - {:?} {:?}", attribute.name(), attribute.value());
    // }
    // }

    Ok(())
}
