use std::io::{self, BufRead, Write};
use std::process::Command;

fn ask(question: &str) -> String {
    print!("{}: ", question);
    io::stdout().flush().unwrap();
    let mut answer_string = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut answer_string).unwrap();
    // io::stdout().flush().unwrap();
    return answer_string.trim().to_string();
}

fn main() {
    println!("Welcome to Wireguard Management App");
    // let mut command = String::new();
    // let stdin = io::stdin();
    // stdin.lock().read_line(&mut command).unwrap();
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

    // if command == "add" {
    //     let profile_name = ask("What is is the name of the user?");
    // }
}
