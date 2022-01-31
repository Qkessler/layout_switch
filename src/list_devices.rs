extern crate udev;

use std::io;

use udev::{Device, Enumerator};

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

#[cfg(test)]
mod tests {
    struct MockEnumerator;
    struct MockDevice {
        properties: Vec<MockEntry<'static>>,
    }
    struct MockEntry<'a> {
        name: &'a str,
        value: &'a str,
    }

    impl MockDevice {
        pub fn properties(&self) -> Result<Vec<MockEntry<'static>>> {
            Ok(self.properties)
        }
    }

    impl MockEnumerator {
        pub fn scan_devices(&mut self) -> Result<Vec<MockDevice>> {
            Ok(vec![MockDevice {
                properties: vec![MockEntry {
                    name: ID_SERIAL,
                    value: "not_the_keyboard",
                }],
            }, MockDevice {
                properties: vec![MockEntry {
                    name: ID_SERIAL,
                    value: "this_is_the_keyboard",
                }]
            }])
        }
    }

    use std::io::Result;

    use super::*;

    #[test]
    fn test_find_by_serial_id() {
        let mut enumerator = Enumerator::new().unwrap();
        let mut enumerator = MockEnumerator; 

        // find_by_serial_id(&mut enumerator, "TKC_Portico");
        assert_eq!(2 + 2, 4);
    }
}
