mod list_devices;

use common_macros::hash_map;
use clap::Parser;
use home::home_dir;
use list_devices::find_for_serial_ids;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, process::Command};
use udev::Enumerator;

const CONFIG_FILE_PATH: &str = "/.config/layout_switcher/config.json";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LayoutSwitcherConfig {
    keyboards: HashMap<String, Vec<String>>,
}

/// Program to change the keyboard layout depending on the
/// usb keyboard that is connected, with either the command line
/// arguments or the config file under the
/// ".config/layout_switcher/config.json" file.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Keyboard serial id.
    #[clap(short, long)]
    keyboard: Option<String>,

    /// Commands to run if the keyboard is connected, in JSON format.
    #[clap(short, long)]
    commands: Option<String>,
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

fn parse_args() -> Option<(String, Vec<String>)> {
    let args = Args::parse();

    match (args.keyboard, args.commands) {
        (Some(keyboard_id), Some(commands)) => {
            Some((keyboard_id, serde_json::from_str(&commands).unwrap()))
        }
        _ => None,
    }
}

fn get_config_from_file() -> LayoutSwitcherConfig {
    let config_file_str = format!(
        "{}{}",
        home_dir().unwrap().to_str().unwrap(),
        CONFIG_FILE_PATH
    );

    let config_file = File::open(config_file_str).unwrap();
    let reader = BufReader::new(config_file);

    serde_json::from_reader(reader).unwrap()
}

fn main() {
    let keyboard_commands_args = parse_args();

    let layout_config = get_config_with_args(keyboard_commands_args.as_ref());

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

    for device in enumerator.scan_devices().unwrap() {
        for property in device.properties() {
            if property.name().to_str().unwrap() == "ID_SERIAL" {
                println!("    - {:?} {:?}", property.name(), property.value());
            }
        }
    }
    // let output = set_layout("xkblayout-state", &["print", "%v"]).unwrap();
    // println!("{:?}", from_utf8(&output.stdout).unwrap());
}

fn get_config_with_args(
    keyboard_commands_args: Option<&(String, Vec<String>)>,
) -> LayoutSwitcherConfig {
    let mut layout_config = get_config_from_file();
    match keyboard_commands_args {
        Some((keyboard, commands)) => {
            if layout_config.keyboards.contains_key(keyboard) {
                layout_config.keyboards.insert(keyboard.clone(), commands.clone());
            }
            layout_config
        },
       None => layout_config
    }
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
