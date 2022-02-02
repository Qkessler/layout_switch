mod list_devices;

use common_macros::hash_map;
use std::collections::HashMap;

use list_devices::list_devices;
use list_devices::set_layout;

fn main() {
    let layout_config = LayoutSwitcherConfig {
        keyboards: hash_map! {
            "TKC_Portico" => vec!["echo 'this is so cool'"],
        },
    };
    let mut enumerator = Enumerator::new().unwrap();
    let keyboard_id = find_for_serial_ids(&mut enumerator, &layout_config.keyboards);

    if let Some(keyboard_id) = keyboard_id {
        println!("device: {}", keyboard_id);
        println!("commands: {:?}", layout_config.keyboards);
        println!(
            "commands: {:?}",
            layout_config.keyboards.get(keyboard_id.as_str())
        );
    }
    // list_devices().ok();

    /*
    1. Get the list of keyboards, it will be ordered secuentially with priority.
    loop // could loop every 0.5s(?) {
        2. If one of them is connected, run the commands in the config file to change its layout.
        3. Else, do nothing.
    }
    */
    // let output = set_layout("xkblayout-state", &["print", "%v"]).unwrap();
    // println!("{:?}", from_utf8(&output.stdout).unwrap());
}
