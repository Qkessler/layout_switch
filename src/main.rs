mod list_devices;

use common_macros::hash_map;
use home::home_dir;
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
    let config_file_path = "/.config/layout_switcher/config.json";
    let config_file_str = format!(
        "{}{}",
        home_dir().unwrap().to_str().unwrap(),
        config_file_path
    );

    let config_file = File::open(config_file_str).unwrap();
    let reader = BufReader::new(config_file);
    let layout_config: LayoutSwitcherConfig = serde_json::from_reader(reader).unwrap();

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
        run_commands(layout_config.keyboards.get("default").unwrap());
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
