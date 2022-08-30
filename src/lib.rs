use std::process::Command;

pub mod models;
pub mod utils;

use models::ServerProfile;

pub fn install_wireguard() {
    let cmd_output = Command::new("apt")
        .arg("install")
        .arg("wireguard")
        .output()
        .unwrap();
    if !&cmd_output.status.success() {
        print!("There was something went wrong with install.");
        println!("{}", String::from_utf8_lossy(&cmd_output.stderr));
    } else {
        println!("Wireguard installed successfully");
    }
}

pub fn initialize(install_path: Option<String>) -> Option<ServerProfile> {
    ServerProfile::read_from_config(install_path.unwrap_or("/home/giri/wireguard_mg".to_owned()))
}
