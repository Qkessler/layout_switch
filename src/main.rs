mod list_devices;

use common_macros::hash_map;
use list_devices::find_for_serial_ids;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, process::Command};
use udev::Enumerator;

use list_devices::set_layout;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LayoutSwitcherConfig {
    keyboards: HashMap<String, Vec<String>>,
}

fn run_commands(commands: &[String]) {
    commands.iter().for_each(|command| {
        println!("command {}", *command);

        println!(
            "{:?}",
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .unwrap_or_else(|_| panic!("command {} didn't work", *command))
        );
    });
}

fn main() {
    let mut layout_config = LayoutSwitcherConfig {
        keyboards: hash_map! {
            "TKC_Portico".to_string() => vec!["echo 'this is so cool'".to_string()],
        },
    };
    let config_string = serde_json::to_string(&layout_config).unwrap();
    println!("{:?}", config_string);
    layout_config = serde_json::from_str(&config_string).unwrap();
    println!("{:?}", layout_config);

    let file = File::open("/home/qkessler/.config/layout_switcher/config.json");
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        let layout_config_from_reader: LayoutSwitcherConfig =
            serde_json::from_reader(reader).unwrap();
        println!("{:?}", layout_config_from_reader);
    } else {
        println!("nope");
    }
    return;
    let mut enumerator = Enumerator::new().unwrap();
    let keyboard_id = find_for_serial_ids(&mut enumerator, &layout_config.keyboards);

    if let Some(keyboard_id) = keyboard_id {
        println!("device: {}", keyboard_id);
        println!("commands: {:?}", layout_config.keyboards);
        println!(
            "commands: {:?}",
            layout_config.keyboards.get(keyboard_id.as_str())
        );
        println!("running commands...");
        run_commands(layout_config.keyboards.get(keyboard_id.as_str()).unwrap());
    } else {
        println!("no keyboard found, running default commands");
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_layout_config() {
        let layout_config = LayoutSwitcherConfig {
            keyboards: hash_map! {
                "keyboard_serial_id".to_string() => vec!["echo 'this is so cool'".to_string()],
            },
        };

        assert_eq!(
            "{\"keyboards\":{\"keyboard_serial_id\":[\"echo 'this is so cool'\"]}}",
            serde_json::to_string(&layout_config).unwrap()
        );
    }
}
