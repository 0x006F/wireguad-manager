use std::io::{self, BufRead};
use std::process::Command;

fn main() {
    println!("Welcome to Wireguard Management App");
    println!("Please select an option:");

    let mut command = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut command).unwrap();
    let command = command.trim();

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
}
