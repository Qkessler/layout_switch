mod list_devices;

use std::str::from_utf8;

use list_devices::list_devices;
use list_devices::set_layout;

fn main() {
    list_devices().ok();

    let output = set_layout("xkblayout-state", &["print", "%v"]).unwrap();
    println!("{:?}", from_utf8(&output.stdout).unwrap());
}


