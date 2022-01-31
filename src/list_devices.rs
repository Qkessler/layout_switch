extern crate udev;

use std::io;

use udev::Device;

pub fn list_devices() -> io::Result<()> {
    let mut enumerator = udev::Enumerator::new()?;

    let keyboard_list: Vec<String> = vec!["TKC_Portico".to_string()];

    let keyboards = enumerator
        .scan_devices()?
        .filter(|d| {
            d.properties().any(|p| {
                p.name().to_str().unwrap() == "ID_SERIAL"
                    && keyboard_list.contains(&p.value().to_str().unwrap().to_string())
            })
        })
        .collect::<Vec<Device>>();

    println!("{:#?}", keyboards);

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
