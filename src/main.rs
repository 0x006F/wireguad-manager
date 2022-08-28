use std::process::Command;

mod models;
mod utils;

use models::ServerProfile;
use utils::ask;

fn main() {
    println!("Welcome to Wireguard Management App");
    let command = ask("select an option");
    if command == "install" {
        let cmd_output = Command::new("apt")
            .arg("install")
            .arg("wireguard")
            .output()
            .unwrap();
        if !&cmd_output.status.success() {
            print!("There was something went wrong with install.");
            println!("{}", String::from_utf8_lossy(&cmd_output.stderr));
            return;
        } else {
            println!("Wireguard installed successfully");
            return;
        }
    }

    // let test: UserProfile = serde_json::from_value(json!({"name":"asd"})).unwrap();
    // println!("{:?}", test);

    // let server_config = read_server_config();
    // if server_config.is_none() {
    //     println!("Could not read server config. Exiting..");
    //     std::process::exit(1);
    // }

    // let c = ServerProfile::generate(
    //     "100.100.100.100".to_owned(),
    //     "100.100.100.100".to_owned(),
    //     "ens5".to_owned(),
    //     None,
    // );

    // c.persist();

    // if command == "add" {
    //     let profile_name = ask("What is is the name of the user?");
    // }

    let config = ServerProfile::read_from_config();

    if config.is_some() {
        let config = config.unwrap();
        println!("{} {} {}", config.public_ip, config.private_ip, config.port);
    }
}
